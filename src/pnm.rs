use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
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
#[no_mangle]
pub unsafe extern "C" fn pnm_create_from_file(
    mut path: *const libc::c_char,
) -> *mut pnm_t {
    let mut current_block: u64;
    let mut f: *mut FILE = fopen(path, b"rb\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        return 0 as *mut pnm_t;
    }
    let mut pnm: *mut pnm_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<pnm_t>() as libc::c_ulong,
    ) as *mut pnm_t;
    (*pnm).format = -(1 as libc::c_int);
    let mut tmp: [libc::c_char; 1024] = [0; 1024];
    let mut nparams: libc::c_int = 0 as libc::c_int;
    let mut params: [libc::c_int; 3] = [0; 3];
    loop {
        if !(nparams < 3 as libc::c_int
            && !((*pnm).format == 4 as libc::c_int && nparams == 2 as libc::c_int))
        {
            current_block = 7172762164747879670;
            break;
        }
        if (fgets(
            tmp.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                as libc::c_int,
            f,
        ))
            .is_null()
        {
            current_block = 10470556844075724936;
            break;
        }
        if tmp[0 as libc::c_int as usize] as libc::c_int == '#' as i32 {
            continue;
        }
        let mut p: *mut libc::c_char = tmp.as_mut_ptr();
        if (*pnm).format == -(1 as libc::c_int)
            && tmp[0 as libc::c_int as usize] as libc::c_int == 'P' as i32
        {
            (*pnm).format = tmp[1 as libc::c_int as usize] as libc::c_int - '0' as i32;
            p = &mut *tmp.as_mut_ptr().offset(2 as libc::c_int as isize)
                as *mut libc::c_char;
        }
        while nparams < 3 as libc::c_int && *p as libc::c_int != 0 as libc::c_int {
            while *p as libc::c_int == ' ' as i32 {
                p = p.offset(1);
            }
            if (*p as libc::c_int) < '0' as i32 || *p as libc::c_int > '9' as i32 {
                break;
            }
            let mut acc: libc::c_int = 0 as libc::c_int;
            while *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
                acc = acc * 10 as libc::c_int + *p as libc::c_int - '0' as i32;
                p = p.offset(1);
            }
            let fresh0 = nparams;
            nparams = nparams + 1;
            params[fresh0 as usize] = acc;
            p = p.offset(1);
        }
    }
    match current_block {
        7172762164747879670 => {
            (*pnm).width = params[0 as libc::c_int as usize];
            (*pnm).height = params[1 as libc::c_int as usize];
            (*pnm).max = params[2 as libc::c_int as usize];
            match (*pnm).format {
                4 => {
                    current_block = 4775909272756257391;
                    match current_block {
                        15004371738079956865 => {
                            if (*pnm).max == 255 as libc::c_int {
                                (*pnm)
                                    .buflen = ((*pnm).width * (*pnm).height * 3 as libc::c_int)
                                    as uint32_t;
                            } else if (*pnm).max == 65535 as libc::c_int {
                                (*pnm)
                                    .buflen = (2 as libc::c_int * (*pnm).width * (*pnm).height
                                    * 3 as libc::c_int) as uint32_t;
                            }
                            let ref mut fresh3 = (*pnm).buf;
                            *fresh3 = malloc((*pnm).buflen as libc::c_ulong)
                                as *mut uint8_t;
                            let mut len_1: size_t = fread(
                                (*pnm).buf as *mut libc::c_void,
                                1 as libc::c_int as libc::c_ulong,
                                (*pnm).buflen as libc::c_ulong,
                                f,
                            );
                            if !(len_1 != (*pnm).buflen as libc::c_ulong) {
                                fclose(f);
                                return pnm;
                            }
                        }
                        4775909272756257391 => {
                            (*pnm).max = 1 as libc::c_int;
                            (*pnm)
                                .buflen = ((*pnm).height
                                * (((*pnm).width + 7 as libc::c_int) / 8 as libc::c_int))
                                as uint32_t;
                            let ref mut fresh1 = (*pnm).buf;
                            *fresh1 = malloc((*pnm).buflen as libc::c_ulong)
                                as *mut uint8_t;
                            let mut len: size_t = fread(
                                (*pnm).buf as *mut libc::c_void,
                                1 as libc::c_int as libc::c_ulong,
                                (*pnm).buflen as libc::c_ulong,
                                f,
                            );
                            if !(len != (*pnm).buflen as libc::c_ulong) {
                                fclose(f);
                                return pnm;
                            }
                        }
                        _ => {
                            if (*pnm).max == 255 as libc::c_int {
                                (*pnm).buflen = ((*pnm).width * (*pnm).height) as uint32_t;
                            } else if (*pnm).max == 65535 as libc::c_int {
                                (*pnm)
                                    .buflen = (2 as libc::c_int * (*pnm).width * (*pnm).height)
                                    as uint32_t;
                            }
                            let ref mut fresh2 = (*pnm).buf;
                            *fresh2 = malloc((*pnm).buflen as libc::c_ulong)
                                as *mut uint8_t;
                            let mut len_0: size_t = fread(
                                (*pnm).buf as *mut libc::c_void,
                                1 as libc::c_int as libc::c_ulong,
                                (*pnm).buflen as libc::c_ulong,
                                f,
                            );
                            if !(len_0 != (*pnm).buflen as libc::c_ulong) {
                                fclose(f);
                                return pnm;
                            }
                        }
                    }
                }
                5 => {
                    current_block = 7828949454673616476;
                    match current_block {
                        15004371738079956865 => {
                            if (*pnm).max == 255 as libc::c_int {
                                (*pnm)
                                    .buflen = ((*pnm).width * (*pnm).height * 3 as libc::c_int)
                                    as uint32_t;
                            } else if (*pnm).max == 65535 as libc::c_int {
                                (*pnm)
                                    .buflen = (2 as libc::c_int * (*pnm).width * (*pnm).height
                                    * 3 as libc::c_int) as uint32_t;
                            }
                            let ref mut fresh3 = (*pnm).buf;
                            *fresh3 = malloc((*pnm).buflen as libc::c_ulong)
                                as *mut uint8_t;
                            let mut len_1: size_t = fread(
                                (*pnm).buf as *mut libc::c_void,
                                1 as libc::c_int as libc::c_ulong,
                                (*pnm).buflen as libc::c_ulong,
                                f,
                            );
                            if !(len_1 != (*pnm).buflen as libc::c_ulong) {
                                fclose(f);
                                return pnm;
                            }
                        }
                        4775909272756257391 => {
                            (*pnm).max = 1 as libc::c_int;
                            (*pnm)
                                .buflen = ((*pnm).height
                                * (((*pnm).width + 7 as libc::c_int) / 8 as libc::c_int))
                                as uint32_t;
                            let ref mut fresh1 = (*pnm).buf;
                            *fresh1 = malloc((*pnm).buflen as libc::c_ulong)
                                as *mut uint8_t;
                            let mut len: size_t = fread(
                                (*pnm).buf as *mut libc::c_void,
                                1 as libc::c_int as libc::c_ulong,
                                (*pnm).buflen as libc::c_ulong,
                                f,
                            );
                            if !(len != (*pnm).buflen as libc::c_ulong) {
                                fclose(f);
                                return pnm;
                            }
                        }
                        _ => {
                            if (*pnm).max == 255 as libc::c_int {
                                (*pnm).buflen = ((*pnm).width * (*pnm).height) as uint32_t;
                            } else if (*pnm).max == 65535 as libc::c_int {
                                (*pnm)
                                    .buflen = (2 as libc::c_int * (*pnm).width * (*pnm).height)
                                    as uint32_t;
                            }
                            let ref mut fresh2 = (*pnm).buf;
                            *fresh2 = malloc((*pnm).buflen as libc::c_ulong)
                                as *mut uint8_t;
                            let mut len_0: size_t = fread(
                                (*pnm).buf as *mut libc::c_void,
                                1 as libc::c_int as libc::c_ulong,
                                (*pnm).buflen as libc::c_ulong,
                                f,
                            );
                            if !(len_0 != (*pnm).buflen as libc::c_ulong) {
                                fclose(f);
                                return pnm;
                            }
                        }
                    }
                }
                6 => {
                    current_block = 15004371738079956865;
                    match current_block {
                        15004371738079956865 => {
                            if (*pnm).max == 255 as libc::c_int {
                                (*pnm)
                                    .buflen = ((*pnm).width * (*pnm).height * 3 as libc::c_int)
                                    as uint32_t;
                            } else if (*pnm).max == 65535 as libc::c_int {
                                (*pnm)
                                    .buflen = (2 as libc::c_int * (*pnm).width * (*pnm).height
                                    * 3 as libc::c_int) as uint32_t;
                            }
                            let ref mut fresh3 = (*pnm).buf;
                            *fresh3 = malloc((*pnm).buflen as libc::c_ulong)
                                as *mut uint8_t;
                            let mut len_1: size_t = fread(
                                (*pnm).buf as *mut libc::c_void,
                                1 as libc::c_int as libc::c_ulong,
                                (*pnm).buflen as libc::c_ulong,
                                f,
                            );
                            if !(len_1 != (*pnm).buflen as libc::c_ulong) {
                                fclose(f);
                                return pnm;
                            }
                        }
                        4775909272756257391 => {
                            (*pnm).max = 1 as libc::c_int;
                            (*pnm)
                                .buflen = ((*pnm).height
                                * (((*pnm).width + 7 as libc::c_int) / 8 as libc::c_int))
                                as uint32_t;
                            let ref mut fresh1 = (*pnm).buf;
                            *fresh1 = malloc((*pnm).buflen as libc::c_ulong)
                                as *mut uint8_t;
                            let mut len: size_t = fread(
                                (*pnm).buf as *mut libc::c_void,
                                1 as libc::c_int as libc::c_ulong,
                                (*pnm).buflen as libc::c_ulong,
                                f,
                            );
                            if !(len != (*pnm).buflen as libc::c_ulong) {
                                fclose(f);
                                return pnm;
                            }
                        }
                        _ => {
                            if (*pnm).max == 255 as libc::c_int {
                                (*pnm).buflen = ((*pnm).width * (*pnm).height) as uint32_t;
                            } else if (*pnm).max == 65535 as libc::c_int {
                                (*pnm)
                                    .buflen = (2 as libc::c_int * (*pnm).width * (*pnm).height)
                                    as uint32_t;
                            }
                            let ref mut fresh2 = (*pnm).buf;
                            *fresh2 = malloc((*pnm).buflen as libc::c_ulong)
                                as *mut uint8_t;
                            let mut len_0: size_t = fread(
                                (*pnm).buf as *mut libc::c_void,
                                1 as libc::c_int as libc::c_ulong,
                                (*pnm).buflen as libc::c_ulong,
                                f,
                            );
                            if !(len_0 != (*pnm).buflen as libc::c_ulong) {
                                fclose(f);
                                return pnm;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
    fclose(f);
    if !pnm.is_null() {
        free((*pnm).buf as *mut libc::c_void);
        free(pnm as *mut libc::c_void);
    }
    return 0 as *mut pnm_t;
}
#[no_mangle]
pub unsafe extern "C" fn pnm_destroy(mut pnm: *mut pnm_t) {
    if pnm.is_null() {
        return;
    }
    free((*pnm).buf as *mut libc::c_void);
    free(pnm as *mut libc::c_void);
}
