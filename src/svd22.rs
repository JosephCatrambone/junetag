use ::libc;
extern "C" {
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn fmin(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn svd22(
    mut A: *const libc::c_double,
    mut U: *mut libc::c_double,
    mut S: *mut libc::c_double,
    mut V: *mut libc::c_double,
) {
    let mut A00: libc::c_double = *A.offset(0 as libc::c_int as isize);
    let mut A01: libc::c_double = *A.offset(1 as libc::c_int as isize);
    let mut A10: libc::c_double = *A.offset(2 as libc::c_int as isize);
    let mut A11: libc::c_double = *A.offset(3 as libc::c_int as isize);
    let mut B0: libc::c_double = A00 + A11;
    let mut B1: libc::c_double = A00 - A11;
    let mut B2: libc::c_double = A01 + A10;
    let mut B3: libc::c_double = A01 - A10;
    let mut PminusT: libc::c_double = atan2(B3, B0);
    let mut PplusT: libc::c_double = atan2(B2, B1);
    let mut P: libc::c_double = (PminusT + PplusT) / 2 as libc::c_int as libc::c_double;
    let mut T: libc::c_double = (-PminusT + PplusT) / 2 as libc::c_int as libc::c_double;
    let mut CP: libc::c_double = cos(P);
    let mut SP: libc::c_double = sin(P);
    let mut CT: libc::c_double = cos(T);
    let mut ST: libc::c_double = sin(T);
    *U.offset(0 as libc::c_int as isize) = CT;
    *U.offset(1 as libc::c_int as isize) = -ST;
    *U.offset(2 as libc::c_int as isize) = ST;
    *U.offset(3 as libc::c_int as isize) = CT;
    *V.offset(0 as libc::c_int as isize) = CP;
    *V.offset(1 as libc::c_int as isize) = -SP;
    *V.offset(2 as libc::c_int as isize) = SP;
    *V.offset(3 as libc::c_int as isize) = CP;
    let mut CPmT: libc::c_double = cos(P - T);
    let mut SPmT: libc::c_double = sin(P - T);
    let mut C0: libc::c_double = 0 as libc::c_int as libc::c_double;
    if fabs(CPmT) > fabs(SPmT) {
        C0 = B0 / CPmT;
    } else {
        C0 = B3 / SPmT;
    }
    let mut CPpT: libc::c_double = cos(P + T);
    let mut SPpT: libc::c_double = sin(P + T);
    let mut C1: libc::c_double = 0 as libc::c_int as libc::c_double;
    if fabs(CPpT) > fabs(SPpT) {
        C1 = B1 / CPpT;
    } else {
        C1 = B2 / SPpT;
    }
    let mut e: libc::c_double = (C0 + C1) / 2 as libc::c_int as libc::c_double;
    let mut f: libc::c_double = (C0 - C1) / 2 as libc::c_int as libc::c_double;
    if e < 0 as libc::c_int as libc::c_double {
        e = -e;
        *U.offset(0 as libc::c_int as isize) = -*U.offset(0 as libc::c_int as isize);
        *U.offset(2 as libc::c_int as isize) = -*U.offset(2 as libc::c_int as isize);
    }
    if f < 0 as libc::c_int as libc::c_double {
        f = -f;
        *U.offset(1 as libc::c_int as isize) = -*U.offset(1 as libc::c_int as isize);
        *U.offset(3 as libc::c_int as isize) = -*U.offset(3 as libc::c_int as isize);
    }
    if e > f {
        *S.offset(0 as libc::c_int as isize) = e;
        *S.offset(1 as libc::c_int as isize) = f;
    } else {
        *S.offset(0 as libc::c_int as isize) = f;
        *S.offset(1 as libc::c_int as isize) = e;
        let mut tmp: [libc::c_double; 2] = [0.; 2];
        tmp[0 as libc::c_int as usize] = *U.offset(0 as libc::c_int as isize);
        tmp[1 as libc::c_int as usize] = *U.offset(2 as libc::c_int as isize);
        *U.offset(0 as libc::c_int as isize) = *U.offset(1 as libc::c_int as isize);
        *U.offset(2 as libc::c_int as isize) = *U.offset(3 as libc::c_int as isize);
        *U.offset(1 as libc::c_int as isize) = tmp[0 as libc::c_int as usize];
        *U.offset(3 as libc::c_int as isize) = tmp[1 as libc::c_int as usize];
        tmp[0 as libc::c_int as usize] = *V.offset(0 as libc::c_int as isize);
        tmp[1 as libc::c_int as usize] = *V.offset(2 as libc::c_int as isize);
        *V.offset(0 as libc::c_int as isize) = *V.offset(1 as libc::c_int as isize);
        *V.offset(2 as libc::c_int as isize) = *V.offset(3 as libc::c_int as isize);
        *V.offset(1 as libc::c_int as isize) = tmp[0 as libc::c_int as usize];
        *V.offset(3 as libc::c_int as isize) = tmp[1 as libc::c_int as usize];
    };
}
#[no_mangle]
pub unsafe extern "C" fn svd_sym_singular_values(
    mut A00: libc::c_double,
    mut A01: libc::c_double,
    mut A11: libc::c_double,
    mut Lmin: *mut libc::c_double,
    mut Lmax: *mut libc::c_double,
) {
    let mut A10: libc::c_double = A01;
    let mut B0: libc::c_double = A00 + A11;
    let mut B1: libc::c_double = A00 - A11;
    let mut B2: libc::c_double = A01 + A10;
    let mut B3: libc::c_double = A01 - A10;
    let mut PminusT: libc::c_double = atan2(B3, B0);
    let mut PplusT: libc::c_double = atan2(B2, B1);
    let mut P: libc::c_double = (PminusT + PplusT) / 2 as libc::c_int as libc::c_double;
    let mut T: libc::c_double = (-PminusT + PplusT) / 2 as libc::c_int as libc::c_double;
    let mut CPmT: libc::c_double = cos(P - T);
    let mut SPmT: libc::c_double = sin(P - T);
    let mut C0: libc::c_double = 0 as libc::c_int as libc::c_double;
    if fabs(CPmT) > fabs(SPmT) {
        C0 = B0 / CPmT;
    } else {
        C0 = B3 / SPmT;
    }
    let mut CPpT: libc::c_double = cos(P + T);
    let mut SPpT: libc::c_double = sin(P + T);
    let mut C1: libc::c_double = 0 as libc::c_int as libc::c_double;
    if fabs(CPpT) > fabs(SPpT) {
        C1 = B1 / CPpT;
    } else {
        C1 = B2 / SPpT;
    }
    let mut e: libc::c_double = (C0 + C1) / 2 as libc::c_int as libc::c_double;
    let mut f: libc::c_double = (C0 - C1) / 2 as libc::c_int as libc::c_double;
    *Lmin = fmin(e, f);
    *Lmax = fmax(e, f);
}
