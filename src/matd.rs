use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn free(__ptr: *mut libc::c_void);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn svd22(
        A: *const libc::c_double,
        U: *mut libc::c_double,
        S: *mut libc::c_double,
        V: *mut libc::c_double,
    );
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct matd_t {
    pub nrows: libc::c_uint,
    pub ncols: libc::c_uint,
    pub data: [libc::c_double; 0],
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
pub struct matd_svd_t {
    pub U: *mut matd_t,
    pub S: *mut matd_t,
    pub V: *mut matd_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct matd_chol_t {
    pub is_spd: libc::c_int,
    pub u: *mut matd_t,
}
#[inline]
unsafe extern "C" fn sq(mut v: libc::c_double) -> libc::c_double {
    return v * v;
}
#[inline]
unsafe extern "C" fn matd_is_scalar(mut a: *const matd_t) -> libc::c_int {
    return ((*a).ncols <= 1 as libc::c_int as libc::c_uint
        && (*a).nrows <= 1 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn matd_create(
    mut rows: libc::c_int,
    mut cols: libc::c_int,
) -> *mut matd_t {
    if rows == 0 as libc::c_int || cols == 0 as libc::c_int {
        return matd_create_scalar(0 as libc::c_int as libc::c_double);
    }
    let mut m: *mut matd_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        (::std::mem::size_of::<matd_t>() as libc::c_ulong)
            .wrapping_add(
                ((rows * cols) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ),
            ),
    ) as *mut matd_t;
    (*m).nrows = rows as libc::c_uint;
    (*m).ncols = cols as libc::c_uint;
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn matd_create_scalar(mut v: libc::c_double) -> *mut matd_t {
    let mut m: *mut matd_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        (::std::mem::size_of::<matd_t>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut matd_t;
    (*m).nrows = 0 as libc::c_int as libc::c_uint;
    (*m).ncols = 0 as libc::c_int as libc::c_uint;
    *((*m).data).as_mut_ptr().offset(0 as libc::c_int as isize) = v;
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn matd_create_data(
    mut rows: libc::c_int,
    mut cols: libc::c_int,
    mut data: *const libc::c_double,
) -> *mut matd_t {
    if rows == 0 as libc::c_int || cols == 0 as libc::c_int {
        return matd_create_scalar(*data.offset(0 as libc::c_int as isize));
    }
    let mut m: *mut matd_t = matd_create(rows, cols);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < rows * cols {
        *((*m).data).as_mut_ptr().offset(i as isize) = *data.offset(i as isize);
        i += 1;
    }
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn matd_create_dataf(
    mut rows: libc::c_int,
    mut cols: libc::c_int,
    mut data: *const libc::c_float,
) -> *mut matd_t {
    if rows == 0 as libc::c_int || cols == 0 as libc::c_int {
        return matd_create_scalar(
            *data.offset(0 as libc::c_int as isize) as libc::c_double,
        );
    }
    let mut m: *mut matd_t = matd_create(rows, cols);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < rows * cols {
        *((*m).data)
            .as_mut_ptr()
            .offset(i as isize) = *data.offset(i as isize) as libc::c_double;
        i += 1;
    }
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn matd_identity(mut dim: libc::c_int) -> *mut matd_t {
    if dim == 0 as libc::c_int {
        return matd_create_scalar(1 as libc::c_int as libc::c_double);
    }
    let mut m: *mut matd_t = matd_create(dim, dim);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < dim {
        *((*m).data)
            .as_mut_ptr()
            .offset(
                (i as libc::c_uint)
                    .wrapping_mul((*m).ncols)
                    .wrapping_add(i as libc::c_uint) as isize,
            ) = 1 as libc::c_int as libc::c_double;
        i += 1;
    }
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn matd_get(
    mut m: *const matd_t,
    mut row: libc::c_int,
    mut col: libc::c_int,
) -> libc::c_double {
    return *((*m).data)
        .as_ptr()
        .offset(
            (row as libc::c_uint)
                .wrapping_mul((*m).ncols)
                .wrapping_add(col as libc::c_uint) as isize,
        );
}
#[no_mangle]
pub unsafe extern "C" fn matd_put(
    mut m: *mut matd_t,
    mut row: libc::c_int,
    mut col: libc::c_int,
    mut value: libc::c_double,
) {
    if matd_is_scalar(m) != 0 {
        matd_put_scalar(m, value);
        return;
    }
    *((*m).data)
        .as_mut_ptr()
        .offset(
            (row as libc::c_uint)
                .wrapping_mul((*m).ncols)
                .wrapping_add(col as libc::c_uint) as isize,
        ) = value;
}
#[no_mangle]
pub unsafe extern "C" fn matd_get_scalar(mut m: *const matd_t) -> libc::c_double {
    return *((*m).data).as_ptr().offset(0 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn matd_put_scalar(mut m: *mut matd_t, mut value: libc::c_double) {
    *((*m).data).as_mut_ptr().offset(0 as libc::c_int as isize) = value;
}
#[no_mangle]
pub unsafe extern "C" fn matd_copy(mut m: *const matd_t) -> *mut matd_t {
    let mut x: *mut matd_t = matd_create(
        (*m).nrows as libc::c_int,
        (*m).ncols as libc::c_int,
    );
    if matd_is_scalar(m) != 0 {
        *((*x).data)
            .as_mut_ptr()
            .offset(
                0 as libc::c_int as isize,
            ) = *((*m).data).as_ptr().offset(0 as libc::c_int as isize);
    } else {
        memcpy(
            ((*x).data).as_mut_ptr() as *mut libc::c_void,
            ((*m).data).as_ptr() as *const libc::c_void,
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul((*m).ncols as libc::c_ulong)
                .wrapping_mul((*m).nrows as libc::c_ulong),
        );
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn matd_select(
    mut a: *const matd_t,
    mut r0: libc::c_int,
    mut r1: libc::c_int,
    mut c0: libc::c_int,
    mut c1: libc::c_int,
) -> *mut matd_t {
    let mut nrows: libc::c_int = r1 - r0 + 1 as libc::c_int;
    let mut ncols: libc::c_int = c1 - c0 + 1 as libc::c_int;
    let mut r: *mut matd_t = matd_create(nrows, ncols);
    let mut row: libc::c_int = r0;
    while row <= r1 {
        let mut col: libc::c_int = c0;
        while col <= c1 {
            *((*r).data)
                .as_mut_ptr()
                .offset(
                    ((row - r0) as libc::c_uint)
                        .wrapping_mul((*r).ncols)
                        .wrapping_add((col - c0) as libc::c_uint) as isize,
                ) = *((*a).data)
                .as_ptr()
                .offset(
                    (row as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(col as libc::c_uint) as isize,
                );
            col += 1;
        }
        row += 1;
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn matd_print(mut m: *const matd_t, mut fmt: *const libc::c_char) {
    if matd_is_scalar(m) != 0 {
        printf(
            fmt,
            *((*m).data)
                .as_ptr()
                .offset(
                    (0 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*m).ncols)
                        .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                ),
        );
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    } else {
        let mut i: libc::c_int = 0 as libc::c_int;
        while (i as libc::c_uint) < (*m).nrows {
            let mut j: libc::c_int = 0 as libc::c_int;
            while (j as libc::c_uint) < (*m).ncols {
                printf(
                    fmt,
                    *((*m).data)
                        .as_ptr()
                        .offset(
                            (i as libc::c_uint)
                                .wrapping_mul((*m).ncols)
                                .wrapping_add(j as libc::c_uint) as isize,
                        ),
                );
                j += 1;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn matd_print_transpose(
    mut m: *const matd_t,
    mut fmt: *const libc::c_char,
) {
    if matd_is_scalar(m) != 0 {
        printf(
            fmt,
            *((*m).data)
                .as_ptr()
                .offset(
                    (0 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*m).ncols)
                        .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                ),
        );
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    } else {
        let mut j: libc::c_int = 0 as libc::c_int;
        while (j as libc::c_uint) < (*m).ncols {
            let mut i: libc::c_int = 0 as libc::c_int;
            while (i as libc::c_uint) < (*m).nrows {
                printf(
                    fmt,
                    *((*m).data)
                        .as_ptr()
                        .offset(
                            (i as libc::c_uint)
                                .wrapping_mul((*m).ncols)
                                .wrapping_add(j as libc::c_uint) as isize,
                        ),
                );
                i += 1;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            j += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn matd_destroy(mut m: *mut matd_t) {
    if m.is_null() {
        return;
    }
    free(m as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn matd_multiply(
    mut a: *const matd_t,
    mut b: *const matd_t,
) -> *mut matd_t {
    if matd_is_scalar(a) != 0 {
        return matd_scale(b, *((*a).data).as_ptr().offset(0 as libc::c_int as isize));
    }
    if matd_is_scalar(b) != 0 {
        return matd_scale(a, *((*b).data).as_ptr().offset(0 as libc::c_int as isize));
    }
    let mut m: *mut matd_t = matd_create(
        (*a).nrows as libc::c_int,
        (*b).ncols as libc::c_int,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_uint) < (*m).nrows {
        let mut j: libc::c_int = 0 as libc::c_int;
        while (j as libc::c_uint) < (*m).ncols {
            let mut acc: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut k: libc::c_int = 0 as libc::c_int;
            while (k as libc::c_uint) < (*a).ncols {
                acc
                    += *((*a).data)
                        .as_ptr()
                        .offset(
                            (i as libc::c_uint)
                                .wrapping_mul((*a).ncols)
                                .wrapping_add(k as libc::c_uint) as isize,
                        )
                        * *((*b).data)
                            .as_ptr()
                            .offset(
                                (k as libc::c_uint)
                                    .wrapping_mul((*b).ncols)
                                    .wrapping_add(j as libc::c_uint) as isize,
                            );
                k += 1;
            }
            *((*m).data)
                .as_mut_ptr()
                .offset(
                    (i as libc::c_uint)
                        .wrapping_mul((*m).ncols)
                        .wrapping_add(j as libc::c_uint) as isize,
                ) = acc;
            j += 1;
        }
        i += 1;
    }
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn matd_scale(
    mut a: *const matd_t,
    mut s: libc::c_double,
) -> *mut matd_t {
    if matd_is_scalar(a) != 0 {
        return matd_create_scalar(
            *((*a).data).as_ptr().offset(0 as libc::c_int as isize) * s,
        );
    }
    let mut m: *mut matd_t = matd_create(
        (*a).nrows as libc::c_int,
        (*a).ncols as libc::c_int,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_uint) < (*m).nrows {
        let mut j: libc::c_int = 0 as libc::c_int;
        while (j as libc::c_uint) < (*m).ncols {
            *((*m).data)
                .as_mut_ptr()
                .offset(
                    (i as libc::c_uint)
                        .wrapping_mul((*m).ncols)
                        .wrapping_add(j as libc::c_uint) as isize,
                ) = s
                * *((*a).data)
                    .as_ptr()
                    .offset(
                        (i as libc::c_uint)
                            .wrapping_mul((*a).ncols)
                            .wrapping_add(j as libc::c_uint) as isize,
                    );
            j += 1;
        }
        i += 1;
    }
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn matd_scale_inplace(mut a: *mut matd_t, mut s: libc::c_double) {
    if matd_is_scalar(a) != 0 {
        *((*a).data).as_mut_ptr().offset(0 as libc::c_int as isize) *= s;
        return;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_uint) < (*a).nrows {
        let mut j: libc::c_int = 0 as libc::c_int;
        while (j as libc::c_uint) < (*a).ncols {
            *((*a).data)
                .as_mut_ptr()
                .offset(
                    (i as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(j as libc::c_uint) as isize,
                ) *= s;
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn matd_add(
    mut a: *const matd_t,
    mut b: *const matd_t,
) -> *mut matd_t {
    if matd_is_scalar(a) != 0 {
        return matd_create_scalar(
            *((*a).data).as_ptr().offset(0 as libc::c_int as isize)
                + *((*b).data).as_ptr().offset(0 as libc::c_int as isize),
        );
    }
    let mut m: *mut matd_t = matd_create(
        (*a).nrows as libc::c_int,
        (*a).ncols as libc::c_int,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_uint) < (*m).nrows {
        let mut j: libc::c_int = 0 as libc::c_int;
        while (j as libc::c_uint) < (*m).ncols {
            *((*m).data)
                .as_mut_ptr()
                .offset(
                    (i as libc::c_uint)
                        .wrapping_mul((*m).ncols)
                        .wrapping_add(j as libc::c_uint) as isize,
                ) = *((*a).data)
                .as_ptr()
                .offset(
                    (i as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(j as libc::c_uint) as isize,
                )
                + *((*b).data)
                    .as_ptr()
                    .offset(
                        (i as libc::c_uint)
                            .wrapping_mul((*b).ncols)
                            .wrapping_add(j as libc::c_uint) as isize,
                    );
            j += 1;
        }
        i += 1;
    }
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn matd_add_inplace(mut a: *mut matd_t, mut b: *const matd_t) {
    if matd_is_scalar(a) != 0 {
        *((*a).data).as_mut_ptr().offset(0 as libc::c_int as isize)
            += *((*b).data).as_ptr().offset(0 as libc::c_int as isize);
        return;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_uint) < (*a).nrows {
        let mut j: libc::c_int = 0 as libc::c_int;
        while (j as libc::c_uint) < (*a).ncols {
            *((*a).data)
                .as_mut_ptr()
                .offset(
                    (i as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(j as libc::c_uint) as isize,
                )
                += *((*b).data)
                    .as_ptr()
                    .offset(
                        (i as libc::c_uint)
                            .wrapping_mul((*b).ncols)
                            .wrapping_add(j as libc::c_uint) as isize,
                    );
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn matd_subtract(
    mut a: *const matd_t,
    mut b: *const matd_t,
) -> *mut matd_t {
    if matd_is_scalar(a) != 0 {
        return matd_create_scalar(
            *((*a).data).as_ptr().offset(0 as libc::c_int as isize)
                - *((*b).data).as_ptr().offset(0 as libc::c_int as isize),
        );
    }
    let mut m: *mut matd_t = matd_create(
        (*a).nrows as libc::c_int,
        (*a).ncols as libc::c_int,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_uint) < (*m).nrows {
        let mut j: libc::c_int = 0 as libc::c_int;
        while (j as libc::c_uint) < (*m).ncols {
            *((*m).data)
                .as_mut_ptr()
                .offset(
                    (i as libc::c_uint)
                        .wrapping_mul((*m).ncols)
                        .wrapping_add(j as libc::c_uint) as isize,
                ) = *((*a).data)
                .as_ptr()
                .offset(
                    (i as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(j as libc::c_uint) as isize,
                )
                - *((*b).data)
                    .as_ptr()
                    .offset(
                        (i as libc::c_uint)
                            .wrapping_mul((*b).ncols)
                            .wrapping_add(j as libc::c_uint) as isize,
                    );
            j += 1;
        }
        i += 1;
    }
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn matd_subtract_inplace(
    mut a: *mut matd_t,
    mut b: *const matd_t,
) {
    if matd_is_scalar(a) != 0 {
        *((*a).data).as_mut_ptr().offset(0 as libc::c_int as isize)
            -= *((*b).data).as_ptr().offset(0 as libc::c_int as isize);
        return;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_uint) < (*a).nrows {
        let mut j: libc::c_int = 0 as libc::c_int;
        while (j as libc::c_uint) < (*a).ncols {
            *((*a).data)
                .as_mut_ptr()
                .offset(
                    (i as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(j as libc::c_uint) as isize,
                )
                -= *((*b).data)
                    .as_ptr()
                    .offset(
                        (i as libc::c_uint)
                            .wrapping_mul((*b).ncols)
                            .wrapping_add(j as libc::c_uint) as isize,
                    );
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn matd_transpose(mut a: *const matd_t) -> *mut matd_t {
    if matd_is_scalar(a) != 0 {
        return matd_create_scalar(
            *((*a).data).as_ptr().offset(0 as libc::c_int as isize),
        );
    }
    let mut m: *mut matd_t = matd_create(
        (*a).ncols as libc::c_int,
        (*a).nrows as libc::c_int,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_uint) < (*a).nrows {
        let mut j: libc::c_int = 0 as libc::c_int;
        while (j as libc::c_uint) < (*a).ncols {
            *((*m).data)
                .as_mut_ptr()
                .offset(
                    (j as libc::c_uint)
                        .wrapping_mul((*m).ncols)
                        .wrapping_add(i as libc::c_uint) as isize,
                ) = *((*a).data)
                .as_ptr()
                .offset(
                    (i as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(j as libc::c_uint) as isize,
                );
            j += 1;
        }
        i += 1;
    }
    return m;
}
unsafe extern "C" fn matd_det_general(mut a: *const matd_t) -> libc::c_double {
    let mut mlu: *mut matd_plu_t = matd_plu(a);
    let mut L: *mut matd_t = matd_plu_l(mlu);
    let mut U: *mut matd_t = matd_plu_u(mlu);
    let mut detL: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut detU: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_uint) < (*a).nrows {
        detL *= matd_get(L, i, i);
        detU *= matd_get(U, i, i);
        i += 1;
    }
    let mut det: libc::c_double = (*mlu).pivsign as libc::c_double * detL * detU;
    matd_plu_destroy(mlu);
    matd_destroy(L);
    matd_destroy(U);
    return det;
}
#[no_mangle]
pub unsafe extern "C" fn matd_det(mut a: *const matd_t) -> libc::c_double {
    match (*a).nrows {
        0 => {}
        1 => return *((*a).data).as_ptr().offset(0 as libc::c_int as isize),
        2 => {
            return *((*a).data).as_ptr().offset(0 as libc::c_int as isize)
                * *((*a).data).as_ptr().offset(3 as libc::c_int as isize)
                - *((*a).data).as_ptr().offset(1 as libc::c_int as isize)
                    * *((*a).data).as_ptr().offset(2 as libc::c_int as isize);
        }
        3 => {
            return *((*a).data).as_ptr().offset(0 as libc::c_int as isize)
                * *((*a).data).as_ptr().offset(4 as libc::c_int as isize)
                * *((*a).data).as_ptr().offset(8 as libc::c_int as isize)
                - *((*a).data).as_ptr().offset(0 as libc::c_int as isize)
                    * *((*a).data).as_ptr().offset(5 as libc::c_int as isize)
                    * *((*a).data).as_ptr().offset(7 as libc::c_int as isize)
                + *((*a).data).as_ptr().offset(1 as libc::c_int as isize)
                    * *((*a).data).as_ptr().offset(5 as libc::c_int as isize)
                    * *((*a).data).as_ptr().offset(6 as libc::c_int as isize)
                - *((*a).data).as_ptr().offset(1 as libc::c_int as isize)
                    * *((*a).data).as_ptr().offset(3 as libc::c_int as isize)
                    * *((*a).data).as_ptr().offset(8 as libc::c_int as isize)
                + *((*a).data).as_ptr().offset(2 as libc::c_int as isize)
                    * *((*a).data).as_ptr().offset(3 as libc::c_int as isize)
                    * *((*a).data).as_ptr().offset(7 as libc::c_int as isize)
                - *((*a).data).as_ptr().offset(2 as libc::c_int as isize)
                    * *((*a).data).as_ptr().offset(4 as libc::c_int as isize)
                    * *((*a).data).as_ptr().offset(6 as libc::c_int as isize);
        }
        4 => {
            let mut m00: libc::c_double = *((*a).data)
                .as_ptr()
                .offset(
                    (0 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                );
            let mut m01: libc::c_double = *((*a).data)
                .as_ptr()
                .offset(
                    (0 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                );
            let mut m02: libc::c_double = *((*a).data)
                .as_ptr()
                .offset(
                    (0 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                );
            let mut m03: libc::c_double = *((*a).data)
                .as_ptr()
                .offset(
                    (0 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
                );
            let mut m10: libc::c_double = *((*a).data)
                .as_ptr()
                .offset(
                    (1 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                );
            let mut m11: libc::c_double = *((*a).data)
                .as_ptr()
                .offset(
                    (1 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                );
            let mut m12: libc::c_double = *((*a).data)
                .as_ptr()
                .offset(
                    (1 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                );
            let mut m13: libc::c_double = *((*a).data)
                .as_ptr()
                .offset(
                    (1 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
                );
            let mut m20: libc::c_double = *((*a).data)
                .as_ptr()
                .offset(
                    (2 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                );
            let mut m21: libc::c_double = *((*a).data)
                .as_ptr()
                .offset(
                    (2 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                );
            let mut m22: libc::c_double = *((*a).data)
                .as_ptr()
                .offset(
                    (2 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                );
            let mut m23: libc::c_double = *((*a).data)
                .as_ptr()
                .offset(
                    (2 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
                );
            let mut m30: libc::c_double = *((*a).data)
                .as_ptr()
                .offset(
                    (3 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                );
            let mut m31: libc::c_double = *((*a).data)
                .as_ptr()
                .offset(
                    (3 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                );
            let mut m32: libc::c_double = *((*a).data)
                .as_ptr()
                .offset(
                    (3 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                );
            let mut m33: libc::c_double = *((*a).data)
                .as_ptr()
                .offset(
                    (3 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
                );
            return m00 * m11 * m22 * m33 - m00 * m11 * m23 * m32 - m00 * m21 * m12 * m33
                + m00 * m21 * m13 * m32 + m00 * m31 * m12 * m23 - m00 * m31 * m13 * m22
                - m10 * m01 * m22 * m33 + m10 * m01 * m23 * m32 + m10 * m21 * m02 * m33
                - m10 * m21 * m03 * m32 - m10 * m31 * m02 * m23 + m10 * m31 * m03 * m22
                + m20 * m01 * m12 * m33 - m20 * m01 * m13 * m32 - m20 * m11 * m02 * m33
                + m20 * m11 * m03 * m32 + m20 * m31 * m02 * m13 - m20 * m31 * m03 * m12
                - m30 * m01 * m12 * m23 + m30 * m01 * m13 * m22 + m30 * m11 * m02 * m23
                - m30 * m11 * m03 * m22 - m30 * m21 * m02 * m13 + m30 * m21 * m03 * m12;
        }
        _ => return matd_det_general(a),
    }
    return 0 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn matd_inverse(mut x: *const matd_t) -> *mut matd_t {
    let mut m: *mut matd_t = 0 as *mut matd_t;
    if matd_is_scalar(x) != 0 {
        if *((*x).data).as_ptr().offset(0 as libc::c_int as isize)
            == 0 as libc::c_int as libc::c_double
        {
            return 0 as *mut matd_t;
        }
        return matd_create_scalar(
            1.0f64 / *((*x).data).as_ptr().offset(0 as libc::c_int as isize),
        );
    }
    match (*x).nrows {
        1 => {
            let mut det: libc::c_double = *((*x).data)
                .as_ptr()
                .offset(0 as libc::c_int as isize);
            if det == 0 as libc::c_int as libc::c_double {
                return 0 as *mut matd_t;
            }
            let mut invdet: libc::c_double = 1.0f64 / det;
            m = matd_create((*x).nrows as libc::c_int, (*x).nrows as libc::c_int);
            *((*m).data)
                .as_mut_ptr()
                .offset(
                    (0 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*m).ncols)
                        .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                ) = 1.0f64 * invdet;
            return m;
        }
        2 => {
            let mut det_0: libc::c_double = *((*x).data)
                .as_ptr()
                .offset(0 as libc::c_int as isize)
                * *((*x).data).as_ptr().offset(3 as libc::c_int as isize)
                - *((*x).data).as_ptr().offset(1 as libc::c_int as isize)
                    * *((*x).data).as_ptr().offset(2 as libc::c_int as isize);
            if det_0 == 0 as libc::c_int as libc::c_double {
                return 0 as *mut matd_t;
            }
            let mut invdet_0: libc::c_double = 1.0f64 / det_0;
            m = matd_create((*x).nrows as libc::c_int, (*x).nrows as libc::c_int);
            *((*m).data)
                .as_mut_ptr()
                .offset(
                    (0 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*m).ncols)
                        .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                ) = *((*x).data)
                .as_ptr()
                .offset(
                    (1 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*x).ncols)
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                ) * invdet_0;
            *((*m).data)
                .as_mut_ptr()
                .offset(
                    (0 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*m).ncols)
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                ) = -*((*x).data)
                .as_ptr()
                .offset(
                    (0 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*x).ncols)
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                ) * invdet_0;
            *((*m).data)
                .as_mut_ptr()
                .offset(
                    (1 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*m).ncols)
                        .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                ) = -*((*x).data)
                .as_ptr()
                .offset(
                    (1 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*x).ncols)
                        .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                ) * invdet_0;
            *((*m).data)
                .as_mut_ptr()
                .offset(
                    (1 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*m).ncols)
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                ) = *((*x).data)
                .as_ptr()
                .offset(
                    (0 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*x).ncols)
                        .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                ) * invdet_0;
            return m;
        }
        _ => {
            let mut plu: *mut matd_plu_t = matd_plu(x);
            let mut inv: *mut matd_t = 0 as *mut matd_t;
            if (*plu).singular == 0 {
                let mut ident: *mut matd_t = matd_identity((*x).nrows as libc::c_int);
                inv = matd_plu_solve(plu, ident);
                matd_destroy(ident);
            }
            matd_plu_destroy(plu);
            return inv;
        }
    };
}
#[inline]
unsafe extern "C" fn matd_op_gobble_right(
    mut expr: *const libc::c_char,
    mut pos: *mut libc::c_int,
    mut acc: *mut matd_t,
    mut garb: *mut *mut matd_t,
    mut garbpos: *mut libc::c_int,
) -> *mut matd_t {
    while *expr.offset(*pos as isize) as libc::c_int != 0 as libc::c_int {
        match *expr.offset(*pos as isize) as libc::c_int {
            39 => {
                let mut res: *mut matd_t = matd_transpose(acc);
                let ref mut fresh0 = *garb.offset(*garbpos as isize);
                *fresh0 = res;
                *garbpos += 1;
                acc = res;
                *pos += 1;
            }
            94 => {
                let mut res_0: *mut matd_t = matd_inverse(acc);
                let ref mut fresh1 = *garb.offset(*garbpos as isize);
                *fresh1 = res_0;
                *garbpos += 1;
                acc = res_0;
                *pos += 3 as libc::c_int;
            }
            _ => return acc,
        }
    }
    return acc;
}
unsafe extern "C" fn matd_op_recurse(
    mut expr: *const libc::c_char,
    mut pos: *mut libc::c_int,
    mut acc: *mut matd_t,
    mut args: *mut *mut matd_t,
    mut argpos: *mut libc::c_int,
    mut garb: *mut *mut matd_t,
    mut garbpos: *mut libc::c_int,
    mut oneterm: libc::c_int,
) -> *mut matd_t {
    while *expr.offset(*pos as isize) as libc::c_int != 0 as libc::c_int {
        match *expr.offset(*pos as isize) as libc::c_int {
            40 => {
                if oneterm != 0 && !acc.is_null() {
                    return acc;
                }
                *pos += 1;
                let mut rhs: *mut matd_t = matd_op_recurse(
                    expr,
                    pos,
                    0 as *mut matd_t,
                    args,
                    argpos,
                    garb,
                    garbpos,
                    0 as libc::c_int,
                );
                rhs = matd_op_gobble_right(expr, pos, rhs, garb, garbpos);
                if acc.is_null() {
                    acc = rhs;
                } else {
                    let mut res: *mut matd_t = matd_multiply(acc, rhs);
                    let ref mut fresh2 = *garb.offset(*garbpos as isize);
                    *fresh2 = res;
                    *garbpos += 1;
                    acc = res;
                }
            }
            41 => {
                if oneterm != 0 {
                    return acc;
                }
                *pos += 1;
                return acc;
            }
            42 => {
                *pos += 1;
                let mut rhs_0: *mut matd_t = matd_op_recurse(
                    expr,
                    pos,
                    0 as *mut matd_t,
                    args,
                    argpos,
                    garb,
                    garbpos,
                    1 as libc::c_int,
                );
                rhs_0 = matd_op_gobble_right(expr, pos, rhs_0, garb, garbpos);
                if acc.is_null() {
                    acc = rhs_0;
                } else {
                    let mut res_0: *mut matd_t = matd_multiply(acc, rhs_0);
                    let ref mut fresh3 = *garb.offset(*garbpos as isize);
                    *fresh3 = res_0;
                    *garbpos += 1;
                    acc = res_0;
                }
            }
            70 => {
                let mut rhs_1: *mut matd_t = *args.offset(*argpos as isize);
                let ref mut fresh4 = *garb.offset(*garbpos as isize);
                *fresh4 = rhs_1;
                *garbpos += 1;
                *pos += 1;
                *argpos += 1;
                rhs_1 = matd_op_gobble_right(expr, pos, rhs_1, garb, garbpos);
                if acc.is_null() {
                    acc = rhs_1;
                } else {
                    let mut res_1: *mut matd_t = matd_multiply(acc, rhs_1);
                    let ref mut fresh5 = *garb.offset(*garbpos as isize);
                    *fresh5 = res_1;
                    *garbpos += 1;
                    acc = res_1;
                }
            }
            77 => {
                let mut rhs_2: *mut matd_t = *args.offset(*argpos as isize);
                *pos += 1;
                *argpos += 1;
                rhs_2 = matd_op_gobble_right(expr, pos, rhs_2, garb, garbpos);
                if acc.is_null() {
                    acc = rhs_2;
                } else {
                    let mut res_2: *mut matd_t = matd_multiply(acc, rhs_2);
                    let ref mut fresh6 = *garb.offset(*garbpos as isize);
                    *fresh6 = res_2;
                    *garbpos += 1;
                    acc = res_2;
                }
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 46 => {
                let mut start: *const libc::c_char = &*expr.offset(*pos as isize)
                    as *const libc::c_char;
                let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut s: libc::c_double = strtod(start, &mut end);
                *pos = (*pos as libc::c_long + end.offset_from(start) as libc::c_long)
                    as libc::c_int;
                let mut rhs_3: *mut matd_t = matd_create_scalar(s);
                let ref mut fresh7 = *garb.offset(*garbpos as isize);
                *fresh7 = rhs_3;
                *garbpos += 1;
                rhs_3 = matd_op_gobble_right(expr, pos, rhs_3, garb, garbpos);
                if acc.is_null() {
                    acc = rhs_3;
                } else {
                    let mut res_3: *mut matd_t = matd_multiply(acc, rhs_3);
                    let ref mut fresh8 = *garb.offset(*garbpos as isize);
                    *fresh8 = res_3;
                    *garbpos += 1;
                    acc = res_3;
                }
            }
            43 => {
                if oneterm != 0 && !acc.is_null() {
                    return acc;
                }
                *pos += 1;
                let mut rhs_4: *mut matd_t = matd_op_recurse(
                    expr,
                    pos,
                    0 as *mut matd_t,
                    args,
                    argpos,
                    garb,
                    garbpos,
                    1 as libc::c_int,
                );
                rhs_4 = matd_op_gobble_right(expr, pos, rhs_4, garb, garbpos);
                let mut res_4: *mut matd_t = matd_add(acc, rhs_4);
                let ref mut fresh9 = *garb.offset(*garbpos as isize);
                *fresh9 = res_4;
                *garbpos += 1;
                acc = res_4;
            }
            45 => {
                if oneterm != 0 && !acc.is_null() {
                    return acc;
                }
                if acc.is_null() {
                    *pos += 1;
                    let mut rhs_5: *mut matd_t = matd_op_recurse(
                        expr,
                        pos,
                        0 as *mut matd_t,
                        args,
                        argpos,
                        garb,
                        garbpos,
                        1 as libc::c_int,
                    );
                    rhs_5 = matd_op_gobble_right(expr, pos, rhs_5, garb, garbpos);
                    let mut res_5: *mut matd_t = matd_scale(
                        rhs_5,
                        -(1 as libc::c_int) as libc::c_double,
                    );
                    let ref mut fresh10 = *garb.offset(*garbpos as isize);
                    *fresh10 = res_5;
                    *garbpos += 1;
                    acc = res_5;
                } else {
                    *pos += 1;
                    let mut rhs_6: *mut matd_t = matd_op_recurse(
                        expr,
                        pos,
                        0 as *mut matd_t,
                        args,
                        argpos,
                        garb,
                        garbpos,
                        1 as libc::c_int,
                    );
                    rhs_6 = matd_op_gobble_right(expr, pos, rhs_6, garb, garbpos);
                    let mut res_6: *mut matd_t = matd_subtract(acc, rhs_6);
                    let ref mut fresh11 = *garb.offset(*garbpos as isize);
                    *fresh11 = res_6;
                    *garbpos += 1;
                    acc = res_6;
                }
            }
            32 => {
                *pos += 1;
            }
            _ => {
                fflush(stderr);
            }
        }
    }
    return acc;
}
#[no_mangle]
pub unsafe extern "C" fn matd_op(
    mut expr: *const libc::c_char,
    mut args: ...
) -> *mut matd_t {
    let mut nargs: libc::c_int = 0 as libc::c_int;
    let mut exprlen: libc::c_int = 0 as libc::c_int;
    let mut p: *const libc::c_char = expr;
    while *p as libc::c_int != 0 as libc::c_int {
        if *p as libc::c_int == 'M' as i32 || *p as libc::c_int == 'F' as i32 {
            nargs += 1;
        }
        exprlen += 1;
        p = p.offset(1);
    }
    if exprlen == 0 {
        return 0 as *mut matd_t;
    }
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    let mut args_0: *mut *mut matd_t = malloc(
        (::std::mem::size_of::<*mut matd_t>() as libc::c_ulong)
            .wrapping_mul(nargs as libc::c_ulong),
    ) as *mut *mut matd_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < nargs {
        let ref mut fresh12 = *args_0.offset(i as isize);
        *fresh12 = ap.arg::<*mut matd_t>();
        i += 1;
    }
    let mut pos: libc::c_int = 0 as libc::c_int;
    let mut argpos: libc::c_int = 0 as libc::c_int;
    let mut garbpos: libc::c_int = 0 as libc::c_int;
    let mut garb: *mut *mut matd_t = malloc(
        (::std::mem::size_of::<*mut matd_t>() as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(exprlen as libc::c_ulong),
    ) as *mut *mut matd_t;
    let mut res: *mut matd_t = matd_op_recurse(
        expr,
        &mut pos,
        0 as *mut matd_t,
        args_0,
        &mut argpos,
        garb,
        &mut garbpos,
        0 as libc::c_int,
    );
    free(args_0 as *mut libc::c_void);
    let mut res_copy: *mut matd_t = if !res.is_null() {
        matd_copy(res)
    } else {
        0 as *mut matd_t
    };
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < garbpos {
        matd_destroy(*garb.offset(i_0 as isize));
        i_0 += 1;
    }
    free(garb as *mut libc::c_void);
    return res_copy;
}
#[no_mangle]
pub unsafe extern "C" fn matd_vec_mag(mut a: *const matd_t) -> libc::c_double {
    let mut mag: libc::c_double = 0.0f64;
    let mut len: libc::c_int = ((*a).nrows).wrapping_mul((*a).ncols) as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len {
        mag += sq(*((*a).data).as_ptr().offset(i as isize));
        i += 1;
    }
    return sqrt(mag);
}
#[no_mangle]
pub unsafe extern "C" fn matd_vec_dist(
    mut a: *const matd_t,
    mut b: *const matd_t,
) -> libc::c_double {
    let mut lena: libc::c_int = ((*a).nrows).wrapping_mul((*a).ncols) as libc::c_int;
    return matd_vec_dist_n(a, b, lena);
}
#[no_mangle]
pub unsafe extern "C" fn matd_vec_dist_n(
    mut a: *const matd_t,
    mut b: *const matd_t,
    mut n: libc::c_int,
) -> libc::c_double {
    let mut lena: libc::c_int = ((*a).nrows).wrapping_mul((*a).ncols) as libc::c_int;
    let mut lenb: libc::c_int = ((*b).nrows).wrapping_mul((*b).ncols) as libc::c_int;
    let mut mag: libc::c_double = 0.0f64;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        mag
            += sq(
                *((*a).data).as_ptr().offset(i as isize)
                    - *((*b).data).as_ptr().offset(i as isize),
            );
        i += 1;
    }
    return sqrt(mag);
}
#[inline]
unsafe extern "C" fn max_idx(
    mut A: *const matd_t,
    mut row: libc::c_int,
    mut maxcol: libc::c_int,
) -> libc::c_int {
    let mut maxi: libc::c_int = 0 as libc::c_int;
    let mut maxv: libc::c_double = -(1 as libc::c_int) as libc::c_double;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < maxcol {
        if !(i == row) {
            let mut v: libc::c_double = fabs(
                *((*A).data)
                    .as_ptr()
                    .offset(
                        (row as libc::c_uint)
                            .wrapping_mul((*A).ncols)
                            .wrapping_add(i as libc::c_uint) as isize,
                    ),
            );
            if v > maxv {
                maxi = i;
                maxv = v;
            }
        }
        i += 1;
    }
    return maxi;
}
#[no_mangle]
pub unsafe extern "C" fn matd_vec_dot_product(
    mut a: *const matd_t,
    mut b: *const matd_t,
) -> libc::c_double {
    let mut adim: libc::c_int = ((*a).ncols).wrapping_mul((*a).nrows) as libc::c_int;
    let mut bdim: libc::c_int = ((*b).ncols).wrapping_mul((*b).nrows) as libc::c_int;
    let mut acc: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < adim {
        acc
            += *((*a).data).as_ptr().offset(i as isize)
                * *((*b).data).as_ptr().offset(i as isize);
        i += 1;
    }
    return acc;
}
#[no_mangle]
pub unsafe extern "C" fn matd_vec_normalize(mut a: *const matd_t) -> *mut matd_t {
    let mut mag: libc::c_double = matd_vec_mag(a);
    let mut b: *mut matd_t = matd_create(
        (*a).nrows as libc::c_int,
        (*a).ncols as libc::c_int,
    );
    let mut len: libc::c_int = ((*a).nrows).wrapping_mul((*a).ncols) as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len {
        *((*b).data)
            .as_mut_ptr()
            .offset(i as isize) = *((*a).data).as_ptr().offset(i as isize) / mag;
        i += 1;
    }
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn matd_crossproduct(
    mut a: *const matd_t,
    mut b: *const matd_t,
) -> *mut matd_t {
    let mut r: *mut matd_t = matd_create(
        (*a).nrows as libc::c_int,
        (*a).ncols as libc::c_int,
    );
    *((*r).data)
        .as_mut_ptr()
        .offset(
            0 as libc::c_int as isize,
        ) = *((*a).data).as_ptr().offset(1 as libc::c_int as isize)
        * *((*b).data).as_ptr().offset(2 as libc::c_int as isize)
        - *((*a).data).as_ptr().offset(2 as libc::c_int as isize)
            * *((*b).data).as_ptr().offset(1 as libc::c_int as isize);
    *((*r).data)
        .as_mut_ptr()
        .offset(
            1 as libc::c_int as isize,
        ) = *((*a).data).as_ptr().offset(2 as libc::c_int as isize)
        * *((*b).data).as_ptr().offset(0 as libc::c_int as isize)
        - *((*a).data).as_ptr().offset(0 as libc::c_int as isize)
            * *((*b).data).as_ptr().offset(2 as libc::c_int as isize);
    *((*r).data)
        .as_mut_ptr()
        .offset(
            2 as libc::c_int as isize,
        ) = *((*a).data).as_ptr().offset(0 as libc::c_int as isize)
        * *((*b).data).as_ptr().offset(1 as libc::c_int as isize)
        - *((*a).data).as_ptr().offset(1 as libc::c_int as isize)
            * *((*b).data).as_ptr().offset(0 as libc::c_int as isize);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn matd_err_inf(
    mut a: *const matd_t,
    mut b: *const matd_t,
) -> libc::c_double {
    let mut maxf: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_uint) < (*a).nrows {
        let mut j: libc::c_int = 0 as libc::c_int;
        while (j as libc::c_uint) < (*a).ncols {
            let mut av: libc::c_double = *((*a).data)
                .as_ptr()
                .offset(
                    (i as libc::c_uint)
                        .wrapping_mul((*a).ncols)
                        .wrapping_add(j as libc::c_uint) as isize,
                );
            let mut bv: libc::c_double = *((*b).data)
                .as_ptr()
                .offset(
                    (i as libc::c_uint)
                        .wrapping_mul((*b).ncols)
                        .wrapping_add(j as libc::c_uint) as isize,
                );
            let mut err: libc::c_double = fabs(av - bv);
            maxf = fmax(maxf, err);
            j += 1;
        }
        i += 1;
    }
    return maxf;
}
unsafe extern "C" fn matd_svd_tall(
    mut A: *mut matd_t,
    mut flags: libc::c_int,
) -> matd_svd_t {
    let mut B: *mut matd_t = matd_copy(A);
    let mut LS: *mut matd_t = matd_identity((*A).nrows as libc::c_int);
    let mut RS: *mut matd_t = matd_identity((*A).ncols as libc::c_int);
    let mut current_block_47: u64;
    let mut hhidx: libc::c_int = 0 as libc::c_int;
    while (hhidx as libc::c_uint) < (*A).nrows {
        if (hhidx as libc::c_uint) < (*A).ncols {
            let mut vlen: libc::c_int = ((*A).nrows).wrapping_sub(hhidx as libc::c_uint)
                as libc::c_int;
            let mut v: *mut libc::c_double = malloc(
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(vlen as libc::c_ulong),
            ) as *mut libc::c_double;
            let mut mag2: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < vlen {
                *v
                    .offset(
                        i as isize,
                    ) = *((*B).data)
                    .as_mut_ptr()
                    .offset(
                        ((hhidx + i) as libc::c_uint)
                            .wrapping_mul((*B).ncols)
                            .wrapping_add(hhidx as libc::c_uint) as isize,
                    );
                mag2 += *v.offset(i as isize) * *v.offset(i as isize);
                i += 1;
            }
            let mut oldv0: libc::c_double = *v.offset(0 as libc::c_int as isize);
            if oldv0 < 0 as libc::c_int as libc::c_double {
                *v.offset(0 as libc::c_int as isize) -= sqrt(mag2);
            } else {
                *v.offset(0 as libc::c_int as isize) += sqrt(mag2);
            }
            mag2
                += -oldv0 * oldv0
                    + *v.offset(0 as libc::c_int as isize)
                        * *v.offset(0 as libc::c_int as isize);
            let mut mag: libc::c_double = sqrt(mag2);
            if mag == 0 as libc::c_int as libc::c_double {
                free(v as *mut libc::c_void);
                current_block_47 = 14916268686031723178;
            } else {
                let mut i_0: libc::c_int = 0 as libc::c_int;
                while i_0 < vlen {
                    *v.offset(i_0 as isize) /= mag;
                    i_0 += 1;
                }
                let mut i_1: libc::c_int = 0 as libc::c_int;
                while (i_1 as libc::c_uint) < (*LS).nrows {
                    let mut dot: libc::c_double = 0 as libc::c_int as libc::c_double;
                    let mut j: libc::c_int = 0 as libc::c_int;
                    while j < vlen {
                        dot
                            += *((*LS).data)
                                .as_mut_ptr()
                                .offset(
                                    (i_1 as libc::c_uint)
                                        .wrapping_mul((*LS).ncols)
                                        .wrapping_add((hhidx + j) as libc::c_uint) as isize,
                                ) * *v.offset(j as isize);
                        j += 1;
                    }
                    let mut j_0: libc::c_int = 0 as libc::c_int;
                    while j_0 < vlen {
                        *((*LS).data)
                            .as_mut_ptr()
                            .offset(
                                (i_1 as libc::c_uint)
                                    .wrapping_mul((*LS).ncols)
                                    .wrapping_add((hhidx + j_0) as libc::c_uint) as isize,
                            )
                            -= 2 as libc::c_int as libc::c_double * dot
                                * *v.offset(j_0 as isize);
                        j_0 += 1;
                    }
                    i_1 += 1;
                }
                let mut i_2: libc::c_int = 0 as libc::c_int;
                while (i_2 as libc::c_uint) < (*B).ncols {
                    let mut dot_0: libc::c_double = 0 as libc::c_int as libc::c_double;
                    let mut j_1: libc::c_int = 0 as libc::c_int;
                    while j_1 < vlen {
                        dot_0
                            += *((*B).data)
                                .as_mut_ptr()
                                .offset(
                                    ((hhidx + j_1) as libc::c_uint)
                                        .wrapping_mul((*B).ncols)
                                        .wrapping_add(i_2 as libc::c_uint) as isize,
                                ) * *v.offset(j_1 as isize);
                        j_1 += 1;
                    }
                    let mut j_2: libc::c_int = 0 as libc::c_int;
                    while j_2 < vlen {
                        *((*B).data)
                            .as_mut_ptr()
                            .offset(
                                ((hhidx + j_2) as libc::c_uint)
                                    .wrapping_mul((*B).ncols)
                                    .wrapping_add(i_2 as libc::c_uint) as isize,
                            )
                            -= 2 as libc::c_int as libc::c_double * dot_0
                                * *v.offset(j_2 as isize);
                        j_2 += 1;
                    }
                    i_2 += 1;
                }
                free(v as *mut libc::c_void);
                current_block_47 = 3934796541983872331;
            }
        } else {
            current_block_47 = 3934796541983872331;
        }
        match current_block_47 {
            3934796541983872331 => {
                if ((hhidx + 2 as libc::c_int) as libc::c_uint) < (*A).ncols {
                    let mut vlen_0: libc::c_int = ((*A).ncols)
                        .wrapping_sub(hhidx as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                    let mut v_0: *mut libc::c_double = malloc(
                        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                            .wrapping_mul(vlen_0 as libc::c_ulong),
                    ) as *mut libc::c_double;
                    let mut mag2_0: libc::c_double = 0 as libc::c_int as libc::c_double;
                    let mut i_3: libc::c_int = 0 as libc::c_int;
                    while i_3 < vlen_0 {
                        *v_0
                            .offset(
                                i_3 as isize,
                            ) = *((*B).data)
                            .as_mut_ptr()
                            .offset(
                                (hhidx as libc::c_uint)
                                    .wrapping_mul((*B).ncols)
                                    .wrapping_add(
                                        (hhidx + i_3 + 1 as libc::c_int) as libc::c_uint,
                                    ) as isize,
                            );
                        mag2_0 += *v_0.offset(i_3 as isize) * *v_0.offset(i_3 as isize);
                        i_3 += 1;
                    }
                    let mut oldv0_0: libc::c_double = *v_0
                        .offset(0 as libc::c_int as isize);
                    if oldv0_0 < 0 as libc::c_int as libc::c_double {
                        *v_0.offset(0 as libc::c_int as isize) -= sqrt(mag2_0);
                    } else {
                        *v_0.offset(0 as libc::c_int as isize) += sqrt(mag2_0);
                    }
                    mag2_0
                        += -oldv0_0 * oldv0_0
                            + *v_0.offset(0 as libc::c_int as isize)
                                * *v_0.offset(0 as libc::c_int as isize);
                    let mut mag_0: libc::c_double = sqrt(mag2_0);
                    if mag_0 == 0 as libc::c_int as libc::c_double {
                        free(v_0 as *mut libc::c_void);
                    } else {
                        let mut i_4: libc::c_int = 0 as libc::c_int;
                        while i_4 < vlen_0 {
                            *v_0.offset(i_4 as isize) /= mag_0;
                            i_4 += 1;
                        }
                        let mut i_5: libc::c_int = 0 as libc::c_int;
                        while (i_5 as libc::c_uint) < (*RS).nrows {
                            let mut dot_1: libc::c_double = 0 as libc::c_int
                                as libc::c_double;
                            let mut j_3: libc::c_int = 0 as libc::c_int;
                            while j_3 < vlen_0 {
                                dot_1
                                    += *((*RS).data)
                                        .as_mut_ptr()
                                        .offset(
                                            (i_5 as libc::c_uint)
                                                .wrapping_mul((*RS).ncols)
                                                .wrapping_add(
                                                    (hhidx + 1 as libc::c_int + j_3) as libc::c_uint,
                                                ) as isize,
                                        ) * *v_0.offset(j_3 as isize);
                                j_3 += 1;
                            }
                            let mut j_4: libc::c_int = 0 as libc::c_int;
                            while j_4 < vlen_0 {
                                *((*RS).data)
                                    .as_mut_ptr()
                                    .offset(
                                        (i_5 as libc::c_uint)
                                            .wrapping_mul((*RS).ncols)
                                            .wrapping_add(
                                                (hhidx + 1 as libc::c_int + j_4) as libc::c_uint,
                                            ) as isize,
                                    )
                                    -= 2 as libc::c_int as libc::c_double * dot_1
                                        * *v_0.offset(j_4 as isize);
                                j_4 += 1;
                            }
                            i_5 += 1;
                        }
                        let mut i_6: libc::c_int = 0 as libc::c_int;
                        while (i_6 as libc::c_uint) < (*B).nrows {
                            let mut dot_2: libc::c_double = 0 as libc::c_int
                                as libc::c_double;
                            let mut j_5: libc::c_int = 0 as libc::c_int;
                            while j_5 < vlen_0 {
                                dot_2
                                    += *((*B).data)
                                        .as_mut_ptr()
                                        .offset(
                                            (i_6 as libc::c_uint)
                                                .wrapping_mul((*B).ncols)
                                                .wrapping_add(
                                                    (hhidx + 1 as libc::c_int + j_5) as libc::c_uint,
                                                ) as isize,
                                        ) * *v_0.offset(j_5 as isize);
                                j_5 += 1;
                            }
                            let mut j_6: libc::c_int = 0 as libc::c_int;
                            while j_6 < vlen_0 {
                                *((*B).data)
                                    .as_mut_ptr()
                                    .offset(
                                        (i_6 as libc::c_uint)
                                            .wrapping_mul((*B).ncols)
                                            .wrapping_add(
                                                (hhidx + 1 as libc::c_int + j_6) as libc::c_uint,
                                            ) as isize,
                                    )
                                    -= 2 as libc::c_int as libc::c_double * dot_2
                                        * *v_0.offset(j_6 as isize);
                                j_6 += 1;
                            }
                            i_6 += 1;
                        }
                        free(v_0 as *mut libc::c_void);
                    }
                }
            }
            _ => {}
        }
        hhidx += 1;
    }
    let mut maxiters: libc::c_int = (200 as libc::c_int as libc::c_uint)
        .wrapping_add(
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*A).nrows)
                .wrapping_mul((*A).ncols),
        ) as libc::c_int;
    let mut iter: libc::c_int = 0;
    let mut maxv: libc::c_double = 0.;
    let mut tol: libc::c_double = 1E-10f64;
    let find_max_method: libc::c_int = 1 as libc::c_int;
    let mut maxrowidx: *mut libc::c_int = malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((*B).ncols as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut lastmaxi: libc::c_int = 0;
    let mut lastmaxj: libc::c_int = 0;
    if find_max_method == 1 as libc::c_int {
        let mut i_7: libc::c_int = 2 as libc::c_int;
        while (i_7 as libc::c_uint) < (*B).ncols {
            *maxrowidx.offset(i_7 as isize) = max_idx(B, i_7, (*B).ncols as libc::c_int);
            i_7 += 1;
        }
        lastmaxi = 0 as libc::c_int;
        lastmaxj = 1 as libc::c_int;
    }
    iter = 0 as libc::c_int;
    while iter < maxiters {
        if (*B).ncols < 2 as libc::c_int as libc::c_uint {
            break;
        }
        let mut maxi: libc::c_int = 0;
        let mut maxj: libc::c_int = 0;
        if find_max_method == 1 as libc::c_int {
            maxi = -(1 as libc::c_int);
            maxv = -(1 as libc::c_int) as libc::c_double;
            let mut rowi: libc::c_int = 0 as libc::c_int;
            while (rowi as libc::c_uint) < (*B).ncols {
                let mut thismaxv: libc::c_double = 0.;
                if rowi == lastmaxi || rowi == lastmaxj {
                    *maxrowidx
                        .offset(
                            rowi as isize,
                        ) = max_idx(B, rowi, (*B).ncols as libc::c_int);
                    thismaxv = fabs(
                        *((*B).data)
                            .as_mut_ptr()
                            .offset(
                                (rowi as libc::c_uint)
                                    .wrapping_mul((*B).ncols)
                                    .wrapping_add(
                                        *maxrowidx.offset(rowi as isize) as libc::c_uint,
                                    ) as isize,
                            ),
                    );
                } else if *maxrowidx.offset(rowi as isize) == lastmaxi
                        || *maxrowidx.offset(rowi as isize) == lastmaxj
                    {
                    *maxrowidx
                        .offset(
                            rowi as isize,
                        ) = max_idx(B, rowi, (*B).ncols as libc::c_int);
                    thismaxv = fabs(
                        *((*B).data)
                            .as_mut_ptr()
                            .offset(
                                (rowi as libc::c_uint)
                                    .wrapping_mul((*B).ncols)
                                    .wrapping_add(
                                        *maxrowidx.offset(rowi as isize) as libc::c_uint,
                                    ) as isize,
                            ),
                    );
                } else {
                    thismaxv = fabs(
                        *((*B).data)
                            .as_mut_ptr()
                            .offset(
                                (rowi as libc::c_uint)
                                    .wrapping_mul((*B).ncols)
                                    .wrapping_add(
                                        *maxrowidx.offset(rowi as isize) as libc::c_uint,
                                    ) as isize,
                            ),
                    );
                    if lastmaxi != rowi {
                        let mut v_1: libc::c_double = fabs(
                            *((*B).data)
                                .as_mut_ptr()
                                .offset(
                                    (rowi as libc::c_uint)
                                        .wrapping_mul((*B).ncols)
                                        .wrapping_add(lastmaxi as libc::c_uint) as isize,
                                ),
                        );
                        if v_1 > thismaxv {
                            thismaxv = v_1;
                            *maxrowidx.offset(rowi as isize) = lastmaxi;
                        }
                    }
                    if lastmaxj != rowi {
                        let mut v_2: libc::c_double = fabs(
                            *((*B).data)
                                .as_mut_ptr()
                                .offset(
                                    (rowi as libc::c_uint)
                                        .wrapping_mul((*B).ncols)
                                        .wrapping_add(lastmaxj as libc::c_uint) as isize,
                                ),
                        );
                        if v_2 > thismaxv {
                            thismaxv = v_2;
                            *maxrowidx.offset(rowi as isize) = lastmaxj;
                        }
                    }
                }
                if thismaxv > maxv {
                    maxv = thismaxv;
                    maxi = rowi;
                }
                rowi += 1;
            }
            maxj = *maxrowidx.offset(maxi as isize);
            lastmaxi = maxi;
            lastmaxj = maxj;
            if maxv < tol {
                break;
            }
        } else if find_max_method == 2 as libc::c_int {
            maxv = -(1 as libc::c_int) as libc::c_double;
            let mut i_8: libc::c_int = 0 as libc::c_int;
            while (i_8 as libc::c_uint) < (*B).ncols {
                let mut j_7: libc::c_int = 0 as libc::c_int;
                while (j_7 as libc::c_uint) < (*B).ncols {
                    if !(i_8 == j_7) {
                        let mut v_3: libc::c_double = fabs(
                            *((*B).data)
                                .as_mut_ptr()
                                .offset(
                                    (i_8 as libc::c_uint)
                                        .wrapping_mul((*B).ncols)
                                        .wrapping_add(j_7 as libc::c_uint) as isize,
                                ),
                        );
                        if v_3 > maxv {
                            maxi = i_8;
                            maxj = j_7;
                            maxv = v_3;
                        }
                    }
                    j_7 += 1;
                }
                i_8 += 1;
            }
            if maxv < tol {
                break;
            }
        }
        let mut A0: libc::c_double = *((*B).data)
            .as_mut_ptr()
            .offset(
                (maxi as libc::c_uint)
                    .wrapping_mul((*B).ncols)
                    .wrapping_add(maxi as libc::c_uint) as isize,
            );
        let mut A1: libc::c_double = *((*B).data)
            .as_mut_ptr()
            .offset(
                (maxi as libc::c_uint)
                    .wrapping_mul((*B).ncols)
                    .wrapping_add(maxj as libc::c_uint) as isize,
            );
        let mut A2: libc::c_double = *((*B).data)
            .as_mut_ptr()
            .offset(
                (maxj as libc::c_uint)
                    .wrapping_mul((*B).ncols)
                    .wrapping_add(maxi as libc::c_uint) as isize,
            );
        let mut A3: libc::c_double = *((*B).data)
            .as_mut_ptr()
            .offset(
                (maxj as libc::c_uint)
                    .wrapping_mul((*B).ncols)
                    .wrapping_add(maxj as libc::c_uint) as isize,
            );
        let mut AQ: [libc::c_double; 4] = [0.; 4];
        AQ[0 as libc::c_int as usize] = A0;
        AQ[1 as libc::c_int as usize] = A1;
        AQ[2 as libc::c_int as usize] = A2;
        AQ[3 as libc::c_int as usize] = A3;
        let mut U: [libc::c_double; 4] = [0.; 4];
        let mut S: [libc::c_double; 2] = [0.; 2];
        let mut V: [libc::c_double; 4] = [0.; 4];
        svd22(
            AQ.as_mut_ptr() as *const libc::c_double,
            U.as_mut_ptr(),
            S.as_mut_ptr(),
            V.as_mut_ptr(),
        );
        let mut i_9: libc::c_int = 0 as libc::c_int;
        while (i_9 as libc::c_uint) < (*LS).nrows {
            let mut vi: libc::c_double = *((*LS).data)
                .as_mut_ptr()
                .offset(
                    (i_9 as libc::c_uint)
                        .wrapping_mul((*LS).ncols)
                        .wrapping_add(maxi as libc::c_uint) as isize,
                );
            let mut vj: libc::c_double = *((*LS).data)
                .as_mut_ptr()
                .offset(
                    (i_9 as libc::c_uint)
                        .wrapping_mul((*LS).ncols)
                        .wrapping_add(maxj as libc::c_uint) as isize,
                );
            *((*LS).data)
                .as_mut_ptr()
                .offset(
                    (i_9 as libc::c_uint)
                        .wrapping_mul((*LS).ncols)
                        .wrapping_add(maxi as libc::c_uint) as isize,
                ) = U[0 as libc::c_int as usize] * vi
                + U[2 as libc::c_int as usize] * vj;
            *((*LS).data)
                .as_mut_ptr()
                .offset(
                    (i_9 as libc::c_uint)
                        .wrapping_mul((*LS).ncols)
                        .wrapping_add(maxj as libc::c_uint) as isize,
                ) = U[1 as libc::c_int as usize] * vi
                + U[3 as libc::c_int as usize] * vj;
            i_9 += 1;
        }
        let mut i_10: libc::c_int = 0 as libc::c_int;
        while (i_10 as libc::c_uint) < (*RS).nrows {
            let mut vi_0: libc::c_double = *((*RS).data)
                .as_mut_ptr()
                .offset(
                    (i_10 as libc::c_uint)
                        .wrapping_mul((*RS).ncols)
                        .wrapping_add(maxi as libc::c_uint) as isize,
                );
            let mut vj_0: libc::c_double = *((*RS).data)
                .as_mut_ptr()
                .offset(
                    (i_10 as libc::c_uint)
                        .wrapping_mul((*RS).ncols)
                        .wrapping_add(maxj as libc::c_uint) as isize,
                );
            *((*RS).data)
                .as_mut_ptr()
                .offset(
                    (i_10 as libc::c_uint)
                        .wrapping_mul((*RS).ncols)
                        .wrapping_add(maxi as libc::c_uint) as isize,
                ) = V[0 as libc::c_int as usize] * vi_0
                + V[2 as libc::c_int as usize] * vj_0;
            *((*RS).data)
                .as_mut_ptr()
                .offset(
                    (i_10 as libc::c_uint)
                        .wrapping_mul((*RS).ncols)
                        .wrapping_add(maxj as libc::c_uint) as isize,
                ) = V[1 as libc::c_int as usize] * vi_0
                + V[3 as libc::c_int as usize] * vj_0;
            i_10 += 1;
        }
        let mut i_11: libc::c_int = 0 as libc::c_int;
        while (i_11 as libc::c_uint) < (*B).ncols {
            let mut vi_1: libc::c_double = *((*B).data)
                .as_mut_ptr()
                .offset(
                    (maxi as libc::c_uint)
                        .wrapping_mul((*B).ncols)
                        .wrapping_add(i_11 as libc::c_uint) as isize,
                );
            let mut vj_1: libc::c_double = *((*B).data)
                .as_mut_ptr()
                .offset(
                    (maxj as libc::c_uint)
                        .wrapping_mul((*B).ncols)
                        .wrapping_add(i_11 as libc::c_uint) as isize,
                );
            *((*B).data)
                .as_mut_ptr()
                .offset(
                    (maxi as libc::c_uint)
                        .wrapping_mul((*B).ncols)
                        .wrapping_add(i_11 as libc::c_uint) as isize,
                ) = U[0 as libc::c_int as usize] * vi_1
                + U[2 as libc::c_int as usize] * vj_1;
            *((*B).data)
                .as_mut_ptr()
                .offset(
                    (maxj as libc::c_uint)
                        .wrapping_mul((*B).ncols)
                        .wrapping_add(i_11 as libc::c_uint) as isize,
                ) = U[1 as libc::c_int as usize] * vi_1
                + U[3 as libc::c_int as usize] * vj_1;
            i_11 += 1;
        }
        let mut i_12: libc::c_int = 0 as libc::c_int;
        while (i_12 as libc::c_uint) < (*B).nrows {
            let mut vi_2: libc::c_double = *((*B).data)
                .as_mut_ptr()
                .offset(
                    (i_12 as libc::c_uint)
                        .wrapping_mul((*B).ncols)
                        .wrapping_add(maxi as libc::c_uint) as isize,
                );
            let mut vj_2: libc::c_double = *((*B).data)
                .as_mut_ptr()
                .offset(
                    (i_12 as libc::c_uint)
                        .wrapping_mul((*B).ncols)
                        .wrapping_add(maxj as libc::c_uint) as isize,
                );
            *((*B).data)
                .as_mut_ptr()
                .offset(
                    (i_12 as libc::c_uint)
                        .wrapping_mul((*B).ncols)
                        .wrapping_add(maxi as libc::c_uint) as isize,
                ) = V[0 as libc::c_int as usize] * vi_2
                + V[2 as libc::c_int as usize] * vj_2;
            *((*B).data)
                .as_mut_ptr()
                .offset(
                    (i_12 as libc::c_uint)
                        .wrapping_mul((*B).ncols)
                        .wrapping_add(maxj as libc::c_uint) as isize,
                ) = V[1 as libc::c_int as usize] * vi_2
                + V[3 as libc::c_int as usize] * vj_2;
            i_12 += 1;
        }
        iter += 1;
    }
    free(maxrowidx as *mut libc::c_void);
    if flags & 1 as libc::c_int == 0 && iter == maxiters {
        fflush(stderr);
    }
    let mut idxs: *mut libc::c_int = malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((*A).ncols as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut vals: *mut libc::c_double = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*A).ncols as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut i_13: libc::c_int = 0 as libc::c_int;
    while (i_13 as libc::c_uint) < (*A).ncols {
        *idxs.offset(i_13 as isize) = i_13;
        *vals
            .offset(
                i_13 as isize,
            ) = *((*B).data)
            .as_mut_ptr()
            .offset(
                (i_13 as libc::c_uint)
                    .wrapping_mul((*B).ncols)
                    .wrapping_add(i_13 as libc::c_uint) as isize,
            );
        i_13 += 1;
    }
    let mut changed: libc::c_int = 0;
    loop {
        changed = 0 as libc::c_int;
        let mut i_14: libc::c_int = 0 as libc::c_int;
        while ((i_14 + 1 as libc::c_int) as libc::c_uint) < (*A).ncols {
            if fabs(*vals.offset((i_14 + 1 as libc::c_int) as isize))
                > fabs(*vals.offset(i_14 as isize))
            {
                let mut tmpi: libc::c_int = *idxs.offset(i_14 as isize);
                *idxs
                    .offset(
                        i_14 as isize,
                    ) = *idxs.offset((i_14 + 1 as libc::c_int) as isize);
                *idxs.offset((i_14 + 1 as libc::c_int) as isize) = tmpi;
                let mut tmpv: libc::c_double = *vals.offset(i_14 as isize);
                *vals
                    .offset(
                        i_14 as isize,
                    ) = *vals.offset((i_14 + 1 as libc::c_int) as isize);
                *vals.offset((i_14 + 1 as libc::c_int) as isize) = tmpv;
                changed = 1 as libc::c_int;
            }
            i_14 += 1;
        }
        if !(changed != 0) {
            break;
        }
    }
    let mut LP: *mut matd_t = matd_identity((*A).nrows as libc::c_int);
    let mut RP: *mut matd_t = matd_identity((*A).ncols as libc::c_int);
    let mut i_15: libc::c_int = 0 as libc::c_int;
    while (i_15 as libc::c_uint) < (*A).ncols {
        *((*LP).data)
            .as_mut_ptr()
            .offset(
                (*idxs.offset(i_15 as isize) as libc::c_uint)
                    .wrapping_mul((*LP).ncols)
                    .wrapping_add(*idxs.offset(i_15 as isize) as libc::c_uint) as isize,
            ) = 0 as libc::c_int as libc::c_double;
        *((*RP).data)
            .as_mut_ptr()
            .offset(
                (*idxs.offset(i_15 as isize) as libc::c_uint)
                    .wrapping_mul((*RP).ncols)
                    .wrapping_add(*idxs.offset(i_15 as isize) as libc::c_uint) as isize,
            ) = 0 as libc::c_int as libc::c_double;
        *((*LP).data)
            .as_mut_ptr()
            .offset(
                (*idxs.offset(i_15 as isize) as libc::c_uint)
                    .wrapping_mul((*LP).ncols)
                    .wrapping_add(i_15 as libc::c_uint) as isize,
            ) = (if *vals.offset(i_15 as isize) < 0 as libc::c_int as libc::c_double {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        }) as libc::c_double;
        *((*RP).data)
            .as_mut_ptr()
            .offset(
                (*idxs.offset(i_15 as isize) as libc::c_uint)
                    .wrapping_mul((*RP).ncols)
                    .wrapping_add(i_15 as libc::c_uint) as isize,
            ) = 1 as libc::c_int as libc::c_double;
        i_15 += 1;
    }
    free(idxs as *mut libc::c_void);
    free(vals as *mut libc::c_void);
    B = matd_op(b"M'*F*M\0" as *const u8 as *const libc::c_char, LP, B, RP);
    LS = matd_op(b"F*M\0" as *const u8 as *const libc::c_char, LS, LP);
    RS = matd_op(b"F*M\0" as *const u8 as *const libc::c_char, RS, RP);
    matd_destroy(LP);
    matd_destroy(RP);
    let mut res: matd_svd_t = matd_svd_t {
        U: 0 as *mut matd_t,
        S: 0 as *mut matd_t,
        V: 0 as *mut matd_t,
    };
    memset(
        &mut res as *mut matd_svd_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<matd_svd_t>() as libc::c_ulong,
    );
    let mut i_16: libc::c_int = 0 as libc::c_int;
    while (i_16 as libc::c_uint) < (*B).nrows {
        let mut j_8: libc::c_int = 0 as libc::c_int;
        while (j_8 as libc::c_uint) < (*B).ncols {
            if i_16 != j_8 {
                *((*B).data)
                    .as_mut_ptr()
                    .offset(
                        (i_16 as libc::c_uint)
                            .wrapping_mul((*B).ncols)
                            .wrapping_add(j_8 as libc::c_uint) as isize,
                    ) = 0 as libc::c_int as libc::c_double;
            }
            j_8 += 1;
        }
        i_16 += 1;
    }
    res.U = LS;
    res.S = B;
    res.V = RS;
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn matd_svd(mut A: *mut matd_t) -> matd_svd_t {
    return matd_svd_flags(A, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn matd_svd_flags(
    mut A: *mut matd_t,
    mut flags: libc::c_int,
) -> matd_svd_t {
    let mut res: matd_svd_t = matd_svd_t {
        U: 0 as *mut matd_t,
        S: 0 as *mut matd_t,
        V: 0 as *mut matd_t,
    };
    if (*A).ncols <= (*A).nrows {
        res = matd_svd_tall(A, flags);
    } else {
        let mut At: *mut matd_t = matd_transpose(A);
        let mut tmp: matd_svd_t = matd_svd_tall(At, flags);
        memset(
            &mut res as *mut matd_svd_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<matd_svd_t>() as libc::c_ulong,
        );
        res.U = tmp.V;
        res.S = matd_transpose(tmp.S);
        res.V = tmp.U;
        matd_destroy(tmp.S);
        matd_destroy(At);
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn matd_plu(mut a: *const matd_t) -> *mut matd_plu_t {
    let mut piv: *mut libc::c_uint = calloc(
        (*a).nrows as libc::c_ulong,
        ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
    ) as *mut libc::c_uint;
    let mut pivsign: libc::c_int = 1 as libc::c_int;
    let mut lu: *mut matd_t = matd_copy(a);
    let mut mlu: *mut matd_plu_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<matd_plu_t>() as libc::c_ulong,
    ) as *mut matd_plu_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_uint) < (*a).nrows {
        *piv.offset(i as isize) = i as libc::c_uint;
        i += 1;
    }
    let mut j: libc::c_int = 0 as libc::c_int;
    while (j as libc::c_uint) < (*a).ncols {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while (i_0 as libc::c_uint) < (*a).nrows {
            let mut kmax: libc::c_int = if i_0 < j { i_0 } else { j };
            let mut acc: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut k: libc::c_int = 0 as libc::c_int;
            while k < kmax {
                acc
                    += *((*lu).data)
                        .as_mut_ptr()
                        .offset(
                            (i_0 as libc::c_uint)
                                .wrapping_mul((*lu).ncols)
                                .wrapping_add(k as libc::c_uint) as isize,
                        )
                        * *((*lu).data)
                            .as_mut_ptr()
                            .offset(
                                (k as libc::c_uint)
                                    .wrapping_mul((*lu).ncols)
                                    .wrapping_add(j as libc::c_uint) as isize,
                            );
                k += 1;
            }
            *((*lu).data)
                .as_mut_ptr()
                .offset(
                    (i_0 as libc::c_uint)
                        .wrapping_mul((*lu).ncols)
                        .wrapping_add(j as libc::c_uint) as isize,
                ) -= acc;
            i_0 += 1;
        }
        let mut p: libc::c_int = j;
        let mut i_1: libc::c_int = j + 1 as libc::c_int;
        while (i_1 as libc::c_uint) < (*lu).nrows {
            if fabs(
                *((*lu).data)
                    .as_mut_ptr()
                    .offset(
                        (i_1 as libc::c_uint)
                            .wrapping_mul((*lu).ncols)
                            .wrapping_add(j as libc::c_uint) as isize,
                    ),
            )
                > fabs(
                    *((*lu).data)
                        .as_mut_ptr()
                        .offset(
                            (p as libc::c_uint)
                                .wrapping_mul((*lu).ncols)
                                .wrapping_add(j as libc::c_uint) as isize,
                        ),
                )
            {
                p = i_1;
            }
            i_1 += 1;
        }
        if p != j {
            let mut tmp: *mut libc::c_double = malloc(
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul((*lu).ncols as libc::c_ulong),
            ) as *mut libc::c_double;
            memcpy(
                tmp as *mut libc::c_void,
                &mut *((*lu).data)
                    .as_mut_ptr()
                    .offset(
                        (p as libc::c_uint)
                            .wrapping_mul((*lu).ncols)
                            .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                    ) as *mut libc::c_double as *const libc::c_void,
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul((*lu).ncols as libc::c_ulong),
            );
            memcpy(
                &mut *((*lu).data)
                    .as_mut_ptr()
                    .offset(
                        (p as libc::c_uint)
                            .wrapping_mul((*lu).ncols)
                            .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                    ) as *mut libc::c_double as *mut libc::c_void,
                &mut *((*lu).data)
                    .as_mut_ptr()
                    .offset(
                        (j as libc::c_uint)
                            .wrapping_mul((*lu).ncols)
                            .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                    ) as *mut libc::c_double as *const libc::c_void,
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul((*lu).ncols as libc::c_ulong),
            );
            memcpy(
                &mut *((*lu).data)
                    .as_mut_ptr()
                    .offset(
                        (j as libc::c_uint)
                            .wrapping_mul((*lu).ncols)
                            .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                    ) as *mut libc::c_double as *mut libc::c_void,
                tmp as *const libc::c_void,
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul((*lu).ncols as libc::c_ulong),
            );
            let mut k_0: libc::c_int = *piv.offset(p as isize) as libc::c_int;
            *piv.offset(p as isize) = *piv.offset(j as isize);
            *piv.offset(j as isize) = k_0 as libc::c_uint;
            pivsign = -pivsign;
            free(tmp as *mut libc::c_void);
        }
        let mut LUjj: libc::c_double = *((*lu).data)
            .as_mut_ptr()
            .offset(
                (j as libc::c_uint)
                    .wrapping_mul((*lu).ncols)
                    .wrapping_add(j as libc::c_uint) as isize,
            );
        if fabs(LUjj) < 1e-8f64 {
            (*mlu).singular = 1 as libc::c_int;
        }
        if (j as libc::c_uint) < (*lu).ncols && (j as libc::c_uint) < (*lu).nrows
            && LUjj != 0 as libc::c_int as libc::c_double
        {
            LUjj = 1.0f64 / LUjj;
            let mut i_2: libc::c_int = j + 1 as libc::c_int;
            while (i_2 as libc::c_uint) < (*lu).nrows {
                *((*lu).data)
                    .as_mut_ptr()
                    .offset(
                        (i_2 as libc::c_uint)
                            .wrapping_mul((*lu).ncols)
                            .wrapping_add(j as libc::c_uint) as isize,
                    ) *= LUjj;
                i_2 += 1;
            }
        }
        j += 1;
    }
    let ref mut fresh13 = (*mlu).lu;
    *fresh13 = lu;
    let ref mut fresh14 = (*mlu).piv;
    *fresh14 = piv;
    (*mlu).pivsign = pivsign;
    return mlu;
}
#[no_mangle]
pub unsafe extern "C" fn matd_plu_destroy(mut mlu: *mut matd_plu_t) {
    matd_destroy((*mlu).lu);
    free((*mlu).piv as *mut libc::c_void);
    memset(
        mlu as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<matd_plu_t>() as libc::c_ulong,
    );
    free(mlu as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn matd_plu_det(mut mlu: *const matd_plu_t) -> libc::c_double {
    let mut lu: *mut matd_t = (*mlu).lu;
    let mut det: libc::c_double = (*mlu).pivsign as libc::c_double;
    if (*lu).nrows == (*lu).ncols {
        let mut i: libc::c_int = 0 as libc::c_int;
        while (i as libc::c_uint) < (*lu).ncols {
            det
                *= *((*lu).data)
                    .as_mut_ptr()
                    .offset(
                        (i as libc::c_uint)
                            .wrapping_mul((*lu).ncols)
                            .wrapping_add(i as libc::c_uint) as isize,
                    );
            i += 1;
        }
    }
    return det;
}
#[no_mangle]
pub unsafe extern "C" fn matd_plu_p(mut mlu: *const matd_plu_t) -> *mut matd_t {
    let mut lu: *mut matd_t = (*mlu).lu;
    let mut P: *mut matd_t = matd_create(
        (*lu).nrows as libc::c_int,
        (*lu).nrows as libc::c_int,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_uint) < (*lu).nrows {
        *((*P).data)
            .as_mut_ptr()
            .offset(
                (*((*mlu).piv).offset(i as isize))
                    .wrapping_mul((*P).ncols)
                    .wrapping_add(i as libc::c_uint) as isize,
            ) = 1 as libc::c_int as libc::c_double;
        i += 1;
    }
    return P;
}
#[no_mangle]
pub unsafe extern "C" fn matd_plu_l(mut mlu: *const matd_plu_t) -> *mut matd_t {
    let mut lu: *mut matd_t = (*mlu).lu;
    let mut L: *mut matd_t = matd_create(
        (*lu).nrows as libc::c_int,
        (*lu).ncols as libc::c_int,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_uint) < (*lu).nrows {
        *((*L).data)
            .as_mut_ptr()
            .offset(
                (i as libc::c_uint)
                    .wrapping_mul((*L).ncols)
                    .wrapping_add(i as libc::c_uint) as isize,
            ) = 1 as libc::c_int as libc::c_double;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < i {
            *((*L).data)
                .as_mut_ptr()
                .offset(
                    (i as libc::c_uint)
                        .wrapping_mul((*L).ncols)
                        .wrapping_add(j as libc::c_uint) as isize,
                ) = *((*lu).data)
                .as_mut_ptr()
                .offset(
                    (i as libc::c_uint)
                        .wrapping_mul((*lu).ncols)
                        .wrapping_add(j as libc::c_uint) as isize,
                );
            j += 1;
        }
        i += 1;
    }
    return L;
}
#[no_mangle]
pub unsafe extern "C" fn matd_plu_u(mut mlu: *const matd_plu_t) -> *mut matd_t {
    let mut lu: *mut matd_t = (*mlu).lu;
    let mut U: *mut matd_t = matd_create(
        (*lu).ncols as libc::c_int,
        (*lu).ncols as libc::c_int,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_uint) < (*lu).ncols {
        let mut j: libc::c_int = 0 as libc::c_int;
        while (j as libc::c_uint) < (*lu).ncols {
            if i <= j {
                *((*U).data)
                    .as_mut_ptr()
                    .offset(
                        (i as libc::c_uint)
                            .wrapping_mul((*U).ncols)
                            .wrapping_add(j as libc::c_uint) as isize,
                    ) = *((*lu).data)
                    .as_mut_ptr()
                    .offset(
                        (i as libc::c_uint)
                            .wrapping_mul((*lu).ncols)
                            .wrapping_add(j as libc::c_uint) as isize,
                    );
            }
            j += 1;
        }
        i += 1;
    }
    return U;
}
#[no_mangle]
pub unsafe extern "C" fn matd_plu_solve(
    mut mlu: *const matd_plu_t,
    mut b: *const matd_t,
) -> *mut matd_t {
    let mut x: *mut matd_t = matd_copy(b);
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_uint) < (*(*mlu).lu).nrows {
        memcpy(
            &mut *((*x).data)
                .as_mut_ptr()
                .offset(
                    (i as libc::c_uint)
                        .wrapping_mul((*x).ncols)
                        .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                ) as *mut libc::c_double as *mut libc::c_void,
            &*((*b).data)
                .as_ptr()
                .offset(
                    (*((*mlu).piv).offset(i as isize))
                        .wrapping_mul((*b).ncols)
                        .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                ) as *const libc::c_double as *const libc::c_void,
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul((*b).ncols as libc::c_ulong),
        );
        i += 1;
    }
    let mut k: libc::c_int = 0 as libc::c_int;
    while (k as libc::c_uint) < (*(*mlu).lu).nrows {
        let mut i_0: libc::c_int = k + 1 as libc::c_int;
        while (i_0 as libc::c_uint) < (*(*mlu).lu).nrows {
            let mut LUik: libc::c_double = -*((*(*mlu).lu).data)
                .as_mut_ptr()
                .offset(
                    (i_0 as libc::c_uint)
                        .wrapping_mul((*(*mlu).lu).ncols)
                        .wrapping_add(k as libc::c_uint) as isize,
                );
            let mut t: libc::c_int = 0 as libc::c_int;
            while (t as libc::c_uint) < (*b).ncols {
                *((*x).data)
                    .as_mut_ptr()
                    .offset(
                        (i_0 as libc::c_uint)
                            .wrapping_mul((*x).ncols)
                            .wrapping_add(t as libc::c_uint) as isize,
                    )
                    += *((*x).data)
                        .as_mut_ptr()
                        .offset(
                            (k as libc::c_uint)
                                .wrapping_mul((*x).ncols)
                                .wrapping_add(t as libc::c_uint) as isize,
                        ) * LUik;
                t += 1;
            }
            i_0 += 1;
        }
        k += 1;
    }
    let mut k_0: libc::c_int = ((*(*mlu).lu).ncols)
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
    while k_0 >= 0 as libc::c_int {
        let mut LUkk: libc::c_double = 1.0f64
            / *((*(*mlu).lu).data)
                .as_mut_ptr()
                .offset(
                    (k_0 as libc::c_uint)
                        .wrapping_mul((*(*mlu).lu).ncols)
                        .wrapping_add(k_0 as libc::c_uint) as isize,
                );
        let mut t_0: libc::c_int = 0 as libc::c_int;
        while (t_0 as libc::c_uint) < (*b).ncols {
            *((*x).data)
                .as_mut_ptr()
                .offset(
                    (k_0 as libc::c_uint)
                        .wrapping_mul((*x).ncols)
                        .wrapping_add(t_0 as libc::c_uint) as isize,
                ) *= LUkk;
            t_0 += 1;
        }
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < k_0 {
            let mut LUik_0: libc::c_double = -*((*(*mlu).lu).data)
                .as_mut_ptr()
                .offset(
                    (i_1 as libc::c_uint)
                        .wrapping_mul((*(*mlu).lu).ncols)
                        .wrapping_add(k_0 as libc::c_uint) as isize,
                );
            let mut t_1: libc::c_int = 0 as libc::c_int;
            while (t_1 as libc::c_uint) < (*b).ncols {
                *((*x).data)
                    .as_mut_ptr()
                    .offset(
                        (i_1 as libc::c_uint)
                            .wrapping_mul((*x).ncols)
                            .wrapping_add(t_1 as libc::c_uint) as isize,
                    )
                    += *((*x).data)
                        .as_mut_ptr()
                        .offset(
                            (k_0 as libc::c_uint)
                                .wrapping_mul((*x).ncols)
                                .wrapping_add(t_1 as libc::c_uint) as isize,
                        ) * LUik_0;
                t_1 += 1;
            }
            i_1 += 1;
        }
        k_0 -= 1;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn matd_solve(
    mut A: *mut matd_t,
    mut b: *mut matd_t,
) -> *mut matd_t {
    let mut mlu: *mut matd_plu_t = matd_plu(A);
    let mut x: *mut matd_t = matd_plu_solve(mlu, b);
    matd_plu_destroy(mlu);
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn matd_chol(mut A: *mut matd_t) -> *mut matd_chol_t {
    let mut N: libc::c_int = (*A).nrows as libc::c_int;
    let mut U: *mut matd_t = matd_copy(A);
    let mut is_spd: libc::c_int = 1 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < N {
        let mut d: libc::c_double = *((*U).data)
            .as_mut_ptr()
            .offset(
                (i as libc::c_uint)
                    .wrapping_mul((*U).ncols)
                    .wrapping_add(i as libc::c_uint) as isize,
            );
        is_spd &= (d > 0 as libc::c_int as libc::c_double) as libc::c_int;
        if d < 1e-8f64 {
            d = 1e-8f64;
        }
        d = 1.0f64 / sqrt(d);
        let mut j: libc::c_int = i;
        while j < N {
            *((*U).data)
                .as_mut_ptr()
                .offset(
                    (i as libc::c_uint)
                        .wrapping_mul((*U).ncols)
                        .wrapping_add(j as libc::c_uint) as isize,
                ) *= d;
            j += 1;
        }
        let mut j_0: libc::c_int = i + 1 as libc::c_int;
        while j_0 < N {
            let mut s: libc::c_double = *((*U).data)
                .as_mut_ptr()
                .offset(
                    (i as libc::c_uint)
                        .wrapping_mul((*U).ncols)
                        .wrapping_add(j_0 as libc::c_uint) as isize,
                );
            if !(s == 0 as libc::c_int as libc::c_double) {
                let mut k: libc::c_int = j_0;
                while k < N {
                    *((*U).data)
                        .as_mut_ptr()
                        .offset(
                            (j_0 as libc::c_uint)
                                .wrapping_mul((*U).ncols)
                                .wrapping_add(k as libc::c_uint) as isize,
                        )
                        -= *((*U).data)
                            .as_mut_ptr()
                            .offset(
                                (i as libc::c_uint)
                                    .wrapping_mul((*U).ncols)
                                    .wrapping_add(k as libc::c_uint) as isize,
                            ) * s;
                    k += 1;
                }
            }
            j_0 += 1;
        }
        i += 1;
    }
    let mut chol: *mut matd_chol_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<matd_chol_t>() as libc::c_ulong,
    ) as *mut matd_chol_t;
    (*chol).is_spd = is_spd;
    let ref mut fresh15 = (*chol).u;
    *fresh15 = U;
    return chol;
}
#[no_mangle]
pub unsafe extern "C" fn matd_chol_destroy(mut chol: *mut matd_chol_t) {
    matd_destroy((*chol).u);
    free(chol as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn matd_ltransposetriangle_solve(
    mut u: *mut matd_t,
    mut b: *const libc::c_double,
    mut x: *mut libc::c_double,
) {
    let mut n: libc::c_int = (*u).ncols as libc::c_int;
    memcpy(
        x as *mut libc::c_void,
        b as *const libc::c_void,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        *x.offset(i as isize)
            /= *((*u).data)
                .as_mut_ptr()
                .offset(
                    (i as libc::c_uint)
                        .wrapping_mul((*u).ncols)
                        .wrapping_add(i as libc::c_uint) as isize,
                );
        let mut j: libc::c_int = i + 1 as libc::c_int;
        while (j as libc::c_uint) < (*u).ncols {
            *x.offset(j as isize)
                -= *x.offset(i as isize)
                    * *((*u).data)
                        .as_mut_ptr()
                        .offset(
                            (i as libc::c_uint)
                                .wrapping_mul((*u).ncols)
                                .wrapping_add(j as libc::c_uint) as isize,
                        );
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn matd_ltriangle_solve(
    mut L: *mut matd_t,
    mut b: *const libc::c_double,
    mut x: *mut libc::c_double,
) {
    let mut n: libc::c_int = (*L).ncols as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let mut acc: libc::c_double = *b.offset(i as isize);
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < i {
            acc
                -= *((*L).data)
                    .as_mut_ptr()
                    .offset(
                        (i as libc::c_uint)
                            .wrapping_mul((*L).ncols)
                            .wrapping_add(j as libc::c_uint) as isize,
                    ) * *x.offset(j as isize);
            j += 1;
        }
        *x
            .offset(
                i as isize,
            ) = acc
            / *((*L).data)
                .as_mut_ptr()
                .offset(
                    (i as libc::c_uint)
                        .wrapping_mul((*L).ncols)
                        .wrapping_add(i as libc::c_uint) as isize,
                );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn matd_utriangle_solve(
    mut u: *mut matd_t,
    mut b: *const libc::c_double,
    mut x: *mut libc::c_double,
) {
    let mut i: libc::c_int = ((*u).ncols).wrapping_sub(1 as libc::c_int as libc::c_uint)
        as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut bi: libc::c_double = *b.offset(i as isize);
        let mut diag: libc::c_double = *((*u).data)
            .as_mut_ptr()
            .offset(
                (i as libc::c_uint)
                    .wrapping_mul((*u).ncols)
                    .wrapping_add(i as libc::c_uint) as isize,
            );
        let mut j: libc::c_int = i + 1 as libc::c_int;
        while (j as libc::c_uint) < (*u).ncols {
            bi
                -= *((*u).data)
                    .as_mut_ptr()
                    .offset(
                        (i as libc::c_uint)
                            .wrapping_mul((*u).ncols)
                            .wrapping_add(j as libc::c_uint) as isize,
                    ) * *x.offset(j as isize);
            j += 1;
        }
        *x.offset(i as isize) = bi / diag;
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn matd_chol_solve(
    mut chol: *const matd_chol_t,
    mut b: *const matd_t,
) -> *mut matd_t {
    let mut u: *mut matd_t = (*chol).u;
    let mut x: *mut matd_t = matd_copy(b);
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_uint) < (*u).nrows {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < i {
            let mut k: libc::c_int = 0 as libc::c_int;
            while (k as libc::c_uint) < (*b).ncols {
                *((*x).data)
                    .as_mut_ptr()
                    .offset(
                        (i as libc::c_uint)
                            .wrapping_mul((*x).ncols)
                            .wrapping_add(k as libc::c_uint) as isize,
                    )
                    -= *((*u).data)
                        .as_mut_ptr()
                        .offset(
                            (j as libc::c_uint)
                                .wrapping_mul((*u).ncols)
                                .wrapping_add(i as libc::c_uint) as isize,
                        )
                        * *((*x).data)
                            .as_mut_ptr()
                            .offset(
                                (j as libc::c_uint)
                                    .wrapping_mul((*x).ncols)
                                    .wrapping_add(k as libc::c_uint) as isize,
                            );
                k += 1;
            }
            j += 1;
        }
        let mut k_0: libc::c_int = 0 as libc::c_int;
        while (k_0 as libc::c_uint) < (*b).ncols {
            *((*x).data)
                .as_mut_ptr()
                .offset(
                    (i as libc::c_uint)
                        .wrapping_mul((*x).ncols)
                        .wrapping_add(k_0 as libc::c_uint) as isize,
                )
                /= *((*u).data)
                    .as_mut_ptr()
                    .offset(
                        (i as libc::c_uint)
                            .wrapping_mul((*u).ncols)
                            .wrapping_add(i as libc::c_uint) as isize,
                    );
            k_0 += 1;
        }
        i += 1;
    }
    let mut k_1: libc::c_int = ((*u).ncols)
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
    while k_1 >= 0 as libc::c_int {
        let mut LUkk: libc::c_double = 1.0f64
            / *((*u).data)
                .as_mut_ptr()
                .offset(
                    (k_1 as libc::c_uint)
                        .wrapping_mul((*u).ncols)
                        .wrapping_add(k_1 as libc::c_uint) as isize,
                );
        let mut t: libc::c_int = 0 as libc::c_int;
        while (t as libc::c_uint) < (*b).ncols {
            *((*x).data)
                .as_mut_ptr()
                .offset(
                    (k_1 as libc::c_uint)
                        .wrapping_mul((*x).ncols)
                        .wrapping_add(t as libc::c_uint) as isize,
                ) *= LUkk;
            t += 1;
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < k_1 {
            let mut LUik: libc::c_double = -*((*u).data)
                .as_mut_ptr()
                .offset(
                    (i_0 as libc::c_uint)
                        .wrapping_mul((*u).ncols)
                        .wrapping_add(k_1 as libc::c_uint) as isize,
                );
            let mut t_0: libc::c_int = 0 as libc::c_int;
            while (t_0 as libc::c_uint) < (*b).ncols {
                *((*x).data)
                    .as_mut_ptr()
                    .offset(
                        (i_0 as libc::c_uint)
                            .wrapping_mul((*x).ncols)
                            .wrapping_add(t_0 as libc::c_uint) as isize,
                    )
                    += *((*x).data)
                        .as_mut_ptr()
                        .offset(
                            (k_1 as libc::c_uint)
                                .wrapping_mul((*x).ncols)
                                .wrapping_add(t_0 as libc::c_uint) as isize,
                        ) * LUik;
                t_0 += 1;
            }
            i_0 += 1;
        }
        k_1 -= 1;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn matd_chol_inverse(mut a: *mut matd_t) -> *mut matd_t {
    let mut chol: *mut matd_chol_t = matd_chol(a);
    let mut eye: *mut matd_t = matd_identity((*a).nrows as libc::c_int);
    let mut inv: *mut matd_t = matd_chol_solve(chol, eye);
    matd_destroy(eye);
    matd_chol_destroy(chol);
    return inv;
}
#[no_mangle]
pub unsafe extern "C" fn matd_max(mut m: *mut matd_t) -> libc::c_double {
    let mut d: libc::c_double = -1.7976931348623157e+308f64;
    let mut x: libc::c_int = 0 as libc::c_int;
    while (x as libc::c_uint) < (*m).nrows {
        let mut y: libc::c_int = 0 as libc::c_int;
        while (y as libc::c_uint) < (*m).ncols {
            if *((*m).data)
                .as_mut_ptr()
                .offset(
                    (x as libc::c_uint)
                        .wrapping_mul((*m).ncols)
                        .wrapping_add(y as libc::c_uint) as isize,
                ) > d
            {
                d = *((*m).data)
                    .as_mut_ptr()
                    .offset(
                        (x as libc::c_uint)
                            .wrapping_mul((*m).ncols)
                            .wrapping_add(y as libc::c_uint) as isize,
                    );
            }
            y += 1;
        }
        x += 1;
    }
    return d;
}
