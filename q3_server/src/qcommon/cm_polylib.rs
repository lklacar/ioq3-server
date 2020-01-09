// =============== BEGIN cm_polylib_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct winding_t {
    pub numpoints: libc::c_int,
    pub p: [crate::src::qcommon::q_shared::vec3_t; 0],
}
use ::libc;

pub mod q_shared_h {

    // parameters to the main Error routine

    // pop up the need-cd dialog
    // client disconnected from the server

    // don't kill server

    // print to console and disconnect from game

    // exit the entire game with a popup window

    /*
    ==============================================================

    MATHLIB

    ==============================================================
    */

    #[inline]

    pub unsafe extern "C" fn VectorLength(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return crate::stdlib::sqrt(
            (*v.offset(0 as libc::c_int as isize) * *v.offset(0 as libc::c_int as isize)
                + *v.offset(1 as libc::c_int as isize) * *v.offset(1 as libc::c_int as isize)
                + *v.offset(2 as libc::c_int as isize) * *v.offset(2 as libc::c_int as isize))
                as libc::c_double,
        ) as crate::src::qcommon::q_shared::vec_t;
    }
    #[inline]

    pub unsafe extern "C" fn CrossProduct(
        mut v1: *const crate::src::qcommon::q_shared::vec_t,
        mut v2: *const crate::src::qcommon::q_shared::vec_t,
        mut cross: *mut crate::src::qcommon::q_shared::vec_t,
    ) {
        *cross.offset(0 as libc::c_int as isize) = *v1.offset(1 as libc::c_int as isize)
            * *v2.offset(2 as libc::c_int as isize)
            - *v1.offset(2 as libc::c_int as isize) * *v2.offset(1 as libc::c_int as isize);
        *cross.offset(1 as libc::c_int as isize) = *v1.offset(2 as libc::c_int as isize)
            * *v2.offset(0 as libc::c_int as isize)
            - *v1.offset(0 as libc::c_int as isize) * *v2.offset(2 as libc::c_int as isize);
        *cross.offset(2 as libc::c_int as isize) = *v1.offset(0 as libc::c_int as isize)
            * *v2.offset(1 as libc::c_int as isize)
            - *v1.offset(1 as libc::c_int as isize) * *v2.offset(0 as libc::c_int as isize);
    }
    use crate::stdlib::sqrt;

    // __Q_SHARED_H
}

pub use crate::stdlib::intptr_t;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::src::qcommon::cm_polylib::q_shared_h::CrossProduct;
pub use crate::src::qcommon::cm_polylib::q_shared_h::VectorLength;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::VectorNormalize2;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
use crate::stdlib::fabs;
use crate::stdlib::printf;
use crate::stdlib::sqrt;

use crate::src::qcommon::common::Z_Free;
use crate::src::qcommon::common::Z_MallocDebug;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
// this is only used for visualization tools in cm_ debug functions
// counters are only bumped when running single threaded,
// because they are an awful coherence problem
#[no_mangle]

pub static mut c_active_windings: libc::c_int = 0;
#[no_mangle]

pub static mut c_peak_windings: libc::c_int = 0;
#[no_mangle]

pub static mut c_winding_allocs: libc::c_int = 0;
#[no_mangle]

pub static mut c_winding_points: libc::c_int = 0;
// frees the original if clipped
#[no_mangle]

pub unsafe extern "C" fn pw(mut w: *mut crate::src::qcommon::cm_polylib::winding_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*w).numpoints {
        crate::stdlib::printf(
            b"(%5.1f, %5.1f, %5.1f)\n\x00" as *const u8 as *const libc::c_char,
            (*(*w).p.as_mut_ptr().offset(i as isize))[0 as libc::c_int as usize] as libc::c_double,
            (*(*w).p.as_mut_ptr().offset(i as isize))[1 as libc::c_int as usize] as libc::c_double,
            (*(*w).p.as_mut_ptr().offset(i as isize))[2 as libc::c_int as usize] as libc::c_double,
        );
        i += 1
    }
}
/*
=============
AllocWinding
=============
*/
#[no_mangle]

pub unsafe extern "C" fn AllocWinding(
    mut points: libc::c_int,
) -> *mut crate::src::qcommon::cm_polylib::winding_t {
    let mut w: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    let mut s: libc::c_int = 0;
    c_winding_allocs += 1;
    c_winding_points += points;
    c_active_windings += 1;
    if c_active_windings > c_peak_windings {
        c_peak_windings = c_active_windings
    }
    s = (::std::mem::size_of::<crate::src::qcommon::q_shared::vec_t>() as libc::c_ulong)
        .wrapping_mul(3 as libc::c_int as libc::c_ulong)
        .wrapping_mul(points as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<libc::c_int>() as libc::c_ulong) as libc::c_int;
    w = crate::src::qcommon::common::Z_MallocDebug(
        s,
        b"s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/qcommon/cm_polylib.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        61 as libc::c_int,
    ) as *mut crate::src::qcommon::cm_polylib::winding_t;
    crate::stdlib::memset(w as *mut libc::c_void, 0 as libc::c_int, s as libc::c_ulong);
    return w;
}
#[no_mangle]

pub unsafe extern "C" fn FreeWinding(mut w: *mut crate::src::qcommon::cm_polylib::winding_t) {
    if *(w as *mut libc::c_uint) == 0xdeaddead as libc::c_uint {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"FreeWinding: freed a freed winding\x00" as *const u8 as *const libc::c_char,
        );
    }
    *(w as *mut libc::c_uint) = 0xdeaddead as libc::c_uint;
    c_active_windings -= 1;
    crate::src::qcommon::common::Z_Free(w as *mut libc::c_void);
}
/*
============
RemoveColinearPoints
============
*/
#[no_mangle]

pub static mut c_removed: libc::c_int = 0;
#[no_mangle]

