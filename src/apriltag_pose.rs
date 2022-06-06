use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn atan(_: libc::c_double) -> libc::c_double;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn matd_create(rows: libc::c_int, cols: libc::c_int) -> *mut matd_t;
    fn matd_create_data(
        rows: libc::c_int,
        cols: libc::c_int,
        data: *const libc::c_double,
    ) -> *mut matd_t;
    fn matd_identity(dim: libc::c_int) -> *mut matd_t;
    fn matd_get(m: *const matd_t, row: libc::c_int, col: libc::c_int) -> libc::c_double;
    fn matd_put(
        m: *mut matd_t,
        row: libc::c_int,
        col: libc::c_int,
        value: libc::c_double,
    );
    fn matd_copy(m: *const matd_t) -> *mut matd_t;
    fn matd_add_inplace(a: *mut matd_t, b: *const matd_t);
    fn matd_subtract(a: *const matd_t, b: *const matd_t) -> *mut matd_t;
    fn matd_scale_inplace(a: *mut matd_t, s: libc::c_double);
    fn matd_multiply(a: *const matd_t, b: *const matd_t) -> *mut matd_t;
    fn matd_det(a: *const matd_t) -> libc::c_double;
    fn matd_inverse(a: *const matd_t) -> *mut matd_t;
    fn matd_vec_normalize(a: *const matd_t) -> *mut matd_t;
    fn matd_crossproduct(a: *const matd_t, b: *const matd_t) -> *mut matd_t;
    fn matd_op(expr: *const libc::c_char, _: ...) -> *mut matd_t;
    fn matd_destroy(m: *mut matd_t);
    fn matd_svd(A: *mut matd_t) -> matd_svd_t;
    fn homography_to_pose(
        H: *const matd_t,
        fx: libc::c_double,
        fy: libc::c_double,
        cx: libc::c_double,
        cy: libc::c_double,
    ) -> *mut matd_t;
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub struct matd_svd_t {
    pub U: *mut matd_t,
    pub S: *mut matd_t,
    pub V: *mut matd_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct apriltag_detection {
    pub family: *mut apriltag_family_t,
    pub id: libc::c_int,
    pub hamming: libc::c_int,
    pub decision_margin: libc::c_float,
    pub H: *mut matd_t,
    pub c: [libc::c_double; 2],
    pub p: [[libc::c_double; 2]; 4],
}
pub type apriltag_detection_t = apriltag_detection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct apriltag_detection_info_t {
    pub det: *mut apriltag_detection_t,
    pub tagsize: libc::c_double,
    pub fx: libc::c_double,
    pub fy: libc::c_double,
    pub cx: libc::c_double,
    pub cy: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct apriltag_pose_t {
    pub R: *mut matd_t,
    pub t: *mut matd_t,
}
#[no_mangle]
pub unsafe extern "C" fn calculate_F(mut v: *mut matd_t) -> *mut matd_t {
    let mut outer_product: *mut matd_t = matd_op(
        b"MM'\0" as *const u8 as *const libc::c_char,
        v,
        v,
        v,
        v,
    );
    let mut inner_product: *mut matd_t = matd_op(
        b"M'M\0" as *const u8 as *const libc::c_char,
        v,
        v,
    );
    matd_scale_inplace(
        outer_product,
        1.0f64 / *((*inner_product).data).as_mut_ptr().offset(0 as libc::c_int as isize),
    );
    matd_destroy(inner_product);
    return outer_product;
}
#[no_mangle]
pub unsafe extern "C" fn matd_to_double(mut a: *mut matd_t) -> libc::c_double {
    let mut d: libc::c_double = *((*a).data)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize);
    matd_destroy(a);
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn orthogonal_iteration(
    mut v: *mut *mut matd_t,
    mut p: *mut *mut matd_t,
    mut t: *mut *mut matd_t,
    mut R: *mut *mut matd_t,
    mut n_points: libc::c_int,
    mut n_steps: libc::c_int,
) -> libc::c_double {
    let mut p_mean: *mut matd_t = matd_create(3 as libc::c_int, 1 as libc::c_int);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n_points {
        matd_add_inplace(p_mean, *p.offset(i as isize));
        i += 1;
    }
    matd_scale_inplace(p_mean, 1.0f64 / n_points as libc::c_double);
    let mut p_res: *mut *mut matd_t = malloc(
        (::std::mem::size_of::<*mut matd_t>() as libc::c_ulong)
            .wrapping_mul(n_points as libc::c_ulong),
    ) as *mut *mut matd_t;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n_points {
        let ref mut fresh0 = *p_res.offset(i_0 as isize);
        *fresh0 = matd_op(
            b"M-M\0" as *const u8 as *const libc::c_char,
            *p.offset(i_0 as isize),
            p_mean,
        );
        i_0 += 1;
    }
    let mut F: *mut *mut matd_t = malloc(
        (::std::mem::size_of::<*mut matd_t>() as libc::c_ulong)
            .wrapping_mul(n_points as libc::c_ulong),
    ) as *mut *mut matd_t;
    let mut avg_F: *mut matd_t = matd_create(3 as libc::c_int, 3 as libc::c_int);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < n_points {
        let ref mut fresh1 = *F.offset(i_1 as isize);
        *fresh1 = calculate_F(*v.offset(i_1 as isize));
        matd_add_inplace(avg_F, *F.offset(i_1 as isize));
        i_1 += 1;
    }
    matd_scale_inplace(avg_F, 1.0f64 / n_points as libc::c_double);
    let mut I3: *mut matd_t = matd_identity(3 as libc::c_int);
    let mut M1: *mut matd_t = matd_subtract(I3, avg_F);
    let mut M1_inv: *mut matd_t = matd_inverse(M1);
    matd_destroy(avg_F);
    matd_destroy(M1);
    let mut prev_error: libc::c_double = ::std::f64::INFINITY;
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < n_steps {
        let mut M2: *mut matd_t = matd_create(3 as libc::c_int, 1 as libc::c_int);
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < n_points {
            let mut M2_update: *mut matd_t = matd_op(
                b"(M - M)*M*M\0" as *const u8 as *const libc::c_char,
                *F.offset(j as isize),
                I3,
                *R,
                *p.offset(j as isize),
            );
            matd_add_inplace(M2, M2_update);
            matd_destroy(M2_update);
            j += 1;
        }
        matd_scale_inplace(M2, 1.0f64 / n_points as libc::c_double);
        matd_destroy(*t);
        *t = matd_multiply(M1_inv, M2);
        matd_destroy(M2);
        let mut q: *mut *mut matd_t = malloc(
            (::std::mem::size_of::<*mut matd_t>() as libc::c_ulong)
                .wrapping_mul(n_points as libc::c_ulong),
        ) as *mut *mut matd_t;
        let mut q_mean: *mut matd_t = matd_create(3 as libc::c_int, 1 as libc::c_int);
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < n_points {
            let ref mut fresh2 = *q.offset(j_0 as isize);
            *fresh2 = matd_op(
                b"M*(M*M+M)\0" as *const u8 as *const libc::c_char,
                *F.offset(j_0 as isize),
                *R,
                *p.offset(j_0 as isize),
                *t,
            );
            matd_add_inplace(q_mean, *q.offset(j_0 as isize));
            j_0 += 1;
        }
        matd_scale_inplace(q_mean, 1.0f64 / n_points as libc::c_double);
        let mut M3: *mut matd_t = matd_create(3 as libc::c_int, 3 as libc::c_int);
        let mut j_1: libc::c_int = 0 as libc::c_int;
        while j_1 < n_points {
            let mut M3_update: *mut matd_t = matd_op(
                b"(M-M)*M'\0" as *const u8 as *const libc::c_char,
                *q.offset(j_1 as isize),
                q_mean,
                *p_res.offset(j_1 as isize),
            );
            matd_add_inplace(M3, M3_update);
            matd_destroy(M3_update);
            j_1 += 1;
        }
        let mut M3_svd: matd_svd_t = matd_svd(M3);
        matd_destroy(M3);
        matd_destroy(*R);
        *R = matd_op(b"M*M'\0" as *const u8 as *const libc::c_char, M3_svd.U, M3_svd.V);
        let mut R_det: libc::c_double = matd_det(*R);
        if R_det < 0 as libc::c_int as libc::c_double {
            matd_put(
                *R,
                0 as libc::c_int,
                2 as libc::c_int,
                -matd_get(*R, 0 as libc::c_int, 2 as libc::c_int),
            );
            matd_put(
                *R,
                1 as libc::c_int,
                2 as libc::c_int,
                -matd_get(*R, 1 as libc::c_int, 2 as libc::c_int),
            );
            matd_put(
                *R,
                2 as libc::c_int,
                2 as libc::c_int,
                -matd_get(*R, 2 as libc::c_int, 2 as libc::c_int),
            );
        }
        matd_destroy(M3_svd.U);
        matd_destroy(M3_svd.S);
        matd_destroy(M3_svd.V);
        matd_destroy(q_mean);
        let mut j_2: libc::c_int = 0 as libc::c_int;
        while j_2 < n_points {
            matd_destroy(*q.offset(j_2 as isize));
            j_2 += 1;
        }
        let mut error: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut j_3: libc::c_int = 0 as libc::c_int;
        while j_3 < 4 as libc::c_int {
            let mut err_vec: *mut matd_t = matd_op(
                b"(M-M)(MM+M)\0" as *const u8 as *const libc::c_char,
                I3,
                *F.offset(j_3 as isize),
                *R,
                *p.offset(j_3 as isize),
                *t,
            );
            error
                += matd_to_double(
                    matd_op(
                        b"M'M\0" as *const u8 as *const libc::c_char,
                        err_vec,
                        err_vec,
                    ),
                );
            matd_destroy(err_vec);
            j_3 += 1;
        }
        prev_error = error;
        free(q as *mut libc::c_void);
        i_2 += 1;
    }
    matd_destroy(I3);
    matd_destroy(M1_inv);
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < n_points {
        matd_destroy(*p_res.offset(i_3 as isize));
        matd_destroy(*F.offset(i_3 as isize));
        i_3 += 1;
    }
    free(p_res as *mut libc::c_void);
    free(F as *mut libc::c_void);
    matd_destroy(p_mean);
    return prev_error;
}
#[no_mangle]
pub unsafe extern "C" fn polyval(
    mut p: *mut libc::c_double,
    mut degree: libc::c_int,
    mut x: libc::c_double,
) -> libc::c_double {
    let mut ret: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i <= degree {
        ret += *p.offset(i as isize) * pow(x, i as libc::c_double);
        i += 1;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn solve_poly_approx(
    mut p: *mut libc::c_double,
    mut degree: libc::c_int,
    mut roots: *mut libc::c_double,
    mut n_roots: *mut libc::c_int,
) {
    static mut MAX_ROOT: libc::c_int = 1000 as libc::c_int;
    if degree == 1 as libc::c_int {
        if fabs(*p.offset(0 as libc::c_int as isize))
            > MAX_ROOT as libc::c_double * fabs(*p.offset(1 as libc::c_int as isize))
        {
            *n_roots = 0 as libc::c_int;
        } else {
            *roots
                .offset(
                    0 as libc::c_int as isize,
                ) = -*p.offset(0 as libc::c_int as isize)
                / *p.offset(1 as libc::c_int as isize);
            *n_roots = 1 as libc::c_int;
        }
        return;
    }
    let mut p_der: *mut libc::c_double = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(degree as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < degree {
        *p_der
            .offset(
                i as isize,
            ) = (i + 1 as libc::c_int) as libc::c_double
            * *p.offset((i + 1 as libc::c_int) as isize);
        i += 1;
    }
    let mut der_roots: *mut libc::c_double = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((degree - 1 as libc::c_int) as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut n_der_roots: libc::c_int = 0;
    solve_poly_approx(p_der, degree - 1 as libc::c_int, der_roots, &mut n_der_roots);
    *n_roots = 0 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 <= n_der_roots {
        let mut min: libc::c_double = 0.;
        if i_0 == 0 as libc::c_int {
            min = -MAX_ROOT as libc::c_double;
        } else {
            min = *der_roots.offset((i_0 - 1 as libc::c_int) as isize);
        }
        let mut max: libc::c_double = 0.;
        if i_0 == n_der_roots {
            max = MAX_ROOT as libc::c_double;
        } else {
            max = *der_roots.offset(i_0 as isize);
        }
        if polyval(p, degree, min) * polyval(p, degree, max)
            < 0 as libc::c_int as libc::c_double
        {
            let mut lower: libc::c_double = 0.;
            let mut upper: libc::c_double = 0.;
            if polyval(p, degree, min) < polyval(p, degree, max) {
                lower = min;
                upper = max;
            } else {
                lower = max;
                upper = min;
            }
            let mut root: libc::c_double = 0.5f64 * (lower + upper);
            let mut dx_old: libc::c_double = upper - lower;
            let mut dx: libc::c_double = dx_old;
            let mut f: libc::c_double = polyval(p, degree, root);
            let mut df: libc::c_double = polyval(p_der, degree - 1 as libc::c_int, root);
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < 100 as libc::c_int {
                if (f + df * (upper - root)) * (f + df * (lower - root))
                    > 0 as libc::c_int as libc::c_double
                    || fabs(2 as libc::c_int as libc::c_double * f) > fabs(dx_old * df)
                {
                    dx_old = dx;
                    dx = 0.5f64 * (upper - lower);
                    root = lower + dx;
                } else {
                    dx_old = dx;
                    dx = -f / df;
                    root += dx;
                }
                if root == upper || root == lower {
                    break;
                }
                f = polyval(p, degree, root);
                df = polyval(p_der, degree - 1 as libc::c_int, root);
                if f > 0 as libc::c_int as libc::c_double {
                    upper = root;
                } else {
                    lower = root;
                }
                j += 1;
            }
            let fresh3 = *n_roots;
            *n_roots = *n_roots + 1;
            *roots.offset(fresh3 as isize) = root;
        } else if polyval(p, degree, max) == 0 as libc::c_int as libc::c_double {
            let fresh4 = *n_roots;
            *n_roots = *n_roots + 1;
            *roots.offset(fresh4 as isize) = max;
        }
        i_0 += 1;
    }
    free(der_roots as *mut libc::c_void);
    free(p_der as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn fix_pose_ambiguities(
    mut v: *mut *mut matd_t,
    mut p: *mut *mut matd_t,
    mut t: *mut matd_t,
    mut R: *mut matd_t,
    mut n_points: libc::c_int,
) -> *mut matd_t {
    let mut I3: *mut matd_t = matd_identity(3 as libc::c_int);
    let mut R_t_3: *mut matd_t = matd_vec_normalize(t);
    let mut e_x: *mut matd_t = matd_create(3 as libc::c_int, 1 as libc::c_int);
    *((*e_x).data)
        .as_mut_ptr()
        .offset(
            (0 as libc::c_int as libc::c_uint)
                .wrapping_mul((*e_x).ncols)
                .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
        ) = 1 as libc::c_int as libc::c_double;
    let mut R_t_1_tmp: *mut matd_t = matd_op(
        b"M-(M'*M)*M\0" as *const u8 as *const libc::c_char,
        e_x,
        e_x,
        R_t_3,
        R_t_3,
    );
    let mut R_t_1: *mut matd_t = matd_vec_normalize(R_t_1_tmp);
    matd_destroy(e_x);
    matd_destroy(R_t_1_tmp);
    let mut R_t_2: *mut matd_t = matd_crossproduct(R_t_3, R_t_1);
    let mut R_t: *mut matd_t = matd_create_data(
        3 as libc::c_int,
        3 as libc::c_int,
        [
            *((*R_t_1).data)
                .as_mut_ptr()
                .offset(
                    (0 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*R_t_1).ncols)
                        .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                ),
            *((*R_t_1).data)
                .as_mut_ptr()
                .offset(
                    (0 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*R_t_1).ncols)
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                ),
            *((*R_t_1).data)
                .as_mut_ptr()
                .offset(
                    (0 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*R_t_1).ncols)
                        .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                ),
            *((*R_t_2).data)
                .as_mut_ptr()
                .offset(
                    (0 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*R_t_2).ncols)
                        .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                ),
            *((*R_t_2).data)
                .as_mut_ptr()
                .offset(
                    (0 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*R_t_2).ncols)
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                ),
            *((*R_t_2).data)
                .as_mut_ptr()
                .offset(
                    (0 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*R_t_2).ncols)
                        .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                ),
            *((*R_t_3).data)
                .as_mut_ptr()
                .offset(
                    (0 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*R_t_3).ncols)
                        .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                ),
            *((*R_t_3).data)
                .as_mut_ptr()
                .offset(
                    (0 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*R_t_3).ncols)
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                ),
            *((*R_t_3).data)
                .as_mut_ptr()
                .offset(
                    (0 as libc::c_int as libc::c_uint)
                        .wrapping_mul((*R_t_3).ncols)
                        .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                ),
        ]
            .as_mut_ptr(),
    );
    matd_destroy(R_t_1);
    matd_destroy(R_t_2);
    matd_destroy(R_t_3);
    let mut R_1_prime: *mut matd_t = matd_multiply(R_t, R);
    let mut r31: libc::c_double = *((*R_1_prime).data)
        .as_mut_ptr()
        .offset(
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*R_1_prime).ncols)
                .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
        );
    let mut r32: libc::c_double = *((*R_1_prime).data)
        .as_mut_ptr()
        .offset(
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*R_1_prime).ncols)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        );
    let mut hypotenuse: libc::c_double = sqrt(r31 * r31 + r32 * r32);
    if hypotenuse < 1e-100f64 {
        r31 = 1 as libc::c_int as libc::c_double;
        r32 = 0 as libc::c_int as libc::c_double;
        hypotenuse = 1 as libc::c_int as libc::c_double;
    }
    let mut R_z: *mut matd_t = matd_create_data(
        3 as libc::c_int,
        3 as libc::c_int,
        [
            r31 / hypotenuse,
            -r32 / hypotenuse,
            0 as libc::c_int as libc::c_double,
            r32 / hypotenuse,
            r31 / hypotenuse,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            1 as libc::c_int as libc::c_double,
        ]
            .as_mut_ptr(),
    );
    let mut R_trans: *mut matd_t = matd_multiply(R_1_prime, R_z);
    let mut sin_gamma: libc::c_double = -*((*R_trans).data)
        .as_mut_ptr()
        .offset(
            (0 as libc::c_int as libc::c_uint)
                .wrapping_mul((*R_trans).ncols)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        );
    let mut cos_gamma: libc::c_double = *((*R_trans).data)
        .as_mut_ptr()
        .offset(
            (1 as libc::c_int as libc::c_uint)
                .wrapping_mul((*R_trans).ncols)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        );
    let mut R_gamma: *mut matd_t = matd_create_data(
        3 as libc::c_int,
        3 as libc::c_int,
        [
            cos_gamma,
            -sin_gamma,
            0 as libc::c_int as libc::c_double,
            sin_gamma,
            cos_gamma,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            1 as libc::c_int as libc::c_double,
        ]
            .as_mut_ptr(),
    );
    let mut sin_beta: libc::c_double = -*((*R_trans).data)
        .as_mut_ptr()
        .offset(
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*R_trans).ncols)
                .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
        );
    let mut cos_beta: libc::c_double = *((*R_trans).data)
        .as_mut_ptr()
        .offset(
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*R_trans).ncols)
                .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
        );
    let mut t_initial: libc::c_double = atan2(sin_beta, cos_beta);
    matd_destroy(R_trans);
    let mut v_trans: *mut *mut matd_t = malloc(
        (::std::mem::size_of::<*mut matd_t>() as libc::c_ulong)
            .wrapping_mul(n_points as libc::c_ulong),
    ) as *mut *mut matd_t;
    let mut p_trans: *mut *mut matd_t = malloc(
        (::std::mem::size_of::<*mut matd_t>() as libc::c_ulong)
            .wrapping_mul(n_points as libc::c_ulong),
    ) as *mut *mut matd_t;
    let mut F_trans: *mut *mut matd_t = malloc(
        (::std::mem::size_of::<*mut matd_t>() as libc::c_ulong)
            .wrapping_mul(n_points as libc::c_ulong),
    ) as *mut *mut matd_t;
    let mut avg_F_trans: *mut matd_t = matd_create(3 as libc::c_int, 3 as libc::c_int);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n_points {
        let ref mut fresh5 = *p_trans.offset(i as isize);
        *fresh5 = matd_op(
            b"M'*M\0" as *const u8 as *const libc::c_char,
            R_z,
            *p.offset(i as isize),
        );
        let ref mut fresh6 = *v_trans.offset(i as isize);
        *fresh6 = matd_op(
            b"M*M\0" as *const u8 as *const libc::c_char,
            R_t,
            *v.offset(i as isize),
        );
        let ref mut fresh7 = *F_trans.offset(i as isize);
        *fresh7 = calculate_F(*v_trans.offset(i as isize));
        matd_add_inplace(avg_F_trans, *F_trans.offset(i as isize));
        i += 1;
    }
    matd_scale_inplace(avg_F_trans, 1.0f64 / n_points as libc::c_double);
    let mut G: *mut matd_t = matd_op(
        b"(M-M)^-1\0" as *const u8 as *const libc::c_char,
        I3,
        avg_F_trans,
    );
    matd_scale_inplace(G, 1.0f64 / n_points as libc::c_double);
    let mut M1: *mut matd_t = matd_create_data(
        3 as libc::c_int,
        3 as libc::c_int,
        [
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            2 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            -(2 as libc::c_int) as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
        ]
            .as_mut_ptr(),
    );
    let mut M2: *mut matd_t = matd_create_data(
        3 as libc::c_int,
        3 as libc::c_int,
        [
            -(1 as libc::c_int) as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            1 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
            -(1 as libc::c_int) as libc::c_double,
        ]
            .as_mut_ptr(),
    );
    let mut b0: *mut matd_t = matd_create(3 as libc::c_int, 1 as libc::c_int);
    let mut b1: *mut matd_t = matd_create(3 as libc::c_int, 1 as libc::c_int);
    let mut b2: *mut matd_t = matd_create(3 as libc::c_int, 1 as libc::c_int);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n_points {
        let mut op_tmp1: *mut matd_t = matd_op(
            b"(M-M)MM\0" as *const u8 as *const libc::c_char,
            *F_trans.offset(i_0 as isize),
            I3,
            R_gamma,
            *p_trans.offset(i_0 as isize),
        );
        let mut op_tmp2: *mut matd_t = matd_op(
            b"(M-M)MMM\0" as *const u8 as *const libc::c_char,
            *F_trans.offset(i_0 as isize),
            I3,
            R_gamma,
            M1,
            *p_trans.offset(i_0 as isize),
        );
        let mut op_tmp3: *mut matd_t = matd_op(
            b"(M-M)MMM\0" as *const u8 as *const libc::c_char,
            *F_trans.offset(i_0 as isize),
            I3,
            R_gamma,
            M2,
            *p_trans.offset(i_0 as isize),
        );
        matd_add_inplace(b0, op_tmp1);
        matd_add_inplace(b1, op_tmp2);
        matd_add_inplace(b2, op_tmp3);
        matd_destroy(op_tmp1);
        matd_destroy(op_tmp2);
        matd_destroy(op_tmp3);
        i_0 += 1;
    }
    let mut b0_: *mut matd_t = matd_multiply(G, b0);
    let mut b1_: *mut matd_t = matd_multiply(G, b1);
    let mut b2_: *mut matd_t = matd_multiply(G, b2);
    let mut a0: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut a1: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut a2: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut a3: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut a4: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < n_points {
        let mut c0: *mut matd_t = matd_op(
            b"(M-M)(MM+M)\0" as *const u8 as *const libc::c_char,
            I3,
            *F_trans.offset(i_1 as isize),
            R_gamma,
            *p_trans.offset(i_1 as isize),
            b0_,
        );
        let mut c1: *mut matd_t = matd_op(
            b"(M-M)(MMM+M)\0" as *const u8 as *const libc::c_char,
            I3,
            *F_trans.offset(i_1 as isize),
            R_gamma,
            M1,
            *p_trans.offset(i_1 as isize),
            b1_,
        );
        let mut c2: *mut matd_t = matd_op(
            b"(M-M)(MMM+M)\0" as *const u8 as *const libc::c_char,
            I3,
            *F_trans.offset(i_1 as isize),
            R_gamma,
            M2,
            *p_trans.offset(i_1 as isize),
            b2_,
        );
        a0
            += matd_to_double(
                matd_op(b"M'M\0" as *const u8 as *const libc::c_char, c0, c0),
            );
        a1
            += matd_to_double(
                matd_op(b"2M'M\0" as *const u8 as *const libc::c_char, c0, c1),
            );
        a2
            += matd_to_double(
                matd_op(
                    b"M'M+2M'M\0" as *const u8 as *const libc::c_char,
                    c1,
                    c1,
                    c0,
                    c2,
                ),
            );
        a3
            += matd_to_double(
                matd_op(b"2M'M\0" as *const u8 as *const libc::c_char, c1, c2),
            );
        a4
            += matd_to_double(
                matd_op(b"M'M\0" as *const u8 as *const libc::c_char, c2, c2),
            );
        matd_destroy(c0);
        matd_destroy(c1);
        matd_destroy(c2);
        i_1 += 1;
    }
    matd_destroy(b0);
    matd_destroy(b1);
    matd_destroy(b2);
    matd_destroy(b0_);
    matd_destroy(b1_);
    matd_destroy(b2_);
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < n_points {
        matd_destroy(*p_trans.offset(i_2 as isize));
        matd_destroy(*v_trans.offset(i_2 as isize));
        matd_destroy(*F_trans.offset(i_2 as isize));
        i_2 += 1;
    }
    free(p_trans as *mut libc::c_void);
    free(v_trans as *mut libc::c_void);
    free(F_trans as *mut libc::c_void);
    matd_destroy(avg_F_trans);
    matd_destroy(G);
    let mut p0: libc::c_double = a1;
    let mut p1: libc::c_double = 2 as libc::c_int as libc::c_double * a2
        - 4 as libc::c_int as libc::c_double * a0;
    let mut p2: libc::c_double = 3 as libc::c_int as libc::c_double * a3
        - 3 as libc::c_int as libc::c_double * a1;
    let mut p3: libc::c_double = 4 as libc::c_int as libc::c_double * a4
        - 2 as libc::c_int as libc::c_double * a2;
    let mut p4: libc::c_double = -a3;
    let mut roots: [libc::c_double; 4] = [0.; 4];
    let mut n_roots: libc::c_int = 0;
    solve_poly_approx(
        [p0, p1, p2, p3, p4].as_mut_ptr(),
        4 as libc::c_int,
        roots.as_mut_ptr(),
        &mut n_roots,
    );
    let mut minima: [libc::c_double; 4] = [0.; 4];
    let mut n_minima: libc::c_int = 0 as libc::c_int;
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < n_roots {
        let mut t1: libc::c_double = roots[i_3 as usize];
        let mut t2: libc::c_double = t1 * t1;
        let mut t3: libc::c_double = t1 * t2;
        let mut t4: libc::c_double = t1 * t3;
        let mut t5: libc::c_double = t1 * t4;
        if a2 - 2 as libc::c_int as libc::c_double * a0
            + (3 as libc::c_int as libc::c_double * a3
                - 6 as libc::c_int as libc::c_double * a1) * t1
            + (6 as libc::c_int as libc::c_double * a4
                - 8 as libc::c_int as libc::c_double * a2
                + 10 as libc::c_int as libc::c_double * a0) * t2
            + (-(8 as libc::c_int) as libc::c_double * a3
                + 6 as libc::c_int as libc::c_double * a1) * t3
            + (-(6 as libc::c_int) as libc::c_double * a4
                + 3 as libc::c_int as libc::c_double * a2) * t4 + a3 * t5
            >= 0 as libc::c_int as libc::c_double
        {
            let mut t_cur: libc::c_double = 2 as libc::c_int as libc::c_double
                * atan(roots[i_3 as usize]);
            if fabs(t_cur - t_initial) > 0.1f64 {
                let fresh8 = n_minima;
                n_minima = n_minima + 1;
                minima[fresh8 as usize] = roots[i_3 as usize];
            }
        }
        i_3 += 1;
    }
    let mut ret: *mut matd_t = 0 as *mut matd_t;
    if n_minima == 1 as libc::c_int {
        let mut t_cur_0: libc::c_double = minima[0 as libc::c_int as usize];
        let mut R_beta: *mut matd_t = matd_copy(M2);
        matd_scale_inplace(R_beta, t_cur_0);
        matd_add_inplace(R_beta, M1);
        matd_scale_inplace(R_beta, t_cur_0);
        matd_add_inplace(R_beta, I3);
        matd_scale_inplace(
            R_beta,
            1 as libc::c_int as libc::c_double
                / (1 as libc::c_int as libc::c_double + t_cur_0 * t_cur_0),
        );
        ret = matd_op(
            b"M'MMM'\0" as *const u8 as *const libc::c_char,
            R_t,
            R_gamma,
            R_beta,
            R_z,
        );
        matd_destroy(R_beta);
    } else if n_minima > 1 as libc::c_int {
        fflush(stderr);
    }
    matd_destroy(I3);
    matd_destroy(M1);
    matd_destroy(M2);
    matd_destroy(R_t);
    matd_destroy(R_gamma);
    matd_destroy(R_z);
    matd_destroy(R_1_prime);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn estimate_pose_for_tag_homography(
    mut info: *mut apriltag_detection_info_t,
    mut solution: *mut apriltag_pose_t,
) {
    let mut scale: libc::c_double = (*info).tagsize / 2.0f64;
    let mut M_H: *mut matd_t = homography_to_pose(
        (*(*info).det).H,
        -(*info).fx,
        (*info).fy,
        (*info).cx,
        (*info).cy,
    );
    *((*M_H).data)
        .as_mut_ptr()
        .offset(
            (0 as libc::c_int as libc::c_uint)
                .wrapping_mul((*M_H).ncols)
                .wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
        ) *= scale;
    *((*M_H).data)
        .as_mut_ptr()
        .offset(
            (1 as libc::c_int as libc::c_uint)
                .wrapping_mul((*M_H).ncols)
                .wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
        ) *= scale;
    *((*M_H).data)
        .as_mut_ptr()
        .offset(
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*M_H).ncols)
                .wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
        ) *= scale;
    let mut fix: *mut matd_t = matd_create(4 as libc::c_int, 4 as libc::c_int);
    *((*fix).data)
        .as_mut_ptr()
        .offset(
            (0 as libc::c_int as libc::c_uint)
                .wrapping_mul((*fix).ncols)
                .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
        ) = 1 as libc::c_int as libc::c_double;
    *((*fix).data)
        .as_mut_ptr()
        .offset(
            (1 as libc::c_int as libc::c_uint)
                .wrapping_mul((*fix).ncols)
                .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        ) = -(1 as libc::c_int) as libc::c_double;
    *((*fix).data)
        .as_mut_ptr()
        .offset(
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*fix).ncols)
                .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
        ) = -(1 as libc::c_int) as libc::c_double;
    *((*fix).data)
        .as_mut_ptr()
        .offset(
            (3 as libc::c_int as libc::c_uint)
                .wrapping_mul((*fix).ncols)
                .wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
        ) = 1 as libc::c_int as libc::c_double;
    let mut initial_pose: *mut matd_t = matd_multiply(fix, M_H);
    matd_destroy(M_H);
    matd_destroy(fix);
    let ref mut fresh9 = (*solution).R;
    *fresh9 = matd_create(3 as libc::c_int, 3 as libc::c_int);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            *((*(*solution).R).data)
                .as_mut_ptr()
                .offset(
                    (i as libc::c_uint)
                        .wrapping_mul((*(*solution).R).ncols)
                        .wrapping_add(j as libc::c_uint) as isize,
                ) = *((*initial_pose).data)
                .as_mut_ptr()
                .offset(
                    (i as libc::c_uint)
                        .wrapping_mul((*initial_pose).ncols)
                        .wrapping_add(j as libc::c_uint) as isize,
                );
            j += 1;
        }
        i += 1;
    }
    let ref mut fresh10 = (*solution).t;
    *fresh10 = matd_create(3 as libc::c_int, 1 as libc::c_int);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 3 as libc::c_int {
        *((*(*solution).t).data)
            .as_mut_ptr()
            .offset(
                (i_0 as libc::c_uint)
                    .wrapping_mul((*(*solution).t).ncols)
                    .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
            ) = *((*initial_pose).data)
            .as_mut_ptr()
            .offset(
                (i_0 as libc::c_uint)
                    .wrapping_mul((*initial_pose).ncols)
                    .wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
            );
        i_0 += 1;
    }
    matd_destroy(initial_pose);
}
#[no_mangle]
pub unsafe extern "C" fn estimate_tag_pose_orthogonal_iteration(
    mut info: *mut apriltag_detection_info_t,
    mut err1: *mut libc::c_double,
    mut solution1: *mut apriltag_pose_t,
    mut err2: *mut libc::c_double,
    mut solution2: *mut apriltag_pose_t,
    mut nIters: libc::c_int,
) {
    let mut scale: libc::c_double = (*info).tagsize / 2.0f64;
    let mut p: [*mut matd_t; 4] = [
        matd_create_data(
            3 as libc::c_int,
            1 as libc::c_int,
            [-scale, scale, 0 as libc::c_int as libc::c_double].as_mut_ptr(),
        ),
        matd_create_data(
            3 as libc::c_int,
            1 as libc::c_int,
            [scale, scale, 0 as libc::c_int as libc::c_double].as_mut_ptr(),
        ),
        matd_create_data(
            3 as libc::c_int,
            1 as libc::c_int,
            [scale, -scale, 0 as libc::c_int as libc::c_double].as_mut_ptr(),
        ),
        matd_create_data(
            3 as libc::c_int,
            1 as libc::c_int,
            [-scale, -scale, 0 as libc::c_int as libc::c_double].as_mut_ptr(),
        ),
    ];
    let mut v: [*mut matd_t; 4] = [0 as *mut matd_t; 4];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        v[i
            as usize] = matd_create_data(
            3 as libc::c_int,
            1 as libc::c_int,
            [
                ((*(*info).det).p[i as usize][0 as libc::c_int as usize] - (*info).cx)
                    / (*info).fx,
                ((*(*info).det).p[i as usize][1 as libc::c_int as usize] - (*info).cy)
                    / (*info).fy,
                1 as libc::c_int as libc::c_double,
            ]
                .as_mut_ptr(),
        );
        i += 1;
    }
    estimate_pose_for_tag_homography(info, solution1);
    *err1 = orthogonal_iteration(
        v.as_mut_ptr(),
        p.as_mut_ptr(),
        &mut (*solution1).t,
        &mut (*solution1).R,
        4 as libc::c_int,
        nIters,
    );
    let ref mut fresh11 = (*solution2).R;
    *fresh11 = fix_pose_ambiguities(
        v.as_mut_ptr(),
        p.as_mut_ptr(),
        (*solution1).t,
        (*solution1).R,
        4 as libc::c_int,
    );
    if !((*solution2).R).is_null() {
        let ref mut fresh12 = (*solution2).t;
        *fresh12 = matd_create(3 as libc::c_int, 1 as libc::c_int);
        *err2 = orthogonal_iteration(
            v.as_mut_ptr(),
            p.as_mut_ptr(),
            &mut (*solution2).t,
            &mut (*solution2).R,
            4 as libc::c_int,
            nIters,
        );
    } else {
        *err2 = ::std::f64::INFINITY;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 4 as libc::c_int {
        matd_destroy(p[i_0 as usize]);
        matd_destroy(v[i_0 as usize]);
        i_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn estimate_tag_pose(
    mut info: *mut apriltag_detection_info_t,
    mut pose: *mut apriltag_pose_t,
) -> libc::c_double {
    let mut err1: libc::c_double = 0.;
    let mut err2: libc::c_double = 0.;
    let mut pose1: apriltag_pose_t = apriltag_pose_t {
        R: 0 as *mut matd_t,
        t: 0 as *mut matd_t,
    };
    let mut pose2: apriltag_pose_t = apriltag_pose_t {
        R: 0 as *mut matd_t,
        t: 0 as *mut matd_t,
    };
    estimate_tag_pose_orthogonal_iteration(
        info,
        &mut err1,
        &mut pose1,
        &mut err2,
        &mut pose2,
        50 as libc::c_int,
    );
    if err1 <= err2 {
        let ref mut fresh13 = (*pose).R;
        *fresh13 = pose1.R;
        let ref mut fresh14 = (*pose).t;
        *fresh14 = pose1.t;
        if !(pose2.R).is_null() {
            matd_destroy(pose2.t);
        }
        matd_destroy(pose2.R);
        return err1;
    } else {
        let ref mut fresh15 = (*pose).R;
        *fresh15 = pose2.R;
        let ref mut fresh16 = (*pose).t;
        *fresh16 = pose2.t;
        matd_destroy(pose1.R);
        matd_destroy(pose1.t);
        return err2;
    };
}
