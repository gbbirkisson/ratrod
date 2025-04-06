use std::fmt;

// Ref: https://github.com/varnishcache/varnish-cache/tree/6.0/include

#[repr(C)]
pub struct VrtCtx {
    pub magic: u32,
    pub syntax: u32,
    pub method: u32,
    pub handling: *mut u32, // not in director context
    pub vclver: u32,
    pub msg: *mut Vsb,
    pub vsl: *mut VslLog,
    pub vcl: *mut Vcl,
    pub ws: *mut Ws,
    pub sp: *mut Sess,
    pub req: *mut Req,
    pub http_req: *mut Http,
    pub http_req_top: *mut Http,
    pub http_resp: *mut Http,
    pub bo: *mut Busyobj,
    pub http_bereq: *mut Http,
    pub http_beresp: *mut Http,
    pub now: f64, // vtim_real is a double
    pub specific: *mut libc::c_void,
}

impl fmt::Debug for VrtCtx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VrtCtx")
            .field("magic", &format!("0x{:08x}", self.magic))
            .field("syntax", &self.syntax)
            .field("method", &self.method)
            .field("handling", &self.handling)
            .field("vclver", &self.vclver)
            .field("msg", &self.msg)
            .field("vsl", &self.vsl)
            .field("vcl", &self.vcl)
            .field("ws", &self.ws)
            .field("sp", &self.sp)
            .field("req", &self.req)
            .field("http_req", &self.http_req)
            .field("http_req_top", &self.http_req_top)
            .field("http_resp", &self.http_resp)
            .field("bo", &self.bo)
            .field("http_bereq", &self.http_bereq)
            .field("http_beresp", &self.http_beresp)
            .field("now", &self.now)
            .field("specific", &self.specific)
            .finish()
    }
}

#[repr(C)]
pub struct Vsb {
    pub magic: u32,
    pub s_error: libc::c_int,
    pub s_buf: *mut libc::c_char,
    pub s_size: isize,
    pub s_len: isize,
    pub s_flags: libc::c_int,
    pub s_indent: libc::c_int,
}

impl fmt::Debug for Vsb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vsb")
            .field("magic", &format!("0x{:08x}", self.magic))
            .field("s_error", &self.s_error)
            .field("s_buf", &self.s_buf)
            .field("s_size", &self.s_size)
            .field("s_len", &self.s_len)
            .field("s_flags", &self.s_flags)
            .field("s_indent", &self.s_indent)
            .finish()
    }
}

#[repr(C)]
pub struct VslLog {
    pub wlb: *mut u32,
    pub wlp: *mut u32,
    pub wle: *mut u32,
    pub wlr: u32,
    pub wid: u32,
}

impl fmt::Debug for VslLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VslLog")
            .field("wlb", &self.wlb)
            .field("wlp", &self.wlp)
            .field("wle", &self.wle)
            .field("wlr", &self.wlr)
            .field("wid", &self.wid)
            .finish()
    }
}

#[repr(C)]
pub struct Ws {
    pub magic: u32,
    pub id: [libc::c_char; 4],
    pub s: *mut libc::c_char,
    pub f: *mut libc::c_char,
    pub r: *mut libc::c_char,
    pub e: *mut libc::c_char,
}

impl fmt::Debug for Ws {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.id.iter().map(|&c| c as u8).collect::<Vec<u8>>();
        let id_str = unsafe { std::str::from_utf8_unchecked(&bytes) };

        f.debug_struct("Ws")
            .field("magic", &format!("0x{:08x}", self.magic))
            .field("id", &id_str)
            .field("s", &self.s)
            .field("f", &self.f)
            .field("r", &self.r)
            .field("e", &self.e)
            .finish()
    }
}

#[repr(C)]
pub struct Sess {
    pub magic: u32,
    pub sattr: [u16; 0], // SA_LAST not defined, using 0
    pub listen_sock: *mut ListenSock,
    pub refcnt: libc::c_int,
    pub fd: libc::c_int,
    pub vxid: u32,
    pub mtx: Lock,
    pub pool: *mut Pool,
    pub ws: [Ws; 1],
    pub t_open: f64,       // vtim_real is a double
    pub t_idle: f64,       // vtim_real is a double
    pub timeout_idle: f64, // vtim_dur is a double
}

impl fmt::Debug for Sess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sess")
            .field("magic", &format!("0x{:08x}", self.magic))
            .field("listen_sock", &self.listen_sock)
            .field("refcnt", &self.refcnt)
            .field("fd", &self.fd)
            .field("vxid", &self.vxid)
            .field("pool", &self.pool)
            .field("ws", &self.ws)
            .field("t_open", &self.t_open)
            .field("t_idle", &self.t_idle)
            .field("timeout_idle", &self.timeout_idle)
            .finish()
    }
}

