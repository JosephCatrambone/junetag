use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
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
pub type uint8_t = __uint8_t;
pub type C2RustUnnamed = libc::c_uint;
pub const PAM_GRAYSCALE: C2RustUnnamed = 5003;
pub const PAM_RGB: C2RustUnnamed = 5002;
pub const PAM_RGB_ALPHA: C2RustUnnamed = 5001;
pub const PAM_GRAYSCALE_ALPHA: C2RustUnnamed = 5000;
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
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pam_create_from_file(
    mut inpath: *const libc::c_char,
) -> *mut pam_t {
    let mut current_block: u64;
    let mut infile: *mut FILE = fopen(
        inpath,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if infile.is_null() {
        printf(
            b"pam.c: couldn't open input file: %s\n\0" as *const u8
                as *const libc::c_char,
            inpath,
        );
        return 0 as *mut pam_t;
    }
    let mut pam: *mut pam_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<pam_t>() as libc::c_ulong,
    ) as *mut pam_t;
    (*pam).width = -(1 as libc::c_int);
    (*pam).height = -(1 as libc::c_int);
    (*pam).depth = -(1 as libc::c_int);
    (*pam).maxval = -(1 as libc::c_int);
    (*pam).type_0 = -(1 as libc::c_int);
    let mut linenumber: libc::c_int = 0 as libc::c_int;
    loop {
        let mut line: [libc::c_char; 1024] = [0; 1024];
        if (fgets(
            line.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                as libc::c_int,
            infile,
        ))
            .is_null()
        {
            printf(b"pam.c: unexpected EOF\n\0" as *const u8 as *const libc::c_char);
            current_block = 16478091051250932223;
            break;
        } else {
            linenumber += 1;
            let mut tok0: *mut libc::c_char = line.as_mut_ptr();
            let mut tok1: *mut libc::c_char = 0 as *mut libc::c_char;
            if line[0 as libc::c_int as usize] as libc::c_int == '#' as i32 {
                continue;
            }
            let mut linelen: size_t = strlen(line.as_mut_ptr());
            let mut idx: libc::c_int = 0 as libc::c_int;
            while (idx as libc::c_ulong) < linelen {
                if line[idx as usize] as libc::c_int == ' ' as i32 {
                    line[idx as usize] = 0 as libc::c_int as libc::c_char;
                    if !tok1.is_null() {
                        printf(
                            b"pam.c: More than two tokens, %s:%d\n\0" as *const u8
                                as *const libc::c_char,
                            inpath,
                            linenumber,
                        );
                    }
                    tok1 = &mut *line
                        .as_mut_ptr()
                        .offset((idx + 1 as libc::c_int) as isize) as *mut libc::c_char;
                }
                if line[idx as usize] as libc::c_int == '\n' as i32 {
                    line[idx as usize] = 0 as libc::c_int as libc::c_char;
                }
                idx += 1;
            }
            if strcmp(tok0, b"P7\0" as *const u8 as *const libc::c_char) == 0 {
                continue;
            }
            if strcmp(tok0, b"ENDHDR\0" as *const u8 as *const libc::c_char) == 0 {
                current_block = 2500484646272006982;
                break;
            }
            if strcmp(tok0, b"WIDTH\0" as *const u8 as *const libc::c_char) == 0
                && !tok1.is_null()
            {
                (*pam).width = atoi(tok1);
            } else if strcmp(tok0, b"HEIGHT\0" as *const u8 as *const libc::c_char) == 0
                    && !tok1.is_null()
                {
                (*pam).height = atoi(tok1);
            } else if strcmp(tok0, b"DEPTH\0" as *const u8 as *const libc::c_char) == 0
                    && !tok1.is_null()
                {
                (*pam).depth = atoi(tok1);
            } else if strcmp(tok0, b"MAXVAL\0" as *const u8 as *const libc::c_char) == 0
                    && !tok1.is_null()
                {
                (*pam).maxval = atoi(tok1);
            } else if strcmp(tok0, b"TUPLTYPE\0" as *const u8 as *const libc::c_char)
                    == 0 && !tok1.is_null()
                {
                if strcmp(tok1, b"GRAYSCALE_ALPHA\0" as *const u8 as *const libc::c_char)
                    == 0
                {
                    (*pam).type_0 = PAM_GRAYSCALE_ALPHA as libc::c_int;
                } else if strcmp(
                        tok1,
                        b"RGB_ALPHA\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                    (*pam).type_0 = PAM_RGB_ALPHA as libc::c_int;
                } else if strcmp(tok1, b"RGB\0" as *const u8 as *const libc::c_char) == 0
                    {
                    (*pam).type_0 = PAM_RGB as libc::c_int;
                } else if strcmp(
                        tok1,
                        b"GRAYSCALE\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                    (*pam).type_0 = PAM_GRAYSCALE as libc::c_int;
                } else {
                    printf(
                        b"pam.c: unrecognized tupl type %s\n\0" as *const u8
                            as *const libc::c_char,
                        tok1,
                    );
                }
            } else {
                printf(
                    b"pam.c: unrecognized attribute %s\n\0" as *const u8
                        as *const libc::c_char,
                    tok0,
                );
            }
        }
    }
    match current_block {
        2500484646272006982 => {
            if (*pam).width < 0 as libc::c_int || (*pam).height < 0 as libc::c_int
                || (*pam).depth < 0 as libc::c_int || (*pam).maxval < 0 as libc::c_int
                || (*pam).type_0 < 0 as libc::c_int
            {
                printf(
                    b"pam.c: missing required metadata field\n\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                (*pam).datalen = (*pam).width * (*pam).height * (*pam).depth;
                let ref mut fresh0 = (*pam).data;
                *fresh0 = malloc((*pam).datalen as libc::c_ulong) as *mut uint8_t;
                if (*pam).datalen as libc::c_ulong
                    != fread(
                        (*pam).data as *mut libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        (*pam).datalen as libc::c_ulong,
                        infile,
                    )
                {
                    printf(
                        b"pam.c: couldn't read body\n\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    fclose(infile);
                    return pam;
                }
            }
        }
        _ => {}
    }
    free(pam as *mut libc::c_void);
    fclose(infile);
    return 0 as *mut pam_t;
}
#[no_mangle]
pub unsafe extern "C" fn pam_write_file(
    mut pam: *mut pam_t,
    mut outpath: *const libc::c_char,
) -> libc::c_int {
    let mut f: *mut FILE = fopen(outpath, b"w+\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        return -(1 as libc::c_int);
    }
    let mut tupl: *const libc::c_char = 0 as *const libc::c_char;
    match (*pam).type_0 {
        5000 => {
            tupl = b"GRAYSCALE_ALPHA\0" as *const u8 as *const libc::c_char;
        }
        5001 => {
            tupl = b"RGB_ALPHA\0" as *const u8 as *const libc::c_char;
        }
        5002 => {
            tupl = b"RGB\0" as *const u8 as *const libc::c_char;
        }
        5003 => {
            tupl = b"GRAYSCALE\0" as *const u8 as *const libc::c_char;
        }
        _ => {}
    }
    fprintf(
        f,
        b"P7\nWIDTH %d\nHEIGHT %d\nDEPTH %d\nMAXVAL %d\nTUPLTYPE %s\nENDHDR\n\0"
            as *const u8 as *const libc::c_char,
        (*pam).width,
        (*pam).height,
        (*pam).depth,
        (*pam).maxval,
        tupl,
    );
    let mut len: libc::c_int = (*pam).width * (*pam).height * (*pam).depth;
    if len as libc::c_ulong
        != fwrite(
            (*pam).data as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            len as libc::c_ulong,
            f,
        )
    {
        fclose(f);
        return -(2 as libc::c_int);
    }
    fclose(f);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pam_destroy(mut pam: *mut pam_t) {
    if pam.is_null() {
        return;
    }
    free((*pam).data as *mut libc::c_void);
    free(pam as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn pam_copy(mut pam: *mut pam_t) -> *mut pam_t {
    let mut copy: *mut pam_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<pam_t>() as libc::c_ulong,
    ) as *mut pam_t;
    (*copy).width = (*pam).width;
    (*copy).height = (*pam).height;
    (*copy).depth = (*pam).depth;
    (*copy).maxval = (*pam).maxval;
    (*copy).type_0 = (*pam).type_0;
    (*copy).datalen = (*pam).datalen;
    let ref mut fresh1 = (*copy).data;
    *fresh1 = malloc((*pam).datalen as libc::c_ulong) as *mut uint8_t;
    memcpy(
        (*copy).data as *mut libc::c_void,
        (*pam).data as *const libc::c_void,
        (*pam).datalen as libc::c_ulong,
    );
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn pam_convert(
    mut in_0: *mut pam_t,
    mut type_0: libc::c_int,
) -> *mut pam_t {
    if type_0 == (*in_0).type_0 {
        return pam_copy(in_0);
    }
    let mut w: libc::c_int = (*in_0).width;
    let mut h: libc::c_int = (*in_0).height;
    let mut out: *mut pam_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<pam_t>() as libc::c_ulong,
    ) as *mut pam_t;
    (*out).type_0 = type_0;
    (*out).width = w;
    (*out).height = h;
    (*out).maxval = (*in_0).maxval;
    (*out).depth = 4 as libc::c_int;
    (*out).datalen = 4 as libc::c_int * w * h;
    let ref mut fresh2 = (*out).data;
    *fresh2 = malloc((*out).datalen as libc::c_ulong) as *mut uint8_t;
    if (*in_0).type_0 == PAM_RGB as libc::c_int {
        let mut y: libc::c_int = 0 as libc::c_int;
        while y < h {
            let mut x: libc::c_int = 0 as libc::c_int;
            while x < w {
                *((*out).data)
                    .offset(
                        (y * 4 as libc::c_int * w + 4 as libc::c_int * x
                            + 0 as libc::c_int) as isize,
                    ) = *((*in_0).data)
                    .offset(
                        (y * 3 as libc::c_int * w + 3 as libc::c_int * x
                            + 0 as libc::c_int) as isize,
                    );
                *((*out).data)
                    .offset(
                        (y * 4 as libc::c_int * w + 4 as libc::c_int * x
                            + 1 as libc::c_int) as isize,
                    ) = *((*in_0).data)
                    .offset(
                        (y * 3 as libc::c_int * w + 3 as libc::c_int * x
                            + 1 as libc::c_int) as isize,
                    );
                *((*out).data)
                    .offset(
                        (y * 4 as libc::c_int * w + 4 as libc::c_int * x
                            + 2 as libc::c_int) as isize,
                    ) = *((*in_0).data)
                    .offset(
                        (y * 3 as libc::c_int * w + 3 as libc::c_int * x
                            + 2 as libc::c_int) as isize,
                    );
                *((*out).data)
                    .offset(
                        (y * 4 as libc::c_int * w + 4 as libc::c_int * x
                            + 3 as libc::c_int) as isize,
                    ) = 255 as libc::c_int as uint8_t;
                x += 1;
            }
            y += 1;
        }
    } else {
        printf(
            b"pam.c unsupported type %d\n\0" as *const u8 as *const libc::c_char,
            (*in_0).type_0,
        );
    }
    return out;
}
