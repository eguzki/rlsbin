use crate::envoy_rls::server::envoy::extensions::common::ratelimit::v3::RateLimitDescriptor;

pub const X_OVERLIMIT_ENTRY_NAME: &'static str = "x-overlimit";

pub fn has_x_overlimit(descriptor: &RateLimitDescriptor) -> bool {
    descriptor
        .entries
        .iter()
        .any(|entry| entry.key == X_OVERLIMIT_ENTRY_NAME)
}