#[repr(C)]
pub struct Http {
    pub magic: u32,
    pub shd: u16,
    pub hd: *mut Txt,
    pub hdf: *mut u8,
    pub nhd: u16,
    pub logtag: libc::c_int, // enum VSL_tag_e
    pub vsl: *mut VslLog,
    pub ws: *mut Ws,
    pub status: u16,
    pub protover: u8,
    pub conds: u8,
}

impl fmt::Debug for Http {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Http")
            .field("magic", &format!("0x{:08x}", self.magic))
            .field("shd", &self.shd)
            .field("hd", &self.hd)
            .field("hdf", &self.hdf)
            .field("nhd", &self.nhd)
            .field("logtag", &self.logtag)
            .field("vsl", &self.vsl)
            .field("ws", &self.ws)
            .field("status", &self.status)
            .field("protover", &self.protover)
            .field("conds", &self.conds)
            .finish()
    }
}

#[repr(C)]
pub struct Txt {
    pub b: *const libc::c_char,
    pub e: *const libc::c_char,
}

impl fmt::Debug for Txt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Calculate length safely
        let len = if !self.b.is_null() && !self.e.is_null() && self.e >= self.b {
            unsafe { self.e.offset_from(self.b) as usize }
        } else {
            0
        };

        // Get the content as a string if possible
        let content = if len > 0 {
            unsafe {
                // Create a slice from the memory between b and e
                let slice = std::slice::from_raw_parts(self.b as *const u8, len);
                // Try to convert to UTF-8, fallback to showing as hex
                match std::str::from_utf8(slice) {
                    Ok(s) => format!("{:?}", s),
                    Err(_) => {
                        // If not valid UTF-8, display as hex
                        if len > 32 {
                            // Truncate if too long
                            format!(
                                "[{} bytes: {}...]",
                                len,
                                slice[..32]
                                    .iter()
                                    .map(|b| format!("{:02x}", b))
                                    .collect::<Vec<_>>()
                                    .join(" ")
                            )
                        } else {
                            format!(
                                "[{} bytes: {}]",
                                len,
                                slice
                                    .iter()
                                    .map(|b| format!("{:02x}", b))
                                    .collect::<Vec<_>>()
                                    .join(" ")
                            )
                        }
                    }
                }
            }
        } else {
            String::from("(empty)")
        };

        f.debug_struct("Txt")
            .field("b", &self.b)
            .field("e", &self.e)
            .field("len", &len)
            .field("content", &content)
            .finish()
    }
}

#[repr(C)]
pub struct Req {
    pub magic: u32,
    pub req_step: libc::c_int,        // enum req_step
    pub req_body_status: libc::c_int, // enum req_body_state_e
    pub doclose: libc::c_int,         // enum sess_close
    pub restarts: libc::c_int,
    pub esi_level: libc::c_int,
    pub top: *mut Req,

    // Request flags (these need to be manually checked against tbl/req_flags.h)
    pub is_hit: u32,
    pub is_grace: u32,
    pub is_pass: u32,
    pub is_hitmiss: u32,
    pub is_hitpass: u32,
    pub is_synth: u32,
    pub is_transmuted: u32,
    pub hash_ignore_busy: u32,
    pub hash_always_miss: u32,
    pub hash_ignore_vary: u32,
    pub exp_first: u32,
    pub disable_esi: u32,
    pub resp_hdrs_started: u32,

    pub err_code: u16,
    pub err_reason: *const libc::c_char,

    pub sp: *mut Sess,
    pub wrk: *mut Worker,
    pub task: PoolTask,

    pub transport: *const Transport,
    pub transport_priv: *mut libc::c_void,

    pub w_list: TailqEntryReq,

    pub body_oc: *mut Objcore,
    pub hash_objhead: *mut Objhead,

    pub vary_b: *mut u8,
    pub vary_l: *mut u8,
    pub vary_e: *mut u8,

    pub digest: [u8; 32], // Assuming DIGEST_LEN is 32

    pub d_ttl: libc::c_double,
    pub d_grace: libc::c_double,

    pub req_bodybytes: isize,
    pub storage: *const Stevedore,

    pub director_hint: *const Director,
    pub vcl: *mut Vcl,

    pub ws_req: usize,

    pub t_first: libc::c_double,
    pub t_prev: libc::c_double,
    pub t_req: libc::c_double,

    pub htc: *mut HttpConn,
    pub vfc: *mut VfpCtx,
    pub client_identity: *const libc::c_char,

    pub http: *mut Http,
    pub http0: *mut Http,

    pub resp: *mut Http,
    pub resp_len: i64,

    pub ws: [Ws; 1],
    pub objcore: *mut Objcore,
    pub stale_oc: *mut Objcore,

    pub vdc: *mut VdpCtx,

    pub res_mode: u32,

    pub vsl: [VslLog; 1],

    pub acct: AcctReq,

