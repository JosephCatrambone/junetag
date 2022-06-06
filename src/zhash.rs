use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub struct zhash {
    pub keysz: size_t,
    pub valuesz: size_t,
    pub entrysz: libc::c_int,
    pub hash: Option::<unsafe extern "C" fn(*const libc::c_void) -> uint32_t>,
    pub equals: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
    pub size: libc::c_int,
    pub entries: *mut libc::c_char,
    pub nentries: libc::c_int,
}
pub type zhash_t = zhash;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zhash_iterator {
    pub zh: *mut zhash_t,
    pub czh: *const zhash_t,
    pub last_entry: libc::c_int,
}
pub type zhash_iterator_t = zhash_iterator;
#[derive(Copy, Clone)]
#[repr(C)]
pub union uintpointer {
    pub p: *const libc::c_void,
    pub i: uint32_t,
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
pub unsafe extern "C" fn zhash_create_capacity(
    mut keysz: size_t,
    mut valuesz: size_t,
    mut hash: Option::<unsafe extern "C" fn(*const libc::c_void) -> uint32_t>,
    mut equals: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
    mut capacity: libc::c_int,
) -> *mut zhash_t {
    let mut _nentries: libc::c_int = 4 as libc::c_int * capacity;
    if _nentries < 8 as libc::c_int {
        _nentries = 8 as libc::c_int;
    }
    let mut nentries: libc::c_int = _nentries;
    if nentries & nentries - 1 as libc::c_int != 0 as libc::c_int {
        nentries = 8 as libc::c_int;
        while nentries < _nentries {
            nentries *= 2 as libc::c_int;
        }
    }
    let mut zh: *mut zhash_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<zhash_t>() as libc::c_ulong,
    ) as *mut zhash_t;
    (*zh).keysz = keysz;
    (*zh).valuesz = valuesz;
    let ref mut fresh2 = (*zh).hash;
    *fresh2 = hash;
    let ref mut fresh3 = (*zh).equals;
    *fresh3 = equals;
    (*zh).nentries = nentries;
    (*zh)
        .entrysz = (1 as libc::c_int as libc::c_ulong)
        .wrapping_add((*zh).keysz)
        .wrapping_add((*zh).valuesz) as libc::c_int;
    let ref mut fresh4 = (*zh).entries;
    *fresh4 = calloc((*zh).nentries as libc::c_ulong, (*zh).entrysz as libc::c_ulong)
        as *mut libc::c_char;
    (*zh).nentries = nentries;
    return zh;
}
#[no_mangle]
pub unsafe extern "C" fn zhash_create(
    mut keysz: size_t,
    mut valuesz: size_t,
    mut hash: Option::<unsafe extern "C" fn(*const libc::c_void) -> uint32_t>,
    mut equals: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
) -> *mut zhash_t {
    return zhash_create_capacity(keysz, valuesz, hash, equals, 8 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn zhash_destroy(mut zh: *mut zhash_t) {
    if zh.is_null() {
        return;
    }
    free((*zh).entries as *mut libc::c_void);
    free(zh as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn zhash_size(mut zh: *const zhash_t) -> libc::c_int {
    return (*zh).size;
}
#[no_mangle]
pub unsafe extern "C" fn zhash_clear(mut zh: *mut zhash_t) {
    memset(
        (*zh).entries as *mut libc::c_void,
        0 as libc::c_int,
        ((*zh).nentries * (*zh).entrysz) as libc::c_ulong,
    );
    (*zh).size = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zhash_get_volatile(
    mut zh: *const zhash_t,
    mut key: *const libc::c_void,
    mut out_value: *mut libc::c_void,
) -> libc::c_int {
    let mut code: uint32_t = ((*zh).hash).expect("non-null function pointer")(key);
    let mut entry_idx: uint32_t = code
        & ((*zh).nentries - 1 as libc::c_int) as libc::c_uint;
    while *((*zh).entries)
        .offset(entry_idx.wrapping_mul((*zh).entrysz as libc::c_uint) as isize) != 0
    {
        let mut this_key: *mut libc::c_void = &mut *((*zh).entries)
            .offset(
                entry_idx
                    .wrapping_mul((*zh).entrysz as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) as *mut libc::c_char as *mut libc::c_void;
        if ((*zh).equals).expect("non-null function pointer")(key, this_key) != 0 {
            let ref mut fresh5 = *(out_value as *mut *mut libc::c_void);
            *fresh5 = &mut *((*zh).entries)
                .offset(
                    (entry_idx
                        .wrapping_mul((*zh).entrysz as libc::c_uint)
                        .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
                        .wrapping_add((*zh).keysz) as isize,
                ) as *mut libc::c_char as *mut libc::c_void;
            return 1 as libc::c_int;
        }
        entry_idx = entry_idx.wrapping_add(1 as libc::c_int as libc::c_uint)
            & ((*zh).nentries - 1 as libc::c_int) as libc::c_uint;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zhash_get(
    mut zh: *const zhash_t,
    mut key: *const libc::c_void,
    mut out_value: *mut libc::c_void,
) -> libc::c_int {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    if zhash_get_volatile(
        zh,
        key,
        &mut tmp as *mut *mut libc::c_void as *mut libc::c_void,
    ) != 0
    {
        memcpy(out_value, tmp, (*zh).valuesz);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zhash_put(
    mut zh: *mut zhash_t,
    mut key: *const libc::c_void,
    mut value: *const libc::c_void,
    mut oldkey: *mut libc::c_void,
    mut oldvalue: *mut libc::c_void,
) -> libc::c_int {
    let mut code: uint32_t = ((*zh).hash).expect("non-null function pointer")(key);
    let mut entry_idx: uint32_t = code
        & ((*zh).nentries - 1 as libc::c_int) as libc::c_uint;
    while *((*zh).entries)
        .offset(entry_idx.wrapping_mul((*zh).entrysz as libc::c_uint) as isize) != 0
    {
        let mut this_key: *mut libc::c_void = &mut *((*zh).entries)
            .offset(
                entry_idx
                    .wrapping_mul((*zh).entrysz as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) as *mut libc::c_char as *mut libc::c_void;
        let mut this_value: *mut libc::c_void = &mut *((*zh).entries)
            .offset(
                (entry_idx
                    .wrapping_mul((*zh).entrysz as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
                    .wrapping_add((*zh).keysz) as isize,
            ) as *mut libc::c_char as *mut libc::c_void;
        if ((*zh).equals).expect("non-null function pointer")(key, this_key) != 0 {
            if !oldkey.is_null() {
                memcpy(oldkey, this_key, (*zh).keysz);
            }
            if !oldvalue.is_null() {
                memcpy(oldvalue, this_value, (*zh).valuesz);
            }
            memcpy(this_key, key, (*zh).keysz);
            memcpy(this_value, value, (*zh).valuesz);
            *((*zh).entries)
                .offset(
                    entry_idx.wrapping_mul((*zh).entrysz as libc::c_uint) as isize,
                ) = 1 as libc::c_int as libc::c_char;
            return 1 as libc::c_int;
        }
        entry_idx = entry_idx.wrapping_add(1 as libc::c_int as libc::c_uint)
            & ((*zh).nentries - 1 as libc::c_int) as libc::c_uint;
    }
    *((*zh).entries)
        .offset(
            entry_idx.wrapping_mul((*zh).entrysz as libc::c_uint) as isize,
        ) = 1 as libc::c_int as libc::c_char;
    memcpy(
        &mut *((*zh).entries)
            .offset(
                entry_idx
                    .wrapping_mul((*zh).entrysz as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) as *mut libc::c_char as *mut libc::c_void,
        key,
        (*zh).keysz,
    );
    memcpy(
        &mut *((*zh).entries)
            .offset(
                (entry_idx
                    .wrapping_mul((*zh).entrysz as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
                    .wrapping_add((*zh).keysz) as isize,
            ) as *mut libc::c_char as *mut libc::c_void,
        value,
        (*zh).valuesz,
    );
    let ref mut fresh6 = (*zh).size;
    *fresh6 += 1;
    if (*zh).nentries < 2 as libc::c_int * (*zh).size {
        let mut newhash: *mut zhash_t = zhash_create_capacity(
            (*zh).keysz,
            (*zh).valuesz,
            (*zh).hash,
            (*zh).equals,
            (*zh).size,
        );
        let mut idx: libc::c_int = 0 as libc::c_int;
        while idx < (*zh).nentries {
            if *((*zh).entries).offset((idx * (*zh).entrysz) as isize) != 0 {
                let mut this_key_0: *mut libc::c_void = &mut *((*zh).entries)
                    .offset((idx * (*zh).entrysz + 1 as libc::c_int) as isize)
                    as *mut libc::c_char as *mut libc::c_void;
                let mut this_value_0: *mut libc::c_void = &mut *((*zh).entries)
                    .offset(
                        ((idx * (*zh).entrysz + 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_add((*zh).keysz) as isize,
                    ) as *mut libc::c_char as *mut libc::c_void;
                zhash_put(
                    newhash,
                    this_key_0,
                    this_value_0,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                ) != 0;
            }
            idx += 1;
        }
        let mut tmp: zhash_t = zhash_t {
            keysz: 0,
            valuesz: 0,
            entrysz: 0,
            hash: None,
            equals: None,
            size: 0,
            entries: 0 as *mut libc::c_char,
            nentries: 0,
        };
        memcpy(
            &mut tmp as *mut zhash_t as *mut libc::c_void,
            zh as *const libc::c_void,
            ::std::mem::size_of::<zhash_t>() as libc::c_ulong,
        );
        memcpy(
            zh as *mut libc::c_void,
            newhash as *const libc::c_void,
            ::std::mem::size_of::<zhash_t>() as libc::c_ulong,
        );
        memcpy(
            newhash as *mut libc::c_void,
            &mut tmp as *mut zhash_t as *const libc::c_void,
            ::std::mem::size_of::<zhash_t>() as libc::c_ulong,
        );
        zhash_destroy(newhash);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zhash_remove(
    mut zh: *mut zhash_t,
    mut key: *const libc::c_void,
    mut old_key: *mut libc::c_void,
    mut old_value: *mut libc::c_void,
) -> libc::c_int {
    let mut code: uint32_t = ((*zh).hash).expect("non-null function pointer")(key);
    let mut entry_idx: uint32_t = code
        & ((*zh).nentries - 1 as libc::c_int) as libc::c_uint;
    while *((*zh).entries)
        .offset(entry_idx.wrapping_mul((*zh).entrysz as libc::c_uint) as isize) != 0
    {
        let mut this_key: *mut libc::c_void = &mut *((*zh).entries)
            .offset(
                entry_idx
                    .wrapping_mul((*zh).entrysz as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) as *mut libc::c_char as *mut libc::c_void;
        let mut this_value: *mut libc::c_void = &mut *((*zh).entries)
            .offset(
                (entry_idx
                    .wrapping_mul((*zh).entrysz as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
                    .wrapping_add((*zh).keysz) as isize,
            ) as *mut libc::c_char as *mut libc::c_void;
        if ((*zh).equals).expect("non-null function pointer")(key, this_key) != 0 {
            if !old_key.is_null() {
                memcpy(old_key, this_key, (*zh).keysz);
            }
            if !old_value.is_null() {
                memcpy(old_value, this_value, (*zh).valuesz);
            }
            *((*zh).entries)
                .offset(
                    entry_idx.wrapping_mul((*zh).entrysz as libc::c_uint) as isize,
                ) = 0 as libc::c_int as libc::c_char;
            let ref mut fresh7 = (*zh).size;
            *fresh7 -= 1;
            loop {
                entry_idx = entry_idx.wrapping_add(1 as libc::c_int as libc::c_uint)
                    & ((*zh).nentries - 1 as libc::c_int) as libc::c_uint;
                if !(*((*zh).entries)
                    .offset(
                        entry_idx.wrapping_mul((*zh).entrysz as libc::c_uint) as isize,
                    ) != 0)
                {
                    break;
                }
                let mut tmp: *mut libc::c_char = malloc(
                    (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                        .wrapping_mul((*zh).entrysz as libc::c_ulong),
                ) as *mut libc::c_char;
                memcpy(
                    tmp as *mut libc::c_void,
                    &mut *((*zh).entries)
                        .offset(
                            entry_idx.wrapping_mul((*zh).entrysz as libc::c_uint)
                                as isize,
                        ) as *mut libc::c_char as *const libc::c_void,
                    (*zh).entrysz as libc::c_ulong,
                );
                *((*zh).entries)
                    .offset(
                        entry_idx.wrapping_mul((*zh).entrysz as libc::c_uint) as isize,
                    ) = 0 as libc::c_int as libc::c_char;
                let ref mut fresh8 = (*zh).size;
                *fresh8 -= 1;
                zhash_put(
                    zh,
                    &mut *tmp.offset(1 as libc::c_int as isize) as *mut libc::c_char
                        as *const libc::c_void,
                    &mut *tmp
                        .offset(
                            (1 as libc::c_int as libc::c_ulong).wrapping_add((*zh).keysz)
                                as isize,
                        ) as *mut libc::c_char as *const libc::c_void,
                    0 as *mut libc::c_void,
                    0 as *mut libc::c_void,
                ) != 0;
                free(tmp as *mut libc::c_void);
            }
            return 1 as libc::c_int;
        }
        entry_idx = entry_idx.wrapping_add(1 as libc::c_int as libc::c_uint)
            & ((*zh).nentries - 1 as libc::c_int) as libc::c_uint;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zhash_copy(mut zh: *const zhash_t) -> *mut zhash_t {
    let mut newhash: *mut zhash_t = zhash_create_capacity(
        (*zh).keysz,
        (*zh).valuesz,
        (*zh).hash,
        (*zh).equals,
        (*zh).size,
    );
    let mut entry_idx: libc::c_int = 0 as libc::c_int;
    while entry_idx < (*zh).nentries {
        if *((*zh).entries).offset((entry_idx * (*zh).entrysz) as isize) != 0 {
            let mut this_key: *mut libc::c_void = &mut *((*zh).entries)
                .offset((entry_idx * (*zh).entrysz + 1 as libc::c_int) as isize)
                as *mut libc::c_char as *mut libc::c_void;
            let mut this_value: *mut libc::c_void = &mut *((*zh).entries)
                .offset(
                    ((entry_idx * (*zh).entrysz + 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_add((*zh).keysz) as isize,
                ) as *mut libc::c_char as *mut libc::c_void;
            zhash_put(
                newhash,
                this_key,
                this_value,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            ) != 0;
        }
        entry_idx += 1;
    }
    return newhash;
}
#[no_mangle]
pub unsafe extern "C" fn zhash_contains(
    mut zh: *const zhash_t,
    mut key: *const libc::c_void,
) -> libc::c_int {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    return zhash_get_volatile(
        zh,
        key,
        &mut tmp as *mut *mut libc::c_void as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn zhash_iterator_init(
    mut zh: *mut zhash_t,
    mut zit: *mut zhash_iterator_t,
) {
    let ref mut fresh9 = (*zit).zh;
    *fresh9 = zh;
    let ref mut fresh10 = (*zit).czh;
    *fresh10 = zh;
    (*zit).last_entry = -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn zhash_iterator_init_const(
    mut zh: *const zhash_t,
    mut zit: *mut zhash_iterator_t,
) {
    let ref mut fresh11 = (*zit).zh;
    *fresh11 = 0 as *mut zhash_t;
    let ref mut fresh12 = (*zit).czh;
    *fresh12 = zh;
    (*zit).last_entry = -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn zhash_iterator_next_volatile(
    mut zit: *mut zhash_iterator_t,
    mut outkey: *mut libc::c_void,
    mut outvalue: *mut libc::c_void,
) -> libc::c_int {
    let mut zh: *const zhash_t = (*zit).czh;
    loop {
        if (*zit).last_entry + 1 as libc::c_int >= (*zh).nentries {
            return 0 as libc::c_int;
        }
        let ref mut fresh13 = (*zit).last_entry;
        *fresh13 += 1;
        if *((*zh).entries).offset(((*zit).last_entry * (*zh).entrysz) as isize) != 0 {
            let mut this_key: *mut libc::c_void = &mut *((*zh).entries)
                .offset(((*zit).last_entry * (*zh).entrysz + 1 as libc::c_int) as isize)
                as *mut libc::c_char as *mut libc::c_void;
            let mut this_value: *mut libc::c_void = &mut *((*zh).entries)
                .offset(
                    (((*zit).last_entry * (*zh).entrysz + 1 as libc::c_int)
                        as libc::c_ulong)
                        .wrapping_add((*zh).keysz) as isize,
                ) as *mut libc::c_char as *mut libc::c_void;
            if !outkey.is_null() {
                let ref mut fresh14 = *(outkey as *mut *mut libc::c_void);
                *fresh14 = this_key;
            }
            if !outvalue.is_null() {
                let ref mut fresh15 = *(outvalue as *mut *mut libc::c_void);
                *fresh15 = this_value;
            }
            return 1 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn zhash_iterator_next(
    mut zit: *mut zhash_iterator_t,
    mut outkey: *mut libc::c_void,
    mut outvalue: *mut libc::c_void,
) -> libc::c_int {
    let mut zh: *const zhash_t = (*zit).czh;
    let mut outkeyp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut outvaluep: *mut libc::c_void = 0 as *mut libc::c_void;
    if zhash_iterator_next_volatile(
        zit,
        &mut outkeyp as *mut *mut libc::c_void as *mut libc::c_void,
        &mut outvaluep as *mut *mut libc::c_void as *mut libc::c_void,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if !outkey.is_null() {
        memcpy(outkey, outkeyp, (*zh).keysz);
    }
    if !outvalue.is_null() {
        memcpy(outvalue, outvaluep, (*zh).valuesz);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zhash_iterator_remove(mut zit: *mut zhash_iterator_t) {
    let mut zh: *mut zhash_t = (*zit).zh;
    *((*zh).entries)
        .offset(
            ((*zit).last_entry * (*zh).entrysz) as isize,
        ) = 0 as libc::c_int as libc::c_char;
    let ref mut fresh16 = (*zh).size;
    *fresh16 -= 1;
    let mut entry_idx: libc::c_int = (*zit).last_entry + 1 as libc::c_int
        & (*zh).nentries - 1 as libc::c_int;
    while *((*zh).entries).offset((entry_idx * (*zh).entrysz) as isize) != 0 {
        let mut tmp: *mut libc::c_char = malloc(
            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul((*zh).entrysz as libc::c_ulong),
        ) as *mut libc::c_char;
        memcpy(
            tmp as *mut libc::c_void,
            &mut *((*zh).entries).offset((entry_idx * (*zh).entrysz) as isize)
                as *mut libc::c_char as *const libc::c_void,
            (*zh).entrysz as libc::c_ulong,
        );
        *((*zh).entries)
            .offset(
                (entry_idx * (*zh).entrysz) as isize,
            ) = 0 as libc::c_int as libc::c_char;
        let ref mut fresh17 = (*zh).size;
        *fresh17 -= 1;
        zhash_put(
            zh,
            &mut *tmp.offset(1 as libc::c_int as isize) as *mut libc::c_char
                as *const libc::c_void,
            &mut *tmp
                .offset(
                    (1 as libc::c_int as libc::c_ulong).wrapping_add((*zh).keysz)
                        as isize,
                ) as *mut libc::c_char as *const libc::c_void,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
        ) != 0;
        free(tmp as *mut libc::c_void);
        entry_idx = entry_idx + 1 as libc::c_int & (*zh).nentries - 1 as libc::c_int;
    }
    let ref mut fresh18 = (*zit).last_entry;
    *fresh18 -= 1;
}
#[no_mangle]
pub unsafe extern "C" fn zhash_map_keys(
    mut zh: *mut zhash_t,
    mut f: Option::<unsafe extern "C" fn() -> ()>,
) {
    if f.is_none() {
        return;
    }
    let mut itr: zhash_iterator_t = zhash_iterator_t {
        zh: 0 as *mut zhash_t,
        czh: 0 as *const zhash_t,
        last_entry: 0,
    };
    zhash_iterator_init(zh, &mut itr);
    let mut key: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    while zhash_iterator_next_volatile(
        &mut itr,
        &mut key as *mut *mut libc::c_void as *mut libc::c_void,
        &mut value as *mut *mut libc::c_void as *mut libc::c_void,
    ) != 0
    {
        ::std::mem::transmute::<_, fn(_)>(f.expect("non-null function pointer"))(key);
    }
}
#[no_mangle]
pub unsafe extern "C" fn zhash_vmap_keys(
    mut zh: *mut zhash_t,
    mut f: Option::<unsafe extern "C" fn() -> ()>,
) {
    if f.is_none() {
        return;
    }
    let mut itr: zhash_iterator_t = zhash_iterator_t {
        zh: 0 as *mut zhash_t,
        czh: 0 as *const zhash_t,
        last_entry: 0,
    };
    zhash_iterator_init(zh, &mut itr);
    let mut key: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    while zhash_iterator_next_volatile(
        &mut itr,
        &mut key as *mut *mut libc::c_void as *mut libc::c_void,
        &mut value as *mut *mut libc::c_void as *mut libc::c_void,
    ) != 0
    {
        let mut p: *mut libc::c_void = *(key as *mut *mut libc::c_void);
        ::std::mem::transmute::<_, fn(_)>(f.expect("non-null function pointer"))(p);
    }
}
#[no_mangle]
pub unsafe extern "C" fn zhash_map_values(
    mut zh: *mut zhash_t,
    mut f: Option::<unsafe extern "C" fn() -> ()>,
) {
    if f.is_none() {
        return;
    }
    let mut itr: zhash_iterator_t = zhash_iterator_t {
        zh: 0 as *mut zhash_t,
        czh: 0 as *const zhash_t,
        last_entry: 0,
    };
    zhash_iterator_init(zh, &mut itr);
    let mut key: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    while zhash_iterator_next_volatile(
        &mut itr,
        &mut key as *mut *mut libc::c_void as *mut libc::c_void,
        &mut value as *mut *mut libc::c_void as *mut libc::c_void,
    ) != 0
    {
        ::std::mem::transmute::<_, fn(_)>(f.expect("non-null function pointer"))(value);
    }
}
#[no_mangle]
pub unsafe extern "C" fn zhash_vmap_values(
    mut zh: *mut zhash_t,
    mut f: Option::<unsafe extern "C" fn() -> ()>,
) {
    if f.is_none() {
        return;
    }
    let mut itr: zhash_iterator_t = zhash_iterator_t {
        zh: 0 as *mut zhash_t,
        czh: 0 as *const zhash_t,
        last_entry: 0,
    };
    zhash_iterator_init(zh, &mut itr);
    let mut key: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    while zhash_iterator_next_volatile(
        &mut itr,
        &mut key as *mut *mut libc::c_void as *mut libc::c_void,
        &mut value as *mut *mut libc::c_void as *mut libc::c_void,
    ) != 0
    {
        let mut p: *mut libc::c_void = *(value as *mut *mut libc::c_void);
        ::std::mem::transmute::<_, fn(_)>(f.expect("non-null function pointer"))(p);
    }
}
#[no_mangle]
pub unsafe extern "C" fn zhash_keys(mut zh: *const zhash_t) -> *mut zarray_t {
    let mut za: *mut zarray_t = zarray_create((*zh).keysz);
    let mut itr: zhash_iterator_t = zhash_iterator_t {
        zh: 0 as *mut zhash_t,
        czh: 0 as *const zhash_t,
        last_entry: 0,
    };
    zhash_iterator_init_const(zh, &mut itr);
    let mut key: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    while zhash_iterator_next_volatile(
        &mut itr,
        &mut key as *mut *mut libc::c_void as *mut libc::c_void,
        &mut value as *mut *mut libc::c_void as *mut libc::c_void,
    ) != 0
    {
        zarray_add(za, key);
    }
    return za;
}
#[no_mangle]
pub unsafe extern "C" fn zhash_values(mut zh: *const zhash_t) -> *mut zarray_t {
    let mut za: *mut zarray_t = zarray_create((*zh).valuesz);
    let mut itr: zhash_iterator_t = zhash_iterator_t {
        zh: 0 as *mut zhash_t,
        czh: 0 as *const zhash_t,
        last_entry: 0,
    };
    zhash_iterator_init_const(zh, &mut itr);
    let mut key: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    while zhash_iterator_next_volatile(
        &mut itr,
        &mut key as *mut *mut libc::c_void as *mut libc::c_void,
        &mut value as *mut *mut libc::c_void as *mut libc::c_void,
    ) != 0
    {
        zarray_add(za, value);
    }
    return za;
}
#[no_mangle]
pub unsafe extern "C" fn zhash_uint32_hash(mut _a: *const libc::c_void) -> uint32_t {
    let mut a: uint32_t = *(_a as *mut uint32_t);
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn zhash_uint32_equals(
    mut _a: *const libc::c_void,
    mut _b: *const libc::c_void,
) -> libc::c_int {
    let mut a: uint32_t = *(_a as *mut uint32_t);
    let mut b: uint32_t = *(_b as *mut uint32_t);
    return (a == b) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zhash_uint64_hash(mut _a: *const libc::c_void) -> uint32_t {
    let mut a: uint64_t = *(_a as *mut uint64_t);
    return (a ^ a >> 32 as libc::c_int) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn zhash_uint64_equals(
    mut _a: *const libc::c_void,
    mut _b: *const libc::c_void,
) -> libc::c_int {
    let mut a: uint64_t = *(_a as *mut uint64_t);
    let mut b: uint64_t = *(_b as *mut uint64_t);
    return (a == b) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zhash_ptr_hash(mut a: *const libc::c_void) -> uint32_t {
    let mut ip: uintpointer = uintpointer {
        p: 0 as *const libc::c_void,
    };
    ip.p = *(a as *mut *mut libc::c_void);
    let mut hash: uint32_t = ip.i;
    hash ^= hash >> 7 as libc::c_int;
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn zhash_ptr_equals(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut ptra: *const libc::c_void = *(a as *mut *mut libc::c_void);
    let mut ptrb: *const libc::c_void = *(b as *mut *mut libc::c_void);
    return (ptra == ptrb) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zhash_str_equals(
    mut _a: *const libc::c_void,
    mut _b: *const libc::c_void,
) -> libc::c_int {
    let mut a: *mut libc::c_char = *(_a as *mut *mut libc::c_char);
    let mut b: *mut libc::c_char = *(_b as *mut *mut libc::c_char);
    return (strcmp(a, b) == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zhash_str_hash(mut _a: *const libc::c_void) -> uint32_t {
    let mut a: *mut libc::c_char = *(_a as *mut *mut libc::c_char);
    let mut hash: uint32_t = 0 as libc::c_int as uint32_t;
    while *a as libc::c_int != 0 as libc::c_int {
        hash = (hash as libc::c_uint)
            .wrapping_add(
                (hash << 1 as libc::c_int)
                    .wrapping_add(hash << 4 as libc::c_int)
                    .wrapping_add(hash << 7 as libc::c_int)
                    .wrapping_add(hash << 8 as libc::c_int)
                    .wrapping_add(hash << 24 as libc::c_int),
            ) as uint32_t as uint32_t;
        hash ^= *a as libc::c_uint;
        a = a.offset(1);
    }
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn zhash_debug(mut zh: *mut zhash_t) {
    let mut entry_idx: libc::c_int = 0 as libc::c_int;
    while entry_idx < (*zh).nentries {
        let mut k: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
        memcpy(
            &mut k as *mut *mut libc::c_char as *mut libc::c_void,
            &mut *((*zh).entries)
                .offset((entry_idx * (*zh).entrysz + 1 as libc::c_int) as isize)
                as *mut libc::c_char as *const libc::c_void,
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        );
        memcpy(
            &mut v as *mut *mut libc::c_char as *mut libc::c_void,
            &mut *((*zh).entries)
                .offset(
                    ((entry_idx * (*zh).entrysz + 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_add((*zh).keysz) as isize,
                ) as *mut libc::c_char as *const libc::c_void,
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        );
        printf(
            b"%d: %d, %s => %s\n\0" as *const u8 as *const libc::c_char,
            entry_idx,
            *((*zh).entries).offset((entry_idx * (*zh).entrysz) as isize) as libc::c_int,
            k,
            v,
        );
        entry_idx += 1;
    }
}
