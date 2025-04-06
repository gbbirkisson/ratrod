mod ctx;
mod ret;
use crate::ctx::VrtCtx;

/// Simple test
#[unsafe(no_mangle)]
pub extern "C" fn ratrod() {
    println!("Hello from ratrod")
}

/// Set vcl return with rust
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ratrod_fail(ctx: *const VrtCtx) {
    fail!(ctx);
}

/// Muck about with the request
#[allow(unsafe_op_in_unsafe_fn)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ratrod_ctx(ctx: *const VrtCtx) -> i32 {
    let ctx_ref = dbg!(&*ctx);
    let req = dbg!(&*ctx_ref.req);
    let http = dbg!(&*req.http);
    let _headers = dbg!(&*http.hd);

    0
}
