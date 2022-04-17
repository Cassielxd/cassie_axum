mod async_cancel;
mod async_cell;
mod bindings;
pub mod error;
mod error_codes;
mod extensions;
mod flags;
mod gotham_state;
mod inspector;
mod module_specifier;
mod modules;
mod normalize_path;
mod ops;
mod ops_builtin;
mod ops_metrics;
mod resources;
mod runtime;

// Re-exports
pub use anyhow;
pub use futures;
pub use parking_lot;
pub use serde;
pub use serde_json;
pub use serde_v8;
pub use serde_v8::Buffer as ZeroCopyBuf;
pub use serde_v8::ByteString;
pub use serde_v8::StringOrBuffer;
pub use serde_v8::U16String;
pub use url;
pub use v8;

pub use crate::async_cancel::CancelFuture;
pub use crate::async_cancel::CancelHandle;
pub use crate::async_cancel::CancelTryFuture;
pub use crate::async_cancel::Cancelable;
pub use crate::async_cancel::Canceled;
pub use crate::async_cancel::TryCancelable;
pub use crate::async_cell::AsyncMut;
pub use crate::async_cell::AsyncMutFuture;
pub use crate::async_cell::AsyncRef;
pub use crate::async_cell::AsyncRefCell;
pub use crate::async_cell::AsyncRefFuture;
pub use crate::async_cell::RcLike;
pub use crate::async_cell::RcRef;
pub use crate::extensions::Extension;
pub use crate::extensions::ExtensionBuilder;
pub use crate::extensions::OpDecl;
pub use crate::extensions::OpMiddlewareFn;
pub use crate::flags::v8_set_flags;
pub use crate::inspector::InspectorMsg;
pub use crate::inspector::InspectorMsgKind;
pub use crate::inspector::InspectorSessionProxy;
pub use crate::inspector::JsRuntimeInspector;
pub use crate::inspector::LocalInspectorSession;
pub use crate::module_specifier::resolve_import;
pub use crate::module_specifier::resolve_path;
pub use crate::module_specifier::resolve_url;
pub use crate::module_specifier::resolve_url_or_path;
pub use crate::module_specifier::ModuleResolutionError;
pub use crate::module_specifier::ModuleSpecifier;
pub use crate::module_specifier::DUMMY_SPECIFIER;
pub use crate::modules::FsModuleLoader;
pub use crate::modules::ModuleId;
pub use crate::modules::ModuleLoader;
pub use crate::modules::ModuleSource;
pub use crate::modules::ModuleSourceFuture;
pub use crate::modules::ModuleType;
pub use crate::modules::NoopModuleLoader;
pub use crate::normalize_path::normalize_path;
pub use crate::ops::Op;
pub use crate::ops::OpAsyncFuture;
pub use crate::ops::OpCall;
pub use crate::ops::OpError;
pub use crate::ops::OpFn;
pub use crate::ops::OpId;
pub use crate::ops::OpResult;
pub use crate::ops::OpState;
pub use crate::ops::PromiseId;
pub use crate::ops_metrics::OpsTracker;
pub use crate::resources::AsyncResult;
pub use crate::resources::Resource;
pub use crate::resources::ResourceId;
pub use crate::resources::ResourceTable;
pub use crate::runtime::CompiledWasmModuleStore;
pub use crate::runtime::CrossIsolateStore;
pub use crate::runtime::GetErrorClassFn;
pub use crate::runtime::JsErrorCreateFn;
pub use crate::runtime::JsRuntime;
pub use crate::runtime::RuntimeOptions;
pub use crate::runtime::SharedArrayBufferStore;
pub use crate::runtime::Snapshot;
pub use cassie_ops::op;

pub fn v8_version() -> &'static str {
    v8::V8::get_version()
}

/// An internal module re-exporting funcs used by the #[op] (`cassie_ops`) macro
#[doc(hidden)]
pub mod _ops {
    pub use super::bindings::throw_type_error;
    pub use super::error_codes::get_error_code;
    pub use super::ops::to_op_result;
    pub use super::ops::OpCtx;
    pub use super::runtime::queue_async_op;
}

/// A helper macro that will return a call site in Rust code. Should be
/// used when executing internal one-line scripts for JsRuntime lifecycle.
///
/// Returns a string in form of: "`[cassie:<filename>:<line>:<column>]`"
#[macro_export]
macro_rules! located_script_name {
    () => {
        format!(
            "[cassie:{}:{}:{}]",
            std::file!(),
            std::line!(),
            std::column!()
        );
    };
}