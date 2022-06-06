use ::libc;
extern "C" {
    pub type workerpool;
    pub type getopt;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn log(_: libc::c_double) -> libc::c_double;
    fn __errno_location() -> *mut libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn image_u8_create(width: libc::c_uint, height: libc::c_uint) -> *mut image_u8_t;
    fn image_u8_create_from_pnm(path: *const libc::c_char) -> *mut image_u8_t;
    fn image_u8_destroy(im: *mut image_u8_t);
    fn image_u8_write_pnm(
        im: *const image_u8_t,
        path: *const libc::c_char,
    ) -> libc::c_int;
    fn apriltag_detector_create() -> *mut apriltag_detector_t;
    fn apriltag_detector_add_family_bits(
        td: *mut apriltag_detector_t,
        fam: *mut apriltag_family_t,
        bits_corrected: libc::c_int,
    );
    fn apriltag_detector_destroy(td: *mut apriltag_detector_t);
    fn apriltag_detector_detect(
        td: *mut apriltag_detector_t,
        im_orig: *mut image_u8_t,
    ) -> *mut zarray_t;
    fn apriltag_detections_destroy(detections: *mut zarray_t);
    fn tag36h11_create() -> *mut apriltag_family_t;
    fn tag36h11_destroy(tf: *mut apriltag_family_t);
    fn tag25h9_create() -> *mut apriltag_family_t;
    fn tag25h9_destroy(tf: *mut apriltag_family_t);
    fn tag16h5_create() -> *mut apriltag_family_t;
    fn tag16h5_destroy(tf: *mut apriltag_family_t);
    fn tagCircle21h7_create() -> *mut apriltag_family_t;
    fn tagCircle21h7_destroy(tf: *mut apriltag_family_t);
    fn tagCircle49h12_create() -> *mut apriltag_family_t;
    fn tagCircle49h12_destroy(tf: *mut apriltag_family_t);
    fn tagCustom48h12_create() -> *mut apriltag_family_t;
    fn tagCustom48h12_destroy(tf: *mut apriltag_family_t);
    fn tagStandard41h12_create() -> *mut apriltag_family_t;
    fn tagStandard41h12_destroy(tf: *mut apriltag_family_t);
    fn tagStandard52h13_create() -> *mut apriltag_family_t;
    fn tagStandard52h13_destroy(tf: *mut apriltag_family_t);
    fn str_ends_with(haystack: *const libc::c_char, needle: *const libc::c_char) -> bool;
    fn getopt_destroy(gopt: *mut getopt_t);
    fn getopt_create() -> *mut getopt_t;
    fn getopt_parse(
        gopt: *mut getopt_t,
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        showErrors: libc::c_int,
    ) -> libc::c_int;
    fn getopt_do_usage(gopt: *mut getopt_t);
    fn getopt_add_bool(
        gopt: *mut getopt_t,
        sopt: libc::c_char,
        lname: *const libc::c_char,
        def: libc::c_int,
        help: *const libc::c_char,
    );
    fn getopt_add_int(
        gopt: *mut getopt_t,
        sopt: libc::c_char,
        lname: *const libc::c_char,
        def: *const libc::c_char,
        help: *const libc::c_char,
    );
    fn getopt_add_string(
        gopt: *mut getopt_t,
        sopt: libc::c_char,
        lname: *const libc::c_char,
        def: *const libc::c_char,
        help: *const libc::c_char,
    );
    fn getopt_add_double(
        gopt: *mut getopt_t,
        sopt: libc::c_char,
        lname: *const libc::c_char,
        def: *const libc::c_char,
        help: *const libc::c_char,
    );
    fn getopt_get_string(
        gopt: *mut getopt_t,
        lname: *const libc::c_char,
    ) -> *const libc::c_char;
    fn getopt_get_int(getopt: *mut getopt_t, lname: *const libc::c_char) -> libc::c_int;
    fn getopt_get_bool(getopt: *mut getopt_t, lname: *const libc::c_char) -> libc::c_int;
    fn getopt_get_double(
        getopt: *mut getopt_t,
        lname: *const libc::c_char,
    ) -> libc::c_double;
    fn getopt_get_extra_args(gopt: *mut getopt_t) -> *const zarray_t;
    fn pjpeg_create_from_file(
        path: *const libc::c_char,
        flags: uint32_t,
        error: *mut libc::c_int,
    ) -> *mut pjpeg_t;
    fn pjpeg_destroy(pj: *mut pjpeg_t);
    fn pjpeg_to_u8_baseline(pj: *mut pjpeg_t) -> *mut image_u8_t;
    fn pjpeg_to_u8x3_baseline(pj: *mut pjpeg_t) -> *mut image_u8x3_t;
    fn image_u8x3_destroy(im: *mut image_u8x3_t);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct matd_t {
    pub nrows: libc::c_uint,
    pub ncols: libc::c_uint,
    pub data: [libc::c_double; 0],
}
pub type uint8_t = __uint8_t;
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
pub type getopt_t = getopt;
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
#[inline]
unsafe extern "C" fn zarray_size(mut za: *const zarray_t) -> libc::c_int {
    return (*za).size;
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
    let ref mut fresh0 = *(p as *mut *mut libc::c_void);
    *fresh0 = &mut *((*za).data)
        .offset((idx as libc::c_ulong).wrapping_mul((*za).el_sz) as isize)
        as *mut libc::c_char as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn timeprofile_display(mut tp: *mut timeprofile_t) {
    let mut lastutime: int64_t = (*tp).utime;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < zarray_size((*tp).stamps) {
        let mut stamp: *mut timeprofile_entry = 0 as *mut timeprofile_entry;
        zarray_get_volatile(
            (*tp).stamps,
            i,
            &mut stamp as *mut *mut timeprofile_entry as *mut libc::c_void,
        );
        let mut cumtime: libc::c_double = ((*stamp).utime - (*tp).utime)
            as libc::c_double / 1000000.0f64;
        let mut parttime: libc::c_double = ((*stamp).utime - lastutime) as libc::c_double
            / 1000000.0f64;
        printf(
            b"%2d %32s %15f ms %15f ms\n\0" as *const u8 as *const libc::c_char,
            i,
            ((*stamp).name).as_mut_ptr(),
            parttime * 1000 as libc::c_int as libc::c_double,
            cumtime * 1000 as libc::c_int as libc::c_double,
        );
        lastutime = (*stamp).utime;
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn timeprofile_total_utime(mut tp: *mut timeprofile_t) -> uint64_t {
    if zarray_size((*tp).stamps) == 0 as libc::c_int {
        return 0 as libc::c_int as uint64_t;
    }
    let mut first: *mut timeprofile_entry = 0 as *mut timeprofile_entry;
    let mut last: *mut timeprofile_entry = 0 as *mut timeprofile_entry;
    zarray_get_volatile(
        (*tp).stamps,
        0 as libc::c_int,
        &mut first as *mut *mut timeprofile_entry as *mut libc::c_void,
    );
    zarray_get_volatile(
        (*tp).stamps,
        zarray_size((*tp).stamps) - 1 as libc::c_int,
        &mut last as *mut *mut timeprofile_entry as *mut libc::c_void,
    );
    return ((*last).utime - (*first).utime) as uint64_t;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut getopt: *mut getopt_t = getopt_create();
    getopt_add_bool(
        getopt,
        'h' as i32 as libc::c_char,
        b"help\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        b"Show this help\0" as *const u8 as *const libc::c_char,
    );
    getopt_add_bool(
        getopt,
        'd' as i32 as libc::c_char,
        b"debug\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        b"Enable debugging output (slow)\0" as *const u8 as *const libc::c_char,
    );
    getopt_add_bool(
        getopt,
        'q' as i32 as libc::c_char,
        b"quiet\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        b"Reduce output\0" as *const u8 as *const libc::c_char,
    );
    getopt_add_string(
        getopt,
        'f' as i32 as libc::c_char,
        b"family\0" as *const u8 as *const libc::c_char,
        b"tag36h11\0" as *const u8 as *const libc::c_char,
        b"Tag family to use\0" as *const u8 as *const libc::c_char,
    );
    getopt_add_int(
        getopt,
        'i' as i32 as libc::c_char,
        b"iters\0" as *const u8 as *const libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char,
        b"Repeat processing on input set this many times\0" as *const u8
            as *const libc::c_char,
    );
    getopt_add_int(
        getopt,
        't' as i32 as libc::c_char,
        b"threads\0" as *const u8 as *const libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char,
        b"Use this many CPU threads\0" as *const u8 as *const libc::c_char,
    );
    getopt_add_int(
        getopt,
        'a' as i32 as libc::c_char,
        b"hamming\0" as *const u8 as *const libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char,
        b"Detect tags with up to this many bit errors.\0" as *const u8
            as *const libc::c_char,
    );
    getopt_add_double(
        getopt,
        'x' as i32 as libc::c_char,
        b"decimate\0" as *const u8 as *const libc::c_char,
        b"2.0\0" as *const u8 as *const libc::c_char,
        b"Decimate input image by this factor\0" as *const u8 as *const libc::c_char,
    );
    getopt_add_double(
        getopt,
        'b' as i32 as libc::c_char,
        b"blur\0" as *const u8 as *const libc::c_char,
        b"0.0\0" as *const u8 as *const libc::c_char,
        b"Apply low-pass blur to input; negative sharpens\0" as *const u8
            as *const libc::c_char,
    );
    getopt_add_bool(
        getopt,
        '0' as i32 as libc::c_char,
        b"refine-edges\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        b"Spend more time trying to align edges of tags\0" as *const u8
            as *const libc::c_char,
    );
    if getopt_parse(getopt, argc, argv, 1 as libc::c_int) == 0
        || getopt_get_bool(getopt, b"help\0" as *const u8 as *const libc::c_char) != 0
    {
        printf(
            b"Usage: %s [options] <input files>\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        getopt_do_usage(getopt);
        exit(0 as libc::c_int);
    }
    let mut inputs: *const zarray_t = getopt_get_extra_args(getopt);
    let mut tf: *mut apriltag_family_t = 0 as *mut apriltag_family_t;
    let mut famname: *const libc::c_char = getopt_get_string(
        getopt,
        b"family\0" as *const u8 as *const libc::c_char,
    );
    if strcmp(famname, b"tag36h11\0" as *const u8 as *const libc::c_char) == 0 {
        tf = tag36h11_create();
    } else if strcmp(famname, b"tag25h9\0" as *const u8 as *const libc::c_char) == 0 {
        tf = tag25h9_create();
    } else if strcmp(famname, b"tag16h5\0" as *const u8 as *const libc::c_char) == 0 {
        tf = tag16h5_create();
    } else if strcmp(famname, b"tagCircle21h7\0" as *const u8 as *const libc::c_char)
            == 0
        {
        tf = tagCircle21h7_create();
    } else if strcmp(famname, b"tagCircle49h12\0" as *const u8 as *const libc::c_char)
            == 0
        {
        tf = tagCircle49h12_create();
    } else if strcmp(famname, b"tagStandard41h12\0" as *const u8 as *const libc::c_char)
            == 0
        {
        tf = tagStandard41h12_create();
    } else if strcmp(famname, b"tagStandard52h13\0" as *const u8 as *const libc::c_char)
            == 0
        {
        tf = tagStandard52h13_create();
    } else if strcmp(famname, b"tagCustom48h12\0" as *const u8 as *const libc::c_char)
            == 0
        {
        tf = tagCustom48h12_create();
    } else {
        printf(
            b"Unrecognized tag family name. Use e.g. \"tag36h11\".\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    let mut td: *mut apriltag_detector_t = apriltag_detector_create();
    apriltag_detector_add_family_bits(
        td,
        tf,
        getopt_get_int(getopt, b"hamming\0" as *const u8 as *const libc::c_char),
    );
    match *__errno_location() {
        22 => {
            printf(
                b"\"hamming\" parameter is out-of-range.\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(-(1 as libc::c_int));
        }
        12 => {
            printf(
                b"Unable to add family to detector due to insufficient memory to allocate the tag-family decoder. Try reducing \"hamming\" from %d or choose an alternative tag family.\n\0"
                    as *const u8 as *const libc::c_char,
                getopt_get_int(getopt, b"hamming\0" as *const u8 as *const libc::c_char),
            );
            exit(-(1 as libc::c_int));
        }
        _ => {}
    }
    (*td)
        .quad_decimate = getopt_get_double(
        getopt,
        b"decimate\0" as *const u8 as *const libc::c_char,
    ) as libc::c_float;
    (*td)
        .quad_sigma = getopt_get_double(
        getopt,
        b"blur\0" as *const u8 as *const libc::c_char,
    ) as libc::c_float;
    (*td)
        .nthreads = getopt_get_int(
        getopt,
        b"threads\0" as *const u8 as *const libc::c_char,
    );
    (*td)
        .debug = getopt_get_bool(getopt, b"debug\0" as *const u8 as *const libc::c_char);
    (*td)
        .refine_edges = getopt_get_bool(
        getopt,
        b"refine-edges\0" as *const u8 as *const libc::c_char,
    );
    let mut quiet: libc::c_int = getopt_get_bool(
        getopt,
        b"quiet\0" as *const u8 as *const libc::c_char,
    );
    let mut maxiters: libc::c_int = getopt_get_int(
        getopt,
        b"iters\0" as *const u8 as *const libc::c_char,
    );
    let hamm_hist_max: libc::c_int = 10 as libc::c_int;
    let mut iter: libc::c_int = 0 as libc::c_int;
    while iter < maxiters {
        let mut total_quads: libc::c_int = 0 as libc::c_int;
        let mut total_hamm_hist: [libc::c_int; 10] = [0; 10];
        memset(
            total_hamm_hist.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[libc::c_int; 10]>() as libc::c_ulong,
        );
        let mut total_time: libc::c_double = 0 as libc::c_int as libc::c_double;
        if maxiters > 1 as libc::c_int {
            printf(
                b"iter %d / %d\n\0" as *const u8 as *const libc::c_char,
                iter + 1 as libc::c_int,
                maxiters,
            );
        }
        let mut current_block_108: u64;
        let mut input: libc::c_int = 0 as libc::c_int;
        while input < zarray_size(inputs) {
            let mut hamm_hist: [libc::c_int; 10] = [0; 10];
            memset(
                hamm_hist.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<[libc::c_int; 10]>() as libc::c_ulong,
            );
            let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
            zarray_get(
                inputs,
                input,
                &mut path as *mut *mut libc::c_char as *mut libc::c_void,
            );
            if quiet == 0 {
                printf(b"loading %s\n\0" as *const u8 as *const libc::c_char, path);
            } else {
                printf(b"%20s \0" as *const u8 as *const libc::c_char, path);
            }
            let mut im: *mut image_u8_t = 0 as *mut image_u8_t;
            if str_ends_with(path, b"pnm\0" as *const u8 as *const libc::c_char)
                as libc::c_int != 0
                || str_ends_with(path, b"PNM\0" as *const u8 as *const libc::c_char)
                    as libc::c_int != 0
                || str_ends_with(path, b"pgm\0" as *const u8 as *const libc::c_char)
                    as libc::c_int != 0
                || str_ends_with(path, b"PGM\0" as *const u8 as *const libc::c_char)
                    as libc::c_int != 0
            {
                im = image_u8_create_from_pnm(path);
                current_block_108 = 2467484839200770573;
            } else if str_ends_with(path, b"jpg\0" as *const u8 as *const libc::c_char)
                    as libc::c_int != 0
                    || str_ends_with(path, b"JPG\0" as *const u8 as *const libc::c_char)
                        as libc::c_int != 0
                {
                let mut err: libc::c_int = 0 as libc::c_int;
                let mut pjpeg: *mut pjpeg_t = pjpeg_create_from_file(
                    path,
                    0 as libc::c_int as uint32_t,
                    &mut err,
                );
                if pjpeg.is_null() {
                    printf(
                        b"pjpeg failed to load: %s, error %d\n\0" as *const u8
                            as *const libc::c_char,
                        path,
                        err,
                    );
                    current_block_108 = 6560072651652764009;
                } else {
                    im = pjpeg_to_u8_baseline(pjpeg);
                    pjpeg_destroy(pjpeg);
                    current_block_108 = 2467484839200770573;
                }
            } else {
                current_block_108 = 2467484839200770573;
            }
            match current_block_108 {
                2467484839200770573 => {
                    if im.is_null() {
                        printf(
                            b"couldn't load %s\n\0" as *const u8 as *const libc::c_char,
                            path,
                        );
                    } else {
                        printf(
                            b"image: %s %dx%d\n\0" as *const u8 as *const libc::c_char,
                            path,
                            (*im).width,
                            (*im).height,
                        );
                        let mut detections: *mut zarray_t = apriltag_detector_detect(
                            td,
                            im,
                        );
                        if *__errno_location() == 11 as libc::c_int {
                            printf(
                                b"Unable to create the %d threads requested.\n\0"
                                    as *const u8 as *const libc::c_char,
                                (*td).nthreads,
                            );
                            exit(-(1 as libc::c_int));
                        }
                        let mut i: libc::c_int = 0 as libc::c_int;
                        while i < zarray_size(detections) {
                            let mut det: *mut apriltag_detection_t = 0
                                as *mut apriltag_detection_t;
                            zarray_get(
                                detections,
                                i,
                                &mut det as *mut *mut apriltag_detection_t
                                    as *mut libc::c_void,
                            );
                            if quiet == 0 {
                                printf(
                                    b"detection %3d: id (%2dx%2d)-%-4d, hamming %d, margin %8.3f\n\0"
                                        as *const u8 as *const libc::c_char,
                                    i,
                                    (*(*det).family).nbits,
                                    (*(*det).family).h,
                                    (*det).id,
                                    (*det).hamming,
                                    (*det).decision_margin as libc::c_double,
                                );
                            }
                            hamm_hist[(*det).hamming as usize] += 1;
                            total_hamm_hist[(*det).hamming as usize] += 1;
                            i += 1;
                        }
                        apriltag_detections_destroy(detections);
                        if quiet == 0 {
                            timeprofile_display((*td).tp);
                        }
                        total_quads = (total_quads as libc::c_uint)
                            .wrapping_add((*td).nquads) as libc::c_int as libc::c_int;
                        if quiet == 0 {
                            printf(b"hamm \0" as *const u8 as *const libc::c_char);
                        }
                        let mut i_0: libc::c_int = 0 as libc::c_int;
                        while i_0 < hamm_hist_max {
                            printf(
                                b"%5d \0" as *const u8 as *const libc::c_char,
                                hamm_hist[i_0 as usize],
                            );
                            i_0 += 1;
                        }
                        let mut t: libc::c_double = timeprofile_total_utime((*td).tp)
                            as libc::c_double / 1.0E3f64;
                        total_time += t;
                        printf(b"%12.3f \0" as *const u8 as *const libc::c_char, t);
                        printf(
                            b"%5d\0" as *const u8 as *const libc::c_char,
                            (*td).nquads,
                        );
                        printf(b"\n\0" as *const u8 as *const libc::c_char);
                        image_u8_destroy(im);
                    }
                }
                _ => {}
            }
            input += 1;
        }
        printf(b"Summary\n\0" as *const u8 as *const libc::c_char);
        printf(b"hamm \0" as *const u8 as *const libc::c_char);
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < hamm_hist_max {
            printf(
                b"%5d \0" as *const u8 as *const libc::c_char,
                total_hamm_hist[i_1 as usize],
            );
            i_1 += 1;
        }
        printf(b"%12.3f \0" as *const u8 as *const libc::c_char, total_time);
        printf(b"%5d\0" as *const u8 as *const libc::c_char, total_quads);
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        iter += 1;
    }
    apriltag_detector_destroy(td);
    if strcmp(famname, b"tag36h11\0" as *const u8 as *const libc::c_char) == 0 {
        tag36h11_destroy(tf);
    } else if strcmp(famname, b"tag25h9\0" as *const u8 as *const libc::c_char) == 0 {
        tag25h9_destroy(tf);
    } else if strcmp(famname, b"tag16h5\0" as *const u8 as *const libc::c_char) == 0 {
        tag16h5_destroy(tf);
    } else if strcmp(famname, b"tagCircle21h7\0" as *const u8 as *const libc::c_char)
            == 0
        {
        tagCircle21h7_destroy(tf);
    } else if strcmp(famname, b"tagCircle49h12\0" as *const u8 as *const libc::c_char)
            == 0
        {
        tagCircle49h12_destroy(tf);
    } else if strcmp(famname, b"tagStandard41h12\0" as *const u8 as *const libc::c_char)
            == 0
        {
        tagStandard41h12_destroy(tf);
    } else if strcmp(famname, b"tagStandard52h13\0" as *const u8 as *const libc::c_char)
            == 0
        {
        tagStandard52h13_destroy(tf);
    } else if strcmp(famname, b"tagCustom48h12\0" as *const u8 as *const libc::c_char)
            == 0
        {
        tagCustom48h12_destroy(tf);
    }
    getopt_destroy(getopt);
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
