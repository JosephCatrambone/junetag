use ::libc;
extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn zarray_vmap(za: *mut zarray_t, f: Option::<unsafe extern "C" fn() -> ()>);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
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
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
pub struct string_buffer {
    pub s: *mut libc::c_char,
    pub alloc: libc::c_int,
    pub size: size_t,
}
pub type string_buffer_t = string_buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string_feeder {
    pub s: *mut libc::c_char,
    pub len: size_t,
    pub pos: size_t,
    pub line: libc::c_int,
    pub col: libc::c_int,
}
pub type string_feeder_t = string_feeder;
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
#[no_mangle]
pub unsafe extern "C" fn sprintf_alloc(
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    let mut buf: *mut libc::c_char = vsprintf_alloc(fmt, args_0.as_va_list());
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn vsprintf_alloc(
    mut fmt: *const libc::c_char,
    mut orig_args: ::std::ffi::VaList,
) -> *mut libc::c_char {
    let mut size: libc::c_int = 16 as libc::c_int;
    let mut buf: *mut libc::c_char = malloc(
        (size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut returnsize: libc::c_int = 0;
    let mut args: ::std::ffi::VaListImpl;
    args = orig_args.clone();
    returnsize = vsnprintf(buf, size as libc::c_ulong, fmt, args.as_va_list());
    if returnsize < size {
        return buf;
    }
    free(buf as *mut libc::c_void);
    size = returnsize + 1 as libc::c_int;
    buf = malloc(
        (size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    args = orig_args.clone();
    returnsize = vsnprintf(buf, size as libc::c_ulong, fmt, args.as_va_list());
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn _str_concat_private(
    mut first: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    let mut arg: *const libc::c_char = first;
    while !arg.is_null() {
        len = (len as libc::c_ulong).wrapping_add(strlen(arg)) as size_t as size_t;
        arg = args_0.arg::<*const libc::c_char>();
    }
    let mut str: *mut libc::c_char = malloc(
        len
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = str;
    let mut args_1: ::std::ffi::VaListImpl;
    args_1 = args.clone();
    let mut arg_0: *const libc::c_char = first;
    while !arg_0.is_null() {
        while *arg_0 != 0 {
            let fresh2 = arg_0;
            arg_0 = arg_0.offset(1);
            let fresh3 = ptr;
            ptr = ptr.offset(1);
            *fresh3 = *fresh2;
        }
        arg_0 = args_1.arg::<*const libc::c_char>();
    }
    *ptr = '\u{0}' as i32 as libc::c_char;
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn str_diff_idx(
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut lena: size_t = strlen(a);
    let mut lenb: size_t = strlen(b);
    let mut minlen: size_t = if lena < lenb { lena } else { lenb };
    while (i as libc::c_ulong) < minlen {
        if *a.offset(i as isize) as libc::c_int != *b.offset(i as isize) as libc::c_int {
            break;
        }
        i += 1;
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn str_split(
    mut str: *const libc::c_char,
    mut delim: *const libc::c_char,
) -> *mut zarray_t {
    let mut parts: *mut zarray_t = zarray_create(
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    );
    let mut sb: *mut string_buffer_t = string_buffer_create();
    let mut delim_len: size_t = strlen(delim);
    let mut len: size_t = strlen(str);
    let mut pos: size_t = 0 as libc::c_int as size_t;
    while pos < len {
        if str_starts_with(&*str.offset(pos as isize), delim) as libc::c_int != 0
            && delim_len > 0 as libc::c_int as libc::c_ulong
        {
            pos = (pos as libc::c_ulong).wrapping_add(delim_len) as size_t as size_t;
            if string_buffer_size(sb) > 0 as libc::c_int as libc::c_ulong {
                let mut part: *mut libc::c_char = string_buffer_to_string(sb);
                zarray_add(
                    parts,
                    &mut part as *mut *mut libc::c_char as *const libc::c_void,
                );
            }
            string_buffer_reset(sb);
        } else {
            string_buffer_append(sb, *str.offset(pos as isize));
            pos = pos.wrapping_add(1);
        }
    }
    if string_buffer_size(sb) > 0 as libc::c_int as libc::c_ulong {
        let mut part_0: *mut libc::c_char = string_buffer_to_string(sb);
        zarray_add(parts, &mut part_0 as *mut *mut libc::c_char as *const libc::c_void);
    }
    string_buffer_destroy(sb);
    return parts;
}
#[no_mangle]
pub unsafe extern "C" fn str_split_spaces(
    mut str: *const libc::c_char,
) -> *mut zarray_t {
    let mut parts: *mut zarray_t = zarray_create(
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    );
    let mut len: size_t = strlen(str);
    let mut pos: size_t = 0 as libc::c_int as size_t;
    while pos < len {
        while pos < len && *str.offset(pos as isize) as libc::c_int == ' ' as i32 {
            pos = pos.wrapping_add(1);
        }
        if pos < len {
            let mut off0: size_t = pos;
            while pos < len && *str.offset(pos as isize) as libc::c_int != ' ' as i32 {
                pos = pos.wrapping_add(1);
            }
            let mut off1: size_t = pos;
            let mut len_off: size_t = off1.wrapping_sub(off0);
            let mut tok: *mut libc::c_char = malloc(
                len_off.wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            memcpy(
                tok as *mut libc::c_void,
                &*str.offset(off0 as isize) as *const libc::c_char
                    as *const libc::c_void,
                len_off,
            );
            *tok.offset(len_off as isize) = 0 as libc::c_int as libc::c_char;
            zarray_add(parts, &mut tok as *mut *mut libc::c_char as *const libc::c_void);
        }
    }
    return parts;
}
#[no_mangle]
pub unsafe extern "C" fn str_split_destroy(mut za: *mut zarray_t) {
    if za.is_null() {
        return;
    }
    zarray_vmap(
        za,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ())),
    );
    zarray_destroy(za);
}
#[no_mangle]
pub unsafe extern "C" fn str_trim(mut str: *mut libc::c_char) -> *mut libc::c_char {
    return str_lstrip(str_rstrip(str));
}
#[no_mangle]
pub unsafe extern "C" fn str_lstrip(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = str;
    let mut end: *mut libc::c_char = str.offset(strlen(str) as isize);
    while ptr != end
        && *(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        ptr = ptr.offset(1);
    }
    memmove(
        str as *mut libc::c_void,
        ptr as *const libc::c_void,
        (strlen(ptr)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn str_rstrip(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = str
        .offset(strlen(str) as isize)
        .offset(-(1 as libc::c_int as isize));
    while ptr.offset(1 as libc::c_int as isize) != str
        && *(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        ptr = ptr.offset(-1);
    }
    *ptr.offset(1 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn str_indexof(
    mut haystack: *const libc::c_char,
    mut needle: *const libc::c_char,
) -> libc::c_int {
    let mut hlen: libc::c_int = strlen(haystack) as libc::c_int;
    let mut nlen: libc::c_int = strlen(needle) as libc::c_int;
    if nlen > hlen {
        return -(1 as libc::c_int);
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i <= hlen - nlen {
        if strncmp(&*haystack.offset(i as isize), needle, nlen as libc::c_ulong) == 0 {
            return i;
        }
        i += 1;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn str_last_indexof(
    mut haystack: *const libc::c_char,
    mut needle: *const libc::c_char,
) -> libc::c_int {
    let mut hlen: libc::c_int = strlen(haystack) as libc::c_int;
    let mut nlen: libc::c_int = strlen(needle) as libc::c_int;
    let mut last_index: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i <= hlen - nlen {
        if strncmp(&*haystack.offset(i as isize), needle, nlen as libc::c_ulong) == 0 {
            last_index = i;
        }
        i += 1;
    }
    return last_index;
}
#[no_mangle]
pub unsafe extern "C" fn str_tolowercase(mut s: *mut libc::c_char) -> *mut libc::c_char {
    let mut slen: size_t = strlen(s);
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_ulong) < slen {
        if *s.offset(i as isize) as libc::c_int >= 'A' as i32
            && *s.offset(i as isize) as libc::c_int <= 'Z' as i32
        {
            *s
                .offset(
                    i as isize,
                ) = (*s.offset(i as isize) as libc::c_int + 'a' as i32 - 'A' as i32)
                as libc::c_char;
        }
        i += 1;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn str_touppercase(mut s: *mut libc::c_char) -> *mut libc::c_char {
    let mut slen: size_t = strlen(s);
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_ulong) < slen {
        if *s.offset(i as isize) as libc::c_int >= 'a' as i32
            && *s.offset(i as isize) as libc::c_int <= 'z' as i32
        {
            *s
                .offset(
                    i as isize,
                ) = (*s.offset(i as isize) as libc::c_int - ('a' as i32 - 'A' as i32))
                as libc::c_char;
        }
        i += 1;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn string_buffer_create() -> *mut string_buffer_t {
    let mut sb: *mut string_buffer_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<string_buffer_t>() as libc::c_ulong,
    ) as *mut string_buffer_t;
    (*sb).alloc = 32 as libc::c_int;
    let ref mut fresh4 = (*sb).s;
    *fresh4 = calloc((*sb).alloc as libc::c_ulong, 1 as libc::c_int as libc::c_ulong)
        as *mut libc::c_char;
    return sb;
}
#[no_mangle]
pub unsafe extern "C" fn string_buffer_destroy(mut sb: *mut string_buffer_t) {
    if sb.is_null() {
        return;
    }
    if !((*sb).s).is_null() {
        free((*sb).s as *mut libc::c_void);
    }
    memset(
        sb as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<string_buffer_t>() as libc::c_ulong,
    );
    free(sb as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn string_buffer_append(
    mut sb: *mut string_buffer_t,
    mut c: libc::c_char,
) {
    if ((*sb).size).wrapping_add(2 as libc::c_int as libc::c_ulong)
        >= (*sb).alloc as libc::c_ulong
    {
        (*sb).alloc *= 2 as libc::c_int;
        let ref mut fresh5 = (*sb).s;
        *fresh5 = realloc((*sb).s as *mut libc::c_void, (*sb).alloc as libc::c_ulong)
            as *mut libc::c_char;
    }
    let ref mut fresh6 = (*sb).size;
    let fresh7 = *fresh6;
    *fresh6 = (*fresh6).wrapping_add(1);
    *((*sb).s).offset(fresh7 as isize) = c;
    *((*sb).s).offset((*sb).size as isize) = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn string_buffer_pop_back(
    mut sb: *mut string_buffer_t,
) -> libc::c_char {
    if (*sb).size == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_char;
    }
    let ref mut fresh8 = (*sb).size;
    *fresh8 = (*fresh8).wrapping_sub(1);
    let mut back: libc::c_char = *((*sb).s).offset(*fresh8 as isize);
    *((*sb).s).offset((*sb).size as isize) = 0 as libc::c_int as libc::c_char;
    return back;
}
#[no_mangle]
pub unsafe extern "C" fn string_buffer_appendf(
    mut sb: *mut string_buffer_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut size: libc::c_int = 16 as libc::c_int;
    let mut buf: *mut libc::c_char = malloc(
        (size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut returnsize: libc::c_int = 0;
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    returnsize = vsnprintf(buf, size as libc::c_ulong, fmt, args_0.as_va_list());
    if returnsize >= size {
        free(buf as *mut libc::c_void);
        size = returnsize + 1 as libc::c_int;
        buf = malloc(
            (size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        args_0 = args.clone();
        returnsize = vsnprintf(buf, size as libc::c_ulong, fmt, args_0.as_va_list());
    }
    string_buffer_append_string(sb, buf);
    free(buf as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn string_buffer_append_string(
    mut sb: *mut string_buffer_t,
    mut str: *const libc::c_char,
) {
    let mut len: size_t = strlen(str);
    while ((*sb).size).wrapping_add(len).wrapping_add(1 as libc::c_int as libc::c_ulong)
        >= (*sb).alloc as libc::c_ulong
    {
        (*sb).alloc *= 2 as libc::c_int;
        let ref mut fresh9 = (*sb).s;
        *fresh9 = realloc((*sb).s as *mut libc::c_void, (*sb).alloc as libc::c_ulong)
            as *mut libc::c_char;
    }
    memcpy(
        &mut *((*sb).s).offset((*sb).size as isize) as *mut libc::c_char
            as *mut libc::c_void,
        str as *const libc::c_void,
        len,
    );
    let ref mut fresh10 = (*sb).size;
    *fresh10 = (*fresh10 as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    *((*sb).s).offset((*sb).size as isize) = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn string_buffer_ends_with(
    mut sb: *mut string_buffer_t,
    mut str: *const libc::c_char,
) -> bool {
    return str_ends_with((*sb).s, str);
}
#[no_mangle]
pub unsafe extern "C" fn string_buffer_to_string(
    mut sb: *mut string_buffer_t,
) -> *mut libc::c_char {
    return strdup((*sb).s);
}
#[no_mangle]
pub unsafe extern "C" fn string_buffer_size(mut sb: *mut string_buffer_t) -> size_t {
    return (*sb).size;
}
#[no_mangle]
pub unsafe extern "C" fn string_buffer_reset(mut sb: *mut string_buffer_t) {
    *((*sb).s).offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    (*sb).size = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn string_feeder_create(
    mut str: *const libc::c_char,
) -> *mut string_feeder_t {
    let mut sf: *mut string_feeder_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<string_feeder_t>() as libc::c_ulong,
    ) as *mut string_feeder_t;
    let ref mut fresh11 = (*sf).s;
    *fresh11 = strdup(str);
    (*sf).len = strlen((*sf).s);
    (*sf).line = 1 as libc::c_int;
    (*sf).col = 0 as libc::c_int;
    (*sf).pos = 0 as libc::c_int as size_t;
    return sf;
}
#[no_mangle]
pub unsafe extern "C" fn string_feeder_get_line(
    mut sf: *mut string_feeder_t,
) -> libc::c_int {
    return (*sf).line;
}
#[no_mangle]
pub unsafe extern "C" fn string_feeder_get_column(
    mut sf: *mut string_feeder_t,
) -> libc::c_int {
    return (*sf).col;
}
#[no_mangle]
pub unsafe extern "C" fn string_feeder_destroy(mut sf: *mut string_feeder_t) {
    if sf.is_null() {
        return;
    }
    free((*sf).s as *mut libc::c_void);
    memset(
        sf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<string_feeder_t>() as libc::c_ulong,
    );
    free(sf as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn string_feeder_has_next(mut sf: *mut string_feeder_t) -> bool {
    return *((*sf).s).offset((*sf).pos as isize) as libc::c_int != 0 as libc::c_int
        && (*sf).pos <= (*sf).len;
}
#[no_mangle]
pub unsafe extern "C" fn string_feeder_next(
    mut sf: *mut string_feeder_t,
) -> libc::c_char {
    let ref mut fresh12 = (*sf).pos;
    let fresh13 = *fresh12;
    *fresh12 = (*fresh12).wrapping_add(1);
    let mut c: libc::c_char = *((*sf).s).offset(fresh13 as isize);
    if c as libc::c_int == '\n' as i32 {
        let ref mut fresh14 = (*sf).line;
        *fresh14 += 1;
        (*sf).col = 0 as libc::c_int;
    } else {
        let ref mut fresh15 = (*sf).col;
        *fresh15 += 1;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn string_feeder_next_length(
    mut sf: *mut string_feeder_t,
    mut length: size_t,
) -> *mut libc::c_char {
    if ((*sf).pos).wrapping_add(length) > (*sf).len {
        length = ((*sf).len).wrapping_sub((*sf).pos);
    }
    let mut substr: *mut libc::c_char = calloc(
        length.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
    ) as *mut libc::c_char;
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_ulong) < length {
        *substr.offset(i as isize) = string_feeder_next(sf);
        i += 1;
    }
    return substr;
}
#[no_mangle]
pub unsafe extern "C" fn string_feeder_peek(
    mut sf: *mut string_feeder_t,
) -> libc::c_char {
    return *((*sf).s).offset((*sf).pos as isize);
}
#[no_mangle]
pub unsafe extern "C" fn string_feeder_peek_length(
    mut sf: *mut string_feeder_t,
    mut length: size_t,
) -> *mut libc::c_char {
    if ((*sf).pos).wrapping_add(length) > (*sf).len {
        length = ((*sf).len).wrapping_sub((*sf).pos);
    }
    let mut substr: *mut libc::c_char = calloc(
        length.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
    ) as *mut libc::c_char;
    memcpy(
        substr as *mut libc::c_void,
        &mut *((*sf).s).offset((*sf).pos as isize) as *mut libc::c_char
            as *const libc::c_void,
        length.wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    return substr;
}
#[no_mangle]
pub unsafe extern "C" fn string_feeder_starts_with(
    mut sf: *mut string_feeder_t,
    mut str: *const libc::c_char,
) -> bool {
    return str_starts_with(&mut *((*sf).s).offset((*sf).pos as isize), str);
}
#[no_mangle]
pub unsafe extern "C" fn string_feeder_require(
    mut sf: *mut string_feeder_t,
    mut str: *const libc::c_char,
) {
    let mut len: size_t = strlen(str);
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_ulong) < len {
        let mut c: libc::c_char = string_feeder_next(sf);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn str_ends_with(
    mut haystack: *const libc::c_char,
    mut needle: *const libc::c_char,
) -> bool {
    let mut lens: size_t = strlen(haystack);
    let mut lenneedle: size_t = strlen(needle);
    if lenneedle > lens {
        return 0 as libc::c_int != 0;
    }
    return strncmp(
        &*haystack.offset(lens.wrapping_sub(lenneedle) as isize),
        needle,
        lenneedle,
    ) == 0;
}
#[inline]
unsafe extern "C" fn str_starts_with(
    mut haystack: *const libc::c_char,
    mut needle: *const libc::c_char,
) -> bool {
    let mut pos: libc::c_int = 0 as libc::c_int;
    while *haystack.offset(pos as isize) as libc::c_int
        == *needle.offset(pos as isize) as libc::c_int
        && *needle.offset(pos as isize) as libc::c_int != 0 as libc::c_int
    {
        pos += 1;
    }
    return *needle.offset(pos as isize) as libc::c_int == 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn str_starts_with_any(
    mut haystack: *const libc::c_char,
    mut needles: *mut *const libc::c_char,
    mut num_needles: libc::c_int,
) -> bool {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < num_needles {
        if str_starts_with(haystack, *needles.offset(i as isize)) {
            return 1 as libc::c_int != 0;
        }
        i += 1;
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn str_matches_any(
    mut haystack: *const libc::c_char,
    mut needles: *mut *const libc::c_char,
    mut num_needles: libc::c_int,
) -> bool {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < num_needles {
        if strcmp(haystack, *needles.offset(i as isize)) == 0 {
            return 1 as libc::c_int != 0;
        }
        i += 1;
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn str_substring(
    mut str: *const libc::c_char,
    mut startidx: size_t,
    mut endidx: libc::c_long,
) -> *mut libc::c_char {
    if endidx < 0 as libc::c_int as libc::c_long {
        endidx = strlen(str) as libc::c_long;
    }
    let mut blen: size_t = (endidx as libc::c_ulong).wrapping_sub(startidx);
    let mut b: *mut libc::c_char = malloc(
        blen.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        b as *mut libc::c_void,
        &*str.offset(startidx as isize) as *const libc::c_char as *const libc::c_void,
        blen,
    );
    *b.offset(blen as isize) = 0 as libc::c_int as libc::c_char;
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn str_replace(
    mut haystack: *const libc::c_char,
    mut needle: *const libc::c_char,
    mut replacement: *const libc::c_char,
) -> *mut libc::c_char {
    let mut sb: *mut string_buffer_t = string_buffer_create();
    let mut haystack_len: size_t = strlen(haystack);
    let mut needle_len: size_t = strlen(needle);
    let mut pos: libc::c_int = 0 as libc::c_int;
    while (pos as libc::c_ulong) < haystack_len {
        if needle_len > 0 as libc::c_int as libc::c_ulong
            && str_starts_with(&*haystack.offset(pos as isize), needle) as libc::c_int
                != 0
        {
            string_buffer_append_string(sb, replacement);
            pos = (pos as libc::c_ulong).wrapping_add(needle_len) as libc::c_int
                as libc::c_int;
        } else {
            string_buffer_append(sb, *haystack.offset(pos as isize));
            pos += 1;
        }
    }
    if needle_len == 0 as libc::c_int as libc::c_ulong
        && haystack_len == 0 as libc::c_int as libc::c_ulong
    {
        string_buffer_append_string(sb, replacement);
    }
    let mut res: *mut libc::c_char = string_buffer_to_string(sb);
    string_buffer_destroy(sb);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn str_replace_many(
    mut _haystack: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    let mut haystack: *mut libc::c_char = strdup(_haystack);
    loop {
        let mut needle: *mut libc::c_char = ap.arg::<*mut libc::c_char>();
        if needle.is_null() {
            break;
        }
        let mut replacement: *mut libc::c_char = ap.arg::<*mut libc::c_char>();
        let mut tmp: *mut libc::c_char = str_replace(haystack, needle, replacement);
        free(haystack as *mut libc::c_void);
        haystack = tmp;
    }
    return haystack;
}
unsafe extern "C" fn buffer_appendf(
    mut _buf: *mut *mut libc::c_char,
    mut bufpos: *mut libc::c_int,
    mut fmt: *mut libc::c_void,
    mut args: ...
) {
    let mut buf: *mut libc::c_char = *_buf;
    let mut ap: ::std::ffi::VaListImpl;
    let mut salloc: libc::c_int = 128 as libc::c_int;
    let mut s: *mut libc::c_char = malloc(salloc as libc::c_ulong) as *mut libc::c_char;
    ap = args.clone();
    let mut slen: libc::c_int = vsnprintf(
        s,
        salloc as libc::c_ulong,
        fmt as *const libc::c_char,
        ap.as_va_list(),
    );
    if slen >= salloc {
        s = realloc(s as *mut libc::c_void, (slen + 1 as libc::c_int) as libc::c_ulong)
            as *mut libc::c_char;
        ap = args.clone();
        vsprintf(s, fmt as *const libc::c_char, ap.as_va_list());
    }
    buf = realloc(
        buf as *mut libc::c_void,
        (*bufpos + slen + 1 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_char;
    *_buf = buf;
    memcpy(
        &mut *buf.offset(*bufpos as isize) as *mut libc::c_char as *mut libc::c_void,
        s as *const libc::c_void,
        (slen + 1 as libc::c_int) as libc::c_ulong,
    );
    *bufpos += slen;
    free(s as *mut libc::c_void);
}
unsafe extern "C" fn is_variable_character(mut c: libc::c_char) -> libc::c_int {
    if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32 {
        return 1 as libc::c_int;
    }
    if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'Z' as i32 {
        return 1 as libc::c_int;
    }
    if c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32 {
        return 1 as libc::c_int;
    }
    if c as libc::c_int == '_' as i32 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn str_expand_envs(
    mut in_0: *const libc::c_char,
) -> *mut libc::c_char {
    let mut inlen: size_t = strlen(in_0);
    let mut inpos: size_t = 0 as libc::c_int as size_t;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outpos: libc::c_int = 0 as libc::c_int;
    while inpos < inlen {
        if *in_0.offset(inpos as isize) as libc::c_int != '$' as i32 {
            buffer_appendf(
                &mut out as *mut *mut libc::c_char,
                &mut outpos as *mut libc::c_int,
                b"%c\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
                *in_0.offset(inpos as isize) as libc::c_int,
            );
            inpos = inpos.wrapping_add(1);
        } else {
            inpos = inpos.wrapping_add(1);
            let mut varname: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut varnamepos: libc::c_int = 0 as libc::c_int;
            while inpos < inlen
                && is_variable_character(*in_0.offset(inpos as isize)) != 0
            {
                buffer_appendf(
                    &mut varname as *mut *mut libc::c_char,
                    &mut varnamepos as *mut libc::c_int,
                    b"%c\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
                    *in_0.offset(inpos as isize) as libc::c_int,
                );
                inpos = inpos.wrapping_add(1);
            }
            let mut env: *mut libc::c_char = getenv(varname);
            if !env.is_null() {
                buffer_appendf(
                    &mut out as *mut *mut libc::c_char,
                    &mut outpos as *mut libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
                    env,
                );
            }
            free(varname as *mut libc::c_void);
        }
    }
    return out;
}
