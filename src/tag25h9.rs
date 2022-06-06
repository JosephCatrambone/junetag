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
static mut codedata: [uint64_t; 35] = [
    0x156f1f4 as libc::c_ulong,
    0x1f28cd5 as libc::c_ulong,
    0x16ce32c as libc::c_ulong,
    0x1ea379c as libc::c_ulong,
    0x1390f89 as libc::c_ulong,
    0x34fad0 as libc::c_ulong,
    0x7dcdb5 as libc::c_ulong,
    0x119ba95 as libc::c_ulong,
    0x1ae9daa as libc::c_ulong,
    0xdf02aa as libc::c_ulong,
    0x82fc15 as libc::c_ulong,
    0x465123 as libc::c_ulong,
    0xceee98 as libc::c_ulong,
    0x1f17260 as libc::c_ulong,
    0x14429cd as libc::c_ulong,
    0x17248a8 as libc::c_ulong,
    0x16ad452 as libc::c_ulong,
    0x9670ad as libc::c_ulong,
    0x16f65b2 as libc::c_ulong,
    0xb8322b as libc::c_ulong,
    0x5d715b as libc::c_ulong,
    0x1a1c7e7 as libc::c_ulong,
    0xd7890d as libc::c_ulong,
    0x1813522 as libc::c_ulong,
    0x1c9c611 as libc::c_ulong,
    0x99e4a4 as libc::c_ulong,
    0x855234 as libc::c_ulong,
    0x17b81c0 as libc::c_ulong,
    0xc294bb as libc::c_ulong,
    0x89fae3 as libc::c_ulong,
    0x44df5f as libc::c_ulong,
    0x1360159 as libc::c_ulong,
    0xec31e8 as libc::c_ulong,
    0x1bcc0f6 as libc::c_ulong,
    0xa64f8d as libc::c_ulong,
];
#[no_mangle]
pub unsafe extern "C" fn tag25h9_create() -> *mut apriltag_family_t {
    let mut tf: *mut apriltag_family_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<apriltag_family_t>() as libc::c_ulong,
    ) as *mut apriltag_family_t;
    let ref mut fresh0 = (*tf).name;
    *fresh0 = strdup(b"tag25h9\0" as *const u8 as *const libc::c_char);
    (*tf).h = 9 as libc::c_int as uint32_t;
    (*tf).ncodes = 35 as libc::c_int as uint32_t;
    let ref mut fresh1 = (*tf).codes;
    *fresh1 = codedata.as_mut_ptr();
    (*tf).nbits = 25 as libc::c_int as uint32_t;
    let ref mut fresh2 = (*tf).bit_x;
    *fresh2 = calloc(
        25 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
    ) as *mut uint32_t;
    let ref mut fresh3 = (*tf).bit_y;
    *fresh3 = calloc(
        25 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
    ) as *mut uint32_t;
    *((*tf).bit_x).offset(0 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(0 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(1 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(1 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(2 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(2 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(3 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(3 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(4 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(4 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(5 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(5 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(6 as libc::c_int as isize) = 5 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(6 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(7 as libc::c_int as isize) = 5 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(7 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(8 as libc::c_int as isize) = 5 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(8 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(9 as libc::c_int as isize) = 5 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(9 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(10 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(10 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(11 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(11 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(12 as libc::c_int as isize) = 5 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(12 as libc::c_int as isize) = 5 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(13 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(13 as libc::c_int as isize) = 5 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(14 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(14 as libc::c_int as isize) = 5 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(15 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(15 as libc::c_int as isize) = 5 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(16 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(16 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(17 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(17 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(18 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(18 as libc::c_int as isize) = 5 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(19 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(19 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(20 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(20 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(21 as libc::c_int as isize) = 1 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(21 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(22 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(22 as libc::c_int as isize) = 4 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(23 as libc::c_int as isize) = 2 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(23 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_x).offset(24 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    *((*tf).bit_y).offset(24 as libc::c_int as isize) = 3 as libc::c_int as uint32_t;
    (*tf).width_at_border = 7 as libc::c_int;
    (*tf).total_width = 9 as libc::c_int;
    (*tf).reversed_border = 0 as libc::c_int != 0;
    return tf;
}
#[no_mangle]
pub unsafe extern "C" fn tag25h9_destroy(mut tf: *mut apriltag_family_t) {
    free((*tf).bit_x as *mut libc::c_void);
    free((*tf).bit_y as *mut libc::c_void);
    free((*tf).name as *mut libc::c_void);
    free(tf as *mut libc::c_void);
}
