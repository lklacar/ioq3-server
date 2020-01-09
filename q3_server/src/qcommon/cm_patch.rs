// =============== BEGIN cm_patch_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct patchCollide_s {
    pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub numPlanes: libc::c_int,
    pub planes: *mut crate::src::qcommon::cm_patch::patchPlane_t,
    pub numFacets: libc::c_int,
    pub facets: *mut crate::src::qcommon::cm_patch::facet_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct facet_t {
    pub surfacePlane: libc::c_int,
    pub numBorders: libc::c_int,
    pub borderPlanes: [libc::c_int; 26],
    pub borderInward: [libc::c_int; 26],
    pub borderNoAdjust: [crate::src::qcommon::q_shared::qboolean; 26],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct patchPlane_t {
    pub plane: [libc::c_float; 4],
    pub signbits: libc::c_int,
}

pub type patchCollide_t = crate::src::qcommon::cm_patch::patchCollide_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cGrid_t {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub wrapWidth: crate::src::qcommon::q_shared::qboolean,
    pub wrapHeight: crate::src::qcommon::q_shared::qboolean,
    pub points: [[crate::src::qcommon::q_shared::vec3_t; 129]; 129],
}
use ::libc;

pub mod q_shared_h {
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
    //
    // q_shared.h -- included first by ALL program modules.
    // A user mod should never modify this file
    // Heartbeat for dpmaster protocol. You shouldn't change this unless you know what you're doing
    // When com_gamename is LEGACY_MASTER_GAMENAME, use quake3 master protocol.
    // You shouldn't change this unless you know what you're doing
    // number of supported master servers
    // standard demo extension
    //Ignore __attribute__ on non-gcc platforms
    /* *********************************************************************
     VM Considerations

     The VM can not use the standard system headers because we aren't really
     using the compiler they were meant for.  We use bg_lib.h which contains
     prototypes for the functions we define for our own use in bg_lib.c.

     When writing mods, please add needed headers HERE, do not start including
     stuff like <stdio.h> in the various .c files that make up each of the VMs
     since you will be including system headers files can will have issues.

     Remember, if you use a C library function that is not defined in bg_lib.c,
     you will have to add your own version for support in the VM.

    **********************************************************************/
    //=============================================================

    // parameters to the main Error routine

    // pop up the need-cd dialog
    // client disconnected from the server

    // don't kill server

    // print to console and disconnect from game

    // exit the entire game with a popup window

    // font rendering values used by ui and cgame
    // default
    // default

    /*
    ==============================================================

    MATHLIB

    ==============================================================
    */

    /*
    // if your system does not have lrintf() and round() you can try this block. Please also open a bug report at bugzilla.icculus.org
    // or write a mail to the ioq3 mailing list.
    #else
      #define Q_ftol(v) ((long) (v))
      #define Q_round(v) do { if((v) < 0) (v) -= 0.5f; else (v) += 0.5f; (v) = Q_ftol((v)); } while(0)
      #define Q_SnapVector(vec) \
        do\
        {\
            vec3_t *temp = (vec);\
            \
            Q_round((*temp)[0]);\
            Q_round((*temp)[1]);\
            Q_round((*temp)[2]);\
        } while(0)
    #endif
    */
    // reciprocal square root
    // this isn't a real cheap function to call!
    // just in case you don't want to use the macros
    // fast vector normalize routine that does not check to make sure
    // that length != 0, nor does it return length, uses rsqrt approximation
    // returns vector length
    // perpendicular vector could be replaced by this
    //int	PlaneTypeForNormal (vec3_t normal);
    //=============================================
    //int		COM_ParseInfos( char *buf, int max, char infos[][MAX_INFO_STRING] );
    //token types
    // string
    // literal
    // number
    // name
    // punctuation
    // data is an in/out parm, returns a parsed out token
    // mode parm for FS_FOpenFile
    //=============================================
    // portable case insensitive compare
    // buffer size safe library replacements
    // strlen that discounts Quake color sequences
    // removes color sequences from string
    // Count the number of char tocount encountered in string
    //=============================================
    // 64-bit integers for global rankings interface
    // implemented as a struct for qvm compatibility
    //=============================================
    /*
    short	BigShort(short l);
    short	LittleShort(short l);
    int		BigLong (int l);
    int		LittleLong (int l);
    qint64  BigLong64 (qint64 l);
    qint64  LittleLong64 (qint64 l);
    float	BigFloat (const float *l);
    float	LittleFloat (const float *l);

    void	Swap_Init (void);
    */
    //=============================================
    //
    // key / value info strings
    //
    // this is only here so the functions in q_shared.c and bg_*.c can link
    /*
    ==========================================================

    CVARS (console variables)

    Many variables can be used for cheating purposes, so when
    cheats is zero, force all unspecified variables to their
    default values.
    ==========================================================
    */
    // set to cause it to be saved to vars.rc
    // used for system variables, not for player
    // specific configurations
    // sent to server on connect or change
    // sent in response to front end requests
    // these cvars will be duplicated on all clients
    // don't allow change from console at all,
    // but can be set from the command line
    // will only change when C code next does
    // a Cvar_Get(), so it can't be changed
    // without proper initialization.  modified
    // will be set, even though the value hasn't
    // changed yet
    // display only, cannot be set by user at all
    // created by a set command
    // can be set even when cheats are disabled, but is not archived
    // can not be changed if cheats are disabled
    // do not clear when a cvar_restart is issued
    // cvar was created by a server the client connected to.
    // cvar was created exclusively in one of the VMs.
    // prevent modifying this var from VMs or the server
    // These flags are only returned by the Cvar_Flags() function
    // Cvar was modified
    // Cvar doesn't exist.
    // nothing outside the Cvar_*() functions should modify these fields!
    // cvar_restart will reset to this value
    // for CVAR_LATCH vars
    // set each time the cvar is changed
    // incremented each time the cvar is changed
    // atof( string )
    // atoi( string )
    // the modules that run in the virtual machine can't access the cvar_t directly,
    // so they must ask for structured updates
    /*
    ==============================================================

    VoIP

    ==============================================================
    */
    // if you change the count of flags be sure to also change VOIP_FLAGNUM
    // spatialized voip message
    // non-spatialized voip message
    // number of flags voip knows. You will have to bump protocol version number if you
    // change this.
    /*
    ==============================================================

    COLLISION DETECTION

    ==============================================================
    */
    // plane types are used to speed some tests
    // 0-2 are axial planes
    /*
    =================
    PlaneTypeForNormal
    =================
    */
    // plane_t structure
    // !!! if this is changed, it must be changed in asm code too !!!

    // a trace is returned when a box is swept through the world

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

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::cm_local_h::sphere_t;
pub use crate::cm_local_h::traceWork_t;
pub use crate::src::qcommon::cm_load::cm_playerCurveClip;
pub use crate::src::qcommon::cm_patch::q_shared_h::CrossProduct;
pub use crate::src::qcommon::cm_patch::q_shared_h::VectorLength;
pub use crate::src::qcommon::cm_polylib::winding_t;
pub use crate::src::qcommon::cm_polylib::BaseWindingForPlane;
pub use crate::src::qcommon::cm_polylib::ChopWindingInPlace;
pub use crate::src::qcommon::cm_polylib::CopyWinding;
pub use crate::src::qcommon::cm_polylib::FreeWinding;
pub use crate::src::qcommon::cm_polylib::WindingBounds;
pub use crate::src::qcommon::cm_test::CM_BoundsIntersect;
use crate::src::qcommon::common::Com_DPrintf;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::Hunk_AllocDebug;
use crate::src::qcommon::cvar::Cvar_Get;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::AddPointToBounds;
pub use crate::src::qcommon::q_math::ClearBounds;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::h_dontcare;
pub use crate::src::qcommon::q_shared::h_high;
pub use crate::src::qcommon::q_shared::h_low;
pub use crate::src::qcommon::q_shared::ha_pref;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trace_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
use crate::stdlib::fabs;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
use crate::stdlib::sqrt;
extern "C" {
    /*
    =======================================================================

    DEBUGGING

    =======================================================================
    */
    /*
    ==================
    CM_DrawDebugSurface

    Called from the renderer
    ==================
    */
    #[no_mangle]
    pub fn BotDrawDebugPolygons(
        drawPoly: Option<
            unsafe extern "C" fn(_: libc::c_int, _: libc::c_int, _: *mut libc::c_float) -> (),
        >,
        value: libc::c_int,
    );
}

pub const EN_TOP: C2RustUnnamed_9 = 0;

pub const EN_LEFT: C2RustUnnamed_9 = 3;

pub const EN_BOTTOM: C2RustUnnamed_9 = 2;

pub const EN_RIGHT: C2RustUnnamed_9 = 1;

pub type C2RustUnnamed_9 = libc::c_uint;
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
/*

This file does not reference any globals, and has these entry points:

void CM_ClearLevelPatches( void );
struct patchCollide_s	*CM_GeneratePatchCollide( int width, int height, const vec3_t *points );
void CM_TraceThroughPatchCollide( traceWork_t *tw, const struct patchCollide_s *pc );
qboolean CM_PositionTestInPatchCollide( traceWork_t *tw, const struct patchCollide_s *pc );
void CM_DrawDebugSurface( void (*drawPoly)(int color, int numPoints, flaot *points) );


WARNING: this may misbehave with meshes that have rows or columns that only
degenerate a few triangles.  Completely degenerate rows and columns are handled
properly.
*/
/*
#define	MAX_FACETS			1024
#define	MAX_PATCH_PLANES	2048

typedef struct {
    float	plane[4];
    int		signbits;		// signx + (signy<<1) + (signz<<2), used as lookup during collision
} patchPlane_t;

typedef struct {
    int			surfacePlane;
    int			numBorders;		// 3 or four + 6 axial bevels + 4 or 3 * 4 edge bevels
    int			borderPlanes[4+6+16];
    int			borderInward[4+6+16];
    qboolean	borderNoAdjust[4+6+16];
} facet_t;

typedef struct patchCollide_s {
    vec3_t	bounds[2];
    int		numPlanes;			// surface planes plus edge planes
    patchPlane_t	*planes;
    int		numFacets;
    facet_t	*facets;
} patchCollide_t;


#define	MAX_GRID_SIZE	129

typedef struct {
    int			width;
    int			height;
    qboolean	wrapWidth;
    qboolean	wrapHeight;
    vec3_t	points[MAX_GRID_SIZE][MAX_GRID_SIZE];	// [width][height]
} cGrid_t;

#define	SUBDIVIDE_DISTANCE	16	//4	// never more than this units away from curve
#define	PLANE_TRI_EPSILON	0.1
#define	WRAP_POINT_EPSILON	0.1
*/
#[no_mangle]

pub static mut c_totalPatchBlocks: libc::c_int = 0;
#[no_mangle]

pub static mut c_totalPatchSurfaces: libc::c_int = 0;
#[no_mangle]

pub static mut c_totalPatchEdges: libc::c_int = 0;

static mut debugPatchCollide: *const crate::src::qcommon::cm_patch::patchCollide_t =
    0 as *const crate::src::qcommon::cm_patch::patchCollide_t;

static mut debugFacet: *const crate::src::qcommon::cm_patch::facet_t =
    0 as *const crate::src::qcommon::cm_patch::facet_t;

static mut debugBlock: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;

static mut debugBlockPoints: [crate::src::qcommon::q_shared::vec3_t; 4] = [[0.; 3]; 4];
/*
=================
CM_ClearLevelPatches
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_ClearLevelPatches() {
    debugPatchCollide = 0 as *const crate::src::qcommon::cm_patch::patchCollide_t;
    debugFacet = 0 as *const crate::src::qcommon::cm_patch::facet_t;
}
/*
=================
CM_SignbitsForNormal
=================
*/

unsafe extern "C" fn CM_SignbitsForNormal(
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    bits = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        if *normal.offset(j as isize) < 0 as libc::c_int as libc::c_float {
            bits |= (1 as libc::c_int) << j
        }
        j += 1
    }
    return bits;
}
/*
=====================
CM_PlaneFromPoints

Returns false if the triangle is degenrate.
The normal will point out of the clock for clockwise ordered points
=====================
*/

unsafe extern "C" fn CM_PlaneFromPoints(
    mut plane: *mut crate::src::qcommon::q_shared::vec_t,
    mut a: *mut crate::src::qcommon::q_shared::vec_t,
    mut b: *mut crate::src::qcommon::q_shared::vec_t,
    mut c: *mut crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut d1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut d2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    d1[0 as libc::c_int as usize] =
        *b.offset(0 as libc::c_int as isize) - *a.offset(0 as libc::c_int as isize);
    d1[1 as libc::c_int as usize] =
        *b.offset(1 as libc::c_int as isize) - *a.offset(1 as libc::c_int as isize);
    d1[2 as libc::c_int as usize] =
        *b.offset(2 as libc::c_int as isize) - *a.offset(2 as libc::c_int as isize);
    d2[0 as libc::c_int as usize] =
        *c.offset(0 as libc::c_int as isize) - *a.offset(0 as libc::c_int as isize);
    d2[1 as libc::c_int as usize] =
        *c.offset(1 as libc::c_int as isize) - *a.offset(1 as libc::c_int as isize);
    d2[2 as libc::c_int as usize] =
        *c.offset(2 as libc::c_int as isize) - *a.offset(2 as libc::c_int as isize);
    CrossProduct(
        d2.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        d1.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        plane,
    );
    if crate::src::qcommon::q_math::VectorNormalize(plane) == 0 as libc::c_int as libc::c_float {
        return crate::src::qcommon::q_shared::qfalse;
    }
    *plane.offset(3 as libc::c_int as isize) = *a.offset(0 as libc::c_int as isize)
        * *plane.offset(0 as libc::c_int as isize)
        + *a.offset(1 as libc::c_int as isize) * *plane.offset(1 as libc::c_int as isize)
        + *a.offset(2 as libc::c_int as isize) * *plane.offset(2 as libc::c_int as isize);
    return crate::src::qcommon::q_shared::qtrue;
}
/*
================================================================================

GRID SUBDIVISION

================================================================================
*/
/*
=================
CM_NeedsSubdivision

Returns true if the given quadratic curve is not flat enough for our
collision detection purposes
=================
*/

unsafe extern "C" fn CM_NeedsSubdivision(
    mut a: *mut crate::src::qcommon::q_shared::vec_t,
    mut b: *mut crate::src::qcommon::q_shared::vec_t,
    mut c: *mut crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut cmid: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut lmid: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut delta: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    // calculate the linear midpoint
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        lmid[i as usize] = (0.5f64
            * (*a.offset(i as isize) + *c.offset(i as isize)) as libc::c_double)
            as crate::src::qcommon::q_shared::vec_t;
        i += 1
    }
    // calculate the exact curve midpoint
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        cmid[i as usize] = (0.5f64
            * (0.5f64 * (*a.offset(i as isize) + *b.offset(i as isize)) as libc::c_double
                + 0.5f64 * (*b.offset(i as isize) + *c.offset(i as isize)) as libc::c_double))
            as crate::src::qcommon::q_shared::vec_t;
        i += 1
    }
    // see if the curve is far enough away from the linear mid
    delta[0 as libc::c_int as usize] =
        cmid[0 as libc::c_int as usize] - lmid[0 as libc::c_int as usize];
    delta[1 as libc::c_int as usize] =
        cmid[1 as libc::c_int as usize] - lmid[1 as libc::c_int as usize];
    delta[2 as libc::c_int as usize] =
        cmid[2 as libc::c_int as usize] - lmid[2 as libc::c_int as usize];
    dist = VectorLength(delta.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    return (dist >= 16 as libc::c_int as libc::c_float) as libc::c_int
        as crate::src::qcommon::q_shared::qboolean;
}
/*
===============
CM_Subdivide

a, b, and c are control points.
the subdivided sequence will be: a, out1, out2, out3, c
===============
*/

unsafe extern "C" fn CM_Subdivide(
    mut a: *mut crate::src::qcommon::q_shared::vec_t,
    mut b: *mut crate::src::qcommon::q_shared::vec_t,
    mut c: *mut crate::src::qcommon::q_shared::vec_t,
    mut out1: *mut crate::src::qcommon::q_shared::vec_t,
    mut out2: *mut crate::src::qcommon::q_shared::vec_t,
    mut out3: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        *out1.offset(i as isize) = (0.5f64
            * (*a.offset(i as isize) + *b.offset(i as isize)) as libc::c_double)
            as crate::src::qcommon::q_shared::vec_t;
        *out3.offset(i as isize) = (0.5f64
            * (*b.offset(i as isize) + *c.offset(i as isize)) as libc::c_double)
            as crate::src::qcommon::q_shared::vec_t;
        *out2.offset(i as isize) = (0.5f64
            * (*out1.offset(i as isize) + *out3.offset(i as isize)) as libc::c_double)
            as crate::src::qcommon::q_shared::vec_t;
        i += 1
    }
}
/*
=================
CM_TransposeGrid

Swaps the rows and columns in place
=================
*/

