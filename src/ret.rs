use crate::VrtCtx;

#[allow(dead_code)]
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

// Macros for each VclRet variant

#[macro_export]
macro_rules! abandon {
    ($ctx:expr) => {
        $crate::ret::vrt_handling($ctx, $crate::ret::VclRet::Abandon)
    };
}

#[macro_export]
macro_rules! deliver {
    ($ctx:expr) => {
        $crate::ret::vrt_handling($ctx, $crate::ret::VclRet::Deliver)
    };
}

#[macro_export]
macro_rules! error {
    ($ctx:expr) => {
        $crate::ret::vrt_handling($ctx, $crate::ret::VclRet::Error)
    };
}

#[macro_export]
macro_rules! fail {
    ($ctx:expr) => {
        $crate::ret::vrt_handling($ctx, $crate::ret::VclRet::Fail)
    };
}

#[macro_export]
macro_rules! fetch {
    ($ctx:expr) => {
        $crate::ret::vrt_handling($ctx, $crate::ret::VclRet::Fetch)
    };
}

#[macro_export]
macro_rules! hash {
    ($ctx:expr) => {
        $crate::ret::vrt_handling($ctx, $crate::ret::VclRet::Hash)
    };
}

#[macro_export]
macro_rules! lookup {
    ($ctx:expr) => {
        $crate::ret::vrt_handling($ctx, $crate::ret::VclRet::Lookup)
    };
}

#[macro_export]
macro_rules! miss {
    ($ctx:expr) => {
        $crate::ret::vrt_handling($ctx, $crate::ret::VclRet::Miss)
    };
}

#[macro_export]
macro_rules! ok {
    ($ctx:expr) => {
        $crate::ret::vrt_handling($ctx, $crate::ret::VclRet::Ok)
    };
}

#[macro_export]
macro_rules! pass {
    ($ctx:expr) => {
        $crate::ret::vrt_handling($ctx, $crate::ret::VclRet::Pass)
    };
}

#[macro_export]
macro_rules! pipe {
    ($ctx:expr) => {
        $crate::ret::vrt_handling($ctx, $crate::ret::VclRet::Pipe)
    };
}

#[macro_export]
macro_rules! purge {
    ($ctx:expr) => {
        $crate::ret::vrt_handling($ctx, $crate::ret::VclRet::Purge)
    };
}

#[macro_export]
macro_rules! restart {
    ($ctx:expr) => {
        $crate::ret::vrt_handling($ctx, $crate::ret::VclRet::Restart)
    };
}

#[macro_export]
macro_rules! retry {
    ($ctx:expr) => {
        $crate::ret::vrt_handling($ctx, $crate::ret::VclRet::Retry)
    };
}

#[macro_export]
macro_rules! synth {
    ($ctx:expr) => {
        $crate::ret::vrt_handling($ctx, $crate::ret::VclRet::Synth)
    };
}

#[macro_export]
macro_rules! vcl {
    ($ctx:expr) => {
        $crate::ret::vrt_handling($ctx, $crate::ret::VclRet::Vcl)
    };
}
