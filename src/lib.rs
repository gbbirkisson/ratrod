#![allow(clippy::missing_safety_doc, unsafe_op_in_unsafe_fn)]

mod ctx;
mod returns;
use crate::ctx::VrtCtx;
use crate::returns::{VclRet, vrt_handling};

/// Simple test
#[unsafe(no_mangle)]
pub extern "C" fn ratrod() {
    println!("Hello from ratrod")
}

/// Set vcl return with rust
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ratrod_fail(ctx: *const VrtCtx) {
    vrt_handling(ctx, VclRet::Fail);
}

/// Muck about with the request
/// Ref: https://github.com/varnishcache/varnish-cache/tree/6.0/include
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ratrod_ctx(ctx: *const VrtCtx) -> i32 {
    // Null check
    if ctx.is_null() {
        return -1;
    }

    // Dereference the pointer to access fields
    let ctx_ref = &*ctx;
    println!("Processing VCL version: {}", ctx_ref.vclver);

    // Pull out the request
    let req = &*ctx_ref.req;
    let http = &*req.http;
    let headers = &*http.hd;
    println!("{:?}", req.http);
    // println!("{:?}", headers); // This will segfault 🙈

    0
}
