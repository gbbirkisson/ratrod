use crate::ctx::req::Req;
use std::ffi::c_char;
use std::ffi::c_void;
use std::fmt;
use std::slice;

pub mod http;
pub mod req;

#[repr(C)]
pub struct Txt {
    pub b: *const c_char, // Beginning pointer
    pub e: *const c_char, // Ending pointer
}

impl Txt {
    /// Gets the length of the text in bytes
    pub fn len(&self) -> usize {
        if self.b.is_null() || self.e.is_null() {
            return 0;
        }
        unsafe { self.e.offset_from(self.b) as usize }
    }

    /// Checks if the text is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Safely convert to a Rust string slice if possible
    pub fn as_str(&self) -> Option<&str> {
        if self.is_empty() {
            return None;
        }

        unsafe {
            let slice = slice::from_raw_parts(self.b as *const u8, self.len());

            std::str::from_utf8(slice).ok()
        }
    }

    /// Get the raw bytes without UTF-8 validation
    pub fn as_bytes(&self) -> &[u8] {
        if self.is_empty() {
            return &[];
        }

        unsafe { slice::from_raw_parts(self.b as *const u8, self.len()) }
    }

    /// Print the content as a debug hex dump
    pub fn hex_dump(&self) -> String {
        let bytes = self.as_bytes();
        let mut result = String::new();

        for (i, byte) in bytes.iter().enumerate() {
            if i % 16 == 0 && i > 0 {
                result.push('\n');
            }
            result.push_str(&format!("{:02x} ", byte));
        }

        result
    }
}

// Implement Display for easy printing
impl fmt::Display for Txt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.as_str() {
            Some(s) => write!(f, "{}", s),
            None => {
                if self.is_empty() {
                    write!(f, "<empty>")
                } else {
                    write!(f, "<binary data of length {}>", self.len())
                }
            }
        }
    }
}

// Implement Debug for detailed inspection
impl fmt::Debug for Txt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Txt")
            .field("ptr", &format!("b: {:?}, e: {:?}", self.b, self.e))
            .field("len", &self.len())
            .field("content", &self.to_string())
            .finish()
    }
}

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

    pub req: *mut Req,
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
