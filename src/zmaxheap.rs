use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn random() -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int32_t = __int32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zmaxheap {
    pub el_sz: size_t,
    pub size: libc::c_int,
    pub alloc: libc::c_int,
    pub values: *mut libc::c_float,
    pub data: *mut libc::c_char,
    pub swap: Option::<
        unsafe extern "C" fn(*mut zmaxheap_t, libc::c_int, libc::c_int) -> (),
    >,
}
pub type zmaxheap_t = zmaxheap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zmaxheap_iterator {
    pub heap: *mut zmaxheap_t,
    pub in_0: libc::c_int,
    pub out: libc::c_int,
}
pub type zmaxheap_iterator_t = zmaxheap_iterator;
#[inline]
unsafe extern "C" fn swap_default(
    mut heap: *mut zmaxheap_t,
    mut a: libc::c_int,
    mut b: libc::c_int,
) {
    let mut t: libc::c_float = *((*heap).values).offset(a as isize);
    *((*heap).values).offset(a as isize) = *((*heap).values).offset(b as isize);
    *((*heap).values).offset(b as isize) = t;
    let mut tmp: *mut libc::c_char = malloc(
        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul((*heap).el_sz),
    ) as *mut libc::c_char;
    memcpy(
        tmp as *mut libc::c_void,
        &mut *((*heap).data)
            .offset((a as libc::c_ulong).wrapping_mul((*heap).el_sz) as isize)
            as *mut libc::c_char as *const libc::c_void,
        (*heap).el_sz,
    );
    memcpy(
        &mut *((*heap).data)
            .offset((a as libc::c_ulong).wrapping_mul((*heap).el_sz) as isize)
            as *mut libc::c_char as *mut libc::c_void,
        &mut *((*heap).data)
            .offset((b as libc::c_ulong).wrapping_mul((*heap).el_sz) as isize)
            as *mut libc::c_char as *const libc::c_void,
        (*heap).el_sz,
    );
    memcpy(
        &mut *((*heap).data)
            .offset((b as libc::c_ulong).wrapping_mul((*heap).el_sz) as isize)
            as *mut libc::c_char as *mut libc::c_void,
        tmp as *const libc::c_void,
        (*heap).el_sz,
    );
    free(tmp as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn swap_pointer(
    mut heap: *mut zmaxheap_t,
    mut a: libc::c_int,
    mut b: libc::c_int,
) {
    let mut t: libc::c_float = *((*heap).values).offset(a as isize);
    *((*heap).values).offset(a as isize) = *((*heap).values).offset(b as isize);
    *((*heap).values).offset(b as isize) = t;
    let mut pp: *mut *mut libc::c_void = (*heap).data as *mut *mut libc::c_void;
    let mut tmp: *mut libc::c_void = *pp.offset(a as isize);
    let ref mut fresh0 = *pp.offset(a as isize);
    *fresh0 = *pp.offset(b as isize);
    let ref mut fresh1 = *pp.offset(b as isize);
    *fresh1 = tmp;
}
#[no_mangle]
pub unsafe extern "C" fn zmaxheap_create(mut el_sz: size_t) -> *mut zmaxheap_t {
    let mut heap: *mut zmaxheap_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<zmaxheap_t>() as libc::c_ulong,
    ) as *mut zmaxheap_t;
    (*heap).el_sz = el_sz;
    let ref mut fresh2 = (*heap).swap;
    *fresh2 = Some(
        swap_default
            as unsafe extern "C" fn(*mut zmaxheap_t, libc::c_int, libc::c_int) -> (),
    );
    if el_sz == ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong {
        let ref mut fresh3 = (*heap).swap;
        *fresh3 = Some(
            swap_pointer
                as unsafe extern "C" fn(*mut zmaxheap_t, libc::c_int, libc::c_int) -> (),
        );
    }
    return heap;
}
#[no_mangle]
pub unsafe extern "C" fn zmaxheap_destroy(mut heap: *mut zmaxheap_t) {
    free((*heap).values as *mut libc::c_void);
    free((*heap).data as *mut libc::c_void);
    memset(
        heap as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<zmaxheap_t>() as libc::c_ulong,
    );
    free(heap as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn zmaxheap_size(mut heap: *mut zmaxheap_t) -> libc::c_int {
    return (*heap).size;
}
#[no_mangle]
pub unsafe extern "C" fn zmaxheap_ensure_capacity(
    mut heap: *mut zmaxheap_t,
    mut capacity: libc::c_int,
) {
    if (*heap).alloc >= capacity {
        return;
    }
    let mut newcap: libc::c_int = (*heap).alloc;
    while newcap < capacity {
        if newcap < 16 as libc::c_int {
            newcap = 16 as libc::c_int;
        } else {
            newcap *= 2 as libc::c_int;
        }
    }
    let ref mut fresh4 = (*heap).values;
    *fresh4 = realloc(
        (*heap).values as *mut libc::c_void,
        (newcap as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut libc::c_float;
    let ref mut fresh5 = (*heap).data;
    *fresh5 = realloc(
        (*heap).data as *mut libc::c_void,
        (newcap as libc::c_ulong).wrapping_mul((*heap).el_sz),
    ) as *mut libc::c_char;
    (*heap).alloc = newcap;
}
#[no_mangle]
pub unsafe extern "C" fn zmaxheap_add(
    mut heap: *mut zmaxheap_t,
    mut p: *mut libc::c_void,
    mut v: libc::c_float,
) {
    zmaxheap_ensure_capacity(heap, (*heap).size + 1 as libc::c_int);
    let mut idx: libc::c_int = (*heap).size;
    *((*heap).values).offset(idx as isize) = v;
    memcpy(
        &mut *((*heap).data)
            .offset((idx as libc::c_ulong).wrapping_mul((*heap).el_sz) as isize)
            as *mut libc::c_char as *mut libc::c_void,
        p,
        (*heap).el_sz,
    );
    let ref mut fresh6 = (*heap).size;
    *fresh6 += 1;
    while idx > 0 as libc::c_int {
        let mut parent: libc::c_int = (idx - 1 as libc::c_int) / 2 as libc::c_int;
        if *((*heap).values).offset(parent as isize) >= v {
            break;
        }
        ((*heap).swap).expect("non-null function pointer")(heap, idx, parent);
        idx = parent;
    }
}
#[no_mangle]
pub unsafe extern "C" fn zmaxheap_vmap(
    mut heap: *mut zmaxheap_t,
    mut f: Option::<unsafe extern "C" fn() -> ()>,
) {
    let mut idx: libc::c_int = 0 as libc::c_int;
    while idx < (*heap).size {
        let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
        memcpy(
            &mut p as *mut *mut libc::c_void as *mut libc::c_void,
            &mut *((*heap).data)
                .offset((idx as libc::c_ulong).wrapping_mul((*heap).el_sz) as isize)
                as *mut libc::c_char as *const libc::c_void,
            (*heap).el_sz,
        );
        if p.is_null() {
            fflush(stderr);
        }
        ::std::mem::transmute::<_, fn(_)>(f.expect("non-null function pointer"))(p);
        idx += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn zmaxheap_remove_index(
    mut heap: *mut zmaxheap_t,
    mut idx: libc::c_int,
    mut p: *mut libc::c_void,
    mut v: *mut libc::c_float,
) -> libc::c_int {
    if idx >= (*heap).size {
        return 0 as libc::c_int;
    }
    if !v.is_null() {
        *v = *((*heap).values).offset(idx as isize);
    }
    if !p.is_null() {
        memcpy(
            p,
            &mut *((*heap).data)
                .offset((idx as libc::c_ulong).wrapping_mul((*heap).el_sz) as isize)
                as *mut libc::c_char as *const libc::c_void,
            (*heap).el_sz,
        );
    }
    let ref mut fresh7 = (*heap).size;
    *fresh7 -= 1;
    if idx == (*heap).size {
        return 1 as libc::c_int;
    }
    *((*heap).values)
        .offset(idx as isize) = *((*heap).values).offset((*heap).size as isize);
    memcpy(
        &mut *((*heap).data)
            .offset((idx as libc::c_ulong).wrapping_mul((*heap).el_sz) as isize)
            as *mut libc::c_char as *mut libc::c_void,
        &mut *((*heap).data)
            .offset(((*heap).el_sz).wrapping_mul((*heap).size as libc::c_ulong) as isize)
            as *mut libc::c_char as *const libc::c_void,
        (*heap).el_sz,
    );
    let mut parent: libc::c_int = idx;
    let mut parent_score: libc::c_float = *((*heap).values).offset(idx as isize);
    while parent < (*heap).size {
        let mut left: libc::c_int = 2 as libc::c_int * parent + 1 as libc::c_int;
        let mut right: libc::c_int = left + 1 as libc::c_int;
        let mut left_score: libc::c_float = if left < (*heap).size {
            *((*heap).values).offset(left as isize)
        } else {
            -::std::f32::INFINITY
        };
        let mut right_score: libc::c_float = if right < (*heap).size {
            *((*heap).values).offset(right as isize)
        } else {
            -::std::f32::INFINITY
        };
        if parent_score >= left_score && parent_score >= right_score {
            break;
        }
        if left_score >= right_score {
            ((*heap).swap).expect("non-null function pointer")(heap, parent, left);
            parent = left;
        } else {
            ((*heap).swap).expect("non-null function pointer")(heap, parent, right);
            parent = right;
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zmaxheap_remove_max(
    mut heap: *mut zmaxheap_t,
    mut p: *mut libc::c_void,
    mut v: *mut libc::c_float,
) -> libc::c_int {
    return zmaxheap_remove_index(heap, 0 as libc::c_int, p, v);
}
#[no_mangle]
pub unsafe extern "C" fn zmaxheap_iterator_init(
    mut heap: *mut zmaxheap_t,
    mut it: *mut zmaxheap_iterator_t,
) {
    memset(
        it as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<zmaxheap_iterator_t>() as libc::c_ulong,
    );
    let ref mut fresh8 = (*it).heap;
    *fresh8 = heap;
    (*it).in_0 = 0 as libc::c_int;
    (*it).out = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zmaxheap_iterator_next(
    mut it: *mut zmaxheap_iterator_t,
    mut p: *mut libc::c_void,
    mut v: *mut libc::c_float,
) -> libc::c_int {
    let mut heap: *mut zmaxheap_t = (*it).heap;
    if (*it).in_0 >= zmaxheap_size(heap) {
        return 0 as libc::c_int;
    }
    *v = *((*heap).values).offset((*it).in_0 as isize);
    memcpy(
        p,
        &mut *((*heap).data)
            .offset(((*it).in_0 as libc::c_ulong).wrapping_mul((*heap).el_sz) as isize)
            as *mut libc::c_char as *const libc::c_void,
        (*heap).el_sz,
    );
    if (*it).in_0 != (*it).out {
        *((*heap).values)
            .offset((*it).out as isize) = *((*heap).values).offset((*it).in_0 as isize);
        memcpy(
            &mut *((*heap).data)
                .offset(
                    ((*it).out as libc::c_ulong).wrapping_mul((*heap).el_sz) as isize,
                ) as *mut libc::c_char as *mut libc::c_void,
            &mut *((*heap).data)
                .offset(
                    ((*it).in_0 as libc::c_ulong).wrapping_mul((*heap).el_sz) as isize,
                ) as *mut libc::c_char as *const libc::c_void,
            (*heap).el_sz,
        );
    }
    let ref mut fresh9 = (*it).in_0;
    *fresh9 += 1;
    let ref mut fresh10 = (*it).out;
    *fresh10 += 1;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zmaxheap_iterator_next_volatile(
    mut it: *mut zmaxheap_iterator_t,
    mut p: *mut libc::c_void,
    mut v: *mut libc::c_float,
) -> libc::c_int {
    let mut heap: *mut zmaxheap_t = (*it).heap;
    if (*it).in_0 >= zmaxheap_size(heap) {
        return 0 as libc::c_int;
    }
    *v = *((*heap).values).offset((*it).in_0 as isize);
    let ref mut fresh11 = *(p as *mut *mut libc::c_void);
    *fresh11 = &mut *((*heap).data)
        .offset(((*it).in_0 as libc::c_ulong).wrapping_mul((*heap).el_sz) as isize)
        as *mut libc::c_char as *mut libc::c_void;
    if (*it).in_0 != (*it).out {
        *((*heap).values)
            .offset((*it).out as isize) = *((*heap).values).offset((*it).in_0 as isize);
        memcpy(
            &mut *((*heap).data)
                .offset(
                    ((*it).out as libc::c_ulong).wrapping_mul((*heap).el_sz) as isize,
                ) as *mut libc::c_char as *mut libc::c_void,
            &mut *((*heap).data)
                .offset(
                    ((*it).in_0 as libc::c_ulong).wrapping_mul((*heap).el_sz) as isize,
                ) as *mut libc::c_char as *const libc::c_void,
            (*heap).el_sz,
        );
    }
    let ref mut fresh12 = (*it).in_0;
    *fresh12 += 1;
    let ref mut fresh13 = (*it).out;
    *fresh13 += 1;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zmaxheap_iterator_remove(mut it: *mut zmaxheap_iterator_t) {
    let ref mut fresh14 = (*it).out;
    *fresh14 -= 1;
}
unsafe extern "C" fn maxheapify(mut heap: *mut zmaxheap_t, mut parent: libc::c_int) {
    let mut left: libc::c_int = 2 as libc::c_int * parent + 1 as libc::c_int;
    let mut right: libc::c_int = 2 as libc::c_int * parent + 2 as libc::c_int;
    let mut betterchild: libc::c_int = parent;
    if left < (*heap).size
        && *((*heap).values).offset(left as isize)
            > *((*heap).values).offset(betterchild as isize)
    {
        betterchild = left;
    }
    if right < (*heap).size
        && *((*heap).values).offset(right as isize)
            > *((*heap).values).offset(betterchild as isize)
    {
        betterchild = right;
    }
    if betterchild != parent {
        ((*heap).swap).expect("non-null function pointer")(heap, parent, betterchild);
        return maxheapify(heap, betterchild);
    }
}
#[no_mangle]
pub unsafe extern "C" fn zmaxheap_iterator_finish(mut it: *mut zmaxheap_iterator_t) {
    if (*it).in_0 == (*it).out {
        return;
    }
    let mut heap: *mut zmaxheap_t = (*it).heap;
    (*heap).size = (*it).out;
    let mut i: libc::c_int = (*heap).size / 2 as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        maxheapify(heap, i);
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn zmaxheap_test() {
    let mut cap: libc::c_int = 10000 as libc::c_int;
    let mut sz: libc::c_int = 0 as libc::c_int;
    let mut vals: *mut int32_t = calloc(
        ::std::mem::size_of::<int32_t>() as libc::c_ulong,
        cap as libc::c_ulong,
    ) as *mut int32_t;
    let mut heap: *mut zmaxheap_t = zmaxheap_create(
        ::std::mem::size_of::<int32_t>() as libc::c_ulong,
    );
    let mut maxsz: libc::c_int = 0 as libc::c_int;
    let mut zcnt: libc::c_int = 0 as libc::c_int;
    let mut iter: libc::c_int = 0 as libc::c_int;
    while iter < 5000000 as libc::c_int {
        if random() & 1 as libc::c_int as libc::c_long
            == 0 as libc::c_int as libc::c_long && sz < cap
        {
            let mut v: int32_t = (random() / 1000 as libc::c_int as libc::c_long)
                as int32_t;
            let mut fv: libc::c_float = v as libc::c_float;
            *vals.offset(sz as isize) = v;
            zmaxheap_add(heap, &mut v as *mut int32_t as *mut libc::c_void, fv);
            sz += 1;
        } else {
            let mut maxv: libc::c_int = -(1 as libc::c_int);
            let mut maxi: libc::c_int = -(1 as libc::c_int);
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < sz {
                if *vals.offset(i as isize) > maxv {
                    maxv = *vals.offset(i as isize);
                    maxi = i;
                }
                i += 1;
            }
            let mut outv: int32_t = 0;
            let mut outfv: libc::c_float = 0.;
            let mut res: libc::c_int = zmaxheap_remove_max(
                heap,
                &mut outv as *mut int32_t as *mut libc::c_void,
                &mut outfv,
            );
            if !(sz == 0 as libc::c_int) {
                *vals
                    .offset(
                        maxi as isize,
                    ) = *vals.offset((sz - 1 as libc::c_int) as isize);
                sz -= 1;
            }
        }
        if sz > maxsz {
            maxsz = sz;
        }
        if maxsz > 0 as libc::c_int && sz == 0 as libc::c_int {
            zcnt += 1;
        }
        iter += 1;
    }
    printf(
        b"max size: %d, zcount %d\n\0" as *const u8 as *const libc::c_char,
        maxsz,
        zcnt,
    );
    free(vals as *mut libc::c_void);
}