unsafe extern "C" fn CM_TransposeGrid(mut grid: *mut crate::src::qcommon::cm_patch::cGrid_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut temp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut tempWrap: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    if (*grid).width > (*grid).height {
        i = 0 as libc::c_int;
        while i < (*grid).height {
            j = i + 1 as libc::c_int;
            while j < (*grid).width {
                if j < (*grid).height {
                    // swap the value
                    temp[0 as libc::c_int as usize] =
                        (*grid).points[i as usize][j as usize][0 as libc::c_int as usize];
                    temp[1 as libc::c_int as usize] =
                        (*grid).points[i as usize][j as usize][1 as libc::c_int as usize];
                    temp[2 as libc::c_int as usize] =
                        (*grid).points[i as usize][j as usize][2 as libc::c_int as usize];
                    (*grid).points[i as usize][j as usize][0 as libc::c_int as usize] =
                        (*grid).points[j as usize][i as usize][0 as libc::c_int as usize];
                    (*grid).points[i as usize][j as usize][1 as libc::c_int as usize] =
                        (*grid).points[j as usize][i as usize][1 as libc::c_int as usize];
                    (*grid).points[i as usize][j as usize][2 as libc::c_int as usize] =
                        (*grid).points[j as usize][i as usize][2 as libc::c_int as usize];
                    (*grid).points[j as usize][i as usize][0 as libc::c_int as usize] =
                        temp[0 as libc::c_int as usize];
                    (*grid).points[j as usize][i as usize][1 as libc::c_int as usize] =
                        temp[1 as libc::c_int as usize];
                    (*grid).points[j as usize][i as usize][2 as libc::c_int as usize] =
                        temp[2 as libc::c_int as usize]
                } else {
                    // just copy
                    (*grid).points[i as usize][j as usize][0 as libc::c_int as usize] =
                        (*grid).points[j as usize][i as usize][0 as libc::c_int as usize];
                    (*grid).points[i as usize][j as usize][1 as libc::c_int as usize] =
                        (*grid).points[j as usize][i as usize][1 as libc::c_int as usize];
                    (*grid).points[i as usize][j as usize][2 as libc::c_int as usize] =
                        (*grid).points[j as usize][i as usize][2 as libc::c_int as usize]
                }
                j += 1
            }
            i += 1
        }
    } else {
        i = 0 as libc::c_int;
        while i < (*grid).width {
            j = i + 1 as libc::c_int;
            while j < (*grid).height {
                if j < (*grid).width {
                    // swap the value
                    temp[0 as libc::c_int as usize] =
                        (*grid).points[j as usize][i as usize][0 as libc::c_int as usize];
                    temp[1 as libc::c_int as usize] =
                        (*grid).points[j as usize][i as usize][1 as libc::c_int as usize];
                    temp[2 as libc::c_int as usize] =
                        (*grid).points[j as usize][i as usize][2 as libc::c_int as usize];
                    (*grid).points[j as usize][i as usize][0 as libc::c_int as usize] =
                        (*grid).points[i as usize][j as usize][0 as libc::c_int as usize];
                    (*grid).points[j as usize][i as usize][1 as libc::c_int as usize] =
                        (*grid).points[i as usize][j as usize][1 as libc::c_int as usize];
                    (*grid).points[j as usize][i as usize][2 as libc::c_int as usize] =
                        (*grid).points[i as usize][j as usize][2 as libc::c_int as usize];
                    (*grid).points[i as usize][j as usize][0 as libc::c_int as usize] =
                        temp[0 as libc::c_int as usize];
                    (*grid).points[i as usize][j as usize][1 as libc::c_int as usize] =
                        temp[1 as libc::c_int as usize];
                    (*grid).points[i as usize][j as usize][2 as libc::c_int as usize] =
                        temp[2 as libc::c_int as usize]
                } else {
                    // just copy
                    (*grid).points[j as usize][i as usize][0 as libc::c_int as usize] =
                        (*grid).points[i as usize][j as usize][0 as libc::c_int as usize];
                    (*grid).points[j as usize][i as usize][1 as libc::c_int as usize] =
                        (*grid).points[i as usize][j as usize][1 as libc::c_int as usize];
                    (*grid).points[j as usize][i as usize][2 as libc::c_int as usize] =
                        (*grid).points[i as usize][j as usize][2 as libc::c_int as usize]
                }
                j += 1
            }
            i += 1
        }
    }
    l = (*grid).width;
    (*grid).width = (*grid).height;
    (*grid).height = l;
    tempWrap = (*grid).wrapWidth;
    (*grid).wrapWidth = (*grid).wrapHeight;
    (*grid).wrapHeight = tempWrap;
}
/*
===================
CM_SetGridWrapWidth

If the left and right columns are exactly equal, set grid->wrapWidth qtrue
===================
*/

unsafe extern "C" fn CM_SetGridWrapWidth(mut grid: *mut crate::src::qcommon::cm_patch::cGrid_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut d: libc::c_float = 0.;
    i = 0 as libc::c_int;
    while i < (*grid).height {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            d = (*grid).points[0 as libc::c_int as usize][i as usize][j as usize]
                - (*grid).points[((*grid).width - 1 as libc::c_int) as usize][i as usize]
                    [j as usize];
            if (d as libc::c_double) < -0.1f64 || d as libc::c_double > 0.1f64 {
                break;
            }
            j += 1
        }
        if j != 3 as libc::c_int {
            break;
        }
        i += 1
    }
    if i == (*grid).height {
        (*grid).wrapWidth = crate::src::qcommon::q_shared::qtrue
    } else {
        (*grid).wrapWidth = crate::src::qcommon::q_shared::qfalse
    };
}
/*
=================
CM_SubdivideGridColumns

Adds columns as necessary to the grid until
all the aproximating points are within SUBDIVIDE_DISTANCE
from the true curve
=================
*/

unsafe extern "C" fn CM_SubdivideGridColumns(
    mut grid: *mut crate::src::qcommon::cm_patch::cGrid_t,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*grid).width - 2 as libc::c_int {
        // grid->points[i][x] is an interpolating control point
        // grid->points[i+1][x] is an aproximating control point
        // grid->points[i+2][x] is an interpolating control point
        //
        // first see if we can collapse the aproximating collumn away
        //
        j = 0 as libc::c_int;
        while j < (*grid).height {
            if CM_NeedsSubdivision(
                (*grid).points[i as usize][j as usize].as_mut_ptr(),
                (*grid).points[(i + 1 as libc::c_int) as usize][j as usize].as_mut_ptr(),
                (*grid).points[(i + 2 as libc::c_int) as usize][j as usize].as_mut_ptr(),
            ) as u64
                != 0
            {
                break;
            }
            j += 1
        }
        if j == (*grid).height {
            // the new aproximating point at i+1 may need to be removed
            // or subdivided farther, so don't advance i
            // all of the points were close enough to the linear midpoints
            // that we can collapse the entire column away
            j = 0 as libc::c_int;
            while j < (*grid).height {
                // remove the column
                k = i + 2 as libc::c_int;
                while k < (*grid).width {
                    (*grid).points[(k - 1 as libc::c_int) as usize][j as usize]
                        [0 as libc::c_int as usize] =
                        (*grid).points[k as usize][j as usize][0 as libc::c_int as usize];
                    (*grid).points[(k - 1 as libc::c_int) as usize][j as usize]
                        [1 as libc::c_int as usize] =
                        (*grid).points[k as usize][j as usize][1 as libc::c_int as usize];
                    (*grid).points[(k - 1 as libc::c_int) as usize][j as usize]
                        [2 as libc::c_int as usize] =
                        (*grid).points[k as usize][j as usize][2 as libc::c_int as usize];
                    k += 1
                }
                j += 1
            }
            (*grid).width -= 1;
            // go to the next curve segment
            i += 1
        } else {
            //
            // we need to subdivide the curve
            //
            j = 0 as libc::c_int;
            while j < (*grid).height {
                let mut prev: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
                let mut mid: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
                let mut next: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
                // save the control points now
                prev[0 as libc::c_int as usize] =
                    (*grid).points[i as usize][j as usize][0 as libc::c_int as usize];
                prev[1 as libc::c_int as usize] =
                    (*grid).points[i as usize][j as usize][1 as libc::c_int as usize];
                prev[2 as libc::c_int as usize] =
                    (*grid).points[i as usize][j as usize][2 as libc::c_int as usize];
                mid[0 as libc::c_int as usize] = (*grid).points[(i + 1 as libc::c_int) as usize]
                    [j as usize][0 as libc::c_int as usize];
                mid[1 as libc::c_int as usize] = (*grid).points[(i + 1 as libc::c_int) as usize]
                    [j as usize][1 as libc::c_int as usize];
                mid[2 as libc::c_int as usize] = (*grid).points[(i + 1 as libc::c_int) as usize]
                    [j as usize][2 as libc::c_int as usize];
                next[0 as libc::c_int as usize] = (*grid).points[(i + 2 as libc::c_int) as usize]
                    [j as usize][0 as libc::c_int as usize];
                next[1 as libc::c_int as usize] = (*grid).points[(i + 2 as libc::c_int) as usize]
                    [j as usize][1 as libc::c_int as usize];
                next[2 as libc::c_int as usize] = (*grid).points[(i + 2 as libc::c_int) as usize]
                    [j as usize][2 as libc::c_int as usize];
                // make room for two additional columns in the grid
                // columns i+1 will be replaced, column i+2 will become i+4
                // i+1, i+2, and i+3 will be generated
                k = (*grid).width - 1 as libc::c_int;
                while k > i + 1 as libc::c_int {
                    (*grid).points[(k + 2 as libc::c_int) as usize][j as usize]
                        [0 as libc::c_int as usize] =
                        (*grid).points[k as usize][j as usize][0 as libc::c_int as usize];
                    (*grid).points[(k + 2 as libc::c_int) as usize][j as usize]
                        [1 as libc::c_int as usize] =
                        (*grid).points[k as usize][j as usize][1 as libc::c_int as usize];
                    (*grid).points[(k + 2 as libc::c_int) as usize][j as usize]
                        [2 as libc::c_int as usize] =
                        (*grid).points[k as usize][j as usize][2 as libc::c_int as usize];
                    k -= 1
                }
                // generate the subdivided points
                CM_Subdivide(
                    prev.as_mut_ptr(),
                    mid.as_mut_ptr(),
                    next.as_mut_ptr(),
                    (*grid).points[(i + 1 as libc::c_int) as usize][j as usize].as_mut_ptr(),
                    (*grid).points[(i + 2 as libc::c_int) as usize][j as usize].as_mut_ptr(),
                    (*grid).points[(i + 3 as libc::c_int) as usize][j as usize].as_mut_ptr(),
                );
                j += 1
            }
            (*grid).width += 2 as libc::c_int
        }
    }
}

unsafe extern "C" fn CM_ComparePoints(
    mut a: *mut libc::c_float,
    mut b: *mut libc::c_float,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut d: libc::c_float = 0.;
    d = *a.offset(0 as libc::c_int as isize) - *b.offset(0 as libc::c_int as isize);
    if (d as libc::c_double) < -0.1f64 || d as libc::c_double > 0.1f64 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    d = *a.offset(1 as libc::c_int as isize) - *b.offset(1 as libc::c_int as isize);
    if (d as libc::c_double) < -0.1f64 || d as libc::c_double > 0.1f64 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    d = *a.offset(2 as libc::c_int as isize) - *b.offset(2 as libc::c_int as isize);
    if (d as libc::c_double) < -0.1f64 || d as libc::c_double > 0.1f64 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
=================
CM_RemoveDegenerateColumns

If there are any identical columns, remove them
=================
*/

unsafe extern "C" fn CM_RemoveDegenerateColumns(
    mut grid: *mut crate::src::qcommon::cm_patch::cGrid_t,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*grid).width - 1 as libc::c_int {
        j = 0 as libc::c_int;
        while j < (*grid).height {
            if CM_ComparePoints(
                (*grid).points[i as usize][j as usize].as_mut_ptr(),
                (*grid).points[(i + 1 as libc::c_int) as usize][j as usize].as_mut_ptr(),
            ) as u64
                == 0
            {
                break;
            }
            j += 1
        }
        if !(j != (*grid).height) {
            j = 0 as libc::c_int;
            while j < (*grid).height {
                // remove the column
                k = i + 2 as libc::c_int;
                while k < (*grid).width {
                    (*grid).points[(k - 1 as libc::c_int) as usize][j as usize]
                        [0 as libc::c_int as usize] =
                        (*grid).points[k as usize][j as usize][0 as libc::c_int as usize];
                    (*grid).points[(k - 1 as libc::c_int) as usize][j as usize]
                        [1 as libc::c_int as usize] =
                        (*grid).points[k as usize][j as usize][1 as libc::c_int as usize];
                    (*grid).points[(k - 1 as libc::c_int) as usize][j as usize]
                        [2 as libc::c_int as usize] =
                        (*grid).points[k as usize][j as usize][2 as libc::c_int as usize];
                    k += 1
                }
                j += 1
            }
            (*grid).width -= 1;
            // check against the next column
            i -= 1
        }
        i += 1
        // not degenerate
    }
}
/*
================================================================================

PATCH COLLIDE GENERATION

================================================================================
*/

static mut numPlanes: libc::c_int = 0;

static mut planes: [crate::src::qcommon::cm_patch::patchPlane_t; 2048] =
    [crate::src::qcommon::cm_patch::patchPlane_t {
        plane: [0.; 4],
        signbits: 0,
    }; 2048];

static mut numFacets: libc::c_int = 0;

static mut facets: [crate::src::qcommon::cm_patch::facet_t; 1024] =
    [crate::src::qcommon::cm_patch::facet_t {
        surfacePlane: 0,
        numBorders: 0,
        borderPlanes: [0; 26],
        borderInward: [0; 26],
        borderNoAdjust: [crate::src::qcommon::q_shared::qfalse; 26],
    }; 1024];
/*
==================
CM_PlaneEqual
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_PlaneEqual(
    mut p: *mut crate::src::qcommon::cm_patch::patchPlane_t,
    mut plane: *mut libc::c_float,
    mut flipped: *mut libc::c_int,
) -> libc::c_int {
    let mut invplane: [libc::c_float; 4] = [0.; 4];
    if crate::stdlib::fabs(
        ((*p).plane[0 as libc::c_int as usize] - *plane.offset(0 as libc::c_int as isize))
            as libc::c_double,
    ) < 0.0001f64
        && crate::stdlib::fabs(
            ((*p).plane[1 as libc::c_int as usize] - *plane.offset(1 as libc::c_int as isize))
                as libc::c_double,
        ) < 0.0001f64
        && crate::stdlib::fabs(
            ((*p).plane[2 as libc::c_int as usize] - *plane.offset(2 as libc::c_int as isize))
                as libc::c_double,
        ) < 0.0001f64
        && crate::stdlib::fabs(
            ((*p).plane[3 as libc::c_int as usize] - *plane.offset(3 as libc::c_int as isize))
                as libc::c_double,
        ) < 0.02f64
    {
        *flipped = crate::src::qcommon::q_shared::qfalse as libc::c_int;
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    }
    invplane[0 as libc::c_int as usize] = -*plane.offset(0 as libc::c_int as isize);
    invplane[1 as libc::c_int as usize] = -*plane.offset(1 as libc::c_int as isize);
    invplane[2 as libc::c_int as usize] = -*plane.offset(2 as libc::c_int as isize);
    invplane[3 as libc::c_int as usize] = -*plane.offset(3 as libc::c_int as isize);
    if crate::stdlib::fabs(
        ((*p).plane[0 as libc::c_int as usize] - invplane[0 as libc::c_int as usize])
            as libc::c_double,
    ) < 0.0001f64
        && crate::stdlib::fabs(
            ((*p).plane[1 as libc::c_int as usize] - invplane[1 as libc::c_int as usize])
                as libc::c_double,
        ) < 0.0001f64
        && crate::stdlib::fabs(
            ((*p).plane[2 as libc::c_int as usize] - invplane[2 as libc::c_int as usize])
                as libc::c_double,
        ) < 0.0001f64
        && crate::stdlib::fabs(
            ((*p).plane[3 as libc::c_int as usize] - invplane[3 as libc::c_int as usize])
                as libc::c_double,
        ) < 0.02f64
    {
        *flipped = crate::src::qcommon::q_shared::qtrue as libc::c_int;
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    }
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
/*
==================
CM_SnapVector
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_SnapVector(mut normal: *mut crate::src::qcommon::q_shared::vec_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if crate::stdlib::fabs(
            (*normal.offset(i as isize) - 1 as libc::c_int as libc::c_float) as libc::c_double,
        ) < 0.0001f64
        {
            let ref mut fresh0 = *normal.offset(2 as libc::c_int as isize);
            *fresh0 = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
            let ref mut fresh1 = *normal.offset(1 as libc::c_int as isize);
            *fresh1 = *fresh0;
            *normal.offset(0 as libc::c_int as isize) = *fresh1;
            *normal.offset(i as isize) = 1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
            break;
        } else if crate::stdlib::fabs(
            (*normal.offset(i as isize) - -(1 as libc::c_int) as libc::c_float) as libc::c_double,
        ) < 0.0001f64
        {
            let ref mut fresh2 = *normal.offset(2 as libc::c_int as isize);
            *fresh2 = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
            let ref mut fresh3 = *normal.offset(1 as libc::c_int as isize);
            *fresh3 = *fresh2;
            *normal.offset(0 as libc::c_int as isize) = *fresh3;
            *normal.offset(i as isize) =
                -(1 as libc::c_int) as crate::src::qcommon::q_shared::vec_t;
            break;
        } else {
            i += 1
        }
    }
}
/*
==================
CM_FindPlane2
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_FindPlane2(
    mut plane: *mut libc::c_float,
    mut flipped: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    // see if the points are close enough to an existing plane
    i = 0 as libc::c_int;
    while i < numPlanes {
        if CM_PlaneEqual(&mut *planes.as_mut_ptr().offset(i as isize), plane, flipped) != 0 {
            return i;
        }
        i += 1
    }
    // add a new plane
    if numPlanes == 2048 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"MAX_PATCH_PLANES\x00" as *const u8 as *const libc::c_char,
        );
    }
    planes[numPlanes as usize].plane[0 as libc::c_int as usize] =
        *plane.offset(0 as libc::c_int as isize);
    planes[numPlanes as usize].plane[1 as libc::c_int as usize] =
        *plane.offset(1 as libc::c_int as isize);
    planes[numPlanes as usize].plane[2 as libc::c_int as usize] =
        *plane.offset(2 as libc::c_int as isize);
    planes[numPlanes as usize].plane[3 as libc::c_int as usize] =
        *plane.offset(3 as libc::c_int as isize);
    planes[numPlanes as usize].signbits = CM_SignbitsForNormal(plane);
    numPlanes += 1;
    *flipped = crate::src::qcommon::q_shared::qfalse as libc::c_int;
    return numPlanes - 1 as libc::c_int;
}
/*
==================
CM_FindPlane
==================
*/

