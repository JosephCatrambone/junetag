use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct apriltag_family {
    pub ncodes: uint32_t,
    pub codes: *mut uint64_t,
    pub width_at_border: libc::c_int,
    pub total_width: libc::c_int,
    pub reversed_border: bool,
    pub nbits: uint32_t,
    pub bit_x: *mut uint32_t,
    pub bit_y: *mut uint32_t,
    pub h: uint32_t,
    pub name: *mut libc::c_char,
    pub impl_0: *mut libc::c_void,
}
pub type apriltag_family_t = apriltag_family;
static mut codedata: [uint64_t; 38] = [
    0x157863 as libc::c_ulong,
    0x47e28 as libc::c_ulong,
    0x1383ed as libc::c_ulong,
    0x953c as libc::c_ulong,
    0xda68b as libc::c_ulong,
    0x1cac50 as libc::c_ulong,
    0xbb215 as libc::c_ulong,
    0x16ceee as libc::c_ulong,
    0x5d4b3 as libc::c_ulong,
    0x1ff751 as libc::c_ulong,
    0xefd16 as libc::c_ulong,
    0x72b3e as libc::c_ulong,
    0x163103 as libc::c_ulong,
    0x106e56 as libc::c_ulong,
    0x1996b9 as libc::c_ulong,
    0xc0234 as libc::c_ulong,
    0x624d2 as libc::c_ulong,
    0x1fa985 as libc::c_ulong,
    0x344a5 as libc::c_ulong,
    0x762fb as libc::c_ulong,
    0x19e92b as libc::c_ulong,
    0x43755 as libc::c_ulong,
    0x1a4f4 as libc::c_ulong,
    0x10fad8 as libc::c_ulong,
    0x1b52 as libc::c_ulong,
    0x17e59f as libc::c_ulong,
    0xe6f70 as libc::c_ulong,
    0xed47a as libc::c_ulong,
    0xc9931 as libc::c_ulong,
    0x14df2 as libc::c_ulong,
    0xa06f1 as libc::c_ulong,
    0xe5041 as libc::c_ulong,
    0x12ec03 as libc::c_ulong,
    0x16724e as libc::c_ulong,
    0xaf1a5 as libc::c_ulong,
    0x8a8ac as libc::c_ulong,
    0x15b39 as libc::c_ulong,
    0x1ec1e3 as libc::c_ulong,
];
#[no_mangle]
pub unsafe extern "C" fn tagCircle21h7_create() -> *mut apriltag_family_t {
    let mut tf: *mut apriltag_family_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<apriltag_family_t>() as libc::c_ulong,
    ) as *mut apriltag_family_t;
    let ref mut fresh0 = (*tf).name;
    *fresh0 = strdup(b"tagCircle21h7\0" as *const u8 as *const libc::c_char);
    (*tf).h = 7 as libc::c_int as uint32_t;
    (*tf).ncodes = 38 as libc::c_int as uint32_t;
    let ref mut fresh1 = (*tf).codes;
    *fresh1 = codedata.as_mut_ptr();
    (*tf).nbits = 21 as libc::c_int as uint32_t;
    let ref mut fresh2 = (*tf).bit_x;
    *fresh2 = calloc(
        21 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
    ) as *mut uint32_t;
    let ref mut fresh3 = (*tf).bit_y;
    *fresh3 = calloc(
        21 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
    ) as *mut uint32_t;
    *((*tf).bit_x).offset(0 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(0 as libc::c_int as isize) = -(2 as libc::c_int) as uint32_t;
    *((*tf).bit_x).offset(1 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(1 as libc::c_int as isize) = -(2 as libc::c_int) as uint32_t;
    *((*tf).bit_x).offset(2 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(2 as libc::c_int as isize) = -(2 as libc::c_int) as uint32_t;
    *((*tf).bit_x).offset(3 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(3 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(4 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(4 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(5 as libc::c_int as isize) = 6 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(5 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(6 as libc::c_int as isize) = 6 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(6 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(7 as libc::c_int as isize) = 6 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(7 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(8 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(8 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(9 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(9 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(10 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(10 as libc::c_int as isize) = 6 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(11 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(11 as libc::c_int as isize) = 6 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(12 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(12 as libc::c_int as isize) = 6 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(13 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(13 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(14 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(14 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(15 as libc::c_int as isize) = -(2 as libc::c_int) as uint32_t;
    *((*tf).bit_y).offset(15 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(16 as libc::c_int as isize) = -(2 as libc::c_int) as uint32_t;
    *((*tf).bit_y).offset(16 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(17 as libc::c_int as isize) = -(2 as libc::c_int) as uint32_t;
    *((*tf).bit_y).offset(17 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(18 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(18 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(19 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(19 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(20 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(20 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    (*tf).width_at_border = 5 as libc::c_int;
    (*tf).total_width = 9 as libc::c_int;
    (*tf).reversed_border = 1 as libc::c_int != 0;
    return tf;
}
#[no_mangle]
pub unsafe extern "C" fn tagCircle21h7_destroy(mut tf: *mut apriltag_family_t) {
    free((*tf).bit_x as *mut libc::c_void);
    free((*tf).bit_y as *mut libc::c_void);
    free((*tf).name as *mut libc::c_void);
    free(tf as *mut libc::c_void);
}