pub unsafe extern "C" fn RemoveColinearPoints(
    mut w: *mut crate::src::qcommon::cm_polylib::winding_t,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut v1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut nump: libc::c_int = 0;
    let mut p: [crate::src::qcommon::q_shared::vec3_t; 64] = [[0.; 3]; 64];
    nump = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*w).numpoints {
        j = (i + 1 as libc::c_int) % (*w).numpoints;
        k = (i + (*w).numpoints - 1 as libc::c_int) % (*w).numpoints;
        v1[0 as libc::c_int as usize] = (*(*w).p.as_mut_ptr().offset(j as isize))
            [0 as libc::c_int as usize]
            - (*(*w).p.as_mut_ptr().offset(i as isize))[0 as libc::c_int as usize];
        v1[1 as libc::c_int as usize] = (*(*w).p.as_mut_ptr().offset(j as isize))
            [1 as libc::c_int as usize]
            - (*(*w).p.as_mut_ptr().offset(i as isize))[1 as libc::c_int as usize];
        v1[2 as libc::c_int as usize] = (*(*w).p.as_mut_ptr().offset(j as isize))
            [2 as libc::c_int as usize]
            - (*(*w).p.as_mut_ptr().offset(i as isize))[2 as libc::c_int as usize];
        v2[0 as libc::c_int as usize] = (*(*w).p.as_mut_ptr().offset(i as isize))
            [0 as libc::c_int as usize]
            - (*(*w).p.as_mut_ptr().offset(k as isize))[0 as libc::c_int as usize];
        v2[1 as libc::c_int as usize] = (*(*w).p.as_mut_ptr().offset(i as isize))
            [1 as libc::c_int as usize]
            - (*(*w).p.as_mut_ptr().offset(k as isize))[1 as libc::c_int as usize];
        v2[2 as libc::c_int as usize] = (*(*w).p.as_mut_ptr().offset(i as isize))
            [2 as libc::c_int as usize]
            - (*(*w).p.as_mut_ptr().offset(k as isize))[2 as libc::c_int as usize];
        crate::src::qcommon::q_math::VectorNormalize2(
            v1.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            v1.as_mut_ptr(),
        );
        crate::src::qcommon::q_math::VectorNormalize2(
            v2.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            v2.as_mut_ptr(),
        );
        if ((v1[0 as libc::c_int as usize] * v2[0 as libc::c_int as usize]
            + v1[1 as libc::c_int as usize] * v2[1 as libc::c_int as usize]
            + v1[2 as libc::c_int as usize] * v2[2 as libc::c_int as usize])
            as libc::c_double)
            < 0.999f64
        {
            p[nump as usize][0 as libc::c_int as usize] =
                (*(*w).p.as_mut_ptr().offset(i as isize))[0 as libc::c_int as usize];
            p[nump as usize][1 as libc::c_int as usize] =
                (*(*w).p.as_mut_ptr().offset(i as isize))[1 as libc::c_int as usize];
            p[nump as usize][2 as libc::c_int as usize] =
                (*(*w).p.as_mut_ptr().offset(i as isize))[2 as libc::c_int as usize];
            nump += 1
        }
        i += 1
    }
    if nump == (*w).numpoints {
        return;
    }
    c_removed += (*w).numpoints - nump;
    (*w).numpoints = nump;
    crate::stdlib::memcpy(
        (*w).p.as_mut_ptr() as *mut libc::c_void,
        p.as_mut_ptr() as *const libc::c_void,
        (nump as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            crate::src::qcommon::q_shared::vec3_t,
        >() as libc::c_ulong),
    );
}
/*
============
WindingPlane
============
*/
#[no_mangle]

