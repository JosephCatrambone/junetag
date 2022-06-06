use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn pam_create_from_file(inpath: *const libc::c_char) -> *mut pam_t;
    fn pam_destroy(pam: *mut pam_t);
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
pub struct pam {
    pub type_0: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub depth: libc::c_int,
    pub maxval: libc::c_int,
    pub datalen: libc::c_int,
    pub data: *mut uint8_t,
}
pub type pam_t = pam;
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
pub struct image_u8x4 {
    pub width: int32_t,
    pub height: int32_t,
    pub stride: int32_t,
    pub buf: *mut uint8_t,
}
pub type image_u8x4_t = image_u8x4;
#[no_mangle]
pub unsafe extern "C" fn image_u8x4_create(
    mut width: libc::c_uint,
    mut height: libc::c_uint,
) -> *mut image_u8x4_t {
    return image_u8x4_create_alignment(width, height, 64 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn image_u8x4_create_alignment(
    mut width: libc::c_uint,
    mut height: libc::c_uint,
    mut alignment: libc::c_uint,
) -> *mut image_u8x4_t {
    let mut stride: libc::c_int = (4 as libc::c_int as libc::c_uint).wrapping_mul(width)
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
    let mut tmp: image_u8x4_t = {
        let mut init = image_u8x4 {
            width: width as int32_t,
            height: height as int32_t,
            stride: stride,
            buf: buf,
        };
        init
    };
    let mut im: *mut image_u8x4_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<image_u8x4_t>() as libc::c_ulong,
    ) as *mut image_u8x4_t;
    memcpy(
        im as *mut libc::c_void,
        &mut tmp as *mut image_u8x4_t as *const libc::c_void,
        ::std::mem::size_of::<image_u8x4_t>() as libc::c_ulong,
    );
    return im;
}
#[no_mangle]
pub unsafe extern "C" fn image_u8x4_copy(
    mut in_0: *const image_u8x4_t,
) -> *mut image_u8x4_t {
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
    let mut tmp: image_u8x4_t = {
        let mut init = image_u8x4 {
            width: (*in_0).width,
            height: (*in_0).height,
            stride: (*in_0).stride,
            buf: buf,
        };
        init
    };
    let mut copy: *mut image_u8x4_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<image_u8x4_t>() as libc::c_ulong,
    ) as *mut image_u8x4_t;
    memcpy(
        copy as *mut libc::c_void,
        &mut tmp as *mut image_u8x4_t as *const libc::c_void,
        ::std::mem::size_of::<image_u8x4_t>() as libc::c_ulong,
    );
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn image_u8x4_destroy(mut im: *mut image_u8x4_t) {
    if im.is_null() {
        return;
    }
    free((*im).buf as *mut libc::c_void);
    free(im as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn image_u8x4_create_from_pam(
    mut inpath: *const libc::c_char,
) -> *mut image_u8x4_t {
    let mut pam: *mut pam_t = pam_create_from_file(inpath);
    if pam.is_null() {
        return 0 as *mut image_u8x4_t;
    }
    let mut im: *mut image_u8x4_t = image_u8x4_create(
        (*pam).width as libc::c_uint,
        (*pam).height as libc::c_uint,
    );
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < (*pam).height {
        if (*pam).depth == 1 as libc::c_int {
            let mut x: libc::c_int = 0 as libc::c_int;
            while x < (*pam).width {
                *((*im).buf)
                    .offset(
                        (y * (*im).stride + 4 as libc::c_int * x + 0 as libc::c_int)
                            as isize,
                    ) = *((*pam).data)
                    .offset(((*pam).width * y + x + 0 as libc::c_int) as isize);
                *((*im).buf)
                    .offset(
                        (y * (*im).stride + 4 as libc::c_int * x + 1 as libc::c_int)
                            as isize,
                    ) = *((*pam).data)
                    .offset(((*pam).width * y + x + 0 as libc::c_int) as isize);
                *((*im).buf)
                    .offset(
                        (y * (*im).stride + 4 as libc::c_int * x + 2 as libc::c_int)
                            as isize,
                    ) = *((*pam).data)
                    .offset(((*pam).width * y + x + 0 as libc::c_int) as isize);
                *((*im).buf)
                    .offset(
                        (y * (*im).stride + 4 as libc::c_int * x + 3 as libc::c_int)
                            as isize,
                    ) = 255 as libc::c_int as uint8_t;
                x += 1;
            }
        } else if (*pam).depth == 3 as libc::c_int {
            let mut x_0: libc::c_int = 0 as libc::c_int;
            while x_0 < (*pam).width {
                *((*im).buf)
                    .offset(
                        (y * (*im).stride + 4 as libc::c_int * x_0 + 0 as libc::c_int)
                            as isize,
                    ) = *((*pam).data)
                    .offset(
                        (3 as libc::c_int * (*pam).width * y + 3 as libc::c_int * x_0
                            + 0 as libc::c_int) as isize,
                    );
                *((*im).buf)
                    .offset(
                        (y * (*im).stride + 4 as libc::c_int * x_0 + 1 as libc::c_int)
                            as isize,
                    ) = *((*pam).data)
                    .offset(
                        (3 as libc::c_int * (*pam).width * y + 3 as libc::c_int * x_0
                            + 1 as libc::c_int) as isize,
                    );
                *((*im).buf)
                    .offset(
                        (y * (*im).stride + 4 as libc::c_int * x_0 + 2 as libc::c_int)
                            as isize,
                    ) = *((*pam).data)
                    .offset(
                        (3 as libc::c_int * (*pam).width * y + 3 as libc::c_int * x_0
                            + 2 as libc::c_int) as isize,
                    );
                *((*im).buf)
                    .offset(
                        (y * (*im).stride + 4 as libc::c_int * x_0 + 3 as libc::c_int)
                            as isize,
                    ) = 255 as libc::c_int as uint8_t;
                x_0 += 1;
            }
        } else if (*pam).depth == 4 as libc::c_int {
            memcpy(
                &mut *((*im).buf).offset((y * (*im).stride) as isize) as *mut uint8_t
                    as *mut libc::c_void,
                &mut *((*pam).data)
                    .offset((4 as libc::c_int * (*pam).width * y) as isize)
                    as *mut uint8_t as *const libc::c_void,
                (4 as libc::c_int * (*pam).width) as libc::c_ulong,
            );
        }
        y += 1;
    }
    pam_destroy(pam);
    return im;
}
#[no_mangle]
pub unsafe extern "C" fn image_u8x4_create_from_pnm(
    mut path: *const libc::c_char,
) -> *mut image_u8x4_t {
    let mut pnmp: *mut pnm_t = pnm_create_from_file(path);
    if pnmp.is_null() {
        return 0 as *mut image_u8x4_t;
    }
    let mut pnm: pnm_t = *pnmp;
    let mut imp: *mut image_u8x4_t = 0 as *mut image_u8x4_t;
    match pnm.format {
        5 => {
            imp = image_u8x4_create(
                pnm.width as libc::c_uint,
                pnm.height as libc::c_uint,
            );
            let im: image_u8x4_t = *imp;
            let mut y: libc::c_int = 0 as libc::c_int;
            while y < im.height {
                let mut x: libc::c_int = 0 as libc::c_int;
                while x < im.width {
                    let mut gray: uint8_t = *(pnm.buf)
                        .offset((y * pnm.width + x) as isize);
                    *(im.buf)
                        .offset(
                            (y * im.stride + 4 as libc::c_int * x + 0 as libc::c_int)
                                as isize,
                        ) = gray;
                    *(im.buf)
                        .offset(
                            (y * im.stride + 4 as libc::c_int * x + 1 as libc::c_int)
                                as isize,
                        ) = gray;
                    *(im.buf)
                        .offset(
                            (y * im.stride + 4 as libc::c_int * x + 2 as libc::c_int)
                                as isize,
                        ) = gray;
                    *(im.buf)
                        .offset(
                            (y * im.stride + 4 as libc::c_int * x + 3 as libc::c_int)
                                as isize,
                        ) = 0xff as libc::c_int as uint8_t;
                    x += 1;
                }
                y += 1;
            }
        }
        6 => {
            imp = image_u8x4_create(
                pnm.width as libc::c_uint,
                pnm.height as libc::c_uint,
            );
            let im_0: image_u8x4_t = *imp;
            let mut y_0: libc::c_int = 0 as libc::c_int;
            while y_0 < im_0.height {
                let mut x_0: libc::c_int = 0 as libc::c_int;
                while x_0 < im_0.width {
                    let mut r: uint8_t = *(pnm.buf)
                        .offset(
                            (y_0 * pnm.width * 3 as libc::c_int + 3 as libc::c_int * x_0
                                + 0 as libc::c_int) as isize,
                        );
                    let mut g: uint8_t = *(pnm.buf)
                        .offset(
                            (y_0 * pnm.width * 3 as libc::c_int + 3 as libc::c_int * x_0
                                + 1 as libc::c_int) as isize,
                        );
                    let mut b: uint8_t = *(pnm.buf)
                        .offset(
                            (y_0 * pnm.width * 3 as libc::c_int + 3 as libc::c_int * x_0
                                + 2 as libc::c_int) as isize,
                        );
                    *(im_0.buf)
                        .offset(
                            (y_0 * im_0.stride + 4 as libc::c_int * x_0
                                + 0 as libc::c_int) as isize,
                        ) = r;
                    *(im_0.buf)
                        .offset(
                            (y_0 * im_0.stride + 4 as libc::c_int * x_0
                                + 1 as libc::c_int) as isize,
                        ) = g;
                    *(im_0.buf)
                        .offset(
                            (y_0 * im_0.stride + 4 as libc::c_int * x_0
                                + 2 as libc::c_int) as isize,
                        ) = b;
                    *(im_0.buf)
                        .offset(
                            (y_0 * im_0.stride + 4 as libc::c_int * x_0
                                + 3 as libc::c_int) as isize,
                        ) = 0xff as libc::c_int as uint8_t;
                    x_0 += 1;
                }
                y_0 += 1;
            }
        }
        _ => {}
    }
    pnm_destroy(pnmp);
    return imp;
}
#[no_mangle]
pub unsafe extern "C" fn image_u8x4_write_pnm(
    mut imp: *const image_u8x4_t,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let im: image_u8x4_t = *imp;
    let mut f: *mut FILE = fopen(path, b"wb\0" as *const u8 as *const libc::c_char);
    let mut res: libc::c_int = 0 as libc::c_int;
    if f.is_null() {
        res = -(1 as libc::c_int);
    } else {
        fprintf(
            f,
            b"P6\n%d %d\n255\n\0" as *const u8 as *const libc::c_char,
            im.width,
            im.height,
        );
        let mut y: libc::c_int = im.height - 1 as libc::c_int;
        while y >= 0 as libc::c_int {
            let mut x: libc::c_int = 0 as libc::c_int;
            while x < im.width {
                let mut r: uint8_t = *(im.buf)
                    .offset(
                        (y * im.stride + 4 as libc::c_int * x + 0 as libc::c_int)
                            as isize,
                    );
                let mut g: uint8_t = *(im.buf)
                    .offset(
                        (y * im.stride + 4 as libc::c_int * x + 1 as libc::c_int)
                            as isize,
                    );
                let mut b: uint8_t = *(im.buf)
                    .offset(
                        (y * im.stride + 4 as libc::c_int * x + 2 as libc::c_int)
                            as isize,
                    );
                fwrite(
                    &mut r as *mut uint8_t as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    1 as libc::c_int as libc::c_ulong,
                    f,
                );
                fwrite(
                    &mut g as *mut uint8_t as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    1 as libc::c_int as libc::c_ulong,
                    f,
                );
                fwrite(
                    &mut b as *mut uint8_t as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    1 as libc::c_int as libc::c_ulong,
                    f,
                );
                x += 1;
            }
            y -= 1;
        }
    }
    if !f.is_null() {
        fclose(f);
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn image_u8x4_write_pam(
    mut im: *const image_u8x4_t,
    mut path: *const libc::c_char,
) {
    let mut f: *mut FILE = fopen(path, b"w\0" as *const u8 as *const libc::c_char);
    fprintf(f, b"P7\n\0" as *const u8 as *const libc::c_char);
    fprintf(f, b"WIDTH %d\n\0" as *const u8 as *const libc::c_char, (*im).width);
    fprintf(f, b"HEIGHT %d\n\0" as *const u8 as *const libc::c_char, (*im).height);
    fprintf(f, b"DEPTH 4\n\0" as *const u8 as *const libc::c_char);
    fprintf(f, b"MAXVAL 255\n\0" as *const u8 as *const libc::c_char);
    fprintf(f, b"TUPLTYPE RGB_ALPHA\n\0" as *const u8 as *const libc::c_char);
    fprintf(f, b"ENDHDR\n\0" as *const u8 as *const libc::c_char);
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < (*im).height {
        fwrite(
            &mut *((*im).buf).offset((y * (*im).stride) as isize) as *mut uint8_t
                as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            (4 as libc::c_int * (*im).width) as libc::c_ulong,
            f,
        );
        y += 1;
    }
    fclose(f);
}
