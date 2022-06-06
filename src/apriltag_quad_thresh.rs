use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type workerpool;
    pub type zmaxheap;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn fmin(_: libc::c_double, _: libc::c_double) -> libc::c_double;
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
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn random() -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn image_u8_create(width: libc::c_uint, height: libc::c_uint) -> *mut image_u8_t;
    fn image_u8_create_alignment(
        width: libc::c_uint,
        height: libc::c_uint,
        alignment: libc::c_uint,
    ) -> *mut image_u8_t;
    fn image_u8_copy(in_0: *const image_u8_t) -> *mut image_u8_t;
    fn image_u8_darken(im: *mut image_u8_t);
    fn image_u8_destroy(im: *mut image_u8_t);
    fn image_u8_write_pnm(
        im: *const image_u8_t,
        path: *const libc::c_char,
    ) -> libc::c_int;
    fn workerpool_add_task(
        wp: *mut workerpool_t,
        f: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        p: *mut libc::c_void,
    );
    fn workerpool_run(wp: *mut workerpool_t);
    fn utime_now() -> int64_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn image_u8x3_create(width: libc::c_uint, height: libc::c_uint) -> *mut image_u8x3_t;
    fn image_u8x3_destroy(im: *mut image_u8x3_t);
    fn image_u8x3_write_pnm(
        im: *const image_u8x3_t,
        path: *const libc::c_char,
    ) -> libc::c_int;
    fn zmaxheap_create(el_sz: size_t) -> *mut zmaxheap_t;
    fn zmaxheap_destroy(heap: *mut zmaxheap_t);
    fn zmaxheap_add(heap: *mut zmaxheap_t, p: *mut libc::c_void, v: libc::c_float);
    fn zmaxheap_remove_max(
        heap: *mut zmaxheap_t,
        p: *mut libc::c_void,
        v: *mut libc::c_float,
    ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
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
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub struct unionfind {
    pub maxid: uint32_t,
    pub data: *mut ufrec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ufrec {
    pub parent: uint32_t,
    pub size: uint32_t,
}
pub type unionfind_t = unionfind;
pub type zmaxheap_t = zmaxheap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint64_zarray_entry {
    pub id: uint64_t,
    pub cluster: *mut zarray_t,
    pub next: *mut uint64_zarray_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pt {
    pub x: uint16_t,
    pub y: uint16_t,
    pub gx: int16_t,
    pub gy: int16_t,
    pub slope: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unionfind_task {
    pub y0: libc::c_int,
    pub y1: libc::c_int,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub s: libc::c_int,
    pub uf: *mut unionfind_t,
    pub im: *mut image_u8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quad_task {
    pub clusters: *mut zarray_t,
    pub cidx0: libc::c_int,
    pub cidx1: libc::c_int,
    pub quads: *mut zarray_t,
    pub td: *mut apriltag_detector_t,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub im: *mut image_u8_t,
    pub tag_width: libc::c_int,
    pub normal_border: bool,
    pub reversed_border: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cluster_task {
    pub y0: libc::c_int,
    pub y1: libc::c_int,
    pub w: libc::c_int,
    pub s: libc::c_int,
    pub nclustermap: libc::c_int,
    pub uf: *mut unionfind_t,
    pub im: *mut image_u8_t,
    pub clusters: *mut zarray_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct remove_vertex {
    pub i: libc::c_int,
    pub left: libc::c_int,
    pub right: libc::c_int,
    pub err: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segment {
    pub is_vertex: libc::c_int,
    pub left: libc::c_int,
    pub right: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line_fit_pt {
    pub Mx: libc::c_double,
    pub My: libc::c_double,
    pub Mxx: libc::c_double,
    pub Myy: libc::c_double,
    pub Mxy: libc::c_double,
    pub W: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cluster_hash {
    pub hash: uint32_t,
    pub id: uint64_t,
    pub data: *mut zarray_t,
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
    let ref mut fresh0 = (*za).data;
    *fresh0 = realloc(
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
    let ref mut fresh1 = (*za).size;
    *fresh1 += 1;
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
    let ref mut fresh2 = *(p as *mut *mut libc::c_void);
    *fresh2 = &mut *((*za).data)
        .offset((idx as libc::c_ulong).wrapping_mul((*za).el_sz) as isize)
        as *mut libc::c_char as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn zarray_add_all(
    mut dest: *mut zarray_t,
    mut source: *const zarray_t,
) {
    let mut tmp: *mut libc::c_char = calloc(
        1 as libc::c_int as libc::c_ulong,
        (*dest).el_sz,
    ) as *mut libc::c_char;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < zarray_size(source) {
        zarray_get(source, i, tmp as *mut libc::c_void);
        zarray_add(dest, tmp as *const libc::c_void);
        i += 1;
    }
    free(tmp as *mut libc::c_void);
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
unsafe extern "C" fn unionfind_create(mut maxid: uint32_t) -> *mut unionfind_t {
    let mut uf: *mut unionfind_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<unionfind_t>() as libc::c_ulong,
    ) as *mut unionfind_t;
    (*uf).maxid = maxid;
    let ref mut fresh3 = (*uf).data;
    *fresh3 = malloc(
        (maxid.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<ufrec>() as libc::c_ulong),
    ) as *mut ufrec;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i <= maxid {
        (*((*uf).data).offset(i as isize)).size = 1 as libc::c_int as uint32_t;
        (*((*uf).data).offset(i as isize)).parent = i;
        i = i.wrapping_add(1);
    }
    return uf;
}
#[inline]
unsafe extern "C" fn unionfind_destroy(mut uf: *mut unionfind_t) {
    free((*uf).data as *mut libc::c_void);
    free(uf as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn unionfind_get_representative(
    mut uf: *mut unionfind_t,
    mut id: uint32_t,
) -> uint32_t {
    let mut root: uint32_t = id;
    while (*((*uf).data).offset(root as isize)).parent != root {
        root = (*((*uf).data).offset(root as isize)).parent;
    }
    while (*((*uf).data).offset(id as isize)).parent != root {
        let mut tmp: uint32_t = (*((*uf).data).offset(id as isize)).parent;
        (*((*uf).data).offset(id as isize)).parent = root;
        id = tmp;
    }
    return root;
}
#[inline]
unsafe extern "C" fn unionfind_get_set_size(
    mut uf: *mut unionfind_t,
    mut id: uint32_t,
) -> uint32_t {
    let mut repid: uint32_t = unionfind_get_representative(uf, id);
    return (*((*uf).data).offset(repid as isize)).size;
}
#[inline]
unsafe extern "C" fn unionfind_connect(
    mut uf: *mut unionfind_t,
    mut aid: uint32_t,
    mut bid: uint32_t,
) -> uint32_t {
    let mut aroot: uint32_t = unionfind_get_representative(uf, aid);
    let mut broot: uint32_t = unionfind_get_representative(uf, bid);
    if aroot == broot {
        return aroot;
    }
    let mut asize: uint32_t = (*((*uf).data).offset(aroot as isize)).size;
    let mut bsize: uint32_t = (*((*uf).data).offset(broot as isize)).size;
    if asize > bsize {
        (*((*uf).data).offset(broot as isize)).parent = aroot;
        let ref mut fresh4 = (*((*uf).data).offset(aroot as isize)).size;
        *fresh4 = (*fresh4 as libc::c_uint).wrapping_add(bsize) as uint32_t as uint32_t;
        return aroot;
    } else {
        (*((*uf).data).offset(aroot as isize)).parent = broot;
        let ref mut fresh5 = (*((*uf).data).offset(broot as isize)).size;
        *fresh5 = (*fresh5 as libc::c_uint).wrapping_add(asize) as uint32_t as uint32_t;
        return broot;
    };
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
#[inline]
unsafe extern "C" fn sq(mut v: libc::c_double) -> libc::c_double {
    return v * v;
}
#[inline]
unsafe extern "C" fn imin(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn u64hash_2(mut x: uint64_t) -> uint32_t {
    return ((2654435761 as libc::c_long as libc::c_ulong).wrapping_mul(x)
        >> 32 as libc::c_int) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn fit_line(
    mut lfps: *mut line_fit_pt,
    mut sz: libc::c_int,
    mut i0: libc::c_int,
    mut i1: libc::c_int,
    mut lineparm: *mut libc::c_double,
    mut err: *mut libc::c_double,
    mut mse: *mut libc::c_double,
) {
    let mut Mx: libc::c_double = 0.;
    let mut My: libc::c_double = 0.;
    let mut Mxx: libc::c_double = 0.;
    let mut Myy: libc::c_double = 0.;
    let mut Mxy: libc::c_double = 0.;
    let mut W: libc::c_double = 0.;
    let mut N: libc::c_int = 0;
    if i0 < i1 {
        N = i1 - i0 + 1 as libc::c_int;
        Mx = (*lfps.offset(i1 as isize)).Mx;
        My = (*lfps.offset(i1 as isize)).My;
        Mxx = (*lfps.offset(i1 as isize)).Mxx;
        Mxy = (*lfps.offset(i1 as isize)).Mxy;
        Myy = (*lfps.offset(i1 as isize)).Myy;
        W = (*lfps.offset(i1 as isize)).W;
        if i0 > 0 as libc::c_int {
            Mx -= (*lfps.offset((i0 - 1 as libc::c_int) as isize)).Mx;
            My -= (*lfps.offset((i0 - 1 as libc::c_int) as isize)).My;
            Mxx -= (*lfps.offset((i0 - 1 as libc::c_int) as isize)).Mxx;
            Mxy -= (*lfps.offset((i0 - 1 as libc::c_int) as isize)).Mxy;
            Myy -= (*lfps.offset((i0 - 1 as libc::c_int) as isize)).Myy;
            W -= (*lfps.offset((i0 - 1 as libc::c_int) as isize)).W;
        }
    } else {
        Mx = (*lfps.offset((sz - 1 as libc::c_int) as isize)).Mx
            - (*lfps.offset((i0 - 1 as libc::c_int) as isize)).Mx;
        My = (*lfps.offset((sz - 1 as libc::c_int) as isize)).My
            - (*lfps.offset((i0 - 1 as libc::c_int) as isize)).My;
        Mxx = (*lfps.offset((sz - 1 as libc::c_int) as isize)).Mxx
            - (*lfps.offset((i0 - 1 as libc::c_int) as isize)).Mxx;
        Mxy = (*lfps.offset((sz - 1 as libc::c_int) as isize)).Mxy
            - (*lfps.offset((i0 - 1 as libc::c_int) as isize)).Mxy;
        Myy = (*lfps.offset((sz - 1 as libc::c_int) as isize)).Myy
            - (*lfps.offset((i0 - 1 as libc::c_int) as isize)).Myy;
        W = (*lfps.offset((sz - 1 as libc::c_int) as isize)).W
            - (*lfps.offset((i0 - 1 as libc::c_int) as isize)).W;
        Mx += (*lfps.offset(i1 as isize)).Mx;
        My += (*lfps.offset(i1 as isize)).My;
        Mxx += (*lfps.offset(i1 as isize)).Mxx;
        Mxy += (*lfps.offset(i1 as isize)).Mxy;
        Myy += (*lfps.offset(i1 as isize)).Myy;
        W += (*lfps.offset(i1 as isize)).W;
        N = sz - i0 + i1 + 1 as libc::c_int;
    }
    let mut Ex: libc::c_double = Mx / W;
    let mut Ey: libc::c_double = My / W;
    let mut Cxx: libc::c_double = Mxx / W - Ex * Ex;
    let mut Cxy: libc::c_double = Mxy / W - Ex * Ey;
    let mut Cyy: libc::c_double = Myy / W - Ey * Ey;
    let mut eig_small: libc::c_double = 0.5f64
        * (Cxx + Cyy
            - sqrtf(
                ((Cxx - Cyy) * (Cxx - Cyy)
                    + 4 as libc::c_int as libc::c_double * Cxy * Cxy) as libc::c_float,
            ) as libc::c_double);
    if !lineparm.is_null() {
        *lineparm.offset(0 as libc::c_int as isize) = Ex;
        *lineparm.offset(1 as libc::c_int as isize) = Ey;
        let mut eig: libc::c_double = 0.5f64
            * (Cxx + Cyy
                + sqrtf(
                    ((Cxx - Cyy) * (Cxx - Cyy)
                        + 4 as libc::c_int as libc::c_double * Cxy * Cxy)
                        as libc::c_float,
                ) as libc::c_double);
        let mut nx1: libc::c_double = Cxx - eig;
        let mut ny1: libc::c_double = Cxy;
        let mut M1: libc::c_double = nx1 * nx1 + ny1 * ny1;
        let mut nx2: libc::c_double = Cxy;
        let mut ny2: libc::c_double = Cyy - eig;
        let mut M2: libc::c_double = nx2 * nx2 + ny2 * ny2;
        let mut nx: libc::c_double = 0.;
        let mut ny: libc::c_double = 0.;
        let mut M: libc::c_double = 0.;
        if M1 > M2 {
            nx = nx1;
            ny = ny1;
            M = M1;
        } else {
            nx = nx2;
            ny = ny2;
            M = M2;
        }
        let mut length: libc::c_double = sqrtf(M as libc::c_float) as libc::c_double;
        *lineparm.offset(2 as libc::c_int as isize) = nx / length;
        *lineparm.offset(3 as libc::c_int as isize) = ny / length;
    }
    if !err.is_null() {
        *err = N as libc::c_double * eig_small;
    }
    if !mse.is_null() {
        *mse = eig_small;
    }
}
#[no_mangle]
pub unsafe extern "C" fn pt_compare_angle(
    mut a: *mut pt,
    mut b: *mut pt,
) -> libc::c_float {
    return (*a).slope - (*b).slope;
}
#[no_mangle]
pub unsafe extern "C" fn err_compare_descending(
    mut _a: *const libc::c_void,
    mut _b: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const libc::c_double = _a as *const libc::c_double;
    let mut b: *const libc::c_double = _b as *const libc::c_double;
    return if *a < *b { 1 as libc::c_int } else { -(1 as libc::c_int) };
}
#[no_mangle]
pub unsafe extern "C" fn quad_segment_maxima(
    mut td: *mut apriltag_detector_t,
    mut cluster: *mut zarray_t,
    mut lfps: *mut line_fit_pt,
    mut indices: *mut libc::c_int,
) -> libc::c_int {
    let mut sz: libc::c_int = zarray_size(cluster);
    let mut ksz: libc::c_int = imin(20 as libc::c_int, sz / 12 as libc::c_int);
    if ksz < 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut errs: *mut libc::c_double = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(sz as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < sz {
        fit_line(
            lfps,
            sz,
            (i + sz - ksz) % sz,
            (i + ksz) % sz,
            0 as *mut libc::c_double,
            &mut *errs.offset(i as isize),
            0 as *mut libc::c_double,
        );
        i += 1;
    }
    let mut y: *mut libc::c_double = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(sz as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut sigma: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut cutoff: libc::c_double = 0.05f64;
    let mut fsz: libc::c_int = (sqrt(
        -log(cutoff) * 2 as libc::c_int as libc::c_double * sigma * sigma,
    ) + 1 as libc::c_int as libc::c_double) as libc::c_int;
    fsz = 2 as libc::c_int * fsz + 1 as libc::c_int;
    let mut f: *mut libc::c_float = malloc(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(fsz as libc::c_ulong),
    ) as *mut libc::c_float;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < fsz {
        let mut j: libc::c_int = i_0 - fsz / 2 as libc::c_int;
        *f
            .offset(
                i_0 as isize,
            ) = exp(
            (-j * j) as libc::c_double
                / (2 as libc::c_int as libc::c_double * sigma * sigma),
        ) as libc::c_float;
        i_0 += 1;
    }
    let mut iy: libc::c_int = 0 as libc::c_int;
    while iy < sz {
        let mut acc: libc::c_double = 0 as libc::c_int as libc::c_double;
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < fsz {
            acc
                += *errs.offset(((iy + i_1 - fsz / 2 as libc::c_int + sz) % sz) as isize)
                    * *f.offset(i_1 as isize) as libc::c_double;
            i_1 += 1;
        }
        *y.offset(iy as isize) = acc;
        iy += 1;
    }
    memcpy(
        errs as *mut libc::c_void,
        y as *const libc::c_void,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(sz as libc::c_ulong),
    );
    free(y as *mut libc::c_void);
    free(f as *mut libc::c_void);
    let mut maxima: *mut libc::c_int = malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(sz as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut maxima_errs: *mut libc::c_double = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(sz as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut nmaxima: libc::c_int = 0 as libc::c_int;
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < sz {
        if *errs.offset(i_2 as isize)
            > *errs.offset(((i_2 + 1 as libc::c_int) % sz) as isize)
            && *errs.offset(i_2 as isize)
                > *errs.offset(((i_2 + sz - 1 as libc::c_int) % sz) as isize)
        {
            *maxima.offset(nmaxima as isize) = i_2;
            *maxima_errs.offset(nmaxima as isize) = *errs.offset(i_2 as isize);
            nmaxima += 1;
        }
        i_2 += 1;
    }
    free(errs as *mut libc::c_void);
    if nmaxima < 4 as libc::c_int {
        free(maxima as *mut libc::c_void);
        free(maxima_errs as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    let mut max_nmaxima: libc::c_int = (*td).qtp.max_nmaxima;
    if nmaxima > max_nmaxima {
        let mut maxima_errs_copy: *mut libc::c_double = malloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(nmaxima as libc::c_ulong),
        ) as *mut libc::c_double;
        memcpy(
            maxima_errs_copy as *mut libc::c_void,
            maxima_errs as *const libc::c_void,
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(nmaxima as libc::c_ulong),
        );
        qsort(
            maxima_errs_copy as *mut libc::c_void,
            nmaxima as size_t,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
            Some(
                err_compare_descending
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        let mut maxima_thresh: libc::c_double = *maxima_errs_copy
            .offset(max_nmaxima as isize);
        let mut out: libc::c_int = 0 as libc::c_int;
        let mut in_0: libc::c_int = 0 as libc::c_int;
        while in_0 < nmaxima {
            if !(*maxima_errs.offset(in_0 as isize) <= maxima_thresh) {
                let fresh6 = out;
                out = out + 1;
                *maxima.offset(fresh6 as isize) = *maxima.offset(in_0 as isize);
            }
            in_0 += 1;
        }
        nmaxima = out;
        free(maxima_errs_copy as *mut libc::c_void);
    }
    free(maxima_errs as *mut libc::c_void);
    let mut best_indices: [libc::c_int; 4] = [0; 4];
    let mut best_error: libc::c_double = ::std::f32::INFINITY as libc::c_double;
    let mut err01: libc::c_double = 0.;
    let mut err12: libc::c_double = 0.;
    let mut err23: libc::c_double = 0.;
    let mut err30: libc::c_double = 0.;
    let mut mse01: libc::c_double = 0.;
    let mut mse12: libc::c_double = 0.;
    let mut mse23: libc::c_double = 0.;
    let mut mse30: libc::c_double = 0.;
    let mut params01: [libc::c_double; 4] = [0.; 4];
    let mut params12: [libc::c_double; 4] = [0.; 4];
    let mut params23: [libc::c_double; 4] = [0.; 4];
    let mut params30: [libc::c_double; 4] = [0.; 4];
    let mut max_dot: libc::c_double = (*td).qtp.cos_critical_rad as libc::c_double;
    let mut m0: libc::c_int = 0 as libc::c_int;
    while m0 < nmaxima - 3 as libc::c_int {
        let mut i0: libc::c_int = *maxima.offset(m0 as isize);
        let mut m1: libc::c_int = m0 + 1 as libc::c_int;
        while m1 < nmaxima - 2 as libc::c_int {
            let mut i1: libc::c_int = *maxima.offset(m1 as isize);
            fit_line(lfps, sz, i0, i1, params01.as_mut_ptr(), &mut err01, &mut mse01);
            if !(mse01 > (*td).qtp.max_line_fit_mse as libc::c_double) {
                let mut m2: libc::c_int = m1 + 1 as libc::c_int;
                while m2 < nmaxima - 1 as libc::c_int {
                    let mut i2: libc::c_int = *maxima.offset(m2 as isize);
                    fit_line(
                        lfps,
                        sz,
                        i1,
                        i2,
                        params12.as_mut_ptr(),
                        &mut err12,
                        &mut mse12,
                    );
                    if !(mse12 > (*td).qtp.max_line_fit_mse as libc::c_double) {
                        let mut dot: libc::c_double = params01[2 as libc::c_int as usize]
                            * params12[2 as libc::c_int as usize]
                            + params01[3 as libc::c_int as usize]
                                * params12[3 as libc::c_int as usize];
                        if !(fabs(dot) > max_dot) {
                            let mut m3: libc::c_int = m2 + 1 as libc::c_int;
                            while m3 < nmaxima {
                                let mut i3: libc::c_int = *maxima.offset(m3 as isize);
                                fit_line(
                                    lfps,
                                    sz,
                                    i2,
                                    i3,
                                    params23.as_mut_ptr(),
                                    &mut err23,
                                    &mut mse23,
                                );
                                if !(mse23 > (*td).qtp.max_line_fit_mse as libc::c_double) {
                                    fit_line(
                                        lfps,
                                        sz,
                                        i3,
                                        i0,
                                        params30.as_mut_ptr(),
                                        &mut err30,
                                        &mut mse30,
                                    );
                                    if !(mse30 > (*td).qtp.max_line_fit_mse as libc::c_double) {
                                        let mut err: libc::c_double = err01 + err12 + err23 + err30;
                                        if err < best_error {
                                            best_error = err;
                                            best_indices[0 as libc::c_int as usize] = i0;
                                            best_indices[1 as libc::c_int as usize] = i1;
                                            best_indices[2 as libc::c_int as usize] = i2;
                                            best_indices[3 as libc::c_int as usize] = i3;
                                        }
                                    }
                                }
                                m3 += 1;
                            }
                        }
                    }
                    m2 += 1;
                }
            }
            m1 += 1;
        }
        m0 += 1;
    }
    free(maxima as *mut libc::c_void);
    if best_error == ::std::f32::INFINITY as libc::c_double {
        return 0 as libc::c_int;
    }
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < 4 as libc::c_int {
        *indices.offset(i_3 as isize) = best_indices[i_3 as usize];
        i_3 += 1;
    }
    if (best_error / sz as libc::c_double) < (*td).qtp.max_line_fit_mse as libc::c_double
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quad_segment_agg(
    mut td: *mut apriltag_detector_t,
    mut cluster: *mut zarray_t,
    mut lfps: *mut line_fit_pt,
    mut indices: *mut libc::c_int,
) -> libc::c_int {
    let mut sz: libc::c_int = zarray_size(cluster);
    let mut heap: *mut zmaxheap_t = zmaxheap_create(
        ::std::mem::size_of::<*mut remove_vertex>() as libc::c_ulong,
    );
    let mut rvalloc_pos: libc::c_int = 0 as libc::c_int;
    let mut rvalloc_size: libc::c_int = 3 as libc::c_int * sz;
    let mut rvalloc: *mut remove_vertex = calloc(
        rvalloc_size as libc::c_ulong,
        ::std::mem::size_of::<remove_vertex>() as libc::c_ulong,
    ) as *mut remove_vertex;
    let mut segs: *mut segment = calloc(
        sz as libc::c_ulong,
        ::std::mem::size_of::<segment>() as libc::c_ulong,
    ) as *mut segment;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < sz {
        let fresh7 = rvalloc_pos;
        rvalloc_pos = rvalloc_pos + 1;
        let mut rv: *mut remove_vertex = &mut *rvalloc.offset(fresh7 as isize)
            as *mut remove_vertex;
        (*rv).i = i;
        if i == 0 as libc::c_int {
            (*rv).left = sz - 1 as libc::c_int;
            (*rv).right = 1 as libc::c_int;
        } else {
            (*rv).left = i - 1 as libc::c_int;
            (*rv).right = (i + 1 as libc::c_int) % sz;
        }
        fit_line(
            lfps,
            sz,
            (*rv).left,
            (*rv).right,
            0 as *mut libc::c_double,
            0 as *mut libc::c_double,
            &mut (*rv).err,
        );
        zmaxheap_add(
            heap,
            &mut rv as *mut *mut remove_vertex as *mut libc::c_void,
            -(*rv).err as libc::c_float,
        );
        (*segs.offset(i as isize)).left = (*rv).left;
        (*segs.offset(i as isize)).right = (*rv).right;
        (*segs.offset(i as isize)).is_vertex = 1 as libc::c_int;
        i += 1;
    }
    let mut nvertices: libc::c_int = sz;
    while nvertices > 4 as libc::c_int {
        let mut rv_0: *mut remove_vertex = 0 as *mut remove_vertex;
        let mut err: libc::c_float = 0.;
        let mut res: libc::c_int = zmaxheap_remove_max(
            heap,
            &mut rv_0 as *mut *mut remove_vertex as *mut libc::c_void,
            &mut err,
        );
        if res == 0 {
            return 0 as libc::c_int;
        }
        if (*segs.offset((*rv_0).i as isize)).is_vertex == 0
            || (*segs.offset((*rv_0).left as isize)).is_vertex == 0
            || (*segs.offset((*rv_0).right as isize)).is_vertex == 0
        {
            continue;
        }
        (*segs.offset((*rv_0).i as isize)).is_vertex = 0 as libc::c_int;
        (*segs.offset((*rv_0).left as isize)).right = (*rv_0).right;
        (*segs.offset((*rv_0).right as isize)).left = (*rv_0).left;
        let fresh8 = rvalloc_pos;
        rvalloc_pos = rvalloc_pos + 1;
        let mut child: *mut remove_vertex = &mut *rvalloc.offset(fresh8 as isize)
            as *mut remove_vertex;
        (*child).i = (*rv_0).left;
        (*child).left = (*segs.offset((*rv_0).left as isize)).left;
        (*child).right = (*rv_0).right;
        fit_line(
            lfps,
            sz,
            (*child).left,
            (*child).right,
            0 as *mut libc::c_double,
            0 as *mut libc::c_double,
            &mut (*child).err,
        );
        zmaxheap_add(
            heap,
            &mut child as *mut *mut remove_vertex as *mut libc::c_void,
            -(*child).err as libc::c_float,
        );
        let fresh9 = rvalloc_pos;
        rvalloc_pos = rvalloc_pos + 1;
        let mut child_0: *mut remove_vertex = &mut *rvalloc.offset(fresh9 as isize)
            as *mut remove_vertex;
        (*child_0).i = (*rv_0).right;
        (*child_0).left = (*rv_0).left;
        (*child_0).right = (*segs.offset((*rv_0).right as isize)).right;
        fit_line(
            lfps,
            sz,
            (*child_0).left,
            (*child_0).right,
            0 as *mut libc::c_double,
            0 as *mut libc::c_double,
            &mut (*child_0).err,
        );
        zmaxheap_add(
            heap,
            &mut child_0 as *mut *mut remove_vertex as *mut libc::c_void,
            -(*child_0).err as libc::c_float,
        );
        nvertices -= 1;
    }
    free(rvalloc as *mut libc::c_void);
    zmaxheap_destroy(heap);
    let mut idx: libc::c_int = 0 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < sz {
        if (*segs.offset(i_0 as isize)).is_vertex != 0 {
            let fresh10 = idx;
            idx = idx + 1;
            *indices.offset(fresh10 as isize) = i_0;
        }
        i_0 += 1;
    }
    free(segs as *mut libc::c_void);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn compute_lfps(
    mut sz: libc::c_int,
    mut cluster: *mut zarray_t,
    mut im: *mut image_u8_t,
) -> *mut line_fit_pt {
    let mut lfps: *mut line_fit_pt = calloc(
        sz as libc::c_ulong,
        ::std::mem::size_of::<line_fit_pt>() as libc::c_ulong,
    ) as *mut line_fit_pt;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < sz {
        let mut p: *mut pt = 0 as *mut pt;
        zarray_get_volatile(cluster, i, &mut p as *mut *mut pt as *mut libc::c_void);
        if i > 0 as libc::c_int {
            memcpy(
                &mut *lfps.offset(i as isize) as *mut line_fit_pt as *mut libc::c_void,
                &mut *lfps.offset((i - 1 as libc::c_int) as isize) as *mut line_fit_pt
                    as *const libc::c_void,
                ::std::mem::size_of::<line_fit_pt>() as libc::c_ulong,
            );
        }
        let mut delta: libc::c_double = 0.5f64;
        let mut x: libc::c_double = (*p).x as libc::c_int as libc::c_double * 0.5f64
            + delta;
        let mut y: libc::c_double = (*p).y as libc::c_int as libc::c_double * 0.5f64
            + delta;
        let mut ix: libc::c_int = x as libc::c_int;
        let mut iy: libc::c_int = y as libc::c_int;
        let mut W: libc::c_double = 1 as libc::c_int as libc::c_double;
        if ix > 0 as libc::c_int && (ix + 1 as libc::c_int) < (*im).width
            && iy > 0 as libc::c_int && (iy + 1 as libc::c_int) < (*im).height
        {
            let mut grad_x: libc::c_int = *((*im).buf)
                .offset((iy * (*im).stride + ix + 1 as libc::c_int) as isize)
                as libc::c_int
                - *((*im).buf)
                    .offset((iy * (*im).stride + ix - 1 as libc::c_int) as isize)
                    as libc::c_int;
            let mut grad_y: libc::c_int = *((*im).buf)
                .offset(((iy + 1 as libc::c_int) * (*im).stride + ix) as isize)
                as libc::c_int
                - *((*im).buf)
                    .offset(((iy - 1 as libc::c_int) * (*im).stride + ix) as isize)
                    as libc::c_int;
            W = sqrt((grad_x * grad_x + grad_y * grad_y) as libc::c_double)
                + 1 as libc::c_int as libc::c_double;
        }
        let mut fx: libc::c_double = x;
        let mut fy: libc::c_double = y;
        (*lfps.offset(i as isize)).Mx += W * fx;
        (*lfps.offset(i as isize)).My += W * fy;
        (*lfps.offset(i as isize)).Mxx += W * fx * fx;
        (*lfps.offset(i as isize)).Mxy += W * fx * fy;
        (*lfps.offset(i as isize)).Myy += W * fy * fy;
        (*lfps.offset(i as isize)).W += W;
        i += 1;
    }
    return lfps;
}
#[inline]
unsafe extern "C" fn ptsort(mut pts: *mut pt, mut sz: libc::c_int) {
    if sz <= 1 as libc::c_int {
        return;
    }
    if sz == 2 as libc::c_int {
        let mut tmp: pt = pt {
            x: 0,
            y: 0,
            gx: 0,
            gy: 0,
            slope: 0.,
        };
        if pt_compare_angle(
            &mut *pts.offset(0 as libc::c_int as isize),
            &mut *pts.offset(1 as libc::c_int as isize),
        ) > 0 as libc::c_int as libc::c_float
        {
            tmp = *pts.offset(0 as libc::c_int as isize);
            *pts
                .offset(
                    0 as libc::c_int as isize,
                ) = *pts.offset(1 as libc::c_int as isize);
            *pts.offset(1 as libc::c_int as isize) = tmp;
        }
        return;
    }
    if sz == 3 as libc::c_int {
        let mut tmp_0: pt = pt {
            x: 0,
            y: 0,
            gx: 0,
            gy: 0,
            slope: 0.,
        };
        if pt_compare_angle(
            &mut *pts.offset(0 as libc::c_int as isize),
            &mut *pts.offset(1 as libc::c_int as isize),
        ) > 0 as libc::c_int as libc::c_float
        {
            tmp_0 = *pts.offset(0 as libc::c_int as isize);
            *pts
                .offset(
                    0 as libc::c_int as isize,
                ) = *pts.offset(1 as libc::c_int as isize);
            *pts.offset(1 as libc::c_int as isize) = tmp_0;
        }
        if pt_compare_angle(
            &mut *pts.offset(1 as libc::c_int as isize),
            &mut *pts.offset(2 as libc::c_int as isize),
        ) > 0 as libc::c_int as libc::c_float
        {
            tmp_0 = *pts.offset(1 as libc::c_int as isize);
            *pts
                .offset(
                    1 as libc::c_int as isize,
                ) = *pts.offset(2 as libc::c_int as isize);
            *pts.offset(2 as libc::c_int as isize) = tmp_0;
        }
        if pt_compare_angle(
            &mut *pts.offset(0 as libc::c_int as isize),
            &mut *pts.offset(1 as libc::c_int as isize),
        ) > 0 as libc::c_int as libc::c_float
        {
            tmp_0 = *pts.offset(0 as libc::c_int as isize);
            *pts
                .offset(
                    0 as libc::c_int as isize,
                ) = *pts.offset(1 as libc::c_int as isize);
            *pts.offset(1 as libc::c_int as isize) = tmp_0;
        }
        return;
    }
    if sz == 4 as libc::c_int {
        let mut tmp_1: pt = pt {
            x: 0,
            y: 0,
            gx: 0,
            gy: 0,
            slope: 0.,
        };
        if pt_compare_angle(
            &mut *pts.offset(0 as libc::c_int as isize),
            &mut *pts.offset(1 as libc::c_int as isize),
        ) > 0 as libc::c_int as libc::c_float
        {
            tmp_1 = *pts.offset(0 as libc::c_int as isize);
            *pts
                .offset(
                    0 as libc::c_int as isize,
                ) = *pts.offset(1 as libc::c_int as isize);
            *pts.offset(1 as libc::c_int as isize) = tmp_1;
        }
        if pt_compare_angle(
            &mut *pts.offset(2 as libc::c_int as isize),
            &mut *pts.offset(3 as libc::c_int as isize),
        ) > 0 as libc::c_int as libc::c_float
        {
            tmp_1 = *pts.offset(2 as libc::c_int as isize);
            *pts
                .offset(
                    2 as libc::c_int as isize,
                ) = *pts.offset(3 as libc::c_int as isize);
            *pts.offset(3 as libc::c_int as isize) = tmp_1;
        }
        if pt_compare_angle(
            &mut *pts.offset(0 as libc::c_int as isize),
            &mut *pts.offset(2 as libc::c_int as isize),
        ) > 0 as libc::c_int as libc::c_float
        {
            tmp_1 = *pts.offset(0 as libc::c_int as isize);
            *pts
                .offset(
                    0 as libc::c_int as isize,
                ) = *pts.offset(2 as libc::c_int as isize);
            *pts.offset(2 as libc::c_int as isize) = tmp_1;
        }
        if pt_compare_angle(
            &mut *pts.offset(1 as libc::c_int as isize),
            &mut *pts.offset(3 as libc::c_int as isize),
        ) > 0 as libc::c_int as libc::c_float
        {
            tmp_1 = *pts.offset(1 as libc::c_int as isize);
            *pts
                .offset(
                    1 as libc::c_int as isize,
                ) = *pts.offset(3 as libc::c_int as isize);
            *pts.offset(3 as libc::c_int as isize) = tmp_1;
        }
        if pt_compare_angle(
            &mut *pts.offset(1 as libc::c_int as isize),
            &mut *pts.offset(2 as libc::c_int as isize),
        ) > 0 as libc::c_int as libc::c_float
        {
            tmp_1 = *pts.offset(1 as libc::c_int as isize);
            *pts
                .offset(
                    1 as libc::c_int as isize,
                ) = *pts.offset(2 as libc::c_int as isize);
            *pts.offset(2 as libc::c_int as isize) = tmp_1;
        }
        return;
    }
    if sz == 5 as libc::c_int {
        let mut tmp_2: pt = pt {
            x: 0,
            y: 0,
            gx: 0,
            gy: 0,
            slope: 0.,
        };
        if pt_compare_angle(
            &mut *pts.offset(0 as libc::c_int as isize),
            &mut *pts.offset(1 as libc::c_int as isize),
        ) > 0 as libc::c_int as libc::c_float
        {
            tmp_2 = *pts.offset(0 as libc::c_int as isize);
            *pts
                .offset(
                    0 as libc::c_int as isize,
                ) = *pts.offset(1 as libc::c_int as isize);
            *pts.offset(1 as libc::c_int as isize) = tmp_2;
        }
        if pt_compare_angle(
            &mut *pts.offset(3 as libc::c_int as isize),
            &mut *pts.offset(4 as libc::c_int as isize),
        ) > 0 as libc::c_int as libc::c_float
        {
            tmp_2 = *pts.offset(3 as libc::c_int as isize);
            *pts
                .offset(
                    3 as libc::c_int as isize,
                ) = *pts.offset(4 as libc::c_int as isize);
            *pts.offset(4 as libc::c_int as isize) = tmp_2;
        }
        if pt_compare_angle(
            &mut *pts.offset(1 as libc::c_int as isize),
            &mut *pts.offset(2 as libc::c_int as isize),
        ) > 0 as libc::c_int as libc::c_float
        {
            tmp_2 = *pts.offset(1 as libc::c_int as isize);
            *pts
                .offset(
                    1 as libc::c_int as isize,
                ) = *pts.offset(2 as libc::c_int as isize);
            *pts.offset(2 as libc::c_int as isize) = tmp_2;
        }
        if pt_compare_angle(
            &mut *pts.offset(0 as libc::c_int as isize),
            &mut *pts.offset(1 as libc::c_int as isize),
        ) > 0 as libc::c_int as libc::c_float
        {
            tmp_2 = *pts.offset(0 as libc::c_int as isize);
            *pts
                .offset(
                    0 as libc::c_int as isize,
                ) = *pts.offset(1 as libc::c_int as isize);
            *pts.offset(1 as libc::c_int as isize) = tmp_2;
        }
        if pt_compare_angle(
            &mut *pts.offset(0 as libc::c_int as isize),
            &mut *pts.offset(3 as libc::c_int as isize),
        ) > 0 as libc::c_int as libc::c_float
        {
            tmp_2 = *pts.offset(0 as libc::c_int as isize);
            *pts
                .offset(
                    0 as libc::c_int as isize,
                ) = *pts.offset(3 as libc::c_int as isize);
            *pts.offset(3 as libc::c_int as isize) = tmp_2;
        }
        if pt_compare_angle(
            &mut *pts.offset(2 as libc::c_int as isize),
            &mut *pts.offset(4 as libc::c_int as isize),
        ) > 0 as libc::c_int as libc::c_float
        {
            tmp_2 = *pts.offset(2 as libc::c_int as isize);
            *pts
                .offset(
                    2 as libc::c_int as isize,
                ) = *pts.offset(4 as libc::c_int as isize);
            *pts.offset(4 as libc::c_int as isize) = tmp_2;
        }
        if pt_compare_angle(
            &mut *pts.offset(1 as libc::c_int as isize),
            &mut *pts.offset(2 as libc::c_int as isize),
        ) > 0 as libc::c_int as libc::c_float
        {
            tmp_2 = *pts.offset(1 as libc::c_int as isize);
            *pts
                .offset(
                    1 as libc::c_int as isize,
                ) = *pts.offset(2 as libc::c_int as isize);
            *pts.offset(2 as libc::c_int as isize) = tmp_2;
        }
        if pt_compare_angle(
            &mut *pts.offset(2 as libc::c_int as isize),
            &mut *pts.offset(3 as libc::c_int as isize),
        ) > 0 as libc::c_int as libc::c_float
        {
            tmp_2 = *pts.offset(2 as libc::c_int as isize);
            *pts
                .offset(
                    2 as libc::c_int as isize,
                ) = *pts.offset(3 as libc::c_int as isize);
            *pts.offset(3 as libc::c_int as isize) = tmp_2;
        }
        if pt_compare_angle(
            &mut *pts.offset(1 as libc::c_int as isize),
            &mut *pts.offset(2 as libc::c_int as isize),
        ) > 0 as libc::c_int as libc::c_float
        {
            tmp_2 = *pts.offset(1 as libc::c_int as isize);
            *pts
                .offset(
                    1 as libc::c_int as isize,
                ) = *pts.offset(2 as libc::c_int as isize);
            *pts.offset(2 as libc::c_int as isize) = tmp_2;
        }
        return;
    }
    let mut tmp_3: *mut pt = malloc(
        (::std::mem::size_of::<pt>() as libc::c_ulong).wrapping_mul(sz as libc::c_ulong),
    ) as *mut pt;
    memcpy(
        tmp_3 as *mut libc::c_void,
        pts as *const libc::c_void,
        (::std::mem::size_of::<pt>() as libc::c_ulong).wrapping_mul(sz as libc::c_ulong),
    );
    let mut asz: libc::c_int = sz / 2 as libc::c_int;
    let mut bsz: libc::c_int = sz - asz;
    let mut as_0: *mut pt = &mut *tmp_3.offset(0 as libc::c_int as isize) as *mut pt;
    let mut bs: *mut pt = &mut *tmp_3.offset(asz as isize) as *mut pt;
    ptsort(as_0, asz);
    ptsort(bs, bsz);
    let mut apos: libc::c_int = 0 as libc::c_int;
    let mut bpos: libc::c_int = 0 as libc::c_int;
    let mut outpos: libc::c_int = 0 as libc::c_int;
    while (apos + 8 as libc::c_int) < asz && (bpos + 8 as libc::c_int) < bsz {
        if pt_compare_angle(
            &mut *as_0.offset(apos as isize),
            &mut *bs.offset(bpos as isize),
        ) < 0 as libc::c_int as libc::c_float
        {
            let fresh11 = apos;
            apos = apos + 1;
            let fresh12 = outpos;
            outpos = outpos + 1;
            *pts.offset(fresh12 as isize) = *as_0.offset(fresh11 as isize);
        } else {
            let fresh13 = bpos;
            bpos = bpos + 1;
            let fresh14 = outpos;
            outpos = outpos + 1;
            *pts.offset(fresh14 as isize) = *bs.offset(fresh13 as isize);
        }
        if pt_compare_angle(
            &mut *as_0.offset(apos as isize),
            &mut *bs.offset(bpos as isize),
        ) < 0 as libc::c_int as libc::c_float
        {
            let fresh15 = apos;
            apos = apos + 1;
            let fresh16 = outpos;
            outpos = outpos + 1;
            *pts.offset(fresh16 as isize) = *as_0.offset(fresh15 as isize);
        } else {
            let fresh17 = bpos;
            bpos = bpos + 1;
            let fresh18 = outpos;
            outpos = outpos + 1;
            *pts.offset(fresh18 as isize) = *bs.offset(fresh17 as isize);
        }
        if pt_compare_angle(
            &mut *as_0.offset(apos as isize),
            &mut *bs.offset(bpos as isize),
        ) < 0 as libc::c_int as libc::c_float
        {
            let fresh19 = apos;
            apos = apos + 1;
            let fresh20 = outpos;
            outpos = outpos + 1;
            *pts.offset(fresh20 as isize) = *as_0.offset(fresh19 as isize);
        } else {
            let fresh21 = bpos;
            bpos = bpos + 1;
            let fresh22 = outpos;
            outpos = outpos + 1;
            *pts.offset(fresh22 as isize) = *bs.offset(fresh21 as isize);
        }
        if pt_compare_angle(
            &mut *as_0.offset(apos as isize),
            &mut *bs.offset(bpos as isize),
        ) < 0 as libc::c_int as libc::c_float
        {
            let fresh23 = apos;
            apos = apos + 1;
            let fresh24 = outpos;
            outpos = outpos + 1;
            *pts.offset(fresh24 as isize) = *as_0.offset(fresh23 as isize);
        } else {
            let fresh25 = bpos;
            bpos = bpos + 1;
            let fresh26 = outpos;
            outpos = outpos + 1;
            *pts.offset(fresh26 as isize) = *bs.offset(fresh25 as isize);
        }
        if pt_compare_angle(
            &mut *as_0.offset(apos as isize),
            &mut *bs.offset(bpos as isize),
        ) < 0 as libc::c_int as libc::c_float
        {
            let fresh27 = apos;
            apos = apos + 1;
            let fresh28 = outpos;
            outpos = outpos + 1;
            *pts.offset(fresh28 as isize) = *as_0.offset(fresh27 as isize);
        } else {
            let fresh29 = bpos;
            bpos = bpos + 1;
            let fresh30 = outpos;
            outpos = outpos + 1;
            *pts.offset(fresh30 as isize) = *bs.offset(fresh29 as isize);
        }
        if pt_compare_angle(
            &mut *as_0.offset(apos as isize),
            &mut *bs.offset(bpos as isize),
        ) < 0 as libc::c_int as libc::c_float
        {
            let fresh31 = apos;
            apos = apos + 1;
            let fresh32 = outpos;
            outpos = outpos + 1;
            *pts.offset(fresh32 as isize) = *as_0.offset(fresh31 as isize);
        } else {
            let fresh33 = bpos;
            bpos = bpos + 1;
            let fresh34 = outpos;
            outpos = outpos + 1;
            *pts.offset(fresh34 as isize) = *bs.offset(fresh33 as isize);
        }
        if pt_compare_angle(
            &mut *as_0.offset(apos as isize),
            &mut *bs.offset(bpos as isize),
        ) < 0 as libc::c_int as libc::c_float
        {
            let fresh35 = apos;
            apos = apos + 1;
            let fresh36 = outpos;
            outpos = outpos + 1;
            *pts.offset(fresh36 as isize) = *as_0.offset(fresh35 as isize);
        } else {
            let fresh37 = bpos;
            bpos = bpos + 1;
            let fresh38 = outpos;
            outpos = outpos + 1;
            *pts.offset(fresh38 as isize) = *bs.offset(fresh37 as isize);
        }
        if pt_compare_angle(
            &mut *as_0.offset(apos as isize),
            &mut *bs.offset(bpos as isize),
        ) < 0 as libc::c_int as libc::c_float
        {
            let fresh39 = apos;
            apos = apos + 1;
            let fresh40 = outpos;
            outpos = outpos + 1;
            *pts.offset(fresh40 as isize) = *as_0.offset(fresh39 as isize);
        } else {
            let fresh41 = bpos;
            bpos = bpos + 1;
            let fresh42 = outpos;
            outpos = outpos + 1;
            *pts.offset(fresh42 as isize) = *bs.offset(fresh41 as isize);
        }
    }
    while apos < asz && bpos < bsz {
        if pt_compare_angle(
            &mut *as_0.offset(apos as isize),
            &mut *bs.offset(bpos as isize),
        ) < 0 as libc::c_int as libc::c_float
        {
            let fresh43 = apos;
            apos = apos + 1;
            let fresh44 = outpos;
            outpos = outpos + 1;
            *pts.offset(fresh44 as isize) = *as_0.offset(fresh43 as isize);
        } else {
            let fresh45 = bpos;
            bpos = bpos + 1;
            let fresh46 = outpos;
            outpos = outpos + 1;
            *pts.offset(fresh46 as isize) = *bs.offset(fresh45 as isize);
        }
    }
    if apos < asz {
        memcpy(
            &mut *pts.offset(outpos as isize) as *mut pt as *mut libc::c_void,
            &mut *as_0.offset(apos as isize) as *mut pt as *const libc::c_void,
            ((asz - apos) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<pt>() as libc::c_ulong),
        );
    }
    if bpos < bsz {
        memcpy(
            &mut *pts.offset(outpos as isize) as *mut pt as *mut libc::c_void,
            &mut *bs.offset(bpos as isize) as *mut pt as *const libc::c_void,
            ((bsz - bpos) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<pt>() as libc::c_ulong),
        );
    }
    free(tmp_3 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn fit_quad(
    mut td: *mut apriltag_detector_t,
    mut im: *mut image_u8_t,
    mut cluster: *mut zarray_t,
    mut quad: *mut quad,
    mut tag_width: libc::c_int,
    mut normal_border: bool,
    mut reversed_border: bool,
) -> libc::c_int {
    let mut lines: [[libc::c_double; 4]; 4] = [[0.; 4]; 4];
    let mut current_block: u64;
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut sz: libc::c_int = zarray_size(cluster);
    if sz < 24 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut p1: *mut pt = 0 as *mut pt;
    zarray_get_volatile(
        cluster,
        0 as libc::c_int,
        &mut p1 as *mut *mut pt as *mut libc::c_void,
    );
    let mut xmax: uint16_t = (*p1).x;
    let mut xmin: uint16_t = (*p1).x;
    let mut ymax: uint16_t = (*p1).y;
    let mut ymin: uint16_t = (*p1).y;
    let mut pidx: libc::c_int = 1 as libc::c_int;
    while pidx < zarray_size(cluster) {
        let mut p: *mut pt = 0 as *mut pt;
        zarray_get_volatile(cluster, pidx, &mut p as *mut *mut pt as *mut libc::c_void);
        if (*p).x as libc::c_int > xmax as libc::c_int {
            xmax = (*p).x;
        } else if ((*p).x as libc::c_int) < xmin as libc::c_int {
            xmin = (*p).x;
        }
        if (*p).y as libc::c_int > ymax as libc::c_int {
            ymax = (*p).y;
        } else if ((*p).y as libc::c_int) < ymin as libc::c_int {
            ymin = (*p).y;
        }
        pidx += 1;
    }
    if (xmax as libc::c_int - xmin as libc::c_int)
        * (ymax as libc::c_int - ymin as libc::c_int) < tag_width
    {
        return 0 as libc::c_int;
    }
    let mut cx: libc::c_float = ((xmin as libc::c_int + xmax as libc::c_int)
        as libc::c_double * 0.5f64 + 0.05118f64) as libc::c_float;
    let mut cy: libc::c_float = ((ymin as libc::c_int + ymax as libc::c_int)
        as libc::c_double * 0.5f64 + -0.028581f64) as libc::c_float;
    let mut dot: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut quadrants: [[libc::c_float; 2]; 2] = [
        [
            (-(1 as libc::c_int) * ((2 as libc::c_int) << 15 as libc::c_int))
                as libc::c_float,
            0 as libc::c_int as libc::c_float,
        ],
        [
            (2 as libc::c_int * ((2 as libc::c_int) << 15 as libc::c_int))
                as libc::c_float,
            ((2 as libc::c_int) << 15 as libc::c_int) as libc::c_float,
        ],
    ];
    let mut pidx_0: libc::c_int = 0 as libc::c_int;
    while pidx_0 < zarray_size(cluster) {
        let mut p_0: *mut pt = 0 as *mut pt;
        zarray_get_volatile(
            cluster,
            pidx_0,
            &mut p_0 as *mut *mut pt as *mut libc::c_void,
        );
        let mut dx: libc::c_float = (*p_0).x as libc::c_int as libc::c_float - cx;
        let mut dy: libc::c_float = (*p_0).y as libc::c_int as libc::c_float - cy;
        dot
            += dx * (*p_0).gx as libc::c_int as libc::c_float
                + dy * (*p_0).gy as libc::c_int as libc::c_float;
        let mut quadrant: libc::c_float = quadrants[(dy
            > 0 as libc::c_int as libc::c_float) as libc::c_int
            as usize][(dx > 0 as libc::c_int as libc::c_float) as libc::c_int as usize];
        if dy < 0 as libc::c_int as libc::c_float {
            dy = -dy;
            dx = -dx;
        }
        if dx < 0 as libc::c_int as libc::c_float {
            let mut tmp: libc::c_float = dx;
            dx = dy;
            dy = -tmp;
        }
        (*p_0).slope = quadrant + dy / dx;
        pidx_0 += 1;
    }
    (*quad).reversed_border = dot < 0 as libc::c_int as libc::c_float;
    if !reversed_border && (*quad).reversed_border as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    if !normal_border && !(*quad).reversed_border {
        return 0 as libc::c_int;
    }
    ptsort((*cluster).data as *mut pt, zarray_size(cluster));
    let mut outpos: libc::c_int = 1 as libc::c_int;
    let mut last: *mut pt = 0 as *mut pt;
    zarray_get_volatile(
        cluster,
        0 as libc::c_int,
        &mut last as *mut *mut pt as *mut libc::c_void,
    );
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < sz {
        let mut p_1: *mut pt = 0 as *mut pt;
        zarray_get_volatile(cluster, i, &mut p_1 as *mut *mut pt as *mut libc::c_void);
        if (*p_1).x as libc::c_int != (*last).x as libc::c_int
            || (*p_1).y as libc::c_int != (*last).y as libc::c_int
        {
            if i != outpos {
                let mut out: *mut pt = 0 as *mut pt;
                zarray_get_volatile(
                    cluster,
                    outpos,
                    &mut out as *mut *mut pt as *mut libc::c_void,
                );
                memcpy(
                    out as *mut libc::c_void,
                    p_1 as *const libc::c_void,
                    ::std::mem::size_of::<pt>() as libc::c_ulong,
                );
            }
            outpos += 1;
        }
        last = p_1;
        i += 1;
    }
    (*cluster).size = outpos;
    sz = outpos;
    if sz < 24 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut lfps: *mut line_fit_pt = compute_lfps(sz, cluster, im);
    let mut indices: [libc::c_int; 4] = [0; 4];
    if !(quad_segment_maxima(td, cluster, lfps, indices.as_mut_ptr()) == 0) {
        lines = [[0.; 4]; 4];
        let mut i_0: libc::c_int = 0 as libc::c_int;
        loop {
            if !(i_0 < 4 as libc::c_int) {
                current_block = 2310077433060450808;
                break;
            }
            let mut i0: libc::c_int = indices[i_0 as usize];
            let mut i1: libc::c_int = indices[(i_0 + 1 as libc::c_int & 3 as libc::c_int)
                as usize];
            let mut err: libc::c_double = 0.;
            fit_line(
                lfps,
                sz,
                i0,
                i1,
                (lines[i_0 as usize]).as_mut_ptr(),
                0 as *mut libc::c_double,
                &mut err,
            );
            if err > (*td).qtp.max_line_fit_mse as libc::c_double {
                res = 0 as libc::c_int;
                current_block = 2305958262682200376;
                break;
            } else {
                i_0 += 1;
            }
        }
        match current_block {
            2305958262682200376 => {}
            _ => {
                let mut i_1: libc::c_int = 0 as libc::c_int;
                loop {
                    if !(i_1 < 4 as libc::c_int) {
                        current_block = 15734707049249739970;
                        break;
                    }
                    let mut A00: libc::c_double = lines[i_1
                        as usize][3 as libc::c_int as usize];
                    let mut A01: libc::c_double = -lines[(i_1 + 1 as libc::c_int
                        & 3 as libc::c_int) as usize][3 as libc::c_int as usize];
                    let mut A10: libc::c_double = -lines[i_1
                        as usize][2 as libc::c_int as usize];
                    let mut A11: libc::c_double = lines[(i_1 + 1 as libc::c_int
                        & 3 as libc::c_int) as usize][2 as libc::c_int as usize];
                    let mut B0: libc::c_double = -lines[i_1
                        as usize][0 as libc::c_int as usize]
                        + lines[(i_1 + 1 as libc::c_int & 3 as libc::c_int)
                            as usize][0 as libc::c_int as usize];
                    let mut B1: libc::c_double = -lines[i_1
                        as usize][1 as libc::c_int as usize]
                        + lines[(i_1 + 1 as libc::c_int & 3 as libc::c_int)
                            as usize][1 as libc::c_int as usize];
                    let mut det: libc::c_double = A00 * A11 - A10 * A01;
                    let mut W00: libc::c_double = A11 / det;
                    let mut W01: libc::c_double = -A01 / det;
                    if fabs(det) < 0.001f64 {
                        res = 0 as libc::c_int;
                        current_block = 2305958262682200376;
                        break;
                    } else {
                        let mut L0: libc::c_double = W00 * B0 + W01 * B1;
                        (*quad)
                            .p[i_1
                            as usize][0 as libc::c_int
                            as usize] = (lines[i_1 as usize][0 as libc::c_int as usize]
                            + L0 * A00) as libc::c_float;
                        (*quad)
                            .p[i_1
                            as usize][1 as libc::c_int
                            as usize] = (lines[i_1 as usize][1 as libc::c_int as usize]
                            + L0 * A10) as libc::c_float;
                        res = 1 as libc::c_int;
                        i_1 += 1;
                    }
                }
                match current_block {
                    2305958262682200376 => {}
                    _ => {
                        let mut area: libc::c_double = 0 as libc::c_int
                            as libc::c_double;
                        let mut length: [libc::c_double; 3] = [0.; 3];
                        let mut p_2: libc::c_double = 0.;
                        let mut i_2: libc::c_int = 0 as libc::c_int;
                        while i_2 < 3 as libc::c_int {
                            let mut idxa: libc::c_int = i_2;
                            let mut idxb: libc::c_int = (i_2 + 1 as libc::c_int)
                                % 3 as libc::c_int;
                            length[i_2
                                as usize] = sqrt(
                                sq(
                                    ((*quad).p[idxb as usize][0 as libc::c_int as usize]
                                        - (*quad).p[idxa as usize][0 as libc::c_int as usize])
                                        as libc::c_double,
                                )
                                    + sq(
                                        ((*quad).p[idxb as usize][1 as libc::c_int as usize]
                                            - (*quad).p[idxa as usize][1 as libc::c_int as usize])
                                            as libc::c_double,
                                    ),
                            );
                            i_2 += 1;
                        }
                        p_2 = (length[0 as libc::c_int as usize]
                            + length[1 as libc::c_int as usize]
                            + length[2 as libc::c_int as usize])
                            / 2 as libc::c_int as libc::c_double;
                        area
                            += sqrt(
                                p_2 * (p_2 - length[0 as libc::c_int as usize])
                                    * (p_2 - length[1 as libc::c_int as usize])
                                    * (p_2 - length[2 as libc::c_int as usize]),
                            );
                        let mut i_3: libc::c_int = 0 as libc::c_int;
                        while i_3 < 3 as libc::c_int {
                            let mut idxs: [libc::c_int; 4] = [
                                2 as libc::c_int,
                                3 as libc::c_int,
                                0 as libc::c_int,
                                2 as libc::c_int,
                            ];
                            let mut idxa_0: libc::c_int = idxs[i_3 as usize];
                            let mut idxb_0: libc::c_int = idxs[(i_3 + 1 as libc::c_int)
                                as usize];
                            length[i_3
                                as usize] = sqrt(
                                sq(
                                    ((*quad).p[idxb_0 as usize][0 as libc::c_int as usize]
                                        - (*quad).p[idxa_0 as usize][0 as libc::c_int as usize])
                                        as libc::c_double,
                                )
                                    + sq(
                                        ((*quad).p[idxb_0 as usize][1 as libc::c_int as usize]
                                            - (*quad).p[idxa_0 as usize][1 as libc::c_int as usize])
                                            as libc::c_double,
                                    ),
                            );
                            i_3 += 1;
                        }
                        p_2 = (length[0 as libc::c_int as usize]
                            + length[1 as libc::c_int as usize]
                            + length[2 as libc::c_int as usize])
                            / 2 as libc::c_int as libc::c_double;
                        area
                            += sqrt(
                                p_2 * (p_2 - length[0 as libc::c_int as usize])
                                    * (p_2 - length[1 as libc::c_int as usize])
                                    * (p_2 - length[2 as libc::c_int as usize]),
                            );
                        if area
                            < 0.95f64 * tag_width as libc::c_double
                                * tag_width as libc::c_double
                        {
                            res = 0 as libc::c_int;
                        } else {
                            let mut i_4: libc::c_int = 0 as libc::c_int;
                            while i_4 < 4 as libc::c_int {
                                let mut i0_0: libc::c_int = i_4;
                                let mut i1_0: libc::c_int = i_4 + 1 as libc::c_int
                                    & 3 as libc::c_int;
                                let mut i2: libc::c_int = i_4 + 2 as libc::c_int
                                    & 3 as libc::c_int;
                                let mut dx1: libc::c_double = ((*quad)
                                    .p[i1_0 as usize][0 as libc::c_int as usize]
                                    - (*quad).p[i0_0 as usize][0 as libc::c_int as usize])
                                    as libc::c_double;
                                let mut dy1: libc::c_double = ((*quad)
                                    .p[i1_0 as usize][1 as libc::c_int as usize]
                                    - (*quad).p[i0_0 as usize][1 as libc::c_int as usize])
                                    as libc::c_double;
                                let mut dx2: libc::c_double = ((*quad)
                                    .p[i2 as usize][0 as libc::c_int as usize]
                                    - (*quad).p[i1_0 as usize][0 as libc::c_int as usize])
                                    as libc::c_double;
                                let mut dy2: libc::c_double = ((*quad)
                                    .p[i2 as usize][1 as libc::c_int as usize]
                                    - (*quad).p[i1_0 as usize][1 as libc::c_int as usize])
                                    as libc::c_double;
                                let mut cos_dtheta: libc::c_double = (dx1 * dx2 + dy1 * dy2)
                                    / sqrt((dx1 * dx1 + dy1 * dy1) * (dx2 * dx2 + dy2 * dy2));
                                if cos_dtheta > (*td).qtp.cos_critical_rad as libc::c_double
                                    || cos_dtheta
                                        < -(*td).qtp.cos_critical_rad as libc::c_double
                                    || dx1 * dy2 < dy1 * dx2
                                {
                                    res = 0 as libc::c_int;
                                    break;
                                } else {
                                    i_4 += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free(lfps as *mut libc::c_void);
    return res;
}
unsafe extern "C" fn do_unionfind_first_line(
    mut uf: *mut unionfind_t,
    mut im: *mut image_u8_t,
    mut h: libc::c_int,
    mut w: libc::c_int,
    mut s: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut v: uint8_t = 0;
    let mut x: libc::c_int = 1 as libc::c_int;
    while x < w - 1 as libc::c_int {
        v = *((*im).buf).offset((y * s + x) as isize);
        if !(v as libc::c_int == 127 as libc::c_int) {
            if *((*im).buf)
                .offset(((y + 0 as libc::c_int) * s + x + -(1 as libc::c_int)) as isize)
                as libc::c_int == v as libc::c_int
            {
                unionfind_connect(
                    uf,
                    (y * w + x) as uint32_t,
                    ((y + 0 as libc::c_int) * w + x + -(1 as libc::c_int)) as uint32_t,
                );
            }
        }
        x += 1;
    }
}
unsafe extern "C" fn do_unionfind_line2(
    mut uf: *mut unionfind_t,
    mut im: *mut image_u8_t,
    mut h: libc::c_int,
    mut w: libc::c_int,
    mut s: libc::c_int,
    mut y: libc::c_int,
) {
    let mut v_m1_m1: uint8_t = 0;
    let mut v_0_m1: uint8_t = *((*im).buf).offset(((y - 1 as libc::c_int) * s) as isize);
    let mut v_1_m1: uint8_t = *((*im).buf)
        .offset(((y - 1 as libc::c_int) * s + 1 as libc::c_int) as isize);
    let mut v_m1_0: uint8_t = 0;
    let mut v: uint8_t = *((*im).buf).offset((y * s) as isize);
    let mut x: libc::c_int = 1 as libc::c_int;
    while x < w - 1 as libc::c_int {
        v_m1_m1 = v_0_m1;
        v_0_m1 = v_1_m1;
        v_1_m1 = *((*im).buf)
            .offset(((y - 1 as libc::c_int) * s + x + 1 as libc::c_int) as isize);
        v_m1_0 = v;
        v = *((*im).buf).offset((y * s + x) as isize);
        if !(v as libc::c_int == 127 as libc::c_int) {
            if *((*im).buf)
                .offset(((y + 0 as libc::c_int) * s + x + -(1 as libc::c_int)) as isize)
                as libc::c_int == v as libc::c_int
            {
                unionfind_connect(
                    uf,
                    (y * w + x) as uint32_t,
                    ((y + 0 as libc::c_int) * w + x + -(1 as libc::c_int)) as uint32_t,
                );
            }
            if x == 1 as libc::c_int
                || !(v_m1_0 as libc::c_int == v_m1_m1 as libc::c_int
                    && v_m1_m1 as libc::c_int == v_0_m1 as libc::c_int)
            {
                if *((*im).buf)
                    .offset(
                        ((y + -(1 as libc::c_int)) * s + x + 0 as libc::c_int) as isize,
                    ) as libc::c_int == v as libc::c_int
                {
                    unionfind_connect(
                        uf,
                        (y * w + x) as uint32_t,
                        ((y + -(1 as libc::c_int)) * w + x + 0 as libc::c_int)
                            as uint32_t,
                    );
                }
            }
            if v as libc::c_int == 255 as libc::c_int {
                if x == 1 as libc::c_int
                    || !(v_m1_0 as libc::c_int == v_m1_m1 as libc::c_int
                        || v_0_m1 as libc::c_int == v_m1_m1 as libc::c_int)
                {
                    if *((*im).buf)
                        .offset(
                            ((y + -(1 as libc::c_int)) * s + x + -(1 as libc::c_int))
                                as isize,
                        ) as libc::c_int == v as libc::c_int
                    {
                        unionfind_connect(
                            uf,
                            (y * w + x) as uint32_t,
                            ((y + -(1 as libc::c_int)) * w + x + -(1 as libc::c_int))
                                as uint32_t,
                        );
                    }
                }
                if !(v_0_m1 as libc::c_int == v_1_m1 as libc::c_int) {
                    if *((*im).buf)
                        .offset(
                            ((y + -(1 as libc::c_int)) * s + x + 1 as libc::c_int)
                                as isize,
                        ) as libc::c_int == v as libc::c_int
                    {
                        unionfind_connect(
                            uf,
                            (y * w + x) as uint32_t,
                            ((y + -(1 as libc::c_int)) * w + x + 1 as libc::c_int)
                                as uint32_t,
                        );
                    }
                }
            }
        }
        x += 1;
    }
}
unsafe extern "C" fn do_unionfind_task2(mut p: *mut libc::c_void) {
    let mut task: *mut unionfind_task = p as *mut unionfind_task;
    let mut y: libc::c_int = (*task).y0;
    while y < (*task).y1 {
        do_unionfind_line2((*task).uf, (*task).im, (*task).h, (*task).w, (*task).s, y);
        y += 1;
    }
}
unsafe extern "C" fn do_quad_task(mut p: *mut libc::c_void) {
    let mut task: *mut quad_task = p as *mut quad_task;
    let mut clusters: *mut zarray_t = (*task).clusters;
    let mut quads: *mut zarray_t = (*task).quads;
    let mut td: *mut apriltag_detector_t = (*task).td;
    let mut w: libc::c_int = (*task).w;
    let mut h: libc::c_int = (*task).h;
    let mut cidx: libc::c_int = (*task).cidx0;
    while cidx < (*task).cidx1 {
        let mut cluster: *mut zarray_t = 0 as *mut zarray_t;
        zarray_get(
            clusters,
            cidx,
            &mut cluster as *mut *mut zarray_t as *mut libc::c_void,
        );
        if !(zarray_size(cluster) < (*td).qtp.min_cluster_pixels) {
            if !(zarray_size(cluster)
                > 3 as libc::c_int * (2 as libc::c_int * w + 2 as libc::c_int * h))
            {
                let mut quad: quad = quad {
                    p: [[0.; 2]; 4],
                    reversed_border: false,
                    H: 0 as *mut matd_t,
                    Hinv: 0 as *mut matd_t,
                };
                memset(
                    &mut quad as *mut quad as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<quad>() as libc::c_ulong,
                );
                if fit_quad(
                    td,
                    (*task).im,
                    cluster,
                    &mut quad,
                    (*task).tag_width,
                    (*task).normal_border,
                    (*task).reversed_border,
                ) != 0
                {
                    pthread_mutex_lock(&mut (*td).mutex);
                    zarray_add(quads, &mut quad as *mut quad as *const libc::c_void);
                    pthread_mutex_unlock(&mut (*td).mutex);
                }
            }
        }
        cidx += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn threshold(
    mut td: *mut apriltag_detector_t,
    mut im: *mut image_u8_t,
) -> *mut image_u8_t {
    let mut w: libc::c_int = (*im).width;
    let mut h: libc::c_int = (*im).height;
    let mut s: libc::c_int = (*im).stride;
    let mut threshim: *mut image_u8_t = image_u8_create_alignment(
        w as libc::c_uint,
        h as libc::c_uint,
        s as libc::c_uint,
    );
    let tilesz: libc::c_int = 4 as libc::c_int;
    let mut tw: libc::c_int = w / tilesz;
    let mut th: libc::c_int = h / tilesz;
    let mut im_max: *mut uint8_t = calloc(
        (tw * th) as libc::c_ulong,
        ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
    ) as *mut uint8_t;
    let mut im_min: *mut uint8_t = calloc(
        (tw * th) as libc::c_ulong,
        ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
    ) as *mut uint8_t;
    let mut ty: libc::c_int = 0 as libc::c_int;
    while ty < th {
        let mut tx: libc::c_int = 0 as libc::c_int;
        while tx < tw {
            let mut max: uint8_t = 0 as libc::c_int as uint8_t;
            let mut min: uint8_t = 255 as libc::c_int as uint8_t;
            let mut dy: libc::c_int = 0 as libc::c_int;
            while dy < tilesz {
                let mut dx: libc::c_int = 0 as libc::c_int;
                while dx < tilesz {
                    let mut v: uint8_t = *((*im).buf)
                        .offset(((ty * tilesz + dy) * s + tx * tilesz + dx) as isize);
                    if (v as libc::c_int) < min as libc::c_int {
                        min = v;
                    }
                    if v as libc::c_int > max as libc::c_int {
                        max = v;
                    }
                    dx += 1;
                }
                dy += 1;
            }
            *im_max.offset((ty * tw + tx) as isize) = max;
            *im_min.offset((ty * tw + tx) as isize) = min;
            tx += 1;
        }
        ty += 1;
    }
    let mut im_max_tmp: *mut uint8_t = calloc(
        (tw * th) as libc::c_ulong,
        ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
    ) as *mut uint8_t;
    let mut im_min_tmp: *mut uint8_t = calloc(
        (tw * th) as libc::c_ulong,
        ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
    ) as *mut uint8_t;
    let mut ty_0: libc::c_int = 0 as libc::c_int;
    while ty_0 < th {
        let mut tx_0: libc::c_int = 0 as libc::c_int;
        while tx_0 < tw {
            let mut max_0: uint8_t = 0 as libc::c_int as uint8_t;
            let mut min_0: uint8_t = 255 as libc::c_int as uint8_t;
            let mut dy_0: libc::c_int = -(1 as libc::c_int);
            while dy_0 <= 1 as libc::c_int {
                if !(ty_0 + dy_0 < 0 as libc::c_int || ty_0 + dy_0 >= th) {
                    let mut dx_0: libc::c_int = -(1 as libc::c_int);
                    while dx_0 <= 1 as libc::c_int {
                        if !(tx_0 + dx_0 < 0 as libc::c_int || tx_0 + dx_0 >= tw) {
                            let mut m: uint8_t = *im_max
                                .offset(((ty_0 + dy_0) * tw + tx_0 + dx_0) as isize);
                            if m as libc::c_int > max_0 as libc::c_int {
                                max_0 = m;
                            }
                            m = *im_min
                                .offset(((ty_0 + dy_0) * tw + tx_0 + dx_0) as isize);
                            if (m as libc::c_int) < min_0 as libc::c_int {
                                min_0 = m;
                            }
                        }
                        dx_0 += 1;
                    }
                }
                dy_0 += 1;
            }
            *im_max_tmp.offset((ty_0 * tw + tx_0) as isize) = max_0;
            *im_min_tmp.offset((ty_0 * tw + tx_0) as isize) = min_0;
            tx_0 += 1;
        }
        ty_0 += 1;
    }
    free(im_max as *mut libc::c_void);
    free(im_min as *mut libc::c_void);
    im_max = im_max_tmp;
    im_min = im_min_tmp;
    let mut ty_1: libc::c_int = 0 as libc::c_int;
    while ty_1 < th {
        let mut tx_1: libc::c_int = 0 as libc::c_int;
        while tx_1 < tw {
            let mut min_1: libc::c_int = *im_min.offset((ty_1 * tw + tx_1) as isize)
                as libc::c_int;
            let mut max_1: libc::c_int = *im_max.offset((ty_1 * tw + tx_1) as isize)
                as libc::c_int;
            if max_1 - min_1 < (*td).qtp.min_white_black_diff {
                let mut dy_1: libc::c_int = 0 as libc::c_int;
                while dy_1 < tilesz {
                    let mut y: libc::c_int = ty_1 * tilesz + dy_1;
                    let mut dx_1: libc::c_int = 0 as libc::c_int;
                    while dx_1 < tilesz {
                        let mut x: libc::c_int = tx_1 * tilesz + dx_1;
                        *((*threshim).buf)
                            .offset(
                                (y * s + x) as isize,
                            ) = 127 as libc::c_int as uint8_t;
                        dx_1 += 1;
                    }
                    dy_1 += 1;
                }
            } else {
                let mut thresh: uint8_t = (min_1 + (max_1 - min_1) / 2 as libc::c_int)
                    as uint8_t;
                let mut dy_2: libc::c_int = 0 as libc::c_int;
                while dy_2 < tilesz {
                    let mut y_0: libc::c_int = ty_1 * tilesz + dy_2;
                    let mut dx_2: libc::c_int = 0 as libc::c_int;
                    while dx_2 < tilesz {
                        let mut x_0: libc::c_int = tx_1 * tilesz + dx_2;
                        let mut v_0: uint8_t = *((*im).buf)
                            .offset((y_0 * s + x_0) as isize);
                        if v_0 as libc::c_int > thresh as libc::c_int {
                            *((*threshim).buf)
                                .offset(
                                    (y_0 * s + x_0) as isize,
                                ) = 255 as libc::c_int as uint8_t;
                        } else {
                            *((*threshim).buf)
                                .offset(
                                    (y_0 * s + x_0) as isize,
                                ) = 0 as libc::c_int as uint8_t;
                        }
                        dx_2 += 1;
                    }
                    dy_2 += 1;
                }
            }
            tx_1 += 1;
        }
        ty_1 += 1;
    }
    let mut y_1: libc::c_int = 0 as libc::c_int;
    while y_1 < h {
        let mut x0: libc::c_int = 0;
        if y_1 >= th * tilesz {
            x0 = 0 as libc::c_int;
        } else {
            x0 = tw * tilesz;
        }
        let mut ty_2: libc::c_int = y_1 / tilesz;
        if ty_2 >= th {
            ty_2 = th - 1 as libc::c_int;
        }
        let mut x_1: libc::c_int = x0;
        while x_1 < w {
            let mut tx_2: libc::c_int = x_1 / tilesz;
            if tx_2 >= tw {
                tx_2 = tw - 1 as libc::c_int;
            }
            let mut max_2: libc::c_int = *im_max.offset((ty_2 * tw + tx_2) as isize)
                as libc::c_int;
            let mut min_2: libc::c_int = *im_min.offset((ty_2 * tw + tx_2) as isize)
                as libc::c_int;
            let mut thresh_0: libc::c_int = min_2 + (max_2 - min_2) / 2 as libc::c_int;
            let mut v_1: uint8_t = *((*im).buf).offset((y_1 * s + x_1) as isize);
            if v_1 as libc::c_int > thresh_0 {
                *((*threshim).buf)
                    .offset((y_1 * s + x_1) as isize) = 255 as libc::c_int as uint8_t;
            } else {
                *((*threshim).buf)
                    .offset((y_1 * s + x_1) as isize) = 0 as libc::c_int as uint8_t;
            }
            x_1 += 1;
        }
        y_1 += 1;
    }
    free(im_min as *mut libc::c_void);
    free(im_max as *mut libc::c_void);
    if 0 as libc::c_int != 0 || (*td).qtp.deglitch != 0 {
        let mut tmp: *mut image_u8_t = image_u8_create(
            w as libc::c_uint,
            h as libc::c_uint,
        );
        let mut y_2: libc::c_int = 1 as libc::c_int;
        while (y_2 + 1 as libc::c_int) < h {
            let mut x_2: libc::c_int = 1 as libc::c_int;
            while (x_2 + 1 as libc::c_int) < w {
                let mut max_3: uint8_t = 0 as libc::c_int as uint8_t;
                let mut dy_3: libc::c_int = -(1 as libc::c_int);
                while dy_3 <= 1 as libc::c_int {
                    let mut dx_3: libc::c_int = -(1 as libc::c_int);
                    while dx_3 <= 1 as libc::c_int {
                        let mut v_2: uint8_t = *((*threshim).buf)
                            .offset(((y_2 + dy_3) * s + x_2 + dx_3) as isize);
                        if v_2 as libc::c_int > max_3 as libc::c_int {
                            max_3 = v_2;
                        }
                        dx_3 += 1;
                    }
                    dy_3 += 1;
                }
                *((*tmp).buf).offset((y_2 * s + x_2) as isize) = max_3;
                x_2 += 1;
            }
            y_2 += 1;
        }
        let mut y_3: libc::c_int = 1 as libc::c_int;
        while (y_3 + 1 as libc::c_int) < h {
            let mut x_3: libc::c_int = 1 as libc::c_int;
            while (x_3 + 1 as libc::c_int) < w {
                let mut min_3: uint8_t = 255 as libc::c_int as uint8_t;
                let mut dy_4: libc::c_int = -(1 as libc::c_int);
                while dy_4 <= 1 as libc::c_int {
                    let mut dx_4: libc::c_int = -(1 as libc::c_int);
                    while dx_4 <= 1 as libc::c_int {
                        let mut v_3: uint8_t = *((*tmp).buf)
                            .offset(((y_3 + dy_4) * s + x_3 + dx_4) as isize);
                        if (v_3 as libc::c_int) < min_3 as libc::c_int {
                            min_3 = v_3;
                        }
                        dx_4 += 1;
                    }
                    dy_4 += 1;
                }
                *((*threshim).buf).offset((y_3 * s + x_3) as isize) = min_3;
                x_3 += 1;
            }
            y_3 += 1;
        }
        image_u8_destroy(tmp);
    }
    timeprofile_stamp((*td).tp, b"threshold\0" as *const u8 as *const libc::c_char);
    return threshim;
}
#[no_mangle]
pub unsafe extern "C" fn threshold_bayer(
    mut td: *mut apriltag_detector_t,
    mut im: *mut image_u8_t,
) -> *mut image_u8_t {
    let mut w: libc::c_int = (*im).width;
    let mut h: libc::c_int = (*im).height;
    let mut s: libc::c_int = (*im).stride;
    let mut threshim: *mut image_u8_t = image_u8_create_alignment(
        w as libc::c_uint,
        h as libc::c_uint,
        s as libc::c_uint,
    );
    let mut tilesz: libc::c_int = 32 as libc::c_int;
    let mut tw: libc::c_int = w / tilesz + 1 as libc::c_int;
    let mut th: libc::c_int = h / tilesz + 1 as libc::c_int;
    let mut im_max: [*mut uint8_t; 4] = [0 as *mut uint8_t; 4];
    let mut im_min: [*mut uint8_t; 4] = [0 as *mut uint8_t; 4];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        im_max[i
            as usize] = calloc(
            (tw * th) as libc::c_ulong,
            ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
        ) as *mut uint8_t;
        im_min[i
            as usize] = calloc(
            (tw * th) as libc::c_ulong,
            ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
        ) as *mut uint8_t;
        i += 1;
    }
    let mut ty: libc::c_int = 0 as libc::c_int;
    while ty < th {
        let mut tx: libc::c_int = 0 as libc::c_int;
        while tx < tw {
            let mut max: [uint8_t; 4] = [
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
            ];
            let mut min: [uint8_t; 4] = [
                255 as libc::c_int as uint8_t,
                255 as libc::c_int as uint8_t,
                255 as libc::c_int as uint8_t,
                255 as libc::c_int as uint8_t,
            ];
            let mut dy: libc::c_int = 0 as libc::c_int;
            while dy < tilesz {
                if !(ty * tilesz + dy >= h) {
                    let mut dx: libc::c_int = 0 as libc::c_int;
                    while dx < tilesz {
                        if !(tx * tilesz + dx >= w) {
                            let mut idx: libc::c_int = 2 as libc::c_int
                                * (dy & 1 as libc::c_int) + (dx & 1 as libc::c_int);
                            let mut v: uint8_t = *((*im).buf)
                                .offset(
                                    ((ty * tilesz + dy) * s + tx * tilesz + dx) as isize,
                                );
                            if (v as libc::c_int) < min[idx as usize] as libc::c_int {
                                min[idx as usize] = v;
                            }
                            if v as libc::c_int > max[idx as usize] as libc::c_int {
                                max[idx as usize] = v;
                            }
                        }
                        dx += 1;
                    }
                }
                dy += 1;
            }
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < 4 as libc::c_int {
                *(im_max[i_0 as usize])
                    .offset((ty * tw + tx) as isize) = max[i_0 as usize];
                *(im_min[i_0 as usize])
                    .offset((ty * tw + tx) as isize) = min[i_0 as usize];
                i_0 += 1;
            }
            tx += 1;
        }
        ty += 1;
    }
    let mut ty_0: libc::c_int = 0 as libc::c_int;
    while ty_0 < th {
        let mut tx_0: libc::c_int = 0 as libc::c_int;
        while tx_0 < tw {
            let mut max_0: [uint8_t; 4] = [
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
            ];
            let mut min_0: [uint8_t; 4] = [
                255 as libc::c_int as uint8_t,
                255 as libc::c_int as uint8_t,
                255 as libc::c_int as uint8_t,
                255 as libc::c_int as uint8_t,
            ];
            let mut dy_0: libc::c_int = -(1 as libc::c_int);
            while dy_0 <= 1 as libc::c_int {
                if !(ty_0 + dy_0 < 0 as libc::c_int || ty_0 + dy_0 >= th) {
                    let mut dx_0: libc::c_int = -(1 as libc::c_int);
                    while dx_0 <= 1 as libc::c_int {
                        if !(tx_0 + dx_0 < 0 as libc::c_int || tx_0 + dx_0 >= tw) {
                            let mut i_1: libc::c_int = 0 as libc::c_int;
                            while i_1 < 4 as libc::c_int {
                                let mut m: uint8_t = *(im_max[i_1 as usize])
                                    .offset(((ty_0 + dy_0) * tw + tx_0 + dx_0) as isize);
                                if m as libc::c_int > max_0[i_1 as usize] as libc::c_int {
                                    max_0[i_1 as usize] = m;
                                }
                                m = *(im_min[i_1 as usize])
                                    .offset(((ty_0 + dy_0) * tw + tx_0 + dx_0) as isize);
                                if (m as libc::c_int) < min_0[i_1 as usize] as libc::c_int {
                                    min_0[i_1 as usize] = m;
                                }
                                i_1 += 1;
                            }
                        }
                        dx_0 += 1;
                    }
                }
                dy_0 += 1;
            }
            let mut thresh: [uint8_t; 4] = [0; 4];
            let mut i_2: libc::c_int = 0 as libc::c_int;
            while i_2 < 4 as libc::c_int {
                thresh[i_2
                    as usize] = (min_0[i_2 as usize] as libc::c_int
                    + (max_0[i_2 as usize] as libc::c_int
                        - min_0[i_2 as usize] as libc::c_int) / 2 as libc::c_int)
                    as uint8_t;
                i_2 += 1;
            }
            let mut dy_1: libc::c_int = 0 as libc::c_int;
            while dy_1 < tilesz {
                let mut y: libc::c_int = ty_0 * tilesz + dy_1;
                if !(y >= h) {
                    let mut dx_1: libc::c_int = 0 as libc::c_int;
                    while dx_1 < tilesz {
                        let mut x: libc::c_int = tx_0 * tilesz + dx_1;
                        if !(x >= w) {
                            let mut idx_0: libc::c_int = 2 as libc::c_int
                                * (y & 1 as libc::c_int) + (x & 1 as libc::c_int);
                            let mut v_0: uint8_t = *((*im).buf)
                                .offset((y * s + x) as isize);
                            *((*threshim).buf)
                                .offset(
                                    (y * s + x) as isize,
                                ) = (v_0 as libc::c_int
                                > thresh[idx_0 as usize] as libc::c_int) as libc::c_int
                                as uint8_t;
                        }
                        dx_1 += 1;
                    }
                }
                dy_1 += 1;
            }
            tx_0 += 1;
        }
        ty_0 += 1;
    }
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < 4 as libc::c_int {
        free(im_min[i_3 as usize] as *mut libc::c_void);
        free(im_max[i_3 as usize] as *mut libc::c_void);
        i_3 += 1;
    }
    timeprofile_stamp((*td).tp, b"threshold\0" as *const u8 as *const libc::c_char);
    return threshim;
}
#[no_mangle]
pub unsafe extern "C" fn connected_components(
    mut td: *mut apriltag_detector_t,
    mut threshim: *mut image_u8_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut ts: libc::c_int,
) -> *mut unionfind_t {
    let mut uf: *mut unionfind_t = unionfind_create((w * h) as uint32_t);
    if (*td).nthreads <= 1 as libc::c_int {
        do_unionfind_first_line(uf, threshim, h, w, ts);
        let mut y: libc::c_int = 1 as libc::c_int;
        while y < h {
            do_unionfind_line2(uf, threshim, h, w, ts, y);
            y += 1;
        }
    } else {
        do_unionfind_first_line(uf, threshim, h, w, ts);
        let mut sz: libc::c_int = h;
        let mut chunksize: libc::c_int = 1 as libc::c_int
            + sz / (10 as libc::c_int * (*td).nthreads);
        let mut tasks: *mut unionfind_task = malloc(
            (::std::mem::size_of::<unionfind_task>() as libc::c_ulong)
                .wrapping_mul((sz / chunksize + 1 as libc::c_int) as libc::c_ulong),
        ) as *mut unionfind_task;
        let mut ntasks: libc::c_int = 0 as libc::c_int;
        let mut i: libc::c_int = 1 as libc::c_int;
        while i < sz {
            (*tasks.offset(ntasks as isize)).y0 = i;
            (*tasks.offset(ntasks as isize))
                .y1 = imin(sz, i + chunksize - 1 as libc::c_int);
            (*tasks.offset(ntasks as isize)).h = h;
            (*tasks.offset(ntasks as isize)).w = w;
            (*tasks.offset(ntasks as isize)).s = ts;
            let ref mut fresh47 = (*tasks.offset(ntasks as isize)).uf;
            *fresh47 = uf;
            let ref mut fresh48 = (*tasks.offset(ntasks as isize)).im;
            *fresh48 = threshim;
            workerpool_add_task(
                (*td).wp,
                Some(
                    do_unionfind_task2 as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
                &mut *tasks.offset(ntasks as isize) as *mut unionfind_task
                    as *mut libc::c_void,
            );
            ntasks += 1;
            i += chunksize;
        }
        workerpool_run((*td).wp);
        let mut i_0: libc::c_int = 1 as libc::c_int;
        while i_0 < ntasks {
            do_unionfind_line2(
                uf,
                threshim,
                h,
                w,
                ts,
                (*tasks.offset(i_0 as isize)).y0 - 1 as libc::c_int,
            );
            i_0 += 1;
        }
        free(tasks as *mut libc::c_void);
    }
    return uf;
}
#[no_mangle]
pub unsafe extern "C" fn do_gradient_clusters(
    mut threshim: *mut image_u8_t,
    mut ts: libc::c_int,
    mut y0: libc::c_int,
    mut y1: libc::c_int,
    mut w: libc::c_int,
    mut nclustermap: libc::c_int,
    mut uf: *mut unionfind_t,
    mut clusters: *mut zarray_t,
) -> *mut zarray_t {
    let mut clustermap: *mut *mut uint64_zarray_entry = calloc(
        nclustermap as libc::c_ulong,
        ::std::mem::size_of::<*mut uint64_zarray_entry>() as libc::c_ulong,
    ) as *mut *mut uint64_zarray_entry;
    let mut mem_chunk_size: libc::c_int = 2048 as libc::c_int;
    let mut mem_pools: *mut *mut uint64_zarray_entry = malloc(
        (::std::mem::size_of::<*mut uint64_zarray_entry>() as libc::c_ulong)
            .wrapping_mul(
                (1 as libc::c_int + 2 as libc::c_int * nclustermap / mem_chunk_size)
                    as libc::c_ulong,
            ),
    ) as *mut *mut uint64_zarray_entry;
    let mut mem_pool_idx: libc::c_int = 0 as libc::c_int;
    let mut mem_pool_loc: libc::c_int = 0 as libc::c_int;
    let ref mut fresh49 = *mem_pools.offset(mem_pool_idx as isize);
    *fresh49 = calloc(
        mem_chunk_size as libc::c_ulong,
        ::std::mem::size_of::<uint64_zarray_entry>() as libc::c_ulong,
    ) as *mut uint64_zarray_entry;
    let mut y: libc::c_int = y0;
    while y < y1 {
        let mut x: libc::c_int = 1 as libc::c_int;
        while x < w - 1 as libc::c_int {
            let mut v0: uint8_t = *((*threshim).buf).offset((y * ts + x) as isize);
            if !(v0 as libc::c_int == 127 as libc::c_int) {
                let mut rep0: uint64_t = unionfind_get_representative(
                    uf,
                    (y * w + x) as uint32_t,
                ) as uint64_t;
                if !(unionfind_get_set_size(uf, rep0 as uint32_t)
                    < 25 as libc::c_int as libc::c_uint)
                {
                    let mut v1: uint8_t = *((*threshim).buf)
                        .offset(
                            ((y + 0 as libc::c_int) * ts + x + 1 as libc::c_int) as isize,
                        );
                    if v0 as libc::c_int + v1 as libc::c_int == 255 as libc::c_int {
                        let mut rep1: uint64_t = unionfind_get_representative(
                            uf,
                            ((y + 0 as libc::c_int) * w + x + 1 as libc::c_int)
                                as uint32_t,
                        ) as uint64_t;
                        if unionfind_get_set_size(uf, rep1 as uint32_t)
                            > 24 as libc::c_int as libc::c_uint
                        {
                            let mut clusterid: uint64_t = 0;
                            if rep0 < rep1 {
                                clusterid = (rep1 << 32 as libc::c_int).wrapping_add(rep0);
                            } else {
                                clusterid = (rep0 << 32 as libc::c_int).wrapping_add(rep1);
                            }
                            let mut clustermap_bucket: uint32_t = (u64hash_2(clusterid))
                                .wrapping_rem(nclustermap as libc::c_uint);
                            let mut entry: *mut uint64_zarray_entry = *clustermap
                                .offset(clustermap_bucket as isize);
                            while !entry.is_null() && (*entry).id != clusterid {
                                entry = (*entry).next;
                            }
                            if entry.is_null() {
                                if mem_pool_loc == mem_chunk_size {
                                    mem_pool_loc = 0 as libc::c_int;
                                    mem_pool_idx += 1;
                                    let ref mut fresh50 = *mem_pools
                                        .offset(mem_pool_idx as isize);
                                    *fresh50 = calloc(
                                        mem_chunk_size as libc::c_ulong,
                                        ::std::mem::size_of::<uint64_zarray_entry>()
                                            as libc::c_ulong,
                                    ) as *mut uint64_zarray_entry;
                                }
                                entry = (*mem_pools.offset(mem_pool_idx as isize))
                                    .offset(mem_pool_loc as isize);
                                mem_pool_loc += 1;
                                (*entry).id = clusterid;
                                let ref mut fresh51 = (*entry).cluster;
                                *fresh51 = zarray_create(
                                    ::std::mem::size_of::<pt>() as libc::c_ulong,
                                );
                                let ref mut fresh52 = (*entry).next;
                                *fresh52 = *clustermap.offset(clustermap_bucket as isize);
                                let ref mut fresh53 = *clustermap
                                    .offset(clustermap_bucket as isize);
                                *fresh53 = entry;
                            }
                            let mut p: pt = {
                                let mut init = pt {
                                    x: (2 as libc::c_int * x + 1 as libc::c_int) as uint16_t,
                                    y: (2 as libc::c_int * y + 0 as libc::c_int) as uint16_t,
                                    gx: (1 as libc::c_int
                                        * (v1 as libc::c_int - v0 as libc::c_int)) as int16_t,
                                    gy: (0 as libc::c_int
                                        * (v1 as libc::c_int - v0 as libc::c_int)) as int16_t,
                                    slope: 0.,
                                };
                                init
                            };
                            zarray_add(
                                (*entry).cluster,
                                &mut p as *mut pt as *const libc::c_void,
                            );
                        }
                    }
                    let mut v1_0: uint8_t = *((*threshim).buf)
                        .offset(
                            ((y + 1 as libc::c_int) * ts + x + 0 as libc::c_int) as isize,
                        );
                    if v0 as libc::c_int + v1_0 as libc::c_int == 255 as libc::c_int {
                        let mut rep1_0: uint64_t = unionfind_get_representative(
                            uf,
                            ((y + 1 as libc::c_int) * w + x + 0 as libc::c_int)
                                as uint32_t,
                        ) as uint64_t;
                        if unionfind_get_set_size(uf, rep1_0 as uint32_t)
                            > 24 as libc::c_int as libc::c_uint
                        {
                            let mut clusterid_0: uint64_t = 0;
                            if rep0 < rep1_0 {
                                clusterid_0 = (rep1_0 << 32 as libc::c_int)
                                    .wrapping_add(rep0);
                            } else {
                                clusterid_0 = (rep0 << 32 as libc::c_int)
                                    .wrapping_add(rep1_0);
                            }
                            let mut clustermap_bucket_0: uint32_t = (u64hash_2(
                                clusterid_0,
                            ))
                                .wrapping_rem(nclustermap as libc::c_uint);
                            let mut entry_0: *mut uint64_zarray_entry = *clustermap
                                .offset(clustermap_bucket_0 as isize);
                            while !entry_0.is_null() && (*entry_0).id != clusterid_0 {
                                entry_0 = (*entry_0).next;
                            }
                            if entry_0.is_null() {
                                if mem_pool_loc == mem_chunk_size {
                                    mem_pool_loc = 0 as libc::c_int;
                                    mem_pool_idx += 1;
                                    let ref mut fresh54 = *mem_pools
                                        .offset(mem_pool_idx as isize);
                                    *fresh54 = calloc(
                                        mem_chunk_size as libc::c_ulong,
                                        ::std::mem::size_of::<uint64_zarray_entry>()
                                            as libc::c_ulong,
                                    ) as *mut uint64_zarray_entry;
                                }
                                entry_0 = (*mem_pools.offset(mem_pool_idx as isize))
                                    .offset(mem_pool_loc as isize);
                                mem_pool_loc += 1;
                                (*entry_0).id = clusterid_0;
                                let ref mut fresh55 = (*entry_0).cluster;
                                *fresh55 = zarray_create(
                                    ::std::mem::size_of::<pt>() as libc::c_ulong,
                                );
                                let ref mut fresh56 = (*entry_0).next;
                                *fresh56 = *clustermap.offset(clustermap_bucket_0 as isize);
                                let ref mut fresh57 = *clustermap
                                    .offset(clustermap_bucket_0 as isize);
                                *fresh57 = entry_0;
                            }
                            let mut p_0: pt = {
                                let mut init = pt {
                                    x: (2 as libc::c_int * x + 0 as libc::c_int) as uint16_t,
                                    y: (2 as libc::c_int * y + 1 as libc::c_int) as uint16_t,
                                    gx: (0 as libc::c_int
                                        * (v1_0 as libc::c_int - v0 as libc::c_int)) as int16_t,
                                    gy: (1 as libc::c_int
                                        * (v1_0 as libc::c_int - v0 as libc::c_int)) as int16_t,
                                    slope: 0.,
                                };
                                init
                            };
                            zarray_add(
                                (*entry_0).cluster,
                                &mut p_0 as *mut pt as *const libc::c_void,
                            );
                        }
                    }
                    let mut v1_1: uint8_t = *((*threshim).buf)
                        .offset(
                            ((y + 1 as libc::c_int) * ts + x + -(1 as libc::c_int))
                                as isize,
                        );
                    if v0 as libc::c_int + v1_1 as libc::c_int == 255 as libc::c_int {
                        let mut rep1_1: uint64_t = unionfind_get_representative(
                            uf,
                            ((y + 1 as libc::c_int) * w + x + -(1 as libc::c_int))
                                as uint32_t,
                        ) as uint64_t;
                        if unionfind_get_set_size(uf, rep1_1 as uint32_t)
                            > 24 as libc::c_int as libc::c_uint
                        {
                            let mut clusterid_1: uint64_t = 0;
                            if rep0 < rep1_1 {
                                clusterid_1 = (rep1_1 << 32 as libc::c_int)
                                    .wrapping_add(rep0);
                            } else {
                                clusterid_1 = (rep0 << 32 as libc::c_int)
                                    .wrapping_add(rep1_1);
                            }
                            let mut clustermap_bucket_1: uint32_t = (u64hash_2(
                                clusterid_1,
                            ))
                                .wrapping_rem(nclustermap as libc::c_uint);
                            let mut entry_1: *mut uint64_zarray_entry = *clustermap
                                .offset(clustermap_bucket_1 as isize);
                            while !entry_1.is_null() && (*entry_1).id != clusterid_1 {
                                entry_1 = (*entry_1).next;
                            }
                            if entry_1.is_null() {
                                if mem_pool_loc == mem_chunk_size {
                                    mem_pool_loc = 0 as libc::c_int;
                                    mem_pool_idx += 1;
                                    let ref mut fresh58 = *mem_pools
                                        .offset(mem_pool_idx as isize);
                                    *fresh58 = calloc(
                                        mem_chunk_size as libc::c_ulong,
                                        ::std::mem::size_of::<uint64_zarray_entry>()
                                            as libc::c_ulong,
                                    ) as *mut uint64_zarray_entry;
                                }
                                entry_1 = (*mem_pools.offset(mem_pool_idx as isize))
                                    .offset(mem_pool_loc as isize);
                                mem_pool_loc += 1;
                                (*entry_1).id = clusterid_1;
                                let ref mut fresh59 = (*entry_1).cluster;
                                *fresh59 = zarray_create(
                                    ::std::mem::size_of::<pt>() as libc::c_ulong,
                                );
                                let ref mut fresh60 = (*entry_1).next;
                                *fresh60 = *clustermap.offset(clustermap_bucket_1 as isize);
                                let ref mut fresh61 = *clustermap
                                    .offset(clustermap_bucket_1 as isize);
                                *fresh61 = entry_1;
                            }
                            let mut p_1: pt = {
                                let mut init = pt {
                                    x: (2 as libc::c_int * x + -(1 as libc::c_int)) as uint16_t,
                                    y: (2 as libc::c_int * y + 1 as libc::c_int) as uint16_t,
                                    gx: (-(1 as libc::c_int)
                                        * (v1_1 as libc::c_int - v0 as libc::c_int)) as int16_t,
                                    gy: (1 as libc::c_int
                                        * (v1_1 as libc::c_int - v0 as libc::c_int)) as int16_t,
                                    slope: 0.,
                                };
                                init
                            };
                            zarray_add(
                                (*entry_1).cluster,
                                &mut p_1 as *mut pt as *const libc::c_void,
                            );
                        }
                    }
                    let mut v1_2: uint8_t = *((*threshim).buf)
                        .offset(
                            ((y + 1 as libc::c_int) * ts + x + 1 as libc::c_int) as isize,
                        );
                    if v0 as libc::c_int + v1_2 as libc::c_int == 255 as libc::c_int {
                        let mut rep1_2: uint64_t = unionfind_get_representative(
                            uf,
                            ((y + 1 as libc::c_int) * w + x + 1 as libc::c_int)
                                as uint32_t,
                        ) as uint64_t;
                        if unionfind_get_set_size(uf, rep1_2 as uint32_t)
                            > 24 as libc::c_int as libc::c_uint
                        {
                            let mut clusterid_2: uint64_t = 0;
                            if rep0 < rep1_2 {
                                clusterid_2 = (rep1_2 << 32 as libc::c_int)
                                    .wrapping_add(rep0);
                            } else {
                                clusterid_2 = (rep0 << 32 as libc::c_int)
                                    .wrapping_add(rep1_2);
                            }
                            let mut clustermap_bucket_2: uint32_t = (u64hash_2(
                                clusterid_2,
                            ))
                                .wrapping_rem(nclustermap as libc::c_uint);
                            let mut entry_2: *mut uint64_zarray_entry = *clustermap
                                .offset(clustermap_bucket_2 as isize);
                            while !entry_2.is_null() && (*entry_2).id != clusterid_2 {
                                entry_2 = (*entry_2).next;
                            }
                            if entry_2.is_null() {
                                if mem_pool_loc == mem_chunk_size {
                                    mem_pool_loc = 0 as libc::c_int;
                                    mem_pool_idx += 1;
                                    let ref mut fresh62 = *mem_pools
                                        .offset(mem_pool_idx as isize);
                                    *fresh62 = calloc(
                                        mem_chunk_size as libc::c_ulong,
                                        ::std::mem::size_of::<uint64_zarray_entry>()
                                            as libc::c_ulong,
                                    ) as *mut uint64_zarray_entry;
                                }
                                entry_2 = (*mem_pools.offset(mem_pool_idx as isize))
                                    .offset(mem_pool_loc as isize);
                                mem_pool_loc += 1;
                                (*entry_2).id = clusterid_2;
                                let ref mut fresh63 = (*entry_2).cluster;
                                *fresh63 = zarray_create(
                                    ::std::mem::size_of::<pt>() as libc::c_ulong,
                                );
                                let ref mut fresh64 = (*entry_2).next;
                                *fresh64 = *clustermap.offset(clustermap_bucket_2 as isize);
                                let ref mut fresh65 = *clustermap
                                    .offset(clustermap_bucket_2 as isize);
                                *fresh65 = entry_2;
                            }
                            let mut p_2: pt = {
                                let mut init = pt {
                                    x: (2 as libc::c_int * x + 1 as libc::c_int) as uint16_t,
                                    y: (2 as libc::c_int * y + 1 as libc::c_int) as uint16_t,
                                    gx: (1 as libc::c_int
                                        * (v1_2 as libc::c_int - v0 as libc::c_int)) as int16_t,
                                    gy: (1 as libc::c_int
                                        * (v1_2 as libc::c_int - v0 as libc::c_int)) as int16_t,
                                    slope: 0.,
                                };
                                init
                            };
                            zarray_add(
                                (*entry_2).cluster,
                                &mut p_2 as *mut pt as *const libc::c_void,
                            );
                        }
                    }
                }
            }
            x += 1;
        }
        y += 1;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < nclustermap {
        let mut start: libc::c_int = zarray_size(clusters);
        let mut entry_3: *mut uint64_zarray_entry = *clustermap.offset(i as isize);
        while !entry_3.is_null() {
            let mut cluster_hash: *mut cluster_hash = malloc(
                ::std::mem::size_of::<cluster_hash>() as libc::c_ulong,
            ) as *mut cluster_hash;
            (*cluster_hash)
                .hash = (u64hash_2((*entry_3).id))
                .wrapping_rem(nclustermap as libc::c_uint);
            (*cluster_hash).id = (*entry_3).id;
            let ref mut fresh66 = (*cluster_hash).data;
            *fresh66 = (*entry_3).cluster;
            zarray_add(
                clusters,
                &mut cluster_hash as *mut *mut cluster_hash as *const libc::c_void,
            );
            entry_3 = (*entry_3).next;
        }
        let mut end: libc::c_int = zarray_size(clusters);
        let mut n: libc::c_int = end - start;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < n - 1 as libc::c_int {
            let mut k: libc::c_int = 0 as libc::c_int;
            while k < n - j - 1 as libc::c_int {
                let mut hash1: *mut cluster_hash = 0 as *mut cluster_hash;
                let mut hash2: *mut cluster_hash = 0 as *mut cluster_hash;
                zarray_get(
                    clusters,
                    start + k,
                    &mut hash1 as *mut *mut cluster_hash as *mut libc::c_void,
                );
                zarray_get(
                    clusters,
                    start + k + 1 as libc::c_int,
                    &mut hash2 as *mut *mut cluster_hash as *mut libc::c_void,
                );
                if (*hash1).id > (*hash2).id {
                    let mut tmp: cluster_hash = *hash2;
                    *hash2 = *hash1;
                    *hash1 = tmp;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 <= mem_pool_idx {
        free(*mem_pools.offset(i_0 as isize) as *mut libc::c_void);
        i_0 += 1;
    }
    free(mem_pools as *mut libc::c_void);
    free(clustermap as *mut libc::c_void);
    return clusters;
}
unsafe extern "C" fn do_cluster_task(mut p: *mut libc::c_void) {
    let mut task: *mut cluster_task = p as *mut cluster_task;
    do_gradient_clusters(
        (*task).im,
        (*task).s,
        (*task).y0,
        (*task).y1,
        (*task).w,
        (*task).nclustermap,
        (*task).uf,
        (*task).clusters,
    );
}
#[no_mangle]
pub unsafe extern "C" fn merge_clusters(
    mut c1: *mut zarray_t,
    mut c2: *mut zarray_t,
) -> *mut zarray_t {
    let mut ret: *mut zarray_t = zarray_create(
        ::std::mem::size_of::<*mut cluster_hash>() as libc::c_ulong,
    );
    zarray_ensure_capacity(ret, zarray_size(c1) + zarray_size(c2));
    let mut i1: libc::c_int = 0 as libc::c_int;
    let mut i2: libc::c_int = 0 as libc::c_int;
    let mut l1: libc::c_int = zarray_size(c1);
    let mut l2: libc::c_int = zarray_size(c2);
    while i1 < l1 && i2 < l2 {
        let mut h1: *mut cluster_hash = 0 as *mut cluster_hash;
        let mut h2: *mut cluster_hash = 0 as *mut cluster_hash;
        zarray_get(c1, i1, &mut h1 as *mut *mut cluster_hash as *mut libc::c_void);
        zarray_get(c2, i2, &mut h2 as *mut *mut cluster_hash as *mut libc::c_void);
        if (*h1).hash == (*h2).hash && (*h1).id == (*h2).id {
            zarray_add_all((*h1).data, (*h2).data);
            zarray_add(ret, &mut h1 as *mut *mut cluster_hash as *const libc::c_void);
            i1 += 1;
            i2 += 1;
            zarray_destroy((*h2).data);
            free(h2 as *mut libc::c_void);
        } else if (*h2).hash < (*h1).hash
                || (*h2).hash == (*h1).hash && (*h2).id < (*h1).id
            {
            zarray_add(ret, &mut h2 as *mut *mut cluster_hash as *const libc::c_void);
            i2 += 1;
        } else {
            zarray_add(ret, &mut h1 as *mut *mut cluster_hash as *const libc::c_void);
            i1 += 1;
        }
    }
    while i1 < l1 {
        let mut h1_0: *mut cluster_hash = 0 as *mut cluster_hash;
        zarray_get(c1, i1, &mut h1_0 as *mut *mut cluster_hash as *mut libc::c_void);
        zarray_add(ret, &mut h1_0 as *mut *mut cluster_hash as *const libc::c_void);
        i1 += 1;
    }
    while i2 < l2 {
        let mut h2_0: *mut cluster_hash = 0 as *mut cluster_hash;
        zarray_get(c2, i2, &mut h2_0 as *mut *mut cluster_hash as *mut libc::c_void);
        zarray_add(ret, &mut h2_0 as *mut *mut cluster_hash as *const libc::c_void);
        i2 += 1;
    }
    zarray_destroy(c1);
    zarray_destroy(c2);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn gradient_clusters(
    mut td: *mut apriltag_detector_t,
    mut threshim: *mut image_u8_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut ts: libc::c_int,
    mut uf: *mut unionfind_t,
) -> *mut zarray_t {
    let mut clusters: *mut zarray_t = 0 as *mut zarray_t;
    let mut nclustermap: libc::c_int = (0.2f64 * w as libc::c_double
        * h as libc::c_double) as libc::c_int;
    let mut sz: libc::c_int = h - 1 as libc::c_int;
    let mut chunksize: libc::c_int = 1 as libc::c_int + sz / (*td).nthreads;
    let mut tasks: *mut cluster_task = malloc(
        (::std::mem::size_of::<cluster_task>() as libc::c_ulong)
            .wrapping_mul((sz / chunksize + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut cluster_task;
    let mut ntasks: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < sz {
        (*tasks.offset(ntasks as isize)).y0 = i;
        (*tasks.offset(ntasks as isize)).y1 = imin(sz, i + chunksize);
        (*tasks.offset(ntasks as isize)).w = w;
        (*tasks.offset(ntasks as isize)).s = ts;
        let ref mut fresh67 = (*tasks.offset(ntasks as isize)).uf;
        *fresh67 = uf;
        let ref mut fresh68 = (*tasks.offset(ntasks as isize)).im;
        *fresh68 = threshim;
        (*tasks.offset(ntasks as isize))
            .nclustermap = nclustermap / (sz / chunksize + 1 as libc::c_int);
        let ref mut fresh69 = (*tasks.offset(ntasks as isize)).clusters;
        *fresh69 = zarray_create(
            ::std::mem::size_of::<*mut cluster_hash>() as libc::c_ulong,
        );
        workerpool_add_task(
            (*td).wp,
            Some(do_cluster_task as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            &mut *tasks.offset(ntasks as isize) as *mut cluster_task as *mut libc::c_void,
        );
        ntasks += 1;
        i += chunksize;
    }
    workerpool_run((*td).wp);
    let mut clusters_list: *mut *mut zarray_t = malloc(
        (::std::mem::size_of::<*mut zarray_t>() as libc::c_ulong)
            .wrapping_mul(ntasks as libc::c_ulong),
    ) as *mut *mut zarray_t;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < ntasks {
        let ref mut fresh70 = *clusters_list.offset(i_0 as isize);
        *fresh70 = (*tasks.offset(i_0 as isize)).clusters;
        i_0 += 1;
    }
    let mut length: libc::c_int = ntasks;
    while length > 1 as libc::c_int {
        let mut write: libc::c_int = 0 as libc::c_int;
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < length - 1 as libc::c_int {
            let ref mut fresh71 = *clusters_list.offset(write as isize);
            *fresh71 = merge_clusters(
                *clusters_list.offset(i_1 as isize),
                *clusters_list.offset((i_1 + 1 as libc::c_int) as isize),
            );
            write += 1;
            i_1 += 2 as libc::c_int;
        }
        if length % 2 as libc::c_int != 0 {
            let ref mut fresh72 = *clusters_list.offset(write as isize);
            *fresh72 = *clusters_list.offset((length - 1 as libc::c_int) as isize);
        }
        length = (length >> 1 as libc::c_int) + length % 2 as libc::c_int;
    }
    clusters = zarray_create(::std::mem::size_of::<*mut zarray_t>() as libc::c_ulong);
    zarray_ensure_capacity(
        clusters,
        zarray_size(*clusters_list.offset(0 as libc::c_int as isize)),
    );
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < zarray_size(*clusters_list.offset(0 as libc::c_int as isize)) {
        let mut hash: *mut cluster_hash = 0 as *mut cluster_hash;
        zarray_get(
            *clusters_list.offset(0 as libc::c_int as isize),
            i_2,
            &mut hash as *mut *mut cluster_hash as *mut libc::c_void,
        );
        zarray_add(
            clusters,
            &mut (*hash).data as *mut *mut zarray_t as *const libc::c_void,
        );
        free(hash as *mut libc::c_void);
        i_2 += 1;
    }
    zarray_destroy(*clusters_list.offset(0 as libc::c_int as isize));
    free(clusters_list as *mut libc::c_void);
    free(tasks as *mut libc::c_void);
    return clusters;
}
#[no_mangle]
pub unsafe extern "C" fn fit_quads(
    mut td: *mut apriltag_detector_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut clusters: *mut zarray_t,
    mut im: *mut image_u8_t,
) -> *mut zarray_t {
    let mut quads: *mut zarray_t = zarray_create(
        ::std::mem::size_of::<quad>() as libc::c_ulong,
    );
    let mut normal_border: bool = 0 as libc::c_int != 0;
    let mut reversed_border: bool = 0 as libc::c_int != 0;
    let mut min_tag_width: libc::c_int = 1000000 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < zarray_size((*td).tag_families) {
        let mut family: *mut apriltag_family_t = 0 as *mut apriltag_family_t;
        zarray_get(
            (*td).tag_families,
            i,
            &mut family as *mut *mut apriltag_family_t as *mut libc::c_void,
        );
        if (*family).width_at_border < min_tag_width {
            min_tag_width = (*family).width_at_border;
        }
        normal_border = (normal_border as libc::c_int
            | !(*family).reversed_border as libc::c_int) as bool;
        reversed_border = (reversed_border as libc::c_int
            | (*family).reversed_border as libc::c_int) as bool;
        i += 1;
    }
    min_tag_width = (min_tag_width as libc::c_float / (*td).quad_decimate)
        as libc::c_int;
    if min_tag_width < 3 as libc::c_int {
        min_tag_width = 3 as libc::c_int;
    }
    let mut sz: libc::c_int = zarray_size(clusters);
    let mut chunksize: libc::c_int = 1 as libc::c_int
        + sz / (10 as libc::c_int * (*td).nthreads);
    let mut tasks: *mut quad_task = malloc(
        (::std::mem::size_of::<quad_task>() as libc::c_ulong)
            .wrapping_mul((sz / chunksize + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut quad_task;
    let mut ntasks: libc::c_int = 0 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < sz {
        let ref mut fresh73 = (*tasks.offset(ntasks as isize)).td;
        *fresh73 = td;
        (*tasks.offset(ntasks as isize)).cidx0 = i_0;
        (*tasks.offset(ntasks as isize)).cidx1 = imin(sz, i_0 + chunksize);
        (*tasks.offset(ntasks as isize)).h = h;
        (*tasks.offset(ntasks as isize)).w = w;
        let ref mut fresh74 = (*tasks.offset(ntasks as isize)).quads;
        *fresh74 = quads;
        let ref mut fresh75 = (*tasks.offset(ntasks as isize)).clusters;
        *fresh75 = clusters;
        let ref mut fresh76 = (*tasks.offset(ntasks as isize)).im;
        *fresh76 = im;
        (*tasks.offset(ntasks as isize)).tag_width = min_tag_width;
        (*tasks.offset(ntasks as isize)).normal_border = normal_border;
        (*tasks.offset(ntasks as isize)).reversed_border = reversed_border;
        workerpool_add_task(
            (*td).wp,
            Some(do_quad_task as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            &mut *tasks.offset(ntasks as isize) as *mut quad_task as *mut libc::c_void,
        );
        ntasks += 1;
        i_0 += chunksize;
    }
    workerpool_run((*td).wp);
    free(tasks as *mut libc::c_void);
    return quads;
}
#[no_mangle]
pub unsafe extern "C" fn apriltag_quad_thresh(
    mut td: *mut apriltag_detector_t,
    mut im: *mut image_u8_t,
) -> *mut zarray_t {
    let mut w: libc::c_int = (*im).width;
    let mut h: libc::c_int = (*im).height;
    let mut threshim: *mut image_u8_t = threshold(td, im);
    let mut ts: libc::c_int = (*threshim).stride;
    if (*td).debug != 0 {
        image_u8_write_pnm(
            threshim,
            b"debug_threshold.pnm\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut uf: *mut unionfind_t = connected_components(td, threshim, w, h, ts);
    if (*td).debug != 0 {
        let mut d: *mut image_u8x3_t = image_u8x3_create(
            w as libc::c_uint,
            h as libc::c_uint,
        );
        let mut colors: *mut uint32_t = calloc(
            (w * h) as libc::c_ulong,
            ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
        ) as *mut uint32_t;
        let mut y: libc::c_int = 0 as libc::c_int;
        while y < h {
            let mut x: libc::c_int = 0 as libc::c_int;
            while x < w {
                let mut v: uint32_t = unionfind_get_representative(
                    uf,
                    (y * w + x) as uint32_t,
                );
                if !(unionfind_get_set_size(uf, v)
                    < (*td).qtp.min_cluster_pixels as libc::c_uint)
                {
                    let mut color: uint32_t = *colors.offset(v as isize);
                    let mut r: uint8_t = (color >> 16 as libc::c_int) as uint8_t;
                    let mut g: uint8_t = (color >> 8 as libc::c_int) as uint8_t;
                    let mut b: uint8_t = color as uint8_t;
                    if color == 0 as libc::c_int as libc::c_uint {
                        let bias: libc::c_int = 50 as libc::c_int;
                        r = (bias as libc::c_long
                            + random() % (200 as libc::c_int - bias) as libc::c_long)
                            as uint8_t;
                        g = (bias as libc::c_long
                            + random() % (200 as libc::c_int - bias) as libc::c_long)
                            as uint8_t;
                        b = (bias as libc::c_long
                            + random() % (200 as libc::c_int - bias) as libc::c_long)
                            as uint8_t;
                        *colors
                            .offset(
                                v as isize,
                            ) = ((r as libc::c_int) << 16 as libc::c_int
                            | (g as libc::c_int) << 8 as libc::c_int | b as libc::c_int)
                            as uint32_t;
                    }
                    *((*d).buf)
                        .offset(
                            (y * (*d).stride + 3 as libc::c_int * x + 0 as libc::c_int)
                                as isize,
                        ) = r;
                    *((*d).buf)
                        .offset(
                            (y * (*d).stride + 3 as libc::c_int * x + 1 as libc::c_int)
                                as isize,
                        ) = g;
                    *((*d).buf)
                        .offset(
                            (y * (*d).stride + 3 as libc::c_int * x + 2 as libc::c_int)
                                as isize,
                        ) = b;
                }
                x += 1;
            }
            y += 1;
        }
        free(colors as *mut libc::c_void);
        image_u8x3_write_pnm(
            d,
            b"debug_segmentation.pnm\0" as *const u8 as *const libc::c_char,
        );
        image_u8x3_destroy(d);
    }
    timeprofile_stamp((*td).tp, b"unionfind\0" as *const u8 as *const libc::c_char);
    let mut clusters: *mut zarray_t = gradient_clusters(td, threshim, w, h, ts, uf);
    if (*td).debug != 0 {
        let mut d_0: *mut image_u8x3_t = image_u8x3_create(
            w as libc::c_uint,
            h as libc::c_uint,
        );
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < zarray_size(clusters) {
            let mut cluster: *mut zarray_t = 0 as *mut zarray_t;
            zarray_get(
                clusters,
                i,
                &mut cluster as *mut *mut zarray_t as *mut libc::c_void,
            );
            let mut r_0: uint32_t = 0;
            let mut g_0: uint32_t = 0;
            let mut b_0: uint32_t = 0;
            let bias_0: libc::c_int = 50 as libc::c_int;
            r_0 = (bias_0 as libc::c_long
                + random() % (200 as libc::c_int - bias_0) as libc::c_long) as uint32_t;
            g_0 = (bias_0 as libc::c_long
                + random() % (200 as libc::c_int - bias_0) as libc::c_long) as uint32_t;
            b_0 = (bias_0 as libc::c_long
                + random() % (200 as libc::c_int - bias_0) as libc::c_long) as uint32_t;
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < zarray_size(cluster) {
                let mut p: *mut pt = 0 as *mut pt;
                zarray_get_volatile(
                    cluster,
                    j,
                    &mut p as *mut *mut pt as *mut libc::c_void,
                );
                let mut x_0: libc::c_int = (*p).x as libc::c_int / 2 as libc::c_int;
                let mut y_0: libc::c_int = (*p).y as libc::c_int / 2 as libc::c_int;
                *((*d_0).buf)
                    .offset(
                        (y_0 * (*d_0).stride + 3 as libc::c_int * x_0 + 0 as libc::c_int)
                            as isize,
                    ) = r_0 as uint8_t;
                *((*d_0).buf)
                    .offset(
                        (y_0 * (*d_0).stride + 3 as libc::c_int * x_0 + 1 as libc::c_int)
                            as isize,
                    ) = g_0 as uint8_t;
                *((*d_0).buf)
                    .offset(
                        (y_0 * (*d_0).stride + 3 as libc::c_int * x_0 + 2 as libc::c_int)
                            as isize,
                    ) = b_0 as uint8_t;
                j += 1;
            }
            i += 1;
        }
        image_u8x3_write_pnm(
            d_0,
            b"debug_clusters.pnm\0" as *const u8 as *const libc::c_char,
        );
        image_u8x3_destroy(d_0);
    }
    image_u8_destroy(threshim);
    timeprofile_stamp((*td).tp, b"make clusters\0" as *const u8 as *const libc::c_char);
    let mut quads: *mut zarray_t = fit_quads(td, w, h, clusters, im);
    if (*td).debug != 0 {
        let mut f: *mut FILE = fopen(
            b"debug_lines.ps\0" as *const u8 as *const libc::c_char,
            b"w\0" as *const u8 as *const libc::c_char,
        );
        fprintf(f, b"%%!PS\n\n\0" as *const u8 as *const libc::c_char);
        let mut im2: *mut image_u8_t = image_u8_copy(im);
        image_u8_darken(im2);
        image_u8_darken(im2);
        let mut scale: libc::c_double = fmin(
            612.0f64 / (*im).width as libc::c_double,
            792.0f64 / (*im2).height as libc::c_double,
        );
        fprintf(
            f,
            b"%.15f %.15f scale\n\0" as *const u8 as *const libc::c_char,
            scale,
            scale,
        );
        fprintf(
            f,
            b"0 %d translate\n\0" as *const u8 as *const libc::c_char,
            (*im2).height,
        );
        fprintf(f, b"1 -1 scale\n\0" as *const u8 as *const libc::c_char);
        postscript_image(f, im2);
        image_u8_destroy(im2);
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < zarray_size(quads) {
            let mut q: *mut quad = 0 as *mut quad;
            zarray_get_volatile(
                quads,
                i_0,
                &mut q as *mut *mut quad as *mut libc::c_void,
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
                b"%.15f %.15f moveto %.15f %.15f lineto %.15f %.15f lineto %.15f %.15f lineto %.15f %.15f lineto stroke\n\0"
                    as *const u8 as *const libc::c_char,
                (*q).p[0 as libc::c_int as usize][0 as libc::c_int as usize]
                    as libc::c_double,
                (*q).p[0 as libc::c_int as usize][1 as libc::c_int as usize]
                    as libc::c_double,
                (*q).p[1 as libc::c_int as usize][0 as libc::c_int as usize]
                    as libc::c_double,
                (*q).p[1 as libc::c_int as usize][1 as libc::c_int as usize]
                    as libc::c_double,
                (*q).p[2 as libc::c_int as usize][0 as libc::c_int as usize]
                    as libc::c_double,
                (*q).p[2 as libc::c_int as usize][1 as libc::c_int as usize]
                    as libc::c_double,
                (*q).p[3 as libc::c_int as usize][0 as libc::c_int as usize]
                    as libc::c_double,
                (*q).p[3 as libc::c_int as usize][1 as libc::c_int as usize]
                    as libc::c_double,
                (*q).p[0 as libc::c_int as usize][0 as libc::c_int as usize]
                    as libc::c_double,
                (*q).p[0 as libc::c_int as usize][1 as libc::c_int as usize]
                    as libc::c_double,
            );
            i_0 += 1;
        }
        fclose(f);
    }
    timeprofile_stamp(
        (*td).tp,
        b"fit quads to clusters\0" as *const u8 as *const libc::c_char,
    );
    unionfind_destroy(uf);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < zarray_size(clusters) {
        let mut cluster_0: *mut zarray_t = 0 as *mut zarray_t;
        zarray_get(
            clusters,
            i_1,
            &mut cluster_0 as *mut *mut zarray_t as *mut libc::c_void,
        );
        zarray_destroy(cluster_0);
        i_1 += 1;
    }
    zarray_destroy(clusters);
    return quads;
}
