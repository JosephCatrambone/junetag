use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type zhash;
    pub type string_buffer;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn zarray_vmap(za: *mut zarray_t, f: Option::<unsafe extern "C" fn() -> ()>);
    fn zhash_create(
        keysz: size_t,
        valuesz: size_t,
        hash: Option::<unsafe extern "C" fn(*const libc::c_void) -> uint32_t>,
        equals: Option::<
            unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        >,
    ) -> *mut zhash_t;
    fn zhash_destroy(zh: *mut zhash_t);
    fn zhash_contains(zh: *const zhash_t, key: *const libc::c_void) -> libc::c_int;
    fn zhash_get(
        zh: *const zhash_t,
        key: *const libc::c_void,
        out_value: *mut libc::c_void,
    ) -> libc::c_int;
    fn zhash_put(
        zh: *mut zhash_t,
        key: *const libc::c_void,
        value: *const libc::c_void,
        oldkey: *mut libc::c_void,
        oldvalue: *mut libc::c_void,
    ) -> libc::c_int;
    fn zhash_str_hash(a: *const libc::c_void) -> uint32_t;
    fn zhash_str_equals(a: *const libc::c_void, b: *const libc::c_void) -> libc::c_int;
    fn str_starts_with(
        haystack: *const libc::c_char,
        needle: *const libc::c_char,
    ) -> bool;
    fn str_indexof(
        haystack: *const libc::c_char,
        needle: *const libc::c_char,
    ) -> libc::c_int;
    fn string_buffer_create() -> *mut string_buffer_t;
    fn string_buffer_destroy(sb: *mut string_buffer_t);
    fn string_buffer_appendf(sb: *mut string_buffer_t, fmt: *const libc::c_char, _: ...);
    fn string_buffer_to_string(sb: *mut string_buffer_t) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zarray {
    pub el_sz: size_t,
    pub size: libc::c_int,
    pub alloc: libc::c_int,
    pub data: *mut libc::c_char,
}
pub type zarray_t = zarray;
pub type zhash_t = zhash;
pub type string_buffer_t = string_buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct getopt {
    pub lopts: *mut zhash_t,
    pub sopts: *mut zhash_t,
    pub extraargs: *mut zarray_t,
    pub options: *mut zarray_t,
}
pub type getopt_t = getopt;
pub type getopt_option_t = getopt_option;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct getopt_option {
    pub sname: *mut libc::c_char,
    pub lname: *mut libc::c_char,
    pub svalue: *mut libc::c_char,
    pub help: *mut libc::c_char,
    pub type_0: libc::c_int,
    pub spacer: libc::c_int,
    pub was_specified: libc::c_int,
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
#[no_mangle]
pub unsafe extern "C" fn getopt_create() -> *mut getopt_t {
    let mut gopt: *mut getopt_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<getopt_t>() as libc::c_ulong,
    ) as *mut getopt_t;
    let ref mut fresh2 = (*gopt).lopts;
    *fresh2 = zhash_create(
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        ::std::mem::size_of::<*mut getopt_option_t>() as libc::c_ulong,
        Some(zhash_str_hash as unsafe extern "C" fn(*const libc::c_void) -> uint32_t),
        Some(
            zhash_str_equals
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    let ref mut fresh3 = (*gopt).sopts;
    *fresh3 = zhash_create(
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        ::std::mem::size_of::<*mut getopt_option_t>() as libc::c_ulong,
        Some(zhash_str_hash as unsafe extern "C" fn(*const libc::c_void) -> uint32_t),
        Some(
            zhash_str_equals
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    let ref mut fresh4 = (*gopt).options;
    *fresh4 = zarray_create(
        ::std::mem::size_of::<*mut getopt_option_t>() as libc::c_ulong,
    );
    let ref mut fresh5 = (*gopt).extraargs;
    *fresh5 = zarray_create(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong);
    return gopt;
}
#[no_mangle]
pub unsafe extern "C" fn getopt_option_destroy(mut goo: *mut getopt_option_t) {
    free((*goo).sname as *mut libc::c_void);
    free((*goo).lname as *mut libc::c_void);
    free((*goo).svalue as *mut libc::c_void);
    free((*goo).help as *mut libc::c_void);
    memset(
        goo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<getopt_option_t>() as libc::c_ulong,
    );
    free(goo as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn getopt_destroy(mut gopt: *mut getopt_t) {
    zarray_vmap(
        (*gopt).extraargs,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ())),
    );
    zarray_destroy((*gopt).extraargs);
    zarray_vmap(
        (*gopt).options,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut getopt_option_t) -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                getopt_option_destroy as unsafe extern "C" fn(*mut getopt_option_t) -> (),
            ),
        ),
    );
    zarray_destroy((*gopt).options);
    zhash_destroy((*gopt).lopts);
    zhash_destroy((*gopt).sopts);
    memset(
        gopt as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<getopt_t>() as libc::c_ulong,
    );
    free(gopt as *mut libc::c_void);
}
unsafe extern "C" fn getopt_modify_string(
    mut str: *mut *mut libc::c_char,
    mut newvalue: *mut libc::c_char,
) {
    let mut old: *mut libc::c_char = *str;
    *str = newvalue;
    if !old.is_null() {
        free(old as *mut libc::c_void);
    }
}
unsafe extern "C" fn get_arg_assignment(
    mut arg: *mut libc::c_char,
) -> *mut libc::c_char {
    if !str_starts_with(arg, b"--\0" as *const u8 as *const libc::c_char) {
        return 0 as *mut libc::c_char;
    }
    let mut eq_index: libc::c_int = str_indexof(
        arg,
        b"=\0" as *const u8 as *const libc::c_char,
    );
    if eq_index == -(1 as libc::c_int) {
        return 0 as *mut libc::c_char;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < eq_index {
        if *arg.offset(i as isize) as libc::c_int == '\'' as i32
            || *arg.offset(i as isize) as libc::c_int == '"' as i32
        {
            return 0 as *mut libc::c_char;
        }
        i += 1;
    }
    return &mut *arg.offset(eq_index as isize) as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn getopt_parse(
    mut gopt: *mut getopt_t,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut showErrors: libc::c_int,
) -> libc::c_int {
    let mut okay: libc::c_int = 1 as libc::c_int;
    let mut toks: *mut zarray_t = zarray_create(
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    );
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < argc {
        let mut arg: *mut libc::c_char = strdup(*argv.offset(i as isize));
        let mut eq: *mut libc::c_char = get_arg_assignment(arg);
        if eq.is_null() {
            zarray_add(toks, &mut arg as *mut *mut libc::c_char as *const libc::c_void);
        } else {
            let mut val: *mut libc::c_char = strdup(
                &mut *eq.offset(1 as libc::c_int as isize),
            );
            *eq.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
            zarray_add(toks, &mut arg as *mut *mut libc::c_char as *const libc::c_void);
            if *val.offset(0 as libc::c_int as isize) as libc::c_int == '"' as i32 {
                let mut last: size_t = (strlen(val))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                if *val.offset(last as isize) as libc::c_int == '"' as i32 {
                    *val.offset(last as isize) = 0 as libc::c_int as libc::c_char;
                }
                let mut valclean: *mut libc::c_char = strdup(
                    &mut *val.offset(1 as libc::c_int as isize),
                );
                zarray_add(
                    toks,
                    &mut valclean as *mut *mut libc::c_char as *const libc::c_void,
                );
                free(val as *mut libc::c_void);
            } else {
                zarray_add(
                    toks,
                    &mut val as *mut *mut libc::c_char as *const libc::c_void,
                );
            }
        }
        i += 1;
    }
    let mut i_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut tok: *mut libc::c_char = 0 as *mut libc::c_char;
    while i_0 < zarray_size(toks) as libc::c_uint {
        if !tok.is_null() {
            free(tok as *mut libc::c_void);
        }
        zarray_get(
            toks,
            i_0 as libc::c_int,
            &mut tok as *mut *mut libc::c_char as *mut libc::c_void,
        );
        if strncmp(
            tok,
            b"--\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            let mut optname: *mut libc::c_char = &mut *tok
                .offset(2 as libc::c_int as isize) as *mut libc::c_char;
            let mut goo: *mut getopt_option_t = 0 as *mut getopt_option_t;
            zhash_get(
                (*gopt).lopts,
                &mut optname as *mut *mut libc::c_char as *const libc::c_void,
                &mut goo as *mut *mut getopt_option_t as *mut libc::c_void,
            );
            if goo.is_null() {
                okay = 0 as libc::c_int;
                if showErrors != 0 {
                    printf(
                        b"Unknown option --%s\n\0" as *const u8 as *const libc::c_char,
                        optname,
                    );
                }
                i_0 = i_0.wrapping_add(1);
                continue;
            } else {
                (*goo).was_specified = 1 as libc::c_int;
                if (*goo).type_0 == 1 as libc::c_int {
                    if i_0.wrapping_add(1 as libc::c_int as libc::c_uint)
                        < zarray_size(toks) as libc::c_uint
                    {
                        let mut val_0: *mut libc::c_char = 0 as *mut libc::c_char;
                        zarray_get(
                            toks,
                            i_0.wrapping_add(1 as libc::c_int as libc::c_uint)
                                as libc::c_int,
                            &mut val_0 as *mut *mut libc::c_char as *mut libc::c_void,
                        );
                        if strcmp(val_0, b"true\0" as *const u8 as *const libc::c_char)
                            == 0
                        {
                            i_0 = i_0.wrapping_add(2 as libc::c_int as libc::c_uint);
                            getopt_modify_string(&mut (*goo).svalue, val_0);
                            continue;
                        } else if strcmp(
                                val_0,
                                b"false\0" as *const u8 as *const libc::c_char,
                            ) == 0
                            {
                            i_0 = i_0.wrapping_add(2 as libc::c_int as libc::c_uint);
                            getopt_modify_string(&mut (*goo).svalue, val_0);
                            continue;
                        }
                    }
                    getopt_modify_string(
                        &mut (*goo).svalue,
                        strdup(b"true\0" as *const u8 as *const libc::c_char),
                    );
                    i_0 = i_0.wrapping_add(1);
                    continue;
                } else if (*goo).type_0 == 2 as libc::c_int {
                    if i_0.wrapping_add(1 as libc::c_int as libc::c_uint)
                        < zarray_size(toks) as libc::c_uint
                    {
                        let mut val_1: *mut libc::c_char = 0 as *mut libc::c_char;
                        zarray_get(
                            toks,
                            i_0.wrapping_add(1 as libc::c_int as libc::c_uint)
                                as libc::c_int,
                            &mut val_1 as *mut *mut libc::c_char as *mut libc::c_void,
                        );
                        i_0 = i_0.wrapping_add(2 as libc::c_int as libc::c_uint);
                        getopt_modify_string(&mut (*goo).svalue, val_1);
                        continue;
                    } else {
                        okay = 0 as libc::c_int;
                        if showErrors != 0 {
                            printf(
                                b"Option %s requires a string argument.\n\0" as *const u8
                                    as *const libc::c_char,
                                optname,
                            );
                        }
                    }
                }
            }
        }
        if strncmp(
            tok,
            b"-\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as libc::c_ulong,
        ) == 0
            && strncmp(
                tok,
                b"--\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            ) != 0
        {
            let mut len: size_t = strlen(tok);
            let mut pos: libc::c_int = 0;
            pos = 1 as libc::c_int;
            while (pos as libc::c_ulong) < len {
                let mut sopt: [libc::c_char; 2] = [0; 2];
                sopt[0 as libc::c_int as usize] = *tok.offset(pos as isize);
                sopt[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                let mut sopt_ptr: *mut libc::c_char = &mut sopt as *mut [libc::c_char; 2]
                    as *mut libc::c_char;
                let mut goo_0: *mut getopt_option_t = 0 as *mut getopt_option_t;
                zhash_get(
                    (*gopt).sopts,
                    &mut sopt_ptr as *mut *mut libc::c_char as *const libc::c_void,
                    &mut goo_0 as *mut *mut getopt_option_t as *mut libc::c_void,
                );
                if goo_0.is_null() {
                    if pos == 1 as libc::c_int
                        && *(*__ctype_b_loc())
                            .offset(*tok.offset(pos as isize) as libc::c_int as isize)
                            as libc::c_int
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                    {
                        zarray_add(
                            (*gopt).extraargs,
                            &mut tok as *mut *mut libc::c_char as *const libc::c_void,
                        );
                        tok = 0 as *mut libc::c_char;
                        break;
                    } else {
                        okay = 0 as libc::c_int;
                        if showErrors != 0 {
                            printf(
                                b"Unknown option -%c\n\0" as *const u8
                                    as *const libc::c_char,
                                *tok.offset(pos as isize) as libc::c_int,
                            );
                        }
                        i_0 = i_0.wrapping_add(1);
                    }
                } else {
                    (*goo_0).was_specified = 1 as libc::c_int;
                    if (*goo_0).type_0 == 1 as libc::c_int {
                        getopt_modify_string(
                            &mut (*goo_0).svalue,
                            strdup(b"true\0" as *const u8 as *const libc::c_char),
                        );
                    } else if (*goo_0).type_0 == 2 as libc::c_int {
                        if i_0.wrapping_add(1 as libc::c_int as libc::c_uint)
                            < zarray_size(toks) as libc::c_uint
                        {
                            let mut val_2: *mut libc::c_char = 0 as *mut libc::c_char;
                            zarray_get(
                                toks,
                                i_0.wrapping_add(1 as libc::c_int as libc::c_uint)
                                    as libc::c_int,
                                &mut val_2 as *mut *mut libc::c_char as *mut libc::c_void,
                            );
                            if *val_2.offset(0 as libc::c_int as isize) as libc::c_int
                                == '-' as i32
                            {
                                okay = 0 as libc::c_int;
                                if showErrors != 0 {
                                    printf(
                                        b"Ran out of arguments for option block %s\n\0" as *const u8
                                            as *const libc::c_char,
                                        tok,
                                    );
                                }
                            }
                            i_0 = i_0.wrapping_add(1);
                            getopt_modify_string(&mut (*goo_0).svalue, val_2);
                        } else {
                            okay = 0 as libc::c_int;
                            if showErrors != 0 {
                                printf(
                                    b"Option -%c requires a string argument.\n\0" as *const u8
                                        as *const libc::c_char,
                                    *tok.offset(pos as isize) as libc::c_int,
                                );
                            }
                        }
                    }
                }
                pos += 1;
            }
            i_0 = i_0.wrapping_add(1);
        } else {
            zarray_add(
                (*gopt).extraargs,
                &mut tok as *mut *mut libc::c_char as *const libc::c_void,
            );
            tok = 0 as *mut libc::c_char;
            i_0 = i_0.wrapping_add(1);
        }
    }
    if !tok.is_null() {
        free(tok as *mut libc::c_void);
    }
    zarray_destroy(toks);
    return okay;
}
#[no_mangle]
pub unsafe extern "C" fn getopt_add_spacer(
    mut gopt: *mut getopt_t,
    mut s: *const libc::c_char,
) {
    let mut goo: *mut getopt_option_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<getopt_option_t>() as libc::c_ulong,
    ) as *mut getopt_option_t;
    (*goo).spacer = 1 as libc::c_int;
    let ref mut fresh6 = (*goo).help;
    *fresh6 = strdup(s);
    zarray_add(
        (*gopt).options,
        &mut goo as *mut *mut getopt_option_t as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn getopt_add_bool(
    mut gopt: *mut getopt_t,
    mut sopt: libc::c_char,
    mut lname: *const libc::c_char,
    mut def: libc::c_int,
    mut help: *const libc::c_char,
) {
    let mut sname: [libc::c_char; 2] = [0; 2];
    sname[0 as libc::c_int as usize] = sopt;
    sname[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    let mut sname_ptr: *mut libc::c_char = &mut sname as *mut [libc::c_char; 2]
        as *mut libc::c_char;
    if strlen(lname) < 1 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"getopt_add_bool(): must supply option name\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if sopt as libc::c_int == '-' as i32 {
        fprintf(
            stderr,
            b"getopt_add_bool(): invalid option character: '%c'\n\0" as *const u8
                as *const libc::c_char,
            sopt as libc::c_int,
        );
        exit(1 as libc::c_int);
    }
    if zhash_contains(
        (*gopt).lopts,
        &mut lname as *mut *const libc::c_char as *const libc::c_void,
    ) != 0
    {
        fprintf(
            stderr,
            b"getopt_add_bool(): duplicate option name: --%s\n\0" as *const u8
                as *const libc::c_char,
            lname,
        );
        exit(1 as libc::c_int);
    }
    if sopt as libc::c_int != '\u{0}' as i32
        && zhash_contains(
            (*gopt).sopts,
            &mut sname_ptr as *mut *mut libc::c_char as *const libc::c_void,
        ) != 0
    {
        fprintf(
            stderr,
            b"getopt_add_bool(): duplicate option: -%s ('%s')\n\0" as *const u8
                as *const libc::c_char,
            sname.as_mut_ptr(),
            lname,
        );
        exit(1 as libc::c_int);
    }
    let mut goo: *mut getopt_option_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<getopt_option_t>() as libc::c_ulong,
    ) as *mut getopt_option_t;
    let ref mut fresh7 = (*goo).sname;
    *fresh7 = strdup(sname.as_mut_ptr());
    let ref mut fresh8 = (*goo).lname;
    *fresh8 = strdup(lname);
    let ref mut fresh9 = (*goo).svalue;
    *fresh9 = strdup(
        if def != 0 {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
    );
    (*goo).type_0 = 1 as libc::c_int;
    let ref mut fresh10 = (*goo).help;
    *fresh10 = strdup(help);
    zhash_put(
        (*gopt).lopts,
        &mut (*goo).lname as *mut *mut libc::c_char as *const libc::c_void,
        &mut goo as *mut *mut getopt_option_t as *const libc::c_void,
        0 as *mut libc::c_void,
        0 as *mut libc::c_void,
    );
    zhash_put(
        (*gopt).sopts,
        &mut (*goo).sname as *mut *mut libc::c_char as *const libc::c_void,
        &mut goo as *mut *mut getopt_option_t as *const libc::c_void,
        0 as *mut libc::c_void,
        0 as *mut libc::c_void,
    );
    zarray_add(
        (*gopt).options,
        &mut goo as *mut *mut getopt_option_t as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn getopt_add_int(
    mut gopt: *mut getopt_t,
    mut sopt: libc::c_char,
    mut lname: *const libc::c_char,
    mut def: *const libc::c_char,
    mut help: *const libc::c_char,
) {
    getopt_add_string(gopt, sopt, lname, def, help);
}
#[no_mangle]
pub unsafe extern "C" fn getopt_add_double(
    mut gopt: *mut getopt_t,
    mut sopt: libc::c_char,
    mut lname: *const libc::c_char,
    mut def: *const libc::c_char,
    mut help: *const libc::c_char,
) {
    getopt_add_string(gopt, sopt, lname, def, help);
}
#[no_mangle]
pub unsafe extern "C" fn getopt_add_string(
    mut gopt: *mut getopt_t,
    mut sopt: libc::c_char,
    mut lname: *const libc::c_char,
    mut def: *const libc::c_char,
    mut help: *const libc::c_char,
) {
    let mut sname: [libc::c_char; 2] = [0; 2];
    sname[0 as libc::c_int as usize] = sopt;
    sname[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    let mut sname_ptr: *mut libc::c_char = &mut sname as *mut [libc::c_char; 2]
        as *mut libc::c_char;
    if strlen(lname) < 1 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"getopt_add_string(): must supply option name\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if sopt as libc::c_int == '-' as i32 {
        fprintf(
            stderr,
            b"getopt_add_string(): invalid option character: '%c'\n\0" as *const u8
                as *const libc::c_char,
            sopt as libc::c_int,
        );
        exit(1 as libc::c_int);
    }
    if zhash_contains(
        (*gopt).lopts,
        &mut lname as *mut *const libc::c_char as *const libc::c_void,
    ) != 0
    {
        fprintf(
            stderr,
            b"getopt_add_string(): duplicate option name: --%s\n\0" as *const u8
                as *const libc::c_char,
            lname,
        );
        exit(1 as libc::c_int);
    }
    if sopt as libc::c_int != '\u{0}' as i32
        && zhash_contains(
            (*gopt).sopts,
            &mut sname_ptr as *mut *mut libc::c_char as *const libc::c_void,
        ) != 0
    {
        fprintf(
            stderr,
            b"getopt_add_string(): duplicate option: -%s ('%s')\n\0" as *const u8
                as *const libc::c_char,
            sname.as_mut_ptr(),
            lname,
        );
        exit(1 as libc::c_int);
    }
    let mut goo: *mut getopt_option_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<getopt_option_t>() as libc::c_ulong,
    ) as *mut getopt_option_t;
    let ref mut fresh11 = (*goo).sname;
    *fresh11 = strdup(sname.as_mut_ptr());
    let ref mut fresh12 = (*goo).lname;
    *fresh12 = strdup(lname);
    let ref mut fresh13 = (*goo).svalue;
    *fresh13 = strdup(def);
    (*goo).type_0 = 2 as libc::c_int;
    let ref mut fresh14 = (*goo).help;
    *fresh14 = strdup(help);
    zhash_put(
        (*gopt).lopts,
        &mut (*goo).lname as *mut *mut libc::c_char as *const libc::c_void,
        &mut goo as *mut *mut getopt_option_t as *const libc::c_void,
        0 as *mut libc::c_void,
        0 as *mut libc::c_void,
    );
    zhash_put(
        (*gopt).sopts,
        &mut (*goo).sname as *mut *mut libc::c_char as *const libc::c_void,
        &mut goo as *mut *mut getopt_option_t as *const libc::c_void,
        0 as *mut libc::c_void,
        0 as *mut libc::c_void,
    );
    zarray_add(
        (*gopt).options,
        &mut goo as *mut *mut getopt_option_t as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn getopt_get_string(
    mut gopt: *mut getopt_t,
    mut lname: *const libc::c_char,
) -> *const libc::c_char {
    let mut goo: *mut getopt_option_t = 0 as *mut getopt_option_t;
    zhash_get(
        (*gopt).lopts,
        &mut lname as *mut *const libc::c_char as *const libc::c_void,
        &mut goo as *mut *mut getopt_option_t as *mut libc::c_void,
    );
    return (*goo).svalue;
}
#[no_mangle]
pub unsafe extern "C" fn getopt_get_int(
    mut getopt: *mut getopt_t,
    mut lname: *const libc::c_char,
) -> libc::c_int {
    let mut v: *const libc::c_char = getopt_get_string(getopt, lname);
    *__errno_location() = 0 as libc::c_int;
    let mut endptr: *mut libc::c_char = v as *mut libc::c_char;
    let mut val: libc::c_long = strtol(v, &mut endptr, 10 as libc::c_int);
    if *__errno_location() != 0 as libc::c_int {
        fprintf(
            stderr,
            b"--%s argument: strtol failed: %s\n\0" as *const u8 as *const libc::c_char,
            lname,
            strerror(*__errno_location()),
        );
        exit(1 as libc::c_int);
    }
    if endptr == v as *mut libc::c_char {
        fprintf(
            stderr,
            b"--%s argument cannot be parsed as an int\n\0" as *const u8
                as *const libc::c_char,
            lname,
        );
        exit(1 as libc::c_int);
    }
    return val as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getopt_get_bool(
    mut getopt: *mut getopt_t,
    mut lname: *const libc::c_char,
) -> libc::c_int {
    let mut v: *const libc::c_char = getopt_get_string(getopt, lname);
    let mut val: libc::c_int = (strcmp(v, b"true\0" as *const u8 as *const libc::c_char)
        == 0) as libc::c_int;
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn getopt_get_double(
    mut getopt: *mut getopt_t,
    mut lname: *const libc::c_char,
) -> libc::c_double {
    let mut v: *const libc::c_char = getopt_get_string(getopt, lname);
    *__errno_location() = 0 as libc::c_int;
    let mut endptr: *mut libc::c_char = v as *mut libc::c_char;
    let mut d: libc::c_double = strtod(v, &mut endptr);
    if *__errno_location() != 0 as libc::c_int {
        fprintf(
            stderr,
            b"--%s argument: strtod failed: %s\n\0" as *const u8 as *const libc::c_char,
            lname,
            strerror(*__errno_location()),
        );
        exit(1 as libc::c_int);
    }
    if endptr == v as *mut libc::c_char {
        fprintf(
            stderr,
            b"--%s argument cannot be parsed as a double\n\0" as *const u8
                as *const libc::c_char,
            lname,
        );
        exit(1 as libc::c_int);
    }
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn getopt_was_specified(
    mut getopt: *mut getopt_t,
    mut lname: *const libc::c_char,
) -> libc::c_int {
    let mut goo: *mut getopt_option_t = 0 as *mut getopt_option_t;
    zhash_get(
        (*getopt).lopts,
        &mut lname as *mut *const libc::c_char as *const libc::c_void,
        &mut goo as *mut *mut getopt_option_t as *mut libc::c_void,
    );
    if goo.is_null() {
        return 0 as libc::c_int;
    }
    return (*goo).was_specified;
}
#[no_mangle]
pub unsafe extern "C" fn getopt_get_extra_args(
    mut gopt: *mut getopt_t,
) -> *const zarray_t {
    return (*gopt).extraargs;
}
#[no_mangle]
pub unsafe extern "C" fn getopt_do_usage(mut gopt: *mut getopt_t) {
    let mut usage: *mut libc::c_char = getopt_get_usage(gopt);
    printf(b"%s\0" as *const u8 as *const libc::c_char, usage);
    free(usage as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn getopt_get_usage(mut gopt: *mut getopt_t) -> *mut libc::c_char {
    let mut sb: *mut string_buffer_t = string_buffer_create();
    let mut leftmargin: libc::c_int = 2 as libc::c_int;
    let mut longwidth: libc::c_int = 12 as libc::c_int;
    let mut valuewidth: libc::c_int = 10 as libc::c_int;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < zarray_size((*gopt).options) as libc::c_uint {
        let mut goo: *mut getopt_option_t = 0 as *mut getopt_option_t;
        zarray_get(
            (*gopt).options,
            i as libc::c_int,
            &mut goo as *mut *mut getopt_option_t as *mut libc::c_void,
        );
        if !((*goo).spacer != 0) {
            longwidth = if longwidth < strlen((*goo).lname) as libc::c_int {
                strlen((*goo).lname) as libc::c_int
            } else {
                longwidth
            };
            if (*goo).type_0 == 2 as libc::c_int {
                valuewidth = if valuewidth < strlen((*goo).svalue) as libc::c_int {
                    strlen((*goo).svalue) as libc::c_int
                } else {
                    valuewidth
                };
            }
        }
        i = i.wrapping_add(1);
    }
    let mut i_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i_0 < zarray_size((*gopt).options) as libc::c_uint {
        let mut goo_0: *mut getopt_option_t = 0 as *mut getopt_option_t;
        zarray_get(
            (*gopt).options,
            i_0 as libc::c_int,
            &mut goo_0 as *mut *mut getopt_option_t as *mut libc::c_void,
        );
        if (*goo_0).spacer != 0 {
            if ((*goo_0).help).is_null()
                || strlen((*goo_0).help) == 0 as libc::c_int as libc::c_ulong
            {
                string_buffer_appendf(sb, b"\n\0" as *const u8 as *const libc::c_char);
            } else {
                string_buffer_appendf(
                    sb,
                    b"\n%*s%s\n\n\0" as *const u8 as *const libc::c_char,
                    leftmargin,
                    b"\0" as *const u8 as *const libc::c_char,
                    (*goo_0).help,
                );
            }
        } else {
            string_buffer_appendf(
                sb,
                b"%*s\0" as *const u8 as *const libc::c_char,
                leftmargin,
                b"\0" as *const u8 as *const libc::c_char,
            );
            if *((*goo_0).sname).offset(0 as libc::c_int as isize) as libc::c_int
                == 0 as libc::c_int
            {
                string_buffer_appendf(
                    sb,
                    b"     \0" as *const u8 as *const libc::c_char,
                );
            } else {
                string_buffer_appendf(
                    sb,
                    b"-%c | \0" as *const u8 as *const libc::c_char,
                    *((*goo_0).sname).offset(0 as libc::c_int as isize) as libc::c_int,
                );
            }
            string_buffer_appendf(
                sb,
                b"--%*s \0" as *const u8 as *const libc::c_char,
                -longwidth,
                (*goo_0).lname,
            );
            string_buffer_appendf(
                sb,
                b" [ %s ]\0" as *const u8 as *const libc::c_char,
                (*goo_0).svalue,
            );
            string_buffer_appendf(
                sb,
                b"%*s\0" as *const u8 as *const libc::c_char,
                (valuewidth as libc::c_ulong).wrapping_sub(strlen((*goo_0).svalue))
                    as libc::c_int,
                b"\0" as *const u8 as *const libc::c_char,
            );
            string_buffer_appendf(
                sb,
                b" %s   \0" as *const u8 as *const libc::c_char,
                (*goo_0).help,
            );
            string_buffer_appendf(sb, b"\n\0" as *const u8 as *const libc::c_char);
        }
        i_0 = i_0.wrapping_add(1);
    }
    let mut usage: *mut libc::c_char = string_buffer_to_string(sb);
    string_buffer_destroy(sb);
    return usage;
}