unsafe extern "C" fn CM_FindPlane(
    mut p1: *mut libc::c_float,
    mut p2: *mut libc::c_float,
    mut p3: *mut libc::c_float,
) -> libc::c_int {
    let mut plane: [libc::c_float; 4] = [0.; 4];
    let mut i: libc::c_int = 0;
    let mut d: libc::c_float = 0.;
    if CM_PlaneFromPoints(plane.as_mut_ptr(), p1, p2, p3) as u64 == 0 {
        return -(1 as libc::c_int);
    }
    // see if the points are close enough to an existing plane
    i = 0 as libc::c_int;
    while i < numPlanes {
        if !(plane[0 as libc::c_int as usize] * planes[i as usize].plane[0 as libc::c_int as usize]
            + plane[1 as libc::c_int as usize]
                * planes[i as usize].plane[1 as libc::c_int as usize]
            + plane[2 as libc::c_int as usize]
                * planes[i as usize].plane[2 as libc::c_int as usize]
            < 0 as libc::c_int as libc::c_float)
        {
            d = *p1.offset(0 as libc::c_int as isize)
                * planes[i as usize].plane[0 as libc::c_int as usize]
                + *p1.offset(1 as libc::c_int as isize)
                    * planes[i as usize].plane[1 as libc::c_int as usize]
                + *p1.offset(2 as libc::c_int as isize)
                    * planes[i as usize].plane[2 as libc::c_int as usize]
                - planes[i as usize].plane[3 as libc::c_int as usize];
            if !((d as libc::c_double) < -0.1f64 || d as libc::c_double > 0.1f64) {
                d = *p2.offset(0 as libc::c_int as isize)
                    * planes[i as usize].plane[0 as libc::c_int as usize]
                    + *p2.offset(1 as libc::c_int as isize)
                        * planes[i as usize].plane[1 as libc::c_int as usize]
                    + *p2.offset(2 as libc::c_int as isize)
                        * planes[i as usize].plane[2 as libc::c_int as usize]
                    - planes[i as usize].plane[3 as libc::c_int as usize];
                if !((d as libc::c_double) < -0.1f64 || d as libc::c_double > 0.1f64) {
                    d = *p3.offset(0 as libc::c_int as isize)
                        * planes[i as usize].plane[0 as libc::c_int as usize]
                        + *p3.offset(1 as libc::c_int as isize)
                            * planes[i as usize].plane[1 as libc::c_int as usize]
                        + *p3.offset(2 as libc::c_int as isize)
                            * planes[i as usize].plane[2 as libc::c_int as usize]
                        - planes[i as usize].plane[3 as libc::c_int as usize];
                    if !((d as libc::c_double) < -0.1f64 || d as libc::c_double > 0.1f64) {
                        // found it
                        return i;
                    }
                }
            }
        }
        i += 1
        // allow backwards planes?
    }
    // add a new plane
    if numPlanes == 2048 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"MAX_PATCH_PLANES\x00" as *const u8 as *const libc::c_char,
        );
    }
    planes[numPlanes as usize].plane[0 as libc::c_int as usize] = plane[0 as libc::c_int as usize];
    planes[numPlanes as usize].plane[1 as libc::c_int as usize] = plane[1 as libc::c_int as usize];
    planes[numPlanes as usize].plane[2 as libc::c_int as usize] = plane[2 as libc::c_int as usize];
    planes[numPlanes as usize].plane[3 as libc::c_int as usize] = plane[3 as libc::c_int as usize];
    planes[numPlanes as usize].signbits = CM_SignbitsForNormal(plane.as_mut_ptr());
    numPlanes += 1;
    return numPlanes - 1 as libc::c_int;
}
/*
==================
CM_PointOnPlaneSide
==================
*/

unsafe extern "C" fn CM_PointOnPlaneSide(
    mut p: *mut libc::c_float,
    mut planeNum: libc::c_int,
) -> libc::c_int {
    let mut plane: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut d: libc::c_float = 0.;
    if planeNum == -(1 as libc::c_int) {
        return 2 as libc::c_int;
    }
    plane = planes[planeNum as usize].plane.as_mut_ptr();
    d = *p.offset(0 as libc::c_int as isize) * *plane.offset(0 as libc::c_int as isize)
        + *p.offset(1 as libc::c_int as isize) * *plane.offset(1 as libc::c_int as isize)
        + *p.offset(2 as libc::c_int as isize) * *plane.offset(2 as libc::c_int as isize)
        - *plane.offset(3 as libc::c_int as isize);
    if d as libc::c_double > 0.1f64 {
        return 0 as libc::c_int;
    }
    if (d as libc::c_double) < -0.1f64 {
        return 1 as libc::c_int;
    }
    return 2 as libc::c_int;
}
/*
==================
CM_GridPlane
==================
*/

unsafe extern "C" fn CM_GridPlane(
    mut gridPlanes: *mut [[libc::c_int; 2]; 129],
    mut i: libc::c_int,
    mut j: libc::c_int,
    mut tri: libc::c_int,
) -> libc::c_int {
    let mut p: libc::c_int = 0;
    p = (*gridPlanes.offset(i as isize))[j as usize][tri as usize];
    if p != -(1 as libc::c_int) {
        return p;
    }
    p = (*gridPlanes.offset(i as isize))[j as usize][(tri == 0) as libc::c_int as usize];
    if p != -(1 as libc::c_int) {
        return p;
    }
    // should never happen
    crate::src::qcommon::common::Com_Printf(
        b"WARNING: CM_GridPlane unresolvable\n\x00" as *const u8 as *const libc::c_char,
    );
    return -(1 as libc::c_int);
}
/*
==================
CM_EdgePlaneNum
==================
*/

unsafe extern "C" fn CM_EdgePlaneNum(
    mut grid: *mut crate::src::qcommon::cm_patch::cGrid_t,
    mut gridPlanes: *mut [[libc::c_int; 2]; 129],
    mut i: libc::c_int,
    mut j: libc::c_int,
    mut k: libc::c_int,
) -> libc::c_int {
    let mut p1: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut p2: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p: libc::c_int = 0;
    match k {
        0 => {
            // top border
            p1 = (*grid).points[i as usize][j as usize].as_mut_ptr();
            p2 = (*grid).points[(i + 1 as libc::c_int) as usize][j as usize].as_mut_ptr();
            p = CM_GridPlane(gridPlanes, i, j, 0 as libc::c_int);
            if p == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            up[0 as libc::c_int as usize] = *p1.offset(0 as libc::c_int as isize)
                + planes[p as usize].plane[0 as libc::c_int as usize]
                    * 4 as libc::c_int as libc::c_float;
            up[1 as libc::c_int as usize] = *p1.offset(1 as libc::c_int as isize)
                + planes[p as usize].plane[1 as libc::c_int as usize]
                    * 4 as libc::c_int as libc::c_float;
            up[2 as libc::c_int as usize] = *p1.offset(2 as libc::c_int as isize)
                + planes[p as usize].plane[2 as libc::c_int as usize]
                    * 4 as libc::c_int as libc::c_float;
            return CM_FindPlane(p1, p2, up.as_mut_ptr());
        }
        2 => {
            // bottom border
            p1 = (*grid).points[i as usize][(j + 1 as libc::c_int) as usize].as_mut_ptr();
            p2 = (*grid).points[(i + 1 as libc::c_int) as usize][(j + 1 as libc::c_int) as usize]
                .as_mut_ptr();
            p = CM_GridPlane(gridPlanes, i, j, 1 as libc::c_int);
            if p == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            up[0 as libc::c_int as usize] = *p1.offset(0 as libc::c_int as isize)
                + planes[p as usize].plane[0 as libc::c_int as usize]
                    * 4 as libc::c_int as libc::c_float;
            up[1 as libc::c_int as usize] = *p1.offset(1 as libc::c_int as isize)
                + planes[p as usize].plane[1 as libc::c_int as usize]
                    * 4 as libc::c_int as libc::c_float;
            up[2 as libc::c_int as usize] = *p1.offset(2 as libc::c_int as isize)
                + planes[p as usize].plane[2 as libc::c_int as usize]
                    * 4 as libc::c_int as libc::c_float;
            return CM_FindPlane(p2, p1, up.as_mut_ptr());
        }
        3 => {
            // left border
            p1 = (*grid).points[i as usize][j as usize].as_mut_ptr();
            p2 = (*grid).points[i as usize][(j + 1 as libc::c_int) as usize].as_mut_ptr();
            p = CM_GridPlane(gridPlanes, i, j, 1 as libc::c_int);
            if p == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            up[0 as libc::c_int as usize] = *p1.offset(0 as libc::c_int as isize)
                + planes[p as usize].plane[0 as libc::c_int as usize]
                    * 4 as libc::c_int as libc::c_float;
            up[1 as libc::c_int as usize] = *p1.offset(1 as libc::c_int as isize)
                + planes[p as usize].plane[1 as libc::c_int as usize]
                    * 4 as libc::c_int as libc::c_float;
            up[2 as libc::c_int as usize] = *p1.offset(2 as libc::c_int as isize)
                + planes[p as usize].plane[2 as libc::c_int as usize]
                    * 4 as libc::c_int as libc::c_float;
            return CM_FindPlane(p2, p1, up.as_mut_ptr());
        }
        1 => {
            // right border
            p1 = (*grid).points[(i + 1 as libc::c_int) as usize][j as usize].as_mut_ptr();
            p2 = (*grid).points[(i + 1 as libc::c_int) as usize][(j + 1 as libc::c_int) as usize]
                .as_mut_ptr();
            p = CM_GridPlane(gridPlanes, i, j, 0 as libc::c_int);
            if p == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            up[0 as libc::c_int as usize] = *p1.offset(0 as libc::c_int as isize)
                + planes[p as usize].plane[0 as libc::c_int as usize]
                    * 4 as libc::c_int as libc::c_float;
            up[1 as libc::c_int as usize] = *p1.offset(1 as libc::c_int as isize)
                + planes[p as usize].plane[1 as libc::c_int as usize]
                    * 4 as libc::c_int as libc::c_float;
            up[2 as libc::c_int as usize] = *p1.offset(2 as libc::c_int as isize)
                + planes[p as usize].plane[2 as libc::c_int as usize]
                    * 4 as libc::c_int as libc::c_float;
            return CM_FindPlane(p1, p2, up.as_mut_ptr());
        }
        4 => {
            // diagonal out of triangle 0
            p1 = (*grid).points[(i + 1 as libc::c_int) as usize][(j + 1 as libc::c_int) as usize]
                .as_mut_ptr();
            p2 = (*grid).points[i as usize][j as usize].as_mut_ptr();
            p = CM_GridPlane(gridPlanes, i, j, 0 as libc::c_int);
            if p == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            up[0 as libc::c_int as usize] = *p1.offset(0 as libc::c_int as isize)
                + planes[p as usize].plane[0 as libc::c_int as usize]
                    * 4 as libc::c_int as libc::c_float;
            up[1 as libc::c_int as usize] = *p1.offset(1 as libc::c_int as isize)
                + planes[p as usize].plane[1 as libc::c_int as usize]
                    * 4 as libc::c_int as libc::c_float;
            up[2 as libc::c_int as usize] = *p1.offset(2 as libc::c_int as isize)
                + planes[p as usize].plane[2 as libc::c_int as usize]
                    * 4 as libc::c_int as libc::c_float;
            return CM_FindPlane(p1, p2, up.as_mut_ptr());
        }
        5 => {
            // diagonal out of triangle 1
            p1 = (*grid).points[i as usize][j as usize].as_mut_ptr();
            p2 = (*grid).points[(i + 1 as libc::c_int) as usize][(j + 1 as libc::c_int) as usize]
                .as_mut_ptr();
            p = CM_GridPlane(gridPlanes, i, j, 1 as libc::c_int);
            if p == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            up[0 as libc::c_int as usize] = *p1.offset(0 as libc::c_int as isize)
                + planes[p as usize].plane[0 as libc::c_int as usize]
                    * 4 as libc::c_int as libc::c_float;
            up[1 as libc::c_int as usize] = *p1.offset(1 as libc::c_int as isize)
                + planes[p as usize].plane[1 as libc::c_int as usize]
                    * 4 as libc::c_int as libc::c_float;
            up[2 as libc::c_int as usize] = *p1.offset(2 as libc::c_int as isize)
                + planes[p as usize].plane[2 as libc::c_int as usize]
                    * 4 as libc::c_int as libc::c_float;
            return CM_FindPlane(p1, p2, up.as_mut_ptr());
        }
        _ => {}
    }
    crate::src::qcommon::common::Com_Error(
        crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
        b"CM_EdgePlaneNum: bad k\x00" as *const u8 as *const libc::c_char,
    );
}
/*
===================
CM_SetBorderInward
===================
*/

