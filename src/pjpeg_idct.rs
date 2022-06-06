use ::libc;
extern "C" {
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[inline]
unsafe extern "C" fn idct_1D_u32(
    mut in_0: *mut int32_t,
    mut instride: libc::c_int,
    mut out: *mut int32_t,
    mut outstride: libc::c_int,
) {
    let mut x: libc::c_int = 0 as libc::c_int;
    while x < 8 as libc::c_int {
        *out.offset((x * outstride) as isize) = 0 as libc::c_int;
        x += 1;
    }
    let mut c: int32_t = 0;
    c = *in_0.offset((0 as libc::c_int * instride) as isize);
    if c != 0 {
        let mut c181: int32_t = c * 181 as libc::c_int;
        let ref mut fresh0 = *out.offset((0 as libc::c_int * outstride) as isize);
        *fresh0 += c181;
        let ref mut fresh1 = *out.offset((1 as libc::c_int * outstride) as isize);
        *fresh1 += c181;
        let ref mut fresh2 = *out.offset((2 as libc::c_int * outstride) as isize);
        *fresh2 += c181;
        let ref mut fresh3 = *out.offset((3 as libc::c_int * outstride) as isize);
        *fresh3 += c181;
        let ref mut fresh4 = *out.offset((4 as libc::c_int * outstride) as isize);
        *fresh4 += c181;
        let ref mut fresh5 = *out.offset((5 as libc::c_int * outstride) as isize);
        *fresh5 += c181;
        let ref mut fresh6 = *out.offset((6 as libc::c_int * outstride) as isize);
        *fresh6 += c181;
        let ref mut fresh7 = *out.offset((7 as libc::c_int * outstride) as isize);
        *fresh7 += c181;
    }
    c = *in_0.offset((1 as libc::c_int * instride) as isize);
    if c != 0 {
        let mut c251: int32_t = c * 251 as libc::c_int;
        let mut c212: int32_t = c * 212 as libc::c_int;
        let mut c142: int32_t = c * 142 as libc::c_int;
        let mut c49: int32_t = c * 49 as libc::c_int;
        let ref mut fresh8 = *out.offset((0 as libc::c_int * outstride) as isize);
        *fresh8 += c251;
        let ref mut fresh9 = *out.offset((1 as libc::c_int * outstride) as isize);
        *fresh9 += c212;
        let ref mut fresh10 = *out.offset((2 as libc::c_int * outstride) as isize);
        *fresh10 += c142;
        let ref mut fresh11 = *out.offset((3 as libc::c_int * outstride) as isize);
        *fresh11 += c49;
        let ref mut fresh12 = *out.offset((4 as libc::c_int * outstride) as isize);
        *fresh12 -= c49;
        let ref mut fresh13 = *out.offset((5 as libc::c_int * outstride) as isize);
        *fresh13 -= c142;
        let ref mut fresh14 = *out.offset((6 as libc::c_int * outstride) as isize);
        *fresh14 -= c212;
        let ref mut fresh15 = *out.offset((7 as libc::c_int * outstride) as isize);
        *fresh15 -= c251;
    }
    c = *in_0.offset((2 as libc::c_int * instride) as isize);
    if c != 0 {
        let mut c236: int32_t = c * 236 as libc::c_int;
        let mut c97: int32_t = c * 97 as libc::c_int;
        let ref mut fresh16 = *out.offset((0 as libc::c_int * outstride) as isize);
        *fresh16 += c236;
        let ref mut fresh17 = *out.offset((1 as libc::c_int * outstride) as isize);
        *fresh17 += c97;
        let ref mut fresh18 = *out.offset((2 as libc::c_int * outstride) as isize);
        *fresh18 -= c97;
        let ref mut fresh19 = *out.offset((3 as libc::c_int * outstride) as isize);
        *fresh19 -= c236;
        let ref mut fresh20 = *out.offset((4 as libc::c_int * outstride) as isize);
        *fresh20 -= c236;
        let ref mut fresh21 = *out.offset((5 as libc::c_int * outstride) as isize);
        *fresh21 -= c97;
        let ref mut fresh22 = *out.offset((6 as libc::c_int * outstride) as isize);
        *fresh22 += c97;
        let ref mut fresh23 = *out.offset((7 as libc::c_int * outstride) as isize);
        *fresh23 += c236;
    }
    c = *in_0.offset((3 as libc::c_int * instride) as isize);
    if c != 0 {
        let mut c212_0: int32_t = c * 212 as libc::c_int;
        let mut c49_0: int32_t = c * 49 as libc::c_int;
        let mut c251_0: int32_t = c * 251 as libc::c_int;
        let mut c142_0: int32_t = c * 142 as libc::c_int;
        let ref mut fresh24 = *out.offset((0 as libc::c_int * outstride) as isize);
        *fresh24 += c212_0;
        let ref mut fresh25 = *out.offset((1 as libc::c_int * outstride) as isize);
        *fresh25 -= c49_0;
        let ref mut fresh26 = *out.offset((2 as libc::c_int * outstride) as isize);
        *fresh26 -= c251_0;
        let ref mut fresh27 = *out.offset((3 as libc::c_int * outstride) as isize);
        *fresh27 -= c142_0;
        let ref mut fresh28 = *out.offset((4 as libc::c_int * outstride) as isize);
        *fresh28 += c142_0;
        let ref mut fresh29 = *out.offset((5 as libc::c_int * outstride) as isize);
        *fresh29 += c251_0;
        let ref mut fresh30 = *out.offset((6 as libc::c_int * outstride) as isize);
        *fresh30 += c49_0;
        let ref mut fresh31 = *out.offset((7 as libc::c_int * outstride) as isize);
        *fresh31 -= c212_0;
    }
    c = *in_0.offset((4 as libc::c_int * instride) as isize);
    if c != 0 {
        let mut c181_0: int32_t = c * 181 as libc::c_int;
        let ref mut fresh32 = *out.offset((0 as libc::c_int * outstride) as isize);
        *fresh32 += c181_0;
        let ref mut fresh33 = *out.offset((1 as libc::c_int * outstride) as isize);
        *fresh33 -= c181_0;
        let ref mut fresh34 = *out.offset((2 as libc::c_int * outstride) as isize);
        *fresh34 -= c181_0;
        let ref mut fresh35 = *out.offset((3 as libc::c_int * outstride) as isize);
        *fresh35 += c181_0;
        let ref mut fresh36 = *out.offset((4 as libc::c_int * outstride) as isize);
        *fresh36 += c181_0;
        let ref mut fresh37 = *out.offset((5 as libc::c_int * outstride) as isize);
        *fresh37 -= c181_0;
        let ref mut fresh38 = *out.offset((6 as libc::c_int * outstride) as isize);
        *fresh38 -= c181_0;
        let ref mut fresh39 = *out.offset((7 as libc::c_int * outstride) as isize);
        *fresh39 += c181_0;
    }
    c = *in_0.offset((5 as libc::c_int * instride) as isize);
    if c != 0 {
        let mut c142_1: int32_t = c * 142 as libc::c_int;
        let mut c251_1: int32_t = c * 251 as libc::c_int;
        let mut c49_1: int32_t = c * 49 as libc::c_int;
        let mut c212_1: int32_t = c * 212 as libc::c_int;
        let ref mut fresh40 = *out.offset((0 as libc::c_int * outstride) as isize);
        *fresh40 += c142_1;
        let ref mut fresh41 = *out.offset((1 as libc::c_int * outstride) as isize);
        *fresh41 -= c251_1;
        let ref mut fresh42 = *out.offset((2 as libc::c_int * outstride) as isize);
        *fresh42 += c49_1;
        let ref mut fresh43 = *out.offset((3 as libc::c_int * outstride) as isize);
        *fresh43 += c212_1;
        let ref mut fresh44 = *out.offset((4 as libc::c_int * outstride) as isize);
        *fresh44 -= c212_1;
        let ref mut fresh45 = *out.offset((5 as libc::c_int * outstride) as isize);
        *fresh45 -= c49_1;
        let ref mut fresh46 = *out.offset((6 as libc::c_int * outstride) as isize);
        *fresh46 += c251_1;
        let ref mut fresh47 = *out.offset((7 as libc::c_int * outstride) as isize);
        *fresh47 -= c142_1;
    }
    c = *in_0.offset((6 as libc::c_int * instride) as isize);
    if c != 0 {
        let mut c97_0: int32_t = c * 97 as libc::c_int;
        let mut c236_0: int32_t = c * 236 as libc::c_int;
        let ref mut fresh48 = *out.offset((0 as libc::c_int * outstride) as isize);
        *fresh48 += c97_0;
        let ref mut fresh49 = *out.offset((1 as libc::c_int * outstride) as isize);
        *fresh49 -= c236_0;
        let ref mut fresh50 = *out.offset((2 as libc::c_int * outstride) as isize);
        *fresh50 += c236_0;
        let ref mut fresh51 = *out.offset((3 as libc::c_int * outstride) as isize);
        *fresh51 -= c97_0;
        let ref mut fresh52 = *out.offset((4 as libc::c_int * outstride) as isize);
        *fresh52 -= c97_0;
        let ref mut fresh53 = *out.offset((5 as libc::c_int * outstride) as isize);
        *fresh53 += c236_0;
        let ref mut fresh54 = *out.offset((6 as libc::c_int * outstride) as isize);
        *fresh54 -= c236_0;
        let ref mut fresh55 = *out.offset((7 as libc::c_int * outstride) as isize);
        *fresh55 += c97_0;
    }
    c = *in_0.offset((7 as libc::c_int * instride) as isize);
    if c != 0 {
        let mut c49_2: int32_t = c * 49 as libc::c_int;
        let mut c142_2: int32_t = c * 142 as libc::c_int;
        let mut c212_2: int32_t = c * 212 as libc::c_int;
        let mut c251_2: int32_t = c * 251 as libc::c_int;
        let ref mut fresh56 = *out.offset((0 as libc::c_int * outstride) as isize);
        *fresh56 += c49_2;
        let ref mut fresh57 = *out.offset((1 as libc::c_int * outstride) as isize);
        *fresh57 -= c142_2;
        let ref mut fresh58 = *out.offset((2 as libc::c_int * outstride) as isize);
        *fresh58 += c212_2;
        let ref mut fresh59 = *out.offset((3 as libc::c_int * outstride) as isize);
        *fresh59 -= c251_2;
        let ref mut fresh60 = *out.offset((4 as libc::c_int * outstride) as isize);
        *fresh60 += c251_2;
        let ref mut fresh61 = *out.offset((5 as libc::c_int * outstride) as isize);
        *fresh61 -= c212_2;
        let ref mut fresh62 = *out.offset((6 as libc::c_int * outstride) as isize);
        *fresh62 += c142_2;
        let ref mut fresh63 = *out.offset((7 as libc::c_int * outstride) as isize);
        *fresh63 -= c49_2;
    }
}
#[no_mangle]
pub unsafe extern "C" fn pjpeg_idct_2D_u32(
    mut in_0: *mut int32_t,
    mut out: *mut uint8_t,
    mut outstride: uint32_t,
) {
    let mut tmp: [int32_t; 64] = [0; 64];
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 8 as libc::c_int {
        idct_1D_u32(
            &mut *in_0.offset((8 as libc::c_int * y) as isize),
            1 as libc::c_int,
            &mut *tmp.as_mut_ptr().offset((8 as libc::c_int * y) as isize),
            1 as libc::c_int,
        );
        y += 1;
    }
    let mut tmp2: [int32_t; 64] = [0; 64];
    let mut x: libc::c_int = 0 as libc::c_int;
    while x < 8 as libc::c_int {
        idct_1D_u32(
            &mut *tmp.as_mut_ptr().offset(x as isize),
            8 as libc::c_int,
            &mut *tmp2.as_mut_ptr().offset(x as isize),
            8 as libc::c_int,
        );
        x += 1;
    }
    let mut y_0: libc::c_int = 0 as libc::c_int;
    while y_0 < 8 as libc::c_int {
        let mut x_0: libc::c_int = 0 as libc::c_int;
        while x_0 < 8 as libc::c_int {
            let mut i: libc::c_int = 8 as libc::c_int * y_0 + x_0;
            let offset: int32_t = ((128 as libc::c_int) << 18 as libc::c_int)
                + ((1 as libc::c_int) << 17 as libc::c_int);
            let mut v: int32_t = tmp2[i as usize] + offset >> 18 as libc::c_int;
            if v < 0 as libc::c_int {
                v = 0 as libc::c_int;
            }
            if v > 255 as libc::c_int {
                v = 255 as libc::c_int;
            }
            *out
                .offset(
                    (y_0 as libc::c_uint)
                        .wrapping_mul(outstride)
                        .wrapping_add(x_0 as libc::c_uint) as isize,
                ) = v as uint8_t;
            x_0 += 1;
        }
        y_0 += 1;
    }
}
#[inline]
unsafe extern "C" fn idct_1D_double(
    mut in_0: *mut libc::c_double,
    mut instride: libc::c_int,
    mut out: *mut libc::c_double,
    mut outstride: libc::c_int,
) {
    let mut x: libc::c_int = 0 as libc::c_int;
    while x < 8 as libc::c_int {
        *out.offset((x * outstride) as isize) = 0 as libc::c_int as libc::c_double;
        x += 1;
    }
    let mut Cu: libc::c_double = 1 as libc::c_int as libc::c_double
        / sqrt(2 as libc::c_int as libc::c_double);
    let mut u: libc::c_int = 0 as libc::c_int;
    while u < 8 as libc::c_int {
        let mut coeff: libc::c_double = *in_0.offset((u * instride) as isize);
        if !(coeff == 0 as libc::c_int as libc::c_double) {
            let mut x_0: libc::c_int = 0 as libc::c_int;
            while x_0 < 8 as libc::c_int {
                *out.offset((x_0 * outstride) as isize)
                    += Cu
                        * cos(
                            ((2 as libc::c_int * x_0 + 1 as libc::c_int) * u)
                                as libc::c_double * 3.14159265358979323846f64
                                / 16 as libc::c_int as libc::c_double,
                        ) * coeff;
                x_0 += 1;
            }
        }
        u += 1;
        Cu = 1 as libc::c_int as libc::c_double;
    }
}
#[no_mangle]
pub unsafe extern "C" fn pjpeg_idct_2D_double(
    mut in_0: *mut int32_t,
    mut out: *mut uint8_t,
    mut outstride: uint32_t,
) {
    let mut din: [libc::c_double; 64] = [0.; 64];
    let mut dout: [libc::c_double; 64] = [0.; 64];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        din[i as usize] = *in_0.offset(i as isize) as libc::c_double;
        i += 1;
    }
    let mut tmp: [libc::c_double; 64] = [0.; 64];
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 8 as libc::c_int {
        idct_1D_double(
            &mut *din.as_mut_ptr().offset((8 as libc::c_int * y) as isize),
            1 as libc::c_int,
            &mut *tmp.as_mut_ptr().offset((8 as libc::c_int * y) as isize),
            1 as libc::c_int,
        );
        y += 1;
    }
    let mut x: libc::c_int = 0 as libc::c_int;
    while x < 8 as libc::c_int {
        idct_1D_double(
            &mut *tmp.as_mut_ptr().offset(x as isize),
            8 as libc::c_int,
            &mut *dout.as_mut_ptr().offset(x as isize),
            8 as libc::c_int,
        );
        x += 1;
    }
    let mut y_0: libc::c_int = 0 as libc::c_int;
    while y_0 < 8 as libc::c_int {
        let mut x_0: libc::c_int = 0 as libc::c_int;
        while x_0 < 8 as libc::c_int {
            let mut i_0: libc::c_int = 8 as libc::c_int * y_0 + x_0;
            dout[i_0
                as usize] = dout[i_0 as usize] / 4 as libc::c_int as libc::c_double
                + 128 as libc::c_int as libc::c_double;
            if dout[i_0 as usize] < 0 as libc::c_int as libc::c_double {
                dout[i_0 as usize] = 0 as libc::c_int as libc::c_double;
            }
            if dout[i_0 as usize] > 255 as libc::c_int as libc::c_double {
                dout[i_0 as usize] = 255 as libc::c_int as libc::c_double;
            }
            *out
                .offset(
                    (y_0 as libc::c_uint)
                        .wrapping_mul(outstride)
                        .wrapping_add(x_0 as libc::c_uint) as isize,
                ) = dout[i_0 as usize] as uint8_t;
            x_0 += 1;
        }
        y_0 += 1;
    }
}
#[inline]
unsafe extern "C" fn njClip(x: libc::c_int) -> libc::c_uchar {
    return (if x < 0 as libc::c_int {
        0 as libc::c_int
    } else if x > 0xff as libc::c_int {
        0xff as libc::c_int
    } else {
        x as libc::c_uchar as libc::c_int
    }) as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn njRowIDCT(mut blk: *mut libc::c_int) {
    let mut x0: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut x3: libc::c_int = 0;
    let mut x4: libc::c_int = 0;
    let mut x5: libc::c_int = 0;
    let mut x6: libc::c_int = 0;
    let mut x7: libc::c_int = 0;
    let mut x8: libc::c_int = 0;
    x1 = *blk.offset(4 as libc::c_int as isize) << 11 as libc::c_int;
    x2 = *blk.offset(6 as libc::c_int as isize);
    x3 = *blk.offset(2 as libc::c_int as isize);
    x4 = *blk.offset(1 as libc::c_int as isize);
    x5 = *blk.offset(7 as libc::c_int as isize);
    x6 = *blk.offset(5 as libc::c_int as isize);
    x7 = *blk.offset(3 as libc::c_int as isize);
    if x1 | x2 | x3 | x4 | x5 | x6 | x7 == 0 {
        let ref mut fresh64 = *blk.offset(7 as libc::c_int as isize);
        *fresh64 = *blk.offset(0 as libc::c_int as isize) << 3 as libc::c_int;
        let ref mut fresh65 = *blk.offset(6 as libc::c_int as isize);
        *fresh65 = *fresh64;
        let ref mut fresh66 = *blk.offset(5 as libc::c_int as isize);
        *fresh66 = *fresh65;
        let ref mut fresh67 = *blk.offset(4 as libc::c_int as isize);
        *fresh67 = *fresh66;
        let ref mut fresh68 = *blk.offset(3 as libc::c_int as isize);
        *fresh68 = *fresh67;
        let ref mut fresh69 = *blk.offset(2 as libc::c_int as isize);
        *fresh69 = *fresh68;
        let ref mut fresh70 = *blk.offset(1 as libc::c_int as isize);
        *fresh70 = *fresh69;
        *blk.offset(0 as libc::c_int as isize) = *fresh70;
        return;
    }
    x0 = (*blk.offset(0 as libc::c_int as isize) << 11 as libc::c_int)
        + 128 as libc::c_int;
    x8 = 565 as libc::c_int * (x4 + x5);
    x4 = x8 + (2841 as libc::c_int - 565 as libc::c_int) * x4;
    x5 = x8 - (2841 as libc::c_int + 565 as libc::c_int) * x5;
    x8 = 2408 as libc::c_int * (x6 + x7);
    x6 = x8 - (2408 as libc::c_int - 1609 as libc::c_int) * x6;
    x7 = x8 - (2408 as libc::c_int + 1609 as libc::c_int) * x7;
    x8 = x0 + x1;
    x0 -= x1;
    x1 = 1108 as libc::c_int * (x3 + x2);
    x2 = x1 - (2676 as libc::c_int + 1108 as libc::c_int) * x2;
    x3 = x1 + (2676 as libc::c_int - 1108 as libc::c_int) * x3;
    x1 = x4 + x6;
    x4 -= x6;
    x6 = x5 + x7;
    x5 -= x7;
    x7 = x8 + x3;
    x8 -= x3;
    x3 = x0 + x2;
    x0 -= x2;
    x2 = 181 as libc::c_int * (x4 + x5) + 128 as libc::c_int >> 8 as libc::c_int;
    x4 = 181 as libc::c_int * (x4 - x5) + 128 as libc::c_int >> 8 as libc::c_int;
    *blk.offset(0 as libc::c_int as isize) = x7 + x1 >> 8 as libc::c_int;
    *blk.offset(1 as libc::c_int as isize) = x3 + x2 >> 8 as libc::c_int;
    *blk.offset(2 as libc::c_int as isize) = x0 + x4 >> 8 as libc::c_int;
    *blk.offset(3 as libc::c_int as isize) = x8 + x6 >> 8 as libc::c_int;
    *blk.offset(4 as libc::c_int as isize) = x8 - x6 >> 8 as libc::c_int;
    *blk.offset(5 as libc::c_int as isize) = x0 - x4 >> 8 as libc::c_int;
    *blk.offset(6 as libc::c_int as isize) = x3 - x2 >> 8 as libc::c_int;
    *blk.offset(7 as libc::c_int as isize) = x7 - x1 >> 8 as libc::c_int;
}
#[inline]
unsafe extern "C" fn njColIDCT(
    mut blk: *const libc::c_int,
    mut out: *mut libc::c_uchar,
    mut stride: libc::c_int,
) {
    let mut x0: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut x3: libc::c_int = 0;
    let mut x4: libc::c_int = 0;
    let mut x5: libc::c_int = 0;
    let mut x6: libc::c_int = 0;
    let mut x7: libc::c_int = 0;
    let mut x8: libc::c_int = 0;
    x1 = *blk.offset((8 as libc::c_int * 4 as libc::c_int) as isize) << 8 as libc::c_int;
    x2 = *blk.offset((8 as libc::c_int * 6 as libc::c_int) as isize);
    x3 = *blk.offset((8 as libc::c_int * 2 as libc::c_int) as isize);
    x4 = *blk.offset((8 as libc::c_int * 1 as libc::c_int) as isize);
    x5 = *blk.offset((8 as libc::c_int * 7 as libc::c_int) as isize);
    x6 = *blk.offset((8 as libc::c_int * 5 as libc::c_int) as isize);
    x7 = *blk.offset((8 as libc::c_int * 3 as libc::c_int) as isize);
    if x1 | x2 | x3 | x4 | x5 | x6 | x7 == 0 {
        x1 = njClip(
            (*blk.offset(0 as libc::c_int as isize) + 32 as libc::c_int
                >> 6 as libc::c_int) + 128 as libc::c_int,
        ) as libc::c_int;
        x0 = 8 as libc::c_int;
        while x0 != 0 {
            *out = x1 as libc::c_uchar;
            out = out.offset(stride as isize);
            x0 -= 1;
        }
        return;
    }
    x0 = (*blk.offset(0 as libc::c_int as isize) << 8 as libc::c_int)
        + 8192 as libc::c_int;
    x8 = 565 as libc::c_int * (x4 + x5) + 4 as libc::c_int;
    x4 = x8 + (2841 as libc::c_int - 565 as libc::c_int) * x4 >> 3 as libc::c_int;
    x5 = x8 - (2841 as libc::c_int + 565 as libc::c_int) * x5 >> 3 as libc::c_int;
    x8 = 2408 as libc::c_int * (x6 + x7) + 4 as libc::c_int;
    x6 = x8 - (2408 as libc::c_int - 1609 as libc::c_int) * x6 >> 3 as libc::c_int;
    x7 = x8 - (2408 as libc::c_int + 1609 as libc::c_int) * x7 >> 3 as libc::c_int;
    x8 = x0 + x1;
    x0 -= x1;
    x1 = 1108 as libc::c_int * (x3 + x2) + 4 as libc::c_int;
    x2 = x1 - (2676 as libc::c_int + 1108 as libc::c_int) * x2 >> 3 as libc::c_int;
    x3 = x1 + (2676 as libc::c_int - 1108 as libc::c_int) * x3 >> 3 as libc::c_int;
    x1 = x4 + x6;
    x4 -= x6;
    x6 = x5 + x7;
    x5 -= x7;
    x7 = x8 + x3;
    x8 -= x3;
    x3 = x0 + x2;
    x0 -= x2;
    x2 = 181 as libc::c_int * (x4 + x5) + 128 as libc::c_int >> 8 as libc::c_int;
    x4 = 181 as libc::c_int * (x4 - x5) + 128 as libc::c_int >> 8 as libc::c_int;
    *out = njClip((x7 + x1 >> 14 as libc::c_int) + 128 as libc::c_int);
    out = out.offset(stride as isize);
    *out = njClip((x3 + x2 >> 14 as libc::c_int) + 128 as libc::c_int);
    out = out.offset(stride as isize);
    *out = njClip((x0 + x4 >> 14 as libc::c_int) + 128 as libc::c_int);
    out = out.offset(stride as isize);
    *out = njClip((x8 + x6 >> 14 as libc::c_int) + 128 as libc::c_int);
    out = out.offset(stride as isize);
    *out = njClip((x8 - x6 >> 14 as libc::c_int) + 128 as libc::c_int);
    out = out.offset(stride as isize);
    *out = njClip((x0 - x4 >> 14 as libc::c_int) + 128 as libc::c_int);
    out = out.offset(stride as isize);
    *out = njClip((x3 - x2 >> 14 as libc::c_int) + 128 as libc::c_int);
    out = out.offset(stride as isize);
    *out = njClip((x7 - x1 >> 14 as libc::c_int) + 128 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn pjpeg_idct_2D_nanojpeg(
    mut in_0: *mut int32_t,
    mut out: *mut uint8_t,
    mut outstride: uint32_t,
) {
    let mut coef: libc::c_int = 0;
    coef = 0 as libc::c_int;
    while coef < 64 as libc::c_int {
        njRowIDCT(&mut *in_0.offset(coef as isize));
        coef += 8 as libc::c_int;
    }
    coef = 0 as libc::c_int;
    while coef < 8 as libc::c_int {
        njColIDCT(
            &mut *in_0.offset(coef as isize),
            &mut *out.offset(coef as isize),
            outstride as libc::c_int,
        );
        coef += 1;
    }
}
