use crate::ctx::Txt;
use crate::ctx::req::{VslLog, Ws};

// Define enum for VSL tags
#[repr(C)]
pub enum VslTagE {
    // VSL_tag_e enum values go here
    // This would typically include values like SLT_Method, etc.
    _Placeholder = 0,
}

// Define the http struct
#[repr(C)]
pub struct Http {
    pub magic: u32,
    pub shd: u16,     // Size of hd space
    pub hd: *mut Txt, // Header array
    pub hdf: *mut u8, // Flags for each header

    // Fields below are zeroed/initialized by http_Teardown
    pub nhd: u16, // Next free header

    pub logtag: VslTagE, // Must be SLT_*Method
    pub vsl: *mut VslLog,

    pub ws: *mut Ws,
    pub status: u16,
    pub protover: u8,
}
