use crate::VrtCtx;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VclRet {
    Abandon = 1,
    Deliver = 2,
    Error = 3,
    Fail = 4,
    Fetch = 5,
    Hash = 6,
    Lookup = 7,
    Miss = 8,
    Ok = 9,
    Pass = 10,
    Pipe = 11,
    Purge = 12,
    Restart = 13,
    Retry = 14,
    Synth = 15,
    Vcl = 16,
}

unsafe extern "C" {
    fn VRT_handling(ctx: *const VrtCtx, hand: u32);
}

pub fn vrt_handling(ctx: *const VrtCtx, ret: VclRet) {
    unsafe {
        VRT_handling(ctx, ret as u32);
    }
}
