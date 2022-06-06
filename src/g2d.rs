use ::libc;
extern "C" {
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zarray {
    pub el_sz: size_t,
    pub size: libc::c_int,
    pub alloc: libc::c_int,
    pub data: *mut libc::c_char,
}
pub type zarray_t = zarray;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct g2d_line_t {
    pub p: [libc::c_double; 2],
    pub u: [libc::c_double; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct g2d_line_segment_t {
    pub line: g2d_line_t,
    pub p1: [libc::c_double; 2],
}
#[inline]
unsafe extern "C" fn zarray_create(mut el_sz: size_t) -> *mut zarray_t {
    let mut za: *mut zarray_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<zarray_t>() as libc::c_ulong,
    ) as *mut zarray_t;
    (*za).el_sz = el_sz;
    return za;
}
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
unsafe extern "C" fn zarray_get(
    mut za: *const zarray_t,
    mut idx: libc::c_int,
    mut p: *mut libc::c_void,
) {
    memcpy(
        p,
        &mut *((*za).data)
            .offset((idx as libc::c_ulong).wrapping_mul((*za).el_sz) as isize)
            as *mut libc::c_char as *const libc::c_void,
        (*za).el_sz,
    );
}
#[inline]
unsafe extern "C" fn zarray_add(mut za: *mut zarray_t, mut p: *const libc::c_void) {
    zarray_ensure_capacity(za, (*za).size + 1 as libc::c_int);
    memcpy(
        &mut *((*za).data)
            .offset(((*za).size as libc::c_ulong).wrapping_mul((*za).el_sz) as isize)
            as *mut libc::c_char as *mut libc::c_void,
        p,
        (*za).el_sz,
    );
    let ref mut fresh1 = (*za).size;
    *fresh1 += 1;
}
#[inline]
unsafe extern "C" fn zarray_size(mut za: *const zarray_t) -> libc::c_int {
    return (*za).size;
}
#[inline]
unsafe extern "C" fn zarray_ensure_capacity(
    mut za: *mut zarray_t,
    mut capacity: libc::c_int,
) {
    if capacity <= (*za).alloc {
        return;
    }
    while (*za).alloc < capacity {
        (*za).alloc *= 2 as libc::c_int;
        if (*za).alloc < 8 as libc::c_int {
            (*za).alloc = 8 as libc::c_int;
        }
    }
    let ref mut fresh2 = (*za).data;
    *fresh2 = realloc(
        (*za).data as *mut libc::c_void,
        ((*za).alloc as libc::c_ulong).wrapping_mul((*za).el_sz),
    ) as *mut libc::c_char;
}
#[inline]
unsafe extern "C" fn zarray_set(
    mut za: *mut zarray_t,
    mut idx: libc::c_int,
    mut p: *const libc::c_void,
    mut outp: *mut libc::c_void,
) {
    if !outp.is_null() {
        memcpy(
            outp,
            &mut *((*za).data)
                .offset((idx as libc::c_ulong).wrapping_mul((*za).el_sz) as isize)
                as *mut libc::c_char as *const libc::c_void,
            (*za).el_sz,
        );
    }
    memcpy(
        &mut *((*za).data)
            .offset((idx as libc::c_ulong).wrapping_mul((*za).el_sz) as isize)
            as *mut libc::c_char as *mut libc::c_void,
        p,
        (*za).el_sz,
    );
}
#[inline]
unsafe extern "C" fn sq(mut v: libc::c_double) -> libc::c_double {
    return v * v;
}
#[inline]
unsafe extern "C" fn mod2pi_positive(mut vin: libc::c_double) -> libc::c_double {
    return vin - 6.2831853071795862319959f64 * floor(vin / 6.2831853071795862319959f64);
}
#[inline]
unsafe extern "C" fn mod2pi(mut vin: libc::c_double) -> libc::c_double {
    return mod2pi_positive(vin + 3.14159265358979323846f64) - 3.14159265358979323846f64;
}
#[inline]
unsafe extern "C" fn dclamp(
    mut a: libc::c_double,
    mut min: libc::c_double,
    mut max: libc::c_double,
) -> libc::c_double {
    if a < min {
        return min;
    }
    if a > max {
        return max;
    }
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn g2d_distance(
    mut a: *const libc::c_double,
    mut b: *const libc::c_double,
) -> libc::c_double {
    return sqrtf(
        (sq(*a.offset(0 as libc::c_int as isize) - *b.offset(0 as libc::c_int as isize))
            + sq(
                *a.offset(1 as libc::c_int as isize)
                    - *b.offset(1 as libc::c_int as isize),
            )) as libc::c_float,
    ) as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn g2d_polygon_create_empty() -> *mut zarray_t {
    return zarray_create(::std::mem::size_of::<[libc::c_double; 2]>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn g2d_polygon_add(
    mut poly: *mut zarray_t,
    mut v: *mut libc::c_double,
) {
    zarray_add(poly, v as *const libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn g2d_polygon_create_data(
    mut v: *mut [libc::c_double; 2],
    mut sz: libc::c_int,
) -> *mut zarray_t {
    let mut points: *mut zarray_t = g2d_polygon_create_empty();
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < sz {
        g2d_polygon_add(points, (*v.offset(i as isize)).as_mut_ptr());
        i += 1;
    }
    return points;
}
#[no_mangle]
pub unsafe extern "C" fn g2d_polygon_create_zeros(mut sz: libc::c_int) -> *mut zarray_t {
    let mut points: *mut zarray_t = zarray_create(
        ::std::mem::size_of::<[libc::c_double; 2]>() as libc::c_ulong,
    );
    let mut z: [libc::c_double; 2] = [
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    ];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < sz {
        zarray_add(points, z.as_mut_ptr() as *const libc::c_void);
        i += 1;
    }
    return points;
}
#[no_mangle]
pub unsafe extern "C" fn g2d_polygon_make_ccw(mut poly: *mut zarray_t) {
    let mut total_theta: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut last_theta: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sz: libc::c_int = zarray_size(poly);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i <= sz {
        let mut p0: [libc::c_double; 2] = [0.; 2];
        let mut p1: [libc::c_double; 2] = [0.; 2];
        zarray_get(
            poly,
            i % sz,
            &mut p0 as *mut [libc::c_double; 2] as *mut libc::c_void,
        );
        zarray_get(
            poly,
            (i + 1 as libc::c_int) % sz,
            &mut p1 as *mut [libc::c_double; 2] as *mut libc::c_void,
        );
        let mut this_theta: libc::c_double = atan2(
            p1[1 as libc::c_int as usize] - p0[1 as libc::c_int as usize],
            p1[0 as libc::c_int as usize] - p0[0 as libc::c_int as usize],
        );
        if i > 0 as libc::c_int {
            let mut dtheta: libc::c_double = mod2pi(this_theta - last_theta);
            total_theta += dtheta;
        }
        last_theta = this_theta;
        i += 1;
    }
    let mut ccw: libc::c_int = (total_theta > 0 as libc::c_int as libc::c_double)
        as libc::c_int;
    if ccw == 0 {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < sz / 2 as libc::c_int {
            let mut a: [libc::c_double; 2] = [0.; 2];
            let mut b: [libc::c_double; 2] = [0.; 2];
            zarray_get(poly, i_0, a.as_mut_ptr() as *mut libc::c_void);
            zarray_get(
                poly,
                sz - 1 as libc::c_int - i_0,
                b.as_mut_ptr() as *mut libc::c_void,
            );
            zarray_set(
                poly,
                i_0,
                b.as_mut_ptr() as *const libc::c_void,
                0 as *mut libc::c_void,
            );
            zarray_set(
                poly,
                sz - 1 as libc::c_int - i_0,
                a.as_mut_ptr() as *const libc::c_void,
                0 as *mut libc::c_void,
            );
            i_0 += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn g2d_polygon_contains_point_ref(
    mut poly: *const zarray_t,
    mut q: *mut libc::c_double,
) -> libc::c_int {
    let mut psz: libc::c_int = zarray_size(poly);
    let mut acc_theta: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut last_theta: libc::c_double = 0.;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i <= psz {
        let mut p: [libc::c_double; 2] = [0.; 2];
        zarray_get(
            poly,
            i % psz,
            &mut p as *mut [libc::c_double; 2] as *mut libc::c_void,
        );
        let mut this_theta: libc::c_double = atan2(
            *q.offset(1 as libc::c_int as isize) - p[1 as libc::c_int as usize],
            *q.offset(0 as libc::c_int as isize) - p[0 as libc::c_int as usize],
        );
        if i != 0 as libc::c_int {
            acc_theta += mod2pi(this_theta - last_theta);
        }
        last_theta = this_theta;
        i += 1;
    }
    return (acc_theta > 3.14159265358979323846f64) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g2d_convex_hull(mut points: *const zarray_t) -> *mut zarray_t {
    let mut hull: *mut zarray_t = zarray_create(
        ::std::mem::size_of::<[libc::c_double; 2]>() as libc::c_ulong,
    );
    let mut insz: libc::c_int = zarray_size(points);
    let mut pleft: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < insz {
        let mut p: *mut libc::c_double = 0 as *mut libc::c_double;
        zarray_get_volatile(
            points,
            i,
            &mut p as *mut *mut libc::c_double as *mut libc::c_void,
        );
        if pleft.is_null()
            || *p.offset(0 as libc::c_int as isize)
                < *pleft.offset(0 as libc::c_int as isize)
        {
            pleft = p;
        }
        i += 1;
    }
    zarray_add(hull, pleft as *const libc::c_void);
    let mut p_0: *mut libc::c_double = pleft;
    loop {
        let mut q: *mut libc::c_double = 0 as *mut libc::c_double;
        let mut n0: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut n1: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < insz {
            let mut thisq: *mut libc::c_double = 0 as *mut libc::c_double;
            zarray_get_volatile(
                points,
                i_0,
                &mut thisq as *mut *mut libc::c_double as *mut libc::c_void,
            );
            if !(thisq == p_0) {
                if q.is_null() {
                    q = thisq;
                    n0 = *q.offset(1 as libc::c_int as isize)
                        - *p_0.offset(1 as libc::c_int as isize);
                    n1 = -*q.offset(0 as libc::c_int as isize)
                        + *p_0.offset(0 as libc::c_int as isize);
                } else {
                    let mut e0: libc::c_double = *thisq.offset(0 as libc::c_int as isize)
                        - *p_0.offset(0 as libc::c_int as isize);
                    let mut e1: libc::c_double = *thisq.offset(1 as libc::c_int as isize)
                        - *p_0.offset(1 as libc::c_int as isize);
                    let mut dot: libc::c_double = e0 * n0 + e1 * n1;
                    if dot > 0 as libc::c_int as libc::c_double {
                        q = thisq;
                        n0 = *q.offset(1 as libc::c_int as isize)
                            - *p_0.offset(1 as libc::c_int as isize);
                        n1 = -*q.offset(0 as libc::c_int as isize)
                            + *p_0.offset(0 as libc::c_int as isize);
                    }
                }
            }
            i_0 += 1;
        }
        if q == pleft {
            break;
        }
        let mut colinear: libc::c_int = 0 as libc::c_int;
        if zarray_size(hull) > 1 as libc::c_int {
            let mut o: *mut libc::c_double = 0 as *mut libc::c_double;
            zarray_get_volatile(
                hull,
                zarray_size(hull) - 2 as libc::c_int,
                &mut o as *mut *mut libc::c_double as *mut libc::c_void,
            );
            let mut e0_0: libc::c_double = *o.offset(0 as libc::c_int as isize)
                - *p_0.offset(0 as libc::c_int as isize);
            let mut e1_0: libc::c_double = *o.offset(1 as libc::c_int as isize)
                - *p_0.offset(1 as libc::c_int as isize);
            if n0 * e0_0 + n1 * e1_0 == 0 as libc::c_int as libc::c_double {
                colinear = 1 as libc::c_int;
            }
        }
        if colinear != 0 {
            zarray_set(
                hull,
                zarray_size(hull) - 1 as libc::c_int,
                q as *const libc::c_void,
                0 as *mut libc::c_void,
            );
        } else {
            zarray_add(hull, q as *const libc::c_void);
        }
        p_0 = q;
    }
    return hull;
}
#[no_mangle]
pub unsafe extern "C" fn g2d_polygon_closest_boundary_point(
    mut poly: *const zarray_t,
    mut q: *const libc::c_double,
    mut p: *mut libc::c_double,
) {
    let mut psz: libc::c_int = zarray_size(poly);
    let mut min_dist: libc::c_double = ::std::f32::INFINITY as libc::c_double;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < psz {
        let mut p0: *mut libc::c_double = 0 as *mut libc::c_double;
        let mut p1: *mut libc::c_double = 0 as *mut libc::c_double;
        zarray_get_volatile(
            poly,
            i,
            &mut p0 as *mut *mut libc::c_double as *mut libc::c_void,
        );
        zarray_get_volatile(
            poly,
            (i + 1 as libc::c_int) % psz,
            &mut p1 as *mut *mut libc::c_double as *mut libc::c_void,
        );
        let mut seg: g2d_line_segment_t = g2d_line_segment_t {
            line: g2d_line_t {
                p: [0.; 2],
                u: [0.; 2],
            },
            p1: [0.; 2],
        };
        g2d_line_segment_init_from_points(
            &mut seg,
            p0 as *const libc::c_double,
            p1 as *const libc::c_double,
        );
        let mut thisp: [libc::c_double; 2] = [0.; 2];
        g2d_line_segment_closest_point(&mut seg, q, thisp.as_mut_ptr());
        let mut dist: libc::c_double = g2d_distance(
            q,
            thisp.as_mut_ptr() as *const libc::c_double,
        );
        if dist < min_dist {
            memcpy(
                p as *mut libc::c_void,
                thisp.as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<[libc::c_double; 2]>() as libc::c_ulong,
            );
            min_dist = dist;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn g2d_polygon_contains_point(
    mut poly: *const zarray_t,
    mut q: *mut libc::c_double,
) -> libc::c_int {
    let mut psz: libc::c_int = zarray_size(poly);
    let mut last_quadrant: libc::c_int = 0;
    let mut quad_acc: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i <= psz {
        let mut p: *mut libc::c_double = 0 as *mut libc::c_double;
        zarray_get_volatile(
            poly,
            i % psz,
            &mut p as *mut *mut libc::c_double as *mut libc::c_void,
        );
        let mut quadrant: libc::c_int = 0;
        if *p.offset(0 as libc::c_int as isize) < *q.offset(0 as libc::c_int as isize) {
            quadrant = if *p.offset(1 as libc::c_int as isize)
                < *q.offset(1 as libc::c_int as isize)
            {
                2 as libc::c_int
            } else {
                1 as libc::c_int
            };
        } else {
            quadrant = if *p.offset(1 as libc::c_int as isize)
                < *q.offset(1 as libc::c_int as isize)
            {
                3 as libc::c_int
            } else {
                0 as libc::c_int
            };
        }
        if i > 0 as libc::c_int {
            let mut dquadrant: libc::c_int = quadrant - last_quadrant;
            match dquadrant {
                -3 | 1 => {
                    quad_acc += 1;
                }
                -1 | 3 => {
                    quad_acc -= 1;
                }
                -2 | 2 => {
                    let mut p0: *mut libc::c_double = 0 as *mut libc::c_double;
                    zarray_get_volatile(
                        poly,
                        i - 1 as libc::c_int,
                        &mut p0 as *mut *mut libc::c_double as *mut libc::c_void,
                    );
                    let mut nx: libc::c_double = *p.offset(1 as libc::c_int as isize)
                        - *q.offset(1 as libc::c_int as isize);
                    let mut ny: libc::c_double = -*p.offset(0 as libc::c_int as isize)
                        + *q.offset(0 as libc::c_int as isize);
                    let mut dot: libc::c_double = nx
                        * (*p0.offset(0 as libc::c_int as isize)
                            - *q.offset(0 as libc::c_int as isize))
                        + ny
                            * (*p0.offset(1 as libc::c_int as isize)
                                - *q.offset(1 as libc::c_int as isize));
                    if dot < 0 as libc::c_int as libc::c_double {
                        quad_acc -= 2 as libc::c_int;
                    } else {
                        quad_acc += 2 as libc::c_int;
                    }
                }
                0 | _ => {}
            }
        }
        last_quadrant = quadrant;
        i += 1;
    }
    let mut v: libc::c_int = (quad_acc >= 2 as libc::c_int
        || quad_acc <= -(2 as libc::c_int)) as libc::c_int;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn g2d_line_init_from_points(
    mut line: *mut g2d_line_t,
    mut p0: *const libc::c_double,
    mut p1: *const libc::c_double,
) {
    (*line).p[0 as libc::c_int as usize] = *p0.offset(0 as libc::c_int as isize);
    (*line).p[1 as libc::c_int as usize] = *p0.offset(1 as libc::c_int as isize);
    (*line)
        .u[0 as libc::c_int
        as usize] = *p1.offset(0 as libc::c_int as isize)
        - *p0.offset(0 as libc::c_int as isize);
    (*line)
        .u[1 as libc::c_int
        as usize] = *p1.offset(1 as libc::c_int as isize)
        - *p0.offset(1 as libc::c_int as isize);
    let mut mag: libc::c_double = sqrtf(
        (sq((*line).u[0 as libc::c_int as usize])
            + sq((*line).u[1 as libc::c_int as usize])) as libc::c_float,
    ) as libc::c_double;
    (*line).u[0 as libc::c_int as usize] /= mag;
    (*line).u[1 as libc::c_int as usize] /= mag;
}
#[no_mangle]
pub unsafe extern "C" fn g2d_line_get_coordinate(
    mut line: *const g2d_line_t,
    mut q: *const libc::c_double,
) -> libc::c_double {
    return (*q.offset(0 as libc::c_int as isize) - (*line).p[0 as libc::c_int as usize])
        * (*line).u[0 as libc::c_int as usize]
        + (*q.offset(1 as libc::c_int as isize) - (*line).p[1 as libc::c_int as usize])
            * (*line).u[1 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn g2d_line_intersect_line(
    mut linea: *const g2d_line_t,
    mut lineb: *const g2d_line_t,
    mut p: *mut libc::c_double,
) -> libc::c_int {
    let mut m00: libc::c_double = 0.;
    let mut m01: libc::c_double = 0.;
    let mut m10: libc::c_double = 0.;
    let mut m11: libc::c_double = 0.;
    let mut i00: libc::c_double = 0.;
    let mut i01: libc::c_double = 0.;
    let mut b00: libc::c_double = 0.;
    let mut b10: libc::c_double = 0.;
    m00 = (*linea).u[0 as libc::c_int as usize];
    m01 = -(*lineb).u[0 as libc::c_int as usize];
    m10 = (*linea).u[1 as libc::c_int as usize];
    m11 = -(*lineb).u[1 as libc::c_int as usize];
    let mut det: libc::c_double = m00 * m11 - m01 * m10;
    if fabs(det) < 0.00000001f64 {
        return 0 as libc::c_int;
    }
    i00 = m11 / det;
    i01 = -m01 / det;
    b00 = (*lineb).p[0 as libc::c_int as usize] - (*linea).p[0 as libc::c_int as usize];
    b10 = (*lineb).p[1 as libc::c_int as usize] - (*linea).p[1 as libc::c_int as usize];
    let mut x00: libc::c_double = 0.;
    x00 = i00 * b00 + i01 * b10;
    if !p.is_null() {
        *p
            .offset(
                0 as libc::c_int as isize,
            ) = (*linea).u[0 as libc::c_int as usize] * x00
            + (*linea).p[0 as libc::c_int as usize];
        *p
            .offset(
                1 as libc::c_int as isize,
            ) = (*linea).u[1 as libc::c_int as usize] * x00
            + (*linea).p[1 as libc::c_int as usize];
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g2d_line_segment_init_from_points(
    mut seg: *mut g2d_line_segment_t,
    mut p0: *const libc::c_double,
    mut p1: *const libc::c_double,
) {
    g2d_line_init_from_points(&mut (*seg).line, p0, p1);
    (*seg).p1[0 as libc::c_int as usize] = *p1.offset(0 as libc::c_int as isize);
    (*seg).p1[1 as libc::c_int as usize] = *p1.offset(1 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn g2d_line_segment_closest_point(
    mut seg: *const g2d_line_segment_t,
    mut q: *const libc::c_double,
    mut p: *mut libc::c_double,
) {
    let mut a: libc::c_double = g2d_line_get_coordinate(
        &(*seg).line,
        ((*seg).line.p).as_ptr(),
    );
    let mut b: libc::c_double = g2d_line_get_coordinate(
        &(*seg).line,
        ((*seg).p1).as_ptr(),
    );
    let mut c: libc::c_double = g2d_line_get_coordinate(&(*seg).line, q);
    if a < b {
        c = dclamp(c, a, b);
    } else {
        c = dclamp(c, b, a);
    }
    *p
        .offset(
            0 as libc::c_int as isize,
        ) = (*seg).line.p[0 as libc::c_int as usize]
        + c * (*seg).line.u[0 as libc::c_int as usize];
    *p
        .offset(
            1 as libc::c_int as isize,
        ) = (*seg).line.p[1 as libc::c_int as usize]
        + c * (*seg).line.u[1 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn g2d_line_segment_intersect_segment(
    mut sega: *const g2d_line_segment_t,
    mut segb: *const g2d_line_segment_t,
    mut p: *mut libc::c_double,
) -> libc::c_int {
    let mut tmp: [libc::c_double; 2] = [0.; 2];
    if g2d_line_intersect_line(&(*sega).line, &(*segb).line, tmp.as_mut_ptr()) == 0 {
        return 0 as libc::c_int;
    }
    let mut a: libc::c_double = g2d_line_get_coordinate(
        &(*sega).line,
        ((*sega).line.p).as_ptr(),
    );
    let mut b: libc::c_double = g2d_line_get_coordinate(
        &(*sega).line,
        ((*sega).p1).as_ptr(),
    );
    let mut c: libc::c_double = g2d_line_get_coordinate(
        &(*sega).line,
        tmp.as_mut_ptr() as *const libc::c_double,
    );
    if c < a && c < b || c > a && c > b {
        return 0 as libc::c_int;
    }
    a = g2d_line_get_coordinate(&(*segb).line, ((*segb).line.p).as_ptr());
    b = g2d_line_get_coordinate(&(*segb).line, ((*segb).p1).as_ptr());
    c = g2d_line_get_coordinate(
        &(*segb).line,
        tmp.as_mut_ptr() as *const libc::c_double,
    );
    if c < a && c < b || c > a && c > b {
        return 0 as libc::c_int;
    }
    if !p.is_null() {
        *p.offset(0 as libc::c_int as isize) = tmp[0 as libc::c_int as usize];
        *p.offset(1 as libc::c_int as isize) = tmp[1 as libc::c_int as usize];
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g2d_line_segment_intersect_line(
    mut seg: *const g2d_line_segment_t,
    mut line: *const g2d_line_t,
    mut p: *mut libc::c_double,
) -> libc::c_int {
    let mut tmp: [libc::c_double; 2] = [0.; 2];
    if g2d_line_intersect_line(&(*seg).line, line, tmp.as_mut_ptr()) == 0 {
        return 0 as libc::c_int;
    }
    let mut a: libc::c_double = g2d_line_get_coordinate(
        &(*seg).line,
        ((*seg).line.p).as_ptr(),
    );
    let mut b: libc::c_double = g2d_line_get_coordinate(
        &(*seg).line,
        ((*seg).p1).as_ptr(),
    );
    let mut c: libc::c_double = g2d_line_get_coordinate(
        &(*seg).line,
        tmp.as_mut_ptr() as *const libc::c_double,
    );
    if c < a && c < b || c > a && c > b {
        return 0 as libc::c_int;
    }
    if !p.is_null() {
        *p.offset(0 as libc::c_int as isize) = tmp[0 as libc::c_int as usize];
        *p.offset(1 as libc::c_int as isize) = tmp[1 as libc::c_int as usize];
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g2d_polygon_intersects_polygon(
    mut polya: *const zarray_t,
    mut polyb: *const zarray_t,
) -> libc::c_int {
    let mut ia: libc::c_int = 0 as libc::c_int;
    while ia < zarray_size(polya) {
        let mut pa0: [libc::c_double; 2] = [0.; 2];
        let mut pa1: [libc::c_double; 2] = [0.; 2];
        zarray_get(polya, ia, pa0.as_mut_ptr() as *mut libc::c_void);
        zarray_get(
            polya,
            (ia + 1 as libc::c_int) % zarray_size(polya),
            pa1.as_mut_ptr() as *mut libc::c_void,
        );
        let mut sega: g2d_line_segment_t = g2d_line_segment_t {
            line: g2d_line_t {
                p: [0.; 2],
                u: [0.; 2],
            },
            p1: [0.; 2],
        };
        g2d_line_segment_init_from_points(
            &mut sega,
            pa0.as_mut_ptr() as *const libc::c_double,
            pa1.as_mut_ptr() as *const libc::c_double,
        );
        let mut ib: libc::c_int = 0 as libc::c_int;
        while ib < zarray_size(polyb) {
            let mut pb0: [libc::c_double; 2] = [0.; 2];
            let mut pb1: [libc::c_double; 2] = [0.; 2];
            zarray_get(polyb, ib, pb0.as_mut_ptr() as *mut libc::c_void);
            zarray_get(
                polyb,
                (ib + 1 as libc::c_int) % zarray_size(polyb),
                pb1.as_mut_ptr() as *mut libc::c_void,
            );
            let mut segb: g2d_line_segment_t = g2d_line_segment_t {
                line: g2d_line_t {
                    p: [0.; 2],
                    u: [0.; 2],
                },
                p1: [0.; 2],
            };
            g2d_line_segment_init_from_points(
                &mut segb,
                pb0.as_mut_ptr() as *const libc::c_double,
                pb1.as_mut_ptr() as *const libc::c_double,
            );
            if g2d_line_segment_intersect_segment(
                &mut sega,
                &mut segb,
                0 as *mut libc::c_double,
            ) != 0
            {
                return 1 as libc::c_int;
            }
            ib += 1;
        }
        ia += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g2d_polygon_contains_polygon(
    mut polya: *const zarray_t,
    mut polyb: *const zarray_t,
) -> libc::c_int {
    if g2d_polygon_intersects_polygon(polya, polyb) != 0 {
        return 0 as libc::c_int;
    }
    let mut p: [libc::c_double; 2] = [0.; 2];
    zarray_get(polyb, 0 as libc::c_int, p.as_mut_ptr() as *mut libc::c_void);
    return g2d_polygon_contains_point(polya, p.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn g2d_polygon_get_interior_point(
    mut poly: *const zarray_t,
    mut p: *mut libc::c_double,
) {
    let mut a: [libc::c_double; 2] = [0.; 2];
    let mut b: [libc::c_double; 2] = [0.; 2];
    let mut c: [libc::c_double; 2] = [0.; 2];
    zarray_get(poly, 0 as libc::c_int, a.as_mut_ptr() as *mut libc::c_void);
    zarray_get(poly, 1 as libc::c_int, b.as_mut_ptr() as *mut libc::c_void);
    zarray_get(poly, 2 as libc::c_int, c.as_mut_ptr() as *mut libc::c_void);
    *p
        .offset(
            0 as libc::c_int as isize,
        ) = (a[0 as libc::c_int as usize] + b[0 as libc::c_int as usize]
        + c[0 as libc::c_int as usize]) / 3 as libc::c_int as libc::c_double;
    *p
        .offset(
            1 as libc::c_int as isize,
        ) = (a[1 as libc::c_int as usize] + b[1 as libc::c_int as usize]
        + c[1 as libc::c_int as usize]) / 3 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn g2d_polygon_overlaps_polygon(
    mut polya: *const zarray_t,
    mut polyb: *const zarray_t,
) -> libc::c_int {
    if g2d_polygon_intersects_polygon(polya, polyb) != 0 {
        return 1 as libc::c_int;
    }
    let mut p: [libc::c_double; 2] = [0.; 2];
    g2d_polygon_get_interior_point(polyb, p.as_mut_ptr());
    if g2d_polygon_contains_point(polya, p.as_mut_ptr()) != 0 {
        return 1 as libc::c_int;
    }
    g2d_polygon_get_interior_point(polya, p.as_mut_ptr());
    if g2d_polygon_contains_point(polyb, p.as_mut_ptr()) != 0 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn double_sort_up(
    mut _a: *const libc::c_void,
    mut _b: *const libc::c_void,
) -> libc::c_int {
    let mut a: libc::c_double = *(_a as *mut libc::c_double);
    let mut b: libc::c_double = *(_b as *mut libc::c_double);
    if a < b {
        return -(1 as libc::c_int);
    }
    if a == b {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g2d_polygon_rasterize(
    mut poly: *const zarray_t,
    mut y: libc::c_double,
    mut x: *mut libc::c_double,
) -> libc::c_int {
    let mut sz: libc::c_int = zarray_size(poly);
    let mut line: g2d_line_t = g2d_line_t {
        p: [0.; 2],
        u: [0.; 2],
    };
    let mut p0: [libc::c_double; 2] = [0 as libc::c_int as libc::c_double, y];
    let mut p1: [libc::c_double; 2] = [1 as libc::c_int as libc::c_double, y];
    g2d_line_init_from_points(
        &mut line,
        p0.as_mut_ptr() as *const libc::c_double,
        p1.as_mut_ptr() as *const libc::c_double,
    );
    let mut xpos: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < sz {
        let mut seg: g2d_line_segment_t = g2d_line_segment_t {
            line: g2d_line_t {
                p: [0.; 2],
                u: [0.; 2],
            },
            p1: [0.; 2],
        };
        let mut p0_0: *mut libc::c_double = 0 as *mut libc::c_double;
        let mut p1_0: *mut libc::c_double = 0 as *mut libc::c_double;
        zarray_get_volatile(
            poly,
            i,
            &mut p0_0 as *mut *mut libc::c_double as *mut libc::c_void,
        );
        zarray_get_volatile(
            poly,
            (i + 1 as libc::c_int) % sz,
            &mut p1_0 as *mut *mut libc::c_double as *mut libc::c_void,
        );
        g2d_line_segment_init_from_points(
            &mut seg,
            p0_0 as *const libc::c_double,
            p1_0 as *const libc::c_double,
        );
        let mut q: [libc::c_double; 2] = [0.; 2];
        if g2d_line_segment_intersect_line(&mut seg, &mut line, q.as_mut_ptr()) != 0 {
            let fresh3 = xpos;
            xpos = xpos + 1;
            *x.offset(fresh3 as isize) = q[0 as libc::c_int as usize];
        }
        i += 1;
    }
    qsort(
        x as *mut libc::c_void,
        xpos as size_t,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        Some(
            double_sort_up
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    return xpos;
}
