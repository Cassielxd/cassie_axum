use oss_rust_sdk::prelude::OSS;
use std::sync::Arc;

pub mod oss_service;
pub mod upload_service;
static OSS_CLIENT: state::Storage<Arc<OSS>> = state::Storage::new();
