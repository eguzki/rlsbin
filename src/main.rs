#[macro_use]
extern crate log;
extern crate clap;

mod config;
mod envoy_rls;

use crate::config::Configuration;
use crate::envoy_rls::server::run_envoy_rls_server;
use clap::{value_parser, Arg, ArgAction, Command};
use const_format::formatcp;

const RLSBIN_VERSION: &str = env!("CARGO_PKG_VERSION");
const RLSBIN_PROFILE: &str = env!("RLSBIN_PROFILE");
const RLSBIN_FEATURES: &str = env!("RLSBIN_FEATURES");
const RLSBIN_HEADER: &str = "Envoy RLS Mock Server";

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = {
        let (config, version) = create_config();
        println!("{RLSBIN_HEADER} {version}");

        info!("Version: {}", version);
        info!("Using config: {:?}", config);
        config
    };

    simple_logger::init_with_level(config.log_level.unwrap_or(log::Level::Error)).unwrap();

    let envoy_rls_address = config.rlp_address();
    let grpc_reflection_service = config.grpc_reflection_service;

    info!("Envoy RLS server starting on {}", envoy_rls_address);
    tokio::spawn(run_envoy_rls_server(
        envoy_rls_address.to_string(),
        grpc_reflection_service,
    ));

    Ok(())
}

fn create_config() -> (Configuration, &'static str) {
    let full_version: &'static str = formatcp!(
        "v{} ({}) {} {}",
        RLSBIN_VERSION,
        env!("RLSBIN_GIT_HASH"),
        RLSBIN_FEATURES,
        RLSBIN_PROFILE,
    );

    // build app
    let cmdline = Command::new(RLSBIN_HEADER)
        .version(full_version)
        .author("Eguzki Astiz Lezaun - github.com/eguzki")
        .about("Rate limiting mock service that integrates with Envoy's RLS protocol")
        .subcommand_negates_reqs(false)
        .subcommand_required(false)
        .arg(
            Arg::new("ip")
                .short('b')
                .long("rls-ip")
                .default_value(Configuration::DEFAULT_IP_BIND)
                .display_order(1)
                .help("The IP to listen on for RLS"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("rls-port")
                .default_value(Configuration::DEFAULT_RLS_PORT)
                .value_parser(value_parser!(u16))
                .display_order(2)
                .help("The port to listen on for RLS"),
        )
        .arg(
            Arg::new("v")
                .short('v')
                .action(ArgAction::Count)
                .value_parser(value_parser!(u8).range(..5))
                .display_order(7)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::new("grpc_reflection_service")
                .long("grpc-reflection-service")
                .action(ArgAction::SetTrue)
                .display_order(10)
                .help("Enables gRPC server reflection service"),
        );

    let matches = cmdline.get_matches();

    let mut config = Configuration::with(
        matches.get_one::<String>("ip").unwrap().into(),
        *matches.get_one::<u16>("port").unwrap(),
        matches.get_flag("grpc_reflection_service"),
    );

    config.log_level = match matches.get_count("v") {
        0 => None,
        1 => Some(log::Level::Warn),
        2 => Some(log::Level::Info),
        3 => Some(log::Level::Debug),
        4 => Some(log::Level::Trace),
        _ => unreachable!("Verbosity should at most be 4!"),
    };

    (config, full_version)
}
