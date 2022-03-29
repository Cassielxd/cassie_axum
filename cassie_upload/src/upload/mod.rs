use oss_rust_sdk::prelude::OSS;

pub mod oss_service;
pub mod upload_service;
static OSS_CLIENT: state::Storage<OSS> = state::Storage::new();