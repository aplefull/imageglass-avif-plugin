use crate::abi::{IGHostApi, IGStringRef};
use std::ffi::c_void;
use std::sync::atomic::{AtomicPtr, Ordering};

static HOST_API: AtomicPtr<IGHostApi> = AtomicPtr::new(std::ptr::null_mut());

/// Stores the host table handed to the entry point
pub fn set_host_api(host_api: *const IGHostApi) {
    HOST_API.store(host_api as *mut IGHostApi, Ordering::Release);
}

/// Polls the host for cancellation of the given opaque token
pub fn is_canceled(cancellation: *mut c_void) -> bool {
    if cancellation.is_null() {
        return false;
    }

    let host = HOST_API.load(Ordering::Acquire);
    if host.is_null() {
        return false;
    }

    // ACTUALLY_SAFE: the host keeps its tables alive for the plugin's lifetime
    unsafe {
        let core = (*host).core;
        match core.as_ref().and_then(|c| c.is_cancellation_requested) {
            Some(f) => f(cancellation) != 0,
            None => false,
        }
    }
}

/// Sends a message to the host's plugin log channel
pub fn log(level: i32, message: &str) {
    let host = HOST_API.load(Ordering::Acquire);
    if host.is_null() {
        return;
    }

    let units: Vec<u16> = message.encode_utf16().collect();

    // ACTUALLY_SAFE: the host reads the slice synchronously, while `units` is alive
    unsafe {
        let core = (*host).core;
        if let Some(f) = core.as_ref().and_then(|c| c.log) {
            f(
                level,
                IGStringRef {
                    data: units.as_ptr(),
                    length: units.len() as i32,
                },
            );
        }
    }
}
