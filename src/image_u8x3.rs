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
    fn exp(_: libc::c_double) -> libc::c_double;
    fn sqrtf(_: libc::c_float) -> libc::c_float;
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
pub struct pnm {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub format: libc::c_int,
    pub max: libc::c_int,
    pub buflen: uint32_t,
    pub buf: *mut uint8_t,
}
pub type pnm_t = pnm;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct image_u8x3 {
    pub width: int32_t,
    pub height: int32_t,
    pub stride: int32_t,
    pub buf: *mut uint8_t,
}
pub type image_u8x3_t = image_u8x3;
#[inline]
unsafe extern "C" fn sq(mut v: libc::c_double) -> libc::c_double {
    return v * v;
}
#[no_mangle]
pub unsafe extern "C" fn image_u8x3_create(
    mut width: libc::c_uint,
    mut height: libc::c_uint,
) -> *mut image_u8x3_t {
    return image_u8x3_create_alignment(
        width,
        height,
        192 as libc::c_int as libc::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn image_u8x3_create_alignment(
    mut width: libc::c_uint,
    mut height: libc::c_uint,
    mut alignment: libc::c_uint,
) -> *mut image_u8x3_t {
    let mut stride: libc::c_int = (3 as libc::c_int as libc::c_uint).wrapping_mul(width)
        as libc::c_int;
    if (stride as libc::c_uint).wrapping_rem(alignment)
        != 0 as libc::c_int as libc::c_uint
    {
        stride = (stride as libc::c_uint)
            .wrapping_add(
                alignment.wrapping_sub((stride as libc::c_uint).wrapping_rem(alignment)),
            ) as libc::c_int as libc::c_int;
    }
    let mut buf: *mut uint8_t = calloc(
        height.wrapping_mul(stride as libc::c_uint) as libc::c_ulong,
        ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
    ) as *mut uint8_t;
    let mut tmp: image_u8x3_t = {
        let mut init = image_u8x3 {
            width: width as int32_t,
            height: height as int32_t,
            stride: stride,
            buf: buf,
        };
        init
    };
    let mut im: *mut image_u8x3_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<image_u8x3_t>() as libc::c_ulong,
    ) as *mut image_u8x3_t;
    memcpy(
        im as *mut libc::c_void,
        &mut tmp as *mut image_u8x3_t as *const libc::c_void,
        ::std::mem::size_of::<image_u8x3_t>() as libc::c_ulong,
    );
    return im;
}
#[no_mangle]
pub unsafe extern "C" fn image_u8x3_copy(
    mut in_0: *const image_u8x3_t,
) -> *mut image_u8x3_t {
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
    let mut tmp: image_u8x3_t = {
        let mut init = image_u8x3 {
            width: (*in_0).width,
            height: (*in_0).height,
            stride: (*in_0).stride,
            buf: buf,
        };
        init
    };
    let mut copy: *mut image_u8x3_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<image_u8x3_t>() as libc::c_ulong,
    ) as *mut image_u8x3_t;
    memcpy(
        copy as *mut libc::c_void,
        &mut tmp as *mut image_u8x3_t as *const libc::c_void,
        ::std::mem::size_of::<image_u8x3_t>() as libc::c_ulong,
    );
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn image_u8x3_destroy(mut im: *mut image_u8x3_t) {
    if im.is_null() {
        return;
    }
    free((*im).buf as *mut libc::c_void);
    free(im as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn image_u8x3_create_from_pnm(
    mut path: *const libc::c_char,
) -> *mut image_u8x3_t {
    let mut pnm: *mut pnm_t = pnm_create_from_file(path);
    if pnm.is_null() {
        return 0 as *mut image_u8x3_t;
    }
    let mut im: *mut image_u8x3_t = 0 as *mut image_u8x3_t;
    match (*pnm).format {
        5 => {
            im = image_u8x3_create(
                (*pnm).width as libc::c_uint,
                (*pnm).height as libc::c_uint,
            );
            let mut y: libc::c_int = 0 as libc::c_int;
            while y < (*im).height {
                let mut x: libc::c_int = 0 as libc::c_int;
                while x < (*im).width {
                    let mut gray: uint8_t = *((*pnm).buf)
                        .offset((y * (*im).width + x) as isize);
                    *((*im).buf)
                        .offset(
                            (y * (*im).stride + x * 3 as libc::c_int + 0 as libc::c_int)
                                as isize,
                        ) = gray;
                    *((*im).buf)
                        .offset(
                            (y * (*im).stride + x * 3 as libc::c_int + 1 as libc::c_int)
                                as isize,
                        ) = gray;
                    *((*im).buf)
                        .offset(
                            (y * (*im).stride + x * 3 as libc::c_int + 2 as libc::c_int)
                                as isize,
                        ) = gray;
                    x += 1;
                }
                y += 1;
            }
        }
        6 => {
            im = image_u8x3_create(
                (*pnm).width as libc::c_uint,
                (*pnm).height as libc::c_uint,
            );
            let mut y_0: libc::c_int = 0 as libc::c_int;
            while y_0 < (*im).height {
                let mut x_0: libc::c_int = 0 as libc::c_int;
                while x_0 < (*im).width {
                    let mut r: uint8_t = *((*pnm).buf)
                        .offset(
                            (y_0 * (*im).width * 3 as libc::c_int
                                + 3 as libc::c_int * x_0) as isize,
                        );
                    let mut g: uint8_t = *((*pnm).buf)
                        .offset(
                            (y_0 * (*im).width * 3 as libc::c_int
                                + 3 as libc::c_int * x_0 + 1 as libc::c_int) as isize,
                        );
                    let mut b: uint8_t = *((*pnm).buf)
                        .offset(
                            (y_0 * (*im).width * 3 as libc::c_int
                                + 3 as libc::c_int * x_0 + 2 as libc::c_int) as isize,
                        );
                    *((*im).buf)
                        .offset(
                            (y_0 * (*im).stride + x_0 * 3 as libc::c_int
                                + 0 as libc::c_int) as isize,
                        ) = r;
                    *((*im).buf)
                        .offset(
                            (y_0 * (*im).stride + x_0 * 3 as libc::c_int
                                + 1 as libc::c_int) as isize,
                        ) = g;
                    *((*im).buf)
                        .offset(
                            (y_0 * (*im).stride + x_0 * 3 as libc::c_int
                                + 2 as libc::c_int) as isize,
                        ) = b;
                    x_0 += 1;
                }
                y_0 += 1;
            }
        }
        _ => {}
    }
    pnm_destroy(pnm);
    return im;
}
#[no_mangle]
pub unsafe extern "C" fn image_u8x3_write_pnm(
    mut im: *const image_u8x3_t,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut linesz: libc::c_int = 0;
    let mut f: *mut FILE = fopen(path, b"wb\0" as *const u8 as *const libc::c_char);
    let mut res: libc::c_int = 0 as libc::c_int;
    if f.is_null() {
        res = -(1 as libc::c_int);
    } else {
        fprintf(
            f,
            b"P6\n%d %d\n255\n\0" as *const u8 as *const libc::c_char,
            (*im).width,
            (*im).height,
        );
        linesz = (*im).width * 3 as libc::c_int;
        let mut y: libc::c_int = 0 as libc::c_int;
        while y < (*im).height {
            if linesz as libc::c_ulong
                != fwrite(
                    &mut *((*im).buf).offset((y * (*im).stride) as isize) as *mut uint8_t
                        as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    linesz as libc::c_ulong,
                    f,
                )
            {
                res = -(1 as libc::c_int);
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
pub unsafe extern "C" fn image_u8x3_draw_line(
    mut im: *mut image_u8x3_t,
    mut x0: libc::c_float,
    mut y0: libc::c_float,
    mut x1: libc::c_float,
    mut y1: libc::c_float,
    mut rgb: *mut uint8_t,
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
            let mut idx: libc::c_int = y * (*im).stride + 3 as libc::c_int * x;
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < 3 as libc::c_int {
                *((*im).buf).offset((idx + i) as isize) = *rgb.offset(i as isize);
                i += 1;
            }
        }
        f = (f as libc::c_double + delta) as libc::c_float;
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
pub unsafe extern "C" fn image_u8x3_gaussian_blur(
    mut im: *mut image_u8x3_t,
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
    let mut c: libc::c_int = 0 as libc::c_int;
    while c < 3 as libc::c_int {
        let mut y: libc::c_int = 0 as libc::c_int;
        while y < (*im).height {
            let mut in_0: *mut uint8_t = malloc(
                (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
                    .wrapping_mul((*im).stride as libc::c_ulong),
            ) as *mut uint8_t;
            let mut out: *mut uint8_t = malloc(
                (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
                    .wrapping_mul((*im).stride as libc::c_ulong),
            ) as *mut uint8_t;
            let mut x_0: libc::c_int = 0 as libc::c_int;
            while x_0 < (*im).width {
                *in_0
                    .offset(
                        x_0 as isize,
                    ) = *((*im).buf)
                    .offset((y * (*im).stride + 3 as libc::c_int * x_0 + c) as isize);
                x_0 += 1;
            }
            convolve(in_0, out, (*im).width, k, ksz);
            free(in_0 as *mut libc::c_void);
            let mut x_1: libc::c_int = 0 as libc::c_int;
            while x_1 < (*im).width {
                *((*im).buf)
                    .offset(
                        (y * (*im).stride + 3 as libc::c_int * x_1 + c) as isize,
                    ) = *out.offset(x_1 as isize);
                x_1 += 1;
            }
            free(out as *mut libc::c_void);
            y += 1;
        }
        let mut x_2: libc::c_int = 0 as libc::c_int;
        while x_2 < (*im).width {
            let mut in_1: *mut uint8_t = malloc(
                (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
                    .wrapping_mul((*im).height as libc::c_ulong),
            ) as *mut uint8_t;
            let mut out_0: *mut uint8_t = malloc(
                (::std::mem::size_of::<uint8_t>() as libc::c_ulong)
                    .wrapping_mul((*im).height as libc::c_ulong),
            ) as *mut uint8_t;
            let mut y_0: libc::c_int = 0 as libc::c_int;
            while y_0 < (*im).height {
                *in_1
                    .offset(
                        y_0 as isize,
                    ) = *((*im).buf)
                    .offset((y_0 * (*im).stride + 3 as libc::c_int * x_2 + c) as isize);
                y_0 += 1;
            }
            convolve(in_1, out_0, (*im).height, k, ksz);
            free(in_1 as *mut libc::c_void);
            let mut y_1: libc::c_int = 0 as libc::c_int;
            while y_1 < (*im).height {
                *((*im).buf)
                    .offset(
                        (y_1 * (*im).stride + 3 as libc::c_int * x_2 + c) as isize,
                    ) = *out_0.offset(y_1 as isize);
                y_1 += 1;
            }
            free(out_0 as *mut libc::c_void);
            x_2 += 1;
        }
        c += 1;
    }
    free(k as *mut libc::c_void);
}
