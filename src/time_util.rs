use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type time_t = __time_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type suseconds_t = __suseconds_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeutil_rest {
    pub acc_time: int64_t,
    pub start_time: int64_t,
}
pub type timeutil_rest_t = timeutil_rest;
#[no_mangle]
pub unsafe extern "C" fn timeutil_rest_create() -> *mut timeutil_rest_t {
    let mut rest: *mut timeutil_rest_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<timeutil_rest_t>() as libc::c_ulong,
    ) as *mut timeutil_rest_t;
    return rest;
}
#[no_mangle]
pub unsafe extern "C" fn timeutil_rest_destroy(mut rest: *mut timeutil_rest_t) {
    free(rest as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn utime_now() -> int64_t {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    return tv.tv_sec * 1000000 as libc::c_int as libc::c_long + tv.tv_usec;
}
#[no_mangle]
pub unsafe extern "C" fn utime_get_seconds(mut v: int64_t) -> int64_t {
    return v / 1000000 as libc::c_int as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn utime_get_useconds(mut v: int64_t) -> int64_t {
    return v % 1000000 as libc::c_int as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn utime_to_timeval(mut v: int64_t, mut tv: *mut timeval) {
    (*tv).tv_sec = utime_get_seconds(v);
    (*tv).tv_usec = utime_get_useconds(v);
}
#[no_mangle]
pub unsafe extern "C" fn utime_to_timespec(mut v: int64_t, mut ts: *mut timespec) {
    (*ts).tv_sec = utime_get_seconds(v);
    (*ts).tv_nsec = utime_get_useconds(v) * 1000 as libc::c_int as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn timeutil_usleep(mut useconds: int64_t) -> int32_t {
    return usleep(useconds as __useconds_t);
}
#[no_mangle]
pub unsafe extern "C" fn timeutil_sleep(mut seconds: libc::c_uint) -> uint32_t {
    return sleep(seconds);
}
#[no_mangle]
pub unsafe extern "C" fn timeutil_sleep_hz(
    mut rest: *mut timeutil_rest_t,
    mut hz: libc::c_double,
) -> int32_t {
    let mut max_delay: int64_t = (1000000 as libc::c_long as libc::c_double / hz)
        as int64_t;
    let mut curr_time: int64_t = utime_now();
    let mut diff: int64_t = curr_time - (*rest).start_time;
    let mut delay: int64_t = max_delay - diff;
    if delay < 0 as libc::c_int as libc::c_long {
        delay = 0 as libc::c_int as int64_t;
    }
    let mut ret: int32_t = timeutil_usleep(delay);
    (*rest).start_time = utime_now();
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn timeutil_timer_reset(mut rest: *mut timeutil_rest_t) {
    (*rest).start_time = utime_now();
    (*rest).acc_time = 0 as libc::c_int as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn timeutil_timer_start(mut rest: *mut timeutil_rest_t) {
    (*rest).start_time = utime_now();
}
#[no_mangle]
pub unsafe extern "C" fn timeutil_timer_stop(mut rest: *mut timeutil_rest_t) {
    let mut curr_time: int64_t = utime_now();
    let mut diff: int64_t = curr_time - (*rest).start_time;
    let ref mut fresh0 = (*rest).acc_time;
    *fresh0 += diff;
}
#[no_mangle]
pub unsafe extern "C" fn timeutil_timer_timeout(
    mut rest: *mut timeutil_rest_t,
    mut timeout_s: libc::c_double,
) -> bool {
    let mut timeout_us: int64_t = (1000000 as libc::c_long as libc::c_double * timeout_s)
        as int64_t;
    return (*rest).acc_time > timeout_us;
}
#[no_mangle]
pub unsafe extern "C" fn time_util_hhmmss_ss_to_utime(
    mut time: libc::c_double,
) -> int64_t {
    let mut utime: int64_t = 0 as libc::c_int as int64_t;
    let mut itime: libc::c_int = time as libc::c_int;
    let mut seconds: libc::c_double = fmod(time, 100.0f64);
    let mut minutes: uint8_t = (itime % 10000 as libc::c_int / 100 as libc::c_int)
        as uint8_t;
    let mut hours: uint8_t = (itime / 10000 as libc::c_int) as uint8_t;
    utime = (utime as libc::c_double + seconds * 100 as libc::c_int as libc::c_double)
        as int64_t;
    utime += (minutes as libc::c_int * 6000 as libc::c_int) as libc::c_long;
    utime += (hours as libc::c_int * 360000 as libc::c_int) as libc::c_long;
    utime *= 10000 as libc::c_int as libc::c_long;
    return utime;
}
#[no_mangle]
pub unsafe extern "C" fn timeutil_ms_to_us(mut ms: int32_t) -> int64_t {
    return ms as int64_t * 1000 as libc::c_int as libc::c_long;
}
