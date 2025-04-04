#![allow(clippy::missing_safety_doc, unsafe_op_in_unsafe_fn)]

use std::ffi::c_void;

/// cbindgen:no-export
#[repr(C)]
pub struct VrtCtx {
    pub magic: u32,
    pub syntax: u32,
    pub method: u32,
    pub handling: *mut u32, // not in director context
    pub vclver: u32,

    pub msg: *mut c_void, // Only in ...init()
    pub vsl: *mut c_void,
    pub vcl: *mut c_void,
    pub ws: *mut c_void,

    pub sp: *mut c_void,

    pub req: *mut c_void,
    pub http_req: *mut c_void,
    pub http_req_top: *mut c_void,
    pub http_resp: *mut c_void,

    pub bo: *mut c_void,
    pub http_bereq: *mut c_void,
    pub http_beresp: *mut c_void,

    pub now: f64,

    // method specific argument:
    // hash:		struct VSHA256Context
    // synth+error:	struct vsb *
    pub specific: *mut c_void,
}

#[unsafe(no_mangle)]
pub extern "C" fn ratrod() {
    println!("lol")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dostuff(ctx: *const VrtCtx) -> i32 {
    // Null check

    if ctx.is_null() {
        return -1;
    }

    // Dereference the pointer to access fields
    let ctx_ref = &*ctx;

    // Process the context
    // Example: Access some fields
    println!("Processing VCL version: {}", ctx_ref.vclver);

    // Return success
    0
}
