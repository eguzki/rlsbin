// ENVOY_RLS_HOST: host // just to become ENVOY_RLS_HOST:ENVOY_RLS_PORT as String
// ENVOY_RLS_PORT: port

#[derive(Debug)]
pub struct Configuration {
    rls_host: String,
    rls_port: u16,
    pub log_level: Option<log::Level>,
    pub grpc_reflection_service: bool,
}

impl Configuration {
    pub const DEFAULT_RLS_PORT: &'static str = "8081";
    pub const DEFAULT_IP_BIND: &'static str = "0.0.0.0";

    #[allow(clippy::too_many_arguments)]
    pub fn with(rls_host: String, rls_port: u16, grpc_reflection_service: bool) -> Self {
        Self {
            rls_host,
            rls_port,
            log_level: None,
            grpc_reflection_service,
        }
    }

    pub fn rlp_address(&self) -> String {
        format!("{}:{}", self.rls_host, self.rls_port)
    }
}

#[cfg(test)]
impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            rls_host: "".to_string(),
            rls_port: 0,
            log_level: None,
            grpc_reflection_service: false,
        }
    }
}
