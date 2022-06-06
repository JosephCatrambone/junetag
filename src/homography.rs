use ::libc;
extern "C" {
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn matd_create(rows: libc::c_int, cols: libc::c_int) -> *mut matd_t;
    fn matd_create_data(
        rows: libc::c_int,
        cols: libc::c_int,
        data: *const libc::c_double,
    ) -> *mut matd_t;
    fn matd_identity(dim: libc::c_int) -> *mut matd_t;
    fn matd_inverse(a: *const matd_t) -> *mut matd_t;
    fn matd_op(expr: *const libc::c_char, _: ...) -> *mut matd_t;
    fn matd_destroy(m: *mut matd_t);
    fn matd_svd(A: *mut matd_t) -> matd_svd_t;
    fn matd_svd_flags(A: *mut matd_t, flags: libc::c_int) -> matd_svd_t;
    fn matd_plu(a: *const matd_t) -> *mut matd_plu_t;
    fn matd_plu_destroy(mlu: *mut matd_plu_t);
    fn matd_plu_solve(mlu: *const matd_plu_t, b: *const matd_t) -> *mut matd_t;
    fn matd_chol(A: *mut matd_t) -> *mut matd_chol_t;
    fn matd_chol_solve(chol: *const matd_chol_t, b: *const matd_t) -> *mut matd_t;
    fn matd_chol_destroy(chol: *mut matd_chol_t);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct matd_t {
    pub nrows: libc::c_uint,
    pub ncols: libc::c_uint,
    pub data: [libc::c_double; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct matd_svd_t {
    pub U: *mut matd_t,
    pub S: *mut matd_t,
    pub V: *mut matd_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct matd_plu_t {
    pub singular: libc::c_int,
    pub piv: *mut libc::c_uint,
    pub pivsign: libc::c_int,
    pub lu: *mut matd_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct matd_chol_t {
    pub is_spd: libc::c_int,
    pub u: *mut matd_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zarray {
    pub el_sz: size_t,
    pub size: libc::c_int,
    pub alloc: libc::c_int,
    pub data: *mut libc::c_char,
}
pub type zarray_t = zarray;
#[inline]
unsafe extern "C" fn zarray_get_volatile(
    mut za: *const zarray_t,
    mut idx: libc::c_int,
    mut p: *mut libc::c_void,
) {
    let ref mut fresh0 = *(p as *mut *mut libc::c_void);
    *fresh0 = &mut *((*za).data)
        .offset((idx as libc::c_ulong).wrapping_mul((*za).el_sz) as isize)
        as *mut libc::c_char as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn zarray_size(mut za: *const zarray_t) -> libc::c_int {
    return (*za).size;
}
#[inline]
unsafe extern "C" fn sq(mut v: libc::c_double) -> libc::c_double {
    return v * v;
}
#[no_mangle]
pub unsafe extern "C" fn homography_compute(
    mut correspondences: *mut zarray_t,
    mut flags: libc::c_int,
) -> *mut matd_t {
    let mut x_cx: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut x_cy: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut y_cx: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut y_cy: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < zarray_size(correspondences) {
        let mut c: *mut libc::c_float = 0 as *mut libc::c_float;
        zarray_get_volatile(
            correspondences,
            i,
            &mut c as *mut *mut libc::c_float as *mut libc::c_void,
        );
        x_cx += *c.offset(0 as libc::c_int as isize) as libc::c_double;
        x_cy += *c.offset(1 as libc::c_int as isize) as libc::c_double;
        y_cx += *c.offset(2 as libc::c_int as isize) as libc::c_double;
        y_cy += *c.offset(3 as libc::c_int as isize) as libc::c_double;
        i += 1;
    }
    let mut sz: libc::c_int = zarray_size(correspondences);
    x_cx /= sz as libc::c_double;
    x_cy /= sz as libc::c_double;
    y_cx /= sz as libc::c_double;
    y_cy /= sz as libc::c_double;
    let mut A: *mut matd_t = matd_create(9 as libc::c_int, 9 as libc::c_int);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < zarray_size(correspondences) {
        let mut c_0: *mut libc::c_float = 0 as *mut libc::c_float;
        zarray_get_volatile(
            correspondences,
            i_0,
            &mut c_0 as *mut *mut libc::c_float as *mut libc::c_void,
        );
        let mut worldx: libc::c_double = *c_0.offset(0 as libc::c_int as isize)
            as libc::c_double - x_cx;
        let mut worldy: libc::c_double = *c_0.offset(1 as libc::c_int as isize)
            as libc::c_double - x_cy;
        let mut imagex: libc::c_double = *c_0.offset(2 as libc::c_int as isize)
            as libc::c_double - y_cx;
        let mut imagey: libc::c_double = *c_0.offset(3 as libc::c_int as isize)
            as libc::c_double - y_cy;
        let mut a03: libc::c_double = -worldx;
        let mut a04: libc::c_double = -worldy;
        let mut a05: libc::c_double = -(1 as libc::c_int) as libc::c_double;
        let mut a06: libc::c_double = worldx * imagey;
        let mut a07: libc::c_double = worldy * imagey;
        let mut a08: libc::c_double = imagey;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (3 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
            ) += a03 * a03;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (3 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(4 as libc::c_int as libc::c_uint) as isize,
            ) += a03 * a04;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (3 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(5 as libc::c_int as libc::c_uint) as isize,
            ) += a03 * a05;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (3 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(6 as libc::c_int as libc::c_uint) as isize,
            ) += a03 * a06;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (3 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(7 as libc::c_int as libc::c_uint) as isize,
            ) += a03 * a07;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (3 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(8 as libc::c_int as libc::c_uint) as isize,
            ) += a03 * a08;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (4 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(4 as libc::c_int as libc::c_uint) as isize,
            ) += a04 * a04;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (4 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(5 as libc::c_int as libc::c_uint) as isize,
            ) += a04 * a05;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (4 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(6 as libc::c_int as libc::c_uint) as isize,
            ) += a04 * a06;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (4 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(7 as libc::c_int as libc::c_uint) as isize,
            ) += a04 * a07;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (4 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(8 as libc::c_int as libc::c_uint) as isize,
            ) += a04 * a08;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (5 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(5 as libc::c_int as libc::c_uint) as isize,
            ) += a05 * a05;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (5 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(6 as libc::c_int as libc::c_uint) as isize,
            ) += a05 * a06;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (5 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(7 as libc::c_int as libc::c_uint) as isize,
            ) += a05 * a07;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (5 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(8 as libc::c_int as libc::c_uint) as isize,
            ) += a05 * a08;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (6 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(6 as libc::c_int as libc::c_uint) as isize,
            ) += a06 * a06;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (6 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(7 as libc::c_int as libc::c_uint) as isize,
            ) += a06 * a07;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (6 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(8 as libc::c_int as libc::c_uint) as isize,
            ) += a06 * a08;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (7 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(7 as libc::c_int as libc::c_uint) as isize,
            ) += a07 * a07;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (7 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(8 as libc::c_int as libc::c_uint) as isize,
            ) += a07 * a08;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (8 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(8 as libc::c_int as libc::c_uint) as isize,
            ) += a08 * a08;
        let mut a10: libc::c_double = worldx;
        let mut a11: libc::c_double = worldy;
        let mut a12: libc::c_double = 1 as libc::c_int as libc::c_double;
        let mut a16: libc::c_double = -worldx * imagex;
        let mut a17: libc::c_double = -worldy * imagex;
        let mut a18: libc::c_double = -imagex;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (0 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
            ) += a10 * a10;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (0 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) += a10 * a11;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (0 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
            ) += a10 * a12;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (0 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(6 as libc::c_int as libc::c_uint) as isize,
            ) += a10 * a16;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (0 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(7 as libc::c_int as libc::c_uint) as isize,
            ) += a10 * a17;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (0 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(8 as libc::c_int as libc::c_uint) as isize,
            ) += a10 * a18;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (1 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) += a11 * a11;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (1 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
            ) += a11 * a12;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (1 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(6 as libc::c_int as libc::c_uint) as isize,
            ) += a11 * a16;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (1 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(7 as libc::c_int as libc::c_uint) as isize,
            ) += a11 * a17;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (1 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(8 as libc::c_int as libc::c_uint) as isize,
            ) += a11 * a18;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (2 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
            ) += a12 * a12;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (2 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(6 as libc::c_int as libc::c_uint) as isize,
            ) += a12 * a16;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (2 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(7 as libc::c_int as libc::c_uint) as isize,
            ) += a12 * a17;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (2 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(8 as libc::c_int as libc::c_uint) as isize,
            ) += a12 * a18;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (6 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(6 as libc::c_int as libc::c_uint) as isize,
            ) += a16 * a16;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (6 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(7 as libc::c_int as libc::c_uint) as isize,
            ) += a16 * a17;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (6 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(8 as libc::c_int as libc::c_uint) as isize,
            ) += a16 * a18;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (7 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(7 as libc::c_int as libc::c_uint) as isize,
            ) += a17 * a17;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (7 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(8 as libc::c_int as libc::c_uint) as isize,
            ) += a17 * a18;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (8 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(8 as libc::c_int as libc::c_uint) as isize,
            ) += a18 * a18;
        let mut a20: libc::c_double = -worldx * imagey;
        let mut a21: libc::c_double = -worldy * imagey;
        let mut a22: libc::c_double = -imagey;
        let mut a23: libc::c_double = worldx * imagex;
        let mut a24: libc::c_double = worldy * imagex;
        let mut a25: libc::c_double = imagex;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (0 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
            ) += a20 * a20;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (0 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) += a20 * a21;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (0 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
            ) += a20 * a22;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (0 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
            ) += a20 * a23;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (0 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(4 as libc::c_int as libc::c_uint) as isize,
            ) += a20 * a24;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (0 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(5 as libc::c_int as libc::c_uint) as isize,
            ) += a20 * a25;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (1 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) += a21 * a21;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (1 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
            ) += a21 * a22;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (1 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
            ) += a21 * a23;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (1 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(4 as libc::c_int as libc::c_uint) as isize,
            ) += a21 * a24;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (1 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(5 as libc::c_int as libc::c_uint) as isize,
            ) += a21 * a25;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (2 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
            ) += a22 * a22;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (2 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
            ) += a22 * a23;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (2 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(4 as libc::c_int as libc::c_uint) as isize,
            ) += a22 * a24;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (2 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(5 as libc::c_int as libc::c_uint) as isize,
            ) += a22 * a25;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (3 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
            ) += a23 * a23;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (3 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(4 as libc::c_int as libc::c_uint) as isize,
            ) += a23 * a24;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (3 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(5 as libc::c_int as libc::c_uint) as isize,
            ) += a23 * a25;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (4 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(4 as libc::c_int as libc::c_uint) as isize,
            ) += a24 * a24;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (4 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(5 as libc::c_int as libc::c_uint) as isize,
            ) += a24 * a25;
        *((*A).data)
            .as_mut_ptr()
            .offset(
                (5 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*A).ncols)
                    .wrapping_add(5 as libc::c_int as libc::c_uint) as isize,
            ) += a25 * a25;
        i_0 += 1;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < 9 as libc::c_int {
        let mut j: libc::c_int = i_1 + 1 as libc::c_int;
        while j < 9 as libc::c_int {
            *((*A).data)
                .as_mut_ptr()
                .offset(
                    (j as libc::c_uint)
                        .wrapping_mul((*A).ncols)
                        .wrapping_add(i_1 as libc::c_uint) as isize,
                ) = *((*A).data)
                .as_mut_ptr()
                .offset(
                    (i_1 as libc::c_uint)
                        .wrapping_mul((*A).ncols)
                        .wrapping_add(j as libc::c_uint) as isize,
                );
            j += 1;
        }
        i_1 += 1;
    }
    let mut H: *mut matd_t = matd_create(3 as libc::c_int, 3 as libc::c_int);
    if flags & 1 as libc::c_int != 0 {
        let mut Ainv: *mut matd_t = matd_inverse(A);
        let mut scale: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut i_2: libc::c_int = 0 as libc::c_int;
        while i_2 < 9 as libc::c_int {
            scale
                += sq(
                    *((*Ainv).data)
                        .as_mut_ptr()
                        .offset(
                            (i_2 as libc::c_uint)
                                .wrapping_mul((*Ainv).ncols)
                                .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                        ),
                );
            i_2 += 1;
        }
        scale = sqrt(scale);
        let mut i_3: libc::c_int = 0 as libc::c_int;
        while i_3 < 3 as libc::c_int {
            let mut j_0: libc::c_int = 0 as libc::c_int;
            while j_0 < 3 as libc::c_int {
                *((*H).data)
                    .as_mut_ptr()
                    .offset(
                        (i_3 as libc::c_uint)
                            .wrapping_mul((*H).ncols)
                            .wrapping_add(j_0 as libc::c_uint) as isize,
                    ) = *((*Ainv).data)
                    .as_mut_ptr()
                    .offset(
                        ((3 as libc::c_int * i_3 + j_0) as libc::c_uint)
                            .wrapping_mul((*Ainv).ncols)
                            .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                    ) / scale;
                j_0 += 1;
            }
            i_3 += 1;
        }
        matd_destroy(Ainv);
    } else {
        let mut svd: matd_svd_t = matd_svd_flags(A, 1 as libc::c_int);
        let mut i_6: libc::c_int = 0 as libc::c_int;
        while i_6 < 3 as libc::c_int {
            let mut j_2: libc::c_int = 0 as libc::c_int;
            while j_2 < 3 as libc::c_int {
                *((*H).data)
                    .as_mut_ptr()
                    .offset(
                        (i_6 as libc::c_uint)
                            .wrapping_mul((*H).ncols)
                            .wrapping_add(j_2 as libc::c_uint) as isize,
                    ) = *((*svd.U).data)
                    .as_mut_ptr()
                    .offset(
                        ((3 as libc::c_int * i_6 + j_2) as libc::c_uint)
                            .wrapping_mul((*svd.U).ncols)
                            .wrapping_add(8 as libc::c_int as libc::c_uint) as isize,
                    );
                j_2 += 1;
            }
            i_6 += 1;
        }
        matd_destroy(svd.U);
        matd_destroy(svd.S);
        matd_destroy(svd.V);
    }
    let mut Tx: *mut matd_t = matd_identity(3 as libc::c_int);
    *((*Tx).data)
        .as_mut_ptr()
        .offset(
            (0 as libc::c_int as libc::c_uint)
                .wrapping_mul((*Tx).ncols)
                .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
        ) = -x_cx;
    *((*Tx).data)
        .as_mut_ptr()
        .offset(
            (1 as libc::c_int as libc::c_uint)
                .wrapping_mul((*Tx).ncols)
                .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
        ) = -x_cy;
    let mut Ty: *mut matd_t = matd_identity(3 as libc::c_int);
    *((*Ty).data)
        .as_mut_ptr()
        .offset(
            (0 as libc::c_int as libc::c_uint)
                .wrapping_mul((*Ty).ncols)
                .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
        ) = y_cx;
    *((*Ty).data)
        .as_mut_ptr()
        .offset(
            (1 as libc::c_int as libc::c_uint)
                .wrapping_mul((*Ty).ncols)
                .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
        ) = y_cy;
    let mut H2: *mut matd_t = matd_op(
        b"M*M*M\0" as *const u8 as *const libc::c_char,
        Ty,
        H,
        Tx,
    );
    matd_destroy(A);
    matd_destroy(Tx);
    matd_destroy(Ty);
    matd_destroy(H);
    return H2;
}
#[no_mangle]
pub unsafe extern "C" fn homography_to_pose(
    mut H: *const matd_t,
    mut fx: libc::c_double,
    mut fy: libc::c_double,
    mut cx: libc::c_double,
    mut cy: libc::c_double,
) -> *mut matd_t {
    let mut R20: libc::c_double = *((*H).data)
        .as_ptr()
        .offset(
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*H).ncols)
                .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
        );
    let mut R21: libc::c_double = *((*H).data)
        .as_ptr()
        .offset(
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*H).ncols)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        );
    let mut TZ: libc::c_double = *((*H).data)
        .as_ptr()
        .offset(
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*H).ncols)
                .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
        );
    let mut R00: libc::c_double = (*((*H).data)
        .as_ptr()
        .offset(
            (0 as libc::c_int as libc::c_uint)
                .wrapping_mul((*H).ncols)
                .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
        ) - cx * R20) / fx;
    let mut R01: libc::c_double = (*((*H).data)
        .as_ptr()
        .offset(
            (0 as libc::c_int as libc::c_uint)
                .wrapping_mul((*H).ncols)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        ) - cx * R21) / fx;
    let mut TX: libc::c_double = (*((*H).data)
        .as_ptr()
        .offset(
            (0 as libc::c_int as libc::c_uint)
                .wrapping_mul((*H).ncols)
                .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
        ) - cx * TZ) / fx;
    let mut R10: libc::c_double = (*((*H).data)
        .as_ptr()
        .offset(
            (1 as libc::c_int as libc::c_uint)
                .wrapping_mul((*H).ncols)
                .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
        ) - cy * R20) / fy;
    let mut R11: libc::c_double = (*((*H).data)
        .as_ptr()
        .offset(
            (1 as libc::c_int as libc::c_uint)
                .wrapping_mul((*H).ncols)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        ) - cy * R21) / fy;
    let mut TY: libc::c_double = (*((*H).data)
        .as_ptr()
        .offset(
            (1 as libc::c_int as libc::c_uint)
                .wrapping_mul((*H).ncols)
                .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
        ) - cy * TZ) / fy;
    let mut length1: libc::c_double = sqrtf(
        (R00 * R00 + R10 * R10 + R20 * R20) as libc::c_float,
    ) as libc::c_double;
    let mut length2: libc::c_double = sqrtf(
        (R01 * R01 + R11 * R11 + R21 * R21) as libc::c_float,
    ) as libc::c_double;
    let mut s: libc::c_double = 1.0f64
        / sqrtf((length1 * length2) as libc::c_float) as libc::c_double;
    if TZ > 0 as libc::c_int as libc::c_double {
        s *= -(1 as libc::c_int) as libc::c_double;
    }
    R20 *= s;
    R21 *= s;
    TZ *= s;
    R00 *= s;
    R01 *= s;
    TX *= s;
    R10 *= s;
    R11 *= s;
    TY *= s;
    let mut R02: libc::c_double = R10 * R21 - R20 * R11;
    let mut R12: libc::c_double = R20 * R01 - R00 * R21;
    let mut R22: libc::c_double = R00 * R11 - R10 * R01;
    let mut R: *mut matd_t = matd_create_data(
        3 as libc::c_int,
        3 as libc::c_int,
        [R00, R01, R02, R10, R11, R12, R20, R21, R22].as_mut_ptr(),
    );
    let mut svd: matd_svd_t = matd_svd(R);
    matd_destroy(R);
    R = matd_op(b"M*M'\0" as *const u8 as *const libc::c_char, svd.U, svd.V);
    matd_destroy(svd.U);
    matd_destroy(svd.S);
    matd_destroy(svd.V);
    R00 = *((*R).data)
        .as_mut_ptr()
        .offset(
            (0 as libc::c_int as libc::c_uint)
                .wrapping_mul((*R).ncols)
                .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
        );
    R01 = *((*R).data)
        .as_mut_ptr()
        .offset(
            (0 as libc::c_int as libc::c_uint)
                .wrapping_mul((*R).ncols)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        );
    R02 = *((*R).data)
        .as_mut_ptr()
        .offset(
            (0 as libc::c_int as libc::c_uint)
                .wrapping_mul((*R).ncols)
                .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
        );
    R10 = *((*R).data)
        .as_mut_ptr()
        .offset(
            (1 as libc::c_int as libc::c_uint)
                .wrapping_mul((*R).ncols)
                .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
        );
    R11 = *((*R).data)
        .as_mut_ptr()
        .offset(
            (1 as libc::c_int as libc::c_uint)
                .wrapping_mul((*R).ncols)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        );
    R12 = *((*R).data)
        .as_mut_ptr()
        .offset(
            (1 as libc::c_int as libc::c_uint)
                .wrapping_mul((*R).ncols)
                .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
        );
    R20 = *((*R).data)
        .as_mut_ptr()
        .offset(
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*R).ncols)
                .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
        );
    R21 = *((*R).data)
        .as_mut_ptr()
        .offset(
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*R).ncols)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        );
    R22 = *((*R).data)
        .as_mut_ptr()
        .offset(
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*R).ncols)
                .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
        );
    matd_destroy(R);
    return matd_create_data(
        4 as libc::c_int,
        4 as libc::c_int,
        [
            R00,
            R01,
            R02,
            TX,
            R10,
            R11,
            R12,
            TY,
            R20,
            R21,
            R22,
            TZ,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            1 as libc::c_int as libc::c_double,
        ]
            .as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn homography_to_model_view(
    mut H: *const matd_t,
    mut F: libc::c_double,
    mut G: libc::c_double,
    mut A: libc::c_double,
    mut B: libc::c_double,
    mut C: libc::c_double,
    mut D: libc::c_double,
) -> *mut matd_t {
    let mut R20: libc::c_double = -*((*H).data)
        .as_ptr()
        .offset(
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*H).ncols)
                .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
        );
    let mut R21: libc::c_double = -*((*H).data)
        .as_ptr()
        .offset(
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*H).ncols)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        );
    let mut TZ: libc::c_double = -*((*H).data)
        .as_ptr()
        .offset(
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*H).ncols)
                .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
        );
    let mut R00: libc::c_double = (*((*H).data)
        .as_ptr()
        .offset(
            (0 as libc::c_int as libc::c_uint)
                .wrapping_mul((*H).ncols)
                .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
        ) - A * R20) / F;
    let mut R01: libc::c_double = (*((*H).data)
        .as_ptr()
        .offset(
            (0 as libc::c_int as libc::c_uint)
                .wrapping_mul((*H).ncols)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        ) - A * R21) / F;
    let mut TX: libc::c_double = (*((*H).data)
        .as_ptr()
        .offset(
            (0 as libc::c_int as libc::c_uint)
                .wrapping_mul((*H).ncols)
                .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
        ) - A * TZ) / F;
    let mut R10: libc::c_double = (*((*H).data)
        .as_ptr()
        .offset(
            (1 as libc::c_int as libc::c_uint)
                .wrapping_mul((*H).ncols)
                .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
        ) - B * R20) / G;
    let mut R11: libc::c_double = (*((*H).data)
        .as_ptr()
        .offset(
            (1 as libc::c_int as libc::c_uint)
                .wrapping_mul((*H).ncols)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        ) - B * R21) / G;
    let mut TY: libc::c_double = (*((*H).data)
        .as_ptr()
        .offset(
            (1 as libc::c_int as libc::c_uint)
                .wrapping_mul((*H).ncols)
                .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
        ) - B * TZ) / G;
    let mut length1: libc::c_double = sqrtf(
        (R00 * R00 + R10 * R10 + R20 * R20) as libc::c_float,
    ) as libc::c_double;
    let mut length2: libc::c_double = sqrtf(
        (R01 * R01 + R11 * R11 + R21 * R21) as libc::c_float,
    ) as libc::c_double;
    let mut s: libc::c_double = 1.0f64
        / sqrtf((length1 * length2) as libc::c_float) as libc::c_double;
    if TZ > 0 as libc::c_int as libc::c_double {
        s *= -(1 as libc::c_int) as libc::c_double;
    }
    R20 *= s;
    R21 *= s;
    TZ *= s;
    R00 *= s;
    R01 *= s;
    TX *= s;
    R10 *= s;
    R11 *= s;
    TY *= s;
    let mut R02: libc::c_double = R10 * R21 - R20 * R11;
    let mut R12: libc::c_double = R20 * R01 - R00 * R21;
    let mut R22: libc::c_double = R00 * R11 - R10 * R01;
    return matd_create_data(
        4 as libc::c_int,
        4 as libc::c_int,
        [
            R00,
            R01,
            R02,
            TX,
            R10,
            R11,
            R12,
            TY,
            R20,
            R21,
            R22,
            TZ,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            1 as libc::c_int as libc::c_double,
        ]
            .as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn quat_to_matrix(
    mut q: *const libc::c_double,
    mut M: *mut matd_t,
) {
    let mut w: libc::c_double = *q.offset(0 as libc::c_int as isize);
    let mut x: libc::c_double = *q.offset(1 as libc::c_int as isize);
    let mut y: libc::c_double = *q.offset(2 as libc::c_int as isize);
    let mut z: libc::c_double = *q.offset(3 as libc::c_int as isize);
    *((*M).data)
        .as_mut_ptr()
        .offset(
            (0 as libc::c_int as libc::c_uint)
                .wrapping_mul((*M).ncols)
                .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
        ) = w * w + x * x - y * y - z * z;
    *((*M).data)
        .as_mut_ptr()
        .offset(
            (0 as libc::c_int as libc::c_uint)
                .wrapping_mul((*M).ncols)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        ) = 2 as libc::c_int as libc::c_double * x * y
        - 2 as libc::c_int as libc::c_double * w * z;
    *((*M).data)
        .as_mut_ptr()
        .offset(
            (0 as libc::c_int as libc::c_uint)
                .wrapping_mul((*M).ncols)
                .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
        ) = 2 as libc::c_int as libc::c_double * x * z
        + 2 as libc::c_int as libc::c_double * w * y;
    *((*M).data)
        .as_mut_ptr()
        .offset(
            (1 as libc::c_int as libc::c_uint)
                .wrapping_mul((*M).ncols)
                .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
        ) = 2 as libc::c_int as libc::c_double * x * y
        + 2 as libc::c_int as libc::c_double * w * z;
    *((*M).data)
        .as_mut_ptr()
        .offset(
            (1 as libc::c_int as libc::c_uint)
                .wrapping_mul((*M).ncols)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        ) = w * w - x * x + y * y - z * z;
    *((*M).data)
        .as_mut_ptr()
        .offset(
            (1 as libc::c_int as libc::c_uint)
                .wrapping_mul((*M).ncols)
                .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
        ) = 2 as libc::c_int as libc::c_double * y * z
        - 2 as libc::c_int as libc::c_double * w * x;
    *((*M).data)
        .as_mut_ptr()
        .offset(
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*M).ncols)
                .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
        ) = 2 as libc::c_int as libc::c_double * x * z
        - 2 as libc::c_int as libc::c_double * w * y;
    *((*M).data)
        .as_mut_ptr()
        .offset(
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*M).ncols)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        ) = 2 as libc::c_int as libc::c_double * y * z
        + 2 as libc::c_int as libc::c_double * w * x;
    *((*M).data)
        .as_mut_ptr()
        .offset(
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*M).ncols)
                .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
        ) = w * w - x * x - y * y + z * z;
}
