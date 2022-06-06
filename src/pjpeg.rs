use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn image_u8_create(width: libc::c_uint, height: libc::c_uint) -> *mut image_u8_t;
    fn image_u8x3_create(width: libc::c_uint, height: libc::c_uint) -> *mut image_u8x3_t;
    fn pjpeg_idct_2D_nanojpeg(
        in_0: *mut int32_t,
        out: *mut uint8_t,
        outstride: uint32_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct image_u8 {
    pub width: int32_t,
    pub height: int32_t,
    pub stride: int32_t,
    pub buf: *mut uint8_t,
}
pub type image_u8_t = image_u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct image_u8x3 {
    pub width: int32_t,
    pub height: int32_t,
    pub stride: int32_t,
    pub buf: *mut uint8_t,
}
pub type image_u8x3_t = image_u8x3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pjpeg_component {
    pub width: uint32_t,
    pub height: uint32_t,
    pub stride: uint32_t,
    pub data: *mut uint8_t,
    pub id: uint8_t,
    pub hv: uint8_t,
    pub scalex: uint8_t,
    pub scaley: uint8_t,
    pub tq: uint8_t,
    pub tda: uint8_t,
}
pub type pjpeg_component_t = pjpeg_component;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pjpeg {
    pub error: libc::c_int,
    pub width: uint32_t,
    pub height: uint32_t,
    pub ncomponents: libc::c_int,
    pub components: *mut pjpeg_component_t,
}
pub type pjpeg_t = pjpeg;
pub type PJPEG_FLAGS = libc::c_uint;
pub const PJPEG_MJPEG: PJPEG_FLAGS = 2;
pub const PJPEG_STRICT: PJPEG_FLAGS = 1;
pub type PJPEG_ERROR = libc::c_uint;
pub const PJEPG_ERR_UNSUPPORTED: PJPEG_ERROR = 10;
pub const PJPEG_ERR_EOF: PJPEG_ERROR = 9;
pub const PJPEG_ERR_RESET: PJPEG_ERROR = 8;
pub const PJPEG_ERR_DRI: PJPEG_ERROR = 7;
pub const PJPEG_ERR_MISSING_DHT: PJPEG_ERROR = 6;
pub const PJPEG_ERR_SOS: PJPEG_ERROR = 5;
pub const PJPEG_ERR_DHT: PJPEG_ERROR = 4;
pub const PJPEG_ERR_SOF: PJPEG_ERROR = 3;
pub const PJPEG_ERR_DQT: PJPEG_ERROR = 2;
pub const PJPEG_ERR_FILE: PJPEG_ERROR = 1;
pub const PJPEG_OKAY: PJPEG_ERROR = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pjpeg_decode_state {
    pub error: libc::c_int,
    pub width: uint32_t,
    pub height: uint32_t,
    pub in_0: *mut uint8_t,
    pub inlen: uint32_t,
    pub flags: uint32_t,
    pub huff_codes: [[pjpeg_huffman_code; 65536]; 4],
    pub huff_codes_present: [libc::c_int; 4],
    pub qtab: [[uint8_t; 64]; 4],
    pub ncomponents: libc::c_int,
    pub components: *mut pjpeg_component_t,
    pub reset_interval: libc::c_int,
    pub reset_count: libc::c_int,
    pub reset_next: libc::c_int,
    pub debug: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pjpeg_huffman_code {
    pub nbits: uint8_t,
    pub code: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bit_decoder {
    pub in_0: *mut uint8_t,
    pub inpos: uint32_t,
    pub inlen: uint32_t,
    pub bits: uint32_t,
    pub nbits_avail: libc::c_int,
    pub error: libc::c_int,
}
static mut mjpeg_dht: [uint8_t; 420] = [
    0xff as libc::c_int as uint8_t,
    0xc4 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
    0x6 as libc::c_int as uint8_t,
    0x7 as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0x9 as libc::c_int as uint8_t,
    0xa as libc::c_int as uint8_t,
    0xb as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
    0x6 as libc::c_int as uint8_t,
    0x7 as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0x9 as libc::c_int as uint8_t,
    0xa as libc::c_int as uint8_t,
    0xb as libc::c_int as uint8_t,
    0x10 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x7d as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0x11 as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
    0x12 as libc::c_int as uint8_t,
    0x21 as libc::c_int as uint8_t,
    0x31 as libc::c_int as uint8_t,
    0x41 as libc::c_int as uint8_t,
    0x6 as libc::c_int as uint8_t,
    0x13 as libc::c_int as uint8_t,
    0x51 as libc::c_int as uint8_t,
    0x61 as libc::c_int as uint8_t,
    0x7 as libc::c_int as uint8_t,
    0x22 as libc::c_int as uint8_t,
    0x71 as libc::c_int as uint8_t,
    0x14 as libc::c_int as uint8_t,
    0x32 as libc::c_int as uint8_t,
    0x81 as libc::c_int as uint8_t,
    0x91 as libc::c_int as uint8_t,
    0xa1 as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0x23 as libc::c_int as uint8_t,
    0x42 as libc::c_int as uint8_t,
    0xb1 as libc::c_int as uint8_t,
    0xc1 as libc::c_int as uint8_t,
    0x15 as libc::c_int as uint8_t,
    0x52 as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0xf0 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x33 as libc::c_int as uint8_t,
    0x62 as libc::c_int as uint8_t,
    0x72 as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0x9 as libc::c_int as uint8_t,
    0xa as libc::c_int as uint8_t,
    0x16 as libc::c_int as uint8_t,
    0x17 as libc::c_int as uint8_t,
    0x18 as libc::c_int as uint8_t,
    0x19 as libc::c_int as uint8_t,
    0x1a as libc::c_int as uint8_t,
    0x25 as libc::c_int as uint8_t,
    0x26 as libc::c_int as uint8_t,
    0x27 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x29 as libc::c_int as uint8_t,
    0x2a as libc::c_int as uint8_t,
    0x34 as libc::c_int as uint8_t,
    0x35 as libc::c_int as uint8_t,
    0x36 as libc::c_int as uint8_t,
    0x37 as libc::c_int as uint8_t,
    0x38 as libc::c_int as uint8_t,
    0x39 as libc::c_int as uint8_t,
    0x3a as libc::c_int as uint8_t,
    0x43 as libc::c_int as uint8_t,
    0x44 as libc::c_int as uint8_t,
    0x45 as libc::c_int as uint8_t,
    0x46 as libc::c_int as uint8_t,
    0x47 as libc::c_int as uint8_t,
    0x48 as libc::c_int as uint8_t,
    0x49 as libc::c_int as uint8_t,
    0x4a as libc::c_int as uint8_t,
    0x53 as libc::c_int as uint8_t,
    0x54 as libc::c_int as uint8_t,
    0x55 as libc::c_int as uint8_t,
    0x56 as libc::c_int as uint8_t,
    0x57 as libc::c_int as uint8_t,
    0x58 as libc::c_int as uint8_t,
    0x59 as libc::c_int as uint8_t,
    0x5a as libc::c_int as uint8_t,
    0x63 as libc::c_int as uint8_t,
    0x64 as libc::c_int as uint8_t,
    0x65 as libc::c_int as uint8_t,
    0x66 as libc::c_int as uint8_t,
    0x67 as libc::c_int as uint8_t,
    0x68 as libc::c_int as uint8_t,
    0x69 as libc::c_int as uint8_t,
    0x6a as libc::c_int as uint8_t,
    0x73 as libc::c_int as uint8_t,
    0x74 as libc::c_int as uint8_t,
    0x75 as libc::c_int as uint8_t,
    0x76 as libc::c_int as uint8_t,
    0x77 as libc::c_int as uint8_t,
    0x78 as libc::c_int as uint8_t,
    0x79 as libc::c_int as uint8_t,
    0x7a as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x85 as libc::c_int as uint8_t,
    0x86 as libc::c_int as uint8_t,
    0x87 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x89 as libc::c_int as uint8_t,
    0x8a as libc::c_int as uint8_t,
    0x92 as libc::c_int as uint8_t,
    0x93 as libc::c_int as uint8_t,
    0x94 as libc::c_int as uint8_t,
    0x95 as libc::c_int as uint8_t,
    0x96 as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0x99 as libc::c_int as uint8_t,
    0x9a as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xa5 as libc::c_int as uint8_t,
    0xa6 as libc::c_int as uint8_t,
    0xa7 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xa9 as libc::c_int as uint8_t,
    0xaa as libc::c_int as uint8_t,
    0xb2 as libc::c_int as uint8_t,
    0xb3 as libc::c_int as uint8_t,
    0xb4 as libc::c_int as uint8_t,
    0xb5 as libc::c_int as uint8_t,
    0xb6 as libc::c_int as uint8_t,
    0xb7 as libc::c_int as uint8_t,
    0xb8 as libc::c_int as uint8_t,
    0xb9 as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xc2 as libc::c_int as uint8_t,
    0xc3 as libc::c_int as uint8_t,
    0xc4 as libc::c_int as uint8_t,
    0xc5 as libc::c_int as uint8_t,
    0xc6 as libc::c_int as uint8_t,
    0xc7 as libc::c_int as uint8_t,
    0xc8 as libc::c_int as uint8_t,
    0xc9 as libc::c_int as uint8_t,
    0xca as libc::c_int as uint8_t,
    0xd2 as libc::c_int as uint8_t,
    0xd3 as libc::c_int as uint8_t,
    0xd4 as libc::c_int as uint8_t,
    0xd5 as libc::c_int as uint8_t,
    0xd6 as libc::c_int as uint8_t,
    0xd7 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xe1 as libc::c_int as uint8_t,
    0xe2 as libc::c_int as uint8_t,
    0xe3 as libc::c_int as uint8_t,
    0xe4 as libc::c_int as uint8_t,
    0xe5 as libc::c_int as uint8_t,
    0xe6 as libc::c_int as uint8_t,
    0xe7 as libc::c_int as uint8_t,
    0xe8 as libc::c_int as uint8_t,
    0xe9 as libc::c_int as uint8_t,
    0xea as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0xf5 as libc::c_int as uint8_t,
    0xf6 as libc::c_int as uint8_t,
    0xf7 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
    0x11 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0x7 as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x77 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0x2 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0x11 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0x5 as libc::c_int as uint8_t,
    0x21 as libc::c_int as uint8_t,
    0x31 as libc::c_int as uint8_t,
    0x6 as libc::c_int as uint8_t,
    0x12 as libc::c_int as uint8_t,
    0x41 as libc::c_int as uint8_t,
    0x51 as libc::c_int as uint8_t,
    0x7 as libc::c_int as uint8_t,
    0x61 as libc::c_int as uint8_t,
    0x71 as libc::c_int as uint8_t,
    0x13 as libc::c_int as uint8_t,
    0x22 as libc::c_int as uint8_t,
    0x32 as libc::c_int as uint8_t,
    0x81 as libc::c_int as uint8_t,
    0x8 as libc::c_int as uint8_t,
    0x14 as libc::c_int as uint8_t,
    0x42 as libc::c_int as uint8_t,
    0x91 as libc::c_int as uint8_t,
    0xa1 as libc::c_int as uint8_t,
    0xb1 as libc::c_int as uint8_t,
    0xc1 as libc::c_int as uint8_t,
    0x9 as libc::c_int as uint8_t,
    0x23 as libc::c_int as uint8_t,
    0x33 as libc::c_int as uint8_t,
    0x52 as libc::c_int as uint8_t,
    0xf0 as libc::c_int as uint8_t,
    0x15 as libc::c_int as uint8_t,
    0x62 as libc::c_int as uint8_t,
    0x72 as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0xa as libc::c_int as uint8_t,
    0x16 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
    0x34 as libc::c_int as uint8_t,
    0xe1 as libc::c_int as uint8_t,
    0x25 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0x17 as libc::c_int as uint8_t,
    0x18 as libc::c_int as uint8_t,
    0x19 as libc::c_int as uint8_t,
    0x1a as libc::c_int as uint8_t,
    0x26 as libc::c_int as uint8_t,
    0x27 as libc::c_int as uint8_t,
    0x28 as libc::c_int as uint8_t,
    0x29 as libc::c_int as uint8_t,
    0x2a as libc::c_int as uint8_t,
    0x35 as libc::c_int as uint8_t,
    0x36 as libc::c_int as uint8_t,
    0x37 as libc::c_int as uint8_t,
    0x38 as libc::c_int as uint8_t,
    0x39 as libc::c_int as uint8_t,
    0x3a as libc::c_int as uint8_t,
    0x43 as libc::c_int as uint8_t,
    0x44 as libc::c_int as uint8_t,
    0x45 as libc::c_int as uint8_t,
    0x46 as libc::c_int as uint8_t,
    0x47 as libc::c_int as uint8_t,
    0x48 as libc::c_int as uint8_t,
    0x49 as libc::c_int as uint8_t,
    0x4a as libc::c_int as uint8_t,
    0x53 as libc::c_int as uint8_t,
    0x54 as libc::c_int as uint8_t,
    0x55 as libc::c_int as uint8_t,
    0x56 as libc::c_int as uint8_t,
    0x57 as libc::c_int as uint8_t,
    0x58 as libc::c_int as uint8_t,
    0x59 as libc::c_int as uint8_t,
    0x5a as libc::c_int as uint8_t,
    0x63 as libc::c_int as uint8_t,
    0x64 as libc::c_int as uint8_t,
    0x65 as libc::c_int as uint8_t,
    0x66 as libc::c_int as uint8_t,
    0x67 as libc::c_int as uint8_t,
    0x68 as libc::c_int as uint8_t,
    0x69 as libc::c_int as uint8_t,
    0x6a as libc::c_int as uint8_t,
    0x73 as libc::c_int as uint8_t,
    0x74 as libc::c_int as uint8_t,
    0x75 as libc::c_int as uint8_t,
    0x76 as libc::c_int as uint8_t,
    0x77 as libc::c_int as uint8_t,
    0x78 as libc::c_int as uint8_t,
    0x79 as libc::c_int as uint8_t,
    0x7a as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0x84 as libc::c_int as uint8_t,
    0x85 as libc::c_int as uint8_t,
    0x86 as libc::c_int as uint8_t,
    0x87 as libc::c_int as uint8_t,
    0x88 as libc::c_int as uint8_t,
    0x89 as libc::c_int as uint8_t,
    0x8a as libc::c_int as uint8_t,
    0x92 as libc::c_int as uint8_t,
    0x93 as libc::c_int as uint8_t,
    0x94 as libc::c_int as uint8_t,
    0x95 as libc::c_int as uint8_t,
    0x96 as libc::c_int as uint8_t,
    0x97 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0x99 as libc::c_int as uint8_t,
    0x9a as libc::c_int as uint8_t,
    0xa2 as libc::c_int as uint8_t,
    0xa3 as libc::c_int as uint8_t,
    0xa4 as libc::c_int as uint8_t,
    0xa5 as libc::c_int as uint8_t,
    0xa6 as libc::c_int as uint8_t,
    0xa7 as libc::c_int as uint8_t,
    0xa8 as libc::c_int as uint8_t,
    0xa9 as libc::c_int as uint8_t,
    0xaa as libc::c_int as uint8_t,
    0xb2 as libc::c_int as uint8_t,
    0xb3 as libc::c_int as uint8_t,
    0xb4 as libc::c_int as uint8_t,
    0xb5 as libc::c_int as uint8_t,
    0xb6 as libc::c_int as uint8_t,
    0xb7 as libc::c_int as uint8_t,
    0xb8 as libc::c_int as uint8_t,
    0xb9 as libc::c_int as uint8_t,
    0xba as libc::c_int as uint8_t,
    0xc2 as libc::c_int as uint8_t,
    0xc3 as libc::c_int as uint8_t,
    0xc4 as libc::c_int as uint8_t,
    0xc5 as libc::c_int as uint8_t,
    0xc6 as libc::c_int as uint8_t,
    0xc7 as libc::c_int as uint8_t,
    0xc8 as libc::c_int as uint8_t,
    0xc9 as libc::c_int as uint8_t,
    0xca as libc::c_int as uint8_t,
    0xd2 as libc::c_int as uint8_t,
    0xd3 as libc::c_int as uint8_t,
    0xd4 as libc::c_int as uint8_t,
    0xd5 as libc::c_int as uint8_t,
    0xd6 as libc::c_int as uint8_t,
    0xd7 as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0xda as libc::c_int as uint8_t,
    0xe2 as libc::c_int as uint8_t,
    0xe3 as libc::c_int as uint8_t,
    0xe4 as libc::c_int as uint8_t,
    0xe5 as libc::c_int as uint8_t,
    0xe6 as libc::c_int as uint8_t,
    0xe7 as libc::c_int as uint8_t,
    0xe8 as libc::c_int as uint8_t,
    0xe9 as libc::c_int as uint8_t,
    0xea as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xf4 as libc::c_int as uint8_t,
    0xf5 as libc::c_int as uint8_t,
    0xf6 as libc::c_int as uint8_t,
    0xf7 as libc::c_int as uint8_t,
    0xf8 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0xfa as libc::c_int as uint8_t,
];
#[inline]
unsafe extern "C" fn max_u8(mut a: uint8_t, mut b: uint8_t) -> uint8_t {
    return (if a as libc::c_int > b as libc::c_int {
        a as libc::c_int
    } else {
        b as libc::c_int
    }) as uint8_t;
}
static mut ZZ: [libc::c_char; 64] = [
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    16 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    17 as libc::c_int as libc::c_char,
    24 as libc::c_int as libc::c_char,
    32 as libc::c_int as libc::c_char,
    25 as libc::c_int as libc::c_char,
    18 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    5 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    19 as libc::c_int as libc::c_char,
    26 as libc::c_int as libc::c_char,
    33 as libc::c_int as libc::c_char,
    40 as libc::c_int as libc::c_char,
    48 as libc::c_int as libc::c_char,
    41 as libc::c_int as libc::c_char,
    34 as libc::c_int as libc::c_char,
    27 as libc::c_int as libc::c_char,
    20 as libc::c_int as libc::c_char,
    13 as libc::c_int as libc::c_char,
    6 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    14 as libc::c_int as libc::c_char,
    21 as libc::c_int as libc::c_char,
    28 as libc::c_int as libc::c_char,
    35 as libc::c_int as libc::c_char,
    42 as libc::c_int as libc::c_char,
    49 as libc::c_int as libc::c_char,
    56 as libc::c_int as libc::c_char,
    57 as libc::c_int as libc::c_char,
    50 as libc::c_int as libc::c_char,
    43 as libc::c_int as libc::c_char,
    36 as libc::c_int as libc::c_char,
    29 as libc::c_int as libc::c_char,
    22 as libc::c_int as libc::c_char,
    15 as libc::c_int as libc::c_char,
    23 as libc::c_int as libc::c_char,
    30 as libc::c_int as libc::c_char,
    37 as libc::c_int as libc::c_char,
    44 as libc::c_int as libc::c_char,
    51 as libc::c_int as libc::c_char,
    58 as libc::c_int as libc::c_char,
    59 as libc::c_int as libc::c_char,
    52 as libc::c_int as libc::c_char,
    45 as libc::c_int as libc::c_char,
    38 as libc::c_int as libc::c_char,
    31 as libc::c_int as libc::c_char,
    39 as libc::c_int as libc::c_char,
    46 as libc::c_int as libc::c_char,
    53 as libc::c_int as libc::c_char,
    60 as libc::c_int as libc::c_char,
    61 as libc::c_int as libc::c_char,
    54 as libc::c_int as libc::c_char,
    47 as libc::c_int as libc::c_char,
    55 as libc::c_int as libc::c_char,
    62 as libc::c_int as libc::c_char,
    63 as libc::c_int as libc::c_char,
];
#[inline]
unsafe extern "C" fn bd_ensure(mut bd: *mut bit_decoder, mut nbits: libc::c_int) {
    while (*bd).nbits_avail < nbits {
        if (*bd).inpos >= (*bd).inlen {
            printf(b"hallucinating 1s!\n\0" as *const u8 as *const libc::c_char);
            (*bd)
                .bits = (*bd).bits << 8 as libc::c_int
                | 0xff as libc::c_int as libc::c_uint;
            (*bd).nbits_avail += 8 as libc::c_int;
        } else {
            let mut nextbyte: uint8_t = *((*bd).in_0).offset((*bd).inpos as isize);
            let ref mut fresh0 = (*bd).inpos;
            *fresh0 = (*fresh0).wrapping_add(1);
            if nextbyte as libc::c_int == 0xff as libc::c_int
                && (*bd).inpos < (*bd).inlen
                && *((*bd).in_0).offset((*bd).inpos as isize) as libc::c_int
                    == 0 as libc::c_int
            {
                nextbyte = 0xff as libc::c_int as uint8_t;
                let ref mut fresh1 = (*bd).inpos;
                *fresh1 = (*fresh1).wrapping_add(1);
            }
            (*bd).bits = (*bd).bits << 8 as libc::c_int | nextbyte as libc::c_uint;
            (*bd).nbits_avail += 8 as libc::c_int;
        }
    }
}
#[inline]
unsafe extern "C" fn bd_peek_bits(
    mut bd: *mut bit_decoder,
    mut nbits: libc::c_int,
) -> uint32_t {
    bd_ensure(bd, nbits);
    return (*bd).bits >> (*bd).nbits_avail - nbits
        & (((1 as libc::c_int) << nbits) - 1 as libc::c_int) as libc::c_uint;
}
#[inline]
unsafe extern "C" fn bd_consume_bits(
    mut bd: *mut bit_decoder,
    mut nbits: libc::c_int,
) -> uint32_t {
    bd_ensure(bd, nbits);
    let mut v: uint32_t = (*bd).bits >> (*bd).nbits_avail - nbits
        & (((1 as libc::c_int) << nbits) - 1 as libc::c_int) as libc::c_uint;
    (*bd).nbits_avail -= nbits;
    return v;
}
#[inline]
unsafe extern "C" fn bd_discard_bytes(
    mut bd: *mut bit_decoder,
    mut nbytes: libc::c_int,
) {
    let ref mut fresh2 = (*bd).inpos;
    *fresh2 = (*fresh2 as libc::c_uint).wrapping_add(nbytes as libc::c_uint) as uint32_t
        as uint32_t;
}
#[inline]
unsafe extern "C" fn bd_has_more(mut bd: *mut bit_decoder) -> libc::c_int {
    return ((*bd).nbits_avail > 0 as libc::c_int || (*bd).inpos < (*bd).inlen)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn bd_discard_to_byte_boundary(mut bd: *mut bit_decoder) {
    (*bd).nbits_avail -= (*bd).nbits_avail & 7 as libc::c_int;
}
#[inline]
unsafe extern "C" fn bd_get_offset(mut bd: *mut bit_decoder) -> uint32_t {
    return ((*bd).inpos)
        .wrapping_sub(((*bd).nbits_avail / 8 as libc::c_int) as libc::c_uint);
}
unsafe extern "C" fn pjpeg_decode_buffer(
    mut pjd: *mut pjpeg_decode_state,
) -> libc::c_int {
    let mut current_block: u64;
    let mut bd: bit_decoder = bit_decoder {
        in_0: 0 as *mut uint8_t,
        inpos: 0,
        inlen: 0,
        bits: 0,
        nbits_avail: 0,
        error: 0,
    };
    memset(
        &mut bd as *mut bit_decoder as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<bit_decoder>() as libc::c_ulong,
    );
    bd.in_0 = (*pjd).in_0;
    bd.inpos = 0 as libc::c_int as uint32_t;
    bd.inlen = (*pjd).inlen;
    let mut marker_sync_skipped: libc::c_int = 0 as libc::c_int;
    let mut marker_sync_skipped_from_offset: libc::c_int = 0 as libc::c_int;
    while bd_has_more(&mut bd) != 0 {
        let mut marker_offset: uint32_t = bd_get_offset(&mut bd);
        bd_discard_to_byte_boundary(&mut bd);
        while bd_consume_bits(&mut bd, 8 as libc::c_int)
            != 0xff as libc::c_int as libc::c_uint
        {
            if marker_sync_skipped == 0 as libc::c_int {
                marker_sync_skipped_from_offset = marker_offset as libc::c_int;
            }
            marker_sync_skipped += 1;
        }
        if marker_sync_skipped != 0 {
            printf(
                b"%08x: skipped %04x bytes\n\0" as *const u8 as *const libc::c_char,
                marker_sync_skipped_from_offset,
                marker_sync_skipped,
            );
            marker_sync_skipped = 0 as libc::c_int;
        }
        let mut marker: uint8_t = bd_consume_bits(&mut bd, 8 as libc::c_int) as uint8_t;
        match marker as libc::c_int {
            216 => {
                continue;
            }
            224 => {
                current_block = 11707519660341826349;
            }
            225 => {
                current_block = 11707519660341826349;
            }
            226 => {
                current_block = 15416006029732730997;
            }
            230 => {
                current_block = 1708676011139628833;
            }
            254 => {
                current_block = 6009453772311597924;
            }
            219 => {
                let mut length_0: uint16_t = bd_consume_bits(&mut bd, 16 as libc::c_int)
                    as uint16_t;
                if (length_0 as libc::c_int - 2 as libc::c_int) % 65 as libc::c_int
                    != 0 as libc::c_int
                {
                    return PJPEG_ERR_DQT as libc::c_int;
                }
                let mut offset: libc::c_int = 0 as libc::c_int;
                while offset < length_0 as libc::c_int - 2 as libc::c_int {
                    let mut pqtq: uint8_t = bd_consume_bits(&mut bd, 8 as libc::c_int)
                        as uint8_t;
                    if pqtq as libc::c_int & 0xf0 as libc::c_int != 0 as libc::c_int
                        || pqtq as libc::c_int & 0xf as libc::c_int >= 4 as libc::c_int
                    {
                        return PJPEG_ERR_DQT as libc::c_int;
                    }
                    let mut id: uint8_t = (pqtq as libc::c_int & 3 as libc::c_int)
                        as uint8_t;
                    let mut i: libc::c_int = 0 as libc::c_int;
                    while i < 64 as libc::c_int {
                        (*pjd)
                            .qtab[id
                            as usize][i
                            as usize] = bd_consume_bits(&mut bd, 8 as libc::c_int)
                            as uint8_t;
                        i += 1;
                    }
                    offset += 65 as libc::c_int;
                }
                continue;
            }
            192 => {
                let mut length_1: uint16_t = bd_consume_bits(&mut bd, 16 as libc::c_int)
                    as uint16_t;
                let mut p: uint8_t = bd_consume_bits(&mut bd, 8 as libc::c_int)
                    as uint8_t;
                if p as libc::c_int != 8 as libc::c_int {
                    return PJPEG_ERR_SOF as libc::c_int;
                }
                (*pjd).height = bd_consume_bits(&mut bd, 16 as libc::c_int);
                (*pjd).width = bd_consume_bits(&mut bd, 16 as libc::c_int);
                let mut nf: libc::c_int = bd_consume_bits(&mut bd, 8 as libc::c_int)
                    as libc::c_int;
                if nf < 1 as libc::c_int || nf > 3 as libc::c_int {
                    return PJPEG_ERR_SOF as libc::c_int;
                }
                (*pjd).ncomponents = nf;
                let ref mut fresh3 = (*pjd).components;
                *fresh3 = calloc(
                    nf as libc::c_ulong,
                    ::std::mem::size_of::<pjpeg_component>() as libc::c_ulong,
                ) as *mut pjpeg_component_t;
                let mut i_0: libc::c_int = 0 as libc::c_int;
                while i_0 < nf {
                    (*((*pjd).components).offset(i_0 as isize))
                        .id = bd_consume_bits(&mut bd, 8 as libc::c_int) as uint8_t;
                    (*((*pjd).components).offset(i_0 as isize))
                        .hv = bd_consume_bits(&mut bd, 8 as libc::c_int) as uint8_t;
                    (*((*pjd).components).offset(i_0 as isize))
                        .scaley = ((*((*pjd).components).offset(i_0 as isize)).hv
                        as libc::c_int & 0xf as libc::c_int) as uint8_t;
                    (*((*pjd).components).offset(i_0 as isize))
                        .scalex = ((*((*pjd).components).offset(i_0 as isize)).hv
                        as libc::c_int >> 4 as libc::c_int) as uint8_t;
                    (*((*pjd).components).offset(i_0 as isize))
                        .tq = bd_consume_bits(&mut bd, 8 as libc::c_int) as uint8_t;
                    i_0 += 1;
                }
                continue;
            }
            193 => {
                current_block = 11179323709771887448;
            }
            194 => {
                current_block = 11179323709771887448;
            }
            195 => {
                current_block = 9881508461921298910;
            }
            197 => {
                current_block = 8782822656447404933;
            }
            198 => {
                current_block = 3453971947608402968;
            }
            199 => {
                current_block = 1896122132975817181;
            }
            200 => {
                current_block = 10122803448468556110;
            }
            201 => {
                current_block = 9188001830758428832;
            }
            202 => {
                current_block = 552179156102226982;
            }
            203 => {
                current_block = 7002095460045954018;
            }
            205 => {
                current_block = 16545609388216349327;
            }
            206 | 207 => {
                current_block = 1753714366925131777;
            }
            196 => {
                let mut length_2: uint16_t = bd_consume_bits(&mut bd, 16 as libc::c_int)
                    as uint16_t;
                length_2 = (length_2 as libc::c_int - 2 as libc::c_int) as uint16_t;
                while length_2 as libc::c_int > 0 as libc::c_int {
                    let mut TcTh: uint8_t = bd_consume_bits(&mut bd, 8 as libc::c_int)
                        as uint8_t;
                    length_2 = length_2.wrapping_sub(1);
                    let mut Tc: uint8_t = (TcTh as libc::c_int >> 4 as libc::c_int)
                        as uint8_t;
                    let mut Th: libc::c_int = TcTh as libc::c_int & 0xf as libc::c_int;
                    if Tc as libc::c_int >= 2 as libc::c_int || Th >= 2 as libc::c_int {
                        return PJPEG_ERR_DHT as libc::c_int;
                    }
                    let mut htidx: libc::c_int = Tc as libc::c_int * 2 as libc::c_int
                        + Th;
                    let mut L: [uint8_t; 17] = [0; 17];
                    L[0 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
                    let mut nbits: libc::c_int = 1 as libc::c_int;
                    while nbits <= 16 as libc::c_int {
                        L[nbits
                            as usize] = bd_consume_bits(&mut bd, 8 as libc::c_int)
                            as uint8_t;
                        length_2 = (length_2 as libc::c_int
                            - L[nbits as usize] as libc::c_int) as uint16_t;
                        nbits += 1;
                    }
                    length_2 = (length_2 as libc::c_int - 16 as libc::c_int) as uint16_t;
                    let mut code_pos: uint32_t = 0 as libc::c_int as uint32_t;
                    let mut nbits_0: libc::c_int = 1 as libc::c_int;
                    while nbits_0 <= 16 as libc::c_int {
                        let mut nvalues: libc::c_int = L[nbits_0 as usize]
                            as libc::c_int;
                        let mut ncodes: uint32_t = ((1 as libc::c_int)
                            << 16 as libc::c_int - nbits_0) as uint32_t;
                        let mut vi: libc::c_int = 0 as libc::c_int;
                        while vi < nvalues {
                            let mut code: uint8_t = bd_consume_bits(
                                &mut bd,
                                8 as libc::c_int,
                            ) as uint8_t;
                            if code_pos.wrapping_add(ncodes)
                                > 0xffff as libc::c_int as libc::c_uint
                            {
                                return PJPEG_ERR_DHT as libc::c_int;
                            }
                            let mut ci: libc::c_int = 0 as libc::c_int;
                            while (ci as libc::c_uint) < ncodes {
                                (*pjd)
                                    .huff_codes[htidx as usize][code_pos as usize]
                                    .nbits = nbits_0 as uint8_t;
                                (*pjd)
                                    .huff_codes[htidx as usize][code_pos as usize]
                                    .code = code;
                                code_pos = code_pos.wrapping_add(1);
                                ci += 1;
                            }
                            vi += 1;
                        }
                        nbits_0 += 1;
                    }
                    (*pjd).huff_codes_present[htidx as usize] = 1 as libc::c_int;
                }
                continue;
            }
            218 => {
                let mut length_3: uint16_t = bd_consume_bits(&mut bd, 16 as libc::c_int)
                    as uint16_t;
                let mut ns: uint8_t = bd_consume_bits(&mut bd, 8 as libc::c_int)
                    as uint8_t;
                let mut comp_idx: *mut uint8_t = calloc(
                    ns as libc::c_ulong,
                    ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
                ) as *mut uint8_t;
                let mut i_1: libc::c_int = 0 as libc::c_int;
                while i_1 < ns as libc::c_int {
                    let mut cs: uint8_t = bd_consume_bits(&mut bd, 8 as libc::c_int)
                        as uint8_t;
                    let mut found: libc::c_int = 0 as libc::c_int;
                    let mut j: libc::c_int = 0 as libc::c_int;
                    while j < (*pjd).ncomponents {
                        if cs as libc::c_int
                            == (*((*pjd).components).offset(j as isize)).id
                                as libc::c_int
                        {
                            (*((*pjd).components).offset(j as isize))
                                .tda = bd_consume_bits(&mut bd, 8 as libc::c_int)
                                as uint8_t;
                            *comp_idx.offset(i_1 as isize) = j as uint8_t;
                            found = 1 as libc::c_int;
                            break;
                        } else {
                            j += 1;
                        }
                    }
                    if found == 0 {
                        return PJPEG_ERR_SOS as libc::c_int;
                    }
                    i_1 += 1;
                }
                let mut ss: uint8_t = bd_consume_bits(&mut bd, 8 as libc::c_int)
                    as uint8_t;
                let mut se: uint8_t = bd_consume_bits(&mut bd, 8 as libc::c_int)
                    as uint8_t;
                let mut Ahl: uint8_t = bd_consume_bits(&mut bd, 8 as libc::c_int)
                    as uint8_t;
                if ss as libc::c_int != 0 as libc::c_int
                    || se as libc::c_int != 0x3f as libc::c_int
                    || Ahl as libc::c_int != 0 as libc::c_int
                {
                    return PJPEG_ERR_SOS as libc::c_int;
                }
                let mut maxmcux: libc::c_int = 0 as libc::c_int;
                let mut maxmcuy: libc::c_int = 0 as libc::c_int;
                let mut i_2: libc::c_int = 0 as libc::c_int;
                while i_2 < ns as libc::c_int {
                    let mut comp: *mut pjpeg_component = &mut *((*pjd).components)
                        .offset(*comp_idx.offset(i_2 as isize) as isize)
                        as *mut pjpeg_component_t;
                    maxmcux = max_u8(
                        maxmcux as uint8_t,
                        ((*comp).scalex as libc::c_int * 8 as libc::c_int) as uint8_t,
                    ) as libc::c_int;
                    maxmcuy = max_u8(
                        maxmcuy as uint8_t,
                        ((*comp).scaley as libc::c_int * 8 as libc::c_int) as uint8_t,
                    ) as libc::c_int;
                    i_2 += 1;
                }
                let mut mcus_x: libc::c_int = ((*pjd).width)
                    .wrapping_add(maxmcux as libc::c_uint)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_div(maxmcux as libc::c_uint) as libc::c_int;
                let mut mcus_y: libc::c_int = ((*pjd).height)
                    .wrapping_add(maxmcuy as libc::c_uint)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    .wrapping_div(maxmcuy as libc::c_uint) as libc::c_int;
                let mut i_3: libc::c_int = 0 as libc::c_int;
                while i_3 < ns as libc::c_int {
                    let mut comp_0: *mut pjpeg_component = &mut *((*pjd).components)
                        .offset(*comp_idx.offset(i_3 as isize) as isize)
                        as *mut pjpeg_component_t;
                    (*comp_0)
                        .width = (mcus_x * (*comp_0).scalex as libc::c_int
                        * 8 as libc::c_int) as uint32_t;
                    (*comp_0)
                        .height = (mcus_y * (*comp_0).scaley as libc::c_int
                        * 8 as libc::c_int) as uint32_t;
                    (*comp_0).stride = (*comp_0).width;
                    let mut alignment: libc::c_int = 32 as libc::c_int;
                    if ((*comp_0).stride).wrapping_rem(alignment as libc::c_uint)
                        != 0 as libc::c_int as libc::c_uint
                    {
                        let ref mut fresh4 = (*comp_0).stride;
                        *fresh4 = (*fresh4 as libc::c_uint)
                            .wrapping_add(
                                (alignment as libc::c_uint)
                                    .wrapping_sub(
                                        ((*comp_0).stride).wrapping_rem(alignment as libc::c_uint),
                                    ),
                            ) as uint32_t as uint32_t;
                    }
                    let ref mut fresh5 = (*comp_0).data;
                    *fresh5 = calloc(
                        ((*comp_0).height).wrapping_mul((*comp_0).stride)
                            as libc::c_ulong,
                        1 as libc::c_int as libc::c_ulong,
                    ) as *mut uint8_t;
                    i_3 += 1;
                }
                let mut dcpred: *mut int32_t = calloc(
                    ns as libc::c_ulong,
                    ::std::mem::size_of::<int32_t>() as libc::c_ulong,
                ) as *mut int32_t;
                (*pjd).reset_count = 0 as libc::c_int;
                let mut mcu_y: libc::c_int = 0 as libc::c_int;
                while mcu_y < mcus_y {
                    let mut mcu_x: libc::c_int = 0 as libc::c_int;
                    while mcu_x < mcus_x {
                        if (*pjd).reset_interval > 0 as libc::c_int
                            && (*pjd).reset_count == (*pjd).reset_interval
                        {
                            bd_discard_to_byte_boundary(&mut bd);
                            loop {
                                let mut value: int32_t = bd_consume_bits(
                                    &mut bd,
                                    8 as libc::c_int,
                                ) as int32_t;
                                if bd.inpos > bd.inlen {
                                    return PJPEG_ERR_EOF as libc::c_int;
                                }
                                if value == 0xff as libc::c_int {
                                    break;
                                }
                                printf(b"RST SYNC\n\0" as *const u8 as *const libc::c_char);
                            }
                            let mut marker_32: int32_t = bd_consume_bits(
                                &mut bd,
                                8 as libc::c_int,
                            ) as int32_t;
                            if marker_32 != 0xd0 as libc::c_int + (*pjd).reset_next {
                                return PJPEG_ERR_RESET as libc::c_int;
                            }
                            (*pjd).reset_count = 0 as libc::c_int;
                            (*pjd)
                                .reset_next = (*pjd).reset_next + 1 as libc::c_int
                                & 0x7 as libc::c_int;
                            memset(
                                dcpred as *mut libc::c_void,
                                0 as libc::c_int,
                                ::std::mem::size_of::<int32_t>() as libc::c_ulong,
                            );
                        }
                        let mut nsidx: libc::c_int = 0 as libc::c_int;
                        while nsidx < ns as libc::c_int {
                            let mut comp_1: *mut pjpeg_component = &mut *((*pjd)
                                .components)
                                .offset(*comp_idx.offset(nsidx as isize) as isize)
                                as *mut pjpeg_component_t;
                            let mut block: [int32_t; 64] = [0; 64];
                            let mut qtabidx: libc::c_int = (*comp_1).tq as libc::c_int;
                            let mut sby: libc::c_int = 0 as libc::c_int;
                            while sby < (*comp_1).scaley as libc::c_int {
                                let mut sbx: libc::c_int = 0 as libc::c_int;
                                while sbx < (*comp_1).scalex as libc::c_int {
                                    memset(
                                        block.as_mut_ptr() as *mut libc::c_void,
                                        0 as libc::c_int,
                                        ::std::mem::size_of::<[int32_t; 64]>() as libc::c_ulong,
                                    );
                                    let mut dc_huff_table_idx: libc::c_int = (*comp_1).tda
                                        as libc::c_int >> 4 as libc::c_int;
                                    let mut ac_huff_table_idx: libc::c_int = 2 as libc::c_int
                                        + ((*comp_1).tda as libc::c_int & 0xf as libc::c_int);
                                    if (*pjd).huff_codes_present[dc_huff_table_idx as usize]
                                        == 0
                                        || (*pjd).huff_codes_present[ac_huff_table_idx as usize]
                                            == 0
                                    {
                                        return PJPEG_ERR_MISSING_DHT as libc::c_int;
                                    }
                                    let mut next16: uint32_t = bd_peek_bits(
                                        &mut bd,
                                        16 as libc::c_int,
                                    );
                                    let mut huff_code: *mut pjpeg_huffman_code = &mut *(*((*pjd)
                                        .huff_codes)
                                        .as_mut_ptr()
                                        .offset(dc_huff_table_idx as isize))
                                        .as_mut_ptr()
                                        .offset(next16 as isize) as *mut pjpeg_huffman_code;
                                    bd_consume_bits(&mut bd, (*huff_code).nbits as libc::c_int);
                                    let mut ssss: libc::c_int = (*huff_code).code as libc::c_int
                                        & 0xf as libc::c_int;
                                    let mut value_0: int32_t = bd_consume_bits(&mut bd, ssss)
                                        as int32_t;
                                    if value_0 & (1 as libc::c_int) << ssss - 1 as libc::c_int
                                        == 0 as libc::c_int
                                    {
                                        value_0 += (-(1 as libc::c_int) << ssss) + 1 as libc::c_int;
                                    }
                                    let ref mut fresh6 = *dcpred.offset(nsidx as isize);
                                    *fresh6 += value_0;
                                    block[0 as libc::c_int
                                        as usize] = *dcpred.offset(nsidx as isize)
                                        * (*pjd).qtab[qtabidx as usize][0 as libc::c_int as usize]
                                            as libc::c_int;
                                    let mut coeff: libc::c_int = 1 as libc::c_int;
                                    while coeff < 64 as libc::c_int {
                                        let mut next16_0: uint32_t = bd_peek_bits(
                                            &mut bd,
                                            16 as libc::c_int,
                                        );
                                        let mut huff_code_0: *mut pjpeg_huffman_code = &mut *(*((*pjd)
                                            .huff_codes)
                                            .as_mut_ptr()
                                            .offset(ac_huff_table_idx as isize))
                                            .as_mut_ptr()
                                            .offset(next16_0 as isize) as *mut pjpeg_huffman_code;
                                        bd_consume_bits(
                                            &mut bd,
                                            (*huff_code_0).nbits as libc::c_int,
                                        );
                                        if (*huff_code_0).code as libc::c_int == 0 as libc::c_int {
                                            break;
                                        }
                                        let mut rrrr: libc::c_int = (*huff_code_0).code
                                            as libc::c_int >> 4 as libc::c_int;
                                        let mut ssss_0: libc::c_int = (*huff_code_0).code
                                            as libc::c_int & 0xf as libc::c_int;
                                        let mut value_1: int32_t = bd_consume_bits(&mut bd, ssss_0)
                                            as int32_t;
                                        if value_1 & (1 as libc::c_int) << ssss_0 - 1 as libc::c_int
                                            == 0 as libc::c_int
                                        {
                                            value_1
                                                += (-(1 as libc::c_int) << ssss_0) + 1 as libc::c_int;
                                        }
                                        coeff += rrrr;
                                        block[ZZ[coeff as usize] as libc::c_int
                                            as usize] = value_1
                                            * (*pjd).qtab[qtabidx as usize][coeff as usize]
                                                as libc::c_int;
                                        coeff += 1;
                                    }
                                    let mut comp_x: uint32_t = ((mcu_x
                                        * (*comp_1).scalex as libc::c_int + sbx) * 8 as libc::c_int)
                                        as uint32_t;
                                    let mut comp_y: uint32_t = ((mcu_y
                                        * (*comp_1).scaley as libc::c_int + sby) * 8 as libc::c_int)
                                        as uint32_t;
                                    let mut dataidx: uint32_t = comp_y
                                        .wrapping_mul((*comp_1).stride)
                                        .wrapping_add(comp_x);
                                    pjpeg_idct_2D_nanojpeg(
                                        block.as_mut_ptr(),
                                        &mut *((*comp_1).data).offset(dataidx as isize),
                                        (*comp_1).stride,
                                    );
                                    sbx += 1;
                                }
                                sby += 1;
                            }
                            nsidx += 1;
                        }
                        let ref mut fresh7 = (*pjd).reset_count;
                        *fresh7 += 1;
                        mcu_x += 1;
                    }
                    mcu_y += 1;
                }
                free(dcpred as *mut libc::c_void);
                free(comp_idx as *mut libc::c_void);
                continue;
            }
            217 => {
                break;
            }
            221 => {
                let mut length_4: uint16_t = bd_consume_bits(&mut bd, 16 as libc::c_int)
                    as uint16_t;
                if length_4 as libc::c_int != 4 as libc::c_int {
                    return PJPEG_ERR_DRI as libc::c_int;
                }
                (*pjd)
                    .reset_interval = bd_consume_bits(&mut bd, 16 as libc::c_int)
                    as libc::c_int;
                continue;
            }
            _ => {
                printf(
                    b"pjepg: Unknown marker %02x at offset %04x\n\0" as *const u8
                        as *const libc::c_char,
                    marker as libc::c_int,
                    marker_offset,
                );
                let mut length_5: uint16_t = bd_consume_bits(&mut bd, 16 as libc::c_int)
                    as uint16_t;
                bd_discard_bytes(&mut bd, length_5 as libc::c_int - 2 as libc::c_int);
                continue;
            }
        }
        match current_block {
            11707519660341826349 => {
                current_block = 15416006029732730997;
            }
            11179323709771887448 => {
                current_block = 9881508461921298910;
            }
            _ => {}
        }
        match current_block {
            15416006029732730997 => {
                current_block = 1708676011139628833;
            }
            9881508461921298910 => {
                current_block = 8782822656447404933;
            }
            _ => {}
        }
        match current_block {
            1708676011139628833 => {
                current_block = 6009453772311597924;
            }
            8782822656447404933 => {
                current_block = 3453971947608402968;
            }
            _ => {}
        }
        match current_block {
            6009453772311597924 => {
                let mut length: uint16_t = bd_consume_bits(&mut bd, 16 as libc::c_int)
                    as uint16_t;
                bd_discard_bytes(&mut bd, length as libc::c_int - 2 as libc::c_int);
                continue;
            }
            3453971947608402968 => {
                current_block = 1896122132975817181;
            }
            _ => {}
        }
        match current_block {
            1896122132975817181 => {
                current_block = 10122803448468556110;
            }
            _ => {}
        }
        match current_block {
            10122803448468556110 => {
                current_block = 9188001830758428832;
            }
            _ => {}
        }
        match current_block {
            9188001830758428832 => {
                current_block = 552179156102226982;
            }
            _ => {}
        }
        match current_block {
            552179156102226982 => {
                current_block = 7002095460045954018;
            }
            _ => {}
        }
        match current_block {
            7002095460045954018 => {
                current_block = 16545609388216349327;
            }
            _ => {}
        }
        match current_block {
            16545609388216349327 => {}
            _ => {}
        }
        printf(
            b"pjepg.c: unsupported JPEG type %02x\n\0" as *const u8
                as *const libc::c_char,
            marker as libc::c_int,
        );
        return PJEPG_ERR_UNSUPPORTED as libc::c_int;
    }
    return PJPEG_OKAY as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pjpeg_destroy(mut pj: *mut pjpeg_t) {
    if pj.is_null() {
        return;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*pj).ncomponents {
        free((*((*pj).components).offset(i as isize)).data as *mut libc::c_void);
        i += 1;
    }
    free((*pj).components as *mut libc::c_void);
    free(pj as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn pjpeg_to_u8_baseline(mut pj: *mut pjpeg_t) -> *mut image_u8_t {
    let mut comp: *mut pjpeg_component_t = &mut *((*pj).components)
        .offset(0 as libc::c_int as isize) as *mut pjpeg_component_t;
    let mut im: *mut image_u8_t = image_u8_create((*pj).width, (*pj).height);
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < (*im).height {
        memcpy(
            &mut *((*im).buf).offset((y * (*im).stride) as isize) as *mut uint8_t
                as *mut libc::c_void,
            &mut *((*comp).data)
                .offset((y as libc::c_uint).wrapping_mul((*comp).stride) as isize)
                as *mut uint8_t as *const libc::c_void,
            (*pj).width as libc::c_ulong,
        );
        y += 1;
    }
    return im;
}
#[inline]
unsafe extern "C" fn clampd(mut v: libc::c_double) -> uint8_t {
    if v < 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int as uint8_t;
    }
    if v > 255 as libc::c_int as libc::c_double {
        return 255 as libc::c_int as uint8_t;
    }
    return v as uint8_t;
}
#[inline]
unsafe extern "C" fn clamp_u8(mut v: int32_t) -> uint8_t {
    if v < 0 as libc::c_int {
        return 0 as libc::c_int as uint8_t;
    }
    if v > 255 as libc::c_int {
        return 255 as libc::c_int as uint8_t;
    }
    return v as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn pjpeg_to_u8x3_baseline(
    mut pj: *mut pjpeg_t,
) -> *mut image_u8x3_t {
    let mut Y: *mut pjpeg_component_t = &mut *((*pj).components)
        .offset(0 as libc::c_int as isize) as *mut pjpeg_component_t;
    let mut Cb: *mut pjpeg_component_t = &mut *((*pj).components)
        .offset(1 as libc::c_int as isize) as *mut pjpeg_component_t;
    let mut Cr: *mut pjpeg_component_t = &mut *((*pj).components)
        .offset(2 as libc::c_int as isize) as *mut pjpeg_component_t;
    let mut Cb_factor_y: libc::c_int = ((*Y).height).wrapping_div((*Cb).height)
        as libc::c_int;
    let mut Cb_factor_x: libc::c_int = ((*Y).width).wrapping_div((*Cb).width)
        as libc::c_int;
    let mut Cr_factor_y: libc::c_int = ((*Y).height).wrapping_div((*Cr).height)
        as libc::c_int;
    let mut Cr_factor_x: libc::c_int = ((*Y).width).wrapping_div((*Cr).width)
        as libc::c_int;
    let mut im: *mut image_u8x3_t = image_u8x3_create((*pj).width, (*pj).height);
    if Cr_factor_y == 1 as libc::c_int && Cr_factor_x == 1 as libc::c_int
        && Cb_factor_y == 1 as libc::c_int && Cb_factor_x == 1 as libc::c_int
    {
        let mut y: libc::c_int = 0 as libc::c_int;
        while (y as libc::c_uint) < (*pj).height {
            let mut x: libc::c_int = 0 as libc::c_int;
            while (x as libc::c_uint) < (*pj).width {
                let mut y_val: int32_t = *((*Y).data)
                    .offset(
                        (y as libc::c_uint)
                            .wrapping_mul((*Y).stride)
                            .wrapping_add(x as libc::c_uint) as isize,
                    ) as libc::c_int * 65536 as libc::c_int;
                let mut cb_val: int32_t = *((*Cb).data)
                    .offset(
                        (y as libc::c_uint)
                            .wrapping_mul((*Cb).stride)
                            .wrapping_add(x as libc::c_uint) as isize,
                    ) as libc::c_int - 128 as libc::c_int;
                let mut cr_val: int32_t = *((*Cr).data)
                    .offset(
                        (y as libc::c_uint)
                            .wrapping_mul((*Cr).stride)
                            .wrapping_add(x as libc::c_uint) as isize,
                    ) as libc::c_int - 128 as libc::c_int;
                let mut r_val: int32_t = y_val + 91881 as libc::c_int * cr_val;
                let mut g_val: int32_t = y_val + -(22554 as libc::c_int) * cb_val
                    - 46802 as libc::c_int * cr_val;
                let mut b_val: int32_t = y_val + 116130 as libc::c_int * cb_val;
                *((*im).buf)
                    .offset(
                        (y * (*im).stride + 3 as libc::c_int * x + 0 as libc::c_int)
                            as isize,
                    ) = clamp_u8(r_val >> 16 as libc::c_int);
                *((*im).buf)
                    .offset(
                        (y * (*im).stride + 3 as libc::c_int * x + 1 as libc::c_int)
                            as isize,
                    ) = clamp_u8(g_val >> 16 as libc::c_int);
                *((*im).buf)
                    .offset(
                        (y * (*im).stride + 3 as libc::c_int * x + 2 as libc::c_int)
                            as isize,
                    ) = clamp_u8(b_val >> 16 as libc::c_int);
                x += 1;
            }
            y += 1;
        }
    } else if Cb_factor_y == Cr_factor_y && Cb_factor_x == Cr_factor_x {
        let mut by: libc::c_int = 0 as libc::c_int;
        while (by as libc::c_uint)
            < ((*pj).height).wrapping_div(Cb_factor_y as libc::c_uint)
        {
            let mut bx: libc::c_int = 0 as libc::c_int;
            while (bx as libc::c_uint)
                < ((*pj).width).wrapping_div(Cb_factor_x as libc::c_uint)
            {
                let mut cb_val_0: int32_t = *((*Cb).data)
                    .offset(
                        (by as libc::c_uint)
                            .wrapping_mul((*Cb).stride)
                            .wrapping_add(bx as libc::c_uint) as isize,
                    ) as libc::c_int - 128 as libc::c_int;
                let mut cr_val_0: int32_t = *((*Cr).data)
                    .offset(
                        (by as libc::c_uint)
                            .wrapping_mul((*Cr).stride)
                            .wrapping_add(bx as libc::c_uint) as isize,
                    ) as libc::c_int - 128 as libc::c_int;
                let mut r0: int32_t = 91881 as libc::c_int * cr_val_0;
                let mut g0: int32_t = -(22554 as libc::c_int) * cb_val_0
                    - 46802 as libc::c_int * cr_val_0;
                let mut b0: int32_t = 116130 as libc::c_int * cb_val_0;
                let mut dy: libc::c_int = 0 as libc::c_int;
                while dy < Cb_factor_y {
                    let mut y_0: libc::c_int = by * Cb_factor_y + dy;
                    let mut dx: libc::c_int = 0 as libc::c_int;
                    while dx < Cb_factor_x {
                        let mut x_0: libc::c_int = bx * Cb_factor_x + dx;
                        let mut y_val_0: int32_t = *((*Y).data)
                            .offset(
                                (y_0 as libc::c_uint)
                                    .wrapping_mul((*Y).stride)
                                    .wrapping_add(x_0 as libc::c_uint) as isize,
                            ) as libc::c_int * 65536 as libc::c_int;
                        let mut r_val_0: int32_t = r0 + y_val_0;
                        let mut g_val_0: int32_t = g0 + y_val_0;
                        let mut b_val_0: int32_t = b0 + y_val_0;
                        *((*im).buf)
                            .offset(
                                (y_0 * (*im).stride + 3 as libc::c_int * x_0
                                    + 0 as libc::c_int) as isize,
                            ) = clamp_u8(r_val_0 >> 16 as libc::c_int);
                        *((*im).buf)
                            .offset(
                                (y_0 * (*im).stride + 3 as libc::c_int * x_0
                                    + 1 as libc::c_int) as isize,
                            ) = clamp_u8(g_val_0 >> 16 as libc::c_int);
                        *((*im).buf)
                            .offset(
                                (y_0 * (*im).stride + 3 as libc::c_int * x_0
                                    + 2 as libc::c_int) as isize,
                            ) = clamp_u8(b_val_0 >> 16 as libc::c_int);
                        dx += 1;
                    }
                    dy += 1;
                }
                bx += 1;
            }
            by += 1;
        }
    } else {
        let mut y_1: libc::c_int = 0 as libc::c_int;
        while (y_1 as libc::c_uint) < (*pj).height {
            let mut x_1: libc::c_int = 0 as libc::c_int;
            while (x_1 as libc::c_uint) < (*pj).width {
                let mut y_val_1: int32_t = *((*Y).data)
                    .offset(
                        (y_1 as libc::c_uint)
                            .wrapping_mul((*Y).stride)
                            .wrapping_add(x_1 as libc::c_uint) as isize,
                    ) as int32_t;
                let mut cb_val_1: int32_t = *((*Cb).data)
                    .offset(
                        ((y_1 / Cb_factor_y) as libc::c_uint)
                            .wrapping_mul((*Cb).stride)
                            .wrapping_add((x_1 / Cb_factor_x) as libc::c_uint) as isize,
                    ) as libc::c_int - 128 as libc::c_int;
                let mut cr_val_1: int32_t = *((*Cr).data)
                    .offset(
                        ((y_1 / Cr_factor_y) as libc::c_uint)
                            .wrapping_mul((*Cr).stride)
                            .wrapping_add((x_1 / Cr_factor_x) as libc::c_uint) as isize,
                    ) as libc::c_int - 128 as libc::c_int;
                let mut r_val_1: uint8_t = clampd(
                    y_val_1 as libc::c_double + 1.402f64 * cr_val_1 as libc::c_double,
                );
                let mut g_val_1: uint8_t = clampd(
                    y_val_1 as libc::c_double - 0.34414f64 * cb_val_1 as libc::c_double
                        - 0.71414f64 * cr_val_1 as libc::c_double,
                );
                let mut b_val_1: uint8_t = clampd(
                    y_val_1 as libc::c_double + 1.772f64 * cb_val_1 as libc::c_double,
                );
                *((*im).buf)
                    .offset(
                        (y_1 * (*im).stride + 3 as libc::c_int * x_1 + 0 as libc::c_int)
                            as isize,
                    ) = r_val_1;
                *((*im).buf)
                    .offset(
                        (y_1 * (*im).stride + 3 as libc::c_int * x_1 + 1 as libc::c_int)
                            as isize,
                    ) = g_val_1;
                *((*im).buf)
                    .offset(
                        (y_1 * (*im).stride + 3 as libc::c_int * x_1 + 2 as libc::c_int)
                            as isize,
                    ) = b_val_1;
                x_1 += 1;
            }
            y_1 += 1;
        }
    }
    return im;
}
#[no_mangle]
pub unsafe extern "C" fn pjpeg_create_from_file(
    mut path: *const libc::c_char,
    mut flags: uint32_t,
    mut error: *mut libc::c_int,
) -> *mut pjpeg_t {
    let mut f: *mut FILE = fopen(path, b"r\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        return 0 as *mut pjpeg_t;
    }
    fseek(f, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
    let mut buflen: libc::c_long = ftell(f);
    let mut buf: *mut uint8_t = malloc(buflen as libc::c_ulong) as *mut uint8_t;
    fseek(f, 0 as libc::c_int as libc::c_long, 0 as libc::c_int);
    let mut res: libc::c_int = fread(
        buf as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        buflen as libc::c_ulong,
        f,
    ) as libc::c_int;
    fclose(f);
    if res as libc::c_long != buflen {
        free(buf as *mut libc::c_void);
        if !error.is_null() {
            *error = PJPEG_ERR_FILE as libc::c_int;
        }
        return 0 as *mut pjpeg_t;
    }
    let mut pj: *mut pjpeg_t = pjpeg_create_from_buffer(
        buf,
        buflen as libc::c_int,
        flags,
        error,
    );
    free(buf as *mut libc::c_void);
    return pj;
}
#[no_mangle]
pub unsafe extern "C" fn pjpeg_create_from_buffer(
    mut buf: *mut uint8_t,
    mut buflen: libc::c_int,
    mut flags: uint32_t,
    mut error: *mut libc::c_int,
) -> *mut pjpeg_t {
    let mut pjd: pjpeg_decode_state = pjpeg_decode_state {
        error: 0,
        width: 0,
        height: 0,
        in_0: 0 as *mut uint8_t,
        inlen: 0,
        flags: 0,
        huff_codes: [[pjpeg_huffman_code {
            nbits: 0,
            code: 0,
        }; 65536]; 4],
        huff_codes_present: [0; 4],
        qtab: [[0; 64]; 4],
        ncomponents: 0,
        components: 0 as *mut pjpeg_component_t,
        reset_interval: 0,
        reset_count: 0,
        reset_next: 0,
        debug: 0,
    };
    memset(
        &mut pjd as *mut pjpeg_decode_state as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<pjpeg_decode_state>() as libc::c_ulong,
    );
    if flags & PJPEG_MJPEG as libc::c_int as libc::c_uint != 0 {
        pjd.in_0 = mjpeg_dht.as_mut_ptr();
        pjd.inlen = ::std::mem::size_of::<[uint8_t; 420]>() as libc::c_ulong as uint32_t;
        let mut result: libc::c_int = pjpeg_decode_buffer(&mut pjd);
    }
    pjd.in_0 = buf;
    pjd.inlen = buflen as uint32_t;
    pjd.flags = flags;
    let mut result_0: libc::c_int = pjpeg_decode_buffer(&mut pjd);
    if !error.is_null() {
        *error = result_0;
    }
    if result_0 != 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < pjd.ncomponents {
            free((*(pjd.components).offset(i as isize)).data as *mut libc::c_void);
            i += 1;
        }
        free(pjd.components as *mut libc::c_void);
        return 0 as *mut pjpeg_t;
    }
    let mut pj: *mut pjpeg_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<pjpeg_t>() as libc::c_ulong,
    ) as *mut pjpeg_t;
    (*pj).width = pjd.width;
    (*pj).height = pjd.height;
    (*pj).ncomponents = pjd.ncomponents;
    let ref mut fresh8 = (*pj).components;
    *fresh8 = pjd.components;
    return pj;
}