pub unsafe extern "C" fn WindingPlane(
    mut w: *mut crate::src::qcommon::cm_polylib::winding_t,
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
    mut dist: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut v1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    v1[0 as libc::c_int as usize] = (*(*w).p.as_mut_ptr().offset(1 as libc::c_int as isize))
        [0 as libc::c_int as usize]
        - (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[0 as libc::c_int as usize];
    v1[1 as libc::c_int as usize] = (*(*w).p.as_mut_ptr().offset(1 as libc::c_int as isize))
        [1 as libc::c_int as usize]
        - (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[1 as libc::c_int as usize];
    v1[2 as libc::c_int as usize] = (*(*w).p.as_mut_ptr().offset(1 as libc::c_int as isize))
        [2 as libc::c_int as usize]
        - (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[2 as libc::c_int as usize];
    v2[0 as libc::c_int as usize] = (*(*w).p.as_mut_ptr().offset(2 as libc::c_int as isize))
        [0 as libc::c_int as usize]
        - (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[0 as libc::c_int as usize];
    v2[1 as libc::c_int as usize] = (*(*w).p.as_mut_ptr().offset(2 as libc::c_int as isize))
        [1 as libc::c_int as usize]
        - (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[1 as libc::c_int as usize];
    v2[2 as libc::c_int as usize] = (*(*w).p.as_mut_ptr().offset(2 as libc::c_int as isize))
        [2 as libc::c_int as usize]
        - (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[2 as libc::c_int as usize];
    CrossProduct(
        v2.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        v1.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        normal,
    );
    crate::src::qcommon::q_math::VectorNormalize2(
        normal as *const crate::src::qcommon::q_shared::vec_t,
        normal,
    );
    *dist = (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        * *normal.offset(0 as libc::c_int as isize)
        + (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
            * *normal.offset(1 as libc::c_int as isize)
        + (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
            * *normal.offset(2 as libc::c_int as isize);
}
/*
=============
WindingArea
=============
*/
#[no_mangle]

pub unsafe extern "C" fn WindingArea(
    mut w: *mut crate::src::qcommon::cm_polylib::winding_t,
) -> crate::src::qcommon::q_shared::vec_t {
    let mut i: libc::c_int = 0;
    let mut d1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut d2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut cross: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut total: crate::src::qcommon::q_shared::vec_t = 0.;
    total = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    i = 2 as libc::c_int;
    while i < (*w).numpoints {
        d1[0 as libc::c_int as usize] = (*(*w)
            .p
            .as_mut_ptr()
            .offset((i - 1 as libc::c_int) as isize))[0 as libc::c_int as usize]
            - (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[0 as libc::c_int as usize];
        d1[1 as libc::c_int as usize] = (*(*w)
            .p
            .as_mut_ptr()
            .offset((i - 1 as libc::c_int) as isize))[1 as libc::c_int as usize]
            - (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[1 as libc::c_int as usize];
        d1[2 as libc::c_int as usize] = (*(*w)
            .p
            .as_mut_ptr()
            .offset((i - 1 as libc::c_int) as isize))[2 as libc::c_int as usize]
            - (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[2 as libc::c_int as usize];
        d2[0 as libc::c_int as usize] = (*(*w).p.as_mut_ptr().offset(i as isize))
            [0 as libc::c_int as usize]
            - (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[0 as libc::c_int as usize];
        d2[1 as libc::c_int as usize] = (*(*w).p.as_mut_ptr().offset(i as isize))
            [1 as libc::c_int as usize]
            - (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[1 as libc::c_int as usize];
        d2[2 as libc::c_int as usize] = (*(*w).p.as_mut_ptr().offset(i as isize))
            [2 as libc::c_int as usize]
            - (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[2 as libc::c_int as usize];
        CrossProduct(
            d1.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            d2.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            cross.as_mut_ptr(),
        );
        total = (total as libc::c_double
            + 0.5f64
                * VectorLength(cross.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t)
                    as libc::c_double) as crate::src::qcommon::q_shared::vec_t;
        i += 1
    }
    return total;
}
/*
=============
WindingBounds
=============
*/
#[no_mangle]

pub unsafe extern "C" fn WindingBounds(
    mut w: *mut crate::src::qcommon::cm_polylib::winding_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut v: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let ref mut fresh0 = *mins.offset(2 as libc::c_int as isize);
    *fresh0 = 65535 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    let ref mut fresh1 = *mins.offset(1 as libc::c_int as isize);
    *fresh1 = *fresh0;
    *mins.offset(0 as libc::c_int as isize) = *fresh1;
    let ref mut fresh2 = *maxs.offset(2 as libc::c_int as isize);
    *fresh2 = -(65535 as libc::c_int) as crate::src::qcommon::q_shared::vec_t;
    let ref mut fresh3 = *maxs.offset(1 as libc::c_int as isize);
    *fresh3 = *fresh2;
    *maxs.offset(0 as libc::c_int as isize) = *fresh3;
    i = 0 as libc::c_int;
    while i < (*w).numpoints {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            v = (*(*w).p.as_mut_ptr().offset(i as isize))[j as usize];
            if v < *mins.offset(j as isize) {
                *mins.offset(j as isize) = v
            }
            if v > *maxs.offset(j as isize) {
                *maxs.offset(j as isize) = v
            }
            j += 1
        }
        i += 1
    }
}
/*
=============
WindingCenter
=============
*/
#[no_mangle]

pub unsafe extern "C" fn WindingCenter(
    mut w: *mut crate::src::qcommon::cm_polylib::winding_t,
    mut center: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut i: libc::c_int = 0;
    let mut scale: libc::c_float = 0.;
    *center.offset(0 as libc::c_int as isize) =
        crate::src::qcommon::q_math::vec3_origin[0 as libc::c_int as usize];
    *center.offset(1 as libc::c_int as isize) =
        crate::src::qcommon::q_math::vec3_origin[1 as libc::c_int as usize];
    *center.offset(2 as libc::c_int as isize) =
        crate::src::qcommon::q_math::vec3_origin[2 as libc::c_int as usize];
    i = 0 as libc::c_int;
    while i < (*w).numpoints {
        *center.offset(0 as libc::c_int as isize) = (*(*w).p.as_mut_ptr().offset(i as isize))
            [0 as libc::c_int as usize]
            + *center.offset(0 as libc::c_int as isize);
        *center.offset(1 as libc::c_int as isize) = (*(*w).p.as_mut_ptr().offset(i as isize))
            [1 as libc::c_int as usize]
            + *center.offset(1 as libc::c_int as isize);
        *center.offset(2 as libc::c_int as isize) = (*(*w).p.as_mut_ptr().offset(i as isize))
            [2 as libc::c_int as usize]
            + *center.offset(2 as libc::c_int as isize);
        i += 1
    }
    scale = (1.0f64 / (*w).numpoints as libc::c_double) as libc::c_float;
    *center.offset(0 as libc::c_int as isize) = *center.offset(0 as libc::c_int as isize) * scale;
    *center.offset(1 as libc::c_int as isize) = *center.offset(1 as libc::c_int as isize) * scale;
    *center.offset(2 as libc::c_int as isize) = *center.offset(2 as libc::c_int as isize) * scale;
}
/*
=================
BaseWindingForPlane
=================
*/
#[no_mangle]

pub unsafe extern "C" fn BaseWindingForPlane(
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
    mut dist: crate::src::qcommon::q_shared::vec_t,
) -> *mut crate::src::qcommon::cm_polylib::winding_t {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut max: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut v: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut org: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vright: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vup: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut w: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    // find the major axis
    max = -(65535 as libc::c_int) as crate::src::qcommon::q_shared::vec_t;
    x = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        v = crate::stdlib::fabs(*normal.offset(i as isize) as libc::c_double)
            as crate::src::qcommon::q_shared::vec_t;
        if v > max {
            x = i;
            max = v
        }
        i += 1
    }
    if x == -(1 as libc::c_int) {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"BaseWindingForPlane: no axis found\x00" as *const u8 as *const libc::c_char,
        );
    }
    vup[0 as libc::c_int as usize] =
        crate::src::qcommon::q_math::vec3_origin[0 as libc::c_int as usize];
    vup[1 as libc::c_int as usize] =
        crate::src::qcommon::q_math::vec3_origin[1 as libc::c_int as usize];
    vup[2 as libc::c_int as usize] =
        crate::src::qcommon::q_math::vec3_origin[2 as libc::c_int as usize];
    match x {
        0 | 1 => {
            vup[2 as libc::c_int as usize] =
                1 as libc::c_int as crate::src::qcommon::q_shared::vec_t
        }
        2 => {
            vup[0 as libc::c_int as usize] =
                1 as libc::c_int as crate::src::qcommon::q_shared::vec_t
        }
        _ => {}
    }
    v = vup[0 as libc::c_int as usize] * *normal.offset(0 as libc::c_int as isize)
        + vup[1 as libc::c_int as usize] * *normal.offset(1 as libc::c_int as isize)
        + vup[2 as libc::c_int as usize] * *normal.offset(2 as libc::c_int as isize);
    vup[0 as libc::c_int as usize] =
        vup[0 as libc::c_int as usize] + *normal.offset(0 as libc::c_int as isize) * -v;
    vup[1 as libc::c_int as usize] =
        vup[1 as libc::c_int as usize] + *normal.offset(1 as libc::c_int as isize) * -v;
    vup[2 as libc::c_int as usize] =
        vup[2 as libc::c_int as usize] + *normal.offset(2 as libc::c_int as isize) * -v;
    crate::src::qcommon::q_math::VectorNormalize2(
        vup.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        vup.as_mut_ptr(),
    );
    org[0 as libc::c_int as usize] = *normal.offset(0 as libc::c_int as isize) * dist;
    org[1 as libc::c_int as usize] = *normal.offset(1 as libc::c_int as isize) * dist;
    org[2 as libc::c_int as usize] = *normal.offset(2 as libc::c_int as isize) * dist;
    CrossProduct(
        vup.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        normal as *const crate::src::qcommon::q_shared::vec_t,
        vright.as_mut_ptr(),
    );
    vup[0 as libc::c_int as usize] =
        vup[0 as libc::c_int as usize] * 65535 as libc::c_int as libc::c_float;
    vup[1 as libc::c_int as usize] =
        vup[1 as libc::c_int as usize] * 65535 as libc::c_int as libc::c_float;
    vup[2 as libc::c_int as usize] =
        vup[2 as libc::c_int as usize] * 65535 as libc::c_int as libc::c_float;
    vright[0 as libc::c_int as usize] =
        vright[0 as libc::c_int as usize] * 65535 as libc::c_int as libc::c_float;
    vright[1 as libc::c_int as usize] =
        vright[1 as libc::c_int as usize] * 65535 as libc::c_int as libc::c_float;
    vright[2 as libc::c_int as usize] =
        vright[2 as libc::c_int as usize] * 65535 as libc::c_int as libc::c_float;
    // project a really big	axis aligned box onto the plane
    w = AllocWinding(4 as libc::c_int);
    (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        org[0 as libc::c_int as usize] - vright[0 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
        org[1 as libc::c_int as usize] - vright[1 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        org[2 as libc::c_int as usize] - vright[2 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
            + vup[0 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
            + vup[1 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
            + vup[2 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        org[0 as libc::c_int as usize] + vright[0 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        org[1 as libc::c_int as usize] + vright[1 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        org[2 as libc::c_int as usize] + vright[2 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*(*w).p.as_mut_ptr().offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
            + vup[0 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*(*w).p.as_mut_ptr().offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
            + vup[1 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(1 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*(*w).p.as_mut_ptr().offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
            + vup[2 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        org[0 as libc::c_int as usize] + vright[0 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
        org[1 as libc::c_int as usize] + vright[1 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        org[2 as libc::c_int as usize] + vright[2 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*(*w).p.as_mut_ptr().offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
            - vup[0 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(2 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*(*w).p.as_mut_ptr().offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
            - vup[1 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(2 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*(*w).p.as_mut_ptr().offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
            - vup[2 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] =
        org[0 as libc::c_int as usize] - vright[0 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(3 as libc::c_int as isize))[1 as libc::c_int as usize] =
        org[1 as libc::c_int as usize] - vright[1 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(3 as libc::c_int as isize))[2 as libc::c_int as usize] =
        org[2 as libc::c_int as usize] - vright[2 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (*(*w).p.as_mut_ptr().offset(3 as libc::c_int as isize))[0 as libc::c_int as usize]
            - vup[0 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(3 as libc::c_int as isize))[1 as libc::c_int as usize] =
        (*(*w).p.as_mut_ptr().offset(3 as libc::c_int as isize))[1 as libc::c_int as usize]
            - vup[1 as libc::c_int as usize];
    (*(*w).p.as_mut_ptr().offset(3 as libc::c_int as isize))[2 as libc::c_int as usize] =
        (*(*w).p.as_mut_ptr().offset(3 as libc::c_int as isize))[2 as libc::c_int as usize]
            - vup[2 as libc::c_int as usize];
    (*w).numpoints = 4 as libc::c_int;
    return w;
}
/*
==================
CopyWinding
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CopyWinding(
    mut w: *mut crate::src::qcommon::cm_polylib::winding_t,
) -> *mut crate::src::qcommon::cm_polylib::winding_t {
    let mut size: crate::stdlib::intptr_t = 0;
    let mut c: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    c = AllocWinding((*w).numpoints);
    size = &mut *(*w).p.as_mut_ptr().offset((*w).numpoints as isize)
        as *mut crate::src::qcommon::q_shared::vec3_t as crate::stdlib::intptr_t
        - w as crate::stdlib::intptr_t;
    crate::stdlib::memcpy(
        c as *mut libc::c_void,
        w as *const libc::c_void,
        size as libc::c_ulong,
    );
    return c;
}
/*
==================
ReverseWinding
==================
*/
#[no_mangle]

pub unsafe extern "C" fn ReverseWinding(
    mut w: *mut crate::src::qcommon::cm_polylib::winding_t,
) -> *mut crate::src::qcommon::cm_polylib::winding_t {
    let mut i: libc::c_int = 0;
    let mut c: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    c = AllocWinding((*w).numpoints);
    i = 0 as libc::c_int;
    while i < (*w).numpoints {
        (*(*c).p.as_mut_ptr().offset(i as isize))[0 as libc::c_int as usize] = (*(*w)
            .p
            .as_mut_ptr()
            .offset(((*w).numpoints - 1 as libc::c_int - i) as isize))[0 as libc::c_int as usize];
        (*(*c).p.as_mut_ptr().offset(i as isize))[1 as libc::c_int as usize] = (*(*w)
            .p
            .as_mut_ptr()
            .offset(((*w).numpoints - 1 as libc::c_int - i) as isize))[1 as libc::c_int as usize];
        (*(*c).p.as_mut_ptr().offset(i as isize))[2 as libc::c_int as usize] = (*(*w)
            .p
            .as_mut_ptr()
            .offset(((*w).numpoints - 1 as libc::c_int - i) as isize))[2 as libc::c_int as usize];
        i += 1
    }
    (*c).numpoints = (*w).numpoints;
    return c;
}
/*
=============
ClipWindingEpsilon
=============
*/
#[no_mangle]

pub unsafe extern "C" fn ClipWindingEpsilon(
    mut in_0: *mut crate::src::qcommon::cm_polylib::winding_t,
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
    mut dist: crate::src::qcommon::q_shared::vec_t,
    mut epsilon: crate::src::qcommon::q_shared::vec_t,
    mut front: *mut *mut crate::src::qcommon::cm_polylib::winding_t,
    mut back: *mut *mut crate::src::qcommon::cm_polylib::winding_t,
) {
    let mut dists: [crate::src::qcommon::q_shared::vec_t; 68] = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ]; // VC 4.2 optimizer bug if not static
    let mut sides: [libc::c_int; 68] = [
        0 as libc::c_int,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut counts: [libc::c_int; 3] = [0; 3];
    static mut dot: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p1: *mut crate::src::qcommon::q_shared::vec_t =
        0 as *mut crate::src::qcommon::q_shared::vec_t;
    let mut p2: *mut crate::src::qcommon::q_shared::vec_t =
        0 as *mut crate::src::qcommon::q_shared::vec_t;
    let mut mid: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut f: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    let mut b: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    let mut maxpts: libc::c_int = 0;
    counts[2 as libc::c_int as usize] = 0 as libc::c_int;
    counts[1 as libc::c_int as usize] = counts[2 as libc::c_int as usize];
    counts[0 as libc::c_int as usize] = counts[1 as libc::c_int as usize];
    // determine sides for each point
    i = 0 as libc::c_int; // can't use counts[0]+2 because
    while i < (*in_0).numpoints {
        dot = (*(*in_0).p.as_mut_ptr().offset(i as isize))[0 as libc::c_int as usize]
            * *normal.offset(0 as libc::c_int as isize)
            + (*(*in_0).p.as_mut_ptr().offset(i as isize))[1 as libc::c_int as usize]
                * *normal.offset(1 as libc::c_int as isize)
            + (*(*in_0).p.as_mut_ptr().offset(i as isize))[2 as libc::c_int as usize]
                * *normal.offset(2 as libc::c_int as isize);
        dot -= dist;
        dists[i as usize] = dot;
        if dot > epsilon {
            sides[i as usize] = 0 as libc::c_int
        } else if dot < -epsilon {
            sides[i as usize] = 1 as libc::c_int
        } else {
            sides[i as usize] = 2 as libc::c_int
        }
        counts[sides[i as usize] as usize] += 1;
        i += 1
    }
    sides[i as usize] = sides[0 as libc::c_int as usize];
    dists[i as usize] = dists[0 as libc::c_int as usize];
    *back = 0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    *front = *back;
    if counts[0 as libc::c_int as usize] == 0 {
        *back = CopyWinding(in_0);
        return;
    }
    if counts[1 as libc::c_int as usize] == 0 {
        *front = CopyWinding(in_0);
        return;
    }
    maxpts = (*in_0).numpoints + 4 as libc::c_int;
    // of fp grouping errors
    f = AllocWinding(maxpts);
    *front = f;
    b = AllocWinding(maxpts);
    *back = b;
    i = 0 as libc::c_int;
    while i < (*in_0).numpoints {
        p1 = (*(*in_0).p.as_mut_ptr().offset(i as isize)).as_mut_ptr();
        if sides[i as usize] == 2 as libc::c_int {
            (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[0 as libc::c_int as usize] =
                *p1.offset(0 as libc::c_int as isize);
            (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[1 as libc::c_int as usize] =
                *p1.offset(1 as libc::c_int as isize);
            (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[2 as libc::c_int as usize] =
                *p1.offset(2 as libc::c_int as isize);
            (*f).numpoints += 1;
            (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[0 as libc::c_int as usize] =
                *p1.offset(0 as libc::c_int as isize);
            (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[1 as libc::c_int as usize] =
                *p1.offset(1 as libc::c_int as isize);
            (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[2 as libc::c_int as usize] =
                *p1.offset(2 as libc::c_int as isize);
            (*b).numpoints += 1
        } else {
            if sides[i as usize] == 0 as libc::c_int {
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[0 as libc::c_int as usize] =
                    *p1.offset(0 as libc::c_int as isize);
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[1 as libc::c_int as usize] =
                    *p1.offset(1 as libc::c_int as isize);
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[2 as libc::c_int as usize] =
                    *p1.offset(2 as libc::c_int as isize);
                (*f).numpoints += 1
            }
            if sides[i as usize] == 1 as libc::c_int {
                (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[0 as libc::c_int as usize] =
                    *p1.offset(0 as libc::c_int as isize);
                (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[1 as libc::c_int as usize] =
                    *p1.offset(1 as libc::c_int as isize);
                (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[2 as libc::c_int as usize] =
                    *p1.offset(2 as libc::c_int as isize);
                (*b).numpoints += 1
            }
            if !(sides[(i + 1 as libc::c_int) as usize] == 2 as libc::c_int
                || sides[(i + 1 as libc::c_int) as usize] == sides[i as usize])
            {
                // generate a split point
                p2 = (*(*in_0)
                    .p
                    .as_mut_ptr()
                    .offset(((i + 1 as libc::c_int) % (*in_0).numpoints) as isize))
                .as_mut_ptr();
                dot = dists[i as usize]
                    / (dists[i as usize] - dists[(i + 1 as libc::c_int) as usize]);
                j = 0 as libc::c_int;
                while j < 3 as libc::c_int {
                    // avoid round off error when possible
                    if *normal.offset(j as isize) == 1 as libc::c_int as libc::c_float {
                        mid[j as usize] = dist
                    } else if *normal.offset(j as isize) == -(1 as libc::c_int) as libc::c_float {
                        mid[j as usize] = -dist
                    } else {
                        mid[j as usize] = *p1.offset(j as isize)
                            + dot * (*p2.offset(j as isize) - *p1.offset(j as isize))
                    }
                    j += 1
                }
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[0 as libc::c_int as usize] =
                    mid[0 as libc::c_int as usize];
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[1 as libc::c_int as usize] =
                    mid[1 as libc::c_int as usize];
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[2 as libc::c_int as usize] =
                    mid[2 as libc::c_int as usize];
                (*f).numpoints += 1;
                (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[0 as libc::c_int as usize] =
                    mid[0 as libc::c_int as usize];
                (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[1 as libc::c_int as usize] =
                    mid[1 as libc::c_int as usize];
                (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[2 as libc::c_int as usize] =
                    mid[2 as libc::c_int as usize];
                (*b).numpoints += 1
            }
        }
        i += 1
    }
    if (*f).numpoints > maxpts || (*b).numpoints > maxpts {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"ClipWinding: points exceeded estimate\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*f).numpoints > 64 as libc::c_int || (*b).numpoints > 64 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"ClipWinding: MAX_POINTS_ON_WINDING\x00" as *const u8 as *const libc::c_char,
        );
    };
}
/*
=============
ChopWindingInPlace
=============
*/
#[no_mangle]

pub unsafe extern "C" fn ChopWindingInPlace(
    mut inout: *mut *mut crate::src::qcommon::cm_polylib::winding_t,
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
    mut dist: crate::src::qcommon::q_shared::vec_t,
    mut epsilon: crate::src::qcommon::q_shared::vec_t,
) {
    let mut in_0: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t; // VC 4.2 optimizer bug if not static
    let mut dists: [crate::src::qcommon::q_shared::vec_t; 68] = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    let mut sides: [libc::c_int; 68] = [
        0 as libc::c_int,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut counts: [libc::c_int; 3] = [0; 3];
    static mut dot: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p1: *mut crate::src::qcommon::q_shared::vec_t =
        0 as *mut crate::src::qcommon::q_shared::vec_t;
    let mut p2: *mut crate::src::qcommon::q_shared::vec_t =
        0 as *mut crate::src::qcommon::q_shared::vec_t;
    let mut mid: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut f: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    let mut maxpts: libc::c_int = 0;
    in_0 = *inout;
    counts[2 as libc::c_int as usize] = 0 as libc::c_int;
    counts[1 as libc::c_int as usize] = counts[2 as libc::c_int as usize];
    counts[0 as libc::c_int as usize] = counts[1 as libc::c_int as usize];
    // determine sides for each point
    i = 0 as libc::c_int; // inout stays the same
    while i < (*in_0).numpoints {
        dot = (*(*in_0).p.as_mut_ptr().offset(i as isize))[0 as libc::c_int as usize]
            * *normal.offset(0 as libc::c_int as isize)
            + (*(*in_0).p.as_mut_ptr().offset(i as isize))[1 as libc::c_int as usize]
                * *normal.offset(1 as libc::c_int as isize)
            + (*(*in_0).p.as_mut_ptr().offset(i as isize))[2 as libc::c_int as usize]
                * *normal.offset(2 as libc::c_int as isize); // can't use counts[0]+2 because
        dot -= dist;
        dists[i as usize] = dot;
        if dot > epsilon {
            sides[i as usize] = 0 as libc::c_int
        } else if dot < -epsilon {
            sides[i as usize] = 1 as libc::c_int
        } else {
            sides[i as usize] = 2 as libc::c_int
        }
        counts[sides[i as usize] as usize] += 1;
        i += 1
    }
    sides[i as usize] = sides[0 as libc::c_int as usize];
    dists[i as usize] = dists[0 as libc::c_int as usize];
    if counts[0 as libc::c_int as usize] == 0 {
        FreeWinding(in_0);
        *inout = 0 as *mut crate::src::qcommon::cm_polylib::winding_t;
        return;
    }
    if counts[1 as libc::c_int as usize] == 0 {
        return;
    }
    maxpts = (*in_0).numpoints + 4 as libc::c_int;
    // of fp grouping errors
    f = AllocWinding(maxpts);
    i = 0 as libc::c_int;
    while i < (*in_0).numpoints {
        p1 = (*(*in_0).p.as_mut_ptr().offset(i as isize)).as_mut_ptr();
        if sides[i as usize] == 2 as libc::c_int {
            (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[0 as libc::c_int as usize] =
                *p1.offset(0 as libc::c_int as isize);
            (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[1 as libc::c_int as usize] =
                *p1.offset(1 as libc::c_int as isize);
            (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[2 as libc::c_int as usize] =
                *p1.offset(2 as libc::c_int as isize);
            (*f).numpoints += 1
        } else {
            if sides[i as usize] == 0 as libc::c_int {
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[0 as libc::c_int as usize] =
                    *p1.offset(0 as libc::c_int as isize);
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[1 as libc::c_int as usize] =
                    *p1.offset(1 as libc::c_int as isize);
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[2 as libc::c_int as usize] =
                    *p1.offset(2 as libc::c_int as isize);
                (*f).numpoints += 1
            }
            if !(sides[(i + 1 as libc::c_int) as usize] == 2 as libc::c_int
                || sides[(i + 1 as libc::c_int) as usize] == sides[i as usize])
            {
                // generate a split point
                p2 = (*(*in_0)
                    .p
                    .as_mut_ptr()
                    .offset(((i + 1 as libc::c_int) % (*in_0).numpoints) as isize))
                .as_mut_ptr();
                dot = dists[i as usize]
                    / (dists[i as usize] - dists[(i + 1 as libc::c_int) as usize]);
                j = 0 as libc::c_int;
                while j < 3 as libc::c_int {
                    // avoid round off error when possible
                    if *normal.offset(j as isize) == 1 as libc::c_int as libc::c_float {
                        mid[j as usize] = dist
                    } else if *normal.offset(j as isize) == -(1 as libc::c_int) as libc::c_float {
                        mid[j as usize] = -dist
                    } else {
                        mid[j as usize] = *p1.offset(j as isize)
                            + dot * (*p2.offset(j as isize) - *p1.offset(j as isize))
                    }
                    j += 1
                }
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[0 as libc::c_int as usize] =
                    mid[0 as libc::c_int as usize];
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[1 as libc::c_int as usize] =
                    mid[1 as libc::c_int as usize];
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[2 as libc::c_int as usize] =
                    mid[2 as libc::c_int as usize];
                (*f).numpoints += 1
            }
        }
        i += 1
    }
    if (*f).numpoints > maxpts {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"ClipWinding: points exceeded estimate\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*f).numpoints > 64 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"ClipWinding: MAX_POINTS_ON_WINDING\x00" as *const u8 as *const libc::c_char,
        );
    }
    FreeWinding(in_0);
    *inout = f;
}
/*
=================
ChopWinding

Returns the fragment of in that is on the front side
of the cliping plane.  The original is freed.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ChopWinding(
    mut in_0: *mut crate::src::qcommon::cm_polylib::winding_t,
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
    mut dist: crate::src::qcommon::q_shared::vec_t,
) -> *mut crate::src::qcommon::cm_polylib::winding_t {
    let mut f: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    let mut b: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    ClipWindingEpsilon(in_0, normal, dist, 0.1f32, &mut f, &mut b);
    FreeWinding(in_0);
    if !b.is_null() {
        FreeWinding(b);
    }
    return f;
}
/*
=================
CheckWinding

=================
*/
#[no_mangle]

pub unsafe extern "C" fn CheckWinding(mut w: *mut crate::src::qcommon::cm_polylib::winding_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p1: *mut crate::src::qcommon::q_shared::vec_t =
        0 as *mut crate::src::qcommon::q_shared::vec_t;
    let mut p2: *mut crate::src::qcommon::q_shared::vec_t =
        0 as *mut crate::src::qcommon::q_shared::vec_t;
    let mut d: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut edgedist: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut edgenormal: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut facenormal: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut area: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut facedist: crate::src::qcommon::q_shared::vec_t = 0.;
    if (*w).numpoints < 3 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"CheckWinding: %i points\x00" as *const u8 as *const libc::c_char,
            (*w).numpoints,
        );
    }
    area = WindingArea(w);
    if area < 1 as libc::c_int as libc::c_float {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"CheckWinding: %f area\x00" as *const u8 as *const libc::c_char,
            area as libc::c_double,
        );
    }
    WindingPlane(w, facenormal.as_mut_ptr(), &mut facedist);
    i = 0 as libc::c_int;
    while i < (*w).numpoints {
        p1 = (*(*w).p.as_mut_ptr().offset(i as isize)).as_mut_ptr();
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            if *p1.offset(j as isize) > 65535 as libc::c_int as libc::c_float
                || *p1.offset(j as isize) < -(65535 as libc::c_int) as libc::c_float
            {
                crate::src::qcommon::common::Com_Error(
                    crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                    b"CheckFace: BUGUS_RANGE: %f\x00" as *const u8 as *const libc::c_char,
                    *p1.offset(j as isize) as libc::c_double,
                );
            }
            j += 1
        }
        j = if i + 1 as libc::c_int == (*w).numpoints {
            0 as libc::c_int
        } else {
            (i) + 1 as libc::c_int
        };
        // check the point is on the face plane
        d = *p1.offset(0 as libc::c_int as isize) * facenormal[0 as libc::c_int as usize]
            + *p1.offset(1 as libc::c_int as isize) * facenormal[1 as libc::c_int as usize]
            + *p1.offset(2 as libc::c_int as isize) * facenormal[2 as libc::c_int as usize]
            - facedist;
        if d < -0.1f32 || d > 0.1f32 {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                b"CheckWinding: point off plane\x00" as *const u8 as *const libc::c_char,
            );
        }
        // check the edge isn't degenerate
        p2 = (*(*w).p.as_mut_ptr().offset(j as isize)).as_mut_ptr();
        dir[0 as libc::c_int as usize] =
            *p2.offset(0 as libc::c_int as isize) - *p1.offset(0 as libc::c_int as isize);
        dir[1 as libc::c_int as usize] =
            *p2.offset(1 as libc::c_int as isize) - *p1.offset(1 as libc::c_int as isize);
        dir[2 as libc::c_int as usize] =
            *p2.offset(2 as libc::c_int as isize) - *p1.offset(2 as libc::c_int as isize);
        if VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t) < 0.1f32 {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                b"CheckWinding: degenerate edge\x00" as *const u8 as *const libc::c_char,
            );
        }
        CrossProduct(
            facenormal.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            edgenormal.as_mut_ptr(),
        );
        crate::src::qcommon::q_math::VectorNormalize2(
            edgenormal.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            edgenormal.as_mut_ptr(),
        );
        edgedist = *p1.offset(0 as libc::c_int as isize) * edgenormal[0 as libc::c_int as usize]
            + *p1.offset(1 as libc::c_int as isize) * edgenormal[1 as libc::c_int as usize]
            + *p1.offset(2 as libc::c_int as isize) * edgenormal[2 as libc::c_int as usize];
        edgedist += 0.1f32;
        // all other points must be on front side
        j = 0 as libc::c_int;
        while j < (*w).numpoints {
            if !(j == i) {
                d = (*(*w).p.as_mut_ptr().offset(j as isize))[0 as libc::c_int as usize]
                    * edgenormal[0 as libc::c_int as usize]
                    + (*(*w).p.as_mut_ptr().offset(j as isize))[1 as libc::c_int as usize]
                        * edgenormal[1 as libc::c_int as usize]
                    + (*(*w).p.as_mut_ptr().offset(j as isize))[2 as libc::c_int as usize]
                        * edgenormal[2 as libc::c_int as usize];
                if d > edgedist {
                    crate::src::qcommon::common::Com_Error(
                        crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                        b"CheckWinding: non-convex\x00" as *const u8 as *const libc::c_char,
                    );
                }
            }
            j += 1
        }
        i += 1
    }
}
/*
============
WindingOnPlaneSide
============
*/
#[no_mangle]

pub unsafe extern "C" fn WindingOnPlaneSide(
    mut w: *mut crate::src::qcommon::cm_polylib::winding_t,
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
    mut dist: crate::src::qcommon::q_shared::vec_t,
) -> libc::c_int {
    let mut front: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut back: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut i: libc::c_int = 0;
    let mut d: crate::src::qcommon::q_shared::vec_t = 0.;
    front = crate::src::qcommon::q_shared::qfalse;
    back = crate::src::qcommon::q_shared::qfalse;
    i = 0 as libc::c_int;
    while i < (*w).numpoints {
        d = (*(*w).p.as_mut_ptr().offset(i as isize))[0 as libc::c_int as usize]
            * *normal.offset(0 as libc::c_int as isize)
            + (*(*w).p.as_mut_ptr().offset(i as isize))[1 as libc::c_int as usize]
                * *normal.offset(1 as libc::c_int as isize)
            + (*(*w).p.as_mut_ptr().offset(i as isize))[2 as libc::c_int as usize]
                * *normal.offset(2 as libc::c_int as isize)
            - dist;
        if d < -0.1f32 {
            if front as u64 != 0 {
                return 3 as libc::c_int;
            }
            back = crate::src::qcommon::q_shared::qtrue
        } else if d > 0.1f32 {
            if back as u64 != 0 {
                return 3 as libc::c_int;
            }
            front = crate::src::qcommon::q_shared::qtrue
        }
        i += 1
    }
    if back as u64 != 0 {
        return 1 as libc::c_int;
    }
    if front as u64 != 0 {
        return 0 as libc::c_int;
    }
    return 2 as libc::c_int;
}
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
// this is only used for visualization tools in cm_ debug functions
// variable sized
// you can define on_epsilon in the makefile as tighter
/*
=================
AddWindingToConvexHull

Both w and *hull are on the same plane
=================
*/
#[no_mangle]

pub unsafe extern "C" fn AddWindingToConvexHull(
    mut w: *mut crate::src::qcommon::cm_polylib::winding_t,
    mut hull: *mut *mut crate::src::qcommon::cm_polylib::winding_t,
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut p: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut copy: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut d: libc::c_float = 0.;
    let mut numHullPoints: libc::c_int = 0;
    let mut numNew: libc::c_int = 0;
    let mut hullPoints: [crate::src::qcommon::q_shared::vec3_t; 128] = [[0.; 3]; 128];
    let mut newHullPoints: [crate::src::qcommon::q_shared::vec3_t; 128] = [[0.; 3]; 128];
    let mut hullDirs: [crate::src::qcommon::q_shared::vec3_t; 128] = [[0.; 3]; 128];
    let mut hullSide: [crate::src::qcommon::q_shared::qboolean; 128] =
        [crate::src::qcommon::q_shared::qfalse; 128];
    let mut outside: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    if (*hull).is_null() {
        *hull = CopyWinding(w);
        return;
    }
    numHullPoints = (**hull).numpoints;
    crate::stdlib::memcpy(
        hullPoints.as_mut_ptr() as *mut libc::c_void,
        (**hull).p.as_mut_ptr() as *const libc::c_void,
        (numHullPoints as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            crate::src::qcommon::q_shared::vec3_t,
        >() as libc::c_ulong),
    );
    i = 0 as libc::c_int;
    while i < (*w).numpoints {
        p = (*(*w).p.as_mut_ptr().offset(i as isize)).as_mut_ptr();
        // calculate hull side vectors
        j = 0 as libc::c_int;
        while j < numHullPoints {
            k = (j + 1 as libc::c_int) % numHullPoints;
            dir[0 as libc::c_int as usize] = hullPoints[k as usize][0 as libc::c_int as usize]
                - hullPoints[j as usize][0 as libc::c_int as usize];
            dir[1 as libc::c_int as usize] = hullPoints[k as usize][1 as libc::c_int as usize]
                - hullPoints[j as usize][1 as libc::c_int as usize];
            dir[2 as libc::c_int as usize] = hullPoints[k as usize][2 as libc::c_int as usize]
                - hullPoints[j as usize][2 as libc::c_int as usize];
            crate::src::qcommon::q_math::VectorNormalize2(
                dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                dir.as_mut_ptr(),
            );
            CrossProduct(
                normal as *const crate::src::qcommon::q_shared::vec_t,
                dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                hullDirs[j as usize].as_mut_ptr(),
            );
            j += 1
        }
        outside = crate::src::qcommon::q_shared::qfalse;
        j = 0 as libc::c_int;
        while j < numHullPoints {
            dir[0 as libc::c_int as usize] = *p.offset(0 as libc::c_int as isize)
                - hullPoints[j as usize][0 as libc::c_int as usize];
            dir[1 as libc::c_int as usize] = *p.offset(1 as libc::c_int as isize)
                - hullPoints[j as usize][1 as libc::c_int as usize];
            dir[2 as libc::c_int as usize] = *p.offset(2 as libc::c_int as isize)
                - hullPoints[j as usize][2 as libc::c_int as usize];
            d = dir[0 as libc::c_int as usize] * hullDirs[j as usize][0 as libc::c_int as usize]
                + dir[1 as libc::c_int as usize] * hullDirs[j as usize][1 as libc::c_int as usize]
                + dir[2 as libc::c_int as usize] * hullDirs[j as usize][2 as libc::c_int as usize];
            if d >= 0.1f32 {
                outside = crate::src::qcommon::q_shared::qtrue
            }
            if d >= -0.1f32 {
                hullSide[j as usize] = crate::src::qcommon::q_shared::qtrue
            } else {
                hullSide[j as usize] = crate::src::qcommon::q_shared::qfalse
            }
            j += 1
        }
        // if the point is effectively inside, do nothing
        if !(outside as u64 == 0) {
            // find the back side to front side transition
            j = 0 as libc::c_int;
            while j < numHullPoints {
                if hullSide[(j % numHullPoints) as usize] as u64 == 0
                    && hullSide[((j + 1 as libc::c_int) % numHullPoints) as usize] as libc::c_uint
                        != 0
                {
                    break;
                }
                j += 1
            }
            if !(j == numHullPoints) {
                // insert the point here
                newHullPoints[0 as libc::c_int as usize][0 as libc::c_int as usize] =
                    *p.offset(0 as libc::c_int as isize);
                newHullPoints[0 as libc::c_int as usize][1 as libc::c_int as usize] =
                    *p.offset(1 as libc::c_int as isize);
                newHullPoints[0 as libc::c_int as usize][2 as libc::c_int as usize] =
                    *p.offset(2 as libc::c_int as isize);
                numNew = 1 as libc::c_int;
                // copy over all points that aren't double fronts
                j = (j + 1 as libc::c_int) % numHullPoints;
                k = 0 as libc::c_int;
                while k < numHullPoints {
                    if !(hullSide[((j + k) % numHullPoints) as usize] as libc::c_uint != 0
                        && hullSide[((j + k + 1 as libc::c_int) % numHullPoints) as usize]
                            as libc::c_uint
                            != 0)
                    {
                        copy = hullPoints[((j + k + 1 as libc::c_int) % numHullPoints) as usize]
                            .as_mut_ptr();
                        newHullPoints[numNew as usize][0 as libc::c_int as usize] =
                            *copy.offset(0 as libc::c_int as isize);
                        newHullPoints[numNew as usize][1 as libc::c_int as usize] =
                            *copy.offset(1 as libc::c_int as isize);
                        newHullPoints[numNew as usize][2 as libc::c_int as usize] =
                            *copy.offset(2 as libc::c_int as isize);
                        numNew += 1
                    }
                    k += 1
                }
                numHullPoints = numNew;
                crate::stdlib::memcpy(
                    hullPoints.as_mut_ptr() as *mut libc::c_void,
                    newHullPoints.as_mut_ptr() as *const libc::c_void,
                    (numHullPoints as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                        crate::src::qcommon::q_shared::vec3_t,
                    >()
                        as libc::c_ulong),
                );
            }
        }
        i += 1
    }
    FreeWinding(*hull);
    w = AllocWinding(numHullPoints);
    (*w).numpoints = numHullPoints;
    *hull = w;
    crate::stdlib::memcpy(
        (*w).p.as_mut_ptr() as *mut libc::c_void,
        hullPoints.as_mut_ptr() as *const libc::c_void,
        (numHullPoints as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            crate::src::qcommon::q_shared::vec3_t,
        >() as libc::c_ulong),
    );
}
