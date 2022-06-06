use ::libc;
extern "C" {
    pub type workerpool;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn random() -> libc::c_long;
    fn srandom(__seed: libc::c_uint);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn matd_create(rows: libc::c_int, cols: libc::c_int) -> *mut matd_t;
    fn matd_create_data(
        rows: libc::c_int,
        cols: libc::c_int,
        data: *const libc::c_double,
    ) -> *mut matd_t;
    fn matd_copy(m: *const matd_t) -> *mut matd_t;
    fn matd_inverse(a: *const matd_t) -> *mut matd_t;
    fn matd_op(expr: *const libc::c_char, _: ...) -> *mut matd_t;
    fn matd_destroy(m: *mut matd_t);
    fn image_u8_create(width: libc::c_uint, height: libc::c_uint) -> *mut image_u8_t;
    fn image_u8_copy(in_0: *const image_u8_t) -> *mut image_u8_t;
    fn image_u8_draw_line(
        im: *mut image_u8_t,
        x0: libc::c_float,
        y0: libc::c_float,
        x1: libc::c_float,
        y1: libc::c_float,
        v: libc::c_int,
        width: libc::c_int,
    );
    fn image_u8_darken(im: *mut image_u8_t);
    fn image_u8_gaussian_blur(
        im: *mut image_u8_t,
        sigma: libc::c_double,
        k: libc::c_int,
    );
    fn image_u8_decimate(im: *mut image_u8_t, factor: libc::c_float) -> *mut image_u8_t;
    fn image_u8_destroy(im: *mut image_u8_t);
    fn image_u8_write_pnm(
        im: *const image_u8_t,
        path: *const libc::c_char,
    ) -> libc::c_int;
    fn workerpool_create(nthreads: libc::c_int) -> *mut workerpool_t;
    fn workerpool_destroy(wp: *mut workerpool_t);
    fn workerpool_add_task(
        wp: *mut workerpool_t,
        f: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        p: *mut libc::c_void,
    );
    fn workerpool_run(wp: *mut workerpool_t);
    fn workerpool_get_nthreads(wp: *mut workerpool_t) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn utime_now() -> int64_t;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn atan2f(_: libc::c_float, _: libc::c_float) -> libc::c_float;
    fn cosf(_: libc::c_float) -> libc::c_float;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sinf(_: libc::c_float) -> libc::c_float;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabsf(_: libc::c_float) -> libc::c_float;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fmin(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn __errno_location() -> *mut libc::c_int;
    fn image_u8x3_create(width: libc::c_uint, height: libc::c_uint) -> *mut image_u8x3_t;
    fn image_u8x3_destroy(im: *mut image_u8x3_t);
    fn image_u8x3_write_pnm(
        im: *const image_u8x3_t,
        path: *const libc::c_char,
    ) -> libc::c_int;
    fn image_u8x3_draw_line(
        im: *mut image_u8x3_t,
        x0: libc::c_float,
        y0: libc::c_float,
        x1: libc::c_float,
        y1: libc::c_float,
        rgb: *mut uint8_t,
        width: libc::c_int,
    );
    fn g2d_polygon_create_zeros(sz: libc::c_int) -> *mut zarray_t;
    fn g2d_polygon_overlaps_polygon(
        polya: *const zarray_t,
        polyb: *const zarray_t,
    ) -> libc::c_int;
    fn apriltag_quad_thresh(
        td: *mut apriltag_detector_t,
        im: *mut image_u8_t,
    ) -> *mut zarray_t;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct matd_t {
    pub nrows: libc::c_uint,
    pub ncols: libc::c_uint,
    pub data: [libc::c_double; 0],
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub struct zarray {
    pub el_sz: size_t,
    pub size: libc::c_int,
    pub alloc: libc::c_int,
    pub data: *mut libc::c_char,
}
pub type zarray_t = zarray;
pub type workerpool_t = workerpool;
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
pub struct timeprofile_entry {
    pub name: [libc::c_char; 32],
    pub utime: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeprofile {
    pub utime: int64_t,
    pub stamps: *mut zarray_t,
}
pub type timeprofile_t = timeprofile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quad {
    pub p: [[libc::c_float; 2]; 4],
    pub reversed_border: bool,
    pub H: *mut matd_t,
    pub Hinv: *mut matd_t,
}
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
pub struct apriltag_quad_thresh_params {
    pub min_cluster_pixels: libc::c_int,
    pub max_nmaxima: libc::c_int,
    pub critical_rad: libc::c_float,
    pub cos_critical_rad: libc::c_float,
    pub max_line_fit_mse: libc::c_float,
    pub min_white_black_diff: libc::c_int,
    pub deglitch: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct apriltag_detector {
    pub nthreads: libc::c_int,
    pub quad_decimate: libc::c_float,
    pub quad_sigma: libc::c_float,
    pub refine_edges: libc::c_int,
    pub decode_sharpening: libc::c_double,
    pub debug: libc::c_int,
    pub qtp: apriltag_quad_thresh_params,
    pub tp: *mut timeprofile_t,
    pub nedges: uint32_t,
    pub nsegments: uint32_t,
    pub nquads: uint32_t,
    pub tag_families: *mut zarray_t,
    pub wp: *mut workerpool_t,
    pub mutex: pthread_mutex_t,
}
pub type apriltag_detector_t = apriltag_detector;
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
pub struct quick_decode {
    pub nentries: libc::c_int,
    pub entries: *mut quick_decode_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quick_decode_entry {
    pub rcode: uint64_t,
    pub id: uint16_t,
    pub hamming: uint8_t,
    pub rotation: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quad_decode_task {
    pub i0: libc::c_int,
    pub i1: libc::c_int,
    pub quads: *mut zarray_t,
    pub td: *mut apriltag_detector_t,
    pub im: *mut image_u8_t,
    pub detections: *mut zarray_t,
    pub im_samples: *mut image_u8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct graymodel {
    pub A: [[libc::c_double; 3]; 3],
    pub B: [libc::c_double; 3],
    pub C: [libc::c_double; 3],
}
#[inline]
unsafe extern "C" fn timeprofile_create() -> *mut timeprofile_t {
    let mut tp: *mut timeprofile_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<timeprofile_t>() as libc::c_ulong,
    ) as *mut timeprofile_t;
    let ref mut fresh0 = (*tp).stamps;
    *fresh0 = zarray_create(::std::mem::size_of::<timeprofile_entry>() as libc::c_ulong);
    (*tp).utime = utime_now();
    return tp;
}
#[inline]
unsafe extern "C" fn timeprofile_stamp(
    mut tp: *mut timeprofile_t,
    mut name: *const libc::c_char,
) {
    let mut tpe: timeprofile_entry = timeprofile_entry {
        name: [0; 32],
        utime: 0,
    };
    strncpy(
        (tpe.name).as_mut_ptr(),
        name,
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
    tpe
        .name[(::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    tpe.utime = utime_now();
    zarray_add((*tp).stamps, &mut tpe as *mut timeprofile_entry as *const libc::c_void);
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
unsafe extern "C" fn zarray_destroy(mut za: *mut zarray_t) {
    if za.is_null() {
        return;
    }
    if !((*za).data).is_null() {
        free((*za).data as *mut libc::c_void);
    }
    memset(
        za as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<zarray_t>() as libc::c_ulong,
    );
    free(za as *mut libc::c_void);
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
    let ref mut fresh1 = (*za).data;
    *fresh1 = realloc(
        (*za).data as *mut libc::c_void,
        ((*za).alloc as libc::c_ulong).wrapping_mul((*za).el_sz),
    ) as *mut libc::c_char;
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
    let ref mut fresh2 = (*za).size;
    *fresh2 += 1;
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
unsafe extern "C" fn zarray_get_volatile(
    mut za: *const zarray_t,
    mut idx: libc::c_int,
    mut p: *mut libc::c_void,
) {
    let ref mut fresh3 = *(p as *mut *mut libc::c_void);
    *fresh3 = &mut *((*za).data)
        .offset((idx as libc::c_ulong).wrapping_mul((*za).el_sz) as isize)
        as *mut libc::c_char as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn zarray_remove_index(
    mut za: *mut zarray_t,
    mut idx: libc::c_int,
    mut shuffle: libc::c_int,
) {
    if shuffle != 0 {
        if idx < (*za).size - 1 as libc::c_int {
            memcpy(
                &mut *((*za).data)
                    .offset((idx as libc::c_ulong).wrapping_mul((*za).el_sz) as isize)
                    as *mut libc::c_char as *mut libc::c_void,
                &mut *((*za).data)
                    .offset(
                        (((*za).size - 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul((*za).el_sz) as isize,
                    ) as *mut libc::c_char as *const libc::c_void,
                (*za).el_sz,
            );
        }
        let ref mut fresh4 = (*za).size;
        *fresh4 -= 1;
        return;
    } else {
        let mut ncopy: libc::c_int = (*za).size - idx - 1 as libc::c_int;
        if ncopy > 0 as libc::c_int {
            memmove(
                &mut *((*za).data)
                    .offset((idx as libc::c_ulong).wrapping_mul((*za).el_sz) as isize)
                    as *mut libc::c_char as *mut libc::c_void,
                &mut *((*za).data)
                    .offset(
                        ((idx + 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul((*za).el_sz) as isize,
                    ) as *mut libc::c_char as *const libc::c_void,
                (ncopy as libc::c_ulong).wrapping_mul((*za).el_sz),
            );
        }
        let ref mut fresh5 = (*za).size;
        *fresh5 -= 1;
        return;
    };
}
#[inline]
unsafe extern "C" fn zarray_remove_value(
    mut za: *mut zarray_t,
    mut p: *const libc::c_void,
    mut shuffle: libc::c_int,
) -> libc::c_int {
    let mut idx: libc::c_int = 0 as libc::c_int;
    while idx < (*za).size {
        if memcmp(
            p,
            &mut *((*za).data)
                .offset((idx as libc::c_ulong).wrapping_mul((*za).el_sz) as isize)
                as *mut libc::c_char as *const libc::c_void,
            (*za).el_sz,
        ) == 0
        {
            zarray_remove_index(za, idx, shuffle);
            return 1 as libc::c_int;
        }
        idx += 1;
    }
    return 0 as libc::c_int;
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
unsafe extern "C" fn zarray_clear(mut za: *mut zarray_t) {
    (*za).size = 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn zarray_sort(
    mut za: *mut zarray_t,
    mut compar: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
) {
    if (*za).size == 0 as libc::c_int {
        return;
    }
    qsort((*za).data as *mut libc::c_void, (*za).size as size_t, (*za).el_sz, compar);
}
#[inline]
unsafe extern "C" fn timeprofile_destroy(mut tp: *mut timeprofile_t) {
    zarray_destroy((*tp).stamps);
    free(tp as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn timeprofile_clear(mut tp: *mut timeprofile_t) {
    zarray_clear((*tp).stamps);
    (*tp).utime = utime_now();
}
#[inline]
unsafe extern "C" fn homography_project(
    mut H: *const matd_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut ox: *mut libc::c_double,
    mut oy: *mut libc::c_double,
) {
    let mut xx: libc::c_double = *((*H).data)
        .as_ptr()
        .offset(
            (0 as libc::c_int as libc::c_uint)
                .wrapping_mul((*H).ncols)
                .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
        ) * x
        + *((*H).data)
            .as_ptr()
            .offset(
                (0 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*H).ncols)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) * y
        + *((*H).data)
            .as_ptr()
            .offset(
                (0 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*H).ncols)
                    .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
            );
    let mut yy: libc::c_double = *((*H).data)
        .as_ptr()
        .offset(
            (1 as libc::c_int as libc::c_uint)
                .wrapping_mul((*H).ncols)
                .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
        ) * x
        + *((*H).data)
            .as_ptr()
            .offset(
                (1 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*H).ncols)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) * y
        + *((*H).data)
            .as_ptr()
            .offset(
                (1 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*H).ncols)
                    .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
            );
    let mut zz: libc::c_double = *((*H).data)
        .as_ptr()
        .offset(
            (2 as libc::c_int as libc::c_uint)
                .wrapping_mul((*H).ncols)
                .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
        ) * x
        + *((*H).data)
            .as_ptr()
            .offset(
                (2 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*H).ncols)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) * y
        + *((*H).data)
            .as_ptr()
            .offset(
                (2 as libc::c_int as libc::c_uint)
                    .wrapping_mul((*H).ncols)
                    .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
            );
    *ox = xx / zz;
    *oy = yy / zz;
}
#[inline]
unsafe extern "C" fn imin(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn imax(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn mat33_chol(
    mut A: *const libc::c_double,
    mut R: *mut libc::c_double,
) {
    *R.offset(0 as libc::c_int as isize) = sqrt(*A.offset(0 as libc::c_int as isize));
    *R
        .offset(
            3 as libc::c_int as isize,
        ) = *A.offset(1 as libc::c_int as isize) / *R.offset(0 as libc::c_int as isize);
    *R
        .offset(
            6 as libc::c_int as isize,
        ) = *A.offset(2 as libc::c_int as isize) / *R.offset(0 as libc::c_int as isize);
    *R
        .offset(
            4 as libc::c_int as isize,
        ) = sqrt(
        *A.offset(4 as libc::c_int as isize)
            - *R.offset(3 as libc::c_int as isize) * *R.offset(3 as libc::c_int as isize),
    );
    *R
        .offset(
            7 as libc::c_int as isize,
        ) = (*A.offset(5 as libc::c_int as isize)
        - *R.offset(3 as libc::c_int as isize) * *R.offset(6 as libc::c_int as isize))
        / *R.offset(4 as libc::c_int as isize);
    *R
        .offset(
            8 as libc::c_int as isize,
        ) = sqrt(
        *A.offset(8 as libc::c_int as isize)
            - *R.offset(6 as libc::c_int as isize) * *R.offset(6 as libc::c_int as isize)
            - *R.offset(7 as libc::c_int as isize) * *R.offset(7 as libc::c_int as isize),
    );
    *R.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
    *R.offset(2 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
    *R.offset(5 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
}
#[inline]
unsafe extern "C" fn mat33_lower_tri_inv(
    mut A: *const libc::c_double,
    mut R: *mut libc::c_double,
) {
    *R
        .offset(
            0 as libc::c_int as isize,
        ) = 1 as libc::c_int as libc::c_double / *A.offset(0 as libc::c_int as isize);
    *R
        .offset(
            3 as libc::c_int as isize,
        ) = -*A.offset(3 as libc::c_int as isize) * *R.offset(0 as libc::c_int as isize)
        / *A.offset(4 as libc::c_int as isize);
    *R
        .offset(
            4 as libc::c_int as isize,
        ) = 1 as libc::c_int as libc::c_double / *A.offset(4 as libc::c_int as isize);
    *R
        .offset(
            6 as libc::c_int as isize,
        ) = (-*A.offset(6 as libc::c_int as isize) * *R.offset(0 as libc::c_int as isize)
        - *A.offset(7 as libc::c_int as isize) * *R.offset(3 as libc::c_int as isize))
        / *A.offset(8 as libc::c_int as isize);
    *R
        .offset(
            7 as libc::c_int as isize,
        ) = -*A.offset(7 as libc::c_int as isize) * *R.offset(4 as libc::c_int as isize)
        / *A.offset(8 as libc::c_int as isize);
    *R
        .offset(
            8 as libc::c_int as isize,
        ) = 1 as libc::c_int as libc::c_double / *A.offset(8 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn mat33_sym_solve(
    mut A: *const libc::c_double,
    mut B: *const libc::c_double,
    mut R: *mut libc::c_double,
) {
    let mut L: [libc::c_double; 9] = [0.; 9];
    mat33_chol(A, L.as_mut_ptr());
    let mut M: [libc::c_double; 9] = [0.; 9];
    mat33_lower_tri_inv(L.as_mut_ptr(), M.as_mut_ptr());
    let mut tmp: [libc::c_double; 3] = [0.; 3];
    tmp[0 as libc::c_int
        as usize] = M[0 as libc::c_int as usize] * *B.offset(0 as libc::c_int as isize);
    tmp[1 as libc::c_int
        as usize] = M[3 as libc::c_int as usize] * *B.offset(0 as libc::c_int as isize)
        + M[4 as libc::c_int as usize] * *B.offset(1 as libc::c_int as isize);
    tmp[2 as libc::c_int
        as usize] = M[6 as libc::c_int as usize] * *B.offset(0 as libc::c_int as isize)
        + M[7 as libc::c_int as usize] * *B.offset(1 as libc::c_int as isize)
        + M[8 as libc::c_int as usize] * *B.offset(2 as libc::c_int as isize);
    *R
        .offset(
            0 as libc::c_int as isize,
        ) = M[0 as libc::c_int as usize] * tmp[0 as libc::c_int as usize]
        + M[3 as libc::c_int as usize] * tmp[1 as libc::c_int as usize]
        + M[6 as libc::c_int as usize] * tmp[2 as libc::c_int as usize];
    *R
        .offset(
            1 as libc::c_int as isize,
        ) = M[4 as libc::c_int as usize] * tmp[1 as libc::c_int as usize]
        + M[7 as libc::c_int as usize] * tmp[2 as libc::c_int as usize];
    *R
        .offset(
            2 as libc::c_int as isize,
        ) = M[8 as libc::c_int as usize] * tmp[2 as libc::c_int as usize];
}
unsafe extern "C" fn postscript_image(mut f: *mut FILE, mut im: *mut image_u8_t) {
    fprintf(
        f,
        b"/picstr %d string def\n\0" as *const u8 as *const libc::c_char,
        (*im).width,
    );
    fprintf(
        f,
        b"%d %d 8 [1 0 0 1 0 0]\n\0" as *const u8 as *const libc::c_char,
        (*im).width,
        (*im).height,
    );
    fprintf(
        f,
        b"{currentfile picstr readhexstring pop}\nimage\n\0" as *const u8
            as *const libc::c_char,
    );
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < (*im).height {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < (*im).width {
            let mut v: uint8_t = *((*im).buf).offset((y * (*im).stride + x) as isize);
            fprintf(f, b"%02x\0" as *const u8 as *const libc::c_char, v as libc::c_int);
            if x % 32 as libc::c_int == 31 as libc::c_int {
                fprintf(f, b"\n\0" as *const u8 as *const libc::c_char);
            }
            x += 1;
        }
        y += 1;
    }
    fprintf(f, b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn graymodel_init(mut gm: *mut graymodel) {
    memset(
        gm as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<graymodel>() as libc::c_ulong,
    );
}
unsafe extern "C" fn graymodel_add(
    mut gm: *mut graymodel,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut gray: libc::c_double,
) {
    (*gm).A[0 as libc::c_int as usize][0 as libc::c_int as usize] += x * x;
    (*gm).A[0 as libc::c_int as usize][1 as libc::c_int as usize] += x * y;
    (*gm).A[0 as libc::c_int as usize][2 as libc::c_int as usize] += x;
    (*gm).A[1 as libc::c_int as usize][1 as libc::c_int as usize] += y * y;
    (*gm).A[1 as libc::c_int as usize][2 as libc::c_int as usize] += y;
    (*gm).A[2 as libc::c_int as usize][2 as libc::c_int as usize]
        += 1 as libc::c_int as libc::c_double;
    (*gm).B[0 as libc::c_int as usize] += x * gray;
    (*gm).B[1 as libc::c_int as usize] += y * gray;
    (*gm).B[2 as libc::c_int as usize] += gray;
}
unsafe extern "C" fn graymodel_solve(mut gm: *mut graymodel) {
    mat33_sym_solve(
        ((*gm).A).as_mut_ptr() as *mut libc::c_double,
        ((*gm).B).as_mut_ptr(),
        ((*gm).C).as_mut_ptr(),
    );
}
unsafe extern "C" fn graymodel_interpolate(
    mut gm: *mut graymodel,
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> libc::c_double {
    return (*gm).C[0 as libc::c_int as usize] * x
        + (*gm).C[1 as libc::c_int as usize] * y + (*gm).C[2 as libc::c_int as usize];
}
unsafe extern "C" fn rotate90(mut w: uint64_t, mut numBits: libc::c_int) -> uint64_t {
    let mut p: libc::c_int = numBits;
    let mut l: uint64_t = 0 as libc::c_int as uint64_t;
    if numBits % 4 as libc::c_int == 1 as libc::c_int {
        p = numBits - 1 as libc::c_int;
        l = 1 as libc::c_int as uint64_t;
    }
    w = w >> l << ((p / 4 as libc::c_int) as libc::c_ulong).wrapping_add(l)
        | w
            >> ((3 as libc::c_int * p / 4 as libc::c_int) as libc::c_ulong)
                .wrapping_add(l) << l | w & l;
    w
        &= ((1 as libc::c_int as uint64_t) << numBits)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    return w;
}
unsafe extern "C" fn quad_destroy(mut quad: *mut quad) {
    if quad.is_null() {
        return;
    }
    matd_destroy((*quad).H);
    matd_destroy((*quad).Hinv);
    free(quad as *mut libc::c_void);
}
unsafe extern "C" fn quad_copy(mut quad: *mut quad) -> *mut quad {
    let mut q: *mut quad = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<quad>() as libc::c_ulong,
    ) as *mut quad;
    memcpy(
        q as *mut libc::c_void,
        quad as *const libc::c_void,
        ::std::mem::size_of::<quad>() as libc::c_ulong,
    );
    if !((*quad).H).is_null() {
        let ref mut fresh6 = (*q).H;
        *fresh6 = matd_copy((*quad).H);
    }
    if !((*quad).Hinv).is_null() {
        let ref mut fresh7 = (*q).Hinv;
        *fresh7 = matd_copy((*quad).Hinv);
    }
    return q;
}
unsafe extern "C" fn quick_decode_add(
    mut qd: *mut quick_decode,
    mut code: uint64_t,
    mut id: libc::c_int,
    mut hamming: libc::c_int,
) {
    let mut bucket: uint32_t = code.wrapping_rem((*qd).nentries as libc::c_ulong)
        as uint32_t;
    while (*((*qd).entries).offset(bucket as isize)).rcode
        != 18446744073709551615 as libc::c_ulong
    {
        bucket = bucket
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_rem((*qd).nentries as libc::c_uint);
    }
    (*((*qd).entries).offset(bucket as isize)).rcode = code;
    (*((*qd).entries).offset(bucket as isize)).id = id as uint16_t;
    (*((*qd).entries).offset(bucket as isize)).hamming = hamming as uint8_t;
}
unsafe extern "C" fn quick_decode_uninit(mut fam: *mut apriltag_family_t) {
    if ((*fam).impl_0).is_null() {
        return;
    }
    let mut qd: *mut quick_decode = (*fam).impl_0 as *mut quick_decode;
    free((*qd).entries as *mut libc::c_void);
    free(qd as *mut libc::c_void);
    let ref mut fresh8 = (*fam).impl_0;
    *fresh8 = 0 as *mut libc::c_void;
}
unsafe extern "C" fn quick_decode_init(
    mut family: *mut apriltag_family_t,
    mut maxhamming: libc::c_int,
) {
    let mut qd: *mut quick_decode = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<quick_decode>() as libc::c_ulong,
    ) as *mut quick_decode;
    let mut capacity: libc::c_int = (*family).ncodes as libc::c_int;
    let mut nbits: libc::c_int = (*family).nbits as libc::c_int;
    if maxhamming >= 1 as libc::c_int {
        capacity = (capacity as libc::c_uint)
            .wrapping_add(((*family).ncodes).wrapping_mul(nbits as libc::c_uint))
            as libc::c_int as libc::c_int;
    }
    if maxhamming >= 2 as libc::c_int {
        capacity = (capacity as libc::c_uint)
            .wrapping_add(
                ((*family).ncodes)
                    .wrapping_mul(nbits as libc::c_uint)
                    .wrapping_mul((nbits - 1 as libc::c_int) as libc::c_uint),
            ) as libc::c_int as libc::c_int;
    }
    if maxhamming >= 3 as libc::c_int {
        capacity = (capacity as libc::c_uint)
            .wrapping_add(
                ((*family).ncodes)
                    .wrapping_mul(nbits as libc::c_uint)
                    .wrapping_mul((nbits - 1 as libc::c_int) as libc::c_uint)
                    .wrapping_mul((nbits - 2 as libc::c_int) as libc::c_uint),
            ) as libc::c_int as libc::c_int;
    }
    (*qd).nentries = capacity * 3 as libc::c_int;
    let ref mut fresh9 = (*qd).entries;
    *fresh9 = calloc(
        (*qd).nentries as libc::c_ulong,
        ::std::mem::size_of::<quick_decode_entry>() as libc::c_ulong,
    ) as *mut quick_decode_entry;
    if ((*qd).entries).is_null() {
        fflush(stderr);
        return;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*qd).nentries {
        (*((*qd).entries).offset(i as isize))
            .rcode = 18446744073709551615 as libc::c_ulong;
        i += 1;
    }
    *__errno_location() = 0 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while (i_0 as libc::c_uint) < (*family).ncodes {
        let mut code: uint64_t = *((*family).codes).offset(i_0 as isize);
        quick_decode_add(qd, code, i_0, 0 as libc::c_int);
        if maxhamming >= 1 as libc::c_int {
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < nbits {
                quick_decode_add(
                    qd,
                    code ^ (1 as libc::c_int as uint64_t) << j,
                    i_0,
                    1 as libc::c_int,
                );
                j += 1;
            }
        }
        if maxhamming >= 2 as libc::c_int {
            let mut j_0: libc::c_int = 0 as libc::c_int;
            while j_0 < nbits {
                let mut k: libc::c_int = 0 as libc::c_int;
                while k < j_0 {
                    quick_decode_add(
                        qd,
                        code ^ (1 as libc::c_int as uint64_t) << j_0
                            ^ (1 as libc::c_int as uint64_t) << k,
                        i_0,
                        2 as libc::c_int,
                    );
                    k += 1;
                }
                j_0 += 1;
            }
        }
        if maxhamming >= 3 as libc::c_int {
            let mut j_1: libc::c_int = 0 as libc::c_int;
            while j_1 < nbits {
                let mut k_0: libc::c_int = 0 as libc::c_int;
                while k_0 < j_1 {
                    let mut m: libc::c_int = 0 as libc::c_int;
                    while m < k_0 {
                        quick_decode_add(
                            qd,
                            code ^ (1 as libc::c_int as uint64_t) << j_1
                                ^ (1 as libc::c_int as uint64_t) << k_0
                                ^ (1 as libc::c_int as uint64_t) << m,
                            i_0,
                            3 as libc::c_int,
                        );
                        m += 1;
                    }
                    k_0 += 1;
                }
                j_1 += 1;
            }
        }
        if maxhamming > 3 as libc::c_int {
            fflush(stderr);
            *__errno_location() = 22 as libc::c_int;
            return;
        }
        i_0 += 1;
    }
    let ref mut fresh10 = (*family).impl_0;
    *fresh10 = qd as *mut libc::c_void;
}
unsafe extern "C" fn quick_decode_codeword(
    mut tf: *mut apriltag_family_t,
    mut rcode: uint64_t,
    mut entry: *mut quick_decode_entry,
) {
    let mut qd: *mut quick_decode = (*tf).impl_0 as *mut quick_decode;
    let mut ridx: libc::c_int = 0 as libc::c_int;
    while !qd.is_null() && ridx < 4 as libc::c_int {
        let mut bucket: libc::c_int = rcode.wrapping_rem((*qd).nentries as libc::c_ulong)
            as libc::c_int;
        while (*((*qd).entries).offset(bucket as isize)).rcode
            != 18446744073709551615 as libc::c_ulong
        {
            if (*((*qd).entries).offset(bucket as isize)).rcode == rcode {
                *entry = *((*qd).entries).offset(bucket as isize);
                (*entry).rotation = ridx as uint8_t;
                return;
            }
            bucket = (bucket + 1 as libc::c_int) % (*qd).nentries;
        }
        rcode = rotate90(rcode, (*tf).nbits as libc::c_int);
        ridx += 1;
    }
    (*entry).rcode = 0 as libc::c_int as uint64_t;
    (*entry).id = 65535 as libc::c_int as uint16_t;
    (*entry).hamming = 255 as libc::c_int as uint8_t;
    (*entry).rotation = 0 as libc::c_int as uint8_t;
}
#[inline]
unsafe extern "C" fn detection_compare_function(
    mut _a: *const libc::c_void,
    mut _b: *const libc::c_void,
) -> libc::c_int {
    let mut a: *mut apriltag_detection_t = *(_a as *mut *mut apriltag_detection_t);
    let mut b: *mut apriltag_detection_t = *(_b as *mut *mut apriltag_detection_t);
    return (*a).id - (*b).id;
}
#[no_mangle]
pub unsafe extern "C" fn apriltag_detector_remove_family(
    mut td: *mut apriltag_detector_t,
    mut fam: *mut apriltag_family_t,
) {
    quick_decode_uninit(fam);
    zarray_remove_value(
        (*td).tag_families,
        &mut fam as *mut *mut apriltag_family_t as *const libc::c_void,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn apriltag_detector_add_family_bits(
    mut td: *mut apriltag_detector_t,
    mut fam: *mut apriltag_family_t,
    mut bits_corrected: libc::c_int,
) {
    zarray_add(
        (*td).tag_families,
        &mut fam as *mut *mut apriltag_family_t as *const libc::c_void,
    );
    if ((*fam).impl_0).is_null() {
        quick_decode_init(fam, bits_corrected);
    }
}
#[no_mangle]
pub unsafe extern "C" fn apriltag_detector_clear_families(
    mut td: *mut apriltag_detector_t,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < zarray_size((*td).tag_families) {
        let mut fam: *mut apriltag_family_t = 0 as *mut apriltag_family_t;
        zarray_get(
            (*td).tag_families,
            i,
            &mut fam as *mut *mut apriltag_family_t as *mut libc::c_void,
        );
        quick_decode_uninit(fam);
        i += 1;
    }
    zarray_clear((*td).tag_families);
}
#[no_mangle]
pub unsafe extern "C" fn apriltag_detector_create() -> *mut apriltag_detector_t {
    let mut td: *mut apriltag_detector_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<apriltag_detector_t>() as libc::c_ulong,
    ) as *mut apriltag_detector_t;
    (*td).nthreads = 1 as libc::c_int;
    (*td).quad_decimate = 2.0f64 as libc::c_float;
    (*td).quad_sigma = 0.0f64 as libc::c_float;
    (*td).qtp.max_nmaxima = 10 as libc::c_int;
    (*td).qtp.min_cluster_pixels = 5 as libc::c_int;
    (*td).qtp.max_line_fit_mse = 10.0f64 as libc::c_float;
    (*td)
        .qtp
        .cos_critical_rad = cos(
        10 as libc::c_int as libc::c_double * 3.14159265358979323846f64
            / 180 as libc::c_int as libc::c_double,
    ) as libc::c_float;
    (*td).qtp.deglitch = 0 as libc::c_int;
    (*td).qtp.min_white_black_diff = 5 as libc::c_int;
    let ref mut fresh11 = (*td).tag_families;
    *fresh11 = zarray_create(
        ::std::mem::size_of::<*mut apriltag_family_t>() as libc::c_ulong,
    );
    pthread_mutex_init(&mut (*td).mutex, 0 as *const pthread_mutexattr_t);
    let ref mut fresh12 = (*td).tp;
    *fresh12 = timeprofile_create();
    (*td).refine_edges = 1 as libc::c_int;
    (*td).decode_sharpening = 0.25f64;
    (*td).debug = 0 as libc::c_int;
    return td;
}
#[no_mangle]
pub unsafe extern "C" fn apriltag_detector_destroy(mut td: *mut apriltag_detector_t) {
    timeprofile_destroy((*td).tp);
    workerpool_destroy((*td).wp);
    apriltag_detector_clear_families(td);
    zarray_destroy((*td).tag_families);
    free(td as *mut libc::c_void);
}
unsafe extern "C" fn homography_compute2(
    mut c: *mut [libc::c_double; 4],
) -> *mut matd_t {
    let mut A: [libc::c_double; 72] = [
        (*c.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize],
        (*c.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize],
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        -(*c.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
            * (*c.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize],
        -(*c.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*c.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize],
        (*c.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize],
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        (*c.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize],
        (*c.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize],
        1 as libc::c_int as libc::c_double,
        -(*c.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
            * (*c.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize],
        -(*c.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*c.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize],
        (*c.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize],
        (*c.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize],
        (*c.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize],
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        -(*c.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
            * (*c.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize],
        -(*c.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*c.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize],
        (*c.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize],
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        (*c.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize],
        (*c.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize],
        1 as libc::c_int as libc::c_double,
        -(*c.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
            * (*c.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize],
        -(*c.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*c.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize],
        (*c.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize],
        (*c.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize],
        (*c.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize],
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        -(*c.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
            * (*c.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize],
        -(*c.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*c.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize],
        (*c.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize],
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        (*c.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize],
        (*c.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize],
        1 as libc::c_int as libc::c_double,
        -(*c.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
            * (*c.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize],
        -(*c.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*c.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize],
        (*c.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize],
        (*c.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize],
        (*c.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize],
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        -(*c.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize]
            * (*c.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize],
        -(*c.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*c.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize],
        (*c.offset(3 as libc::c_int as isize))[2 as libc::c_int as usize],
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        (*c.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize],
        (*c.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize],
        1 as libc::c_int as libc::c_double,
        -(*c.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize]
            * (*c.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize],
        -(*c.offset(3 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*c.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize],
        (*c.offset(3 as libc::c_int as isize))[3 as libc::c_int as usize],
    ];
    let mut epsilon: libc::c_double = 1e-10f64;
    let mut col: libc::c_int = 0 as libc::c_int;
    while col < 8 as libc::c_int {
        let mut max_val: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut max_val_idx: libc::c_int = -(1 as libc::c_int);
        let mut row: libc::c_int = col;
        while row < 8 as libc::c_int {
            let mut val: libc::c_double = fabs(
                A[(row * 9 as libc::c_int + col) as usize],
            );
            if val > max_val {
                max_val = val;
                max_val_idx = row;
            }
            row += 1;
        }
        if max_val < epsilon {
            fflush(stderr);
            return 0 as *mut matd_t;
        }
        if max_val_idx != col {
            let mut i: libc::c_int = col;
            while i < 9 as libc::c_int {
                let mut tmp: libc::c_double = A[(col * 9 as libc::c_int + i) as usize];
                A[(col * 9 as libc::c_int + i)
                    as usize] = A[(max_val_idx * 9 as libc::c_int + i) as usize];
                A[(max_val_idx * 9 as libc::c_int + i) as usize] = tmp;
                i += 1;
            }
        }
        let mut i_0: libc::c_int = col + 1 as libc::c_int;
        while i_0 < 8 as libc::c_int {
            let mut f: libc::c_double = A[(i_0 * 9 as libc::c_int + col) as usize]
                / A[(col * 9 as libc::c_int + col) as usize];
            A[(i_0 * 9 as libc::c_int + col)
                as usize] = 0 as libc::c_int as libc::c_double;
            let mut j: libc::c_int = col + 1 as libc::c_int;
            while j < 9 as libc::c_int {
                A[(i_0 * 9 as libc::c_int + j) as usize]
                    -= f * A[(col * 9 as libc::c_int + j) as usize];
                j += 1;
            }
            i_0 += 1;
        }
        col += 1;
    }
    let mut col_0: libc::c_int = 7 as libc::c_int;
    while col_0 >= 0 as libc::c_int {
        let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut i_1: libc::c_int = col_0 + 1 as libc::c_int;
        while i_1 < 8 as libc::c_int {
            sum
                += A[(col_0 * 9 as libc::c_int + i_1) as usize]
                    * A[(i_1 * 9 as libc::c_int + 8 as libc::c_int) as usize];
            i_1 += 1;
        }
        A[(col_0 * 9 as libc::c_int + 8 as libc::c_int)
            as usize] = (A[(col_0 * 9 as libc::c_int + 8 as libc::c_int) as usize] - sum)
            / A[(col_0 * 9 as libc::c_int + col_0) as usize];
        col_0 -= 1;
    }
    return matd_create_data(
        3 as libc::c_int,
        3 as libc::c_int,
        [
            A[8 as libc::c_int as usize],
            A[17 as libc::c_int as usize],
            A[26 as libc::c_int as usize],
            A[35 as libc::c_int as usize],
            A[44 as libc::c_int as usize],
            A[53 as libc::c_int as usize],
            A[62 as libc::c_int as usize],
            A[71 as libc::c_int as usize],
            1 as libc::c_int as libc::c_double,
        ]
            .as_mut_ptr(),
    );
}
unsafe extern "C" fn quad_update_homographies(mut quad: *mut quad) -> libc::c_int {
    let mut corr_arr: [[libc::c_double; 4]; 4] = [[0.; 4]; 4];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        corr_arr[i
            as usize][0 as libc::c_int
            as usize] = (if i == 0 as libc::c_int || i == 3 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        }) as libc::c_double;
        corr_arr[i
            as usize][1 as libc::c_int
            as usize] = (if i == 0 as libc::c_int || i == 1 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        }) as libc::c_double;
        corr_arr[i
            as usize][2 as libc::c_int
            as usize] = (*quad).p[i as usize][0 as libc::c_int as usize]
            as libc::c_double;
        corr_arr[i
            as usize][3 as libc::c_int
            as usize] = (*quad).p[i as usize][1 as libc::c_int as usize]
            as libc::c_double;
        i += 1;
    }
    if !((*quad).H).is_null() {
        matd_destroy((*quad).H);
    }
    if !((*quad).Hinv).is_null() {
        matd_destroy((*quad).Hinv);
    }
    let ref mut fresh13 = (*quad).H;
    *fresh13 = homography_compute2(corr_arr.as_mut_ptr());
    if !((*quad).H).is_null() {
        let ref mut fresh14 = (*quad).Hinv;
        *fresh14 = matd_inverse((*quad).H);
        if !((*quad).Hinv).is_null() {
            return 0 as libc::c_int;
        }
        matd_destroy((*quad).H);
        let ref mut fresh15 = (*quad).H;
        *fresh15 = 0 as *mut matd_t;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn value_for_pixel(
    mut im: *mut image_u8_t,
    mut px: libc::c_double,
    mut py: libc::c_double,
) -> libc::c_double {
    let mut x1: libc::c_int = floor(px - 0.5f64) as libc::c_int;
    let mut x2: libc::c_int = ceil(px - 0.5f64) as libc::c_int;
    let mut x: libc::c_double = px - 0.5f64 - x1 as libc::c_double;
    let mut y1: libc::c_int = floor(py - 0.5f64) as libc::c_int;
    let mut y2: libc::c_int = ceil(py - 0.5f64) as libc::c_int;
    let mut y: libc::c_double = py - 0.5f64 - y1 as libc::c_double;
    if x1 < 0 as libc::c_int || x2 >= (*im).width || y1 < 0 as libc::c_int
        || y2 >= (*im).height
    {
        return -(1 as libc::c_int) as libc::c_double;
    }
    return *((*im).buf).offset((y1 * (*im).stride + x1) as isize) as libc::c_int
        as libc::c_double * (1 as libc::c_int as libc::c_double - x)
        * (1 as libc::c_int as libc::c_double - y)
        + *((*im).buf).offset((y1 * (*im).stride + x2) as isize) as libc::c_int
            as libc::c_double * x * (1 as libc::c_int as libc::c_double - y)
        + *((*im).buf).offset((y2 * (*im).stride + x1) as isize) as libc::c_int
            as libc::c_double * (1 as libc::c_int as libc::c_double - x) * y
        + *((*im).buf).offset((y2 * (*im).stride + x2) as isize) as libc::c_int
            as libc::c_double * x * y;
}
unsafe extern "C" fn sharpen(
    mut td: *mut apriltag_detector_t,
    mut values: *mut libc::c_double,
    mut size: libc::c_int,
) {
    let mut sharpened: *mut libc::c_double = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(size as libc::c_ulong)
            .wrapping_mul(size as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut kernel: [libc::c_double; 9] = [
        0 as libc::c_int as libc::c_double,
        -(1 as libc::c_int) as libc::c_double,
        0 as libc::c_int as libc::c_double,
        -(1 as libc::c_int) as libc::c_double,
        4 as libc::c_int as libc::c_double,
        -(1 as libc::c_int) as libc::c_double,
        0 as libc::c_int as libc::c_double,
        -(1 as libc::c_int) as libc::c_double,
        0 as libc::c_int as libc::c_double,
    ];
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < size {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < size {
            *sharpened
                .offset((y * size + x) as isize) = 0 as libc::c_int as libc::c_double;
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < 3 as libc::c_int {
                let mut j: libc::c_int = 0 as libc::c_int;
                while j < 3 as libc::c_int {
                    if !((y + i - 1 as libc::c_int) < 0 as libc::c_int
                        || y + i - 1 as libc::c_int > size - 1 as libc::c_int
                        || (x + j - 1 as libc::c_int) < 0 as libc::c_int
                        || x + j - 1 as libc::c_int > size - 1 as libc::c_int)
                    {
                        *sharpened.offset((y * size + x) as isize)
                            += *values
                                .offset(
                                    ((y + i - 1 as libc::c_int) * size
                                        + (x + j - 1 as libc::c_int)) as isize,
                                ) * kernel[(i * 3 as libc::c_int + j) as usize];
                    }
                    j += 1;
                }
                i += 1;
            }
            x += 1;
        }
        y += 1;
    }
    let mut y_0: libc::c_int = 0 as libc::c_int;
    while y_0 < size {
        let mut x_0: libc::c_int = 0 as libc::c_int;
        while x_0 < size {
            *values
                .offset(
                    (y_0 * size + x_0) as isize,
                ) = *values.offset((y_0 * size + x_0) as isize)
                + (*td).decode_sharpening
                    * *sharpened.offset((y_0 * size + x_0) as isize);
            x_0 += 1;
        }
        y_0 += 1;
    }
    free(sharpened as *mut libc::c_void);
}
unsafe extern "C" fn quad_decode(
    mut td: *mut apriltag_detector_t,
    mut family: *mut apriltag_family_t,
    mut im: *mut image_u8_t,
    mut quad: *mut quad,
    mut entry: *mut quick_decode_entry,
    mut im_samples: *mut image_u8_t,
) -> libc::c_float {
    let mut patterns: [libc::c_float; 40] = [
        -0.5f64 as libc::c_float,
        0.5f64 as libc::c_float,
        0 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0.5f64 as libc::c_float,
        0.5f64 as libc::c_float,
        0 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        ((*family).width_at_border as libc::c_double + 0.5f64) as libc::c_float,
        0.5f64 as libc::c_float,
        0 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        ((*family).width_at_border as libc::c_double - 0.5f64) as libc::c_float,
        0.5f64 as libc::c_float,
        0 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0.5f64 as libc::c_float,
        -0.5f64 as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0.5f64 as libc::c_float,
        0.5f64 as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0.5f64 as libc::c_float,
        ((*family).width_at_border as libc::c_double + 0.5f64) as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0.5f64 as libc::c_float,
        ((*family).width_at_border as libc::c_double - 0.5f64) as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    ];
    let mut whitemodel: graymodel = graymodel {
        A: [[0.; 3]; 3],
        B: [0.; 3],
        C: [0.; 3],
    };
    let mut blackmodel: graymodel = graymodel {
        A: [[0.; 3]; 3],
        B: [0.; 3],
        C: [0.; 3],
    };
    graymodel_init(&mut whitemodel);
    graymodel_init(&mut blackmodel);
    let mut pattern_idx: libc::c_int = 0 as libc::c_int;
    while (pattern_idx as libc::c_ulong)
        < (::std::mem::size_of::<[libc::c_float; 40]>() as libc::c_ulong)
            .wrapping_div(
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
                    ),
            )
    {
        let mut pattern: *mut libc::c_float = &mut *patterns
            .as_mut_ptr()
            .offset((pattern_idx * 5 as libc::c_int) as isize) as *mut libc::c_float;
        let mut is_white: libc::c_int = *pattern.offset(4 as libc::c_int as isize)
            as libc::c_int;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*family).width_at_border {
            let mut tagx01: libc::c_double = ((*pattern.offset(0 as libc::c_int as isize)
                + i as libc::c_float * *pattern.offset(2 as libc::c_int as isize))
                / (*family).width_at_border as libc::c_float) as libc::c_double;
            let mut tagy01: libc::c_double = ((*pattern.offset(1 as libc::c_int as isize)
                + i as libc::c_float * *pattern.offset(3 as libc::c_int as isize))
                / (*family).width_at_border as libc::c_float) as libc::c_double;
            let mut tagx: libc::c_double = 2 as libc::c_int as libc::c_double
                * (tagx01 - 0.5f64);
            let mut tagy: libc::c_double = 2 as libc::c_int as libc::c_double
                * (tagy01 - 0.5f64);
            let mut px: libc::c_double = 0.;
            let mut py: libc::c_double = 0.;
            homography_project((*quad).H, tagx, tagy, &mut px, &mut py);
            let mut ix: libc::c_int = px as libc::c_int;
            let mut iy: libc::c_int = py as libc::c_int;
            if !(ix < 0 as libc::c_int || iy < 0 as libc::c_int || ix >= (*im).width
                || iy >= (*im).height)
            {
                let mut v: libc::c_int = *((*im).buf)
                    .offset((iy * (*im).stride + ix) as isize) as libc::c_int;
                if !im_samples.is_null() {
                    *((*im_samples).buf)
                        .offset(
                            (iy * (*im_samples).stride + ix) as isize,
                        ) = ((1 as libc::c_int - is_white) * 255 as libc::c_int)
                        as uint8_t;
                }
                if is_white != 0 {
                    graymodel_add(&mut whitemodel, tagx, tagy, v as libc::c_double);
                } else {
                    graymodel_add(&mut blackmodel, tagx, tagy, v as libc::c_double);
                }
            }
            i += 1;
        }
        pattern_idx += 1;
    }
    if (*family).width_at_border > 1 as libc::c_int {
        graymodel_solve(&mut whitemodel);
        graymodel_solve(&mut blackmodel);
    } else {
        graymodel_solve(&mut whitemodel);
        blackmodel.C[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        blackmodel.C[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_double;
        blackmodel
            .C[2 as libc::c_int
            as usize] = blackmodel.B[2 as libc::c_int as usize]
            / 4 as libc::c_int as libc::c_double;
    }
    if (graymodel_interpolate(
        &mut whitemodel,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    )
        - graymodel_interpolate(
            &mut blackmodel,
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
        ) < 0 as libc::c_int as libc::c_double) as libc::c_int
        != (*family).reversed_border as libc::c_int
    {
        return -(1 as libc::c_int) as libc::c_float;
    }
    let mut black_score: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut white_score: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut black_score_count: libc::c_float = 1 as libc::c_int as libc::c_float;
    let mut white_score_count: libc::c_float = 1 as libc::c_int as libc::c_float;
    let mut values: *mut libc::c_double = calloc(
        ((*family).total_width * (*family).total_width) as libc::c_ulong,
        ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    let mut min_coord: libc::c_int = ((*family).width_at_border - (*family).total_width)
        / 2 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while (i_0 as libc::c_uint) < (*family).nbits {
        let mut bity: libc::c_int = *((*family).bit_y).offset(i_0 as isize)
            as libc::c_int;
        let mut bitx: libc::c_int = *((*family).bit_x).offset(i_0 as isize)
            as libc::c_int;
        let mut tagx01_0: libc::c_double = (bitx as libc::c_double + 0.5f64)
            / (*family).width_at_border as libc::c_double;
        let mut tagy01_0: libc::c_double = (bity as libc::c_double + 0.5f64)
            / (*family).width_at_border as libc::c_double;
        let mut tagx_0: libc::c_double = 2 as libc::c_int as libc::c_double
            * (tagx01_0 - 0.5f64);
        let mut tagy_0: libc::c_double = 2 as libc::c_int as libc::c_double
            * (tagy01_0 - 0.5f64);
        let mut px_0: libc::c_double = 0.;
        let mut py_0: libc::c_double = 0.;
        homography_project((*quad).H, tagx_0, tagy_0, &mut px_0, &mut py_0);
        let mut v_0: libc::c_double = value_for_pixel(im, px_0, py_0);
        if !(v_0 == -(1 as libc::c_int) as libc::c_double) {
            let mut thresh: libc::c_double = (graymodel_interpolate(
                &mut blackmodel,
                tagx_0,
                tagy_0,
            ) + graymodel_interpolate(&mut whitemodel, tagx_0, tagy_0)) / 2.0f64;
            *values
                .offset(
                    ((*family).total_width * (bity - min_coord) + bitx - min_coord)
                        as isize,
                ) = v_0 - thresh;
            if !im_samples.is_null() {
                let mut ix_0: libc::c_int = px_0 as libc::c_int;
                let mut iy_0: libc::c_int = py_0 as libc::c_int;
                *((*im_samples).buf)
                    .offset(
                        (iy_0 * (*im_samples).stride + ix_0) as isize,
                    ) = ((v_0 < thresh) as libc::c_int * 255 as libc::c_int) as uint8_t;
            }
        }
        i_0 += 1;
    }
    sharpen(td, values, (*family).total_width);
    let mut rcode: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while (i_1 as libc::c_uint) < (*family).nbits {
        let mut bity_0: libc::c_int = *((*family).bit_y).offset(i_1 as isize)
            as libc::c_int;
        let mut bitx_0: libc::c_int = *((*family).bit_x).offset(i_1 as isize)
            as libc::c_int;
        rcode = rcode << 1 as libc::c_int;
        let mut v_1: libc::c_double = *values
            .offset(
                ((bity_0 - min_coord) * (*family).total_width + bitx_0 - min_coord)
                    as isize,
            );
        if v_1 > 0 as libc::c_int as libc::c_double {
            white_score = (white_score as libc::c_double + v_1) as libc::c_float;
            white_score_count += 1.;
            rcode |= 1 as libc::c_int as libc::c_ulong;
        } else {
            black_score = (black_score as libc::c_double - v_1) as libc::c_float;
            black_score_count += 1.;
        }
        i_1 += 1;
    }
    quick_decode_codeword(family, rcode, entry);
    free(values as *mut libc::c_void);
    return fmin(
        (white_score / white_score_count) as libc::c_double,
        (black_score / black_score_count) as libc::c_double,
    ) as libc::c_float;
}
unsafe extern "C" fn refine_edges(
    mut td: *mut apriltag_detector_t,
    mut im_orig: *mut image_u8_t,
    mut quad: *mut quad,
) {
    let mut lines: [[libc::c_double; 4]; 4] = [[0.; 4]; 4];
    let mut edge: libc::c_int = 0 as libc::c_int;
    while edge < 4 as libc::c_int {
        let mut a: libc::c_int = edge;
        let mut b: libc::c_int = edge + 1 as libc::c_int & 3 as libc::c_int;
        let mut nx: libc::c_double = ((*quad).p[b as usize][1 as libc::c_int as usize]
            - (*quad).p[a as usize][1 as libc::c_int as usize]) as libc::c_double;
        let mut ny: libc::c_double = (-(*quad).p[b as usize][0 as libc::c_int as usize]
            + (*quad).p[a as usize][0 as libc::c_int as usize]) as libc::c_double;
        let mut mag: libc::c_double = sqrt(nx * nx + ny * ny);
        nx /= mag;
        ny /= mag;
        if (*quad).reversed_border {
            nx = -nx;
            ny = -ny;
        }
        let mut nsamples: libc::c_int = imax(
            16 as libc::c_int,
            (mag / 8 as libc::c_int as libc::c_double) as libc::c_int,
        );
        let mut Mx: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut My: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut Mxx: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut Mxy: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut Myy: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut N: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut s: libc::c_int = 0 as libc::c_int;
        while s < nsamples {
            let mut alpha: libc::c_double = (1.0f64 + s as libc::c_double)
                / (nsamples + 1 as libc::c_int) as libc::c_double;
            let mut x0: libc::c_double = alpha
                * (*quad).p[a as usize][0 as libc::c_int as usize] as libc::c_double
                + (1 as libc::c_int as libc::c_double - alpha)
                    * (*quad).p[b as usize][0 as libc::c_int as usize] as libc::c_double;
            let mut y0: libc::c_double = alpha
                * (*quad).p[a as usize][1 as libc::c_int as usize] as libc::c_double
                + (1 as libc::c_int as libc::c_double - alpha)
                    * (*quad).p[b as usize][1 as libc::c_int as usize] as libc::c_double;
            let mut Mn: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut Mcount: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut range: libc::c_double = ((*td).quad_decimate
                + 1 as libc::c_int as libc::c_float) as libc::c_double;
            let mut n: libc::c_double = -range;
            while n <= range {
                let mut grange: libc::c_double = 1 as libc::c_int as libc::c_double;
                let mut x1: libc::c_int = (x0 + (n + grange) * nx) as libc::c_int;
                let mut y1: libc::c_int = (y0 + (n + grange) * ny) as libc::c_int;
                if !(x1 < 0 as libc::c_int || x1 >= (*im_orig).width
                    || y1 < 0 as libc::c_int || y1 >= (*im_orig).height)
                {
                    let mut x2: libc::c_int = (x0 + (n - grange) * nx) as libc::c_int;
                    let mut y2: libc::c_int = (y0 + (n - grange) * ny) as libc::c_int;
                    if !(x2 < 0 as libc::c_int || x2 >= (*im_orig).width
                        || y2 < 0 as libc::c_int || y2 >= (*im_orig).height)
                    {
                        let mut g1: libc::c_int = *((*im_orig).buf)
                            .offset((y1 * (*im_orig).stride + x1) as isize)
                            as libc::c_int;
                        let mut g2: libc::c_int = *((*im_orig).buf)
                            .offset((y2 * (*im_orig).stride + x2) as isize)
                            as libc::c_int;
                        if !(g1 < g2) {
                            let mut weight: libc::c_double = ((g2 - g1) * (g2 - g1))
                                as libc::c_double;
                            Mn += weight * n;
                            Mcount += weight;
                        }
                    }
                }
                n += 0.25f64;
            }
            if !(Mcount == 0 as libc::c_int as libc::c_double) {
                let mut n0: libc::c_double = Mn / Mcount;
                let mut bestx: libc::c_double = x0 + n0 * nx;
                let mut besty: libc::c_double = y0 + n0 * ny;
                Mx += bestx;
                My += besty;
                Mxx += bestx * bestx;
                Mxy += bestx * besty;
                Myy += besty * besty;
                N += 1.;
            }
            s += 1;
        }
        let mut Ex: libc::c_double = Mx / N;
        let mut Ey: libc::c_double = My / N;
        let mut Cxx: libc::c_double = Mxx / N - Ex * Ex;
        let mut Cxy: libc::c_double = Mxy / N - Ex * Ey;
        let mut Cyy: libc::c_double = Myy / N - Ey * Ey;
        let mut normal_theta: libc::c_double = 0.5f64
            * atan2f(
                (-(2 as libc::c_int) as libc::c_double * Cxy) as libc::c_float,
                (Cyy - Cxx) as libc::c_float,
            ) as libc::c_double;
        nx = cosf(normal_theta as libc::c_float) as libc::c_double;
        ny = sinf(normal_theta as libc::c_float) as libc::c_double;
        lines[edge as usize][0 as libc::c_int as usize] = Ex;
        lines[edge as usize][1 as libc::c_int as usize] = Ey;
        lines[edge as usize][2 as libc::c_int as usize] = nx;
        lines[edge as usize][3 as libc::c_int as usize] = ny;
        edge += 1;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let mut A00: libc::c_double = lines[i as usize][3 as libc::c_int as usize];
        let mut A01: libc::c_double = -lines[(i + 1 as libc::c_int & 3 as libc::c_int)
            as usize][3 as libc::c_int as usize];
        let mut A10: libc::c_double = -lines[i as usize][2 as libc::c_int as usize];
        let mut A11: libc::c_double = lines[(i + 1 as libc::c_int & 3 as libc::c_int)
            as usize][2 as libc::c_int as usize];
        let mut B0: libc::c_double = -lines[i as usize][0 as libc::c_int as usize]
            + lines[(i + 1 as libc::c_int & 3 as libc::c_int)
                as usize][0 as libc::c_int as usize];
        let mut B1: libc::c_double = -lines[i as usize][1 as libc::c_int as usize]
            + lines[(i + 1 as libc::c_int & 3 as libc::c_int)
                as usize][1 as libc::c_int as usize];
        let mut det: libc::c_double = A00 * A11 - A10 * A01;
        if fabs(det) > 0.001f64 {
            let mut W00: libc::c_double = A11 / det;
            let mut W01: libc::c_double = -A01 / det;
            let mut L0: libc::c_double = W00 * B0 + W01 * B1;
            (*quad)
                .p[(i + 1 as libc::c_int & 3 as libc::c_int)
                as usize][0 as libc::c_int
                as usize] = (lines[i as usize][0 as libc::c_int as usize] + L0 * A00)
                as libc::c_float;
            (*quad)
                .p[(i + 1 as libc::c_int & 3 as libc::c_int)
                as usize][1 as libc::c_int
                as usize] = (lines[i as usize][1 as libc::c_int as usize] + L0 * A10)
                as libc::c_float;
        }
        i += 1;
    }
}
unsafe extern "C" fn quad_decode_task(mut _u: *mut libc::c_void) {
    let mut task: *mut quad_decode_task = _u as *mut quad_decode_task;
    let mut td: *mut apriltag_detector_t = (*task).td;
    let mut im: *mut image_u8_t = (*task).im;
    let mut quadidx: libc::c_int = (*task).i0;
    while quadidx < (*task).i1 {
        let mut quad_original: *mut quad = 0 as *mut quad;
        zarray_get_volatile(
            (*task).quads,
            quadidx,
            &mut quad_original as *mut *mut quad as *mut libc::c_void,
        );
        if (*td).refine_edges != 0 {
            refine_edges(td, im, quad_original);
        }
        if !(quad_update_homographies(quad_original) != 0 as libc::c_int) {
            let mut famidx: libc::c_int = 0 as libc::c_int;
            while famidx < zarray_size((*td).tag_families) {
                let mut family: *mut apriltag_family_t = 0 as *mut apriltag_family_t;
                zarray_get(
                    (*td).tag_families,
                    famidx,
                    &mut family as *mut *mut apriltag_family_t as *mut libc::c_void,
                );
                if !((*family).reversed_border as libc::c_int
                    != (*quad_original).reversed_border as libc::c_int)
                {
                    let mut quad: *mut quad = quad_copy(quad_original);
                    let mut entry: quick_decode_entry = quick_decode_entry {
                        rcode: 0,
                        id: 0,
                        hamming: 0,
                        rotation: 0,
                    };
                    let mut decision_margin: libc::c_float = quad_decode(
                        td,
                        family,
                        im,
                        quad,
                        &mut entry,
                        (*task).im_samples,
                    );
                    if decision_margin >= 0 as libc::c_int as libc::c_float
                        && (entry.hamming as libc::c_int) < 255 as libc::c_int
                    {
                        let mut det: *mut apriltag_detection_t = calloc(
                            1 as libc::c_int as libc::c_ulong,
                            ::std::mem::size_of::<apriltag_detection_t>()
                                as libc::c_ulong,
                        ) as *mut apriltag_detection_t;
                        let ref mut fresh16 = (*det).family;
                        *fresh16 = family;
                        (*det).id = entry.id as libc::c_int;
                        (*det).hamming = entry.hamming as libc::c_int;
                        (*det).decision_margin = decision_margin;
                        let mut theta: libc::c_double = entry.rotation as libc::c_int
                            as libc::c_double * 3.14159265358979323846f64 / 2.0f64;
                        let mut c: libc::c_double = cos(theta);
                        let mut s: libc::c_double = sin(theta);
                        let mut R: *mut matd_t = matd_create(
                            3 as libc::c_int,
                            3 as libc::c_int,
                        );
                        *((*R).data)
                            .as_mut_ptr()
                            .offset(
                                (0 as libc::c_int as libc::c_uint)
                                    .wrapping_mul((*R).ncols)
                                    .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                            ) = c;
                        *((*R).data)
                            .as_mut_ptr()
                            .offset(
                                (0 as libc::c_int as libc::c_uint)
                                    .wrapping_mul((*R).ncols)
                                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                            ) = -s;
                        *((*R).data)
                            .as_mut_ptr()
                            .offset(
                                (1 as libc::c_int as libc::c_uint)
                                    .wrapping_mul((*R).ncols)
                                    .wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                            ) = s;
                        *((*R).data)
                            .as_mut_ptr()
                            .offset(
                                (1 as libc::c_int as libc::c_uint)
                                    .wrapping_mul((*R).ncols)
                                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                            ) = c;
                        *((*R).data)
                            .as_mut_ptr()
                            .offset(
                                (2 as libc::c_int as libc::c_uint)
                                    .wrapping_mul((*R).ncols)
                                    .wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                            ) = 1 as libc::c_int as libc::c_double;
                        let ref mut fresh17 = (*det).H;
                        *fresh17 = matd_op(
                            b"M*M\0" as *const u8 as *const libc::c_char,
                            (*quad).H,
                            R,
                        );
                        matd_destroy(R);
                        homography_project(
                            (*det).H,
                            0 as libc::c_int as libc::c_double,
                            0 as libc::c_int as libc::c_double,
                            &mut *((*det).c)
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize),
                            &mut *((*det).c)
                                .as_mut_ptr()
                                .offset(1 as libc::c_int as isize),
                        );
                        let mut i: libc::c_int = 0 as libc::c_int;
                        while i < 4 as libc::c_int {
                            let mut tcx: libc::c_int = if i == 1 as libc::c_int
                                || i == 2 as libc::c_int
                            {
                                1 as libc::c_int
                            } else {
                                -(1 as libc::c_int)
                            };
                            let mut tcy: libc::c_int = if i < 2 as libc::c_int {
                                1 as libc::c_int
                            } else {
                                -(1 as libc::c_int)
                            };
                            let mut p: [libc::c_double; 2] = [0.; 2];
                            homography_project(
                                (*det).H,
                                tcx as libc::c_double,
                                tcy as libc::c_double,
                                &mut *p.as_mut_ptr().offset(0 as libc::c_int as isize),
                                &mut *p.as_mut_ptr().offset(1 as libc::c_int as isize),
                            );
                            (*det)
                                .p[i
                                as usize][0 as libc::c_int
                                as usize] = p[0 as libc::c_int as usize];
                            (*det)
                                .p[i
                                as usize][1 as libc::c_int
                                as usize] = p[1 as libc::c_int as usize];
                            i += 1;
                        }
                        pthread_mutex_lock(&mut (*td).mutex);
                        zarray_add(
                            (*task).detections,
                            &mut det as *mut *mut apriltag_detection_t
                                as *const libc::c_void,
                        );
                        pthread_mutex_unlock(&mut (*td).mutex);
                    }
                    quad_destroy(quad);
                }
                famidx += 1;
            }
        }
        quadidx += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn apriltag_detection_destroy(mut det: *mut apriltag_detection_t) {
    if det.is_null() {
        return;
    }
    matd_destroy((*det).H);
    free(det as *mut libc::c_void);
}
unsafe extern "C" fn prefer_smaller(
    mut pref: libc::c_int,
    mut q0: libc::c_double,
    mut q1: libc::c_double,
) -> libc::c_int {
    if pref != 0 {
        return pref;
    }
    if q0 < q1 {
        return -(1 as libc::c_int);
    }
    if q1 < q0 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn apriltag_detector_detect(
    mut td: *mut apriltag_detector_t,
    mut im_orig: *mut image_u8_t,
) -> *mut zarray_t {
    if zarray_size((*td).tag_families) == 0 as libc::c_int {
        let mut s: *mut zarray_t = zarray_create(
            ::std::mem::size_of::<*mut apriltag_detection_t>() as libc::c_ulong,
        );
        fflush(stderr);
        return s;
    }
    if ((*td).wp).is_null() || (*td).nthreads != workerpool_get_nthreads((*td).wp) {
        workerpool_destroy((*td).wp);
        let ref mut fresh18 = (*td).wp;
        *fresh18 = workerpool_create((*td).nthreads);
        if ((*td).wp).is_null() {
            return zarray_create(
                ::std::mem::size_of::<*mut apriltag_detection_t>() as libc::c_ulong,
            );
        }
    }
    timeprofile_clear((*td).tp);
    timeprofile_stamp((*td).tp, b"init\0" as *const u8 as *const libc::c_char);
    let mut quad_im: *mut image_u8_t = im_orig;
    if (*td).quad_decimate > 1 as libc::c_int as libc::c_float {
        quad_im = image_u8_decimate(im_orig, (*td).quad_decimate);
        timeprofile_stamp((*td).tp, b"decimate\0" as *const u8 as *const libc::c_char);
    }
    if (*td).quad_sigma != 0 as libc::c_int as libc::c_float {
        let mut sigma: libc::c_float = fabsf((*td).quad_sigma);
        let mut ksz: libc::c_int = (4 as libc::c_int as libc::c_float * sigma)
            as libc::c_int;
        if ksz & 1 as libc::c_int == 0 as libc::c_int {
            ksz += 1;
        }
        if ksz > 1 as libc::c_int {
            if (*td).quad_sigma > 0 as libc::c_int as libc::c_float {
                image_u8_gaussian_blur(quad_im, sigma as libc::c_double, ksz);
            } else {
                let mut orig: *mut image_u8_t = image_u8_copy(quad_im);
                image_u8_gaussian_blur(quad_im, sigma as libc::c_double, ksz);
                let mut y: libc::c_int = 0 as libc::c_int;
                while y < (*orig).height {
                    let mut x: libc::c_int = 0 as libc::c_int;
                    while x < (*orig).width {
                        let mut vorig: libc::c_int = *((*orig).buf)
                            .offset((y * (*orig).stride + x) as isize) as libc::c_int;
                        let mut vblur: libc::c_int = *((*quad_im).buf)
                            .offset((y * (*quad_im).stride + x) as isize) as libc::c_int;
                        let mut v: libc::c_int = 2 as libc::c_int * vorig - vblur;
                        if v < 0 as libc::c_int {
                            v = 0 as libc::c_int;
                        }
                        if v > 255 as libc::c_int {
                            v = 255 as libc::c_int;
                        }
                        *((*quad_im).buf)
                            .offset((y * (*quad_im).stride + x) as isize) = v as uint8_t;
                        x += 1;
                    }
                    y += 1;
                }
                image_u8_destroy(orig);
            }
        }
    }
    timeprofile_stamp((*td).tp, b"blur/sharp\0" as *const u8 as *const libc::c_char);
    if (*td).debug != 0 {
        image_u8_write_pnm(
            quad_im,
            b"debug_preprocess.pnm\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut quads: *mut zarray_t = apriltag_quad_thresh(td, quad_im);
    if (*td).quad_decimate > 1 as libc::c_int as libc::c_float {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < zarray_size(quads) {
            let mut q: *mut quad = 0 as *mut quad;
            zarray_get_volatile(quads, i, &mut q as *mut *mut quad as *mut libc::c_void);
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < 4 as libc::c_int {
                if (*td).quad_decimate as libc::c_double == 1.5f64 {
                    (*q).p[j as usize][0 as libc::c_int as usize] *= (*td).quad_decimate;
                    (*q).p[j as usize][1 as libc::c_int as usize] *= (*td).quad_decimate;
                } else {
                    (*q)
                        .p[j
                        as usize][0 as libc::c_int
                        as usize] = (((*q).p[j as usize][0 as libc::c_int as usize]
                        as libc::c_double - 0.5f64)
                        * (*td).quad_decimate as libc::c_double + 0.5f64)
                        as libc::c_float;
                    (*q)
                        .p[j
                        as usize][1 as libc::c_int
                        as usize] = (((*q).p[j as usize][1 as libc::c_int as usize]
                        as libc::c_double - 0.5f64)
                        * (*td).quad_decimate as libc::c_double + 0.5f64)
                        as libc::c_float;
                }
                j += 1;
            }
            i += 1;
        }
    }
    if quad_im != im_orig {
        image_u8_destroy(quad_im);
    }
    let mut detections: *mut zarray_t = zarray_create(
        ::std::mem::size_of::<*mut apriltag_detection_t>() as libc::c_ulong,
    );
    (*td).nquads = zarray_size(quads) as uint32_t;
    timeprofile_stamp((*td).tp, b"quads\0" as *const u8 as *const libc::c_char);
    if (*td).debug != 0 {
        let mut im_quads: *mut image_u8_t = image_u8_copy(im_orig);
        image_u8_darken(im_quads);
        image_u8_darken(im_quads);
        srandom(0 as libc::c_int as libc::c_uint);
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < zarray_size(quads) {
            let mut quad: *mut quad = 0 as *mut quad;
            zarray_get_volatile(
                quads,
                i_0,
                &mut quad as *mut *mut quad as *mut libc::c_void,
            );
            let bias: libc::c_int = 100 as libc::c_int;
            let mut color: libc::c_int = (bias as libc::c_long
                + random() % (255 as libc::c_int - bias) as libc::c_long) as libc::c_int;
            image_u8_draw_line(
                im_quads,
                (*quad).p[0 as libc::c_int as usize][0 as libc::c_int as usize],
                (*quad).p[0 as libc::c_int as usize][1 as libc::c_int as usize],
                (*quad).p[1 as libc::c_int as usize][0 as libc::c_int as usize],
                (*quad).p[1 as libc::c_int as usize][1 as libc::c_int as usize],
                color,
                1 as libc::c_int,
            );
            image_u8_draw_line(
                im_quads,
                (*quad).p[1 as libc::c_int as usize][0 as libc::c_int as usize],
                (*quad).p[1 as libc::c_int as usize][1 as libc::c_int as usize],
                (*quad).p[2 as libc::c_int as usize][0 as libc::c_int as usize],
                (*quad).p[2 as libc::c_int as usize][1 as libc::c_int as usize],
                color,
                1 as libc::c_int,
            );
            image_u8_draw_line(
                im_quads,
                (*quad).p[2 as libc::c_int as usize][0 as libc::c_int as usize],
                (*quad).p[2 as libc::c_int as usize][1 as libc::c_int as usize],
                (*quad).p[3 as libc::c_int as usize][0 as libc::c_int as usize],
                (*quad).p[3 as libc::c_int as usize][1 as libc::c_int as usize],
                color,
                1 as libc::c_int,
            );
            image_u8_draw_line(
                im_quads,
                (*quad).p[3 as libc::c_int as usize][0 as libc::c_int as usize],
                (*quad).p[3 as libc::c_int as usize][1 as libc::c_int as usize],
                (*quad).p[0 as libc::c_int as usize][0 as libc::c_int as usize],
                (*quad).p[0 as libc::c_int as usize][1 as libc::c_int as usize],
                color,
                1 as libc::c_int,
            );
            i_0 += 1;
        }
        image_u8_write_pnm(
            im_quads,
            b"debug_quads_raw.pnm\0" as *const u8 as *const libc::c_char,
        );
        image_u8_destroy(im_quads);
    }
    let mut im_samples: *mut image_u8_t = if (*td).debug != 0 {
        image_u8_copy(im_orig)
    } else {
        0 as *mut image_u8_t
    };
    let mut chunksize: libc::c_int = 1 as libc::c_int
        + zarray_size(quads) / (10 as libc::c_int * (*td).nthreads);
    let mut tasks: *mut quad_decode_task = malloc(
        (::std::mem::size_of::<quad_decode_task>() as libc::c_ulong)
            .wrapping_mul(
                (zarray_size(quads) / chunksize + 1 as libc::c_int) as libc::c_ulong,
            ),
    ) as *mut quad_decode_task;
    let mut ntasks: libc::c_int = 0 as libc::c_int;
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < zarray_size(quads) {
        (*tasks.offset(ntasks as isize)).i0 = i_1;
        (*tasks.offset(ntasks as isize)).i1 = imin(zarray_size(quads), i_1 + chunksize);
        let ref mut fresh19 = (*tasks.offset(ntasks as isize)).quads;
        *fresh19 = quads;
        let ref mut fresh20 = (*tasks.offset(ntasks as isize)).td;
        *fresh20 = td;
        let ref mut fresh21 = (*tasks.offset(ntasks as isize)).im;
        *fresh21 = im_orig;
        let ref mut fresh22 = (*tasks.offset(ntasks as isize)).detections;
        *fresh22 = detections;
        let ref mut fresh23 = (*tasks.offset(ntasks as isize)).im_samples;
        *fresh23 = im_samples;
        workerpool_add_task(
            (*td).wp,
            Some(quad_decode_task as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            &mut *tasks.offset(ntasks as isize) as *mut quad_decode_task
                as *mut libc::c_void,
        );
        ntasks += 1;
        i_1 += chunksize;
    }
    workerpool_run((*td).wp);
    free(tasks as *mut libc::c_void);
    if !im_samples.is_null() {
        image_u8_write_pnm(
            im_samples,
            b"debug_samples.pnm\0" as *const u8 as *const libc::c_char,
        );
        image_u8_destroy(im_samples);
    }
    if (*td).debug != 0 {
        let mut im_quads_0: *mut image_u8_t = image_u8_copy(im_orig);
        image_u8_darken(im_quads_0);
        image_u8_darken(im_quads_0);
        srandom(0 as libc::c_int as libc::c_uint);
        let mut i_2: libc::c_int = 0 as libc::c_int;
        while i_2 < zarray_size(quads) {
            let mut quad_0: *mut quad = 0 as *mut quad;
            zarray_get_volatile(
                quads,
                i_2,
                &mut quad_0 as *mut *mut quad as *mut libc::c_void,
            );
            let bias_0: libc::c_int = 100 as libc::c_int;
            let mut color_0: libc::c_int = (bias_0 as libc::c_long
                + random() % (255 as libc::c_int - bias_0) as libc::c_long)
                as libc::c_int;
            image_u8_draw_line(
                im_quads_0,
                (*quad_0).p[0 as libc::c_int as usize][0 as libc::c_int as usize],
                (*quad_0).p[0 as libc::c_int as usize][1 as libc::c_int as usize],
                (*quad_0).p[1 as libc::c_int as usize][0 as libc::c_int as usize],
                (*quad_0).p[1 as libc::c_int as usize][1 as libc::c_int as usize],
                color_0,
                1 as libc::c_int,
            );
            image_u8_draw_line(
                im_quads_0,
                (*quad_0).p[1 as libc::c_int as usize][0 as libc::c_int as usize],
                (*quad_0).p[1 as libc::c_int as usize][1 as libc::c_int as usize],
                (*quad_0).p[2 as libc::c_int as usize][0 as libc::c_int as usize],
                (*quad_0).p[2 as libc::c_int as usize][1 as libc::c_int as usize],
                color_0,
                1 as libc::c_int,
            );
            image_u8_draw_line(
                im_quads_0,
                (*quad_0).p[2 as libc::c_int as usize][0 as libc::c_int as usize],
                (*quad_0).p[2 as libc::c_int as usize][1 as libc::c_int as usize],
                (*quad_0).p[3 as libc::c_int as usize][0 as libc::c_int as usize],
                (*quad_0).p[3 as libc::c_int as usize][1 as libc::c_int as usize],
                color_0,
                1 as libc::c_int,
            );
            image_u8_draw_line(
                im_quads_0,
                (*quad_0).p[3 as libc::c_int as usize][0 as libc::c_int as usize],
                (*quad_0).p[3 as libc::c_int as usize][1 as libc::c_int as usize],
                (*quad_0).p[0 as libc::c_int as usize][0 as libc::c_int as usize],
                (*quad_0).p[0 as libc::c_int as usize][1 as libc::c_int as usize],
                color_0,
                1 as libc::c_int,
            );
            i_2 += 1;
        }
        image_u8_write_pnm(
            im_quads_0,
            b"debug_quads_fixed.pnm\0" as *const u8 as *const libc::c_char,
        );
        image_u8_destroy(im_quads_0);
    }
    timeprofile_stamp(
        (*td).tp,
        b"decode+refinement\0" as *const u8 as *const libc::c_char,
    );
    let mut poly0: *mut zarray_t = g2d_polygon_create_zeros(4 as libc::c_int);
    let mut poly1: *mut zarray_t = g2d_polygon_create_zeros(4 as libc::c_int);
    let mut i0: libc::c_int = 0 as libc::c_int;
    while i0 < zarray_size(detections) {
        let mut det0: *mut apriltag_detection_t = 0 as *mut apriltag_detection_t;
        zarray_get(
            detections,
            i0,
            &mut det0 as *mut *mut apriltag_detection_t as *mut libc::c_void,
        );
        let mut k: libc::c_int = 0 as libc::c_int;
        while k < 4 as libc::c_int {
            zarray_set(
                poly0,
                k,
                ((*det0).p[k as usize]).as_mut_ptr() as *const libc::c_void,
                0 as *mut libc::c_void,
            );
            k += 1;
        }
        let mut i1: libc::c_int = i0 + 1 as libc::c_int;
        while i1 < zarray_size(detections) {
            let mut det1: *mut apriltag_detection_t = 0 as *mut apriltag_detection_t;
            zarray_get(
                detections,
                i1,
                &mut det1 as *mut *mut apriltag_detection_t as *mut libc::c_void,
            );
            if !((*det0).id != (*det1).id || (*det0).family != (*det1).family) {
                let mut k_0: libc::c_int = 0 as libc::c_int;
                while k_0 < 4 as libc::c_int {
                    zarray_set(
                        poly1,
                        k_0,
                        ((*det1).p[k_0 as usize]).as_mut_ptr() as *const libc::c_void,
                        0 as *mut libc::c_void,
                    );
                    k_0 += 1;
                }
                if g2d_polygon_overlaps_polygon(poly0, poly1) != 0 {
                    let mut pref: libc::c_int = 0 as libc::c_int;
                    pref = prefer_smaller(
                        pref,
                        (*det0).hamming as libc::c_double,
                        (*det1).hamming as libc::c_double,
                    );
                    pref = prefer_smaller(
                        pref,
                        -(*det0).decision_margin as libc::c_double,
                        -(*det1).decision_margin as libc::c_double,
                    );
                    let mut i_3: libc::c_int = 0 as libc::c_int;
                    while i_3 < 4 as libc::c_int {
                        pref = prefer_smaller(
                            pref,
                            (*det0).p[i_3 as usize][0 as libc::c_int as usize],
                            (*det1).p[i_3 as usize][0 as libc::c_int as usize],
                        );
                        pref = prefer_smaller(
                            pref,
                            (*det0).p[i_3 as usize][1 as libc::c_int as usize],
                            (*det1).p[i_3 as usize][1 as libc::c_int as usize],
                        );
                        i_3 += 1;
                    }
                    if pref == 0 as libc::c_int {
                        fflush(stderr);
                    }
                    if pref < 0 as libc::c_int {
                        apriltag_detection_destroy(det1);
                        zarray_remove_index(detections, i1, 1 as libc::c_int);
                        i1 -= 1;
                    } else {
                        apriltag_detection_destroy(det0);
                        zarray_remove_index(detections, i0, 1 as libc::c_int);
                        i0 -= 1;
                        break;
                    }
                }
            }
            i1 += 1;
        }
        i0 += 1;
    }
    zarray_destroy(poly0);
    zarray_destroy(poly1);
    timeprofile_stamp((*td).tp, b"reconcile\0" as *const u8 as *const libc::c_char);
    if (*td).debug != 0 {
        let mut darker: *mut image_u8_t = image_u8_copy(im_orig);
        image_u8_darken(darker);
        image_u8_darken(darker);
        let mut f: *mut FILE = fopen(
            b"debug_output.ps\0" as *const u8 as *const libc::c_char,
            b"w\0" as *const u8 as *const libc::c_char,
        );
        fprintf(f, b"%%!PS\n\n\0" as *const u8 as *const libc::c_char);
        let mut scale: libc::c_double = fmin(
            612.0f64 / (*darker).width as libc::c_double,
            792.0f64 / (*darker).height as libc::c_double,
        );
        fprintf(f, b"%f %f scale\n\0" as *const u8 as *const libc::c_char, scale, scale);
        fprintf(
            f,
            b"0 %d translate\n\0" as *const u8 as *const libc::c_char,
            (*darker).height,
        );
        fprintf(f, b"1 -1 scale\n\0" as *const u8 as *const libc::c_char);
        postscript_image(f, darker);
        image_u8_destroy(darker);
        let mut i_4: libc::c_int = 0 as libc::c_int;
        while i_4 < zarray_size(detections) {
            let mut det: *mut apriltag_detection_t = 0 as *mut apriltag_detection_t;
            zarray_get(
                detections,
                i_4,
                &mut det as *mut *mut apriltag_detection_t as *mut libc::c_void,
            );
            let mut rgb: [libc::c_float; 3] = [0.; 3];
            let mut bias_1: libc::c_int = 100 as libc::c_int;
            let mut j_0: libc::c_int = 0 as libc::c_int;
            while j_0 < 3 as libc::c_int {
                rgb[j_0
                    as usize] = (bias_1 as libc::c_long
                    + random() % (255 as libc::c_int - bias_1) as libc::c_long)
                    as libc::c_float;
                j_0 += 1;
            }
            fprintf(
                f,
                b"%f %f %f setrgbcolor\n\0" as *const u8 as *const libc::c_char,
                (rgb[0 as libc::c_int as usize] / 255.0f32) as libc::c_double,
                (rgb[1 as libc::c_int as usize] / 255.0f32) as libc::c_double,
                (rgb[2 as libc::c_int as usize] / 255.0f32) as libc::c_double,
            );
            fprintf(
                f,
                b"%f %f moveto %f %f lineto %f %f lineto %f %f lineto %f %f lineto stroke\n\0"
                    as *const u8 as *const libc::c_char,
                (*det).p[0 as libc::c_int as usize][0 as libc::c_int as usize],
                (*det).p[0 as libc::c_int as usize][1 as libc::c_int as usize],
                (*det).p[1 as libc::c_int as usize][0 as libc::c_int as usize],
                (*det).p[1 as libc::c_int as usize][1 as libc::c_int as usize],
                (*det).p[2 as libc::c_int as usize][0 as libc::c_int as usize],
                (*det).p[2 as libc::c_int as usize][1 as libc::c_int as usize],
                (*det).p[3 as libc::c_int as usize][0 as libc::c_int as usize],
                (*det).p[3 as libc::c_int as usize][1 as libc::c_int as usize],
                (*det).p[0 as libc::c_int as usize][0 as libc::c_int as usize],
                (*det).p[0 as libc::c_int as usize][1 as libc::c_int as usize],
            );
            i_4 += 1;
        }
        fprintf(f, b"showpage\n\0" as *const u8 as *const libc::c_char);
        fclose(f);
    }
    if (*td).debug != 0 {
        let mut darker_0: *mut image_u8_t = image_u8_copy(im_orig);
        image_u8_darken(darker_0);
        image_u8_darken(darker_0);
        let mut out: *mut image_u8x3_t = image_u8x3_create(
            (*darker_0).width as libc::c_uint,
            (*darker_0).height as libc::c_uint,
        );
        let mut y_0: libc::c_int = 0 as libc::c_int;
        while y_0 < (*im_orig).height {
            let mut x_0: libc::c_int = 0 as libc::c_int;
            while x_0 < (*im_orig).width {
                *((*out).buf)
                    .offset(
                        (y_0 * (*out).stride + 3 as libc::c_int * x_0 + 0 as libc::c_int)
                            as isize,
                    ) = *((*darker_0).buf)
                    .offset((y_0 * (*darker_0).stride + x_0) as isize);
                *((*out).buf)
                    .offset(
                        (y_0 * (*out).stride + 3 as libc::c_int * x_0 + 1 as libc::c_int)
                            as isize,
                    ) = *((*darker_0).buf)
                    .offset((y_0 * (*darker_0).stride + x_0) as isize);
                *((*out).buf)
                    .offset(
                        (y_0 * (*out).stride + 3 as libc::c_int * x_0 + 2 as libc::c_int)
                            as isize,
                    ) = *((*darker_0).buf)
                    .offset((y_0 * (*darker_0).stride + x_0) as isize);
                x_0 += 1;
            }
            y_0 += 1;
        }
        image_u8_destroy(darker_0);
        let mut i_5: libc::c_int = 0 as libc::c_int;
        while i_5 < zarray_size(detections) {
            let mut det_0: *mut apriltag_detection_t = 0 as *mut apriltag_detection_t;
            zarray_get(
                detections,
                i_5,
                &mut det_0 as *mut *mut apriltag_detection_t as *mut libc::c_void,
            );
            let mut rgb_0: [libc::c_float; 3] = [0.; 3];
            let mut bias_2: libc::c_int = 100 as libc::c_int;
            let mut j_1: libc::c_int = 0 as libc::c_int;
            while j_1 < 3 as libc::c_int {
                rgb_0[j_1
                    as usize] = (bias_2 as libc::c_long
                    + random() % (255 as libc::c_int - bias_2) as libc::c_long)
                    as libc::c_float;
                j_1 += 1;
            }
            let mut j_2: libc::c_int = 0 as libc::c_int;
            while j_2 < 4 as libc::c_int {
                let mut k_1: libc::c_int = j_2 + 1 as libc::c_int & 3 as libc::c_int;
                image_u8x3_draw_line(
                    out,
                    (*det_0).p[j_2 as usize][0 as libc::c_int as usize] as libc::c_float,
                    (*det_0).p[j_2 as usize][1 as libc::c_int as usize] as libc::c_float,
                    (*det_0).p[k_1 as usize][0 as libc::c_int as usize] as libc::c_float,
                    (*det_0).p[k_1 as usize][1 as libc::c_int as usize] as libc::c_float,
                    [
                        rgb_0[0 as libc::c_int as usize] as uint8_t,
                        rgb_0[1 as libc::c_int as usize] as uint8_t,
                        rgb_0[2 as libc::c_int as usize] as uint8_t,
                    ]
                        .as_mut_ptr(),
                    1 as libc::c_int,
                );
                j_2 += 1;
            }
            i_5 += 1;
        }
        image_u8x3_write_pnm(
            out,
            b"debug_output.pnm\0" as *const u8 as *const libc::c_char,
        );
        image_u8x3_destroy(out);
    }
    if (*td).debug != 0 {
        let mut f_0: *mut FILE = fopen(
            b"debug_quads.ps\0" as *const u8 as *const libc::c_char,
            b"w\0" as *const u8 as *const libc::c_char,
        );
        fprintf(f_0, b"%%!PS\n\n\0" as *const u8 as *const libc::c_char);
        let mut darker_1: *mut image_u8_t = image_u8_copy(im_orig);
        image_u8_darken(darker_1);
        image_u8_darken(darker_1);
        let mut scale_0: libc::c_double = fmin(
            612.0f64 / (*darker_1).width as libc::c_double,
            792.0f64 / (*darker_1).height as libc::c_double,
        );
        fprintf(
            f_0,
            b"%f %f scale\n\0" as *const u8 as *const libc::c_char,
            scale_0,
            scale_0,
        );
        fprintf(
            f_0,
            b"0 %d translate\n\0" as *const u8 as *const libc::c_char,
            (*darker_1).height,
        );
        fprintf(f_0, b"1 -1 scale\n\0" as *const u8 as *const libc::c_char);
        postscript_image(f_0, darker_1);
        image_u8_destroy(darker_1);
        let mut i_6: libc::c_int = 0 as libc::c_int;
        while i_6 < zarray_size(quads) {
            let mut q_0: *mut quad = 0 as *mut quad;
            zarray_get_volatile(
                quads,
                i_6,
                &mut q_0 as *mut *mut quad as *mut libc::c_void,
            );
            let mut rgb_1: [libc::c_float; 3] = [0.; 3];
            let mut bias_3: libc::c_int = 100 as libc::c_int;
            let mut j_3: libc::c_int = 0 as libc::c_int;
            while j_3 < 3 as libc::c_int {
                rgb_1[j_3
                    as usize] = (bias_3 as libc::c_long
                    + random() % (255 as libc::c_int - bias_3) as libc::c_long)
                    as libc::c_float;
                j_3 += 1;
            }
            fprintf(
                f_0,
                b"%f %f %f setrgbcolor\n\0" as *const u8 as *const libc::c_char,
                (rgb_1[0 as libc::c_int as usize] / 255.0f32) as libc::c_double,
                (rgb_1[1 as libc::c_int as usize] / 255.0f32) as libc::c_double,
                (rgb_1[2 as libc::c_int as usize] / 255.0f32) as libc::c_double,
            );
            fprintf(
                f_0,
                b"%f %f moveto %f %f lineto %f %f lineto %f %f lineto %f %f lineto stroke\n\0"
                    as *const u8 as *const libc::c_char,
                (*q_0).p[0 as libc::c_int as usize][0 as libc::c_int as usize]
                    as libc::c_double,
                (*q_0).p[0 as libc::c_int as usize][1 as libc::c_int as usize]
                    as libc::c_double,
                (*q_0).p[1 as libc::c_int as usize][0 as libc::c_int as usize]
                    as libc::c_double,
                (*q_0).p[1 as libc::c_int as usize][1 as libc::c_int as usize]
                    as libc::c_double,
                (*q_0).p[2 as libc::c_int as usize][0 as libc::c_int as usize]
                    as libc::c_double,
                (*q_0).p[2 as libc::c_int as usize][1 as libc::c_int as usize]
                    as libc::c_double,
                (*q_0).p[3 as libc::c_int as usize][0 as libc::c_int as usize]
                    as libc::c_double,
                (*q_0).p[3 as libc::c_int as usize][1 as libc::c_int as usize]
                    as libc::c_double,
                (*q_0).p[0 as libc::c_int as usize][0 as libc::c_int as usize]
                    as libc::c_double,
                (*q_0).p[0 as libc::c_int as usize][1 as libc::c_int as usize]
                    as libc::c_double,
            );
            i_6 += 1;
        }
        fprintf(f_0, b"showpage\n\0" as *const u8 as *const libc::c_char);
        fclose(f_0);
    }
    timeprofile_stamp((*td).tp, b"debug output\0" as *const u8 as *const libc::c_char);
    let mut i_7: libc::c_int = 0 as libc::c_int;
    while i_7 < zarray_size(quads) {
        let mut quad_1: *mut quad = 0 as *mut quad;
        zarray_get_volatile(
            quads,
            i_7,
            &mut quad_1 as *mut *mut quad as *mut libc::c_void,
        );
        matd_destroy((*quad_1).H);
        matd_destroy((*quad_1).Hinv);
        i_7 += 1;
    }
    zarray_destroy(quads);
    zarray_sort(
        detections,
        Some(
            detection_compare_function
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    timeprofile_stamp((*td).tp, b"cleanup\0" as *const u8 as *const libc::c_char);
    return detections;
}
#[no_mangle]
pub unsafe extern "C" fn apriltag_detections_destroy(mut detections: *mut zarray_t) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < zarray_size(detections) {
        let mut det: *mut apriltag_detection_t = 0 as *mut apriltag_detection_t;
        zarray_get(
            detections,
            i,
            &mut det as *mut *mut apriltag_detection_t as *mut libc::c_void,
        );
        apriltag_detection_destroy(det);
        i += 1;
    }
    zarray_destroy(detections);
}
#[no_mangle]
pub unsafe extern "C" fn apriltag_to_image(
    mut fam: *mut apriltag_family_t,
    mut idx: libc::c_int,
) -> *mut image_u8_t {
    let mut code: uint64_t = *((*fam).codes).offset(idx as isize);
    let mut im: *mut image_u8_t = image_u8_create(
        (*fam).total_width as libc::c_uint,
        (*fam).total_width as libc::c_uint,
    );
    let mut white_border_width: libc::c_int = (*fam).width_at_border
        + (if (*fam).reversed_border as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            2 as libc::c_int
        });
    let mut white_border_start: libc::c_int = ((*fam).total_width - white_border_width)
        / 2 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < white_border_width - 1 as libc::c_int {
        *((*im).buf)
            .offset(
                (white_border_start * (*im).stride + white_border_start + i) as isize,
            ) = 255 as libc::c_int as uint8_t;
        *((*im).buf)
            .offset(
                ((white_border_start + i) * (*im).stride + (*fam).total_width
                    - 1 as libc::c_int - white_border_start) as isize,
            ) = 255 as libc::c_int as uint8_t;
        *((*im).buf)
            .offset(
                (((*fam).total_width - 1 as libc::c_int - white_border_start)
                    * (*im).stride + white_border_start + i + 1 as libc::c_int) as isize,
            ) = 255 as libc::c_int as uint8_t;
        *((*im).buf)
            .offset(
                ((white_border_start + 1 as libc::c_int + i) * (*im).stride
                    + white_border_start) as isize,
            ) = 255 as libc::c_int as uint8_t;
        i += 1 as libc::c_int;
    }
    let mut border_start: libc::c_int = ((*fam).total_width - (*fam).width_at_border)
        / 2 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while (i_0 as libc::c_uint) < (*fam).nbits {
        if code
            & (1 as libc::c_int as uint64_t)
                << ((*fam).nbits)
                    .wrapping_sub(i_0 as libc::c_uint)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) != 0
        {
            *((*im).buf)
                .offset(
                    (*((*fam).bit_y).offset(i_0 as isize))
                        .wrapping_add(border_start as libc::c_uint)
                        .wrapping_mul((*im).stride as libc::c_uint)
                        .wrapping_add(*((*fam).bit_x).offset(i_0 as isize))
                        .wrapping_add(border_start as libc::c_uint) as isize,
                ) = 255 as libc::c_int as uint8_t;
        }
        i_0 += 1;
    }
    return im;
}
