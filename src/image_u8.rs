use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fmin(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn pnm_create_from_file(path: *const libc::c_char) -> *mut pnm_t;
    fn pnm_destroy(pnm: *mut pnm_t);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
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
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
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
pub struct image_f32 {
    pub width: int32_t,
    pub height: int32_t,
    pub stride: int32_t,
    pub buf: *mut libc::c_float,
}
pub type image_f32_t = image_f32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct image_u8_lut {
    pub scale: libc::c_float,
    pub nvalues: libc::c_int,
    pub values: *mut uint8_t,
}
pub type image_u8_lut_t = image_u8_lut;
pub type pnm_t = pnm;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pnm {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub format: libc::c_int,
    pub max: libc::c_int,
    pub buflen: uint32_t,
    pub buf: *mut uint8_t,
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
unsafe extern "C" fn imax(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn iclamp(
    mut v: libc::c_int,
    mut minv: libc::c_int,
    mut maxv: libc::c_int,
) -> libc::c_int {
    return imax(minv, imin(v, maxv));
}
#[no_mangle]
pub unsafe extern "C" fn image_u8_create_stride(
    mut width: libc::c_uint,
    mut height: libc::c_uint,
    mut stride: libc::c_uint,
) -> *mut image_u8_t {
    let mut buf: *mut uint8_t = calloc(
        height.wrapping_mul(stride) as libc::c_ulong,
        ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
    ) as *mut uint8_t;
    let mut tmp: image_u8_t = {
        let mut init = image_u8 {
            width: width as int32_t,
            height: height as int32_t,
            stride: stride as int32_t,
            buf: buf,
        };
        init
    };
    let mut im: *mut image_u8_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<image_u8_t>() as libc::c_ulong,
    ) as *mut image_u8_t;
    memcpy(
        im as *mut libc::c_void,
        &mut tmp as *mut image_u8_t as *const libc::c_void,
        ::std::mem::size_of::<image_u8_t>() as libc::c_ulong,
    );
    return im;
}
#[no_mangle]
pub unsafe extern "C" fn image_u8_create(
    mut width: libc::c_uint,
    mut height: libc::c_uint,
) -> *mut image_u8_t {
    return image_u8_create_alignment(width, height, 96 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn image_u8_create_alignment(
    mut width: libc::c_uint,
    mut height: libc::c_uint,
    mut alignment: libc::c_uint,
) -> *mut image_u8_t {
    let mut stride: libc::c_int = width as libc::c_int;
    if (stride as libc::c_uint).wrapping_rem(alignment)
        != 0 as libc::c_int as libc::c_uint
    {
        stride = (stride as libc::c_uint)
            .wrapping_add(
                alignment.wrapping_sub((stride as libc::c_uint).wrapping_rem(alignment)),
            ) as libc::c_int as libc::c_int;
    }
    return image_u8_create_stride(width, height, stride as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn image_u8_copy(mut in_0: *const image_u8_t) -> *mut image_u8_t {
    let mut buf: *mut uint8_t = malloc(
        (((*in_0).height * (*in_0).stride) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
    ) as *mut uint8_t;
    memcpy(
        buf as *mut libc::c_void,
        (*in_0).buf as *const libc::c_void,
        (((*in_0).height * (*in_0).stride) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong),
    );
    let mut tmp: image_u8_t = {
        let mut init = image_u8 {
            width: (*in_0).width,
            height: (*in_0).height,
            stride: (*in_0).stride,
            buf: buf,
        };
        init
    };
    let mut copy: *mut image_u8_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<image_u8_t>() as libc::c_ulong,
    ) as *mut image_u8_t;
    memcpy(
        copy as *mut libc::c_void,
        &mut tmp as *mut image_u8_t as *const libc::c_void,
        ::std::mem::size_of::<image_u8_t>() as libc::c_ulong,
    );
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn image_u8_destroy(mut im: *mut image_u8_t) {
    if im.is_null() {
        return;
    }
    free((*im).buf as *mut libc::c_void);
    free(im as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn image_u8_create_from_pnm(
    mut path: *const libc::c_char,
) -> *mut image_u8_t {
    return image_u8_create_from_pnm_alignment(path, 96 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn image_u8_create_from_pnm_alignment(
    mut path: *const libc::c_char,
    mut alignment: libc::c_int,
) -> *mut image_u8_t {
    let mut pnm: *mut pnm_t = pnm_create_from_file(path);
    if pnm.is_null() {
        return 0 as *mut image_u8_t;
    }
    let mut im: *mut image_u8_t = 0 as *mut image_u8_t;
    match (*pnm).format {
        5 => {
            im = image_u8_create_alignment(
                (*pnm).width as libc::c_uint,
                (*pnm).height as libc::c_uint,
                alignment as libc::c_uint,
            );
            if (*pnm).max == 255 as libc::c_int {
                let mut y: libc::c_int = 0 as libc::c_int;
                while y < (*im).height {
                    memcpy(
                        &mut *((*im).buf).offset((y * (*im).stride) as isize)
                            as *mut uint8_t as *mut libc::c_void,
                        &mut *((*pnm).buf).offset((y * (*im).width) as isize)
                            as *mut uint8_t as *const libc::c_void,
                        (*im).width as libc::c_ulong,
                    );
                    y += 1;
                }
            } else if (*pnm).max == 65535 as libc::c_int {
                let mut y_0: libc::c_int = 0 as libc::c_int;
                while y_0 < (*im).height {
                    let mut x: libc::c_int = 0 as libc::c_int;
                    while x < (*im).width {
                        *((*im).buf)
                            .offset(
                                (y_0 * (*im).stride + x) as isize,
                            ) = *((*pnm).buf)
                            .offset(
                                (2 as libc::c_int * (y_0 * (*im).width + x)) as isize,
                            );
                        x += 1;
                    }
                    y_0 += 1;
                }
            }
        }
        6 => {
            im = image_u8_create_alignment(
                (*pnm).width as libc::c_uint,
                (*pnm).height as libc::c_uint,
                alignment as libc::c_uint,
            );
            if (*pnm).max == 255 as libc::c_int {
                let mut y_1: libc::c_int = 0 as libc::c_int;
                while y_1 < (*im).height {
                    let mut x_0: libc::c_int = 0 as libc::c_int;
                    while x_0 < (*im).width {
                        let mut gray: uint8_t = ((*((*pnm).buf)
                            .offset(
                                (y_1 * (*im).width * 3 as libc::c_int
                                    + 3 as libc::c_int * x_0 + 0 as libc::c_int) as isize,
                            ) as libc::c_int
                            + *((*pnm).buf)
                                .offset(
                                    (y_1 * (*im).width * 3 as libc::c_int
                                        + 3 as libc::c_int * x_0 + 1 as libc::c_int) as isize,
                                ) as libc::c_int
                            + *((*pnm).buf)
                                .offset(
                                    (y_1 * (*im).width * 3 as libc::c_int
                                        + 3 as libc::c_int * x_0 + 1 as libc::c_int) as isize,
                                ) as libc::c_int
                            + *((*pnm).buf)
                                .offset(
                                    (y_1 * (*im).width * 3 as libc::c_int
                                        + 3 as libc::c_int * x_0 + 2 as libc::c_int) as isize,
                                ) as libc::c_int) / 4 as libc::c_int) as uint8_t;
                        *((*im).buf).offset((y_1 * (*im).stride + x_0) as isize) = gray;
                        x_0 += 1;
                    }
                    y_1 += 1;
                }
            } else if (*pnm).max == 65535 as libc::c_int {
                let mut y_2: libc::c_int = 0 as libc::c_int;
                while y_2 < (*im).height {
                    let mut x_1: libc::c_int = 0 as libc::c_int;
                    while x_1 < (*im).width {
                        let mut r: libc::c_int = *((*pnm).buf)
                            .offset(
                                (6 as libc::c_int * (y_2 * (*im).width + x_1)
                                    + 0 as libc::c_int) as isize,
                            ) as libc::c_int;
                        let mut g: libc::c_int = *((*pnm).buf)
                            .offset(
                                (6 as libc::c_int * (y_2 * (*im).width + x_1)
                                    + 2 as libc::c_int) as isize,
                            ) as libc::c_int;
                        let mut b: libc::c_int = *((*pnm).buf)
                            .offset(
                                (6 as libc::c_int * (y_2 * (*im).width + x_1)
                                    + 4 as libc::c_int) as isize,
                            ) as libc::c_int;
                        *((*im).buf)
                            .offset(
                                (y_2 * (*im).stride + x_1) as isize,
                            ) = ((r + g + g + b) / 4 as libc::c_int) as uint8_t;
                        x_1 += 1;
                    }
                    y_2 += 1;
                }
            }
        }
        4 => {
            im = image_u8_create_alignment(
                (*pnm).width as libc::c_uint,
                (*pnm).height as libc::c_uint,
                alignment as libc::c_uint,
            );
            let mut pbmstride: libc::c_int = ((*im).width + 7 as libc::c_int)
                / 8 as libc::c_int;
            let mut y_3: libc::c_int = 0 as libc::c_int;
            while y_3 < (*im).height {
                let mut x_2: libc::c_int = 0 as libc::c_int;
                while x_2 < (*im).width {
                    let mut byteidx: libc::c_int = y_3 * pbmstride
                        + x_2 / 8 as libc::c_int;
                    let mut bitidx: libc::c_int = 7 as libc::c_int
                        - (x_2 & 7 as libc::c_int);
                    if *((*pnm).buf).offset(byteidx as isize) as libc::c_int >> bitidx
                        & 1 as libc::c_int != 0
                    {
                        *((*im).buf)
                            .offset(
                                (y_3 * (*im).stride + x_2) as isize,
                            ) = 0 as libc::c_int as uint8_t;
                    } else {
                        *((*im).buf)
                            .offset(
                                (y_3 * (*im).stride + x_2) as isize,
                            ) = 255 as libc::c_int as uint8_t;
                    }
                    x_2 += 1;
                }
                y_3 += 1;
            }
        }
        _ => {}
    }
    pnm_destroy(pnm);
    return im;
}
#[no_mangle]
pub unsafe extern "C" fn image_u8_create_from_f32(
    mut fim: *mut image_f32_t,
) -> *mut image_u8_t {
    let mut im: *mut image_u8_t = image_u8_create(
        (*fim).width as libc::c_uint,
        (*fim).height as libc::c_uint,
    );
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < (*fim).height {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < (*fim).width {
            let mut v: libc::c_float = *((*fim).buf)
                .offset((y * (*fim).stride + x) as isize);
            *((*im).buf)
                .offset(
                    (y * (*im).stride + x) as isize,
                ) = (255 as libc::c_int as libc::c_float * v) as libc::c_int as uint8_t;
            x += 1;
        }
        y += 1;
    }
    return im;
}
#[no_mangle]
pub unsafe extern "C" fn image_u8_write_pnm(
    mut im: *const image_u8_t,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut f: *mut FILE = fopen(path, b"wb\0" as *const u8 as *const libc::c_char);
    let mut res: libc::c_int = 0 as libc::c_int;
    if f.is_null() {
        res = -(1 as libc::c_int);
    } else {
        fprintf(
            f,
            b"P5\n%d %d\n255\n\0" as *const u8 as *const libc::c_char,
            (*im).width,
            (*im).height,
        );
        let mut y: libc::c_int = 0 as libc::c_int;
        while y < (*im).height {
            if (*im).width as libc::c_ulong
                != fwrite(
                    &mut *((*im).buf).offset((y * (*im).stride) as isize) as *mut uint8_t
                        as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    (*im).width as libc::c_ulong,
                    f,
                )
            {
                res = -(2 as libc::c_int);
                break;
            } else {
                y += 1;
            }
        }
    }
    if !f.is_null() {
        fclose(f);
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn image_u8_draw_circle(
    mut im: *mut image_u8_t,
    mut x0: libc::c_float,
    mut y0: libc::c_float,
    mut r: libc::c_float,
    mut v: libc::c_int,
) {
    r = r * r;
    let mut y: libc::c_int = (y0 - r) as libc::c_int;
    while y as libc::c_float <= y0 + r {
        let mut x: libc::c_int = (x0 - r) as libc::c_int;
        while x as libc::c_float <= x0 + r {
            let mut d: libc::c_float = (x as libc::c_float - x0)
                * (x as libc::c_float - x0)
                + (y as libc::c_float - y0) * (y as libc::c_float - y0);
            if !(d > r) {
                if x >= 0 as libc::c_int && x < (*im).width && y >= 0 as libc::c_int
                    && y < (*im).height
                {
                    let mut idx: libc::c_int = y * (*im).stride + x;
                    *((*im).buf).offset(idx as isize) = v as uint8_t;
                }
            }
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn image_u8_draw_annulus(
    mut im: *mut image_u8_t,
    mut x0: libc::c_float,
    mut y0: libc::c_float,
    mut r0: libc::c_float,
    mut r1: libc::c_float,
    mut v: libc::c_int,
) {
    r0 = r0 * r0;
    r1 = r1 * r1;
    let mut y: libc::c_int = (y0 - r1) as libc::c_int;
    while y as libc::c_float <= y0 + r1 {
        let mut x: libc::c_int = (x0 - r1) as libc::c_int;
        while x as libc::c_float <= x0 + r1 {
            let mut d: libc::c_float = (x as libc::c_float - x0)
                * (x as libc::c_float - x0)
                + (y as libc::c_float - y0) * (y as libc::c_float - y0);
            if !(d < r0 || d > r1) {
                let mut idx: libc::c_int = y * (*im).stride + x;
                *((*im).buf).offset(idx as isize) = v as uint8_t;
            }
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn image_u8_draw_line(
    mut im: *mut image_u8_t,
    mut x0: libc::c_float,
    mut y0: libc::c_float,
    mut x1: libc::c_float,
    mut y1: libc::c_float,
    mut v: libc::c_int,
    mut width: libc::c_int,
) {
    let mut dist: libc::c_double = sqrtf((y1 - y0) * (y1 - y0) + (x1 - x0) * (x1 - x0))
        as libc::c_double;
    let mut delta: libc::c_double = 0.5f64 / dist;
    let mut f: libc::c_float = 0 as libc::c_int as libc::c_float;
    while f <= 1 as libc::c_int as libc::c_float {
        let mut x: libc::c_int = (x1 + (x0 - x1) * f) as libc::c_int;
        let mut y: libc::c_int = (y1 + (y0 - y1) * f) as libc::c_int;
        if !(x < 0 as libc::c_int || y < 0 as libc::c_int || x >= (*im).width
            || y >= (*im).height)
        {
            let mut idx: libc::c_int = y * (*im).stride + x;
            *((*im).buf).offset(idx as isize) = v as uint8_t;
            if width > 1 as libc::c_int {
                *((*im).buf).offset((idx + 1 as libc::c_int) as isize) = v as uint8_t;
                *((*im).buf).offset((idx + (*im).stride) as isize) = v as uint8_t;
                *((*im).buf)
                    .offset(
                        (idx + 1 as libc::c_int + (*im).stride) as isize,
                    ) = v as uint8_t;
            }
        }
        f = (f as libc::c_double + delta) as libc::c_float;
    }
}
#[no_mangle]
pub unsafe extern "C" fn image_u8_darken(mut im: *mut image_u8_t) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < (*im).height {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < (*im).width {
            let ref mut fresh0 = *((*im).buf).offset(((*im).stride * y + x) as isize);
            *fresh0 = (*fresh0 as libc::c_int / 2 as libc::c_int) as uint8_t;
            x += 1;
        }
        y += 1;
    }
}
unsafe extern "C" fn convolve(
    mut x: *const uint8_t,
    mut y: *mut uint8_t,
    mut sz: libc::c_int,
    mut k: *const uint8_t,
    mut ksz: libc::c_int,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < ksz / 2 as libc::c_int && i < sz {
        *y.offset(i as isize) = *x.offset(i as isize);
        i += 1;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < sz - ksz {
        let mut acc: uint32_t = 0 as libc::c_int as uint32_t;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < ksz {
            acc = (acc as libc::c_uint)
                .wrapping_add(
                    (*k.offset(j as isize) as libc::c_int
                        * *x.offset((i_0 + j) as isize) as libc::c_int) as libc::c_uint,
                ) as uint32_t as uint32_t;
            j += 1;
        }
        *y
            .offset(
                (ksz / 2 as libc::c_int + i_0) as isize,
            ) = (acc >> 8 as libc::c_int) as uint8_t;
        i_0 += 1;
    }
    let mut i_1: libc::c_int = sz - ksz + ksz / 2 as libc::c_int;
    while i_1 < sz {
        *y.offset(i_1 as isize) = *x.offset(i_1 as isize);
        i_1 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn image_u8_convolve_2D(
    mut im: *mut image_u8_t,
    mut k: *const uint8_t,
    mut ksz: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < (*im).height {
        let mut x: *mut uint8_t = malloc(
            (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
                .wrapping_mul((*im).stride as libc::c_ulong),
        ) as *mut uint8_t;
        memcpy(
            x as *mut libc::c_void,
            &mut *((*im).buf).offset((y * (*im).stride) as isize) as *mut uint8_t
                as *const libc::c_void,
            (*im).stride as libc::c_ulong,
        );
        convolve(
            x,
            &mut *((*im).buf).offset((y * (*im).stride) as isize),
            (*im).width,
            k,
            ksz,
        );
        free(x as *mut libc::c_void);
        y += 1;
    }
    let mut x_0: libc::c_int = 0 as libc::c_int;
    while x_0 < (*im).width {
        let mut xb: *mut uint8_t = malloc(
            (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
                .wrapping_mul((*im).height as libc::c_ulong),
        ) as *mut uint8_t;
        let mut yb: *mut uint8_t = malloc(
            (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
                .wrapping_mul((*im).height as libc::c_ulong),
        ) as *mut uint8_t;
        let mut y_0: libc::c_int = 0 as libc::c_int;
        while y_0 < (*im).height {
            *xb
                .offset(
                    y_0 as isize,
                ) = *((*im).buf).offset((y_0 * (*im).stride + x_0) as isize);
            y_0 += 1;
        }
        convolve(xb, yb, (*im).height, k, ksz);
        free(xb as *mut libc::c_void);
        let mut y_1: libc::c_int = 0 as libc::c_int;
        while y_1 < (*im).height {
            *((*im).buf)
                .offset((y_1 * (*im).stride + x_0) as isize) = *yb.offset(y_1 as isize);
            y_1 += 1;
        }
        free(yb as *mut libc::c_void);
        x_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn image_u8_gaussian_blur(
    mut im: *mut image_u8_t,
    mut sigma: libc::c_double,
    mut ksz: libc::c_int,
) {
    if sigma == 0 as libc::c_int as libc::c_double {
        return;
    }
    let mut dk: *mut libc::c_double = malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(ksz as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < ksz {
        let mut x: libc::c_int = -ksz / 2 as libc::c_int + i;
        let mut v: libc::c_double = exp(-0.5f64 * sq(x as libc::c_double / sigma));
        *dk.offset(i as isize) = v;
        i += 1;
    }
    let mut acc: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < ksz {
        acc += *dk.offset(i_0 as isize);
        i_0 += 1;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < ksz {
        *dk.offset(i_1 as isize) /= acc;
        i_1 += 1;
    }
    let mut k: *mut uint8_t = malloc(
        (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
            .wrapping_mul(ksz as libc::c_ulong),
    ) as *mut uint8_t;
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < ksz {
        *k
            .offset(
                i_2 as isize,
            ) = (*dk.offset(i_2 as isize) * 255 as libc::c_int as libc::c_double)
            as uint8_t;
        i_2 += 1;
    }
    free(dk as *mut libc::c_void);
    image_u8_convolve_2D(im, k, ksz);
    free(k as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn image_u8_rotate(
    mut in_0: *const image_u8_t,
    mut rad: libc::c_double,
    mut pad: uint8_t,
) -> *mut image_u8_t {
    let mut iwidth: libc::c_int = (*in_0).width;
    let mut iheight: libc::c_int = (*in_0).height;
    rad = -rad;
    let mut c: libc::c_float = cos(rad) as libc::c_float;
    let mut s: libc::c_float = sin(rad) as libc::c_float;
    let mut p: [[libc::c_float; 2]; 4] = [
        [0 as libc::c_int as libc::c_float, 0 as libc::c_int as libc::c_float],
        [iwidth as libc::c_float, 0 as libc::c_int as libc::c_float],
        [iwidth as libc::c_float, iheight as libc::c_float],
        [0 as libc::c_int as libc::c_float, iheight as libc::c_float],
    ];
    let mut xmin: libc::c_float = ::std::f32::INFINITY;
    let mut xmax: libc::c_float = -::std::f32::INFINITY;
    let mut ymin: libc::c_float = ::std::f32::INFINITY;
    let mut ymax: libc::c_float = -::std::f32::INFINITY;
    let mut icx: libc::c_float = (iwidth as libc::c_double / 2.0f64) as libc::c_float;
    let mut icy: libc::c_float = (iheight as libc::c_double / 2.0f64) as libc::c_float;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let mut px: libc::c_float = p[i as usize][0 as libc::c_int as usize] - icx;
        let mut py: libc::c_float = p[i as usize][1 as libc::c_int as usize] - icy;
        let mut nx: libc::c_float = px * c - py * s;
        let mut ny: libc::c_float = px * s + py * c;
        xmin = fmin(xmin as libc::c_double, nx as libc::c_double) as libc::c_float;
        xmax = fmax(xmax as libc::c_double, nx as libc::c_double) as libc::c_float;
        ymin = fmin(ymin as libc::c_double, ny as libc::c_double) as libc::c_float;
        ymax = fmax(ymax as libc::c_double, ny as libc::c_double) as libc::c_float;
        i += 1;
    }
    let mut owidth: libc::c_int = ceil((xmax - xmin) as libc::c_double) as libc::c_int;
    let mut oheight: libc::c_int = ceil((ymax - ymin) as libc::c_double) as libc::c_int;
    let mut out: *mut image_u8_t = image_u8_create(
        owidth as libc::c_uint,
        oheight as libc::c_uint,
    );
    let mut oy: libc::c_int = 0 as libc::c_int;
    while oy < oheight {
        let mut ox: libc::c_int = 0 as libc::c_int;
        while ox < owidth {
            let mut sx: libc::c_float = (ox as libc::c_double
                - owidth as libc::c_double / 2.0f64 + 0.5f64) as libc::c_float;
            let mut sy: libc::c_float = (oy as libc::c_double
                - oheight as libc::c_double / 2.0f64 + 0.5f64) as libc::c_float;
            let mut ix: libc::c_int = floor((sx * c + sy * s + icx) as libc::c_double)
                as libc::c_int;
            let mut iy: libc::c_int = floor((-sx * s + sy * c + icy) as libc::c_double)
                as libc::c_int;
            if ix >= 0 as libc::c_int && iy >= 0 as libc::c_int && ix < iwidth
                && iy < iheight
            {
                *((*out).buf)
                    .offset(
                        (oy * (*out).stride + ox) as isize,
                    ) = *((*in_0).buf).offset((iy * (*in_0).stride + ix) as isize);
            } else {
                *((*out).buf).offset((oy * (*out).stride + ox) as isize) = pad;
            }
            ox += 1;
        }
        oy += 1;
    }
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn image_u8_decimate(
    mut im: *mut image_u8_t,
    mut ffactor: libc::c_float,
) -> *mut image_u8_t {
    let mut width: libc::c_int = (*im).width;
    let mut height: libc::c_int = (*im).height;
    if ffactor as libc::c_double == 1.5f64 {
        let mut swidth: libc::c_int = width / 3 as libc::c_int * 2 as libc::c_int;
        let mut sheight: libc::c_int = height / 3 as libc::c_int * 2 as libc::c_int;
        let mut decim: *mut image_u8_t = image_u8_create(
            swidth as libc::c_uint,
            sheight as libc::c_uint,
        );
        let mut y: libc::c_int = 0 as libc::c_int;
        let mut sy: libc::c_int = 0 as libc::c_int;
        while sy < sheight {
            let mut x: libc::c_int = 0 as libc::c_int;
            let mut sx: libc::c_int = 0 as libc::c_int;
            while sx < swidth {
                let mut a: uint8_t = *((*im).buf)
                    .offset(
                        ((y + 0 as libc::c_int) * (*im).stride + (x + 0 as libc::c_int))
                            as isize,
                    );
                let mut b: uint8_t = *((*im).buf)
                    .offset(
                        ((y + 0 as libc::c_int) * (*im).stride + (x + 1 as libc::c_int))
                            as isize,
                    );
                let mut c: uint8_t = *((*im).buf)
                    .offset(
                        ((y + 0 as libc::c_int) * (*im).stride + (x + 2 as libc::c_int))
                            as isize,
                    );
                let mut d: uint8_t = *((*im).buf)
                    .offset(
                        ((y + 1 as libc::c_int) * (*im).stride + (x + 0 as libc::c_int))
                            as isize,
                    );
                let mut e: uint8_t = *((*im).buf)
                    .offset(
                        ((y + 1 as libc::c_int) * (*im).stride + (x + 1 as libc::c_int))
                            as isize,
                    );
                let mut f: uint8_t = *((*im).buf)
                    .offset(
                        ((y + 1 as libc::c_int) * (*im).stride + (x + 2 as libc::c_int))
                            as isize,
                    );
                let mut g: uint8_t = *((*im).buf)
                    .offset(
                        ((y + 2 as libc::c_int) * (*im).stride + (x + 0 as libc::c_int))
                            as isize,
                    );
                let mut h: uint8_t = *((*im).buf)
                    .offset(
                        ((y + 2 as libc::c_int) * (*im).stride + (x + 1 as libc::c_int))
                            as isize,
                    );
                let mut i: uint8_t = *((*im).buf)
                    .offset(
                        ((y + 2 as libc::c_int) * (*im).stride + (x + 2 as libc::c_int))
                            as isize,
                    );
                *((*decim).buf)
                    .offset(
                        ((sy + 0 as libc::c_int) * (*decim).stride
                            + (sx + 0 as libc::c_int)) as isize,
                    ) = ((4 as libc::c_int * a as libc::c_int
                    + 2 as libc::c_int * b as libc::c_int
                    + 2 as libc::c_int * d as libc::c_int + e as libc::c_int)
                    / 9 as libc::c_int) as uint8_t;
                *((*decim).buf)
                    .offset(
                        ((sy + 0 as libc::c_int) * (*decim).stride
                            + (sx + 1 as libc::c_int)) as isize,
                    ) = ((4 as libc::c_int * c as libc::c_int
                    + 2 as libc::c_int * b as libc::c_int
                    + 2 as libc::c_int * f as libc::c_int + e as libc::c_int)
                    / 9 as libc::c_int) as uint8_t;
                *((*decim).buf)
                    .offset(
                        ((sy + 1 as libc::c_int) * (*decim).stride
                            + (sx + 0 as libc::c_int)) as isize,
                    ) = ((4 as libc::c_int * g as libc::c_int
                    + 2 as libc::c_int * d as libc::c_int
                    + 2 as libc::c_int * h as libc::c_int + e as libc::c_int)
                    / 9 as libc::c_int) as uint8_t;
                *((*decim).buf)
                    .offset(
                        ((sy + 1 as libc::c_int) * (*decim).stride
                            + (sx + 1 as libc::c_int)) as isize,
                    ) = ((4 as libc::c_int * i as libc::c_int
                    + 2 as libc::c_int * f as libc::c_int
                    + 2 as libc::c_int * h as libc::c_int + e as libc::c_int)
                    / 9 as libc::c_int) as uint8_t;
                x += 3 as libc::c_int;
                sx += 2 as libc::c_int;
            }
            y += 3 as libc::c_int;
            sy += 2 as libc::c_int;
        }
        return decim;
    }
    let mut factor: libc::c_int = ffactor as libc::c_int;
    let mut swidth_0: libc::c_int = 1 as libc::c_int
        + (width - 1 as libc::c_int) / factor;
    let mut sheight_0: libc::c_int = 1 as libc::c_int
        + (height - 1 as libc::c_int) / factor;
    let mut decim_0: *mut image_u8_t = image_u8_create(
        swidth_0 as libc::c_uint,
        sheight_0 as libc::c_uint,
    );
    let mut sy_0: libc::c_int = 0 as libc::c_int;
    let mut y_0: libc::c_int = 0 as libc::c_int;
    while y_0 < height {
        let mut sx_0: libc::c_int = 0 as libc::c_int;
        let mut x_0: libc::c_int = 0 as libc::c_int;
        while x_0 < width {
            *((*decim_0).buf)
                .offset(
                    (sy_0 * (*decim_0).stride + sx_0) as isize,
                ) = *((*im).buf).offset((y_0 * (*im).stride + x_0) as isize);
            sx_0 += 1;
            x_0 += factor;
        }
        sy_0 += 1;
        y_0 += factor;
    }
    return decim_0;
}
#[no_mangle]
pub unsafe extern "C" fn image_u8_fill_line_max(
    mut im: *mut image_u8_t,
    mut lut: *const image_u8_lut_t,
    mut xy0: *const libc::c_float,
    mut xy1: *const libc::c_float,
) {
    let mut max_dist2: libc::c_float = ((*lut).nvalues - 1 as libc::c_int)
        as libc::c_float / (*lut).scale;
    let mut max_dist: libc::c_float = sqrt(max_dist2 as libc::c_double) as libc::c_float;
    let mut theta: libc::c_double = atan2(
        (*xy1.offset(1 as libc::c_int as isize) - *xy0.offset(1 as libc::c_int as isize))
            as libc::c_double,
        (*xy1.offset(0 as libc::c_int as isize) - *xy0.offset(0 as libc::c_int as isize))
            as libc::c_double,
    );
    let mut v: libc::c_double = sin(theta);
    let mut u: libc::c_double = cos(theta);
    let mut ix0: libc::c_int = iclamp(
        (fmin(
            *xy0.offset(0 as libc::c_int as isize) as libc::c_double,
            *xy1.offset(0 as libc::c_int as isize) as libc::c_double,
        ) - max_dist as libc::c_double) as libc::c_int,
        0 as libc::c_int,
        (*im).width - 1 as libc::c_int,
    );
    let mut ix1: libc::c_int = iclamp(
        (fmax(
            *xy0.offset(0 as libc::c_int as isize) as libc::c_double,
            *xy1.offset(0 as libc::c_int as isize) as libc::c_double,
        ) + max_dist as libc::c_double) as libc::c_int,
        0 as libc::c_int,
        (*im).width - 1 as libc::c_int,
    );
    let mut iy0: libc::c_int = iclamp(
        (fmin(
            *xy0.offset(1 as libc::c_int as isize) as libc::c_double,
            *xy1.offset(1 as libc::c_int as isize) as libc::c_double,
        ) - max_dist as libc::c_double) as libc::c_int,
        0 as libc::c_int,
        (*im).height - 1 as libc::c_int,
    );
    let mut iy1: libc::c_int = iclamp(
        (fmax(
            *xy0.offset(1 as libc::c_int as isize) as libc::c_double,
            *xy1.offset(1 as libc::c_int as isize) as libc::c_double,
        ) + max_dist as libc::c_double) as libc::c_int,
        0 as libc::c_int,
        (*im).height - 1 as libc::c_int,
    );
    let mut xy1_line_coord: libc::c_float = ((*xy1.offset(0 as libc::c_int as isize)
        - *xy0.offset(0 as libc::c_int as isize)) as libc::c_double * u
        + (*xy1.offset(1 as libc::c_int as isize)
            - *xy0.offset(1 as libc::c_int as isize)) as libc::c_double * v)
        as libc::c_float;
    let mut min_line_coord: libc::c_float = fmin(
        0 as libc::c_int as libc::c_double,
        xy1_line_coord as libc::c_double,
    ) as libc::c_float;
    let mut max_line_coord: libc::c_float = fmax(
        0 as libc::c_int as libc::c_double,
        xy1_line_coord as libc::c_double,
    ) as libc::c_float;
    let mut iy: libc::c_int = iy0;
    while iy <= iy1 {
        let mut y: libc::c_float = (iy as libc::c_double + 0.5f64) as libc::c_float;
        let mut ix: libc::c_int = ix0;
        while ix <= ix1 {
            let mut x: libc::c_float = (ix as libc::c_double + 0.5f64) as libc::c_float;
            let mut line_coord: libc::c_float = ((x
                - *xy0.offset(0 as libc::c_int as isize)) as libc::c_double * u
                + (y - *xy0.offset(1 as libc::c_int as isize)) as libc::c_double * v)
                as libc::c_float;
            if line_coord < min_line_coord {
                line_coord = min_line_coord;
            } else if line_coord > max_line_coord {
                line_coord = max_line_coord;
            }
            let mut px: libc::c_float = (*xy0.offset(0 as libc::c_int as isize)
                as libc::c_double + line_coord as libc::c_double * u) as libc::c_float;
            let mut py: libc::c_float = (*xy0.offset(1 as libc::c_int as isize)
                as libc::c_double + line_coord as libc::c_double * v) as libc::c_float;
            let mut dist2: libc::c_double = ((x - px) * (x - px) + (y - py) * (y - py))
                as libc::c_double;
            let mut idx: libc::c_int = (dist2 * (*lut).scale as libc::c_double)
                as libc::c_int;
            if !(idx >= (*lut).nvalues) {
                let mut lut_value: uint8_t = *((*lut).values).offset(idx as isize);
                let mut old_value: uint8_t = *((*im).buf)
                    .offset((iy * (*im).stride + ix) as isize);
                if lut_value as libc::c_int > old_value as libc::c_int {
                    *((*im).buf).offset((iy * (*im).stride + ix) as isize) = lut_value;
                }
            }
            ix += 1;
        }
        iy += 1;
    }
}