    pub privs: [VrtPrivs; 1],
}

impl fmt::Debug for Req {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let digest_hex = self
            .digest
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .join("");

        f.debug_struct("Req")
            .field("magic", &format!("0x{:08x}", self.magic))
            .field("req_step", &self.req_step)
            .field("req_body_status", &self.req_body_status)
            .field("doclose", &self.doclose)
            .field("restarts", &self.restarts)
            .field("esi_level", &self.esi_level)
            .field("top", &self.top)
            // Flags (showing just a few key ones)
            .field("is_hit", &(self.is_hit != 0))
            .field("is_grace", &(self.is_grace != 0))
            .field("is_pass", &(self.is_pass != 0))
            .field("is_synth", &(self.is_synth != 0))
            .field("err_code", &self.err_code)
            .field("err_reason", &self.err_reason)
            .field("digest", &digest_hex)
            .field("req_bodybytes", &self.req_bodybytes)
            .field("ws_req", &self.ws_req)
            .field("resp_len", &self.resp_len)
            .field("res_mode", &self.res_mode)
            .finish()
    }
}

#[repr(C)]
pub struct Busyobj {
    pub magic: u32,
    pub end: *mut libc::c_char,
    pub retries: libc::c_int,
    pub req: *mut Req,
    pub sp: *mut Sess,
    pub wrk: *mut Worker,
    pub vfc: *mut VfpCtx,
    pub ws: [Ws; 1],
    pub ws_bo: usize,
    pub bereq0: *mut Http,
    pub bereq: *mut Http,
    pub beresp: *mut Http,
    pub stale_oc: *mut Objcore,
    pub fetch_objcore: *mut Objcore,
    pub htc: *mut HttpConn,
    pub fetch_task: PoolTask,

    // Flags (should be verified against tbl/bo_flags.h)
    pub do_esi: u32,
    pub do_gzip: u32,
    pub do_gunzip: u32,
    pub do_stream: u32,
    pub do_pass: u32,
    pub uncacheable: u32,
    pub failed: u32,

    pub connect_timeout: f64,       // vtim_dur is a double
    pub first_byte_timeout: f64,    // vtim_dur is a double
    pub between_bytes_timeout: f64, // vtim_dur is a double

    pub t_first: libc::c_double,
    pub t_prev: libc::c_double,

    pub acct: AcctBereq,

    pub storage: *const Stevedore,
    pub director_req: *const Director,
    pub director_resp: *const Director,
    pub director_state: libc::c_int, // enum director_state_e
    pub vcl: *mut Vcl,

    pub vsl: [VslLog; 1],

    pub digest: [u8; 32], // Assuming DIGEST_LEN is 32
    pub privs: [VrtPrivs; 1],

    pub err_code: u16,
    pub err_reason: *const libc::c_char,

    pub initial_req_body_status: libc::c_int, // enum req_body_state_e
    pub bereq_body: *mut Objcore,
}

impl fmt::Debug for Busyobj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let digest_hex = self
            .digest
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .join("");

        f.debug_struct("Busyobj")
            .field("magic", &format!("0x{:08x}", self.magic))
            .field("retries", &self.retries)
            .field("req", &self.req)
            .field("sp", &self.sp)
            .field("ws_bo", &self.ws_bo)
            // Flags
            .field("do_esi", &(self.do_esi != 0))
            .field("do_gzip", &(self.do_gzip != 0))
            .field("do_gunzip", &(self.do_gunzip != 0))
            .field("do_stream", &(self.do_stream != 0))
            .field("do_pass", &(self.do_pass != 0))
            .field("uncacheable", &(self.uncacheable != 0))
            .field("failed", &(self.failed != 0))
            .field("connect_timeout", &self.connect_timeout)
            .field("first_byte_timeout", &self.first_byte_timeout)
            .field("between_bytes_timeout", &self.between_bytes_timeout)
            .field("t_first", &self.t_first)
            .field("t_prev", &self.t_prev)
            .field("director_state", &self.director_state)
            .field("digest", &digest_hex)
            .field("err_code", &self.err_code)
            .field("err_reason", &self.err_reason)
            .field("initial_req_body_status", &self.initial_req_body_status)
            .finish()
    }
}

// Placeholder definitions for referenced types we don't have complete info for

#[repr(C)]
pub struct ListenSock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Pool {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Worker {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PoolTask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Transport {
    _private: [u8; 0],
}

#[repr(C)]
pub struct TailqEntryReq {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Objcore {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Objhead {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Stevedore {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Director {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Vcl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct HttpConn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct VfpCtx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct VdpCtx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct AcctReq {
    _private: [u8; 0],
}

#[repr(C)]
pub struct AcctBereq {
    _private: [u8; 0],
}

#[repr(C)]
pub struct VrtPrivs {
    _private: [u8; 0],
}
