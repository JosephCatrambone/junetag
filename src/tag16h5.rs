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
static mut codedata: [uint64_t; 30] = [
    0x27c8 as libc::c_ulong,
    0x31b6 as libc::c_ulong,
    0x3859 as libc::c_ulong,
    0x569c as libc::c_ulong,
    0x6c76 as libc::c_ulong,
    0x7ddb as libc::c_ulong,
    0xaf09 as libc::c_ulong,
    0xf5a1 as libc::c_ulong,
    0xfb8b as libc::c_ulong,
    0x1cb9 as libc::c_ulong,
    0x28ca as libc::c_ulong,
    0xe8dc as libc::c_ulong,
    0x1426 as libc::c_ulong,
    0x5770 as libc::c_ulong,
    0x9253 as libc::c_ulong,
    0xb702 as libc::c_ulong,
    0x63a as libc::c_ulong,
    0x8f34 as libc::c_ulong,
    0xb4c0 as libc::c_ulong,
    0x51ec as libc::c_ulong,
    0xe6f0 as libc::c_ulong,
    0x5fa4 as libc::c_ulong,
    0xdd43 as libc::c_ulong,
    0x1aaa as libc::c_ulong,
    0xe62f as libc::c_ulong,
    0x6dbc as libc::c_ulong,
    0xb6eb as libc::c_ulong,
    0xde10 as libc::c_ulong,
    0x154d as libc::c_ulong,
    0xb57a as libc::c_ulong,
];
#[no_mangle]
pub unsafe extern "C" fn tag16h5_create() -> *mut apriltag_family_t {
    let mut tf: *mut apriltag_family_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<apriltag_family_t>() as libc::c_ulong,
    ) as *mut apriltag_family_t;
    let ref mut fresh0 = (*tf).name;
    *fresh0 = strdup(b"tag16h5\0" as *const u8 as *const libc::c_char);
    (*tf).h = 5 as libc::c_int as uint32_t;
    (*tf).ncodes = 30 as libc::c_int as uint32_t;
    let ref mut fresh1 = (*tf).codes;
    *fresh1 = codedata.as_mut_ptr();
    (*tf).nbits = 16 as libc::c_int as uint32_t;
    let ref mut fresh2 = (*tf).bit_x;
    *fresh2 = calloc(
        16 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
    ) as *mut uint32_t;
    let ref mut fresh3 = (*tf).bit_y;
    *fresh3 = calloc(
        16 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
    ) as *mut uint32_t;
    *((*tf).bit_x).offset(0 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(0 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(1 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(1 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(2 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(2 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(3 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(3 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(4 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(4 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(5 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(5 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(6 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(6 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(7 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(7 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(8 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(8 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(9 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(9 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(10 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(10 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(11 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(11 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(12 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(12 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(13 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(13 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(14 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(14 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(15 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(15 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    (*tf).width_at_border = 6 as libc::c_int;
    (*tf).total_width = 8 as libc::c_int;
    (*tf).reversed_border = 0 as libc::c_int != 0;
    return tf;
}
#[no_mangle]
pub unsafe extern "C" fn tag16h5_destroy(mut tf: *mut apriltag_family_t) {
    free((*tf).bit_x as *mut libc::c_void);
    free((*tf).bit_y as *mut libc::c_void);
    free((*tf).name as *mut libc::c_void);
    free(tf as *mut libc::c_void);
}
