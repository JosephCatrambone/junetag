use ::libc;
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zarray {
    pub el_sz: size_t,
    pub size: libc::c_int,
    pub alloc: libc::c_int,
    pub data: *mut libc::c_char,
}
pub type zarray_t = zarray;
#[no_mangle]
pub unsafe extern "C" fn zstrcmp(
    mut a_pp: *const libc::c_void,
    mut b_pp: *const libc::c_void,
) -> libc::c_int {
    let mut a: *mut libc::c_char = *(a_pp as *mut *mut libc::c_void)
        as *mut libc::c_char;
    let mut b: *mut libc::c_char = *(b_pp as *mut *mut libc::c_void)
        as *mut libc::c_char;
    return strcmp(a, b);
}
#[no_mangle]
pub unsafe extern "C" fn zarray_vmap(
    mut za: *mut zarray_t,
    mut f: Option::<unsafe extern "C" fn() -> ()>,
) {
    let mut idx: libc::c_int = 0 as libc::c_int;
    while idx < (*za).size {
        let mut pp: *mut libc::c_void = &mut *((*za).data)
            .offset((idx as libc::c_ulong).wrapping_mul((*za).el_sz) as isize)
            as *mut libc::c_char as *mut libc::c_void;
        let mut p: *mut libc::c_void = *(pp as *mut *mut libc::c_void);
        ::std::mem::transmute::<_, fn(_)>(f.expect("non-null function pointer"))(p);
        idx += 1;
    }
}
