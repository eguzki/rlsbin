use tonic::{transport, transport::Server, Request, Response, Status};

use crate::envoy_rls::custom_descriptor_entries::has_x_overlimit;
use crate::envoy_rls::server::envoy::service::ratelimit::v3::rate_limit_response::Code;
use crate::envoy_rls::server::envoy::service::ratelimit::v3::rate_limit_service_server::{
    RateLimitService, RateLimitServiceServer,
};
use crate::envoy_rls::server::envoy::service::ratelimit::v3::{
    RateLimitRequest, RateLimitResponse,
};

include!("envoy_types.rs");

pub struct MyRateLimiter;

#[tonic::async_trait]
impl RateLimitService for MyRateLimiter {
    async fn should_rate_limit(
        &self,
        request: Request<RateLimitRequest>,
    ) -> Result<Response<RateLimitResponse>, Status> {
        debug!("Request received: {:?}", request);

        let (_metadata, _ext, req) = request.into_parts();

        let resp_code = if req.descriptors.iter().any(has_x_overlimit) {
            Code::OverLimit
        } else {
            // default value
            Code::Ok
        };

        let reply = RateLimitResponse {
            overall_code: resp_code.into(),
            statuses: vec![],
            request_headers_to_add: vec![],  // TODO
            response_headers_to_add: vec![], // TODO
            raw_body: vec![],
            dynamic_metadata: None,
            quota: None,
        };

        Ok(Response::new(reply))
    }
}

mod rls_proto {
    pub(crate) const RLS_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("rls");
}

pub async fn run_envoy_rls_server(
    address: String,
    grpc_reflection_service: bool,
) -> Result<(), transport::Error> {
    let svc = RateLimitServiceServer::new(MyRateLimiter);

    let reflection_service = match grpc_reflection_service {
        false => None,
        true => Some(
            tonic_reflection::server::Builder::configure()
                .register_encoded_file_descriptor_set(rls_proto::RLS_DESCRIPTOR_SET)
                .build_v1()
                .unwrap(),
        ),
    };

    Server::builder()
        .add_service(svc)
        .add_optional_service(reflection_service)
        .serve(address.parse().unwrap())
        .await
}

#[cfg(test)]
mod tests {
    use tonic::IntoRequest;

    use crate::envoy_rls::custom_descriptor_entries::X_OVERLIMIT_ENTRY_NAME;
    use crate::envoy_rls::server::envoy::extensions::common::ratelimit::v3::rate_limit_descriptor::Entry;
    use crate::envoy_rls::server::envoy::extensions::common::ratelimit::v3::RateLimitDescriptor;

    use super::*;

    #[tokio::test]
    async fn test_returns_ok_correctly() {
        let namespace = "test_namespace";
        let req = RateLimitRequest {
            domain: namespace.to_string(),
            descriptors: vec![RateLimitDescriptor {
                entries: vec![
                    Entry {
                        key: "key-a".to_string(),
                        value: "value-a".to_string(),
                    },
                    Entry {
                        key: "key-b".to_string(),
                        value: "value-b".to_string(),
                    },
                ],
                limit: None,
            }],
            hits_addend: 1,
        };

        let response = MyRateLimiter
            .should_rate_limit(req.clone().into_request())
            .await
            .unwrap()
            .into_inner();
        assert_eq!(response.overall_code, i32::from(Code::Ok));
    }

    #[tokio::test]
    async fn test_returns_overlimit_correctly() {
        let namespace = "test_namespace";
        let req = RateLimitRequest {
            domain: namespace.to_string(),
            descriptors: vec![RateLimitDescriptor {
                entries: vec![
                    Entry {
                        key: X_OVERLIMIT_ENTRY_NAME.to_string(),
                        value: "value-a".to_string(),
                    },
                    Entry {
                        key: "key-b".to_string(),
                        value: "value-b".to_string(),
                    },
                ],
                limit: None,
            }],
            hits_addend: 1,
        };

        let response = MyRateLimiter
            .should_rate_limit(req.clone().into_request())
            .await
            .unwrap()
            .into_inner();
        assert_eq!(response.overall_code, i32::from(Code::OverLimit));
    }
}