unsafe extern "C" fn CM_SetBorderInward(
    mut facet: *mut crate::src::qcommon::cm_patch::facet_t,
    mut grid: *mut crate::src::qcommon::cm_patch::cGrid_t,
    mut gridPlanes: *mut [[libc::c_int; 2]; 129],
    mut i: libc::c_int,
    mut j: libc::c_int,
    mut which: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut points: [*mut libc::c_float; 4] = [0 as *mut libc::c_float; 4];
    let mut numPoints: libc::c_int = 0;
    match which {
        -1 => {
            points[0 as libc::c_int as usize] = (*grid).points[i as usize][j as usize].as_mut_ptr();
            points[1 as libc::c_int as usize] =
                (*grid).points[(i + 1 as libc::c_int) as usize][j as usize].as_mut_ptr();
            points[2 as libc::c_int as usize] = (*grid).points[(i + 1 as libc::c_int) as usize]
                [(j + 1 as libc::c_int) as usize]
                .as_mut_ptr();
            points[3 as libc::c_int as usize] =
                (*grid).points[i as usize][(j + 1 as libc::c_int) as usize].as_mut_ptr();
            numPoints = 4 as libc::c_int
        }
        0 => {
            points[0 as libc::c_int as usize] = (*grid).points[i as usize][j as usize].as_mut_ptr();
            points[1 as libc::c_int as usize] =
                (*grid).points[(i + 1 as libc::c_int) as usize][j as usize].as_mut_ptr();
            points[2 as libc::c_int as usize] = (*grid).points[(i + 1 as libc::c_int) as usize]
                [(j + 1 as libc::c_int) as usize]
                .as_mut_ptr();
            numPoints = 3 as libc::c_int
        }
        1 => {
            points[0 as libc::c_int as usize] = (*grid).points[(i + 1 as libc::c_int) as usize]
                [(j + 1 as libc::c_int) as usize]
                .as_mut_ptr();
            points[1 as libc::c_int as usize] =
                (*grid).points[i as usize][(j + 1 as libc::c_int) as usize].as_mut_ptr();
            points[2 as libc::c_int as usize] = (*grid).points[i as usize][j as usize].as_mut_ptr();
            numPoints = 3 as libc::c_int
        }
        _ => {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
                b"CM_SetBorderInward: bad parameter\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    k = 0 as libc::c_int;
    while k < (*facet).numBorders {
        let mut front: libc::c_int = 0;
        let mut back: libc::c_int = 0;
        front = 0 as libc::c_int;
        back = 0 as libc::c_int;
        l = 0 as libc::c_int;
        while l < numPoints {
            let mut side: libc::c_int = 0;
            side = CM_PointOnPlaneSide(points[l as usize], (*facet).borderPlanes[k as usize]);
            if side == 0 as libc::c_int {
                front += 1
            }
            if side == 1 as libc::c_int {
                back += 1
            }
            l += 1
        }
        if front != 0 && back == 0 {
            (*facet).borderInward[k as usize] = crate::src::qcommon::q_shared::qtrue as libc::c_int
        } else if back != 0 && front == 0 {
            (*facet).borderInward[k as usize] = crate::src::qcommon::q_shared::qfalse as libc::c_int
        } else if front == 0 && back == 0 {
            // flat side border
            (*facet).borderPlanes[k as usize] = -(1 as libc::c_int)
        } else {
            // bisecting side border
            crate::src::qcommon::common::Com_DPrintf(
                b"WARNING: CM_SetBorderInward: mixed plane sides\n\x00" as *const u8
                    as *const libc::c_char,
            );
            (*facet).borderInward[k as usize] =
                crate::src::qcommon::q_shared::qfalse as libc::c_int;
            if debugBlock as u64 == 0 {
                debugBlock = crate::src::qcommon::q_shared::qtrue;
                debugBlockPoints[0 as libc::c_int as usize][0 as libc::c_int as usize] =
                    (*grid).points[i as usize][j as usize][0 as libc::c_int as usize];
                debugBlockPoints[0 as libc::c_int as usize][1 as libc::c_int as usize] =
                    (*grid).points[i as usize][j as usize][1 as libc::c_int as usize];
                debugBlockPoints[0 as libc::c_int as usize][2 as libc::c_int as usize] =
                    (*grid).points[i as usize][j as usize][2 as libc::c_int as usize];
                debugBlockPoints[1 as libc::c_int as usize][0 as libc::c_int as usize] = (*grid)
                    .points[(i + 1 as libc::c_int) as usize][j as usize]
                    [0 as libc::c_int as usize];
                debugBlockPoints[1 as libc::c_int as usize][1 as libc::c_int as usize] = (*grid)
                    .points[(i + 1 as libc::c_int) as usize][j as usize]
                    [1 as libc::c_int as usize];
                debugBlockPoints[1 as libc::c_int as usize][2 as libc::c_int as usize] = (*grid)
                    .points[(i + 1 as libc::c_int) as usize][j as usize]
                    [2 as libc::c_int as usize];
                debugBlockPoints[2 as libc::c_int as usize][0 as libc::c_int as usize] = (*grid)
                    .points[(i + 1 as libc::c_int) as usize][(j + 1 as libc::c_int) as usize]
                    [0 as libc::c_int as usize];
                debugBlockPoints[2 as libc::c_int as usize][1 as libc::c_int as usize] = (*grid)
                    .points[(i + 1 as libc::c_int) as usize][(j + 1 as libc::c_int) as usize]
                    [1 as libc::c_int as usize];
                debugBlockPoints[2 as libc::c_int as usize][2 as libc::c_int as usize] = (*grid)
                    .points[(i + 1 as libc::c_int) as usize][(j + 1 as libc::c_int) as usize]
                    [2 as libc::c_int as usize];
                debugBlockPoints[3 as libc::c_int as usize][0 as libc::c_int as usize] = (*grid)
                    .points[i as usize][(j + 1 as libc::c_int) as usize]
                    [0 as libc::c_int as usize];
                debugBlockPoints[3 as libc::c_int as usize][1 as libc::c_int as usize] = (*grid)
                    .points[i as usize][(j + 1 as libc::c_int) as usize]
                    [1 as libc::c_int as usize];
                debugBlockPoints[3 as libc::c_int as usize][2 as libc::c_int as usize] = (*grid)
                    .points[i as usize][(j + 1 as libc::c_int) as usize]
                    [2 as libc::c_int as usize]
            }
        }
        k += 1
    }
}
/*
==================
CM_ValidateFacet

If the facet isn't bounded by its borders, we screwed up.
==================
*/

unsafe extern "C" fn CM_ValidateFacet(
    mut facet: *mut crate::src::qcommon::cm_patch::facet_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut plane: [libc::c_float; 4] = [0.; 4];
    let mut j: libc::c_int = 0;
    let mut w: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    let mut bounds: [crate::src::qcommon::q_shared::vec3_t; 2] = [[0.; 3]; 2];
    if (*facet).surfacePlane == -(1 as libc::c_int) {
        return crate::src::qcommon::q_shared::qfalse;
    }
    plane[0 as libc::c_int as usize] =
        planes[(*facet).surfacePlane as usize].plane[0 as libc::c_int as usize];
    plane[1 as libc::c_int as usize] =
        planes[(*facet).surfacePlane as usize].plane[1 as libc::c_int as usize];
    plane[2 as libc::c_int as usize] =
        planes[(*facet).surfacePlane as usize].plane[2 as libc::c_int as usize];
    plane[3 as libc::c_int as usize] =
        planes[(*facet).surfacePlane as usize].plane[3 as libc::c_int as usize];
    w = crate::src::qcommon::cm_polylib::BaseWindingForPlane(
        plane.as_mut_ptr(),
        plane[3 as libc::c_int as usize],
    );
    j = 0 as libc::c_int;
    while j < (*facet).numBorders && !w.is_null() {
        if (*facet).borderPlanes[j as usize] == -(1 as libc::c_int) {
            crate::src::qcommon::cm_polylib::FreeWinding(w);
            return crate::src::qcommon::q_shared::qfalse;
        }
        plane[0 as libc::c_int as usize] =
            planes[(*facet).borderPlanes[j as usize] as usize].plane[0 as libc::c_int as usize];
        plane[1 as libc::c_int as usize] =
            planes[(*facet).borderPlanes[j as usize] as usize].plane[1 as libc::c_int as usize];
        plane[2 as libc::c_int as usize] =
            planes[(*facet).borderPlanes[j as usize] as usize].plane[2 as libc::c_int as usize];
        plane[3 as libc::c_int as usize] =
            planes[(*facet).borderPlanes[j as usize] as usize].plane[3 as libc::c_int as usize];
        if (*facet).borderInward[j as usize] == 0 {
            plane[0 as libc::c_int as usize] = crate::src::qcommon::q_math::vec3_origin
                [0 as libc::c_int as usize]
                - plane[0 as libc::c_int as usize];
            plane[1 as libc::c_int as usize] = crate::src::qcommon::q_math::vec3_origin
                [1 as libc::c_int as usize]
                - plane[1 as libc::c_int as usize];
            plane[2 as libc::c_int as usize] = crate::src::qcommon::q_math::vec3_origin
                [2 as libc::c_int as usize]
                - plane[2 as libc::c_int as usize];
            plane[3 as libc::c_int as usize] = -plane[3 as libc::c_int as usize]
        }
        crate::src::qcommon::cm_polylib::ChopWindingInPlace(
            &mut w,
            plane.as_mut_ptr(),
            plane[3 as libc::c_int as usize],
            0.1f32,
        );
        j += 1
    }
    if w.is_null() {
        return crate::src::qcommon::q_shared::qfalse;
        // winding was completely chopped away
    }
    // see if the facet is unreasonably large
    crate::src::qcommon::cm_polylib::WindingBounds(
        w,
        bounds[0 as libc::c_int as usize].as_mut_ptr(),
        bounds[1 as libc::c_int as usize].as_mut_ptr(),
    );
    crate::src::qcommon::cm_polylib::FreeWinding(w);
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        if bounds[1 as libc::c_int as usize][j as usize]
            - bounds[0 as libc::c_int as usize][j as usize]
            > 65535 as libc::c_int as libc::c_float
        {
            return crate::src::qcommon::q_shared::qfalse;
            // we must be missing a plane
        }
        if bounds[0 as libc::c_int as usize][j as usize] >= 65535 as libc::c_int as libc::c_float {
            return crate::src::qcommon::q_shared::qfalse;
        }
        if bounds[1 as libc::c_int as usize][j as usize] <= -(65535 as libc::c_int) as libc::c_float
        {
            return crate::src::qcommon::q_shared::qfalse;
        }
        j += 1
    }
    return crate::src::qcommon::q_shared::qtrue;
    // winding is fine
}
/*
==================
CM_AddFacetBevels
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_AddFacetBevels(mut facet: *mut crate::src::qcommon::cm_patch::facet_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut axis: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut flipped: libc::c_int = 0;
    let mut plane: [libc::c_float; 4] = [0.; 4];
    let mut d: libc::c_float = 0.;
    let mut newplane: [libc::c_float; 4] = [0.; 4];
    let mut w: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    let mut w2: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vec2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    plane[0 as libc::c_int as usize] =
        planes[(*facet).surfacePlane as usize].plane[0 as libc::c_int as usize];
    plane[1 as libc::c_int as usize] =
        planes[(*facet).surfacePlane as usize].plane[1 as libc::c_int as usize];
    plane[2 as libc::c_int as usize] =
        planes[(*facet).surfacePlane as usize].plane[2 as libc::c_int as usize];
    plane[3 as libc::c_int as usize] =
        planes[(*facet).surfacePlane as usize].plane[3 as libc::c_int as usize];
    w = crate::src::qcommon::cm_polylib::BaseWindingForPlane(
        plane.as_mut_ptr(),
        plane[3 as libc::c_int as usize],
    );
    j = 0 as libc::c_int;
    while j < (*facet).numBorders && !w.is_null() {
        if !((*facet).borderPlanes[j as usize] == (*facet).surfacePlane) {
            plane[0 as libc::c_int as usize] =
                planes[(*facet).borderPlanes[j as usize] as usize].plane[0 as libc::c_int as usize];
            plane[1 as libc::c_int as usize] =
                planes[(*facet).borderPlanes[j as usize] as usize].plane[1 as libc::c_int as usize];
            plane[2 as libc::c_int as usize] =
                planes[(*facet).borderPlanes[j as usize] as usize].plane[2 as libc::c_int as usize];
            plane[3 as libc::c_int as usize] =
                planes[(*facet).borderPlanes[j as usize] as usize].plane[3 as libc::c_int as usize];
            if (*facet).borderInward[j as usize] == 0 {
                plane[0 as libc::c_int as usize] = crate::src::qcommon::q_math::vec3_origin
                    [0 as libc::c_int as usize]
                    - plane[0 as libc::c_int as usize];
                plane[1 as libc::c_int as usize] = crate::src::qcommon::q_math::vec3_origin
                    [1 as libc::c_int as usize]
                    - plane[1 as libc::c_int as usize];
                plane[2 as libc::c_int as usize] = crate::src::qcommon::q_math::vec3_origin
                    [2 as libc::c_int as usize]
                    - plane[2 as libc::c_int as usize];
                plane[3 as libc::c_int as usize] = -plane[3 as libc::c_int as usize]
            }
            crate::src::qcommon::cm_polylib::ChopWindingInPlace(
                &mut w,
                plane.as_mut_ptr(),
                plane[3 as libc::c_int as usize],
                0.1f32,
            );
        }
        j += 1
    }
    if w.is_null() {
        return;
    }
    crate::src::qcommon::cm_polylib::WindingBounds(w, mins.as_mut_ptr(), maxs.as_mut_ptr());
    // add the axial planes
    axis = 0 as libc::c_int;
    while axis < 3 as libc::c_int {
        dir = -(1 as libc::c_int);
        while dir <= 1 as libc::c_int {
            plane[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
            plane[1 as libc::c_int as usize] = plane[2 as libc::c_int as usize];
            plane[0 as libc::c_int as usize] = plane[1 as libc::c_int as usize];
            plane[axis as usize] = dir as libc::c_float;
            if dir == 1 as libc::c_int {
                plane[3 as libc::c_int as usize] = maxs[axis as usize]
            } else {
                plane[3 as libc::c_int as usize] = -mins[axis as usize]
            }
            //if it's the surface plane
            if !(CM_PlaneEqual(
                &mut *planes.as_mut_ptr().offset((*facet).surfacePlane as isize),
                plane.as_mut_ptr(),
                &mut flipped,
            ) != 0)
            {
                // see if the plane is already present
                i = 0 as libc::c_int;
                while i < (*facet).numBorders {
                    if CM_PlaneEqual(&mut *planes.as_mut_ptr().offset(*(*facet).borderPlanes.as_mut_ptr().offset(i
                                                                                                                     as
                                                                                                                     isize)
                                                                          as
                                                                          isize),
                                     plane.as_mut_ptr(), &mut flipped) != 0 {
                        break ;
                    }
                    i += 1
                }
                if i == (*facet).numBorders {
                    if (*facet).numBorders
                        >= 4 as libc::c_int + 6 as libc::c_int + 16 as libc::c_int
                    {
                        crate::src::qcommon::common::Com_Printf(
                            b"ERROR: too many bevels\n\x00" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        (*facet).borderPlanes[(*facet).numBorders as usize] =
                            CM_FindPlane2(plane.as_mut_ptr(), &mut flipped);
                        (*facet).borderNoAdjust[(*facet).numBorders as usize] =
                            crate::src::qcommon::q_shared::qfalse;
                        (*facet).borderInward[(*facet).numBorders as usize] = flipped;
                        (*facet).numBorders += 1
                    }
                }
            }
            dir += 2 as libc::c_int
        }
        axis += 1
    }
    //
    // add the edge bevels
    //
    // test the non-axial plane edges
    j = 0 as libc::c_int;
    while j < (*w).numpoints {
        k = (j + 1 as libc::c_int) % (*w).numpoints;
        vec[0 as libc::c_int as usize] = (*(*w).p.as_mut_ptr().offset(j as isize))
            [0 as libc::c_int as usize]
            - (*(*w).p.as_mut_ptr().offset(k as isize))[0 as libc::c_int as usize];
        vec[1 as libc::c_int as usize] = (*(*w).p.as_mut_ptr().offset(j as isize))
            [1 as libc::c_int as usize]
            - (*(*w).p.as_mut_ptr().offset(k as isize))[1 as libc::c_int as usize];
        vec[2 as libc::c_int as usize] = (*(*w).p.as_mut_ptr().offset(j as isize))
            [2 as libc::c_int as usize]
            - (*(*w).p.as_mut_ptr().offset(k as isize))[2 as libc::c_int as usize];
        //if it's a degenerate edge
        if !((crate::src::qcommon::q_math::VectorNormalize(vec.as_mut_ptr()) as libc::c_double)
            < 0.5f64)
        {
            CM_SnapVector(vec.as_mut_ptr()); // axial
            k = 0 as libc::c_int; // only test non-axial edges
            while k < 3 as libc::c_int {
                if vec[k as usize] == -(1 as libc::c_int) as libc::c_float
                    || vec[k as usize] == 1 as libc::c_int as libc::c_float
                {
                    break;
                }
                k += 1
            }
            if !(k < 3 as libc::c_int) {
                // try the six possible slanted axials from this edge
                axis = 0 as libc::c_int;
                while axis < 3 as libc::c_int {
                    dir = -(1 as libc::c_int);
                    while dir <= 1 as libc::c_int {
                        // construct a plane
                        vec2[2 as libc::c_int as usize] =
                            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
                        vec2[1 as libc::c_int as usize] = vec2[2 as libc::c_int as usize];
                        vec2[0 as libc::c_int as usize] = vec2[1 as libc::c_int as usize];
                        vec2[axis as usize] = dir as crate::src::qcommon::q_shared::vec_t;
                        CrossProduct(
                            vec.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                            vec2.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                            plane.as_mut_ptr(),
                        );
                        if !((crate::src::qcommon::q_math::VectorNormalize(plane.as_mut_ptr())
                            as libc::c_double)
                            < 0.5f64)
                        {
                            plane[3 as libc::c_int as usize] =
                                (*(*w).p.as_mut_ptr().offset(j as isize))
                                    [0 as libc::c_int as usize]
                                    * plane[0 as libc::c_int as usize]
                                    + (*(*w).p.as_mut_ptr().offset(j as isize))
                                        [1 as libc::c_int as usize]
                                        * plane[1 as libc::c_int as usize]
                                    + (*(*w).p.as_mut_ptr().offset(j as isize))
                                        [2 as libc::c_int as usize]
                                        * plane[2 as libc::c_int as usize];
                            // if all the points of the facet winding are
                            // behind this plane, it is a proper edge bevel
                            l = 0 as libc::c_int;
                            while l < (*w).numpoints {
                                d = (*(*w).p.as_mut_ptr().offset(l as isize))
                                    [0 as libc::c_int as usize]
                                    * plane[0 as libc::c_int as usize]
                                    + (*(*w).p.as_mut_ptr().offset(l as isize))
                                        [1 as libc::c_int as usize]
                                        * plane[1 as libc::c_int as usize]
                                    + (*(*w).p.as_mut_ptr().offset(l as isize))
                                        [2 as libc::c_int as usize]
                                        * plane[2 as libc::c_int as usize]
                                    - plane[3 as libc::c_int as usize];
                                if d as libc::c_double > 0.1f64 {
                                    break;
                                }
                                l += 1
                                // point in front
                            }
                            if !(l < (*w).numpoints) {
                                //if it's the surface plane
                                if !(CM_PlaneEqual(
                                    &mut *planes
                                        .as_mut_ptr()
                                        .offset((*facet).surfacePlane as isize),
                                    plane.as_mut_ptr(),
                                    &mut flipped,
                                ) != 0)
                                {
                                    // see if the plane is already present
                                    i = 0 as libc::c_int;
                                    while i < (*facet).numBorders {
                                        if CM_PlaneEqual(
                                            &mut *planes.as_mut_ptr().offset(
                                                *(*facet)
                                                    .borderPlanes
                                                    .as_mut_ptr()
                                                    .offset(i as isize)
                                                    as isize,
                                            ),
                                            plane.as_mut_ptr(),
                                            &mut flipped,
                                        ) != 0
                                        {
                                            break;
                                        }
                                        i += 1
                                    }
                                    if i == (*facet).numBorders {
                                        if (*facet).numBorders
                                            >= 4 as libc::c_int
                                                + 6 as libc::c_int
                                                + 16 as libc::c_int
                                        {
                                            crate::src::qcommon::common::Com_Printf(
                                                b"ERROR: too many bevels\n\x00" as *const u8
                                                    as *const libc::c_char,
                                            );
                                        } else {
                                            (*facet).borderPlanes[(*facet).numBorders as usize] =
                                                CM_FindPlane2(plane.as_mut_ptr(), &mut flipped);
                                            k = 0 as libc::c_int;
                                            while k < (*facet).numBorders {
                                                if (*facet).borderPlanes
                                                    [(*facet).numBorders as usize]
                                                    == (*facet).borderPlanes[k as usize]
                                                {
                                                    crate::src::qcommon::common::Com_Printf(
                                                        b"WARNING: bevel plane already used\n\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                }
                                                k += 1
                                            }
                                            (*facet).borderNoAdjust[(*facet).numBorders as usize] =
                                                crate::src::qcommon::q_shared::qfalse;
                                            (*facet).borderInward[(*facet).numBorders as usize] =
                                                flipped;
                                            //
                                            w2 = crate::src::qcommon::cm_polylib::CopyWinding(w); //end if
                                            newplane[0 as libc::c_int as usize] = planes[(*facet)
                                                .borderPlanes
                                                [(*facet).numBorders as usize]
                                                as usize]
                                                .plane
                                                [0 as libc::c_int as usize];
                                            newplane[1 as libc::c_int as usize] = planes[(*facet)
                                                .borderPlanes
                                                [(*facet).numBorders as usize]
                                                as usize]
                                                .plane
                                                [1 as libc::c_int as usize];
                                            newplane[2 as libc::c_int as usize] = planes[(*facet)
                                                .borderPlanes
                                                [(*facet).numBorders as usize]
                                                as usize]
                                                .plane
                                                [2 as libc::c_int as usize];
                                            newplane[3 as libc::c_int as usize] = planes[(*facet)
                                                .borderPlanes
                                                [(*facet).numBorders as usize]
                                                as usize]
                                                .plane
                                                [3 as libc::c_int as usize];
                                            if (*facet).borderInward[(*facet).numBorders as usize]
                                                == 0
                                            {
                                                newplane[0 as libc::c_int as usize] =
                                                    -newplane[0 as libc::c_int as usize];
                                                newplane[1 as libc::c_int as usize] =
                                                    -newplane[1 as libc::c_int as usize];
                                                newplane[2 as libc::c_int as usize] =
                                                    -newplane[2 as libc::c_int as usize];
                                                newplane[3 as libc::c_int as usize] =
                                                    -newplane[3 as libc::c_int as usize]
                                            }
                                            crate::src::qcommon::cm_polylib::ChopWindingInPlace(
                                                &mut w2,
                                                newplane.as_mut_ptr(),
                                                newplane[3 as libc::c_int as usize],
                                                0.1f32,
                                            );
                                            if w2.is_null() {
                                                crate::src::qcommon::common::Com_DPrintf(b"WARNING: CM_AddFacetBevels... invalid bevel\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char);
                                            } else {
                                                crate::src::qcommon::cm_polylib::FreeWinding(w2);
                                                //
                                                (*facet).numBorders += 1
                                            }
                                        }
                                        //already got a bevel
                                        //					break;
                                    }
                                }
                            }
                        }
                        dir += 2 as libc::c_int
                    }
                    axis += 1
                }
            }
        }
        j += 1
    }
    crate::src::qcommon::cm_polylib::FreeWinding(w);
    //add opposite plane
    if (*facet).numBorders >= 4 as libc::c_int + 6 as libc::c_int + 16 as libc::c_int {
        crate::src::qcommon::common::Com_Printf(
            b"ERROR: too many bevels\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*facet).borderPlanes[(*facet).numBorders as usize] = (*facet).surfacePlane;
    (*facet).borderNoAdjust[(*facet).numBorders as usize] = crate::src::qcommon::q_shared::qfalse;
    (*facet).borderInward[(*facet).numBorders as usize] =
        crate::src::qcommon::q_shared::qtrue as libc::c_int;
    (*facet).numBorders += 1;
    //BSPC
}
/*
==================
CM_PatchCollideFromGrid
==================
*/

unsafe extern "C" fn CM_PatchCollideFromGrid(
    mut grid: *mut crate::src::qcommon::cm_patch::cGrid_t,
    mut pf: *mut crate::src::qcommon::cm_patch::patchCollide_t,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p1: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut p2: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut p3: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut gridPlanes: [[[libc::c_int; 2]; 129]; 129] = [[[0; 2]; 129]; 129];
    let mut facet: *mut crate::src::qcommon::cm_patch::facet_t =
        0 as *mut crate::src::qcommon::cm_patch::facet_t;
    let mut borders: [libc::c_int; 4] = [0; 4];
    let mut noAdjust: [libc::c_int; 4] = [0; 4];
    numPlanes = 0 as libc::c_int;
    numFacets = 0 as libc::c_int;
    // find the planes for each triangle of the grid
    i = 0 as libc::c_int;
    while i < (*grid).width - 1 as libc::c_int {
        j = 0 as libc::c_int;
        while j < (*grid).height - 1 as libc::c_int {
            p1 = (*grid).points[i as usize][j as usize].as_mut_ptr();
            p2 = (*grid).points[(i + 1 as libc::c_int) as usize][j as usize].as_mut_ptr();
            p3 = (*grid).points[(i + 1 as libc::c_int) as usize][(j + 1 as libc::c_int) as usize]
                .as_mut_ptr();
            gridPlanes[i as usize][j as usize][0 as libc::c_int as usize] =
                CM_FindPlane(p1, p2, p3);
            p1 = (*grid).points[(i + 1 as libc::c_int) as usize][(j + 1 as libc::c_int) as usize]
                .as_mut_ptr();
            p2 = (*grid).points[i as usize][(j + 1 as libc::c_int) as usize].as_mut_ptr();
            p3 = (*grid).points[i as usize][j as usize].as_mut_ptr();
            gridPlanes[i as usize][j as usize][1 as libc::c_int as usize] =
                CM_FindPlane(p1, p2, p3);
            j += 1
        }
        i += 1
    }
    // create the borders for each facet
    i = 0 as libc::c_int;
    while i < (*grid).width - 1 as libc::c_int {
        j = 0 as libc::c_int;
        while j < (*grid).height - 1 as libc::c_int {
            borders[EN_TOP as libc::c_int as usize] = -(1 as libc::c_int);
            if j > 0 as libc::c_int {
                borders[EN_TOP as libc::c_int as usize] = gridPlanes[i as usize]
                    [(j - 1 as libc::c_int) as usize][1 as libc::c_int as usize]
            } else if (*grid).wrapHeight as u64 != 0 {
                borders[EN_TOP as libc::c_int as usize] = gridPlanes[i as usize]
                    [((*grid).height - 2 as libc::c_int) as usize][1 as libc::c_int as usize]
            }
            noAdjust[EN_TOP as libc::c_int as usize] = (borders[EN_TOP as libc::c_int as usize]
                == gridPlanes[i as usize][j as usize][0 as libc::c_int as usize])
                as libc::c_int;
            if borders[EN_TOP as libc::c_int as usize] == -(1 as libc::c_int)
                || noAdjust[EN_TOP as libc::c_int as usize] != 0
            {
                borders[EN_TOP as libc::c_int as usize] =
                    CM_EdgePlaneNum(grid, gridPlanes.as_mut_ptr(), i, j, 0 as libc::c_int)
            }
            borders[EN_BOTTOM as libc::c_int as usize] = -(1 as libc::c_int);
            if j < (*grid).height - 2 as libc::c_int {
                borders[EN_BOTTOM as libc::c_int as usize] = gridPlanes[i as usize]
                    [(j + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
            } else if (*grid).wrapHeight as u64 != 0 {
                borders[EN_BOTTOM as libc::c_int as usize] =
                    gridPlanes[i as usize][0 as libc::c_int as usize][0 as libc::c_int as usize]
            }
            noAdjust[EN_BOTTOM as libc::c_int as usize] = (borders
                [EN_BOTTOM as libc::c_int as usize]
                == gridPlanes[i as usize][j as usize][1 as libc::c_int as usize])
                as libc::c_int;
            if borders[EN_BOTTOM as libc::c_int as usize] == -(1 as libc::c_int)
                || noAdjust[EN_BOTTOM as libc::c_int as usize] != 0
            {
                borders[EN_BOTTOM as libc::c_int as usize] =
                    CM_EdgePlaneNum(grid, gridPlanes.as_mut_ptr(), i, j, 2 as libc::c_int)
            }
            borders[EN_LEFT as libc::c_int as usize] = -(1 as libc::c_int);
            if i > 0 as libc::c_int {
                borders[EN_LEFT as libc::c_int as usize] = gridPlanes
                    [(i - 1 as libc::c_int) as usize][j as usize][0 as libc::c_int as usize]
            } else if (*grid).wrapWidth as u64 != 0 {
                borders[EN_LEFT as libc::c_int as usize] = gridPlanes
                    [((*grid).width - 2 as libc::c_int) as usize][j as usize]
                    [0 as libc::c_int as usize]
            }
            noAdjust[EN_LEFT as libc::c_int as usize] = (borders[EN_LEFT as libc::c_int as usize]
                == gridPlanes[i as usize][j as usize][1 as libc::c_int as usize])
                as libc::c_int;
            if borders[EN_LEFT as libc::c_int as usize] == -(1 as libc::c_int)
                || noAdjust[EN_LEFT as libc::c_int as usize] != 0
            {
                borders[EN_LEFT as libc::c_int as usize] =
                    CM_EdgePlaneNum(grid, gridPlanes.as_mut_ptr(), i, j, 3 as libc::c_int)
            }
            borders[EN_RIGHT as libc::c_int as usize] = -(1 as libc::c_int);
            if i < (*grid).width - 2 as libc::c_int {
                borders[EN_RIGHT as libc::c_int as usize] = gridPlanes
                    [(i + 1 as libc::c_int) as usize][j as usize][1 as libc::c_int as usize]
            } else if (*grid).wrapWidth as u64 != 0 {
                borders[EN_RIGHT as libc::c_int as usize] =
                    gridPlanes[0 as libc::c_int as usize][j as usize][1 as libc::c_int as usize]
            }
            noAdjust[EN_RIGHT as libc::c_int as usize] = (borders[EN_RIGHT as libc::c_int as usize]
                == gridPlanes[i as usize][j as usize][0 as libc::c_int as usize])
                as libc::c_int;
            if borders[EN_RIGHT as libc::c_int as usize] == -(1 as libc::c_int)
                || noAdjust[EN_RIGHT as libc::c_int as usize] != 0
            {
                borders[EN_RIGHT as libc::c_int as usize] =
                    CM_EdgePlaneNum(grid, gridPlanes.as_mut_ptr(), i, j, 1 as libc::c_int)
            }
            if numFacets == 1024 as libc::c_int {
                crate::src::qcommon::common::Com_Error(
                    crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                    b"MAX_FACETS\x00" as *const u8 as *const libc::c_char,
                );
            }
            facet = &mut *facets.as_mut_ptr().offset(numFacets as isize)
                as *mut crate::src::qcommon::cm_patch::facet_t;
            crate::stdlib::memset(
                facet as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<crate::src::qcommon::cm_patch::facet_t>() as libc::c_ulong,
            );
            if gridPlanes[i as usize][j as usize][0 as libc::c_int as usize]
                == gridPlanes[i as usize][j as usize][1 as libc::c_int as usize]
            {
                if !(gridPlanes[i as usize][j as usize][0 as libc::c_int as usize]
                    == -(1 as libc::c_int))
                {
                    (*facet).surfacePlane =
                        gridPlanes[i as usize][j as usize][0 as libc::c_int as usize];
                    (*facet).numBorders = 4 as libc::c_int;
                    (*facet).borderPlanes[0 as libc::c_int as usize] =
                        borders[EN_TOP as libc::c_int as usize];
                    (*facet).borderNoAdjust[0 as libc::c_int as usize] = noAdjust
                        [EN_TOP as libc::c_int as usize]
                        as crate::src::qcommon::q_shared::qboolean;
                    (*facet).borderPlanes[1 as libc::c_int as usize] =
                        borders[EN_RIGHT as libc::c_int as usize];
                    (*facet).borderNoAdjust[1 as libc::c_int as usize] = noAdjust
                        [EN_RIGHT as libc::c_int as usize]
                        as crate::src::qcommon::q_shared::qboolean;
                    (*facet).borderPlanes[2 as libc::c_int as usize] =
                        borders[EN_BOTTOM as libc::c_int as usize];
                    (*facet).borderNoAdjust[2 as libc::c_int as usize] = noAdjust
                        [EN_BOTTOM as libc::c_int as usize]
                        as crate::src::qcommon::q_shared::qboolean;
                    (*facet).borderPlanes[3 as libc::c_int as usize] =
                        borders[EN_LEFT as libc::c_int as usize];
                    (*facet).borderNoAdjust[3 as libc::c_int as usize] = noAdjust
                        [EN_LEFT as libc::c_int as usize]
                        as crate::src::qcommon::q_shared::qboolean;
                    CM_SetBorderInward(
                        facet,
                        grid,
                        gridPlanes.as_mut_ptr(),
                        i,
                        j,
                        -(1 as libc::c_int),
                    );
                    if CM_ValidateFacet(facet) as u64 != 0 {
                        CM_AddFacetBevels(facet);
                        numFacets += 1
                    }
                }
            } else {
                // two separate triangles
                (*facet).surfacePlane =
                    gridPlanes[i as usize][j as usize][0 as libc::c_int as usize];
                (*facet).numBorders = 3 as libc::c_int;
                (*facet).borderPlanes[0 as libc::c_int as usize] =
                    borders[EN_TOP as libc::c_int as usize];
                (*facet).borderNoAdjust[0 as libc::c_int as usize] = noAdjust
                    [EN_TOP as libc::c_int as usize]
                    as crate::src::qcommon::q_shared::qboolean;
                (*facet).borderPlanes[1 as libc::c_int as usize] =
                    borders[EN_RIGHT as libc::c_int as usize];
                (*facet).borderNoAdjust[1 as libc::c_int as usize] = noAdjust
                    [EN_RIGHT as libc::c_int as usize]
                    as crate::src::qcommon::q_shared::qboolean;
                (*facet).borderPlanes[2 as libc::c_int as usize] =
                    gridPlanes[i as usize][j as usize][1 as libc::c_int as usize];
                if (*facet).borderPlanes[2 as libc::c_int as usize] == -(1 as libc::c_int) {
                    (*facet).borderPlanes[2 as libc::c_int as usize] =
                        borders[EN_BOTTOM as libc::c_int as usize];
                    if (*facet).borderPlanes[2 as libc::c_int as usize] == -(1 as libc::c_int) {
                        (*facet).borderPlanes[2 as libc::c_int as usize] =
                            CM_EdgePlaneNum(grid, gridPlanes.as_mut_ptr(), i, j, 4 as libc::c_int)
                    }
                }
                CM_SetBorderInward(facet, grid, gridPlanes.as_mut_ptr(), i, j, 0 as libc::c_int);
                if CM_ValidateFacet(facet) as u64 != 0 {
                    CM_AddFacetBevels(facet);
                    numFacets += 1
                }
                if numFacets == 1024 as libc::c_int {
                    crate::src::qcommon::common::Com_Error(
                        crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                        b"MAX_FACETS\x00" as *const u8 as *const libc::c_char,
                    );
                }
                facet = &mut *facets.as_mut_ptr().offset(numFacets as isize)
                    as *mut crate::src::qcommon::cm_patch::facet_t;
                crate::stdlib::memset(
                    facet as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<crate::src::qcommon::cm_patch::facet_t>()
                        as libc::c_ulong,
                );
                (*facet).surfacePlane =
                    gridPlanes[i as usize][j as usize][1 as libc::c_int as usize];
                (*facet).numBorders = 3 as libc::c_int;
                (*facet).borderPlanes[0 as libc::c_int as usize] =
                    borders[EN_BOTTOM as libc::c_int as usize];
                (*facet).borderNoAdjust[0 as libc::c_int as usize] = noAdjust
                    [EN_BOTTOM as libc::c_int as usize]
                    as crate::src::qcommon::q_shared::qboolean;
                (*facet).borderPlanes[1 as libc::c_int as usize] =
                    borders[EN_LEFT as libc::c_int as usize];
                (*facet).borderNoAdjust[1 as libc::c_int as usize] = noAdjust
                    [EN_LEFT as libc::c_int as usize]
                    as crate::src::qcommon::q_shared::qboolean;
                (*facet).borderPlanes[2 as libc::c_int as usize] =
                    gridPlanes[i as usize][j as usize][0 as libc::c_int as usize];
                if (*facet).borderPlanes[2 as libc::c_int as usize] == -(1 as libc::c_int) {
                    (*facet).borderPlanes[2 as libc::c_int as usize] =
                        borders[EN_TOP as libc::c_int as usize];
                    if (*facet).borderPlanes[2 as libc::c_int as usize] == -(1 as libc::c_int) {
                        (*facet).borderPlanes[2 as libc::c_int as usize] =
                            CM_EdgePlaneNum(grid, gridPlanes.as_mut_ptr(), i, j, 5 as libc::c_int)
                    }
                }
                CM_SetBorderInward(facet, grid, gridPlanes.as_mut_ptr(), i, j, 1 as libc::c_int);
                if CM_ValidateFacet(facet) as u64 != 0 {
                    CM_AddFacetBevels(facet);
                    numFacets += 1
                }
            }
            j += 1
            // degenrate
        }
        i += 1
    }
    // copy the results out
    (*pf).numPlanes = numPlanes;
    (*pf).numFacets = numFacets;
    (*pf).facets = crate::src::qcommon::common::Hunk_AllocDebug(
        (numFacets as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            crate::src::qcommon::cm_patch::facet_t,
        >() as libc::c_ulong) as libc::c_int,
        crate::src::qcommon::q_shared::h_high,
        b"numFacets * sizeof( *pf->facets )\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/qcommon/cm_patch.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        1159 as libc::c_int,
    ) as *mut crate::src::qcommon::cm_patch::facet_t;
    crate::stdlib::memcpy(
        (*pf).facets as *mut libc::c_void,
        facets.as_mut_ptr() as *const libc::c_void,
        (numFacets as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            crate::src::qcommon::cm_patch::facet_t,
        >() as libc::c_ulong),
    );
    (*pf).planes = crate::src::qcommon::common::Hunk_AllocDebug(
        (numPlanes as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            crate::src::qcommon::cm_patch::patchPlane_t,
        >() as libc::c_ulong) as libc::c_int,
        crate::src::qcommon::q_shared::h_high,
        b"numPlanes * sizeof( *pf->planes )\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/qcommon/cm_patch.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        1161 as libc::c_int,
    ) as *mut crate::src::qcommon::cm_patch::patchPlane_t;
    crate::stdlib::memcpy(
        (*pf).planes as *mut libc::c_void,
        planes.as_mut_ptr() as *const libc::c_void,
        (numPlanes as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            crate::src::qcommon::cm_patch::patchPlane_t,
        >() as libc::c_ulong),
    );
}
/*
===================
CM_GeneratePatchCollide

Creates an internal structure that will be used to perform
collision detection with a patch mesh.

Points is packed as concatenated rows.
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_GeneratePatchCollide(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut points: *mut crate::src::qcommon::q_shared::vec3_t,
) -> *mut crate::src::qcommon::cm_patch::patchCollide_s {
    let mut pf: *mut crate::src::qcommon::cm_patch::patchCollide_t =
        0 as *mut crate::src::qcommon::cm_patch::patchCollide_t;
    let mut grid: crate::src::qcommon::cm_patch::cGrid_t = crate::src::qcommon::cm_patch::cGrid_t {
        width: 0,
        height: 0,
        wrapWidth: crate::src::qcommon::q_shared::qfalse,
        wrapHeight: crate::src::qcommon::q_shared::qfalse,
        points: [[[0.; 3]; 129]; 129],
    };
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if width <= 2 as libc::c_int || height <= 2 as libc::c_int || points.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"CM_GeneratePatchFacets: bad parameters: (%i, %i, %p)\x00" as *const u8
                as *const libc::c_char,
            width,
            height,
            points as *mut libc::c_void,
        );
    }
    if width & 1 as libc::c_int == 0 || height & 1 as libc::c_int == 0 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"CM_GeneratePatchFacets: even sizes are invalid for quadratic meshes\x00" as *const u8
                as *const libc::c_char,
        );
    }
    if width > 129 as libc::c_int || height > 129 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"CM_GeneratePatchFacets: source is > MAX_GRID_SIZE\x00" as *const u8
                as *const libc::c_char,
        );
    }
    // build a grid
    grid.width = width;
    grid.height = height;
    grid.wrapWidth = crate::src::qcommon::q_shared::qfalse;
    grid.wrapHeight = crate::src::qcommon::q_shared::qfalse;
    i = 0 as libc::c_int;
    while i < width {
        j = 0 as libc::c_int;
        while j < height {
            grid.points[i as usize][j as usize][0 as libc::c_int as usize] =
                (*points.offset((j * width + i) as isize))[0 as libc::c_int as usize];
            grid.points[i as usize][j as usize][1 as libc::c_int as usize] =
                (*points.offset((j * width + i) as isize))[1 as libc::c_int as usize];
            grid.points[i as usize][j as usize][2 as libc::c_int as usize] =
                (*points.offset((j * width + i) as isize))[2 as libc::c_int as usize];
            j += 1
        }
        i += 1
    }
    // subdivide the grid
    CM_SetGridWrapWidth(&mut grid);
    CM_SubdivideGridColumns(&mut grid);
    CM_RemoveDegenerateColumns(&mut grid);
    CM_TransposeGrid(&mut grid);
    CM_SetGridWrapWidth(&mut grid);
    CM_SubdivideGridColumns(&mut grid);
    CM_RemoveDegenerateColumns(&mut grid);
    // we now have a grid of points exactly on the curve
    // the approximate surface defined by these points will be
    // collided against
    pf = crate::src::qcommon::common::Hunk_AllocDebug(
        ::std::mem::size_of::<crate::src::qcommon::cm_patch::patchCollide_t>() as libc::c_ulong
            as libc::c_int,
        crate::src::qcommon::q_shared::h_high,
        b"sizeof( *pf )\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/qcommon/cm_patch.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        1219 as libc::c_int,
    ) as *mut crate::src::qcommon::cm_patch::patchCollide_t;
    crate::src::qcommon::q_math::ClearBounds(
        (*pf).bounds[0 as libc::c_int as usize].as_mut_ptr(),
        (*pf).bounds[1 as libc::c_int as usize].as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < grid.width {
        j = 0 as libc::c_int;
        while j < grid.height {
            crate::src::qcommon::q_math::AddPointToBounds(
                grid.points[i as usize][j as usize].as_mut_ptr()
                    as *const crate::src::qcommon::q_shared::vec_t,
                (*pf).bounds[0 as libc::c_int as usize].as_mut_ptr(),
                (*pf).bounds[1 as libc::c_int as usize].as_mut_ptr(),
            );
            j += 1
        }
        i += 1
    }
    c_totalPatchBlocks += (grid.width - 1 as libc::c_int) * (grid.height - 1 as libc::c_int);
    // generate a bsp tree for the surface
    CM_PatchCollideFromGrid(&mut grid, pf);
    // expand by one unit for epsilon purposes
    (*pf).bounds[0 as libc::c_int as usize][0 as libc::c_int as usize] -=
        1 as libc::c_int as libc::c_float;
    (*pf).bounds[0 as libc::c_int as usize][1 as libc::c_int as usize] -=
        1 as libc::c_int as libc::c_float;
    (*pf).bounds[0 as libc::c_int as usize][2 as libc::c_int as usize] -=
        1 as libc::c_int as libc::c_float;
    (*pf).bounds[1 as libc::c_int as usize][0 as libc::c_int as usize] +=
        1 as libc::c_int as libc::c_float;
    (*pf).bounds[1 as libc::c_int as usize][1 as libc::c_int as usize] +=
        1 as libc::c_int as libc::c_float;
    (*pf).bounds[1 as libc::c_int as usize][2 as libc::c_int as usize] +=
        1 as libc::c_int as libc::c_float;
    return pf;
}
/*
================================================================================

TRACE TESTING

================================================================================
*/
/*
====================
CM_TracePointThroughPatchCollide

  special case for point traces because the patch collide "brushes" have no volume
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_TracePointThroughPatchCollide(
    mut tw: *mut crate::cm_local_h::traceWork_t,
    mut pc: *const crate::src::qcommon::cm_patch::patchCollide_s,
) {
    let mut frontFacing: [crate::src::qcommon::q_shared::qboolean; 2048] =
        [crate::src::qcommon::q_shared::qfalse; 2048];
    let mut intersection: [libc::c_float; 2048] = [0.; 2048];
    let mut intersect: libc::c_float = 0.;
    let mut planes_0: *const crate::src::qcommon::cm_patch::patchPlane_t =
        0 as *const crate::src::qcommon::cm_patch::patchPlane_t;
    let mut facet: *const crate::src::qcommon::cm_patch::facet_t =
        0 as *const crate::src::qcommon::cm_patch::facet_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut offset: libc::c_float = 0.;
    let mut d1: libc::c_float = 0.;
    let mut d2: libc::c_float = 0.;
    static mut cv: *mut crate::src::qcommon::q_shared::cvar_t = 0
        as *const crate::src::qcommon::q_shared::cvar_t
        as *mut crate::src::qcommon::q_shared::cvar_t;
    //BSPC
    if (*crate::src::qcommon::cm_load::cm_playerCurveClip).integer == 0 || (*tw).isPoint as u64 == 0
    {
        return;
    }
    // determine the trace's relationship to all planes
    planes_0 = (*pc).planes;
    i = 0 as libc::c_int;
    while i < (*pc).numPlanes {
        offset = (*tw).offsets[(*planes_0).signbits as usize][0 as libc::c_int as usize]
            * (*planes_0).plane[0 as libc::c_int as usize]
            + (*tw).offsets[(*planes_0).signbits as usize][1 as libc::c_int as usize]
                * (*planes_0).plane[1 as libc::c_int as usize]
            + (*tw).offsets[(*planes_0).signbits as usize][2 as libc::c_int as usize]
                * (*planes_0).plane[2 as libc::c_int as usize];
        d1 = (*tw).start[0 as libc::c_int as usize] * (*planes_0).plane[0 as libc::c_int as usize]
            + (*tw).start[1 as libc::c_int as usize] * (*planes_0).plane[1 as libc::c_int as usize]
            + (*tw).start[2 as libc::c_int as usize] * (*planes_0).plane[2 as libc::c_int as usize]
            - (*planes_0).plane[3 as libc::c_int as usize]
            + offset;
        d2 = (*tw).end[0 as libc::c_int as usize] * (*planes_0).plane[0 as libc::c_int as usize]
            + (*tw).end[1 as libc::c_int as usize] * (*planes_0).plane[1 as libc::c_int as usize]
            + (*tw).end[2 as libc::c_int as usize] * (*planes_0).plane[2 as libc::c_int as usize]
            - (*planes_0).plane[3 as libc::c_int as usize]
            + offset;
        if d1 <= 0 as libc::c_int as libc::c_float {
            frontFacing[i as usize] = crate::src::qcommon::q_shared::qfalse
        } else {
            frontFacing[i as usize] = crate::src::qcommon::q_shared::qtrue
        }
        if d1 == d2 {
            intersection[i as usize] = 99999 as libc::c_int as libc::c_float
        } else {
            intersection[i as usize] = d1 / (d1 - d2);
            if intersection[i as usize] <= 0 as libc::c_int as libc::c_float {
                intersection[i as usize] = 99999 as libc::c_int as libc::c_float
            }
        }
        i += 1;
        planes_0 = planes_0.offset(1)
    }
    // see if any of the surface planes are intersected
    facet = (*pc).facets;
    i = 0 as libc::c_int;
    while i < (*pc).numFacets {
        if !(frontFacing[(*facet).surfacePlane as usize] as u64 == 0) {
            intersect = intersection[(*facet).surfacePlane as usize];
            if !(intersect < 0 as libc::c_int as libc::c_float) {
                if !(intersect > (*tw).trace.fraction) {
                    j = 0 as libc::c_int;
                    while j < (*facet).numBorders {
                        k = (*facet).borderPlanes[j as usize];
                        if frontFacing[k as usize] as libc::c_uint
                            ^ (*facet).borderInward[j as usize] as libc::c_uint
                            != 0
                        {
                            if intersection[k as usize] > intersect {
                                break;
                            }
                        } else if intersection[k as usize] < intersect {
                            break;
                        }
                        j += 1
                    }
                    if j == (*facet).numBorders {
                        // we hit this facet
                        if cv.is_null() {
                            cv = crate::src::qcommon::cvar::Cvar_Get(
                                b"r_debugSurfaceUpdate\x00" as *const u8 as *const libc::c_char,
                                b"1\x00" as *const u8 as *const libc::c_char,
                                0 as libc::c_int,
                            )
                        }
                        if (*cv).integer != 0 {
                            debugPatchCollide = pc;
                            debugFacet = facet
                        }
                        //BSPC
                        planes_0 = &mut *(*pc).planes.offset((*facet).surfacePlane as isize)
                            as *mut crate::src::qcommon::cm_patch::patchPlane_t;
                        // calculate intersection with a slight pushoff
                        offset = (*tw).offsets[(*planes_0).signbits as usize]
                            [0 as libc::c_int as usize]
                            * (*planes_0).plane[0 as libc::c_int as usize]
                            + (*tw).offsets[(*planes_0).signbits as usize]
                                [1 as libc::c_int as usize]
                                * (*planes_0).plane[1 as libc::c_int as usize]
                            + (*tw).offsets[(*planes_0).signbits as usize]
                                [2 as libc::c_int as usize]
                                * (*planes_0).plane[2 as libc::c_int as usize];
                        d1 = (*tw).start[0 as libc::c_int as usize]
                            * (*planes_0).plane[0 as libc::c_int as usize]
                            + (*tw).start[1 as libc::c_int as usize]
                                * (*planes_0).plane[1 as libc::c_int as usize]
                            + (*tw).start[2 as libc::c_int as usize]
                                * (*planes_0).plane[2 as libc::c_int as usize]
                            - (*planes_0).plane[3 as libc::c_int as usize]
                            + offset;
                        d2 = (*tw).end[0 as libc::c_int as usize]
                            * (*planes_0).plane[0 as libc::c_int as usize]
                            + (*tw).end[1 as libc::c_int as usize]
                                * (*planes_0).plane[1 as libc::c_int as usize]
                            + (*tw).end[2 as libc::c_int as usize]
                                * (*planes_0).plane[2 as libc::c_int as usize]
                            - (*planes_0).plane[3 as libc::c_int as usize]
                            + offset;
                        (*tw).trace.fraction = ((d1 as libc::c_double - 0.125f64)
                            / (d1 - d2) as libc::c_double)
                            as libc::c_float;
                        if (*tw).trace.fraction < 0 as libc::c_int as libc::c_float {
                            (*tw).trace.fraction = 0 as libc::c_int as libc::c_float
                        }
                        (*tw).trace.plane.normal[0 as libc::c_int as usize] =
                            (*planes_0).plane[0 as libc::c_int as usize];
                        (*tw).trace.plane.normal[1 as libc::c_int as usize] =
                            (*planes_0).plane[1 as libc::c_int as usize];
                        (*tw).trace.plane.normal[2 as libc::c_int as usize] =
                            (*planes_0).plane[2 as libc::c_int as usize];
                        (*tw).trace.plane.dist = (*planes_0).plane[3 as libc::c_int as usize]
                    }
                }
            }
        }
        i += 1;
        facet = facet.offset(1)
        // already hit something closer
    }
}
/*
====================
CM_CheckFacetPlane
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_CheckFacetPlane(
    mut plane: *mut libc::c_float,
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut enterFrac: *mut libc::c_float,
    mut leaveFrac: *mut libc::c_float,
    mut hit: *mut libc::c_int,
) -> libc::c_int {
    let mut d1: libc::c_float = 0.;
    let mut d2: libc::c_float = 0.;
    let mut f: libc::c_float = 0.;
    *hit = crate::src::qcommon::q_shared::qfalse as libc::c_int;
    d1 = *start.offset(0 as libc::c_int as isize) * *plane.offset(0 as libc::c_int as isize)
        + *start.offset(1 as libc::c_int as isize) * *plane.offset(1 as libc::c_int as isize)
        + *start.offset(2 as libc::c_int as isize) * *plane.offset(2 as libc::c_int as isize)
        - *plane.offset(3 as libc::c_int as isize);
    d2 = *end.offset(0 as libc::c_int as isize) * *plane.offset(0 as libc::c_int as isize)
        + *end.offset(1 as libc::c_int as isize) * *plane.offset(1 as libc::c_int as isize)
        + *end.offset(2 as libc::c_int as isize) * *plane.offset(2 as libc::c_int as isize)
        - *plane.offset(3 as libc::c_int as isize);
    // if completely in front of face, no intersection with the entire facet
    if d1 > 0 as libc::c_int as libc::c_float && (d2 as libc::c_double >= 0.125f64 || d2 >= d1) {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    // if it doesn't cross the plane, the plane isn't relevant
    if d1 <= 0 as libc::c_int as libc::c_float && d2 <= 0 as libc::c_int as libc::c_float {
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    }
    // crosses face
    if d1 > d2 {
        f = ((d1 as libc::c_double - 0.125f64) / (d1 - d2) as libc::c_double) as libc::c_float; // leave
        if f < 0 as libc::c_int as libc::c_float {
            f = 0 as libc::c_int as libc::c_float
        }
        // enter
        //always favor previous plane hits and thus also the surface plane hit
        if f > *enterFrac {
            *enterFrac = f;
            *hit = crate::src::qcommon::q_shared::qtrue as libc::c_int
        }
    } else {
        f = ((d1 as libc::c_double + 0.125f64) / (d1 - d2) as libc::c_double) as libc::c_float;
        if f > 1 as libc::c_int as libc::c_float {
            f = 1 as libc::c_int as libc::c_float
        }
        if f < *leaveFrac {
            *leaveFrac = f
        }
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
/*
====================
CM_TraceThroughPatchCollide
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_TraceThroughPatchCollide(
    mut tw: *mut crate::cm_local_h::traceWork_t,
    mut pc: *const crate::src::qcommon::cm_patch::patchCollide_s,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut hit: libc::c_int = 0;
    let mut hitnum: libc::c_int = 0;
    let mut offset: libc::c_float = 0.;
    let mut enterFrac: libc::c_float = 0.;
    let mut leaveFrac: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut planes_0: *mut crate::src::qcommon::cm_patch::patchPlane_t =
        0 as *mut crate::src::qcommon::cm_patch::patchPlane_t;
    let mut facet: *mut crate::src::qcommon::cm_patch::facet_t =
        0 as *mut crate::src::qcommon::cm_patch::facet_t;
    let mut plane: [libc::c_float; 4] = [
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    ];
    let mut bestplane: [libc::c_float; 4] = [
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    ];
    let mut startp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut endp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    static mut cv: *mut crate::src::qcommon::q_shared::cvar_t = 0
        as *const crate::src::qcommon::q_shared::cvar_t
        as *mut crate::src::qcommon::q_shared::cvar_t;
    //BSPC
    if crate::src::qcommon::cm_test::CM_BoundsIntersect(
        (*tw).bounds[0 as libc::c_int as usize].as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        (*tw).bounds[1 as libc::c_int as usize].as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        (*pc).bounds[0 as libc::c_int as usize].as_ptr(),
        (*pc).bounds[1 as libc::c_int as usize].as_ptr(),
    ) as u64
        == 0
    {
        return;
    }
    if (*tw).isPoint as u64 != 0 {
        CM_TracePointThroughPatchCollide(tw, pc);
        return;
    }
    facet = (*pc).facets;
    i = 0 as libc::c_int;
    while i < (*pc).numFacets {
        enterFrac = -1.0f64 as libc::c_float;
        leaveFrac = 1.0f64 as libc::c_float;
        hitnum = -(1 as libc::c_int);
        //
        planes_0 = &mut *(*pc).planes.offset((*facet).surfacePlane as isize)
            as *mut crate::src::qcommon::cm_patch::patchPlane_t;
        plane[0 as libc::c_int as usize] = (*planes_0).plane[0 as libc::c_int as usize];
        plane[1 as libc::c_int as usize] = (*planes_0).plane[1 as libc::c_int as usize];
        plane[2 as libc::c_int as usize] = (*planes_0).plane[2 as libc::c_int as usize];
        plane[3 as libc::c_int as usize] = (*planes_0).plane[3 as libc::c_int as usize];
        if (*tw).sphere.use_0 as u64 != 0 {
            // adjust the plane distance appropriately for radius
            plane[3 as libc::c_int as usize] += (*tw).sphere.radius;
            // find the closest point on the capsule to the plane
            t = plane[0 as libc::c_int as usize] * (*tw).sphere.offset[0 as libc::c_int as usize]
                + plane[1 as libc::c_int as usize] * (*tw).sphere.offset[1 as libc::c_int as usize]
                + plane[2 as libc::c_int as usize] * (*tw).sphere.offset[2 as libc::c_int as usize];
            if t > 0.0f32 {
                startp[0 as libc::c_int as usize] = (*tw).start[0 as libc::c_int as usize]
                    - (*tw).sphere.offset[0 as libc::c_int as usize];
                startp[1 as libc::c_int as usize] = (*tw).start[1 as libc::c_int as usize]
                    - (*tw).sphere.offset[1 as libc::c_int as usize];
                startp[2 as libc::c_int as usize] = (*tw).start[2 as libc::c_int as usize]
                    - (*tw).sphere.offset[2 as libc::c_int as usize];
                endp[0 as libc::c_int as usize] = (*tw).end[0 as libc::c_int as usize]
                    - (*tw).sphere.offset[0 as libc::c_int as usize];
                endp[1 as libc::c_int as usize] = (*tw).end[1 as libc::c_int as usize]
                    - (*tw).sphere.offset[1 as libc::c_int as usize];
                endp[2 as libc::c_int as usize] = (*tw).end[2 as libc::c_int as usize]
                    - (*tw).sphere.offset[2 as libc::c_int as usize]
            } else {
                startp[0 as libc::c_int as usize] = (*tw).start[0 as libc::c_int as usize]
                    + (*tw).sphere.offset[0 as libc::c_int as usize];
                startp[1 as libc::c_int as usize] = (*tw).start[1 as libc::c_int as usize]
                    + (*tw).sphere.offset[1 as libc::c_int as usize];
                startp[2 as libc::c_int as usize] = (*tw).start[2 as libc::c_int as usize]
                    + (*tw).sphere.offset[2 as libc::c_int as usize];
                endp[0 as libc::c_int as usize] = (*tw).end[0 as libc::c_int as usize]
                    + (*tw).sphere.offset[0 as libc::c_int as usize];
                endp[1 as libc::c_int as usize] = (*tw).end[1 as libc::c_int as usize]
                    + (*tw).sphere.offset[1 as libc::c_int as usize];
                endp[2 as libc::c_int as usize] = (*tw).end[2 as libc::c_int as usize]
                    + (*tw).sphere.offset[2 as libc::c_int as usize]
            }
        } else {
            offset = (*tw).offsets[(*planes_0).signbits as usize][0 as libc::c_int as usize]
                * plane[0 as libc::c_int as usize]
                + (*tw).offsets[(*planes_0).signbits as usize][1 as libc::c_int as usize]
                    * plane[1 as libc::c_int as usize]
                + (*tw).offsets[(*planes_0).signbits as usize][2 as libc::c_int as usize]
                    * plane[2 as libc::c_int as usize];
            plane[3 as libc::c_int as usize] -= offset;
            startp[0 as libc::c_int as usize] = (*tw).start[0 as libc::c_int as usize];
            startp[1 as libc::c_int as usize] = (*tw).start[1 as libc::c_int as usize];
            startp[2 as libc::c_int as usize] = (*tw).start[2 as libc::c_int as usize];
            endp[0 as libc::c_int as usize] = (*tw).end[0 as libc::c_int as usize];
            endp[1 as libc::c_int as usize] = (*tw).end[1 as libc::c_int as usize];
            endp[2 as libc::c_int as usize] = (*tw).end[2 as libc::c_int as usize]
        }
        if !(CM_CheckFacetPlane(
            plane.as_mut_ptr(),
            startp.as_mut_ptr(),
            endp.as_mut_ptr(),
            &mut enterFrac,
            &mut leaveFrac,
            &mut hit,
        ) == 0)
        {
            if hit != 0 {
                bestplane[0 as libc::c_int as usize] = plane[0 as libc::c_int as usize];
                bestplane[1 as libc::c_int as usize] = plane[1 as libc::c_int as usize];
                bestplane[2 as libc::c_int as usize] = plane[2 as libc::c_int as usize];
                bestplane[3 as libc::c_int as usize] = plane[3 as libc::c_int as usize]
            }
            j = 0 as libc::c_int;
            while j < (*facet).numBorders {
                planes_0 = &mut *(*pc)
                    .planes
                    .offset(*(*facet).borderPlanes.as_mut_ptr().offset(j as isize) as isize)
                    as *mut crate::src::qcommon::cm_patch::patchPlane_t;
                if (*facet).borderInward[j as usize] != 0 {
                    plane[0 as libc::c_int as usize] =
                        -(*planes_0).plane[0 as libc::c_int as usize];
                    plane[1 as libc::c_int as usize] =
                        -(*planes_0).plane[1 as libc::c_int as usize];
                    plane[2 as libc::c_int as usize] =
                        -(*planes_0).plane[2 as libc::c_int as usize];
                    plane[3 as libc::c_int as usize] = -(*planes_0).plane[3 as libc::c_int as usize]
                } else {
                    plane[0 as libc::c_int as usize] = (*planes_0).plane[0 as libc::c_int as usize];
                    plane[1 as libc::c_int as usize] = (*planes_0).plane[1 as libc::c_int as usize];
                    plane[2 as libc::c_int as usize] = (*planes_0).plane[2 as libc::c_int as usize];
                    plane[3 as libc::c_int as usize] = (*planes_0).plane[3 as libc::c_int as usize]
                }
                if (*tw).sphere.use_0 as u64 != 0 {
                    // adjust the plane distance appropriately for radius
                    plane[3 as libc::c_int as usize] += (*tw).sphere.radius;
                    // find the closest point on the capsule to the plane
                    t = plane[0 as libc::c_int as usize]
                        * (*tw).sphere.offset[0 as libc::c_int as usize]
                        + plane[1 as libc::c_int as usize]
                            * (*tw).sphere.offset[1 as libc::c_int as usize]
                        + plane[2 as libc::c_int as usize]
                            * (*tw).sphere.offset[2 as libc::c_int as usize];
                    if t > 0.0f32 {
                        startp[0 as libc::c_int as usize] = (*tw).start[0 as libc::c_int as usize]
                            - (*tw).sphere.offset[0 as libc::c_int as usize];
                        startp[1 as libc::c_int as usize] = (*tw).start[1 as libc::c_int as usize]
                            - (*tw).sphere.offset[1 as libc::c_int as usize];
                        startp[2 as libc::c_int as usize] = (*tw).start[2 as libc::c_int as usize]
                            - (*tw).sphere.offset[2 as libc::c_int as usize];
                        endp[0 as libc::c_int as usize] = (*tw).end[0 as libc::c_int as usize]
                            - (*tw).sphere.offset[0 as libc::c_int as usize];
                        endp[1 as libc::c_int as usize] = (*tw).end[1 as libc::c_int as usize]
                            - (*tw).sphere.offset[1 as libc::c_int as usize];
                        endp[2 as libc::c_int as usize] = (*tw).end[2 as libc::c_int as usize]
                            - (*tw).sphere.offset[2 as libc::c_int as usize]
                    } else {
                        startp[0 as libc::c_int as usize] = (*tw).start[0 as libc::c_int as usize]
                            + (*tw).sphere.offset[0 as libc::c_int as usize];
                        startp[1 as libc::c_int as usize] = (*tw).start[1 as libc::c_int as usize]
                            + (*tw).sphere.offset[1 as libc::c_int as usize];
                        startp[2 as libc::c_int as usize] = (*tw).start[2 as libc::c_int as usize]
                            + (*tw).sphere.offset[2 as libc::c_int as usize];
                        endp[0 as libc::c_int as usize] = (*tw).end[0 as libc::c_int as usize]
                            + (*tw).sphere.offset[0 as libc::c_int as usize];
                        endp[1 as libc::c_int as usize] = (*tw).end[1 as libc::c_int as usize]
                            + (*tw).sphere.offset[1 as libc::c_int as usize];
                        endp[2 as libc::c_int as usize] = (*tw).end[2 as libc::c_int as usize]
                            + (*tw).sphere.offset[2 as libc::c_int as usize]
                    }
                } else {
                    // NOTE: this works even though the plane might be flipped because the bbox is centered
                    offset = (*tw).offsets[(*planes_0).signbits as usize]
                        [0 as libc::c_int as usize]
                        * plane[0 as libc::c_int as usize]
                        + (*tw).offsets[(*planes_0).signbits as usize][1 as libc::c_int as usize]
                            * plane[1 as libc::c_int as usize]
                        + (*tw).offsets[(*planes_0).signbits as usize][2 as libc::c_int as usize]
                            * plane[2 as libc::c_int as usize];
                    plane[3 as libc::c_int as usize] = (plane[3 as libc::c_int as usize]
                        as libc::c_double
                        + crate::stdlib::fabs(offset as libc::c_double))
                        as libc::c_float;
                    startp[0 as libc::c_int as usize] = (*tw).start[0 as libc::c_int as usize];
                    startp[1 as libc::c_int as usize] = (*tw).start[1 as libc::c_int as usize];
                    startp[2 as libc::c_int as usize] = (*tw).start[2 as libc::c_int as usize];
                    endp[0 as libc::c_int as usize] = (*tw).end[0 as libc::c_int as usize];
                    endp[1 as libc::c_int as usize] = (*tw).end[1 as libc::c_int as usize];
                    endp[2 as libc::c_int as usize] = (*tw).end[2 as libc::c_int as usize]
                }
                if CM_CheckFacetPlane(
                    plane.as_mut_ptr(),
                    startp.as_mut_ptr(),
                    endp.as_mut_ptr(),
                    &mut enterFrac,
                    &mut leaveFrac,
                    &mut hit,
                ) == 0
                {
                    break;
                }
                if hit != 0 {
                    hitnum = j;
                    bestplane[0 as libc::c_int as usize] = plane[0 as libc::c_int as usize];
                    bestplane[1 as libc::c_int as usize] = plane[1 as libc::c_int as usize];
                    bestplane[2 as libc::c_int as usize] = plane[2 as libc::c_int as usize];
                    bestplane[3 as libc::c_int as usize] = plane[3 as libc::c_int as usize]
                }
                j += 1
            }
            if !(j < (*facet).numBorders) {
                //never clip against the back side
                if !(hitnum == (*facet).numBorders - 1 as libc::c_int) {
                    if enterFrac < leaveFrac && enterFrac >= 0 as libc::c_int as libc::c_float {
                        if enterFrac < (*tw).trace.fraction {
                            if enterFrac < 0 as libc::c_int as libc::c_float {
                                enterFrac = 0 as libc::c_int as libc::c_float
                            }
                            if cv.is_null() {
                                cv = crate::src::qcommon::cvar::Cvar_Get(
                                    b"r_debugSurfaceUpdate\x00" as *const u8 as *const libc::c_char,
                                    b"1\x00" as *const u8 as *const libc::c_char,
                                    0 as libc::c_int,
                                )
                            }
                            if !cv.is_null() && (*cv).integer != 0 {
                                debugPatchCollide = pc;
                                debugFacet = facet
                            }
                            //BSPC
                            (*tw).trace.fraction = enterFrac;
                            (*tw).trace.plane.normal[0 as libc::c_int as usize] =
                                bestplane[0 as libc::c_int as usize];
                            (*tw).trace.plane.normal[1 as libc::c_int as usize] =
                                bestplane[1 as libc::c_int as usize];
                            (*tw).trace.plane.normal[2 as libc::c_int as usize] =
                                bestplane[2 as libc::c_int as usize];
                            (*tw).trace.plane.dist = bestplane[3 as libc::c_int as usize]
                        }
                    }
                }
            }
        }
        i += 1;
        facet = facet.offset(1)
    }
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
// negative numbers are leafs
// submodels don't reference the main tree
// the shader that determined the contents
// to avoid repeated testings
// to avoid repeated testings
// if false, visibility is just a single cluster of ffs
// [ numAreas*numAreas ] reference counts
// non-patches will be NULL
// incremented on each trace
// keep 1/8 unit away to keep the position valid before network snapping
// and to avoid various numeric issues
// cm_test.c
// Used for oriented capsule collision detection
// size of the box being swept through the model
// [signbits][x] = either size[0][x] or size[1][x]
// longest corner length from origin
// greatest of abs(size[0]) and abs(size[1])
// enclosing box of start and end surrounding by size
// origin of the model tracing through
// ored contents of the model tracing through
// optimized case
// returned from trace call
// sphere for oriendted capsule collision
// for overflows where each leaf can't be stored individually
// cm_patch.c
/*
=======================================================================

POSITION TEST

=======================================================================
*/
/*
====================
CM_PositionTestInPatchCollide
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_PositionTestInPatchCollide(
    mut tw: *mut crate::cm_local_h::traceWork_t,
    mut pc: *const crate::src::qcommon::cm_patch::patchCollide_s,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut offset: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut planes_0: *mut crate::src::qcommon::cm_patch::patchPlane_t =
        0 as *mut crate::src::qcommon::cm_patch::patchPlane_t;
    let mut facet: *mut crate::src::qcommon::cm_patch::facet_t =
        0 as *mut crate::src::qcommon::cm_patch::facet_t;
    let mut plane: [libc::c_float; 4] = [0.; 4];
    let mut startp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if (*tw).isPoint as u64 != 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    //
    facet = (*pc).facets;
    i = 0 as libc::c_int;
    while i < (*pc).numFacets {
        planes_0 = &mut *(*pc).planes.offset((*facet).surfacePlane as isize)
            as *mut crate::src::qcommon::cm_patch::patchPlane_t;
        plane[0 as libc::c_int as usize] = (*planes_0).plane[0 as libc::c_int as usize];
        plane[1 as libc::c_int as usize] = (*planes_0).plane[1 as libc::c_int as usize];
        plane[2 as libc::c_int as usize] = (*planes_0).plane[2 as libc::c_int as usize];
        plane[3 as libc::c_int as usize] = (*planes_0).plane[3 as libc::c_int as usize];
        if (*tw).sphere.use_0 as u64 != 0 {
            // adjust the plane distance appropriately for radius
            plane[3 as libc::c_int as usize] += (*tw).sphere.radius;
            // find the closest point on the capsule to the plane
            t = plane[0 as libc::c_int as usize] * (*tw).sphere.offset[0 as libc::c_int as usize]
                + plane[1 as libc::c_int as usize] * (*tw).sphere.offset[1 as libc::c_int as usize]
                + plane[2 as libc::c_int as usize] * (*tw).sphere.offset[2 as libc::c_int as usize];
            if t > 0 as libc::c_int as libc::c_float {
                startp[0 as libc::c_int as usize] = (*tw).start[0 as libc::c_int as usize]
                    - (*tw).sphere.offset[0 as libc::c_int as usize];
                startp[1 as libc::c_int as usize] = (*tw).start[1 as libc::c_int as usize]
                    - (*tw).sphere.offset[1 as libc::c_int as usize];
                startp[2 as libc::c_int as usize] = (*tw).start[2 as libc::c_int as usize]
                    - (*tw).sphere.offset[2 as libc::c_int as usize]
            } else {
                startp[0 as libc::c_int as usize] = (*tw).start[0 as libc::c_int as usize]
                    + (*tw).sphere.offset[0 as libc::c_int as usize];
                startp[1 as libc::c_int as usize] = (*tw).start[1 as libc::c_int as usize]
                    + (*tw).sphere.offset[1 as libc::c_int as usize];
                startp[2 as libc::c_int as usize] = (*tw).start[2 as libc::c_int as usize]
                    + (*tw).sphere.offset[2 as libc::c_int as usize]
            }
        } else {
            offset = (*tw).offsets[(*planes_0).signbits as usize][0 as libc::c_int as usize]
                * plane[0 as libc::c_int as usize]
                + (*tw).offsets[(*planes_0).signbits as usize][1 as libc::c_int as usize]
                    * plane[1 as libc::c_int as usize]
                + (*tw).offsets[(*planes_0).signbits as usize][2 as libc::c_int as usize]
                    * plane[2 as libc::c_int as usize];
            plane[3 as libc::c_int as usize] -= offset;
            startp[0 as libc::c_int as usize] = (*tw).start[0 as libc::c_int as usize];
            startp[1 as libc::c_int as usize] = (*tw).start[1 as libc::c_int as usize];
            startp[2 as libc::c_int as usize] = (*tw).start[2 as libc::c_int as usize]
        }
        if !(plane[0 as libc::c_int as usize] * startp[0 as libc::c_int as usize]
            + plane[1 as libc::c_int as usize] * startp[1 as libc::c_int as usize]
            + plane[2 as libc::c_int as usize] * startp[2 as libc::c_int as usize]
            - plane[3 as libc::c_int as usize]
            > 0.0f32)
        {
            j = 0 as libc::c_int;
            while j < (*facet).numBorders {
                planes_0 = &mut *(*pc)
                    .planes
                    .offset(*(*facet).borderPlanes.as_mut_ptr().offset(j as isize) as isize)
                    as *mut crate::src::qcommon::cm_patch::patchPlane_t;
                if (*facet).borderInward[j as usize] != 0 {
                    plane[0 as libc::c_int as usize] =
                        -(*planes_0).plane[0 as libc::c_int as usize];
                    plane[1 as libc::c_int as usize] =
                        -(*planes_0).plane[1 as libc::c_int as usize];
                    plane[2 as libc::c_int as usize] =
                        -(*planes_0).plane[2 as libc::c_int as usize];
                    plane[3 as libc::c_int as usize] = -(*planes_0).plane[3 as libc::c_int as usize]
                } else {
                    plane[0 as libc::c_int as usize] = (*planes_0).plane[0 as libc::c_int as usize];
                    plane[1 as libc::c_int as usize] = (*planes_0).plane[1 as libc::c_int as usize];
                    plane[2 as libc::c_int as usize] = (*planes_0).plane[2 as libc::c_int as usize];
                    plane[3 as libc::c_int as usize] = (*planes_0).plane[3 as libc::c_int as usize]
                }
                if (*tw).sphere.use_0 as u64 != 0 {
                    // adjust the plane distance appropriately for radius
                    plane[3 as libc::c_int as usize] += (*tw).sphere.radius;
                    // find the closest point on the capsule to the plane
                    t = plane[0 as libc::c_int as usize]
                        * (*tw).sphere.offset[0 as libc::c_int as usize]
                        + plane[1 as libc::c_int as usize]
                            * (*tw).sphere.offset[1 as libc::c_int as usize]
                        + plane[2 as libc::c_int as usize]
                            * (*tw).sphere.offset[2 as libc::c_int as usize];
                    if t > 0.0f32 {
                        startp[0 as libc::c_int as usize] = (*tw).start[0 as libc::c_int as usize]
                            - (*tw).sphere.offset[0 as libc::c_int as usize];
                        startp[1 as libc::c_int as usize] = (*tw).start[1 as libc::c_int as usize]
                            - (*tw).sphere.offset[1 as libc::c_int as usize];
                        startp[2 as libc::c_int as usize] = (*tw).start[2 as libc::c_int as usize]
                            - (*tw).sphere.offset[2 as libc::c_int as usize]
                    } else {
                        startp[0 as libc::c_int as usize] = (*tw).start[0 as libc::c_int as usize]
                            + (*tw).sphere.offset[0 as libc::c_int as usize];
                        startp[1 as libc::c_int as usize] = (*tw).start[1 as libc::c_int as usize]
                            + (*tw).sphere.offset[1 as libc::c_int as usize];
                        startp[2 as libc::c_int as usize] = (*tw).start[2 as libc::c_int as usize]
                            + (*tw).sphere.offset[2 as libc::c_int as usize]
                    }
                } else {
                    // NOTE: this works even though the plane might be flipped because the bbox is centered
                    offset = (*tw).offsets[(*planes_0).signbits as usize]
                        [0 as libc::c_int as usize]
                        * plane[0 as libc::c_int as usize]
                        + (*tw).offsets[(*planes_0).signbits as usize][1 as libc::c_int as usize]
                            * plane[1 as libc::c_int as usize]
                        + (*tw).offsets[(*planes_0).signbits as usize][2 as libc::c_int as usize]
                            * plane[2 as libc::c_int as usize];
                    plane[3 as libc::c_int as usize] = (plane[3 as libc::c_int as usize]
                        as libc::c_double
                        + crate::stdlib::fabs(offset as libc::c_double))
                        as libc::c_float;
                    startp[0 as libc::c_int as usize] = (*tw).start[0 as libc::c_int as usize];
                    startp[1 as libc::c_int as usize] = (*tw).start[1 as libc::c_int as usize];
                    startp[2 as libc::c_int as usize] = (*tw).start[2 as libc::c_int as usize]
                }
                if plane[0 as libc::c_int as usize] * startp[0 as libc::c_int as usize]
                    + plane[1 as libc::c_int as usize] * startp[1 as libc::c_int as usize]
                    + plane[2 as libc::c_int as usize] * startp[2 as libc::c_int as usize]
                    - plane[3 as libc::c_int as usize]
                    > 0.0f32
                {
                    break;
                }
                j += 1
            }
            if !(j < (*facet).numBorders) {
                // inside this patch facet
                return crate::src::qcommon::q_shared::qtrue;
            }
        }
        i += 1;
        facet = facet.offset(1)
    }
    return crate::src::qcommon::q_shared::qfalse;
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
// 0 = world, 1 + are bmodels
// returns an ORed contents mask
// only returns non-solid leafs
// overflow if return listsize and if *lastLeaf != list[listsize-1]
// cm_patch.c
#[no_mangle]

pub unsafe extern "C" fn CM_DrawDebugSurface(
    mut drawPoly: Option<
        unsafe extern "C" fn(_: libc::c_int, _: libc::c_int, _: *mut libc::c_float) -> (),
    >,
) {
    static mut cv: *mut crate::src::qcommon::q_shared::cvar_t = 0
        as *const crate::src::qcommon::q_shared::cvar_t
        as *mut crate::src::qcommon::q_shared::cvar_t;
    static mut cv2: *mut crate::src::qcommon::q_shared::cvar_t = 0
        as *const crate::src::qcommon::q_shared::cvar_t
        as *mut crate::src::qcommon::q_shared::cvar_t;
    let mut pc: *const crate::src::qcommon::cm_patch::patchCollide_t =
        0 as *const crate::src::qcommon::cm_patch::patchCollide_t;
    let mut facet: *mut crate::src::qcommon::cm_patch::facet_t =
        0 as *mut crate::src::qcommon::cm_patch::facet_t;
    let mut w: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut curplanenum: libc::c_int = 0;
    let mut planenum: libc::c_int = 0;
    let mut curinward: libc::c_int = 0;
    let mut inward: libc::c_int = 0;
    let mut plane: [libc::c_float; 4] = [0.; 4];
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [
        -(15 as libc::c_int) as crate::src::qcommon::q_shared::vec_t,
        -(15 as libc::c_int) as crate::src::qcommon::q_shared::vec_t,
        -(28 as libc::c_int) as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [
        15 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        15 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        28 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ];
    //vec3_t mins = {0, 0, 0}, maxs = {0, 0, 0};
    let mut v1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if cv2.is_null() {
        cv2 = crate::src::qcommon::cvar::Cvar_Get(
            b"r_debugSurface\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        )
    }
    if (*cv2).integer != 1 as libc::c_int {
        BotDrawDebugPolygons(drawPoly, (*cv2).integer);
        return;
    }
    if debugPatchCollide.is_null() {
        return;
    }
    if cv.is_null() {
        cv = crate::src::qcommon::cvar::Cvar_Get(
            b"cm_debugSize\x00" as *const u8 as *const libc::c_char,
            b"2\x00" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        )
    }
    pc = debugPatchCollide;
    i = 0 as libc::c_int;
    facet = (*pc).facets;
    while i < (*pc).numFacets {
        k = 0 as libc::c_int;
        while k < (*facet).numBorders + 1 as libc::c_int {
            //
            if k < (*facet).numBorders {
                planenum = (*facet).borderPlanes[k as usize];
                inward = (*facet).borderInward[k as usize]
            } else {
                planenum = (*facet).surfacePlane;
                inward = crate::src::qcommon::q_shared::qfalse as libc::c_int
                //continue;
            }
            plane[0 as libc::c_int as usize] =
                (*(*pc).planes.offset(planenum as isize)).plane[0 as libc::c_int as usize];
            plane[1 as libc::c_int as usize] =
                (*(*pc).planes.offset(planenum as isize)).plane[1 as libc::c_int as usize];
            plane[2 as libc::c_int as usize] =
                (*(*pc).planes.offset(planenum as isize)).plane[2 as libc::c_int as usize];
            plane[3 as libc::c_int as usize] =
                (*(*pc).planes.offset(planenum as isize)).plane[3 as libc::c_int as usize];
            //planenum = facet->surfacePlane;
            if inward != 0 {
                plane[0 as libc::c_int as usize] = crate::src::qcommon::q_math::vec3_origin
                    [0 as libc::c_int as usize]
                    - plane[0 as libc::c_int as usize];
                plane[1 as libc::c_int as usize] = crate::src::qcommon::q_math::vec3_origin
                    [1 as libc::c_int as usize]
                    - plane[1 as libc::c_int as usize];
                plane[2 as libc::c_int as usize] = crate::src::qcommon::q_math::vec3_origin
                    [2 as libc::c_int as usize]
                    - plane[2 as libc::c_int as usize];
                plane[3 as libc::c_int as usize] = -plane[3 as libc::c_int as usize]
            }
            plane[3 as libc::c_int as usize] += (*cv).value;
            //*
            n = 0 as libc::c_int; //end for
            while n < 3 as libc::c_int {
                if plane[n as usize] > 0 as libc::c_int as libc::c_float {
                    v1[n as usize] = maxs[n as usize]
                } else {
                    v1[n as usize] = mins[n as usize]
                }
                n += 1
            }
            v2[0 as libc::c_int as usize] = -plane[0 as libc::c_int as usize];
            v2[1 as libc::c_int as usize] = -plane[1 as libc::c_int as usize];
            v2[2 as libc::c_int as usize] = -plane[2 as libc::c_int as usize];
            plane[3 as libc::c_int as usize] = (plane[3 as libc::c_int as usize] as libc::c_double
                + crate::stdlib::fabs(
                    (v1[0 as libc::c_int as usize] * v2[0 as libc::c_int as usize]
                        + v1[1 as libc::c_int as usize] * v2[1 as libc::c_int as usize]
                        + v1[2 as libc::c_int as usize] * v2[2 as libc::c_int as usize])
                        as libc::c_double,
                )) as libc::c_float;
            //*/
            w = crate::src::qcommon::cm_polylib::BaseWindingForPlane(
                plane.as_mut_ptr(),
                plane[3 as libc::c_int as usize],
            );
            j = 0 as libc::c_int;
            while j < (*facet).numBorders + 1 as libc::c_int && !w.is_null() {
                //
                if j < (*facet).numBorders {
                    curplanenum = (*facet).borderPlanes[j as usize];
                    curinward = (*facet).borderInward[j as usize]
                } else {
                    curplanenum = (*facet).surfacePlane;
                    curinward = crate::src::qcommon::q_shared::qfalse as libc::c_int
                    //continue;
                }
                //
                if !(curplanenum == planenum) {
                    plane[0 as libc::c_int as usize] = (*(*pc).planes.offset(curplanenum as isize))
                        .plane[0 as libc::c_int as usize];
                    plane[1 as libc::c_int as usize] = (*(*pc).planes.offset(curplanenum as isize))
                        .plane[1 as libc::c_int as usize];
                    plane[2 as libc::c_int as usize] = (*(*pc).planes.offset(curplanenum as isize))
                        .plane[2 as libc::c_int as usize];
                    plane[3 as libc::c_int as usize] = (*(*pc).planes.offset(curplanenum as isize))
                        .plane[3 as libc::c_int as usize];
                    if curinward == 0 {
                        plane[0 as libc::c_int as usize] = crate::src::qcommon::q_math::vec3_origin
                            [0 as libc::c_int as usize]
                            - plane[0 as libc::c_int as usize];
                        plane[1 as libc::c_int as usize] = crate::src::qcommon::q_math::vec3_origin
                            [1 as libc::c_int as usize]
                            - plane[1 as libc::c_int as usize];
                        plane[2 as libc::c_int as usize] = crate::src::qcommon::q_math::vec3_origin
                            [2 as libc::c_int as usize]
                            - plane[2 as libc::c_int as usize];
                        plane[3 as libc::c_int as usize] = -plane[3 as libc::c_int as usize]
                    }
                    //			if ( !facet->borderNoAdjust[j] ) {
                    plane[3 as libc::c_int as usize] -= (*cv).value;
                    //			}
                    n = 0 as libc::c_int; //end for
                    while n < 3 as libc::c_int {
                        if plane[n as usize] > 0 as libc::c_int as libc::c_float {
                            v1[n as usize] = maxs[n as usize]
                        } else {
                            v1[n as usize] = mins[n as usize]
                        }
                        n += 1
                    }
                    v2[0 as libc::c_int as usize] = -plane[0 as libc::c_int as usize];
                    v2[1 as libc::c_int as usize] = -plane[1 as libc::c_int as usize];
                    v2[2 as libc::c_int as usize] = -plane[2 as libc::c_int as usize];
                    plane[3 as libc::c_int as usize] = (plane[3 as libc::c_int as usize]
                        as libc::c_double
                        - crate::stdlib::fabs(
                            (v1[0 as libc::c_int as usize] * v2[0 as libc::c_int as usize]
                                + v1[1 as libc::c_int as usize] * v2[1 as libc::c_int as usize]
                                + v1[2 as libc::c_int as usize] * v2[2 as libc::c_int as usize])
                                as libc::c_double,
                        )) as libc::c_float;
                    crate::src::qcommon::cm_polylib::ChopWindingInPlace(
                        &mut w,
                        plane.as_mut_ptr(),
                        plane[3 as libc::c_int as usize],
                        0.1f32,
                    );
                }
                j += 1
            }
            if !w.is_null() {
                if facet == debugFacet as *mut crate::src::qcommon::cm_patch::facet_t {
                    drawPoly.expect("non-null function pointer")(
                        4 as libc::c_int,
                        (*w).numpoints,
                        (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize)).as_mut_ptr(),
                    );
                //Com_Printf("blue facet has %d border planes\n", facet->numBorders);
                } else {
                    drawPoly.expect("non-null function pointer")(
                        1 as libc::c_int,
                        (*w).numpoints,
                        (*(*w).p.as_mut_ptr().offset(0 as libc::c_int as isize)).as_mut_ptr(),
                    );
                }
                crate::src::qcommon::cm_polylib::FreeWinding(w);
            } else {
                crate::src::qcommon::common::Com_Printf(
                    b"winding chopped away by border planes\n\x00" as *const u8
                        as *const libc::c_char,
                );
            }
            k += 1
        }
        i += 1;
        facet = facet.offset(1)
    }
    // draw the debug block
    let mut v: [crate::src::qcommon::q_shared::vec3_t; 3] = [[0.; 3]; 3];
    v[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        debugBlockPoints[0 as libc::c_int as usize][0 as libc::c_int as usize];
    v[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        debugBlockPoints[0 as libc::c_int as usize][1 as libc::c_int as usize];
    v[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        debugBlockPoints[0 as libc::c_int as usize][2 as libc::c_int as usize];
    v[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        debugBlockPoints[1 as libc::c_int as usize][0 as libc::c_int as usize];
    v[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        debugBlockPoints[1 as libc::c_int as usize][1 as libc::c_int as usize];
    v[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        debugBlockPoints[1 as libc::c_int as usize][2 as libc::c_int as usize];
    v[2 as libc::c_int as usize][0 as libc::c_int as usize] =
        debugBlockPoints[2 as libc::c_int as usize][0 as libc::c_int as usize];
    v[2 as libc::c_int as usize][1 as libc::c_int as usize] =
        debugBlockPoints[2 as libc::c_int as usize][1 as libc::c_int as usize];
    v[2 as libc::c_int as usize][2 as libc::c_int as usize] =
        debugBlockPoints[2 as libc::c_int as usize][2 as libc::c_int as usize];
    drawPoly.expect("non-null function pointer")(
        2 as libc::c_int,
        3 as libc::c_int,
        v[0 as libc::c_int as usize].as_mut_ptr(),
    );
    v[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        debugBlockPoints[2 as libc::c_int as usize][0 as libc::c_int as usize];
    v[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        debugBlockPoints[2 as libc::c_int as usize][1 as libc::c_int as usize];
    v[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        debugBlockPoints[2 as libc::c_int as usize][2 as libc::c_int as usize];
    v[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        debugBlockPoints[3 as libc::c_int as usize][0 as libc::c_int as usize];
    v[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        debugBlockPoints[3 as libc::c_int as usize][1 as libc::c_int as usize];
    v[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        debugBlockPoints[3 as libc::c_int as usize][2 as libc::c_int as usize];
    v[2 as libc::c_int as usize][0 as libc::c_int as usize] =
        debugBlockPoints[0 as libc::c_int as usize][0 as libc::c_int as usize];
    v[2 as libc::c_int as usize][1 as libc::c_int as usize] =
        debugBlockPoints[0 as libc::c_int as usize][1 as libc::c_int as usize];
    v[2 as libc::c_int as usize][2 as libc::c_int as usize] =
        debugBlockPoints[0 as libc::c_int as usize][2 as libc::c_int as usize];
    drawPoly.expect("non-null function pointer")(
        2 as libc::c_int,
        3 as libc::c_int,
        v[0 as libc::c_int as usize].as_mut_ptr(),
    );
}
