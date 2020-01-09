use ::libc;

pub mod q_shared_h {

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

    pub unsafe extern "C" fn VectorInverse(mut v: *mut crate::src::qcommon::q_shared::vec_t) {
        *v.offset(0 as libc::c_int as isize) = -*v.offset(0 as libc::c_int as isize);
        *v.offset(1 as libc::c_int as isize) = -*v.offset(1 as libc::c_int as isize);
        *v.offset(2 as libc::c_int as isize) = -*v.offset(2 as libc::c_int as isize);
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

pub use crate::aasfile_h::aas_area_s;
pub use crate::aasfile_h::aas_area_t;
pub use crate::aasfile_h::aas_areasettings_s;
pub use crate::aasfile_h::aas_areasettings_t;
pub use crate::aasfile_h::aas_bbox_s;
pub use crate::aasfile_h::aas_bbox_t;
pub use crate::aasfile_h::aas_cluster_s;
pub use crate::aasfile_h::aas_cluster_t;
pub use crate::aasfile_h::aas_edge_s;
pub use crate::aasfile_h::aas_edge_t;
pub use crate::aasfile_h::aas_edgeindex_t;
pub use crate::aasfile_h::aas_face_s;
pub use crate::aasfile_h::aas_face_t;
pub use crate::aasfile_h::aas_faceindex_t;
pub use crate::aasfile_h::aas_node_s;
pub use crate::aasfile_h::aas_node_t;
pub use crate::aasfile_h::aas_plane_s;
pub use crate::aasfile_h::aas_plane_t;
pub use crate::aasfile_h::aas_portal_s;
pub use crate::aasfile_h::aas_portal_t;
pub use crate::aasfile_h::aas_portalindex_t;
pub use crate::aasfile_h::aas_reachability_s;
pub use crate::aasfile_h::aas_reachability_t;
pub use crate::aasfile_h::aas_vertex_t;
pub use crate::be_aas_def_h::aas_entity_s;
pub use crate::be_aas_def_h::aas_entity_t;
pub use crate::be_aas_def_h::aas_link_s;
pub use crate::be_aas_def_h::aas_link_t;
pub use crate::be_aas_def_h::aas_reachabilityareas_s;
pub use crate::be_aas_def_h::aas_reachabilityareas_t;
pub use crate::be_aas_def_h::aas_reversedlink_s;
pub use crate::be_aas_def_h::aas_reversedlink_t;
pub use crate::be_aas_def_h::aas_reversedreachability_s;
pub use crate::be_aas_def_h::aas_reversedreachability_t;
pub use crate::be_aas_def_h::aas_routingcache_s;
pub use crate::be_aas_def_h::aas_routingcache_t;
pub use crate::be_aas_def_h::aas_routingupdate_s;
pub use crate::be_aas_def_h::aas_routingupdate_t;
pub use crate::be_aas_def_h::aas_s;
pub use crate::be_aas_def_h::aas_settings_s;
pub use crate::be_aas_def_h::aas_settings_t;
pub use crate::be_aas_def_h::aas_t;
pub use crate::be_aas_def_h::bsp_link_s;
pub use crate::be_aas_def_h::bsp_link_t;
pub use crate::be_aas_h::aas_clientmove_s;
pub use crate::be_aas_h::aas_clientmove_t;
pub use crate::be_aas_h::aas_entityinfo_s;
pub use crate::be_aas_h::aas_entityinfo_t;
pub use crate::be_aas_h::aas_trace_s;
pub use crate::be_aas_h::aas_trace_t;
pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
pub use crate::src::botlib::be_aas_reach::q_shared_h::CrossProduct;
pub use crate::src::botlib::be_aas_reach::q_shared_h::VectorInverse;
pub use crate::src::botlib::be_aas_reach::q_shared_h::VectorLength;
use crate::src::botlib::l_log::Log_Write;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;
use crate::stdlib::abs;
use crate::stdlib::atoi;
use crate::stdlib::fabs;
use crate::stdlib::fabsf;
use crate::stdlib::memset;
use crate::stdlib::sqrt;
use crate::stdlib::strcmp;
use crate::stdlib::tan;

use crate::src::botlib::be_aas_debug::AAS_PermanentLine;
use crate::src::botlib::l_libvar::LibVarGetValue;
use crate::src::botlib::l_libvar::LibVarValue;
use crate::src::botlib::l_memory::FreeMemory;
use crate::src::botlib::l_memory::GetClearedMemory;

use crate::src::botlib::be_aas_bspq3::AAS_BSPModelMinsMaxsOrigin;
use crate::src::botlib::be_aas_bspq3::AAS_FloatForBSPEpairKey;
use crate::src::botlib::be_aas_bspq3::AAS_IntForBSPEpairKey;
use crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity;
use crate::src::botlib::be_aas_bspq3::AAS_PointContents;
use crate::src::botlib::be_aas_bspq3::AAS_Trace;
use crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey;
use crate::src::botlib::be_aas_bspq3::AAS_VectorForBSPEpairKey;
use crate::src::botlib::be_aas_main::aasworld;
use crate::src::botlib::be_aas_main::AAS_Error;
use crate::src::botlib::be_aas_move::aassettings;
use crate::src::botlib::be_aas_move::AAS_BFGJumpZVelocity;
use crate::src::botlib::be_aas_move::AAS_ClientMovementHitBBox;
use crate::src::botlib::be_aas_move::AAS_DropToFloor;
use crate::src::botlib::be_aas_move::AAS_HorizontalVelocityForJump;
use crate::src::botlib::be_aas_move::AAS_PredictClientMovement;
use crate::src::botlib::be_aas_move::AAS_RocketJumpZVelocity;
use crate::src::botlib::be_aas_sample::AAS_AreaPresenceType;
use crate::src::botlib::be_aas_sample::AAS_LinkEntityClientBBox;
use crate::src::botlib::be_aas_sample::AAS_PointAreaNum;
use crate::src::botlib::be_aas_sample::AAS_PointInsideFace;
use crate::src::botlib::be_aas_sample::AAS_TraceAreas;
use crate::src::botlib::be_aas_sample::AAS_TraceClientBBox;
use crate::src::botlib::be_aas_sample::AAS_UnlinkFromAreas;
extern "C" {
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
    /* ****************************************************************************
     * name:		be_aas_reach.c
     *
     * desc:		reachability calculations
     *
     * $Archive: /MissionPack/code/botlib/be_aas_reach.c $
     *
     *****************************************************************************/
    #[no_mangle]
    pub fn Sys_MilliSeconds() -> libc::c_int;
    #[no_mangle]
    pub static mut botimport: crate::botlib_h::botlib_import_t;
}
//linked reachability

pub type aas_lreachability_t = aas_lreachability_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aas_lreachability_s {
    pub areanum: libc::c_int,
    pub facenum: libc::c_int,
    pub edgenum: libc::c_int,
    pub start: crate::src::qcommon::q_shared::vec3_t,
    pub end: crate::src::qcommon::q_shared::vec3_t,
    pub traveltype: libc::c_int,
    pub traveltime: libc::c_ushort,
    pub next: *mut aas_lreachability_s,
}
//number of the reachable area
//number of the face towards the other area
//number of the edge towards the other area
//start point of inter area movement
//end point of inter area movement
//type of travel required to get to the area
//travel time of the inter area movement
//
//valid area to weapon jump to
//number of reachabilities of each type
#[no_mangle]

pub static mut reach_swim: libc::c_int = 0;
//swim
#[no_mangle]

pub static mut reach_equalfloor: libc::c_int = 0;
//walk on floors with equal height
#[no_mangle]

pub static mut reach_step: libc::c_int = 0;
//step up
#[no_mangle]

pub static mut reach_walk: libc::c_int = 0;
//walk of step
#[no_mangle]

pub static mut reach_barrier: libc::c_int = 0;
//jump up to a barrier
#[no_mangle]

pub static mut reach_waterjump: libc::c_int = 0;
//jump out of water
#[no_mangle]

pub static mut reach_walkoffledge: libc::c_int = 0;
//walk of a ledge
#[no_mangle]

pub static mut reach_jump: libc::c_int = 0;
//jump
#[no_mangle]

pub static mut reach_ladder: libc::c_int = 0;
//climb or descent a ladder
#[no_mangle]

pub static mut reach_teleport: libc::c_int = 0;
//teleport
#[no_mangle]

pub static mut reach_elevator: libc::c_int = 0;
//use an elevator
#[no_mangle]

pub static mut reach_funcbob: libc::c_int = 0;
//use a func bob
#[no_mangle]

pub static mut reach_grapple: libc::c_int = 0;
//grapple hook
#[no_mangle]

pub static mut reach_doublejump: libc::c_int = 0;
//double jump
#[no_mangle]

pub static mut reach_rampjump: libc::c_int = 0;
//ramp jump
#[no_mangle]

pub static mut reach_strafejump: libc::c_int = 0;
//strafe jump (just normal jump but further)
#[no_mangle]

pub static mut reach_rocketjump: libc::c_int = 0;
//rocket jump
#[no_mangle]

pub static mut reach_bfgjump: libc::c_int = 0;
//bfg jump
#[no_mangle]

pub static mut reach_jumppad: libc::c_int = 0;
//jump pads
//if true grapple reachabilities are skipped
#[no_mangle]

pub static mut calcgrapplereach: libc::c_int = 0;
//temporary reachabilities
#[no_mangle]

pub static mut reachabilityheap: *mut aas_lreachability_t =
    0 as *const aas_lreachability_t as *mut aas_lreachability_t;
//heap with reachabilities
#[no_mangle]

pub static mut nextreachability: *mut aas_lreachability_t =
    0 as *const aas_lreachability_t as *mut aas_lreachability_t;
//next free reachability from the heap
#[no_mangle]

pub static mut areareachability: *mut *mut aas_lreachability_t =
    0 as *const *mut aas_lreachability_t as *mut *mut aas_lreachability_t;
//reachability links for every area
#[no_mangle]

pub static mut numlreachabilities: libc::c_int = 0;
//===========================================================================
// returns the surface area of the given face
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_FaceArea(
    mut face: *mut crate::aasfile_h::aas_face_t,
) -> libc::c_float {
    let mut i: libc::c_int = 0; //end for
    let mut edgenum: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut total: libc::c_float = 0.;
    let mut v: *mut crate::src::qcommon::q_shared::vec_t =
        0 as *mut crate::src::qcommon::q_shared::vec_t;
    let mut d1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut d2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut cross: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut edge: *mut crate::aasfile_h::aas_edge_t = 0 as *mut crate::aasfile_h::aas_edge_t;
    edgenum = *crate::src::botlib::be_aas_main::aasworld
        .edgeindex
        .offset((*face).firstedge as isize);
    side = (edgenum < 0 as libc::c_int) as libc::c_int;
    edge = &mut *crate::src::botlib::be_aas_main::aasworld
        .edges
        .offset(
            (crate::stdlib::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(edgenum)
                as isize,
        ) as *mut crate::aasfile_h::aas_edge_t;
    v = (*crate::src::botlib::be_aas_main::aasworld
        .vertexes
        .offset((*edge).v[side as usize] as isize))
    .as_mut_ptr();
    total = 0 as libc::c_int as libc::c_float;
    i = 1 as libc::c_int;
    while i < (*face).numedges - 1 as libc::c_int {
        edgenum = *crate::src::botlib::be_aas_main::aasworld
            .edgeindex
            .offset(((*face).firstedge + i) as isize);
        side = (edgenum < 0 as libc::c_int) as libc::c_int;
        edge = &mut *crate::src::botlib::be_aas_main::aasworld
            .edges
            .offset(
                (crate::stdlib::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(edgenum)
                    as isize,
            ) as *mut crate::aasfile_h::aas_edge_t;
        d1[0 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .vertexes
            .offset((*edge).v[side as usize] as isize))[0 as libc::c_int as usize]
            - *v.offset(0 as libc::c_int as isize);
        d1[1 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .vertexes
            .offset((*edge).v[side as usize] as isize))[1 as libc::c_int as usize]
            - *v.offset(1 as libc::c_int as isize);
        d1[2 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .vertexes
            .offset((*edge).v[side as usize] as isize))[2 as libc::c_int as usize]
            - *v.offset(2 as libc::c_int as isize);
        d2[0 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .vertexes
            .offset((*edge).v[(side == 0) as libc::c_int as usize] as isize))
            [0 as libc::c_int as usize]
            - *v.offset(0 as libc::c_int as isize);
        d2[1 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .vertexes
            .offset((*edge).v[(side == 0) as libc::c_int as usize] as isize))
            [1 as libc::c_int as usize]
            - *v.offset(1 as libc::c_int as isize);
        d2[2 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .vertexes
            .offset((*edge).v[(side == 0) as libc::c_int as usize] as isize))
            [2 as libc::c_int as usize]
            - *v.offset(2 as libc::c_int as isize);
        CrossProduct(
            d1.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            d2.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            cross.as_mut_ptr(),
        );
        total = (total as libc::c_double
            + 0.5f64
                * VectorLength(cross.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t)
                    as libc::c_double) as libc::c_float;
        i += 1
    }
    return total;
}
//end of the function AAS_FaceArea
//===========================================================================
// returns the volume of an area
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaVolume(mut areanum: libc::c_int) -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut edgenum: libc::c_int = 0;
    let mut facenum: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut d: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut a: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut volume: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut corner: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut plane: *mut crate::aasfile_h::aas_plane_t = 0 as *mut crate::aasfile_h::aas_plane_t;
    let mut edge: *mut crate::aasfile_h::aas_edge_t = 0 as *mut crate::aasfile_h::aas_edge_t;
    let mut face: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut area: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    area = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize) as *mut crate::aasfile_h::aas_area_t;
    facenum = *crate::src::botlib::be_aas_main::aasworld
        .faceindex
        .offset((*area).firstface as isize);
    face = &mut *crate::src::botlib::be_aas_main::aasworld
        .faces
        .offset(
            (crate::stdlib::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(facenum)
                as isize,
        ) as *mut crate::aasfile_h::aas_face_t;
    edgenum = *crate::src::botlib::be_aas_main::aasworld
        .edgeindex
        .offset((*face).firstedge as isize);
    edge = &mut *crate::src::botlib::be_aas_main::aasworld
        .edges
        .offset(
            (crate::stdlib::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(edgenum)
                as isize,
        ) as *mut crate::aasfile_h::aas_edge_t;
    //
    corner[0 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
        .vertexes
        .offset((*edge).v[0 as libc::c_int as usize] as isize))[0 as libc::c_int as usize];
    corner[1 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
        .vertexes
        .offset((*edge).v[0 as libc::c_int as usize] as isize))[1 as libc::c_int as usize];
    corner[2 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
        .vertexes
        .offset((*edge).v[0 as libc::c_int as usize] as isize))[2 as libc::c_int as usize];
    //make tetrahedrons to all other faces
    volume = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t; //end for
    i = 0 as libc::c_int;
    while i < (*area).numfaces {
        facenum = crate::stdlib::abs(
            *crate::src::botlib::be_aas_main::aasworld
                .faceindex
                .offset(((*area).firstface + i) as isize),
        );
        face = &mut *crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(facenum as isize) as *mut crate::aasfile_h::aas_face_t;
        side = ((*face).backarea != areanum) as libc::c_int;
        plane = &mut *crate::src::botlib::be_aas_main::aasworld
            .planes
            .offset(((*face).planenum ^ side) as isize)
            as *mut crate::aasfile_h::aas_plane_t;
        d = -(corner[0 as libc::c_int as usize] * (*plane).normal[0 as libc::c_int as usize]
            + corner[1 as libc::c_int as usize] * (*plane).normal[1 as libc::c_int as usize]
            + corner[2 as libc::c_int as usize] * (*plane).normal[2 as libc::c_int as usize]
            - (*plane).dist);
        a = AAS_FaceArea(face);
        volume += d * a;
        i += 1
    }
    volume /= 3 as libc::c_int as libc::c_float;
    return volume;
}
//end of the function AAS_AreaVolume
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_BestReachableLinkArea(
    mut areas: *mut crate::be_aas_def_h::aas_link_t,
) -> libc::c_int {
    let mut link: *mut crate::be_aas_def_h::aas_link_t = 0 as *mut crate::be_aas_def_h::aas_link_t; //end for
    link = areas;
    while !link.is_null() {
        if AAS_AreaGrounded((*link).areanum) != 0 || AAS_AreaSwim((*link).areanum) != 0 {
            return (*link).areanum;
        }
        link = (*link).next_area
        //end if
    }
    //
    link = areas; //end for
    while !link.is_null() {
        if (*link).areanum != 0 {
            return (*link).areanum;
        }
        //FIXME: this is a bad idea when the reachability is not yet
        // calculated when the level items are loaded
        if AAS_AreaReachability((*link).areanum) != 0 {
            return (*link).areanum;
        }
        link = (*link).next_area
    }
    return 0 as libc::c_int;
}
//end of the function AAS_BestReachableLinkArea
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_GetJumpPadInfo(
    mut ent: libc::c_int,
    mut areastart: *mut crate::src::qcommon::q_shared::vec_t,
    mut absmins: *mut crate::src::qcommon::q_shared::vec_t,
    mut absmaxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut velocity: *mut crate::src::qcommon::q_shared::vec_t,
) -> libc::c_int {
    let mut modelnum: libc::c_int = 0;
    let mut ent2: libc::c_int = 0;
    let mut speed: libc::c_float = 0.;
    let mut height: libc::c_float = 0.;
    let mut gravity: libc::c_float = 0.;
    let mut time: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    let mut forward: libc::c_float = 0.;
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut teststart: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut ent2origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut trace: crate::be_aas_h::aas_trace_t = crate::be_aas_h::aas_trace_t {
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        ent: 0,
        lastarea: 0,
        area: 0,
        planenum: 0,
    };
    let mut model: [libc::c_char; 128] = [0; 128];
    let mut target: [libc::c_char; 128] = [0; 128];
    let mut targetname: [libc::c_char; 128] = [0; 128];
    //
    crate::src::botlib::be_aas_bspq3::AAS_FloatForBSPEpairKey(
        ent,
        b"speed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut speed,
    );
    if speed == 0. {
        speed = 1000 as libc::c_int as libc::c_float
    }
    angles[2 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    angles[1 as libc::c_int as usize] = angles[2 as libc::c_int as usize];
    angles[0 as libc::c_int as usize] = angles[1 as libc::c_int as usize];
    //get the mins, maxs and origin of the model
    crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
        ent,
        b"model\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        model.as_mut_ptr(),
        128 as libc::c_int,
    );
    if model[0 as libc::c_int as usize] != 0 {
        modelnum = crate::stdlib::atoi(model.as_mut_ptr().offset(1 as libc::c_int as isize))
    } else {
        modelnum = 0 as libc::c_int
    }
    crate::src::botlib::be_aas_bspq3::AAS_BSPModelMinsMaxsOrigin(
        modelnum,
        angles.as_mut_ptr(),
        absmins,
        absmaxs,
        origin.as_mut_ptr(),
    );
    *absmins.offset(0 as libc::c_int as isize) =
        origin[0 as libc::c_int as usize] + *absmins.offset(0 as libc::c_int as isize);
    *absmins.offset(1 as libc::c_int as isize) =
        origin[1 as libc::c_int as usize] + *absmins.offset(1 as libc::c_int as isize);
    *absmins.offset(2 as libc::c_int as isize) =
        origin[2 as libc::c_int as usize] + *absmins.offset(2 as libc::c_int as isize);
    *absmaxs.offset(0 as libc::c_int as isize) =
        origin[0 as libc::c_int as usize] + *absmaxs.offset(0 as libc::c_int as isize);
    *absmaxs.offset(1 as libc::c_int as isize) =
        origin[1 as libc::c_int as usize] + *absmaxs.offset(1 as libc::c_int as isize);
    *absmaxs.offset(2 as libc::c_int as isize) =
        origin[2 as libc::c_int as usize] + *absmaxs.offset(2 as libc::c_int as isize);
    origin[0 as libc::c_int as usize] =
        *absmins.offset(0 as libc::c_int as isize) + *absmaxs.offset(0 as libc::c_int as isize);
    origin[1 as libc::c_int as usize] =
        *absmins.offset(1 as libc::c_int as isize) + *absmaxs.offset(1 as libc::c_int as isize);
    origin[2 as libc::c_int as usize] =
        *absmins.offset(2 as libc::c_int as isize) + *absmaxs.offset(2 as libc::c_int as isize);
    origin[0 as libc::c_int as usize] = (origin[0 as libc::c_int as usize] as libc::c_double
        * 0.5f64) as crate::src::qcommon::q_shared::vec_t;
    origin[1 as libc::c_int as usize] = (origin[1 as libc::c_int as usize] as libc::c_double
        * 0.5f64) as crate::src::qcommon::q_shared::vec_t;
    origin[2 as libc::c_int as usize] = (origin[2 as libc::c_int as usize] as libc::c_double
        * 0.5f64) as crate::src::qcommon::q_shared::vec_t;
    //get the start areas
    teststart[0 as libc::c_int as usize] = origin[0 as libc::c_int as usize]; //end else
    teststart[1 as libc::c_int as usize] = origin[1 as libc::c_int as usize]; //end if
    teststart[2 as libc::c_int as usize] = origin[2 as libc::c_int as usize];
    teststart[2 as libc::c_int as usize] += 64 as libc::c_int as libc::c_float;
    trace = crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(
        teststart.as_mut_ptr(),
        origin.as_mut_ptr(),
        4 as libc::c_int,
        -(1 as libc::c_int),
    );
    if trace.startsolid as u64 != 0 {
        botimport.Print.expect("non-null function pointer")(
            1 as libc::c_int,
            b"trigger_push start solid\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        *areastart.offset(0 as libc::c_int as isize) = origin[0 as libc::c_int as usize];
        *areastart.offset(1 as libc::c_int as isize) = origin[1 as libc::c_int as usize];
        *areastart.offset(2 as libc::c_int as isize) = origin[2 as libc::c_int as usize]
    } else {
        *areastart.offset(0 as libc::c_int as isize) = trace.endpos[0 as libc::c_int as usize];
        *areastart.offset(1 as libc::c_int as isize) = trace.endpos[1 as libc::c_int as usize];
        *areastart.offset(2 as libc::c_int as isize) = trace.endpos[2 as libc::c_int as usize]
    }
    let ref mut fresh0 = *areastart.offset(2 as libc::c_int as isize);
    *fresh0 = (*fresh0 as libc::c_double + 0.125f64) as crate::src::qcommon::q_shared::vec_t;
    //
    //AAS_DrawPermanentCross(origin, 4, 4);
    //get the target entity
    crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
        ent,
        b"target\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        target.as_mut_ptr(),
        128 as libc::c_int,
    ); //end for
    ent2 = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(0 as libc::c_int); //end if
    while ent2 != 0 {
        if !(crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
            ent2,
            b"targetname\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            targetname.as_mut_ptr(),
            128 as libc::c_int,
        ) == 0)
        {
            if crate::stdlib::strcmp(targetname.as_mut_ptr(), target.as_mut_ptr()) == 0 {
                break;
            }
        }
        ent2 = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(ent2)
    }
    if ent2 == 0 {
        botimport.Print.expect("non-null function pointer")(
            1 as libc::c_int,
            b"trigger_push without target entity %s\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            target.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    crate::src::botlib::be_aas_bspq3::AAS_VectorForBSPEpairKey(
        ent2,
        b"origin\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ent2origin.as_mut_ptr(),
    );
    //
    height = ent2origin[2 as libc::c_int as usize] - origin[2 as libc::c_int as usize]; //end if
    gravity = crate::src::botlib::be_aas_move::aassettings.phys_gravity;
    time = crate::stdlib::sqrt(height as libc::c_double / (0.5f64 * gravity as libc::c_double))
        as libc::c_float;
    if time == 0. {
        botimport.Print.expect("non-null function pointer")(
            1 as libc::c_int,
            b"trigger_push without time\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    // set s.origin2 to the push velocity
    *velocity.offset(0 as libc::c_int as isize) =
        ent2origin[0 as libc::c_int as usize] - origin[0 as libc::c_int as usize];
    *velocity.offset(1 as libc::c_int as isize) =
        ent2origin[1 as libc::c_int as usize] - origin[1 as libc::c_int as usize];
    *velocity.offset(2 as libc::c_int as isize) =
        ent2origin[2 as libc::c_int as usize] - origin[2 as libc::c_int as usize];
    dist = crate::src::qcommon::q_math::VectorNormalize(velocity);
    forward = dist / time;
    //FIXME: why multiply by 1.1
    forward *= 1.1f32;
    *velocity.offset(0 as libc::c_int as isize) =
        *velocity.offset(0 as libc::c_int as isize) * forward;
    *velocity.offset(1 as libc::c_int as isize) =
        *velocity.offset(1 as libc::c_int as isize) * forward;
    *velocity.offset(2 as libc::c_int as isize) =
        *velocity.offset(2 as libc::c_int as isize) * forward;
    *velocity.offset(2 as libc::c_int as isize) = time * gravity;
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function AAS_GetJumpPadInfo
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_BestReachableFromJumpPadArea(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
) -> libc::c_int {
    let mut ent: libc::c_int = 0; //end for
    let mut bot_visualizejumppads: libc::c_int = 0;
    let mut bestareanum: libc::c_int = 0;
    let mut volume: libc::c_float = 0.;
    let mut bestareavolume: libc::c_float = 0.;
    let mut areastart: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut cmdmove: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut bboxmins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut bboxmaxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut absmins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut absmaxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut move_0: crate::be_aas_h::aas_clientmove_t = crate::be_aas_h::aas_clientmove_t {
        endpos: [0.; 3],
        endarea: 0,
        velocity: [0.; 3],
        trace: crate::be_aas_h::aas_trace_t {
            startsolid: crate::src::qcommon::q_shared::qfalse,
            fraction: 0.,
            endpos: [0.; 3],
            ent: 0,
            lastarea: 0,
            area: 0,
            planenum: 0,
        },
        presencetype: 0,
        stopevent: 0,
        endcontents: 0,
        time: 0.,
        frames: 0,
    };
    let mut areas: *mut crate::be_aas_def_h::aas_link_t = 0 as *mut crate::be_aas_def_h::aas_link_t;
    let mut link: *mut crate::be_aas_def_h::aas_link_t = 0 as *mut crate::be_aas_def_h::aas_link_t;
    let mut classname: [libc::c_char; 128] = [0; 128];
    bot_visualizejumppads = crate::src::botlib::l_libvar::LibVarValue(
        b"bot_visualizejumppads\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    bboxmins[0 as libc::c_int as usize] =
        *origin.offset(0 as libc::c_int as isize) + *mins.offset(0 as libc::c_int as isize);
    bboxmins[1 as libc::c_int as usize] =
        *origin.offset(1 as libc::c_int as isize) + *mins.offset(1 as libc::c_int as isize);
    bboxmins[2 as libc::c_int as usize] =
        *origin.offset(2 as libc::c_int as isize) + *mins.offset(2 as libc::c_int as isize);
    bboxmaxs[0 as libc::c_int as usize] =
        *origin.offset(0 as libc::c_int as isize) + *maxs.offset(0 as libc::c_int as isize);
    bboxmaxs[1 as libc::c_int as usize] =
        *origin.offset(1 as libc::c_int as isize) + *maxs.offset(1 as libc::c_int as isize);
    bboxmaxs[2 as libc::c_int as usize] =
        *origin.offset(2 as libc::c_int as isize) + *maxs.offset(2 as libc::c_int as isize);
    ent = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(0 as libc::c_int);
    while ent != 0 {
        if !(crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
            ent,
            b"classname\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            classname.as_mut_ptr(),
            128 as libc::c_int,
        ) == 0)
        {
            if !(crate::stdlib::strcmp(
                classname.as_mut_ptr(),
                b"trigger_push\x00" as *const u8 as *const libc::c_char,
            ) != 0)
            {
                //
                if !(AAS_GetJumpPadInfo(
                    ent,
                    areastart.as_mut_ptr(),
                    absmins.as_mut_ptr(),
                    absmaxs.as_mut_ptr(),
                    velocity.as_mut_ptr(),
                ) == 0)
                {
                    //get the areas the jump pad brush is in
                    areas = crate::src::botlib::be_aas_sample::AAS_LinkEntityClientBBox(
                        absmins.as_mut_ptr(),
                        absmaxs.as_mut_ptr(),
                        -(1 as libc::c_int),
                        4 as libc::c_int,
                    ); //end for
                    link = areas; //end if
                    while !link.is_null() {
                        if AAS_AreaJumpPad((*link).areanum) != 0 {
                            break;
                        }
                        link = (*link).next_area
                    }
                    if link.is_null() {
                        botimport.Print.expect("non-null function pointer")(
                            1 as libc::c_int,
                            b"trigger_push not in any jump pad area\n\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        crate::src::botlib::be_aas_sample::AAS_UnlinkFromAreas(areas);
                    } else {
                        //
                        //botimport.Print(PRT_MESSAGE, "found a trigger_push with velocity %f %f %f\n", velocity[0], velocity[1], velocity[2]);
                        //
                        cmdmove[0 as libc::c_int as usize] =
                            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t; //end if
                        cmdmove[1 as libc::c_int as usize] =
                            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t; //end if
                        cmdmove[2 as libc::c_int as usize] =
                            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
                        crate::stdlib::memset(
                            &mut move_0 as *mut crate::be_aas_h::aas_clientmove_t
                                as *mut libc::c_void,
                            0 as libc::c_int,
                            ::std::mem::size_of::<crate::be_aas_h::aas_clientmove_t>()
                                as libc::c_ulong,
                        );
                        crate::src::botlib::be_aas_move::AAS_ClientMovementHitBBox(
                            &mut move_0,
                            -(1 as libc::c_int),
                            areastart.as_mut_ptr(),
                            2 as libc::c_int,
                            crate::src::qcommon::q_shared::qfalse as libc::c_int,
                            velocity.as_mut_ptr(),
                            cmdmove.as_mut_ptr(),
                            0 as libc::c_int,
                            30 as libc::c_int,
                            0.1f32,
                            bboxmins.as_mut_ptr(),
                            bboxmaxs.as_mut_ptr(),
                            bot_visualizejumppads,
                        );
                        if move_0.frames < 30 as libc::c_int {
                            bestareanum = 0 as libc::c_int;
                            bestareavolume = 0 as libc::c_int as libc::c_float;
                            link = areas;
                            while !link.is_null() {
                                if !(AAS_AreaJumpPad((*link).areanum) == 0) {
                                    volume = AAS_AreaVolume((*link).areanum);
                                    if volume >= bestareavolume {
                                        bestareanum = (*link).areanum;
                                        bestareavolume = volume
                                    }
                                }
                                link = (*link).next_area
                                //end if
                            }
                            crate::src::botlib::be_aas_sample::AAS_UnlinkFromAreas(areas);
                            return bestareanum;
                        }
                        crate::src::botlib::be_aas_sample::AAS_UnlinkFromAreas(areas);
                    }
                }
            }
        }
        ent = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(ent)
    }
    return 0 as libc::c_int;
}
//end of the function AAS_BestReachableFromJumpPadArea
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_BestReachableArea(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut goalorigin: *mut crate::src::qcommon::q_shared::vec_t,
) -> libc::c_int {
    let mut areanum: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut areas: *mut crate::be_aas_def_h::aas_link_t = 0 as *mut crate::be_aas_def_h::aas_link_t;
    let mut absmins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut absmaxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    //vec3_t bbmins, bbmaxs;
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3]; //end if
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut trace: crate::be_aas_h::aas_trace_t = crate::be_aas_h::aas_trace_t {
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        ent: 0,
        lastarea: 0,
        area: 0,
        planenum: 0,
    };
    if crate::src::botlib::be_aas_main::aasworld.loaded == 0 {
        botimport.Print.expect("non-null function pointer")(
            3 as libc::c_int,
            b"AAS_BestReachableArea: aas not loaded\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as libc::c_int;
    }
    //find a point in an area
    start[0 as libc::c_int as usize] = *origin.offset(0 as libc::c_int as isize);
    start[1 as libc::c_int as usize] = *origin.offset(1 as libc::c_int as isize);
    start[2 as libc::c_int as usize] = *origin.offset(2 as libc::c_int as isize);
    areanum = crate::src::botlib::be_aas_sample::AAS_PointAreaNum(start.as_mut_ptr());
    //while no area found fudge around a little
    i = 0 as libc::c_int; //end for
    while i < 5 as libc::c_int && areanum == 0 {
        j = 0 as libc::c_int;
        while j < 5 as libc::c_int && areanum == 0 {
            k = -(1 as libc::c_int);
            while k <= 1 as libc::c_int && areanum == 0 {
                l = -(1 as libc::c_int);
                while l <= 1 as libc::c_int && areanum == 0 {
                    start[0 as libc::c_int as usize] = *origin.offset(0 as libc::c_int as isize);
                    start[1 as libc::c_int as usize] = *origin.offset(1 as libc::c_int as isize);
                    start[2 as libc::c_int as usize] = *origin.offset(2 as libc::c_int as isize);
                    start[0 as libc::c_int as usize] +=
                        j as libc::c_float * 4 as libc::c_int as libc::c_float * k as libc::c_float;
                    start[1 as libc::c_int as usize] +=
                        j as libc::c_float * 4 as libc::c_int as libc::c_float * l as libc::c_float;
                    start[2 as libc::c_int as usize] +=
                        i as libc::c_float * 4 as libc::c_int as libc::c_float;
                    areanum =
                        crate::src::botlib::be_aas_sample::AAS_PointAreaNum(start.as_mut_ptr());
                    l += 1
                }
                k += 1
                //end for
            }
            j += 1
            //end for
        }
        i += 1
        //end for
    }
    //if an area was found
    if areanum != 0 {
        //end if
        //drop client bbox down and try again
        end[0 as libc::c_int as usize] = start[0 as libc::c_int as usize];
        end[1 as libc::c_int as usize] = start[1 as libc::c_int as usize];
        end[2 as libc::c_int as usize] = start[2 as libc::c_int as usize];
        start[2 as libc::c_int as usize] = (start[2 as libc::c_int as usize] as libc::c_double
            + 0.25f64)
            as crate::src::qcommon::q_shared::vec_t;
        end[2 as libc::c_int as usize] -= 50 as libc::c_int as libc::c_float;
        trace = crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(
            start.as_mut_ptr(),
            end.as_mut_ptr(),
            4 as libc::c_int,
            -(1 as libc::c_int),
        );
        if trace.startsolid as u64 == 0 {
            //end else
            areanum =
                crate::src::botlib::be_aas_sample::AAS_PointAreaNum(trace.endpos.as_mut_ptr()); //end if
            *goalorigin.offset(0 as libc::c_int as isize) = trace.endpos[0 as libc::c_int as usize];
            *goalorigin.offset(1 as libc::c_int as isize) = trace.endpos[1 as libc::c_int as usize];
            *goalorigin.offset(2 as libc::c_int as isize) = trace.endpos[2 as libc::c_int as usize];
            //FIXME: cannot enable next line right now because the reachability
            // does not have to be calculated when the level items are loaded
            //if the origin is in an area with reachability
            //if (AAS_AreaReachability(areanum)) return areanum;
            if areanum != 0 {
                return areanum;
            }
        } else {
            //it can very well happen that the AAS_PointAreaNum function tells that
            //a point is in an area and that starting an AAS_TraceClientBBox from that
            //point will return trace.startsolid qtrue
            *goalorigin.offset(0 as libc::c_int as isize) = start[0 as libc::c_int as usize];
            *goalorigin.offset(1 as libc::c_int as isize) = start[1 as libc::c_int as usize];
            *goalorigin.offset(2 as libc::c_int as isize) = start[2 as libc::c_int as usize];
            return areanum;
        }
    }
    //
    //AAS_PresenceTypeBoundingBox(PRESENCE_CROUCH, bbmins, bbmaxs);
    //NOTE: the goal origin does not have to be in the goal area
    // because the bot will have to move towards the item origin anyway
    *goalorigin.offset(0 as libc::c_int as isize) = *origin.offset(0 as libc::c_int as isize);
    *goalorigin.offset(1 as libc::c_int as isize) = *origin.offset(1 as libc::c_int as isize);
    *goalorigin.offset(2 as libc::c_int as isize) = *origin.offset(2 as libc::c_int as isize);
    //
    absmins[0 as libc::c_int as usize] =
        *origin.offset(0 as libc::c_int as isize) + *mins.offset(0 as libc::c_int as isize);
    absmins[1 as libc::c_int as usize] =
        *origin.offset(1 as libc::c_int as isize) + *mins.offset(1 as libc::c_int as isize);
    absmins[2 as libc::c_int as usize] =
        *origin.offset(2 as libc::c_int as isize) + *mins.offset(2 as libc::c_int as isize);
    absmaxs[0 as libc::c_int as usize] =
        *origin.offset(0 as libc::c_int as isize) + *maxs.offset(0 as libc::c_int as isize);
    absmaxs[1 as libc::c_int as usize] =
        *origin.offset(1 as libc::c_int as isize) + *maxs.offset(1 as libc::c_int as isize);
    absmaxs[2 as libc::c_int as usize] =
        *origin.offset(2 as libc::c_int as isize) + *maxs.offset(2 as libc::c_int as isize);
    //add bounding box size
    //VectorSubtract(absmins, bbmaxs, absmins);
    //VectorSubtract(absmaxs, bbmins, absmaxs);
    //link an invalid (-1) entity
    areas = crate::src::botlib::be_aas_sample::AAS_LinkEntityClientBBox(
        absmins.as_mut_ptr(),
        absmaxs.as_mut_ptr(),
        -(1 as libc::c_int),
        4 as libc::c_int,
    );
    //get the reachable link area
    areanum = AAS_BestReachableLinkArea(areas);
    //unlink the invalid entity
    crate::src::botlib::be_aas_sample::AAS_UnlinkFromAreas(areas);
    //
    return areanum;
}
//end of the function AAS_BestReachableArea
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_SetupReachabilityHeap() {
    let mut i: libc::c_int = 0; //end for
    reachabilityheap = crate::src::botlib::l_memory::GetClearedMemory(
        (65536 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<aas_lreachability_t>() as libc::c_ulong),
    ) as *mut aas_lreachability_t;
    i = 0 as libc::c_int;
    while i < 65536 as libc::c_int - 1 as libc::c_int {
        let ref mut fresh1 = (*reachabilityheap.offset(i as isize)).next;
        *fresh1 = &mut *reachabilityheap.offset((i + 1 as libc::c_int) as isize)
            as *mut aas_lreachability_t;
        i += 1
    }
    let ref mut fresh2 =
        (*reachabilityheap.offset((65536 as libc::c_int - 1 as libc::c_int) as isize)).next;
    *fresh2 = 0 as *mut aas_lreachability_s;
    nextreachability = reachabilityheap;
    numlreachabilities = 0 as libc::c_int;
}
//end of the function AAS_InitReachabilityHeap
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ShutDownReachabilityHeap() {
    crate::src::botlib::l_memory::FreeMemory(reachabilityheap as *mut libc::c_void);
    numlreachabilities = 0 as libc::c_int;
}
//end of the function AAS_ShutDownReachabilityHeap
//===========================================================================
// returns a reachability link
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AllocReachability() -> *mut aas_lreachability_t {
    let mut r: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    if nextreachability.is_null() {
        return 0 as *mut aas_lreachability_t;
    }
    //make sure the error message only shows up once
    if (*nextreachability).next.is_null() {
        crate::src::botlib::be_aas_main::AAS_Error(
            b"AAS_MAX_REACHABILITYSIZE\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    //
    r = nextreachability;
    nextreachability = (*nextreachability).next;
    numlreachabilities += 1;
    return r;
}
//end of the function AAS_AllocReachability
//===========================================================================
// frees a reachability link
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_FreeReachability(mut lreach: *mut aas_lreachability_t) {
    crate::stdlib::memset(
        lreach as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<aas_lreachability_t>() as libc::c_ulong,
    );
    (*lreach).next = nextreachability;
    nextreachability = lreach;
    numlreachabilities -= 1;
}
//end of the function AAS_FreeReachability
//===========================================================================
// returns qtrue if the area has reachability links
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaReachability(mut areanum: libc::c_int) -> libc::c_int {
    if areanum < 0 as libc::c_int || areanum >= crate::src::botlib::be_aas_main::aasworld.numareas {
        crate::src::botlib::be_aas_main::AAS_Error(
            b"AAS_AreaReachability: areanum %d out of range\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            areanum,
        ); //end if
        return 0 as libc::c_int;
    }
    return (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .numreachableareas;
}
//end of the function AAS_AreaReachability
//===========================================================================
// returns the surface area of all ground faces together of the area
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaGroundFaceArea(mut areanum: libc::c_int) -> libc::c_float {
    let mut i: libc::c_int = 0; //end for
    let mut total: libc::c_float = 0.;
    let mut area: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut face: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    total = 0 as libc::c_int as libc::c_float;
    area = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize) as *mut crate::aasfile_h::aas_area_t;
    i = 0 as libc::c_int;
    while i < (*area).numfaces {
        face = &mut *crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(
                (crate::stdlib::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(
                    *crate::src::botlib::be_aas_main::aasworld
                        .faceindex
                        .offset(((*area).firstface + i) as isize),
                ) as isize,
            ) as *mut crate::aasfile_h::aas_face_t;
        if !((*face).faceflags & 4 as libc::c_int == 0) {
            //
            total += AAS_FaceArea(face)
        }
        i += 1
    }
    return total;
}
//end of the function AAS_AreaGroundFaceArea
//===========================================================================
// returns the center of a face
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_FaceCenter(
    mut facenum: libc::c_int,
    mut center: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut i: libc::c_int = 0; //end for
    let mut scale: libc::c_float = 0.;
    let mut face: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut edge: *mut crate::aasfile_h::aas_edge_t = 0 as *mut crate::aasfile_h::aas_edge_t;
    face = &mut *crate::src::botlib::be_aas_main::aasworld
        .faces
        .offset(facenum as isize) as *mut crate::aasfile_h::aas_face_t;
    let ref mut fresh3 = *center.offset(2 as libc::c_int as isize);
    *fresh3 = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    let ref mut fresh4 = *center.offset(1 as libc::c_int as isize);
    *fresh4 = *fresh3;
    *center.offset(0 as libc::c_int as isize) = *fresh4;
    i = 0 as libc::c_int;
    while i < (*face).numedges {
        edge = &mut *crate::src::botlib::be_aas_main::aasworld
            .edges
            .offset(
                (crate::stdlib::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(
                    *crate::src::botlib::be_aas_main::aasworld
                        .edgeindex
                        .offset(((*face).firstedge + i) as isize),
                ) as isize,
            ) as *mut crate::aasfile_h::aas_edge_t;
        *center.offset(0 as libc::c_int as isize) = *center.offset(0 as libc::c_int as isize)
            + (*crate::src::botlib::be_aas_main::aasworld
                .vertexes
                .offset((*edge).v[0 as libc::c_int as usize] as isize))[0 as libc::c_int as usize];
        *center.offset(1 as libc::c_int as isize) = *center.offset(1 as libc::c_int as isize)
            + (*crate::src::botlib::be_aas_main::aasworld
                .vertexes
                .offset((*edge).v[0 as libc::c_int as usize] as isize))[1 as libc::c_int as usize];
        *center.offset(2 as libc::c_int as isize) = *center.offset(2 as libc::c_int as isize)
            + (*crate::src::botlib::be_aas_main::aasworld
                .vertexes
                .offset((*edge).v[0 as libc::c_int as usize] as isize))[2 as libc::c_int as usize];
        *center.offset(0 as libc::c_int as isize) = *center.offset(0 as libc::c_int as isize)
            + (*crate::src::botlib::be_aas_main::aasworld
                .vertexes
                .offset((*edge).v[1 as libc::c_int as usize] as isize))[0 as libc::c_int as usize];
        *center.offset(1 as libc::c_int as isize) = *center.offset(1 as libc::c_int as isize)
            + (*crate::src::botlib::be_aas_main::aasworld
                .vertexes
                .offset((*edge).v[1 as libc::c_int as usize] as isize))[1 as libc::c_int as usize];
        *center.offset(2 as libc::c_int as isize) = *center.offset(2 as libc::c_int as isize)
            + (*crate::src::botlib::be_aas_main::aasworld
                .vertexes
                .offset((*edge).v[1 as libc::c_int as usize] as isize))[2 as libc::c_int as usize];
        i += 1
    }
    scale = (0.5f64 / (*face).numedges as libc::c_double) as libc::c_float;
    *center.offset(0 as libc::c_int as isize) = *center.offset(0 as libc::c_int as isize) * scale;
    *center.offset(1 as libc::c_int as isize) = *center.offset(1 as libc::c_int as isize) * scale;
    *center.offset(2 as libc::c_int as isize) = *center.offset(2 as libc::c_int as isize) * scale;
}
//end of the function AAS_FaceCenter
//===========================================================================
// returns the maximum distance a player can fall before being damaged
// damage = deltavelocity*deltavelocity  * 0.0001
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_FallDamageDistance() -> libc::c_int {
    let mut maxzvelocity: libc::c_float = 0.;
    let mut gravity: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    maxzvelocity = crate::stdlib::sqrt((30 as libc::c_int * 10000 as libc::c_int) as libc::c_double)
        as libc::c_float;
    gravity = crate::src::botlib::be_aas_move::aassettings.phys_gravity;
    t = maxzvelocity / gravity;
    return (0.5f64 * gravity as libc::c_double * t as libc::c_double * t as libc::c_double)
        as libc::c_int;
}
//end of the function AAS_FallDamageDistance
//===========================================================================
// distance = 0.5 * gravity * t * t
// vel = t * gravity
// damage = vel * vel * 0.0001
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_FallDelta(mut distance: libc::c_float) -> libc::c_float {
    let mut t: libc::c_float = 0.;
    let mut delta: libc::c_float = 0.;
    let mut gravity: libc::c_float = 0.;
    gravity = crate::src::botlib::be_aas_move::aassettings.phys_gravity;
    t = crate::stdlib::sqrt(
        crate::stdlib::fabs(distance as libc::c_double) * 2 as libc::c_int as libc::c_double
            / gravity as libc::c_double,
    ) as libc::c_float;
    delta = t * gravity;
    return ((delta * delta) as libc::c_double * 0.0001f64) as libc::c_float;
}
//end of the function AAS_FallDelta
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_MaxJumpHeight(mut phys_jumpvel: libc::c_float) -> libc::c_float {
    let mut phys_gravity: libc::c_float = 0.;
    phys_gravity = crate::src::botlib::be_aas_move::aassettings.phys_gravity;
    //maximum height a player can jump with the given initial z velocity
    return (0.5f64
        * phys_gravity as libc::c_double
        * (phys_jumpvel / phys_gravity) as libc::c_double
        * (phys_jumpvel / phys_gravity) as libc::c_double) as libc::c_float;
}
//end of the function MaxJumpHeight
//===========================================================================
// returns true if a player can only crouch in the area
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_MaxJumpDistance(mut phys_jumpvel: libc::c_float) -> libc::c_float {
    let mut phys_gravity: libc::c_float = 0.;
    let mut phys_maxvelocity: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    phys_gravity = crate::src::botlib::be_aas_move::aassettings.phys_gravity;
    phys_maxvelocity = crate::src::botlib::be_aas_move::aassettings.phys_maxvelocity;
    //time a player takes to fall the height
    t = crate::stdlib::sqrt(
        crate::src::botlib::be_aas_move::aassettings.rs_maxjumpfallheight as libc::c_double
            / (0.5f64 * phys_gravity as libc::c_double),
    ) as libc::c_float;
    //maximum distance
    return phys_maxvelocity * (t + phys_jumpvel / phys_gravity);
}
//end of the function AAS_MaxJumpDistance
//===========================================================================
// returns true if a player can only crouch in the area
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaCrouch(mut areanum: libc::c_int) -> libc::c_int {
    if (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .presencetype
        & 2 as libc::c_int
        == 0
    {
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    } else {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    };
}
//end of the function AAS_AreaCrouch
//===========================================================================
// returns qtrue if it is possible to swim in the area
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaSwim(mut areanum: libc::c_int) -> libc::c_int {
    if (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .areaflags
        & 4 as libc::c_int
        != 0
    {
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    } else {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    };
}
//end of the function AAS_AreaSwim
//===========================================================================
// returns qtrue if the area contains a liquid
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaLiquid(mut areanum: libc::c_int) -> libc::c_int {
    if (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .areaflags
        & 4 as libc::c_int
        != 0
    {
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    } else {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    };
}
//end of the function AAS_AreaLiquid
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaLava(mut areanum: libc::c_int) -> libc::c_int {
    return (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .contents
        & 2 as libc::c_int;
}
//end of the function AAS_AreaLava
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaSlime(mut areanum: libc::c_int) -> libc::c_int {
    return (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .contents
        & 4 as libc::c_int;
}
//end of the function AAS_AreaSlime
//===========================================================================
// returns qtrue if the area contains ground faces
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaGrounded(mut areanum: libc::c_int) -> libc::c_int {
    return (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .areaflags
        & 1 as libc::c_int;
}
//end of the function AAS_AreaGround
//===========================================================================
// returns true if the area contains ladder faces
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaLadder(mut areanum: libc::c_int) -> libc::c_int {
    return (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .areaflags
        & 2 as libc::c_int;
}
//end of the function AAS_AreaLadder
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaJumpPad(mut areanum: libc::c_int) -> libc::c_int {
    return (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .contents
        & 128 as libc::c_int;
}
//end of the function AAS_AreaJumpPad
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaTeleporter(mut areanum: libc::c_int) -> libc::c_int {
    return (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .contents
        & 64 as libc::c_int;
}
//end of the function AAS_AreaTeleporter
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaClusterPortal(mut areanum: libc::c_int) -> libc::c_int {
    return (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .contents
        & 8 as libc::c_int;
}
//
//AASINTERN
//AASINTERN
//returns true if the are has reachabilities to other areas
//returns true if the are has reachabilities to other areas
//returns the best reachable area and goal origin for a bounding box at the given origin
//returns the best reachable area and goal origin for a bounding box at the given origin
//returns the best jumppad area from which the bbox at origin is reachable
//returns the best jumppad area from which the bbox at origin is reachable
//returns the next reachability using the given model
//returns the next reachability using the given model
//returns the total area of the ground faces of the given area
//returns the total area of the ground faces of the given area
//returns true if the area is crouch only
//returns true if the area is crouch only
//returns true if a player can swim in this area
//returns true if a player can swim in this area
//returns true if the area is filled with a liquid
//returns true if the area is filled with a liquid
//returns true if the area contains lava
//returns true if the area contains lava
//returns true if the area contains slime
//returns true if the area contains slime
//returns true if the area has one or more ground faces
//returns true if the area has one or more ground faces
//returns true if the area has one or more ladder faces
//returns true if the area has one or more ladder faces
//returns true if the area is a jump pad
//returns true if the area is a jump pad
//returns true if the area is donotenter
//returns true if the area is donotenter
//end of the function AAS_AreaClusterPortal
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_AreaDoNotEnter(mut areanum: libc::c_int) -> libc::c_int {
    return (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize))
    .contents
        & 256 as libc::c_int;
}
//end of the function AAS_AreaDoNotEnter
//===========================================================================
// returns the time it takes perform a barrier jump
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_BarrierJumpTravelTime() -> libc::c_ushort {
    return (crate::src::botlib::be_aas_move::aassettings.phys_jumpvel as libc::c_double
        / (crate::src::botlib::be_aas_move::aassettings.phys_gravity as libc::c_double * 0.1f64))
        as libc::c_ushort;
}
//end op the function AAS_BarrierJumpTravelTime
//===========================================================================
// returns true if there already exists a reachability from area1 to area2
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ReachabilityExists(
    mut area1num: libc::c_int,
    mut area2num: libc::c_int,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut r: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t; //end for
    r = *areareachability.offset(area1num as isize);
    while !r.is_null() {
        if (*r).areanum == area2num {
            return crate::src::qcommon::q_shared::qtrue;
        }
        r = (*r).next
    }
    return crate::src::qcommon::q_shared::qfalse;
}
//end of the function AAS_ReachabilityExists
//===========================================================================
// returns true if there is a solid just after the end point when going
// from start to end
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_NearbySolidOrGap(
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
) -> libc::c_int {
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3]; //end if
    let mut testpoint: crate::src::qcommon::q_shared::vec3_t = [0.; 3]; //end if
    let mut areanum: libc::c_int = 0;
    dir[0 as libc::c_int as usize] =
        *end.offset(0 as libc::c_int as isize) - *start.offset(0 as libc::c_int as isize);
    dir[1 as libc::c_int as usize] =
        *end.offset(1 as libc::c_int as isize) - *start.offset(1 as libc::c_int as isize);
    dir[2 as libc::c_int as usize] =
        *end.offset(2 as libc::c_int as isize) - *start.offset(2 as libc::c_int as isize);
    dir[2 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
    testpoint[0 as libc::c_int as usize] = *end.offset(0 as libc::c_int as isize)
        + dir[0 as libc::c_int as usize] * 48 as libc::c_int as libc::c_float;
    testpoint[1 as libc::c_int as usize] = *end.offset(1 as libc::c_int as isize)
        + dir[1 as libc::c_int as usize] * 48 as libc::c_int as libc::c_float;
    testpoint[2 as libc::c_int as usize] = *end.offset(2 as libc::c_int as isize)
        + dir[2 as libc::c_int as usize] * 48 as libc::c_int as libc::c_float;
    areanum = crate::src::botlib::be_aas_sample::AAS_PointAreaNum(testpoint.as_mut_ptr());
    if areanum == 0 {
        testpoint[2 as libc::c_int as usize] += 16 as libc::c_int as libc::c_float;
        areanum = crate::src::botlib::be_aas_sample::AAS_PointAreaNum(testpoint.as_mut_ptr());
        if areanum == 0 {
            return crate::src::qcommon::q_shared::qtrue as libc::c_int;
        }
    }
    testpoint[0 as libc::c_int as usize] = *end.offset(0 as libc::c_int as isize)
        + dir[0 as libc::c_int as usize] * 64 as libc::c_int as libc::c_float;
    testpoint[1 as libc::c_int as usize] = *end.offset(1 as libc::c_int as isize)
        + dir[1 as libc::c_int as usize] * 64 as libc::c_int as libc::c_float;
    testpoint[2 as libc::c_int as usize] = *end.offset(2 as libc::c_int as isize)
        + dir[2 as libc::c_int as usize] * 64 as libc::c_int as libc::c_float;
    areanum = crate::src::botlib::be_aas_sample::AAS_PointAreaNum(testpoint.as_mut_ptr());
    if areanum != 0 {
        if AAS_AreaSwim(areanum) == 0 && AAS_AreaGrounded(areanum) == 0 {
            return crate::src::qcommon::q_shared::qtrue as libc::c_int;
        }
    }
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//end of the function AAS_SolidGapTime
//===========================================================================
// searches for swim reachabilities between adjacent areas
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Reachability_Swim(
    mut area1num: libc::c_int,
    mut area2num: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut face1num: libc::c_int = 0;
    let mut face2num: libc::c_int = 0;
    let mut side1: libc::c_int = 0;
    let mut area1: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut area2: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut face1: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut plane: *mut crate::aasfile_h::aas_plane_t = 0 as *mut crate::aasfile_h::aas_plane_t;
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if AAS_AreaSwim(area1num) == 0 || AAS_AreaSwim(area2num) == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //if the second area is crouch only
    if (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(area2num as isize))
    .presencetype
        & 2 as libc::c_int
        == 0
    {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    area1 = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(area1num as isize) as *mut crate::aasfile_h::aas_area_t;
    area2 = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(area2num as isize) as *mut crate::aasfile_h::aas_area_t;
    //if the areas are not near enough
    i = 0 as libc::c_int; //end for
    while i < 3 as libc::c_int {
        if (*area1).mins[i as usize]
            > (*area2).maxs[i as usize] + 10 as libc::c_int as libc::c_float
        {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        if (*area1).maxs[i as usize]
            < (*area2).mins[i as usize] - 10 as libc::c_int as libc::c_float
        {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        i += 1
    }
    //find a shared face and create a reachability link
    i = 0 as libc::c_int; //end for
    while i < (*area1).numfaces {
        face1num = *crate::src::botlib::be_aas_main::aasworld
            .faceindex
            .offset(((*area1).firstface + i) as isize);
        side1 = (face1num < 0 as libc::c_int) as libc::c_int;
        face1num = crate::stdlib::abs(face1num);
        //end for
        j = 0 as libc::c_int;
        while j < (*area2).numfaces {
            face2num = crate::stdlib::abs(
                *crate::src::botlib::be_aas_main::aasworld
                    .faceindex
                    .offset(((*area2).firstface + j) as isize),
            );
            //
            //end if
            if face1num == face2num {
                AAS_FaceCenter(face1num, start.as_mut_ptr());
                //
                //end if
                if crate::src::botlib::be_aas_bspq3::AAS_PointContents(start.as_mut_ptr())
                    & (8 as libc::c_int | 16 as libc::c_int | 32 as libc::c_int)
                    != 0
                {
                    //
                    //
                    face1 = &mut *crate::src::botlib::be_aas_main::aasworld
                        .faces
                        .offset(face1num as isize)
                        as *mut crate::aasfile_h::aas_face_t;
                    //create a new reachability link
                    lreach = AAS_AllocReachability();
                    if lreach.is_null() {
                        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                    }
                    (*lreach).areanum = area2num;
                    (*lreach).facenum = face1num;
                    (*lreach).edgenum = 0 as libc::c_int;
                    (*lreach).start[0 as libc::c_int as usize] = start[0 as libc::c_int as usize];
                    (*lreach).start[1 as libc::c_int as usize] = start[1 as libc::c_int as usize];
                    (*lreach).start[2 as libc::c_int as usize] = start[2 as libc::c_int as usize];
                    plane = &mut *crate::src::botlib::be_aas_main::aasworld
                        .planes
                        .offset(((*face1).planenum ^ side1) as isize)
                        as *mut crate::aasfile_h::aas_plane_t;
                    (*lreach).end[0 as libc::c_int as usize] = (*lreach).start
                        [0 as libc::c_int as usize]
                        + (*plane).normal[0 as libc::c_int as usize]
                            * -(2 as libc::c_int) as libc::c_float;
                    (*lreach).end[1 as libc::c_int as usize] = (*lreach).start
                        [1 as libc::c_int as usize]
                        + (*plane).normal[1 as libc::c_int as usize]
                            * -(2 as libc::c_int) as libc::c_float;
                    (*lreach).end[2 as libc::c_int as usize] = (*lreach).start
                        [2 as libc::c_int as usize]
                        + (*plane).normal[2 as libc::c_int as usize]
                            * -(2 as libc::c_int) as libc::c_float;
                    (*lreach).traveltype = 8 as libc::c_int;
                    (*lreach).traveltime = 1 as libc::c_int as libc::c_ushort;
                    //if the volume of the area is rather small
                    if AAS_AreaVolume(area2num) < 800 as libc::c_int as libc::c_float {
                        (*lreach).traveltime = ((*lreach).traveltime as libc::c_int
                            + 200 as libc::c_int)
                            as libc::c_ushort
                    }
                    //if (!(AAS_PointContents(start) & MASK_WATER)) lreach->traveltime += 500;
                    //link the reachability
                    (*lreach).next = *areareachability.offset(area1num as isize);
                    let ref mut fresh5 = *areareachability.offset(area1num as isize);
                    *fresh5 = lreach;
                    reach_swim += 1;
                    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
                }
            }
            j += 1
        }
        i += 1
    }
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//end of the function AAS_Reachability_Swim
//===========================================================================
// searches for reachabilities between adjacent areas with equal floor
// heights
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Reachability_EqualFloorHeight(
    mut area1num: libc::c_int,
    mut area2num: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut edgenum: libc::c_int = 0;
    let mut edgenum1: libc::c_int = 0;
    let mut edgenum2: libc::c_int = 0;
    let mut foundreach: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut height: libc::c_float = 0.;
    let mut bestheight: libc::c_float = 0.;
    let mut length: libc::c_float = 0.;
    let mut bestlength: libc::c_float = 0.;
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut normal: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut invgravity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut gravitydirection: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        -(1 as libc::c_int) as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut edgevec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut area1: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut area2: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut face1: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut face2: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut edge: *mut crate::aasfile_h::aas_edge_t = 0 as *mut crate::aasfile_h::aas_edge_t;
    let mut plane2: *mut crate::aasfile_h::aas_plane_t = 0 as *mut crate::aasfile_h::aas_plane_t;
    let mut lr: aas_lreachability_t = aas_lreachability_t {
        areanum: 0,
        facenum: 0,
        edgenum: 0,
        start: [0.; 3],
        end: [0.; 3],
        traveltype: 0,
        traveltime: 0,
        next: 0 as *mut aas_lreachability_s,
    };
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    if AAS_AreaGrounded(area1num) == 0 || AAS_AreaGrounded(area2num) == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    area1 = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(area1num as isize) as *mut crate::aasfile_h::aas_area_t;
    area2 = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(area2num as isize) as *mut crate::aasfile_h::aas_area_t;
    //if the areas are not near enough in the x-y direction
    i = 0 as libc::c_int; //end for
    while i < 2 as libc::c_int {
        if (*area1).mins[i as usize]
            > (*area2).maxs[i as usize] + 10 as libc::c_int as libc::c_float
        {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        if (*area1).maxs[i as usize]
            < (*area2).mins[i as usize] - 10 as libc::c_int as libc::c_float
        {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        i += 1
    }
    //if area 2 is too high above area 1
    if (*area2).mins[2 as libc::c_int as usize] > (*area1).maxs[2 as libc::c_int as usize] {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //
    invgravity[0 as libc::c_int as usize] = gravitydirection[0 as libc::c_int as usize];
    invgravity[1 as libc::c_int as usize] = gravitydirection[1 as libc::c_int as usize];
    invgravity[2 as libc::c_int as usize] = gravitydirection[2 as libc::c_int as usize];
    VectorInverse(invgravity.as_mut_ptr());
    //
    bestheight = 99999 as libc::c_int as libc::c_float; //make the compiler happy
    bestlength = 0 as libc::c_int as libc::c_float;
    foundreach = crate::src::qcommon::q_shared::qfalse as libc::c_int;
    crate::stdlib::memset(
        &mut lr as *mut aas_lreachability_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<aas_lreachability_t>() as libc::c_ulong,
    );
    //
    //check if the areas have ground faces with a common edge
    //if existing use the lowest common edge for a reachability link
    i = 0 as libc::c_int; //end for
    while i < (*area1).numfaces {
        face1 = &mut *crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(
                (crate::stdlib::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(
                    *crate::src::botlib::be_aas_main::aasworld
                        .faceindex
                        .offset(((*area1).firstface + i) as isize),
                ) as isize,
            ) as *mut crate::aasfile_h::aas_face_t;
        if !((*face1).faceflags & 4 as libc::c_int == 0) {
            //end for
            //
            j = 0 as libc::c_int;
            while j < (*area2).numfaces {
                face2 = &mut *crate::src::botlib::be_aas_main::aasworld.faces.offset(
                    (crate::stdlib::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(
                        *crate::src::botlib::be_aas_main::aasworld
                            .faceindex
                            .offset(((*area2).firstface + j) as isize),
                    ) as isize,
                ) as *mut crate::aasfile_h::aas_face_t;
                if !((*face2).faceflags & 4 as libc::c_int == 0) {
                    //end for
                    //if there is a common edge
                    edgenum1 = 0 as libc::c_int;
                    while edgenum1 < (*face1).numedges {
                        edgenum2 = 0 as libc::c_int;
                        while edgenum2 < (*face2).numedges {
                            if !(crate::stdlib::abs(
                                *crate::src::botlib::be_aas_main::aasworld
                                    .edgeindex
                                    .offset(((*face1).firstedge + edgenum1) as isize),
                            ) != crate::stdlib::abs(
                                *crate::src::botlib::be_aas_main::aasworld
                                    .edgeindex
                                    .offset(((*face2).firstedge + edgenum2) as isize),
                            )) {
                                edgenum = *crate::src::botlib::be_aas_main::aasworld
                                    .edgeindex
                                    .offset(((*face1).firstedge + edgenum1) as isize);
                                side = (edgenum < 0 as libc::c_int) as libc::c_int;
                                edge = &mut *crate::src::botlib::be_aas_main::aasworld.edges.offset(
                                    (crate::stdlib::abs
                                        as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(
                                        edgenum,
                                    ) as isize,
                                )
                                    as *mut crate::aasfile_h::aas_edge_t;
                                //get the length of the edge
                                dir[0 as libc::c_int as usize] =
                                    (*crate::src::botlib::be_aas_main::aasworld
                                        .vertexes
                                        .offset((*edge).v[1 as libc::c_int as usize] as isize))
                                        [0 as libc::c_int as usize]
                                        - (*crate::src::botlib::be_aas_main::aasworld
                                            .vertexes
                                            .offset((*edge).v[0 as libc::c_int as usize] as isize))
                                            [0 as libc::c_int as usize];
                                dir[1 as libc::c_int as usize] =
                                    (*crate::src::botlib::be_aas_main::aasworld
                                        .vertexes
                                        .offset((*edge).v[1 as libc::c_int as usize] as isize))
                                        [1 as libc::c_int as usize]
                                        - (*crate::src::botlib::be_aas_main::aasworld
                                            .vertexes
                                            .offset((*edge).v[0 as libc::c_int as usize] as isize))
                                            [1 as libc::c_int as usize];
                                dir[2 as libc::c_int as usize] =
                                    (*crate::src::botlib::be_aas_main::aasworld
                                        .vertexes
                                        .offset((*edge).v[1 as libc::c_int as usize] as isize))
                                        [2 as libc::c_int as usize]
                                        - (*crate::src::botlib::be_aas_main::aasworld
                                            .vertexes
                                            .offset((*edge).v[0 as libc::c_int as usize] as isize))
                                            [2 as libc::c_int as usize];
                                length =
                                    VectorLength(dir.as_mut_ptr()
                                        as *const crate::src::qcommon::q_shared::vec_t);
                                //get the start point
                                start[0 as libc::c_int as usize] =
                                    (*crate::src::botlib::be_aas_main::aasworld
                                        .vertexes
                                        .offset((*edge).v[0 as libc::c_int as usize] as isize))
                                        [0 as libc::c_int as usize]
                                        + (*crate::src::botlib::be_aas_main::aasworld
                                            .vertexes
                                            .offset((*edge).v[1 as libc::c_int as usize] as isize))
                                            [0 as libc::c_int as usize];
                                start[1 as libc::c_int as usize] =
                                    (*crate::src::botlib::be_aas_main::aasworld
                                        .vertexes
                                        .offset((*edge).v[0 as libc::c_int as usize] as isize))
                                        [1 as libc::c_int as usize]
                                        + (*crate::src::botlib::be_aas_main::aasworld
                                            .vertexes
                                            .offset((*edge).v[1 as libc::c_int as usize] as isize))
                                            [1 as libc::c_int as usize];
                                start[2 as libc::c_int as usize] =
                                    (*crate::src::botlib::be_aas_main::aasworld
                                        .vertexes
                                        .offset((*edge).v[0 as libc::c_int as usize] as isize))
                                        [2 as libc::c_int as usize]
                                        + (*crate::src::botlib::be_aas_main::aasworld
                                            .vertexes
                                            .offset((*edge).v[1 as libc::c_int as usize] as isize))
                                            [2 as libc::c_int as usize];
                                start[0 as libc::c_int as usize] =
                                    (start[0 as libc::c_int as usize] as libc::c_double * 0.5f64)
                                        as crate::src::qcommon::q_shared::vec_t;
                                start[1 as libc::c_int as usize] =
                                    (start[1 as libc::c_int as usize] as libc::c_double * 0.5f64)
                                        as crate::src::qcommon::q_shared::vec_t;
                                start[2 as libc::c_int as usize] =
                                    (start[2 as libc::c_int as usize] as libc::c_double * 0.5f64)
                                        as crate::src::qcommon::q_shared::vec_t;
                                end[0 as libc::c_int as usize] = start[0 as libc::c_int as usize];
                                end[1 as libc::c_int as usize] = start[1 as libc::c_int as usize];
                                end[2 as libc::c_int as usize] = start[2 as libc::c_int as usize];
                                //get the end point several units inside area2
                                //and the start point several units inside area1
                                //NOTE: normal is pointing into area2 because the
                                //face edges are stored counter clockwise
                                edgevec[0 as libc::c_int as usize] =
                                    (*crate::src::botlib::be_aas_main::aasworld
                                        .vertexes
                                        .offset((*edge).v[side as usize] as isize))
                                        [0 as libc::c_int as usize]
                                        - (*crate::src::botlib::be_aas_main::aasworld
                                            .vertexes
                                            .offset(
                                                (*edge).v[(side == 0) as libc::c_int as usize]
                                                    as isize,
                                            ))
                                            [0 as libc::c_int as usize];
                                edgevec[1 as libc::c_int as usize] =
                                    (*crate::src::botlib::be_aas_main::aasworld
                                        .vertexes
                                        .offset((*edge).v[side as usize] as isize))
                                        [1 as libc::c_int as usize]
                                        - (*crate::src::botlib::be_aas_main::aasworld
                                            .vertexes
                                            .offset(
                                                (*edge).v[(side == 0) as libc::c_int as usize]
                                                    as isize,
                                            ))
                                            [1 as libc::c_int as usize];
                                edgevec[2 as libc::c_int as usize] =
                                    (*crate::src::botlib::be_aas_main::aasworld
                                        .vertexes
                                        .offset((*edge).v[side as usize] as isize))
                                        [2 as libc::c_int as usize]
                                        - (*crate::src::botlib::be_aas_main::aasworld
                                            .vertexes
                                            .offset(
                                                (*edge).v[(side == 0) as libc::c_int as usize]
                                                    as isize,
                                            ))
                                            [2 as libc::c_int as usize];
                                plane2 = &mut *crate::src::botlib::be_aas_main::aasworld
                                    .planes
                                    .offset((*face2).planenum as isize)
                                    as *mut crate::aasfile_h::aas_plane_t;
                                CrossProduct(
                                    edgevec.as_mut_ptr()
                                        as *const crate::src::qcommon::q_shared::vec_t,
                                    (*plane2).normal.as_mut_ptr()
                                        as *const crate::src::qcommon::q_shared::vec_t,
                                    normal.as_mut_ptr(),
                                );
                                crate::src::qcommon::q_math::VectorNormalize(normal.as_mut_ptr());
                                //
                                //VectorMA(start, -1, normal, start);
                                end[0 as libc::c_int as usize] = end[0 as libc::c_int as usize]
                                    + normal[0 as libc::c_int as usize]
                                        * 5 as libc::c_int as libc::c_float;
                                end[1 as libc::c_int as usize] = end[1 as libc::c_int as usize]
                                    + normal[1 as libc::c_int as usize]
                                        * 5 as libc::c_int as libc::c_float;
                                end[2 as libc::c_int as usize] = end[2 as libc::c_int as usize]
                                    + normal[2 as libc::c_int as usize]
                                        * 5 as libc::c_int as libc::c_float;
                                start[0 as libc::c_int as usize] = (start[0 as libc::c_int as usize]
                                    as libc::c_double
                                    + normal[0 as libc::c_int as usize] as libc::c_double * 0.1f64)
                                    as crate::src::qcommon::q_shared::vec_t;
                                start[1 as libc::c_int as usize] = (start[1 as libc::c_int as usize]
                                    as libc::c_double
                                    + normal[1 as libc::c_int as usize] as libc::c_double * 0.1f64)
                                    as crate::src::qcommon::q_shared::vec_t;
                                start[2 as libc::c_int as usize] = (start[2 as libc::c_int as usize]
                                    as libc::c_double
                                    + normal[2 as libc::c_int as usize] as libc::c_double * 0.1f64)
                                    as crate::src::qcommon::q_shared::vec_t;
                                end[2 as libc::c_int as usize] =
                                    (end[2 as libc::c_int as usize] as libc::c_double + 0.125f64)
                                        as crate::src::qcommon::q_shared::vec_t;
                                //
                                height = invgravity[0 as libc::c_int as usize]
                                    * start[0 as libc::c_int as usize]
                                    + invgravity[1 as libc::c_int as usize]
                                        * start[1 as libc::c_int as usize]
                                    + invgravity[2 as libc::c_int as usize]
                                        * start[2 as libc::c_int as usize];
                                //NOTE: if there's nearby solid or a gap area after this area
                                //disabled this crap
                                //if (AAS_NearbySolidOrGap(start, end)) height += 200;
                                //NOTE: disabled because it disables reachabilities to very small areas
                                //if (AAS_PointAreaNum(end) != area2num) continue;
                                //get the longest lowest edge
                                if height < bestheight
                                    || height < bestheight + 1 as libc::c_int as libc::c_float
                                        && length > bestlength
                                {
                                    bestheight = height;
                                    bestlength = length;
                                    //create a new reachability link
                                    lr.areanum = area2num;
                                    lr.facenum = 0 as libc::c_int;
                                    lr.edgenum = edgenum;
                                    lr.start[0 as libc::c_int as usize] =
                                        start[0 as libc::c_int as usize];
                                    lr.start[1 as libc::c_int as usize] =
                                        start[1 as libc::c_int as usize];
                                    lr.start[2 as libc::c_int as usize] =
                                        start[2 as libc::c_int as usize];
                                    lr.end[0 as libc::c_int as usize] =
                                        end[0 as libc::c_int as usize];
                                    lr.end[1 as libc::c_int as usize] =
                                        end[1 as libc::c_int as usize];
                                    lr.end[2 as libc::c_int as usize] =
                                        end[2 as libc::c_int as usize];
                                    lr.traveltype = 2 as libc::c_int;
                                    lr.traveltime = 1 as libc::c_int as libc::c_ushort;
                                    foundreach = crate::src::qcommon::q_shared::qtrue as libc::c_int
                                }
                            }
                            edgenum2 += 1
                            //end if
                        }
                        edgenum1 += 1
                        //end for
                    }
                } //end if
                j += 1
            }
        }
        i += 1
    }
    if foundreach != 0 {
        //create a new reachability link
        lreach = AAS_AllocReachability();
        if lreach.is_null() {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        (*lreach).areanum = lr.areanum;
        (*lreach).facenum = lr.facenum;
        (*lreach).edgenum = lr.edgenum;
        (*lreach).start[0 as libc::c_int as usize] = lr.start[0 as libc::c_int as usize];
        (*lreach).start[1 as libc::c_int as usize] = lr.start[1 as libc::c_int as usize];
        (*lreach).start[2 as libc::c_int as usize] = lr.start[2 as libc::c_int as usize];
        (*lreach).end[0 as libc::c_int as usize] = lr.end[0 as libc::c_int as usize];
        (*lreach).end[1 as libc::c_int as usize] = lr.end[1 as libc::c_int as usize];
        (*lreach).end[2 as libc::c_int as usize] = lr.end[2 as libc::c_int as usize];
        (*lreach).traveltype = lr.traveltype;
        (*lreach).traveltime = lr.traveltime;
        (*lreach).next = *areareachability.offset(area1num as isize);
        let ref mut fresh6 = *areareachability.offset(area1num as isize);
        *fresh6 = lreach;
        //if going into a crouch area
        if AAS_AreaCrouch(area1num) == 0 && AAS_AreaCrouch(area2num) != 0 {
            (*lreach).traveltime = ((*lreach).traveltime as libc::c_float
                + crate::src::botlib::be_aas_move::aassettings.rs_startcrouch)
                as libc::c_ushort
        } //end if
          /*
          //NOTE: if there's nearby solid or a gap area after this area
          if (!AAS_NearbySolidOrGap(lreach->start, lreach->end))
          {
              lreach->traveltime += 100;
          } //end if
          */
        //avoid rather small areas
        //if (AAS_AreaGroundFaceArea(lreach->areanum) < 500) lreach->traveltime += 100;
        //
        reach_equalfloor += 1;
        return crate::src::qcommon::q_shared::qtrue as libc::c_int;
    }
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//end of the function AAS_Reachability_EqualFloorHeight
//===========================================================================
// searches step, barrier, waterjump and walk off ledge reachabilities
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Reachability_Step_Barrier_WaterJump_WalkOffLedge(
    mut area1num: libc::c_int,
    mut area2num: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut edge1num: libc::c_int = 0;
    let mut edge2num: libc::c_int = 0;
    let mut areas: [libc::c_int; 10] = [0; 10];
    let mut numareas: libc::c_int = 0;
    let mut ground_bestarea2groundedgenum: libc::c_int = 0;
    let mut ground_foundreach: libc::c_int = 0;
    let mut water_bestarea2groundedgenum: libc::c_int = 0;
    let mut water_foundreach: libc::c_int = 0;
    let mut side1: libc::c_int = 0;
    let mut area1swim: libc::c_int = 0;
    let mut faceside1: libc::c_int = 0;
    let mut groundface1num: libc::c_int = 0;
    let mut dist: libc::c_float = 0.;
    let mut dist1: libc::c_float = 0.;
    let mut dist2: libc::c_float = 0.;
    let mut diff: libc::c_float = 0.;
    let mut ortdot: libc::c_float = 0.;
    //float invgravitydot;
    let mut x1: libc::c_float = 0.;
    let mut x2: libc::c_float = 0.;
    let mut x3: libc::c_float = 0.;
    let mut x4: libc::c_float = 0.;
    let mut y1: libc::c_float = 0.;
    let mut y2: libc::c_float = 0.;
    let mut y3: libc::c_float = 0.;
    let mut y4: libc::c_float = 0.;
    let mut tmp: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut length: libc::c_float = 0.;
    let mut ground_bestlength: libc::c_float = 0.;
    let mut water_bestlength: libc::c_float = 0.;
    let mut ground_bestdist: libc::c_float = 0.;
    let mut water_bestdist: libc::c_float = 0.;
    let mut v1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v3: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v4: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut tmpv: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p1area1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p1area2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p2area1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p2area2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut normal: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut ort: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut edgevec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut ground_beststart: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut ground_bestend: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut ground_bestnormal: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut water_beststart: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut water_bestend: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut water_bestnormal: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut invgravity: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut testpoint: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut plane: *mut crate::aasfile_h::aas_plane_t = 0 as *mut crate::aasfile_h::aas_plane_t;
    let mut area1: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut area2: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut groundface1: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut groundface2: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut edge1: *mut crate::aasfile_h::aas_edge_t = 0 as *mut crate::aasfile_h::aas_edge_t;
    let mut edge2: *mut crate::aasfile_h::aas_edge_t = 0 as *mut crate::aasfile_h::aas_edge_t;
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut trace: crate::be_aas_h::aas_trace_t = crate::be_aas_h::aas_trace_t {
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        ent: 0,
        lastarea: 0,
        area: 0,
        planenum: 0,
    };
    //must be able to walk or swim in the first area
    if AAS_AreaGrounded(area1num) == 0 && AAS_AreaSwim(area1num) == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //
    if AAS_AreaGrounded(area2num) == 0 && AAS_AreaSwim(area2num) == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //
    area1 = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(area1num as isize) as *mut crate::aasfile_h::aas_area_t;
    area2 = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(area2num as isize) as *mut crate::aasfile_h::aas_area_t;
    //if the first area contains a liquid
    area1swim = AAS_AreaSwim(area1num);
    //if the areas are not near enough in the x-y direction
    i = 0 as libc::c_int; //end for
    while i < 2 as libc::c_int {
        if (*area1).mins[i as usize]
            > (*area2).maxs[i as usize] + 10 as libc::c_int as libc::c_float
        {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        if (*area1).maxs[i as usize]
            < (*area2).mins[i as usize] - 10 as libc::c_int as libc::c_float
        {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        i += 1
    }
    //
    ground_foundreach = crate::src::qcommon::q_shared::qfalse as libc::c_int;
    ground_bestdist = 99999 as libc::c_int as libc::c_float;
    ground_bestlength = 0 as libc::c_int as libc::c_float;
    ground_bestarea2groundedgenum = 0 as libc::c_int;
    //
    water_foundreach = crate::src::qcommon::q_shared::qfalse as libc::c_int;
    water_bestdist = 99999 as libc::c_int as libc::c_float;
    water_bestlength = 0 as libc::c_int as libc::c_float;
    water_bestarea2groundedgenum = 0 as libc::c_int;
    let mut current_block_168: u64;
    //
    i = 0 as libc::c_int; //end for
    while i < (*area1).numfaces {
        groundface1num = *crate::src::botlib::be_aas_main::aasworld
            .faceindex
            .offset(((*area1).firstface + i) as isize);
        faceside1 = (groundface1num < 0 as libc::c_int) as libc::c_int;
        groundface1 =
            &mut *crate::src::botlib::be_aas_main::aasworld
                .faces
                .offset(
                    (crate::stdlib::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(
                        groundface1num,
                    ) as isize,
                ) as *mut crate::aasfile_h::aas_face_t;
        //end for
        //if this isn't a ground face
        if (*groundface1).faceflags & 4 as libc::c_int == 0 {
            //end if
            //if we can swim in the first area
            if area1swim != 0 {
                plane = &mut *crate::src::botlib::be_aas_main::aasworld
                    .planes
                    .offset(((*groundface1).planenum ^ (faceside1 == 0) as libc::c_int) as isize)
                    as *mut crate::aasfile_h::aas_plane_t; //end if
                if (((*plane).normal[0 as libc::c_int as usize]
                    * invgravity[0 as libc::c_int as usize]
                    + (*plane).normal[1 as libc::c_int as usize]
                        * invgravity[1 as libc::c_int as usize]
                    + (*plane).normal[2 as libc::c_int as usize]
                        * invgravity[2 as libc::c_int as usize])
                    as libc::c_double)
                    < 0.7f64
                {
                    current_block_168 = 8693738493027456495;
                } else {
                    current_block_168 = 6450597802325118133;
                }
            } else {
                current_block_168 = 8693738493027456495;
            }
        //face plane must be more or less horizontal
        //end else
        } else {
            current_block_168 = 6450597802325118133;
        }
        match current_block_168 {
            6450597802325118133 => {
                //
                k = 0 as libc::c_int;
                while k < (*groundface1).numedges {
                    edge1num = *crate::src::botlib::be_aas_main::aasworld
                        .edgeindex
                        .offset(((*groundface1).firstedge + k) as isize);
                    side1 = (edge1num < 0 as libc::c_int) as libc::c_int;
                    //end for
                    if (*groundface1).faceflags & 4 as libc::c_int == 0 {
                        side1 = (side1 == faceside1) as libc::c_int
                    }
                    edge1num = crate::stdlib::abs(edge1num);
                    edge1 = &mut *crate::src::botlib::be_aas_main::aasworld
                        .edges
                        .offset(edge1num as isize)
                        as *mut crate::aasfile_h::aas_edge_t;
                    v1[0 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
                        .vertexes
                        .offset((*edge1).v[(side1 == 0) as libc::c_int as usize] as isize))
                        [0 as libc::c_int as usize];
                    v1[1 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
                        .vertexes
                        .offset((*edge1).v[(side1 == 0) as libc::c_int as usize] as isize))
                        [1 as libc::c_int as usize];
                    v1[2 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
                        .vertexes
                        .offset((*edge1).v[(side1 == 0) as libc::c_int as usize] as isize))
                        [2 as libc::c_int as usize];
                    v2[0 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
                        .vertexes
                        .offset((*edge1).v[side1 as usize] as isize))
                        [0 as libc::c_int as usize];
                    v2[1 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
                        .vertexes
                        .offset((*edge1).v[side1 as usize] as isize))
                        [1 as libc::c_int as usize];
                    v2[2 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
                        .vertexes
                        .offset((*edge1).v[side1 as usize] as isize))
                        [2 as libc::c_int as usize];
                    edgevec[0 as libc::c_int as usize] =
                        v2[0 as libc::c_int as usize] - v1[0 as libc::c_int as usize];
                    edgevec[1 as libc::c_int as usize] =
                        v2[1 as libc::c_int as usize] - v1[1 as libc::c_int as usize];
                    edgevec[2 as libc::c_int as usize] =
                        v2[2 as libc::c_int as usize] - v1[2 as libc::c_int as usize];
                    CrossProduct(
                        edgevec.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                        invgravity.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                        normal.as_mut_ptr(),
                    );
                    crate::src::qcommon::q_math::VectorNormalize(normal.as_mut_ptr());
                    dist = normal[0 as libc::c_int as usize] * v1[0 as libc::c_int as usize]
                        + normal[1 as libc::c_int as usize] * v1[1 as libc::c_int as usize]
                        + normal[2 as libc::c_int as usize] * v1[2 as libc::c_int as usize];
                    j = 0 as libc::c_int;
                    while j < (*area2).numfaces {
                        groundface2 = &mut *crate::src::botlib::be_aas_main::aasworld.faces.offset(
                            (crate::stdlib::abs
                                as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(
                                *crate::src::botlib::be_aas_main::aasworld
                                    .faceindex
                                    .offset(((*area2).firstface + j) as isize),
                            ) as isize,
                        )
                            as *mut crate::aasfile_h::aas_face_t;
                        //NOTE: for water faces we must take the side area 1 is
                        // on into account because the face is shared and doesn't
                        // have to be oriented correctly
                        //vertexes of the edge
                        //get a vertical plane through the edge
                        //NOTE: normal is pointing into area 2 because the
                        //face edges are stored counter clockwise
                        //check the faces from the second area
                        //end for
                        //must be a ground face
                        if !((*groundface2).faceflags & 4 as libc::c_int == 0) {
                            //check the edges of this ground face
                            l = 0 as libc::c_int;
                            while l < (*groundface2).numedges {
                                edge2num = crate::stdlib::abs(
                                    *crate::src::botlib::be_aas_main::aasworld
                                        .edgeindex
                                        .offset(((*groundface2).firstedge + l) as isize),
                                );
                                edge2 = &mut *crate::src::botlib::be_aas_main::aasworld
                                    .edges
                                    .offset(edge2num as isize)
                                    as *mut crate::aasfile_h::aas_edge_t;
                                //end else
                                //vertexes of the edge
                                v3[0 as libc::c_int as usize] =
                                    (*crate::src::botlib::be_aas_main::aasworld
                                        .vertexes
                                        .offset((*edge2).v[0 as libc::c_int as usize] as isize))
                                        [0 as libc::c_int as usize];
                                v3[1 as libc::c_int as usize] =
                                    (*crate::src::botlib::be_aas_main::aasworld
                                        .vertexes
                                        .offset((*edge2).v[0 as libc::c_int as usize] as isize))
                                        [1 as libc::c_int as usize];
                                v3[2 as libc::c_int as usize] =
                                    (*crate::src::botlib::be_aas_main::aasworld
                                        .vertexes
                                        .offset((*edge2).v[0 as libc::c_int as usize] as isize))
                                        [2 as libc::c_int as usize];
                                v4[0 as libc::c_int as usize] =
                                    (*crate::src::botlib::be_aas_main::aasworld
                                        .vertexes
                                        .offset((*edge2).v[1 as libc::c_int as usize] as isize))
                                        [0 as libc::c_int as usize];
                                v4[1 as libc::c_int as usize] =
                                    (*crate::src::botlib::be_aas_main::aasworld
                                        .vertexes
                                        .offset((*edge2).v[1 as libc::c_int as usize] as isize))
                                        [1 as libc::c_int as usize];
                                v4[2 as libc::c_int as usize] =
                                    (*crate::src::botlib::be_aas_main::aasworld
                                        .vertexes
                                        .offset((*edge2).v[1 as libc::c_int as usize] as isize))
                                        [2 as libc::c_int as usize];
                                //check the distance between the two points and the vertical plane
                                //through the edge of area1
                                diff = normal[0 as libc::c_int as usize]
                                    * v3[0 as libc::c_int as usize]
                                    + normal[1 as libc::c_int as usize]
                                        * v3[1 as libc::c_int as usize]
                                    + normal[2 as libc::c_int as usize]
                                        * v3[2 as libc::c_int as usize]
                                    - dist;
                                if !((diff as libc::c_double) < -0.1f64
                                    || diff as libc::c_double > 0.1f64)
                                {
                                    diff = normal[0 as libc::c_int as usize]
                                        * v4[0 as libc::c_int as usize]
                                        + normal[1 as libc::c_int as usize]
                                            * v4[1 as libc::c_int as usize]
                                        + normal[2 as libc::c_int as usize]
                                            * v4[2 as libc::c_int as usize]
                                        - dist;
                                    if !((diff as libc::c_double) < -0.1f64
                                        || diff as libc::c_double > 0.1f64)
                                    {
                                        //
                                        //project the two ground edges into the step side plane
                                        //and calculate the shortest distance between the two
                                        //edges if they overlap in the direction orthogonal to
                                        //the gravity direction
                                        CrossProduct(
                                            invgravity.as_mut_ptr()
                                                as *const crate::src::qcommon::q_shared::vec_t,
                                            normal.as_mut_ptr()
                                                as *const crate::src::qcommon::q_shared::vec_t,
                                            ort.as_mut_ptr(),
                                        );
                                        //invgravitydot = DotProduct(invgravity, invgravity);
                                        ortdot = ort[0 as libc::c_int as usize]
                                            * ort[0 as libc::c_int as usize]
                                            + ort[1 as libc::c_int as usize]
                                                * ort[1 as libc::c_int as usize]
                                            + ort[2 as libc::c_int as usize]
                                                * ort[2 as libc::c_int as usize];
                                        //projection into the step plane
                                        //NOTE: since gravity is vertical this is just the z coordinate
                                        y1 = v1[2 as libc::c_int as usize]; //DotProduct(v1, invgravity) / invgravitydot;
                                        y2 = v2[2 as libc::c_int as usize]; //DotProduct(v2, invgravity) / invgravitydot;
                                        y3 = v3[2 as libc::c_int as usize]; //DotProduct(v3, invgravity) / invgravitydot;
                                        y4 = v4[2 as libc::c_int as usize]; //DotProduct(v4, invgravity) / invgravitydot;
                                                                            //
                                        x1 = (v1[0 as libc::c_int as usize]
                                            * ort[0 as libc::c_int as usize]
                                            + v1[1 as libc::c_int as usize]
                                                * ort[1 as libc::c_int as usize]
                                            + v1[2 as libc::c_int as usize]
                                                * ort[2 as libc::c_int as usize])
                                            / ortdot;
                                        x2 = (v2[0 as libc::c_int as usize]
                                            * ort[0 as libc::c_int as usize]
                                            + v2[1 as libc::c_int as usize]
                                                * ort[1 as libc::c_int as usize]
                                            + v2[2 as libc::c_int as usize]
                                                * ort[2 as libc::c_int as usize])
                                            / ortdot;
                                        x3 = (v3[0 as libc::c_int as usize]
                                            * ort[0 as libc::c_int as usize]
                                            + v3[1 as libc::c_int as usize]
                                                * ort[1 as libc::c_int as usize]
                                            + v3[2 as libc::c_int as usize]
                                                * ort[2 as libc::c_int as usize])
                                            / ortdot;
                                        x4 = (v4[0 as libc::c_int as usize]
                                            * ort[0 as libc::c_int as usize]
                                            + v4[1 as libc::c_int as usize]
                                                * ort[1 as libc::c_int as usize]
                                            + v4[2 as libc::c_int as usize]
                                                * ort[2 as libc::c_int as usize])
                                            / ortdot;
                                        //
                                        if x1 > x2 {
                                            tmp = x1; //end if
                                            x1 = x2; //end if
                                            x2 = tmp;
                                            tmp = y1;
                                            y1 = y2;
                                            y2 = tmp;
                                            tmpv[0 as libc::c_int as usize] =
                                                v1[0 as libc::c_int as usize];
                                            tmpv[1 as libc::c_int as usize] =
                                                v1[1 as libc::c_int as usize];
                                            tmpv[2 as libc::c_int as usize] =
                                                v1[2 as libc::c_int as usize];
                                            v1[0 as libc::c_int as usize] =
                                                v2[0 as libc::c_int as usize];
                                            v1[1 as libc::c_int as usize] =
                                                v2[1 as libc::c_int as usize];
                                            v1[2 as libc::c_int as usize] =
                                                v2[2 as libc::c_int as usize];
                                            v2[0 as libc::c_int as usize] =
                                                tmpv[0 as libc::c_int as usize];
                                            v2[1 as libc::c_int as usize] =
                                                tmpv[1 as libc::c_int as usize];
                                            v2[2 as libc::c_int as usize] =
                                                tmpv[2 as libc::c_int as usize]
                                        }
                                        if x3 > x4 {
                                            tmp = x3;
                                            x3 = x4;
                                            x4 = tmp;
                                            tmp = y3;
                                            y3 = y4;
                                            y4 = tmp;
                                            tmpv[0 as libc::c_int as usize] =
                                                v3[0 as libc::c_int as usize];
                                            tmpv[1 as libc::c_int as usize] =
                                                v3[1 as libc::c_int as usize];
                                            tmpv[2 as libc::c_int as usize] =
                                                v3[2 as libc::c_int as usize];
                                            v3[0 as libc::c_int as usize] =
                                                v4[0 as libc::c_int as usize];
                                            v3[1 as libc::c_int as usize] =
                                                v4[1 as libc::c_int as usize];
                                            v3[2 as libc::c_int as usize] =
                                                v4[2 as libc::c_int as usize];
                                            v4[0 as libc::c_int as usize] =
                                                tmpv[0 as libc::c_int as usize];
                                            v4[1 as libc::c_int as usize] =
                                                tmpv[1 as libc::c_int as usize];
                                            v4[2 as libc::c_int as usize] =
                                                tmpv[2 as libc::c_int as usize]
                                        }
                                        //if the two projected edge lines have no overlap
                                        if !(x2 <= x3 || x4 <= x1) {
                                            //end if
                                            //if the two lines fully overlap
                                            if x1 as libc::c_double - 0.5f64 < x3 as libc::c_double
                                                && (x4 as libc::c_double)
                                                    < x2 as libc::c_double + 0.5f64
                                                && (x3 as libc::c_double - 0.5f64
                                                    < x1 as libc::c_double
                                                    && (x2 as libc::c_double)
                                                        < x4 as libc::c_double + 0.5f64)
                                            {
                                                //end else
                                                dist1 = y3 - y1; //end if
                                                dist2 = y4 - y2;
                                                p1area1[0 as libc::c_int as usize] =
                                                    v1[0 as libc::c_int as usize];
                                                p1area1[1 as libc::c_int as usize] =
                                                    v1[1 as libc::c_int as usize];
                                                p1area1[2 as libc::c_int as usize] =
                                                    v1[2 as libc::c_int as usize];
                                                p2area1[0 as libc::c_int as usize] =
                                                    v2[0 as libc::c_int as usize];
                                                p2area1[1 as libc::c_int as usize] =
                                                    v2[1 as libc::c_int as usize];
                                                p2area1[2 as libc::c_int as usize] =
                                                    v2[2 as libc::c_int as usize];
                                                p1area2[0 as libc::c_int as usize] =
                                                    v3[0 as libc::c_int as usize];
                                                p1area2[1 as libc::c_int as usize] =
                                                    v3[1 as libc::c_int as usize];
                                                p1area2[2 as libc::c_int as usize] =
                                                    v3[2 as libc::c_int as usize];
                                                p2area2[0 as libc::c_int as usize] =
                                                    v4[0 as libc::c_int as usize];
                                                p2area2[1 as libc::c_int as usize] =
                                                    v4[1 as libc::c_int as usize];
                                                p2area2[2 as libc::c_int as usize] =
                                                    v4[2 as libc::c_int as usize]
                                            } else {
                                                //if the points are equal
                                                if x1 as libc::c_double
                                                    > x3 as libc::c_double - 0.1f64
                                                    && (x1 as libc::c_double)
                                                        < x3 as libc::c_double + 0.1f64
                                                {
                                                    //end if
                                                    dist1 = y3 - y1; //end if
                                                    p1area1[0 as libc::c_int as usize] =
                                                        v1[0 as libc::c_int as usize]; //end if
                                                    p1area1[1 as libc::c_int as usize] =
                                                        v1[1 as libc::c_int as usize];
                                                    p1area1[2 as libc::c_int as usize] =
                                                        v1[2 as libc::c_int as usize];
                                                    p1area2[0 as libc::c_int as usize] =
                                                        v3[0 as libc::c_int as usize];
                                                    p1area2[1 as libc::c_int as usize] =
                                                        v3[1 as libc::c_int as usize];
                                                    p1area2[2 as libc::c_int as usize] =
                                                        v3[2 as libc::c_int as usize]
                                                } else if x1 < x3 {
                                                    y = y1 + (x3 - x1) * (y2 - y1) / (x2 - x1);
                                                    dist1 = y3 - y;
                                                    p1area1[0 as libc::c_int as usize] =
                                                        v3[0 as libc::c_int as usize];
                                                    p1area1[1 as libc::c_int as usize] =
                                                        v3[1 as libc::c_int as usize];
                                                    p1area1[2 as libc::c_int as usize] =
                                                        v3[2 as libc::c_int as usize];
                                                    p1area1[2 as libc::c_int as usize] = y;
                                                    p1area2[0 as libc::c_int as usize] =
                                                        v3[0 as libc::c_int as usize];
                                                    p1area2[1 as libc::c_int as usize] =
                                                        v3[1 as libc::c_int as usize];
                                                    p1area2[2 as libc::c_int as usize] =
                                                        v3[2 as libc::c_int as usize]
                                                } else {
                                                    y = y3 + (x1 - x3) * (y4 - y3) / (x4 - x3);
                                                    dist1 = y - y1;
                                                    p1area1[0 as libc::c_int as usize] =
                                                        v1[0 as libc::c_int as usize];
                                                    p1area1[1 as libc::c_int as usize] =
                                                        v1[1 as libc::c_int as usize];
                                                    p1area1[2 as libc::c_int as usize] =
                                                        v1[2 as libc::c_int as usize];
                                                    p1area2[0 as libc::c_int as usize] =
                                                        v1[0 as libc::c_int as usize];
                                                    p1area2[1 as libc::c_int as usize] =
                                                        v1[1 as libc::c_int as usize];
                                                    p1area2[2 as libc::c_int as usize] =
                                                        v1[2 as libc::c_int as usize];
                                                    p1area2[2 as libc::c_int as usize] = y
                                                }
                                                //end else
                                                if x2 as libc::c_double
                                                    > x4 as libc::c_double - 0.1f64
                                                    && (x2 as libc::c_double)
                                                        < x4 as libc::c_double + 0.1f64
                                                {
                                                    //if the points are equal
                                                    dist2 = y4 - y2; //end if
                                                    p2area1[0 as libc::c_int as usize] =
                                                        v2[0 as libc::c_int as usize]; //end if
                                                    p2area1[1 as libc::c_int as usize] =
                                                        v2[1 as libc::c_int as usize];
                                                    p2area1[2 as libc::c_int as usize] =
                                                        v2[2 as libc::c_int as usize];
                                                    p2area2[0 as libc::c_int as usize] =
                                                        v4[0 as libc::c_int as usize];
                                                    p2area2[1 as libc::c_int as usize] =
                                                        v4[1 as libc::c_int as usize];
                                                    p2area2[2 as libc::c_int as usize] =
                                                        v4[2 as libc::c_int as usize]
                                                } else if x2 < x4 {
                                                    y = y3 + (x2 - x3) * (y4 - y3) / (x4 - x3);
                                                    dist2 = y - y2;
                                                    p2area1[0 as libc::c_int as usize] =
                                                        v2[0 as libc::c_int as usize];
                                                    p2area1[1 as libc::c_int as usize] =
                                                        v2[1 as libc::c_int as usize];
                                                    p2area1[2 as libc::c_int as usize] =
                                                        v2[2 as libc::c_int as usize];
                                                    p2area2[0 as libc::c_int as usize] =
                                                        v2[0 as libc::c_int as usize];
                                                    p2area2[1 as libc::c_int as usize] =
                                                        v2[1 as libc::c_int as usize];
                                                    p2area2[2 as libc::c_int as usize] =
                                                        v2[2 as libc::c_int as usize];
                                                    p2area2[2 as libc::c_int as usize] = y
                                                } else {
                                                    y = y1 + (x4 - x1) * (y2 - y1) / (x2 - x1);
                                                    dist2 = y4 - y;
                                                    p2area1[0 as libc::c_int as usize] =
                                                        v4[0 as libc::c_int as usize];
                                                    p2area1[1 as libc::c_int as usize] =
                                                        v4[1 as libc::c_int as usize];
                                                    p2area1[2 as libc::c_int as usize] =
                                                        v4[2 as libc::c_int as usize];
                                                    p2area1[2 as libc::c_int as usize] = y;
                                                    p2area2[0 as libc::c_int as usize] =
                                                        v4[0 as libc::c_int as usize];
                                                    p2area2[1 as libc::c_int as usize] =
                                                        v4[1 as libc::c_int as usize];
                                                    p2area2[2 as libc::c_int as usize] =
                                                        v4[2 as libc::c_int as usize]
                                                }
                                            }
                                            //if both distances are pretty much equal
                                            //then we take the middle of the points
                                            if dist1 > dist2 - 1 as libc::c_int as libc::c_float
                                                && dist1 < dist2 + 1 as libc::c_int as libc::c_float
                                            {
                                                //end else
                                                dist = dist1; //end if
                                                start[0 as libc::c_int as usize] = p1area1
                                                    [0 as libc::c_int as usize]
                                                    + p2area1[0 as libc::c_int as usize]; //end else if
                                                start[1 as libc::c_int as usize] = p1area1
                                                    [1 as libc::c_int as usize]
                                                    + p2area1[1 as libc::c_int as usize];
                                                start[2 as libc::c_int as usize] = p1area1
                                                    [2 as libc::c_int as usize]
                                                    + p2area1[2 as libc::c_int as usize];
                                                start[0 as libc::c_int as usize] = (start
                                                    [0 as libc::c_int as usize]
                                                    as libc::c_double
                                                    * 0.5f64)
                                                    as crate::src::qcommon::q_shared::vec_t;
                                                start[1 as libc::c_int as usize] = (start
                                                    [1 as libc::c_int as usize]
                                                    as libc::c_double
                                                    * 0.5f64)
                                                    as crate::src::qcommon::q_shared::vec_t;
                                                start[2 as libc::c_int as usize] = (start
                                                    [2 as libc::c_int as usize]
                                                    as libc::c_double
                                                    * 0.5f64)
                                                    as crate::src::qcommon::q_shared::vec_t;
                                                end[0 as libc::c_int as usize] = p1area2
                                                    [0 as libc::c_int as usize]
                                                    + p2area2[0 as libc::c_int as usize];
                                                end[1 as libc::c_int as usize] = p1area2
                                                    [1 as libc::c_int as usize]
                                                    + p2area2[1 as libc::c_int as usize];
                                                end[2 as libc::c_int as usize] = p1area2
                                                    [2 as libc::c_int as usize]
                                                    + p2area2[2 as libc::c_int as usize];
                                                end[0 as libc::c_int as usize] = (end
                                                    [0 as libc::c_int as usize]
                                                    as libc::c_double
                                                    * 0.5f64)
                                                    as crate::src::qcommon::q_shared::vec_t;
                                                end[1 as libc::c_int as usize] = (end
                                                    [1 as libc::c_int as usize]
                                                    as libc::c_double
                                                    * 0.5f64)
                                                    as crate::src::qcommon::q_shared::vec_t;
                                                end[2 as libc::c_int as usize] = (end
                                                    [2 as libc::c_int as usize]
                                                    as libc::c_double
                                                    * 0.5f64)
                                                    as crate::src::qcommon::q_shared::vec_t
                                            } else if dist1 < dist2 {
                                                dist = dist1;
                                                start[0 as libc::c_int as usize] =
                                                    p1area1[0 as libc::c_int as usize];
                                                start[1 as libc::c_int as usize] =
                                                    p1area1[1 as libc::c_int as usize];
                                                start[2 as libc::c_int as usize] =
                                                    p1area1[2 as libc::c_int as usize];
                                                end[0 as libc::c_int as usize] =
                                                    p1area2[0 as libc::c_int as usize];
                                                end[1 as libc::c_int as usize] =
                                                    p1area2[1 as libc::c_int as usize];
                                                end[2 as libc::c_int as usize] =
                                                    p1area2[2 as libc::c_int as usize]
                                            } else {
                                                dist = dist2;
                                                start[0 as libc::c_int as usize] =
                                                    p2area1[0 as libc::c_int as usize];
                                                start[1 as libc::c_int as usize] =
                                                    p2area1[1 as libc::c_int as usize];
                                                start[2 as libc::c_int as usize] =
                                                    p2area1[2 as libc::c_int as usize];
                                                end[0 as libc::c_int as usize] =
                                                    p2area2[0 as libc::c_int as usize];
                                                end[1 as libc::c_int as usize] =
                                                    p2area2[1 as libc::c_int as usize];
                                                end[2 as libc::c_int as usize] =
                                                    p2area2[2 as libc::c_int as usize]
                                            }
                                            //get the length of the overlapping part of the edges of the two areas
                                            dir[0 as libc::c_int as usize] = p2area2
                                                [0 as libc::c_int as usize]
                                                - p1area2[0 as libc::c_int as usize];
                                            dir[1 as libc::c_int as usize] = p2area2
                                                [1 as libc::c_int as usize]
                                                - p1area2[1 as libc::c_int as usize];
                                            dir[2 as libc::c_int as usize] = p2area2
                                                [2 as libc::c_int as usize]
                                                - p1area2[2 as libc::c_int as usize];
                                            length = VectorLength(dir.as_mut_ptr()
                                                as *const crate::src::qcommon::q_shared::vec_t);
                                            //
                                            if (*groundface1).faceflags & 4 as libc::c_int != 0 {
                                                //if the vertical distance is smaller
                                                if dist < ground_bestdist
                                                    || dist
                                                        < ground_bestdist
                                                            + 1 as libc::c_int as libc::c_float
                                                        && length > ground_bestlength
                                                {
                                                    ground_bestdist = dist;
                                                    ground_bestlength = length;
                                                    ground_foundreach =
                                                        crate::src::qcommon::q_shared::qtrue
                                                            as libc::c_int;
                                                    ground_bestarea2groundedgenum = edge1num;
                                                    //best point towards area1
                                                    ground_beststart[0 as libc::c_int as usize] =
                                                        start[0 as libc::c_int as usize];
                                                    ground_beststart[1 as libc::c_int as usize] =
                                                        start[1 as libc::c_int as usize];
                                                    ground_beststart[2 as libc::c_int as usize] =
                                                        start[2 as libc::c_int as usize];
                                                    //normal is pointing into area2
                                                    ground_bestnormal[0 as libc::c_int as usize] =
                                                        normal[0 as libc::c_int as usize];
                                                    ground_bestnormal[1 as libc::c_int as usize] =
                                                        normal[1 as libc::c_int as usize];
                                                    ground_bestnormal[2 as libc::c_int as usize] =
                                                        normal[2 as libc::c_int as usize];
                                                    //best point towards area2
                                                    ground_bestend[0 as libc::c_int as usize] =
                                                        end[0 as libc::c_int as usize];
                                                    ground_bestend[1 as libc::c_int as usize] =
                                                        end[1 as libc::c_int as usize];
                                                    ground_bestend[2 as libc::c_int as usize] =
                                                        end[2 as libc::c_int as usize]
                                                }
                                            //end if
                                            } else if dist < water_bestdist
                                                || dist
                                                    < water_bestdist
                                                        + 1 as libc::c_int as libc::c_float
                                                    && length > water_bestlength
                                            {
                                                water_bestdist = dist;
                                                water_bestlength = length;
                                                water_foundreach =
                                                    crate::src::qcommon::q_shared::qtrue
                                                        as libc::c_int;
                                                water_bestarea2groundedgenum = edge1num;
                                                //if the vertical distance is smaller
                                                //best point towards area1
                                                water_beststart[0 as libc::c_int as usize] =
                                                    start[0 as libc::c_int as usize];
                                                water_beststart[1 as libc::c_int as usize] =
                                                    start[1 as libc::c_int as usize];
                                                water_beststart[2 as libc::c_int as usize] =
                                                    start[2 as libc::c_int as usize];
                                                //normal is pointing into area2
                                                water_bestnormal[0 as libc::c_int as usize] =
                                                    normal[0 as libc::c_int as usize];
                                                water_bestnormal[1 as libc::c_int as usize] =
                                                    normal[1 as libc::c_int as usize];
                                                water_bestnormal[2 as libc::c_int as usize] =
                                                    normal[2 as libc::c_int as usize];
                                                //best point towards area2
                                                water_bestend[0 as libc::c_int as usize] =
                                                    end[0 as libc::c_int as usize];
                                                water_bestend[1 as libc::c_int as usize] =
                                                    end[1 as libc::c_int as usize];
                                                water_bestend[2 as libc::c_int as usize] =
                                                    end[2 as libc::c_int as usize]
                                            }
                                        }
                                    }
                                }
                                l += 1
                            }
                        }
                        j += 1
                    }
                    k += 1
                }
            }
            _ => {}
        }
        i += 1
    }
    //end if
    //						Log_Write("lines no overlap: from area %d to %d\r\n", area1num, area2num);
    //if we can't swim in the area it must be a ground face
    //
    // NOTE: swim reachabilities are already filtered out
    //
    // Steps
    //
    //        ---------
    //        |          step height -> TRAVEL_WALK
    //--------|
    //
    //        ---------
    //~~~~~~~~|          step height and low water -> TRAVEL_WALK
    //--------|
    //
    //~~~~~~~~~~~~~~~~~~
    //        ---------
    //        |          step height and low water up to the step -> TRAVEL_WALK
    //--------|
    //
    //check for a step reachability
    if ground_foundreach != 0 {
        //end if
        //if area2 is higher but lower than the maximum step height
        //NOTE: ground_bestdist >= 0 also catches equal floor reachabilities
        if ground_bestdist >= 0 as libc::c_int as libc::c_float
            && ground_bestdist < crate::src::botlib::be_aas_move::aassettings.phys_maxstep
        {
            //create walk reachability from area1 to area2
            lreach = AAS_AllocReachability(); //1;
            if lreach.is_null() {
                return crate::src::qcommon::q_shared::qfalse as libc::c_int;
            }
            (*lreach).areanum = area2num;
            (*lreach).facenum = 0 as libc::c_int;
            (*lreach).edgenum = ground_bestarea2groundedgenum;
            (*lreach).start[0 as libc::c_int as usize] =
                (ground_beststart[0 as libc::c_int as usize] as libc::c_double
                    + ground_bestnormal[0 as libc::c_int as usize] as libc::c_double * 0.1f64)
                    as crate::src::qcommon::q_shared::vec_t;
            (*lreach).start[1 as libc::c_int as usize] =
                (ground_beststart[1 as libc::c_int as usize] as libc::c_double
                    + ground_bestnormal[1 as libc::c_int as usize] as libc::c_double * 0.1f64)
                    as crate::src::qcommon::q_shared::vec_t;
            (*lreach).start[2 as libc::c_int as usize] =
                (ground_beststart[2 as libc::c_int as usize] as libc::c_double
                    + ground_bestnormal[2 as libc::c_int as usize] as libc::c_double * 0.1f64)
                    as crate::src::qcommon::q_shared::vec_t;
            (*lreach).end[0 as libc::c_int as usize] = ground_bestend[0 as libc::c_int as usize]
                + ground_bestnormal[0 as libc::c_int as usize] * 5 as libc::c_int as libc::c_float;
            (*lreach).end[1 as libc::c_int as usize] = ground_bestend[1 as libc::c_int as usize]
                + ground_bestnormal[1 as libc::c_int as usize] * 5 as libc::c_int as libc::c_float;
            (*lreach).end[2 as libc::c_int as usize] = ground_bestend[2 as libc::c_int as usize]
                + ground_bestnormal[2 as libc::c_int as usize] * 5 as libc::c_int as libc::c_float;
            (*lreach).traveltype = 2 as libc::c_int;
            (*lreach).traveltime = 0 as libc::c_int as libc::c_ushort;
            //if going into a crouch area
            if AAS_AreaCrouch(area1num) == 0 && AAS_AreaCrouch(area2num) != 0 {
                (*lreach).traveltime = ((*lreach).traveltime as libc::c_float
                    + crate::src::botlib::be_aas_move::aassettings.rs_startcrouch)
                    as libc::c_ushort
            } //end if
            (*lreach).next = *areareachability.offset(area1num as isize);
            let ref mut fresh7 = *areareachability.offset(area1num as isize);
            *fresh7 = lreach;
            //NOTE: if there's nearby solid or a gap area after this area
            /*
            if (!AAS_NearbySolidOrGap(lreach->start, lreach->end))
            {
                lreach->traveltime += 100;
            } //end if
            */
            //avoid rather small areas
            //if (AAS_AreaGroundFaceArea(lreach->areanum) < 500) lreach->traveltime += 100;
            //
            reach_step += 1;
            return crate::src::qcommon::q_shared::qtrue as libc::c_int;
        }
        //end if
    }
    //
    // Water Jumps
    //
    //        ---------
    //        |
    //~~~~~~~~|
    //        |
    //        |          higher than step height and water up to waterjump height -> TRAVEL_WATERJUMP
    //--------|
    //
    //~~~~~~~~~~~~~~~~~~
    //        ---------
    //        |
    //        |
    //        |
    //        |          higher than step height and low water up to the step -> TRAVEL_WATERJUMP
    //--------|
    //
    //check for a waterjump reachability
    if water_foundreach != 0 {
        //end if
        //get a test point a little bit towards area1
        testpoint[0 as libc::c_int as usize] = water_bestend[0 as libc::c_int as usize]
            + water_bestnormal[0 as libc::c_int as usize] * -(2 as libc::c_int) as libc::c_float;
        testpoint[1 as libc::c_int as usize] = water_bestend[1 as libc::c_int as usize]
            + water_bestnormal[1 as libc::c_int as usize] * -(2 as libc::c_int) as libc::c_float;
        testpoint[2 as libc::c_int as usize] = water_bestend[2 as libc::c_int as usize]
            + water_bestnormal[2 as libc::c_int as usize] * -(2 as libc::c_int) as libc::c_float;
        //end if
        testpoint[2 as libc::c_int as usize] -=
            crate::src::botlib::be_aas_move::aassettings.phys_maxwaterjump;
        if (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(
                crate::src::botlib::be_aas_sample::AAS_PointAreaNum(testpoint.as_mut_ptr())
                    as isize,
            ))
        .areaflags
            & 4 as libc::c_int
            != 0
        {
            //go down the maximum waterjump height
            //if there IS water the sv_maxwaterjump height below the bestend point
            //don't create ridiculous water jump reachabilities from areas very far below
            //the water surface
            if water_bestdist
                < crate::src::botlib::be_aas_move::aassettings.phys_maxwaterjump
                    + 24 as libc::c_int as libc::c_float
            {
                //waterjumping from or towards a crouch only area is not possible in Quake2
                if (*crate::src::botlib::be_aas_main::aasworld
                    .areasettings
                    .offset(area1num as isize))
                .presencetype
                    & 2 as libc::c_int
                    != 0
                    && (*crate::src::botlib::be_aas_main::aasworld
                        .areasettings
                        .offset(area2num as isize))
                    .presencetype
                        & 2 as libc::c_int
                        != 0
                {
                    //create water jump reachability from area1 to area2
                    lreach = AAS_AllocReachability();
                    if lreach.is_null() {
                        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                    }
                    (*lreach).areanum = area2num;
                    (*lreach).facenum = 0 as libc::c_int;
                    (*lreach).edgenum = water_bestarea2groundedgenum;
                    (*lreach).start[0 as libc::c_int as usize] =
                        water_beststart[0 as libc::c_int as usize];
                    (*lreach).start[1 as libc::c_int as usize] =
                        water_beststart[1 as libc::c_int as usize];
                    (*lreach).start[2 as libc::c_int as usize] =
                        water_beststart[2 as libc::c_int as usize];
                    (*lreach).end[0 as libc::c_int as usize] = water_bestend
                        [0 as libc::c_int as usize]
                        + water_bestnormal[0 as libc::c_int as usize]
                            * 15 as libc::c_int as libc::c_float;
                    (*lreach).end[1 as libc::c_int as usize] = water_bestend
                        [1 as libc::c_int as usize]
                        + water_bestnormal[1 as libc::c_int as usize]
                            * 15 as libc::c_int as libc::c_float;
                    (*lreach).end[2 as libc::c_int as usize] = water_bestend
                        [2 as libc::c_int as usize]
                        + water_bestnormal[2 as libc::c_int as usize]
                            * 15 as libc::c_int as libc::c_float;
                    (*lreach).traveltype = 9 as libc::c_int;
                    (*lreach).traveltime =
                        crate::src::botlib::be_aas_move::aassettings.rs_waterjump as libc::c_ushort;
                    (*lreach).next = *areareachability.offset(area1num as isize);
                    let ref mut fresh8 = *areareachability.offset(area1num as isize);
                    *fresh8 = lreach;
                    //we've got another waterjump reachability
                    reach_waterjump += 1;
                    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
                }
                //end if
            }
            //end if
        }
    }
    //
    // Barrier Jumps
    //
    //        ---------
    //        |
    //        |
    //        |
    //        |         higher than step height lower than barrier height -> TRAVEL_BARRIERJUMP
    //--------|
    //
    //        ---------
    //        |
    //        |
    //        |
    //~~~~~~~~|         higher than step height lower than barrier height
    //--------|         and a thin layer of water in the area to jump from -> TRAVEL_BARRIERJUMP
    //
    //check for a barrier jump reachability
    if ground_foundreach != 0 {
        //end if
        //if area2 is higher but lower than the maximum barrier jump height
        if ground_bestdist > 0 as libc::c_int as libc::c_float
            && ground_bestdist < crate::src::botlib::be_aas_move::aassettings.phys_maxbarrier
        {
            //if no water in area1 or a very thin layer of water on the ground
            if water_foundreach == 0
                || ground_bestdist - water_bestdist < 16 as libc::c_int as libc::c_float
            {
                //cannot perform a barrier jump towards or from a crouch area in Quake2
                if AAS_AreaCrouch(area1num) == 0 && AAS_AreaCrouch(area2num) == 0 {
                    //create barrier jump reachability from area1 to area2
                    lreach = AAS_AllocReachability(); //AAS_BarrierJumpTravelTime();
                    if lreach.is_null() {
                        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                    }
                    (*lreach).areanum = area2num;
                    (*lreach).facenum = 0 as libc::c_int;
                    (*lreach).edgenum = ground_bestarea2groundedgenum;
                    (*lreach).start[0 as libc::c_int as usize] = (ground_beststart
                        [0 as libc::c_int as usize]
                        as libc::c_double
                        + ground_bestnormal[0 as libc::c_int as usize] as libc::c_double * 0.1f64)
                        as crate::src::qcommon::q_shared::vec_t;
                    (*lreach).start[1 as libc::c_int as usize] = (ground_beststart
                        [1 as libc::c_int as usize]
                        as libc::c_double
                        + ground_bestnormal[1 as libc::c_int as usize] as libc::c_double * 0.1f64)
                        as crate::src::qcommon::q_shared::vec_t;
                    (*lreach).start[2 as libc::c_int as usize] = (ground_beststart
                        [2 as libc::c_int as usize]
                        as libc::c_double
                        + ground_bestnormal[2 as libc::c_int as usize] as libc::c_double * 0.1f64)
                        as crate::src::qcommon::q_shared::vec_t;
                    (*lreach).end[0 as libc::c_int as usize] = ground_bestend
                        [0 as libc::c_int as usize]
                        + ground_bestnormal[0 as libc::c_int as usize]
                            * 5 as libc::c_int as libc::c_float;
                    (*lreach).end[1 as libc::c_int as usize] = ground_bestend
                        [1 as libc::c_int as usize]
                        + ground_bestnormal[1 as libc::c_int as usize]
                            * 5 as libc::c_int as libc::c_float;
                    (*lreach).end[2 as libc::c_int as usize] = ground_bestend
                        [2 as libc::c_int as usize]
                        + ground_bestnormal[2 as libc::c_int as usize]
                            * 5 as libc::c_int as libc::c_float;
                    (*lreach).traveltype = 4 as libc::c_int;
                    (*lreach).traveltime = crate::src::botlib::be_aas_move::aassettings
                        .rs_barrierjump
                        as libc::c_ushort;
                    (*lreach).next = *areareachability.offset(area1num as isize);
                    let ref mut fresh9 = *areareachability.offset(area1num as isize);
                    *fresh9 = lreach;
                    //we've got another barrierjump reachability
                    reach_barrier += 1;
                    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
                }
                //end if
            }
            //end if
        }
        //end if
    }
    //
    // Walk and Walk Off Ledge
    //
    //--------|
    //        |          can walk or step back -> TRAVEL_WALK
    //        ---------
    //
    //--------|
    //        |
    //        |
    //        |
    //        |          cannot walk/step back -> TRAVEL_WALKOFFLEDGE
    //        ---------
    //
    //--------|
    //        |
    //        |~~~~~~~~
    //        |
    //        |          cannot step back but can waterjump back -> TRAVEL_WALKOFFLEDGE
    //        ---------  FIXME: create TRAVEL_WALK reach??
    //
    //check for a walk or walk off ledge reachability
    if ground_foundreach != 0 {
        if ground_bestdist < 0 as libc::c_int as libc::c_float {
            //end if
            if ground_bestdist > -crate::src::botlib::be_aas_move::aassettings.phys_maxstep {
                //end if
                //create walk reachability from area1 to area2
                lreach = AAS_AllocReachability();
                if lreach.is_null() {
                    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                }
                (*lreach).areanum = area2num;
                (*lreach).facenum = 0 as libc::c_int;
                (*lreach).edgenum = ground_bestarea2groundedgenum;
                (*lreach).start[0 as libc::c_int as usize] =
                    (ground_beststart[0 as libc::c_int as usize] as libc::c_double
                        + ground_bestnormal[0 as libc::c_int as usize] as libc::c_double * 0.1f64)
                        as crate::src::qcommon::q_shared::vec_t;
                (*lreach).start[1 as libc::c_int as usize] =
                    (ground_beststart[1 as libc::c_int as usize] as libc::c_double
                        + ground_bestnormal[1 as libc::c_int as usize] as libc::c_double * 0.1f64)
                        as crate::src::qcommon::q_shared::vec_t;
                (*lreach).start[2 as libc::c_int as usize] =
                    (ground_beststart[2 as libc::c_int as usize] as libc::c_double
                        + ground_bestnormal[2 as libc::c_int as usize] as libc::c_double * 0.1f64)
                        as crate::src::qcommon::q_shared::vec_t;
                (*lreach).end[0 as libc::c_int as usize] = ground_bestend
                    [0 as libc::c_int as usize]
                    + ground_bestnormal[0 as libc::c_int as usize]
                        * 5 as libc::c_int as libc::c_float;
                (*lreach).end[1 as libc::c_int as usize] = ground_bestend
                    [1 as libc::c_int as usize]
                    + ground_bestnormal[1 as libc::c_int as usize]
                        * 5 as libc::c_int as libc::c_float;
                (*lreach).end[2 as libc::c_int as usize] = ground_bestend
                    [2 as libc::c_int as usize]
                    + ground_bestnormal[2 as libc::c_int as usize]
                        * 5 as libc::c_int as libc::c_float;
                (*lreach).traveltype = 2 as libc::c_int;
                (*lreach).traveltime = 1 as libc::c_int as libc::c_ushort;
                (*lreach).next = *areareachability.offset(area1num as isize);
                let ref mut fresh10 = *areareachability.offset(area1num as isize);
                *fresh10 = lreach;
                //we've got another walk reachability
                reach_walk += 1;
                return crate::src::qcommon::q_shared::qtrue as libc::c_int;
            }
            //end if
            if crate::src::botlib::be_aas_move::aassettings.rs_maxfallheight == 0.
                || crate::stdlib::fabs(ground_bestdist as libc::c_double)
                    < crate::src::botlib::be_aas_move::aassettings.rs_maxfallheight
                        as libc::c_double
            {
                // if no maximum fall height set or less than the max
                //trace a bounding box vertically to check for solids
                ground_bestend[0 as libc::c_int as usize] = ground_bestend
                    [0 as libc::c_int as usize]
                    + ground_bestnormal[0 as libc::c_int as usize]
                        * 2 as libc::c_int as libc::c_float;
                ground_bestend[1 as libc::c_int as usize] = ground_bestend
                    [1 as libc::c_int as usize]
                    + ground_bestnormal[1 as libc::c_int as usize]
                        * 2 as libc::c_int as libc::c_float;
                ground_bestend[2 as libc::c_int as usize] = ground_bestend
                    [2 as libc::c_int as usize]
                    + ground_bestnormal[2 as libc::c_int as usize]
                        * 2 as libc::c_int as libc::c_float;
                start[0 as libc::c_int as usize] = ground_bestend[0 as libc::c_int as usize];
                start[1 as libc::c_int as usize] = ground_bestend[1 as libc::c_int as usize];
                start[2 as libc::c_int as usize] = ground_bestend[2 as libc::c_int as usize];
                start[2 as libc::c_int as usize] = ground_beststart[2 as libc::c_int as usize];
                end[0 as libc::c_int as usize] = ground_bestend[0 as libc::c_int as usize];
                end[1 as libc::c_int as usize] = ground_bestend[1 as libc::c_int as usize];
                end[2 as libc::c_int as usize] = ground_bestend[2 as libc::c_int as usize];
                end[2 as libc::c_int as usize] += 4 as libc::c_int as libc::c_float;
                trace = crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(
                    start.as_mut_ptr(),
                    end.as_mut_ptr(),
                    2 as libc::c_int,
                    -(1 as libc::c_int),
                );
                //end if
                if trace.startsolid as u64 == 0 && trace.fraction as libc::c_double >= 1.0f64 {
                    //if no solids were found
                    //the trace end point must be in the goal area
                    trace.endpos[2 as libc::c_int as usize] += 1 as libc::c_int as libc::c_float;
                    if crate::src::botlib::be_aas_sample::AAS_PointAreaNum(
                        trace.endpos.as_mut_ptr(),
                    ) == area2num
                    {
                        //end if
                        //if not going through a cluster portal
                        numareas = crate::src::botlib::be_aas_sample::AAS_TraceAreas(
                            start.as_mut_ptr(),
                            end.as_mut_ptr(),
                            areas.as_mut_ptr(),
                            0 as *mut crate::src::qcommon::q_shared::vec3_t,
                            (::std::mem::size_of::<[libc::c_int; 10]>() as libc::c_ulong)
                                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                as libc::c_int,
                        );
                        i = 0 as libc::c_int;
                        while i < numareas {
                            if AAS_AreaClusterPortal(areas[i as usize]) != 0 {
                                break;
                            }
                            i += 1
                        }
                        if i >= numareas {
                            //end if
                            //create a walk off ledge reachability from area1 to area2
                            lreach = AAS_AllocReachability();
                            if lreach.is_null() {
                                return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                            }
                            (*lreach).areanum = area2num;
                            (*lreach).facenum = 0 as libc::c_int;
                            (*lreach).edgenum = ground_bestarea2groundedgenum;
                            (*lreach).start[0 as libc::c_int as usize] =
                                ground_beststart[0 as libc::c_int as usize];
                            (*lreach).start[1 as libc::c_int as usize] =
                                ground_beststart[1 as libc::c_int as usize];
                            (*lreach).start[2 as libc::c_int as usize] =
                                ground_beststart[2 as libc::c_int as usize];
                            (*lreach).end[0 as libc::c_int as usize] =
                                ground_bestend[0 as libc::c_int as usize];
                            (*lreach).end[1 as libc::c_int as usize] =
                                ground_bestend[1 as libc::c_int as usize];
                            (*lreach).end[2 as libc::c_int as usize] =
                                ground_bestend[2 as libc::c_int as usize];
                            (*lreach).traveltype = 7 as libc::c_int;
                            (*lreach).traveltime = (crate::src::botlib::be_aas_move::aassettings
                                .rs_startwalkoffledge
                                as libc::c_double
                                + crate::stdlib::fabs(ground_bestdist as libc::c_double)
                                    * 50 as libc::c_int as libc::c_double
                                    / crate::src::botlib::be_aas_move::aassettings.phys_gravity
                                        as libc::c_double)
                                as libc::c_ushort;
                            //if falling from too high and not falling into water
                            if AAS_AreaSwim(area2num) == 0 && AAS_AreaJumpPad(area2num) == 0 {
                                //end if
                                if AAS_FallDelta(ground_bestdist)
                                    > crate::src::botlib::be_aas_move::aassettings.phys_falldelta5
                                {
                                    (*lreach).traveltime = ((*lreach).traveltime as libc::c_float
                                        + crate::src::botlib::be_aas_move::aassettings
                                            .rs_falldamage5)
                                        as libc::c_ushort
                                }
                                if AAS_FallDelta(ground_bestdist)
                                    > crate::src::botlib::be_aas_move::aassettings.phys_falldelta10
                                {
                                    (*lreach).traveltime = ((*lreach).traveltime as libc::c_float
                                        + crate::src::botlib::be_aas_move::aassettings
                                            .rs_falldamage10)
                                        as libc::c_ushort
                                } //end if
                                  //end if
                            }
                            (*lreach).next = *areareachability.offset(area1num as isize);
                            let ref mut fresh11 = *areareachability.offset(area1num as isize);
                            *fresh11 = lreach;
                            //
                            reach_walkoffledge += 1;
                            //NOTE: don't create a weapon (rl, bfg) jump reachability here
                            //because it interferes with other reachabilities
                            //like the ladder reachability
                            return crate::src::qcommon::q_shared::qtrue as libc::c_int;
                        }
                    }
                }
            }
        }
        //end else
    }
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//end of the function AAS_Reachability_Step_Barrier_WaterJump_WalkOffLedge
//===========================================================================
// returns the distance between the two vectors
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn VectorDistance(
    mut v1: *mut crate::src::qcommon::q_shared::vec_t,
    mut v2: *mut crate::src::qcommon::q_shared::vec_t,
) -> libc::c_float {
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    dir[0 as libc::c_int as usize] =
        *v2.offset(0 as libc::c_int as isize) - *v1.offset(0 as libc::c_int as isize);
    dir[1 as libc::c_int as usize] =
        *v2.offset(1 as libc::c_int as isize) - *v1.offset(1 as libc::c_int as isize);
    dir[2 as libc::c_int as usize] =
        *v2.offset(2 as libc::c_int as isize) - *v1.offset(2 as libc::c_int as isize);
    return VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
}
//end of the function VectorDistance
//===========================================================================
// returns true if the first vector is between the last two vectors
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn VectorBetweenVectors(
    mut v: *mut crate::src::qcommon::q_shared::vec_t,
    mut v1: *mut crate::src::qcommon::q_shared::vec_t,
    mut v2: *mut crate::src::qcommon::q_shared::vec_t,
) -> libc::c_int {
    let mut dir1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    dir1[0 as libc::c_int as usize] =
        *v.offset(0 as libc::c_int as isize) - *v1.offset(0 as libc::c_int as isize);
    dir1[1 as libc::c_int as usize] =
        *v.offset(1 as libc::c_int as isize) - *v1.offset(1 as libc::c_int as isize);
    dir1[2 as libc::c_int as usize] =
        *v.offset(2 as libc::c_int as isize) - *v1.offset(2 as libc::c_int as isize);
    dir2[0 as libc::c_int as usize] =
        *v.offset(0 as libc::c_int as isize) - *v2.offset(0 as libc::c_int as isize);
    dir2[1 as libc::c_int as usize] =
        *v.offset(1 as libc::c_int as isize) - *v2.offset(1 as libc::c_int as isize);
    dir2[2 as libc::c_int as usize] =
        *v.offset(2 as libc::c_int as isize) - *v2.offset(2 as libc::c_int as isize);
    return (dir1[0 as libc::c_int as usize] * dir2[0 as libc::c_int as usize]
        + dir1[1 as libc::c_int as usize] * dir2[1 as libc::c_int as usize]
        + dir1[2 as libc::c_int as usize] * dir2[2 as libc::c_int as usize]
        <= 0 as libc::c_int as libc::c_float) as libc::c_int;
}
//end of the function VectorBetweenVectors
//===========================================================================
// returns the mid point between the two vectors
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn VectorMiddle(
    mut v1: *mut crate::src::qcommon::q_shared::vec_t,
    mut v2: *mut crate::src::qcommon::q_shared::vec_t,
    mut middle: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *middle.offset(0 as libc::c_int as isize) =
        *v1.offset(0 as libc::c_int as isize) + *v2.offset(0 as libc::c_int as isize);
    *middle.offset(1 as libc::c_int as isize) =
        *v1.offset(1 as libc::c_int as isize) + *v2.offset(1 as libc::c_int as isize);
    *middle.offset(2 as libc::c_int as isize) =
        *v1.offset(2 as libc::c_int as isize) + *v2.offset(2 as libc::c_int as isize);
    *middle.offset(0 as libc::c_int as isize) =
        (*middle.offset(0 as libc::c_int as isize) as libc::c_double * 0.5f64)
            as crate::src::qcommon::q_shared::vec_t;
    *middle.offset(1 as libc::c_int as isize) =
        (*middle.offset(1 as libc::c_int as isize) as libc::c_double * 0.5f64)
            as crate::src::qcommon::q_shared::vec_t;
    *middle.offset(2 as libc::c_int as isize) =
        (*middle.offset(2 as libc::c_int as isize) as libc::c_double * 0.5f64)
            as crate::src::qcommon::q_shared::vec_t;
}
//end of the function VectorMiddle
//===========================================================================
// calculate a range of points closest to each other on both edges
//
// Parameter:			beststart1		start of the range of points on edge v1-v2
//						beststart2		end of the range of points  on edge v1-v2
//						bestend1		start of the range of points on edge v3-v4
//						bestend2		end of the range of points  on edge v3-v4
//						bestdist		best distance so far
// Returns:				-
// Changes Globals:		-
//===========================================================================
/*
float AAS_ClosestEdgePoints(vec3_t v1, vec3_t v2, vec3_t v3, vec3_t v4,
                            aas_plane_t *plane1, aas_plane_t *plane2,
                            vec3_t beststart, vec3_t bestend, float bestdist)
{
    vec3_t dir1, dir2, p1, p2, p3, p4;
    float a1, a2, b1, b2, dist;
    int founddist;

    //edge vectors
    VectorSubtract(v2, v1, dir1);
    VectorSubtract(v4, v3, dir2);
    //get the horizontal directions
    dir1[2] = 0;
    dir2[2] = 0;
    //
    // p1 = point on an edge vector of area2 closest to v1
    // p2 = point on an edge vector of area2 closest to v2
    // p3 = point on an edge vector of area1 closest to v3
    // p4 = point on an edge vector of area1 closest to v4
    //
    if (dir2[0])
    {
        a2 = dir2[1] / dir2[0];
        b2 = v3[1] - a2 * v3[0];
        //point on the edge vector of area2 closest to v1
        p1[0] = (DotProduct(v1, dir2) - (a2 * dir2[0] + b2 * dir2[1])) / dir2[0];
        p1[1] = a2 * p1[0] + b2;
        //point on the edge vector of area2 closest to v2
        p2[0] = (DotProduct(v2, dir2) - (a2 * dir2[0] + b2 * dir2[1])) / dir2[0];
        p2[1] = a2 * p2[0] + b2;
    } //end if
    else
    {
        //point on the edge vector of area2 closest to v1
        p1[0] = v3[0];
        p1[1] = v1[1];
        //point on the edge vector of area2 closest to v2
        p2[0] = v3[0];
        p2[1] = v2[1];
    } //end else
    //
    if (dir1[0])
    {
        //
        a1 = dir1[1] / dir1[0];
        b1 = v1[1] - a1 * v1[0];
        //point on the edge vector of area1 closest to v3
        p3[0] = (DotProduct(v3, dir1) - (a1 * dir1[0] + b1 * dir1[1])) / dir1[0];
        p3[1] = a1 * p3[0] + b1;
        //point on the edge vector of area1 closest to v4
        p4[0] = (DotProduct(v4, dir1) - (a1 * dir1[0] + b1 * dir1[1])) / dir1[0];
        p4[1] = a1 * p4[0] + b1;
    } //end if
    else
    {
        //point on the edge vector of area1 closest to v3
        p3[0] = v1[0];
        p3[1] = v3[1];
        //point on the edge vector of area1 closest to v4
        p4[0] = v1[0];
        p4[1] = v4[1];
    } //end else
    //start with zero z-coordinates
    p1[2] = 0;
    p2[2] = 0;
    p3[2] = 0;
    p4[2] = 0;
    //calculate the z-coordinates from the ground planes
    p1[2] = (plane2->dist - DotProduct(plane2->normal, p1)) / plane2->normal[2];
    p2[2] = (plane2->dist - DotProduct(plane2->normal, p2)) / plane2->normal[2];
    p3[2] = (plane1->dist - DotProduct(plane1->normal, p3)) / plane1->normal[2];
    p4[2] = (plane1->dist - DotProduct(plane1->normal, p4)) / plane1->normal[2];
    //
    founddist = qfalse;
    //
    if (VectorBetweenVectors(p1, v3, v4))
    {
        dist = VectorDistance(v1, p1);
        if (dist > bestdist - 0.5 && dist < bestdist + 0.5)
        {
            VectorMiddle(beststart, v1, beststart);
            VectorMiddle(bestend, p1, bestend);
        } //end if
        else if (dist < bestdist)
        {
            bestdist = dist;
            VectorCopy(v1, beststart);
            VectorCopy(p1, bestend);
        } //end if
        founddist = qtrue;
    } //end if
    if (VectorBetweenVectors(p2, v3, v4))
    {
        dist = VectorDistance(v2, p2);
        if (dist > bestdist - 0.5 && dist < bestdist + 0.5)
        {
            VectorMiddle(beststart, v2, beststart);
            VectorMiddle(bestend, p2, bestend);
        } //end if
        else if (dist < bestdist)
        {
            bestdist = dist;
            VectorCopy(v2, beststart);
            VectorCopy(p2, bestend);
        } //end if
        founddist = qtrue;
    } //end else if
    if (VectorBetweenVectors(p3, v1, v2))
    {
        dist = VectorDistance(v3, p3);
        if (dist > bestdist - 0.5 && dist < bestdist + 0.5)
        {
            VectorMiddle(beststart, p3, beststart);
            VectorMiddle(bestend, v3, bestend);
        } //end if
        else if (dist < bestdist)
        {
            bestdist = dist;
            VectorCopy(p3, beststart);
            VectorCopy(v3, bestend);
        } //end if
        founddist = qtrue;
    } //end else if
    if (VectorBetweenVectors(p4, v1, v2))
    {
        dist = VectorDistance(v4, p4);
        if (dist > bestdist - 0.5 && dist < bestdist + 0.5)
        {
            VectorMiddle(beststart, p4, beststart);
            VectorMiddle(bestend, v4, bestend);
        } //end if
        else if (dist < bestdist)
        {
            bestdist = dist;
            VectorCopy(p4, beststart);
            VectorCopy(v4, bestend);
        } //end if
        founddist = qtrue;
    } //end else if
    //if no shortest distance was found the shortest distance
    //is between one of the vertexes of edge1 and one of edge2
    if (!founddist)
    {
        dist = VectorDistance(v1, v3);
        if (dist < bestdist)
        {
            bestdist = dist;
            VectorCopy(v1, beststart);
            VectorCopy(v3, bestend);
        } //end if
        dist = VectorDistance(v1, v4);
        if (dist < bestdist)
        {
            bestdist = dist;
            VectorCopy(v1, beststart);
            VectorCopy(v4, bestend);
        } //end if
        dist = VectorDistance(v2, v3);
        if (dist < bestdist)
        {
            bestdist = dist;
            VectorCopy(v2, beststart);
            VectorCopy(v3, bestend);
        } //end if
        dist = VectorDistance(v2, v4);
        if (dist < bestdist)
        {
            bestdist = dist;
            VectorCopy(v2, beststart);
            VectorCopy(v4, bestend);
        } //end if
    } //end if
    return bestdist;
} //end of the function AAS_ClosestEdgePoints*/
#[no_mangle]

pub unsafe extern "C" fn AAS_ClosestEdgePoints(
    mut v1: *mut crate::src::qcommon::q_shared::vec_t,
    mut v2: *mut crate::src::qcommon::q_shared::vec_t,
    mut v3: *mut crate::src::qcommon::q_shared::vec_t,
    mut v4: *mut crate::src::qcommon::q_shared::vec_t,
    mut plane1: *mut crate::aasfile_h::aas_plane_t,
    mut plane2: *mut crate::aasfile_h::aas_plane_t,
    mut beststart1: *mut crate::src::qcommon::q_shared::vec_t,
    mut bestend1: *mut crate::src::qcommon::q_shared::vec_t,
    mut beststart2: *mut crate::src::qcommon::q_shared::vec_t,
    mut bestend2: *mut crate::src::qcommon::q_shared::vec_t,
    mut bestdist: libc::c_float,
) -> libc::c_float {
    let mut dir1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p3: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p4: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut a1: libc::c_float = 0.;
    let mut a2: libc::c_float = 0.;
    let mut b1: libc::c_float = 0.;
    let mut b2: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    let mut dist1: libc::c_float = 0.;
    let mut dist2: libc::c_float = 0.;
    let mut founddist: libc::c_int = 0;
    //edge vectors
    dir1[0 as libc::c_int as usize] =
        *v2.offset(0 as libc::c_int as isize) - *v1.offset(0 as libc::c_int as isize);
    dir1[1 as libc::c_int as usize] =
        *v2.offset(1 as libc::c_int as isize) - *v1.offset(1 as libc::c_int as isize);
    dir1[2 as libc::c_int as usize] =
        *v2.offset(2 as libc::c_int as isize) - *v1.offset(2 as libc::c_int as isize);
    dir2[0 as libc::c_int as usize] =
        *v4.offset(0 as libc::c_int as isize) - *v3.offset(0 as libc::c_int as isize);
    dir2[1 as libc::c_int as usize] =
        *v4.offset(1 as libc::c_int as isize) - *v3.offset(1 as libc::c_int as isize);
    dir2[2 as libc::c_int as usize] =
        *v4.offset(2 as libc::c_int as isize) - *v3.offset(2 as libc::c_int as isize);
    //get the horizontal directions
    dir1[2 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    dir2[2 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    //
    // p1 = point on an edge vector of area2 closest to v1
    // p2 = point on an edge vector of area2 closest to v2
    // p3 = point on an edge vector of area1 closest to v3
    // p4 = point on an edge vector of area1 closest to v4
    //
    if dir2[0 as libc::c_int as usize] != 0. {
        //end else
        a2 = dir2[1 as libc::c_int as usize] / dir2[0 as libc::c_int as usize]; //end if
        b2 = *v3.offset(1 as libc::c_int as isize) - a2 * *v3.offset(0 as libc::c_int as isize);
        //point on the edge vector of area2 closest to v1
        p1[0 as libc::c_int as usize] = (*v1.offset(0 as libc::c_int as isize)
            * dir2[0 as libc::c_int as usize]
            + *v1.offset(1 as libc::c_int as isize) * dir2[1 as libc::c_int as usize]
            + *v1.offset(2 as libc::c_int as isize) * dir2[2 as libc::c_int as usize]
            - (a2 * dir2[0 as libc::c_int as usize] + b2 * dir2[1 as libc::c_int as usize]))
            / dir2[0 as libc::c_int as usize];
        p1[1 as libc::c_int as usize] = a2 * p1[0 as libc::c_int as usize] + b2;
        //point on the edge vector of area2 closest to v2
        p2[0 as libc::c_int as usize] = (*v2.offset(0 as libc::c_int as isize)
            * dir2[0 as libc::c_int as usize]
            + *v2.offset(1 as libc::c_int as isize) * dir2[1 as libc::c_int as usize]
            + *v2.offset(2 as libc::c_int as isize) * dir2[2 as libc::c_int as usize]
            - (a2 * dir2[0 as libc::c_int as usize] + b2 * dir2[1 as libc::c_int as usize]))
            / dir2[0 as libc::c_int as usize];
        p2[1 as libc::c_int as usize] = a2 * p2[0 as libc::c_int as usize] + b2
    } else {
        //point on the edge vector of area2 closest to v1
        p1[0 as libc::c_int as usize] = *v3.offset(0 as libc::c_int as isize);
        p1[1 as libc::c_int as usize] = *v1.offset(1 as libc::c_int as isize);
        //point on the edge vector of area2 closest to v2
        p2[0 as libc::c_int as usize] = *v3.offset(0 as libc::c_int as isize);
        p2[1 as libc::c_int as usize] = *v2.offset(1 as libc::c_int as isize)
    }
    //
    if dir1[0 as libc::c_int as usize] != 0. {
        //end else
        a1 = dir1[1 as libc::c_int as usize] / dir1[0 as libc::c_int as usize]; //end if
        b1 = *v1.offset(1 as libc::c_int as isize) - a1 * *v1.offset(0 as libc::c_int as isize);
        //
        //point on the edge vector of area1 closest to v3
        p3[0 as libc::c_int as usize] = (*v3.offset(0 as libc::c_int as isize)
            * dir1[0 as libc::c_int as usize]
            + *v3.offset(1 as libc::c_int as isize) * dir1[1 as libc::c_int as usize]
            + *v3.offset(2 as libc::c_int as isize) * dir1[2 as libc::c_int as usize]
            - (a1 * dir1[0 as libc::c_int as usize] + b1 * dir1[1 as libc::c_int as usize]))
            / dir1[0 as libc::c_int as usize];
        p3[1 as libc::c_int as usize] = a1 * p3[0 as libc::c_int as usize] + b1;
        //point on the edge vector of area1 closest to v4
        p4[0 as libc::c_int as usize] = (*v4.offset(0 as libc::c_int as isize)
            * dir1[0 as libc::c_int as usize]
            + *v4.offset(1 as libc::c_int as isize) * dir1[1 as libc::c_int as usize]
            + *v4.offset(2 as libc::c_int as isize) * dir1[2 as libc::c_int as usize]
            - (a1 * dir1[0 as libc::c_int as usize] + b1 * dir1[1 as libc::c_int as usize]))
            / dir1[0 as libc::c_int as usize];
        p4[1 as libc::c_int as usize] = a1 * p4[0 as libc::c_int as usize] + b1
    } else {
        //point on the edge vector of area1 closest to v3
        p3[0 as libc::c_int as usize] = *v1.offset(0 as libc::c_int as isize);
        p3[1 as libc::c_int as usize] = *v3.offset(1 as libc::c_int as isize);
        //point on the edge vector of area1 closest to v4
        p4[0 as libc::c_int as usize] = *v1.offset(0 as libc::c_int as isize);
        p4[1 as libc::c_int as usize] = *v4.offset(1 as libc::c_int as isize)
    }
    //start with zero z-coordinates
    p1[2 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    p2[2 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    p3[2 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    p4[2 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    //calculate the z-coordinates from the ground planes
    p1[2 as libc::c_int as usize] = ((*plane2).dist
        - ((*plane2).normal[0 as libc::c_int as usize] * p1[0 as libc::c_int as usize]
            + (*plane2).normal[1 as libc::c_int as usize] * p1[1 as libc::c_int as usize]
            + (*plane2).normal[2 as libc::c_int as usize] * p1[2 as libc::c_int as usize]))
        / (*plane2).normal[2 as libc::c_int as usize];
    p2[2 as libc::c_int as usize] = ((*plane2).dist
        - ((*plane2).normal[0 as libc::c_int as usize] * p2[0 as libc::c_int as usize]
            + (*plane2).normal[1 as libc::c_int as usize] * p2[1 as libc::c_int as usize]
            + (*plane2).normal[2 as libc::c_int as usize] * p2[2 as libc::c_int as usize]))
        / (*plane2).normal[2 as libc::c_int as usize];
    p3[2 as libc::c_int as usize] = ((*plane1).dist
        - ((*plane1).normal[0 as libc::c_int as usize] * p3[0 as libc::c_int as usize]
            + (*plane1).normal[1 as libc::c_int as usize] * p3[1 as libc::c_int as usize]
            + (*plane1).normal[2 as libc::c_int as usize] * p3[2 as libc::c_int as usize]))
        / (*plane1).normal[2 as libc::c_int as usize];
    p4[2 as libc::c_int as usize] = ((*plane1).dist
        - ((*plane1).normal[0 as libc::c_int as usize] * p4[0 as libc::c_int as usize]
            + (*plane1).normal[1 as libc::c_int as usize] * p4[1 as libc::c_int as usize]
            + (*plane1).normal[2 as libc::c_int as usize] * p4[2 as libc::c_int as usize]))
        / (*plane1).normal[2 as libc::c_int as usize];
    //
    founddist = crate::src::qcommon::q_shared::qfalse as libc::c_int;
    //
    if VectorBetweenVectors(p1.as_mut_ptr(), v3, v4) != 0 {
        dist = VectorDistance(v1, p1.as_mut_ptr()); //end if
        if dist as libc::c_double > bestdist as libc::c_double - 0.5f64
            && (dist as libc::c_double) < bestdist as libc::c_double + 0.5f64
        {
            dist1 = VectorDistance(beststart1, v1);
            dist2 = VectorDistance(beststart2, v1); //end if
                                                    //end else
            if dist1 > dist2 {
                //end else
                if dist1 > VectorDistance(beststart1, beststart2) {
                    *beststart2.offset(0 as libc::c_int as isize) =
                        *v1.offset(0 as libc::c_int as isize); //end if
                    *beststart2.offset(1 as libc::c_int as isize) =
                        *v1.offset(1 as libc::c_int as isize); //end if
                    *beststart2.offset(2 as libc::c_int as isize) =
                        *v1.offset(2 as libc::c_int as isize)
                }
            } else if dist2 > VectorDistance(beststart1, beststart2) {
                *beststart1.offset(0 as libc::c_int as isize) =
                    *v1.offset(0 as libc::c_int as isize); //end else if
                *beststart1.offset(1 as libc::c_int as isize) =
                    *v1.offset(1 as libc::c_int as isize); //end if
                *beststart1.offset(2 as libc::c_int as isize) =
                    *v1.offset(2 as libc::c_int as isize)
            }
            dist1 = VectorDistance(bestend1, p1.as_mut_ptr());
            dist2 = VectorDistance(bestend2, p1.as_mut_ptr());
            if dist1 > dist2 {
                if dist1 > VectorDistance(bestend1, bestend2) {
                    *bestend2.offset(0 as libc::c_int as isize) = p1[0 as libc::c_int as usize];
                    *bestend2.offset(1 as libc::c_int as isize) = p1[1 as libc::c_int as usize];
                    *bestend2.offset(2 as libc::c_int as isize) = p1[2 as libc::c_int as usize]
                }
            } else if dist2 > VectorDistance(bestend1, bestend2) {
                *bestend1.offset(0 as libc::c_int as isize) = p1[0 as libc::c_int as usize];
                *bestend1.offset(1 as libc::c_int as isize) = p1[1 as libc::c_int as usize];
                *bestend1.offset(2 as libc::c_int as isize) = p1[2 as libc::c_int as usize]
            }
        } else if dist < bestdist {
            bestdist = dist;
            *beststart1.offset(0 as libc::c_int as isize) = *v1.offset(0 as libc::c_int as isize);
            *beststart1.offset(1 as libc::c_int as isize) = *v1.offset(1 as libc::c_int as isize);
            *beststart1.offset(2 as libc::c_int as isize) = *v1.offset(2 as libc::c_int as isize);
            *beststart2.offset(0 as libc::c_int as isize) = *v1.offset(0 as libc::c_int as isize);
            *beststart2.offset(1 as libc::c_int as isize) = *v1.offset(1 as libc::c_int as isize);
            *beststart2.offset(2 as libc::c_int as isize) = *v1.offset(2 as libc::c_int as isize);
            *bestend1.offset(0 as libc::c_int as isize) = p1[0 as libc::c_int as usize];
            *bestend1.offset(1 as libc::c_int as isize) = p1[1 as libc::c_int as usize];
            *bestend1.offset(2 as libc::c_int as isize) = p1[2 as libc::c_int as usize];
            *bestend2.offset(0 as libc::c_int as isize) = p1[0 as libc::c_int as usize];
            *bestend2.offset(1 as libc::c_int as isize) = p1[1 as libc::c_int as usize];
            *bestend2.offset(2 as libc::c_int as isize) = p1[2 as libc::c_int as usize]
        }
        founddist = crate::src::qcommon::q_shared::qtrue as libc::c_int
    }
    if VectorBetweenVectors(p2.as_mut_ptr(), v3, v4) != 0 {
        dist = VectorDistance(v2, p2.as_mut_ptr());
        if dist as libc::c_double > bestdist as libc::c_double - 0.5f64
            && (dist as libc::c_double) < bestdist as libc::c_double + 0.5f64
        {
            dist1 = VectorDistance(beststart1, v2);
            dist2 = VectorDistance(beststart2, v2);
            //end else
            if dist1 > dist2 {
                //end else
                if dist1 > VectorDistance(beststart1, beststart2) {
                    *beststart2.offset(0 as libc::c_int as isize) =
                        *v2.offset(0 as libc::c_int as isize); //end if
                    *beststart2.offset(1 as libc::c_int as isize) =
                        *v2.offset(1 as libc::c_int as isize); //end if
                    *beststart2.offset(2 as libc::c_int as isize) =
                        *v2.offset(2 as libc::c_int as isize)
                }
            } else if dist2 > VectorDistance(beststart1, beststart2) {
                *beststart1.offset(0 as libc::c_int as isize) =
                    *v2.offset(0 as libc::c_int as isize); //end else if
                *beststart1.offset(1 as libc::c_int as isize) =
                    *v2.offset(1 as libc::c_int as isize); //end if
                *beststart1.offset(2 as libc::c_int as isize) =
                    *v2.offset(2 as libc::c_int as isize)
            }
            dist1 = VectorDistance(bestend1, p2.as_mut_ptr());
            dist2 = VectorDistance(bestend2, p2.as_mut_ptr());
            if dist1 > dist2 {
                if dist1 > VectorDistance(bestend1, bestend2) {
                    *bestend2.offset(0 as libc::c_int as isize) = p2[0 as libc::c_int as usize];
                    *bestend2.offset(1 as libc::c_int as isize) = p2[1 as libc::c_int as usize];
                    *bestend2.offset(2 as libc::c_int as isize) = p2[2 as libc::c_int as usize]
                }
            } else if dist2 > VectorDistance(bestend1, bestend2) {
                *bestend1.offset(0 as libc::c_int as isize) = p2[0 as libc::c_int as usize];
                *bestend1.offset(1 as libc::c_int as isize) = p2[1 as libc::c_int as usize];
                *bestend1.offset(2 as libc::c_int as isize) = p2[2 as libc::c_int as usize]
            }
        } else if dist < bestdist {
            bestdist = dist;
            *beststart1.offset(0 as libc::c_int as isize) = *v2.offset(0 as libc::c_int as isize);
            *beststart1.offset(1 as libc::c_int as isize) = *v2.offset(1 as libc::c_int as isize);
            *beststart1.offset(2 as libc::c_int as isize) = *v2.offset(2 as libc::c_int as isize);
            *beststart2.offset(0 as libc::c_int as isize) = *v2.offset(0 as libc::c_int as isize);
            *beststart2.offset(1 as libc::c_int as isize) = *v2.offset(1 as libc::c_int as isize);
            *beststart2.offset(2 as libc::c_int as isize) = *v2.offset(2 as libc::c_int as isize);
            *bestend1.offset(0 as libc::c_int as isize) = p2[0 as libc::c_int as usize];
            *bestend1.offset(1 as libc::c_int as isize) = p2[1 as libc::c_int as usize];
            *bestend1.offset(2 as libc::c_int as isize) = p2[2 as libc::c_int as usize];
            *bestend2.offset(0 as libc::c_int as isize) = p2[0 as libc::c_int as usize];
            *bestend2.offset(1 as libc::c_int as isize) = p2[1 as libc::c_int as usize];
            *bestend2.offset(2 as libc::c_int as isize) = p2[2 as libc::c_int as usize]
        }
        founddist = crate::src::qcommon::q_shared::qtrue as libc::c_int
    }
    if VectorBetweenVectors(p3.as_mut_ptr(), v1, v2) != 0 {
        dist = VectorDistance(v3, p3.as_mut_ptr());
        if dist as libc::c_double > bestdist as libc::c_double - 0.5f64
            && (dist as libc::c_double) < bestdist as libc::c_double + 0.5f64
        {
            dist1 = VectorDistance(beststart1, p3.as_mut_ptr());
            dist2 = VectorDistance(beststart2, p3.as_mut_ptr());
            //end else
            if dist1 > dist2 {
                //end else
                if dist1 > VectorDistance(beststart1, beststart2) {
                    *beststart2.offset(0 as libc::c_int as isize) = p3[0 as libc::c_int as usize]; //end if
                    *beststart2.offset(1 as libc::c_int as isize) = p3[1 as libc::c_int as usize]; //end if
                    *beststart2.offset(2 as libc::c_int as isize) = p3[2 as libc::c_int as usize]
                }
            } else if dist2 > VectorDistance(beststart1, beststart2) {
                *beststart1.offset(0 as libc::c_int as isize) = p3[0 as libc::c_int as usize]; //end else if
                *beststart1.offset(1 as libc::c_int as isize) = p3[1 as libc::c_int as usize]; //end if
                *beststart1.offset(2 as libc::c_int as isize) = p3[2 as libc::c_int as usize]
            }
            dist1 = VectorDistance(bestend1, v3);
            dist2 = VectorDistance(bestend2, v3);
            if dist1 > dist2 {
                if dist1 > VectorDistance(bestend1, bestend2) {
                    *bestend2.offset(0 as libc::c_int as isize) =
                        *v3.offset(0 as libc::c_int as isize);
                    *bestend2.offset(1 as libc::c_int as isize) =
                        *v3.offset(1 as libc::c_int as isize);
                    *bestend2.offset(2 as libc::c_int as isize) =
                        *v3.offset(2 as libc::c_int as isize)
                }
            } else if dist2 > VectorDistance(bestend1, bestend2) {
                *bestend1.offset(0 as libc::c_int as isize) = *v3.offset(0 as libc::c_int as isize);
                *bestend1.offset(1 as libc::c_int as isize) = *v3.offset(1 as libc::c_int as isize);
                *bestend1.offset(2 as libc::c_int as isize) = *v3.offset(2 as libc::c_int as isize)
            }
        } else if dist < bestdist {
            bestdist = dist;
            *beststart1.offset(0 as libc::c_int as isize) = p3[0 as libc::c_int as usize];
            *beststart1.offset(1 as libc::c_int as isize) = p3[1 as libc::c_int as usize];
            *beststart1.offset(2 as libc::c_int as isize) = p3[2 as libc::c_int as usize];
            *beststart2.offset(0 as libc::c_int as isize) = p3[0 as libc::c_int as usize];
            *beststart2.offset(1 as libc::c_int as isize) = p3[1 as libc::c_int as usize];
            *beststart2.offset(2 as libc::c_int as isize) = p3[2 as libc::c_int as usize];
            *bestend1.offset(0 as libc::c_int as isize) = *v3.offset(0 as libc::c_int as isize);
            *bestend1.offset(1 as libc::c_int as isize) = *v3.offset(1 as libc::c_int as isize);
            *bestend1.offset(2 as libc::c_int as isize) = *v3.offset(2 as libc::c_int as isize);
            *bestend2.offset(0 as libc::c_int as isize) = *v3.offset(0 as libc::c_int as isize);
            *bestend2.offset(1 as libc::c_int as isize) = *v3.offset(1 as libc::c_int as isize);
            *bestend2.offset(2 as libc::c_int as isize) = *v3.offset(2 as libc::c_int as isize)
        }
        founddist = crate::src::qcommon::q_shared::qtrue as libc::c_int
    }
    if VectorBetweenVectors(p4.as_mut_ptr(), v1, v2) != 0 {
        dist = VectorDistance(v4, p4.as_mut_ptr());
        if dist as libc::c_double > bestdist as libc::c_double - 0.5f64
            && (dist as libc::c_double) < bestdist as libc::c_double + 0.5f64
        {
            dist1 = VectorDistance(beststart1, p4.as_mut_ptr());
            dist2 = VectorDistance(beststart2, p4.as_mut_ptr());
            //end else
            if dist1 > dist2 {
                //end else
                if dist1 > VectorDistance(beststart1, beststart2) {
                    *beststart2.offset(0 as libc::c_int as isize) = p4[0 as libc::c_int as usize]; //end if
                    *beststart2.offset(1 as libc::c_int as isize) = p4[1 as libc::c_int as usize]; //end if
                    *beststart2.offset(2 as libc::c_int as isize) = p4[2 as libc::c_int as usize]
                }
            } else if dist2 > VectorDistance(beststart1, beststart2) {
                *beststart1.offset(0 as libc::c_int as isize) = p4[0 as libc::c_int as usize];
                *beststart1.offset(1 as libc::c_int as isize) = p4[1 as libc::c_int as usize];
                *beststart1.offset(2 as libc::c_int as isize) = p4[2 as libc::c_int as usize]
            }
            dist1 = VectorDistance(bestend1, v4);
            dist2 = VectorDistance(bestend2, v4);
            if dist1 > dist2 {
                if dist1 > VectorDistance(bestend1, bestend2) {
                    *bestend2.offset(0 as libc::c_int as isize) =
                        *v4.offset(0 as libc::c_int as isize);
                    *bestend2.offset(1 as libc::c_int as isize) =
                        *v4.offset(1 as libc::c_int as isize);
                    *bestend2.offset(2 as libc::c_int as isize) =
                        *v4.offset(2 as libc::c_int as isize)
                }
            } else if dist2 > VectorDistance(bestend1, bestend2) {
                *bestend1.offset(0 as libc::c_int as isize) = *v4.offset(0 as libc::c_int as isize);
                *bestend1.offset(1 as libc::c_int as isize) = *v4.offset(1 as libc::c_int as isize);
                *bestend1.offset(2 as libc::c_int as isize) = *v4.offset(2 as libc::c_int as isize)
            }
        } else if dist < bestdist {
            bestdist = dist;
            *beststart1.offset(0 as libc::c_int as isize) = p4[0 as libc::c_int as usize];
            *beststart1.offset(1 as libc::c_int as isize) = p4[1 as libc::c_int as usize];
            *beststart1.offset(2 as libc::c_int as isize) = p4[2 as libc::c_int as usize];
            *beststart2.offset(0 as libc::c_int as isize) = p4[0 as libc::c_int as usize];
            *beststart2.offset(1 as libc::c_int as isize) = p4[1 as libc::c_int as usize];
            *beststart2.offset(2 as libc::c_int as isize) = p4[2 as libc::c_int as usize];
            *bestend1.offset(0 as libc::c_int as isize) = *v4.offset(0 as libc::c_int as isize);
            *bestend1.offset(1 as libc::c_int as isize) = *v4.offset(1 as libc::c_int as isize);
            *bestend1.offset(2 as libc::c_int as isize) = *v4.offset(2 as libc::c_int as isize);
            *bestend2.offset(0 as libc::c_int as isize) = *v4.offset(0 as libc::c_int as isize);
            *bestend2.offset(1 as libc::c_int as isize) = *v4.offset(1 as libc::c_int as isize);
            *bestend2.offset(2 as libc::c_int as isize) = *v4.offset(2 as libc::c_int as isize)
        }
        founddist = crate::src::qcommon::q_shared::qtrue as libc::c_int
    }
    //if no shortest distance was found the shortest distance
    //is between one of the vertexes of edge1 and one of edge2
    if founddist == 0 {
        dist = VectorDistance(v1, v3); //end if
                                       //end if
        if dist < bestdist {
            bestdist = dist; //end if
            *beststart1.offset(0 as libc::c_int as isize) = *v1.offset(0 as libc::c_int as isize); //end if
            *beststart1.offset(1 as libc::c_int as isize) = *v1.offset(1 as libc::c_int as isize); //end if
            *beststart1.offset(2 as libc::c_int as isize) = *v1.offset(2 as libc::c_int as isize);
            *beststart2.offset(0 as libc::c_int as isize) = *v1.offset(0 as libc::c_int as isize);
            *beststart2.offset(1 as libc::c_int as isize) = *v1.offset(1 as libc::c_int as isize);
            *beststart2.offset(2 as libc::c_int as isize) = *v1.offset(2 as libc::c_int as isize);
            *bestend1.offset(0 as libc::c_int as isize) = *v3.offset(0 as libc::c_int as isize);
            *bestend1.offset(1 as libc::c_int as isize) = *v3.offset(1 as libc::c_int as isize);
            *bestend1.offset(2 as libc::c_int as isize) = *v3.offset(2 as libc::c_int as isize);
            *bestend2.offset(0 as libc::c_int as isize) = *v3.offset(0 as libc::c_int as isize);
            *bestend2.offset(1 as libc::c_int as isize) = *v3.offset(1 as libc::c_int as isize);
            *bestend2.offset(2 as libc::c_int as isize) = *v3.offset(2 as libc::c_int as isize)
        }
        dist = VectorDistance(v1, v4);
        if dist < bestdist {
            bestdist = dist;
            *beststart1.offset(0 as libc::c_int as isize) = *v1.offset(0 as libc::c_int as isize);
            *beststart1.offset(1 as libc::c_int as isize) = *v1.offset(1 as libc::c_int as isize);
            *beststart1.offset(2 as libc::c_int as isize) = *v1.offset(2 as libc::c_int as isize);
            *beststart2.offset(0 as libc::c_int as isize) = *v1.offset(0 as libc::c_int as isize);
            *beststart2.offset(1 as libc::c_int as isize) = *v1.offset(1 as libc::c_int as isize);
            *beststart2.offset(2 as libc::c_int as isize) = *v1.offset(2 as libc::c_int as isize);
            *bestend1.offset(0 as libc::c_int as isize) = *v4.offset(0 as libc::c_int as isize);
            *bestend1.offset(1 as libc::c_int as isize) = *v4.offset(1 as libc::c_int as isize);
            *bestend1.offset(2 as libc::c_int as isize) = *v4.offset(2 as libc::c_int as isize);
            *bestend2.offset(0 as libc::c_int as isize) = *v4.offset(0 as libc::c_int as isize);
            *bestend2.offset(1 as libc::c_int as isize) = *v4.offset(1 as libc::c_int as isize);
            *bestend2.offset(2 as libc::c_int as isize) = *v4.offset(2 as libc::c_int as isize)
        }
        dist = VectorDistance(v2, v3);
        if dist < bestdist {
            bestdist = dist;
            *beststart1.offset(0 as libc::c_int as isize) = *v2.offset(0 as libc::c_int as isize);
            *beststart1.offset(1 as libc::c_int as isize) = *v2.offset(1 as libc::c_int as isize);
            *beststart1.offset(2 as libc::c_int as isize) = *v2.offset(2 as libc::c_int as isize);
            *beststart2.offset(0 as libc::c_int as isize) = *v2.offset(0 as libc::c_int as isize);
            *beststart2.offset(1 as libc::c_int as isize) = *v2.offset(1 as libc::c_int as isize);
            *beststart2.offset(2 as libc::c_int as isize) = *v2.offset(2 as libc::c_int as isize);
            *bestend1.offset(0 as libc::c_int as isize) = *v3.offset(0 as libc::c_int as isize);
            *bestend1.offset(1 as libc::c_int as isize) = *v3.offset(1 as libc::c_int as isize);
            *bestend1.offset(2 as libc::c_int as isize) = *v3.offset(2 as libc::c_int as isize);
            *bestend2.offset(0 as libc::c_int as isize) = *v3.offset(0 as libc::c_int as isize);
            *bestend2.offset(1 as libc::c_int as isize) = *v3.offset(1 as libc::c_int as isize);
            *bestend2.offset(2 as libc::c_int as isize) = *v3.offset(2 as libc::c_int as isize)
        }
        dist = VectorDistance(v2, v4);
        if dist < bestdist {
            bestdist = dist;
            *beststart1.offset(0 as libc::c_int as isize) = *v2.offset(0 as libc::c_int as isize);
            *beststart1.offset(1 as libc::c_int as isize) = *v2.offset(1 as libc::c_int as isize);
            *beststart1.offset(2 as libc::c_int as isize) = *v2.offset(2 as libc::c_int as isize);
            *beststart2.offset(0 as libc::c_int as isize) = *v2.offset(0 as libc::c_int as isize);
            *beststart2.offset(1 as libc::c_int as isize) = *v2.offset(1 as libc::c_int as isize);
            *beststart2.offset(2 as libc::c_int as isize) = *v2.offset(2 as libc::c_int as isize);
            *bestend1.offset(0 as libc::c_int as isize) = *v4.offset(0 as libc::c_int as isize);
            *bestend1.offset(1 as libc::c_int as isize) = *v4.offset(1 as libc::c_int as isize);
            *bestend1.offset(2 as libc::c_int as isize) = *v4.offset(2 as libc::c_int as isize);
            *bestend2.offset(0 as libc::c_int as isize) = *v4.offset(0 as libc::c_int as isize);
            *bestend2.offset(1 as libc::c_int as isize) = *v4.offset(1 as libc::c_int as isize);
            *bestend2.offset(2 as libc::c_int as isize) = *v4.offset(2 as libc::c_int as isize)
        }
    }
    return bestdist;
}
//end of the function AAS_ClosestEdgePoints
//===========================================================================
// creates possible jump reachabilities between the areas
//
// The two closest points on the ground of the areas are calculated
// One of the points will be on an edge of a ground face of area1 and
// one on an edge of a ground face of area2.
// If there is a range of closest points the point in the middle of this range
// is selected.
// Between these two points there must be one or more gaps.
// If the gaps exist a potential jump is predicted.
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Reachability_Jump(
    mut area1num: libc::c_int,
    mut area2num: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut face1num: libc::c_int = 0;
    let mut face2num: libc::c_int = 0;
    let mut edge1num: libc::c_int = 0;
    let mut edge2num: libc::c_int = 0;
    let mut traveltype: libc::c_int = 0;
    let mut stopevent: libc::c_int = 0;
    let mut areas: [libc::c_int; 10] = [0; 10];
    let mut numareas: libc::c_int = 0;
    let mut phys_jumpvel: libc::c_float = 0.;
    let mut maxjumpdistance: libc::c_float = 0.;
    let mut maxjumpheight: libc::c_float = 0.;
    let mut height: libc::c_float = 0.;
    let mut bestdist: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut v1: *mut crate::src::qcommon::q_shared::vec_t =
        0 as *mut crate::src::qcommon::q_shared::vec_t;
    let mut v2: *mut crate::src::qcommon::q_shared::vec_t =
        0 as *mut crate::src::qcommon::q_shared::vec_t;
    let mut v3: *mut crate::src::qcommon::q_shared::vec_t =
        0 as *mut crate::src::qcommon::q_shared::vec_t;
    let mut v4: *mut crate::src::qcommon::q_shared::vec_t =
        0 as *mut crate::src::qcommon::q_shared::vec_t;
    let mut beststart: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0.,
        0.,
    ];
    let mut beststart2: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0.,
        0.,
    ];
    let mut bestend: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0.,
        0.,
    ];
    let mut bestend2: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0.,
        0.,
    ];
    let mut teststart: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut testend: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut cmdmove: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut sidewards: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut area1: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut area2: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut face1: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut face2: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut edge1: *mut crate::aasfile_h::aas_edge_t = 0 as *mut crate::aasfile_h::aas_edge_t;
    let mut edge2: *mut crate::aasfile_h::aas_edge_t = 0 as *mut crate::aasfile_h::aas_edge_t;
    let mut plane1: *mut crate::aasfile_h::aas_plane_t = 0 as *mut crate::aasfile_h::aas_plane_t;
    let mut plane2: *mut crate::aasfile_h::aas_plane_t = 0 as *mut crate::aasfile_h::aas_plane_t;
    let mut plane: *mut crate::aasfile_h::aas_plane_t = 0 as *mut crate::aasfile_h::aas_plane_t;
    let mut trace: crate::be_aas_h::aas_trace_t = crate::be_aas_h::aas_trace_t {
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        ent: 0,
        lastarea: 0,
        area: 0,
        planenum: 0,
    };
    let mut move_0: crate::be_aas_h::aas_clientmove_t = crate::be_aas_h::aas_clientmove_t {
        endpos: [0.; 3],
        endarea: 0,
        velocity: [0.; 3],
        trace: crate::be_aas_h::aas_trace_t {
            startsolid: crate::src::qcommon::q_shared::qfalse,
            fraction: 0.,
            endpos: [0.; 3],
            ent: 0,
            lastarea: 0,
            area: 0,
            planenum: 0,
        },
        presencetype: 0,
        stopevent: 0,
        endcontents: 0,
        time: 0.,
        frames: 0,
    };
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    if AAS_AreaGrounded(area1num) == 0 || AAS_AreaGrounded(area2num) == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //cannot jump from or to a crouch area
    if AAS_AreaCrouch(area1num) != 0 || AAS_AreaCrouch(area2num) != 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //
    area1 = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(area1num as isize) as *mut crate::aasfile_h::aas_area_t;
    area2 = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(area2num as isize) as *mut crate::aasfile_h::aas_area_t;
    //
    phys_jumpvel = crate::src::botlib::be_aas_move::aassettings.phys_jumpvel;
    //maximum distance a player can jump
    maxjumpdistance = 2 as libc::c_int as libc::c_float * AAS_MaxJumpDistance(phys_jumpvel);
    //maximum height a player can jump with the given initial z velocity
    maxjumpheight = AAS_MaxJumpHeight(phys_jumpvel);
    //if the areas are not near enough in the x-y direction
    i = 0 as libc::c_int; //end for
    while i < 2 as libc::c_int {
        if (*area1).mins[i as usize] > (*area2).maxs[i as usize] + maxjumpdistance {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        if (*area1).maxs[i as usize] < (*area2).mins[i as usize] - maxjumpdistance {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        i += 1
    }
    //if area2 is way to high to jump up to
    if (*area2).mins[2 as libc::c_int as usize]
        > (*area1).maxs[2 as libc::c_int as usize] + maxjumpheight
    {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //
    bestdist = 999999 as libc::c_int as libc::c_float;
    //
    i = 0 as libc::c_int; //end for
    while i < (*area1).numfaces {
        face1num = *crate::src::botlib::be_aas_main::aasworld
            .faceindex
            .offset(((*area1).firstface + i) as isize);
        face1 = &mut *crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(
                (crate::stdlib::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(
                    face1num,
                ) as isize,
            ) as *mut crate::aasfile_h::aas_face_t;
        //end for
        //if not a ground face
        if !((*face1).faceflags & 4 as libc::c_int == 0) {
            //
            j = 0 as libc::c_int;
            while j < (*area2).numfaces {
                face2num = *crate::src::botlib::be_aas_main::aasworld
                    .faceindex
                    .offset(((*area2).firstface + j) as isize);
                face2 = &mut *crate::src::botlib::be_aas_main::aasworld.faces.offset(
                    (crate::stdlib::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(
                        face2num,
                    ) as isize,
                ) as *mut crate::aasfile_h::aas_face_t;
                //end for
                //if not a ground face
                if !((*face2).faceflags & 4 as libc::c_int == 0) {
                    //
                    k = 0 as libc::c_int;
                    while k < (*face1).numedges {
                        edge1num = crate::stdlib::abs(
                            *crate::src::botlib::be_aas_main::aasworld
                                .edgeindex
                                .offset(((*face1).firstedge + k) as isize),
                        );
                        edge1 = &mut *crate::src::botlib::be_aas_main::aasworld
                            .edges
                            .offset(edge1num as isize)
                            as *mut crate::aasfile_h::aas_edge_t;
                        l = 0 as libc::c_int;
                        while l < (*face2).numedges {
                            edge2num = crate::stdlib::abs(
                                *crate::src::botlib::be_aas_main::aasworld
                                    .edgeindex
                                    .offset(((*face2).firstedge + l) as isize),
                            );
                            edge2 = &mut *crate::src::botlib::be_aas_main::aasworld
                                .edges
                                .offset(edge2num as isize)
                                as *mut crate::aasfile_h::aas_edge_t;
                            //end for
                            //calculate the minimum distance between the two edges
                            v1 = (*crate::src::botlib::be_aas_main::aasworld
                                .vertexes
                                .offset((*edge1).v[0 as libc::c_int as usize] as isize))
                            .as_mut_ptr();
                            v2 = (*crate::src::botlib::be_aas_main::aasworld
                                .vertexes
                                .offset((*edge1).v[1 as libc::c_int as usize] as isize))
                            .as_mut_ptr();
                            v3 = (*crate::src::botlib::be_aas_main::aasworld
                                .vertexes
                                .offset((*edge2).v[0 as libc::c_int as usize] as isize))
                            .as_mut_ptr();
                            v4 = (*crate::src::botlib::be_aas_main::aasworld
                                .vertexes
                                .offset((*edge2).v[1 as libc::c_int as usize] as isize))
                            .as_mut_ptr();
                            //get the ground planes
                            plane1 = &mut *crate::src::botlib::be_aas_main::aasworld
                                .planes
                                .offset((*face1).planenum as isize)
                                as *mut crate::aasfile_h::aas_plane_t;
                            plane2 = &mut *crate::src::botlib::be_aas_main::aasworld
                                .planes
                                .offset((*face2).planenum as isize)
                                as *mut crate::aasfile_h::aas_plane_t;
                            //
                            bestdist = AAS_ClosestEdgePoints(
                                v1,
                                v2,
                                v3,
                                v4,
                                plane1,
                                plane2,
                                beststart.as_mut_ptr(),
                                bestend.as_mut_ptr(),
                                beststart2.as_mut_ptr(),
                                bestend2.as_mut_ptr(),
                                bestdist,
                            ); //end if
                            l += 1
                        }
                        k += 1
                    }
                }
                j += 1
            }
        }
        i += 1
    }
    VectorMiddle(
        beststart.as_mut_ptr(),
        beststart2.as_mut_ptr(),
        beststart.as_mut_ptr(),
    );
    VectorMiddle(
        bestend.as_mut_ptr(),
        bestend2.as_mut_ptr(),
        bestend.as_mut_ptr(),
    );
    if bestdist > 4 as libc::c_int as libc::c_float && bestdist < maxjumpdistance {
        //		Log_Write("shortest distance between %d and %d is %f\r\n", area1num, area2num, bestdist);
        // if very close and almost no height difference then the bot can walk
        if bestdist <= 48 as libc::c_int as libc::c_float
            && crate::stdlib::fabs(
                (beststart[2 as libc::c_int as usize] - bestend[2 as libc::c_int as usize])
                    as libc::c_double,
            ) < 8 as libc::c_int as libc::c_double
        {
            //end if
            speed = 400 as libc::c_int as libc::c_float; //end if
            traveltype = 7 as libc::c_int
        } else if crate::src::botlib::be_aas_move::AAS_HorizontalVelocityForJump(
            0 as libc::c_int as libc::c_float,
            beststart.as_mut_ptr(),
            bestend.as_mut_ptr(),
            &mut speed,
        ) != 0
        {
            speed *= 1.2f32; //end else if
            traveltype = 7 as libc::c_int
        } else {
            //FIXME: why multiply with 1.2???
            //get the horizontal speed for the jump, if it isn't possible to calculate this
            //speed (the jump is not possible) then there's no jump reachability created
            if crate::src::botlib::be_aas_move::AAS_HorizontalVelocityForJump(
                phys_jumpvel,
                beststart.as_mut_ptr(),
                bestend.as_mut_ptr(),
                &mut speed,
            ) == 0
            {
                return crate::src::qcommon::q_shared::qfalse as libc::c_int;
            }
            speed *= 1.05f32;
            traveltype = 5 as libc::c_int;
            //
            //NOTE: test if the horizontal distance isn't too small
            dir[0 as libc::c_int as usize] =
                bestend[0 as libc::c_int as usize] - beststart[0 as libc::c_int as usize];
            dir[1 as libc::c_int as usize] =
                bestend[1 as libc::c_int as usize] - beststart[1 as libc::c_int as usize];
            dir[2 as libc::c_int as usize] =
                bestend[2 as libc::c_int as usize] - beststart[2 as libc::c_int as usize];
            dir[2 as libc::c_int as usize] =
                0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
            if VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t)
                < 10 as libc::c_int as libc::c_float
            {
                return crate::src::qcommon::q_shared::qfalse as libc::c_int;
            }
        }
        //
        dir[0 as libc::c_int as usize] =
            bestend[0 as libc::c_int as usize] - beststart[0 as libc::c_int as usize];
        dir[1 as libc::c_int as usize] =
            bestend[1 as libc::c_int as usize] - beststart[1 as libc::c_int as usize];
        dir[2 as libc::c_int as usize] =
            bestend[2 as libc::c_int as usize] - beststart[2 as libc::c_int as usize];
        crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
        teststart[0 as libc::c_int as usize] = beststart[0 as libc::c_int as usize]
            + dir[0 as libc::c_int as usize] * 1 as libc::c_int as libc::c_float;
        teststart[1 as libc::c_int as usize] = beststart[1 as libc::c_int as usize]
            + dir[1 as libc::c_int as usize] * 1 as libc::c_int as libc::c_float;
        teststart[2 as libc::c_int as usize] = beststart[2 as libc::c_int as usize]
            + dir[2 as libc::c_int as usize] * 1 as libc::c_int as libc::c_float;
        //
        testend[0 as libc::c_int as usize] = teststart[0 as libc::c_int as usize];
        testend[1 as libc::c_int as usize] = teststart[1 as libc::c_int as usize];
        testend[2 as libc::c_int as usize] = teststart[2 as libc::c_int as usize];
        testend[2 as libc::c_int as usize] -= 100 as libc::c_int as libc::c_float;
        trace = crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(
            teststart.as_mut_ptr(),
            testend.as_mut_ptr(),
            2 as libc::c_int,
            -(1 as libc::c_int),
        );
        //
        if trace.startsolid as u64 != 0 {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        } //end if
        if trace.fraction < 1 as libc::c_int as libc::c_float {
            plane = &mut *crate::src::botlib::be_aas_main::aasworld
                .planes
                .offset(trace.planenum as isize)
                as *mut crate::aasfile_h::aas_plane_t;
            //end if
            if ((*plane).normal[0 as libc::c_int as usize] * up[0 as libc::c_int as usize]
                + (*plane).normal[1 as libc::c_int as usize] * up[1 as libc::c_int as usize]
                + (*plane).normal[2 as libc::c_int as usize] * up[2 as libc::c_int as usize])
                as libc::c_double
                >= 0.7f64
            {
                // if the bot can stand on the surface
                // if no lava or slime below
                if crate::src::botlib::be_aas_bspq3::AAS_PointContents(trace.endpos.as_mut_ptr())
                    & (8 as libc::c_int | 16 as libc::c_int)
                    == 0
                {
                    if teststart[2 as libc::c_int as usize]
                        - trace.endpos[2 as libc::c_int as usize]
                        <= crate::src::botlib::be_aas_move::aassettings.phys_maxbarrier
                    {
                        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                    }
                }
                //end if
            }
        }
        //
        teststart[0 as libc::c_int as usize] = bestend[0 as libc::c_int as usize]
            + dir[0 as libc::c_int as usize] * -(1 as libc::c_int) as libc::c_float;
        teststart[1 as libc::c_int as usize] = bestend[1 as libc::c_int as usize]
            + dir[1 as libc::c_int as usize] * -(1 as libc::c_int) as libc::c_float;
        teststart[2 as libc::c_int as usize] = bestend[2 as libc::c_int as usize]
            + dir[2 as libc::c_int as usize] * -(1 as libc::c_int) as libc::c_float;
        //
        testend[0 as libc::c_int as usize] = teststart[0 as libc::c_int as usize];
        testend[1 as libc::c_int as usize] = teststart[1 as libc::c_int as usize];
        testend[2 as libc::c_int as usize] = teststart[2 as libc::c_int as usize];
        testend[2 as libc::c_int as usize] -= 100 as libc::c_int as libc::c_float;
        trace = crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(
            teststart.as_mut_ptr(),
            testend.as_mut_ptr(),
            2 as libc::c_int,
            -(1 as libc::c_int),
        );
        //
        if trace.startsolid as u64 != 0 {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        } //end if
        if trace.fraction < 1 as libc::c_int as libc::c_float {
            plane = &mut *crate::src::botlib::be_aas_main::aasworld
                .planes
                .offset(trace.planenum as isize)
                as *mut crate::aasfile_h::aas_plane_t;
            //end if
            if ((*plane).normal[0 as libc::c_int as usize] * up[0 as libc::c_int as usize]
                + (*plane).normal[1 as libc::c_int as usize] * up[1 as libc::c_int as usize]
                + (*plane).normal[2 as libc::c_int as usize] * up[2 as libc::c_int as usize])
                as libc::c_double
                >= 0.7f64
            {
                // if the bot can stand on the surface
                // if no lava or slime below
                if crate::src::botlib::be_aas_bspq3::AAS_PointContents(trace.endpos.as_mut_ptr())
                    & (8 as libc::c_int | 16 as libc::c_int)
                    == 0
                {
                    if teststart[2 as libc::c_int as usize]
                        - trace.endpos[2 as libc::c_int as usize]
                        <= crate::src::botlib::be_aas_move::aassettings.phys_maxbarrier
                    {
                        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                    }
                }
                //end if
            }
        }
        //
        // get command movement
        cmdmove[2 as libc::c_int as usize] =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        cmdmove[1 as libc::c_int as usize] = cmdmove[2 as libc::c_int as usize];
        cmdmove[0 as libc::c_int as usize] = cmdmove[1 as libc::c_int as usize];
        if traveltype & 0xffffff as libc::c_int == 5 as libc::c_int {
            cmdmove[2 as libc::c_int as usize] =
                crate::src::botlib::be_aas_move::aassettings.phys_jumpvel
        } else {
            cmdmove[2 as libc::c_int as usize] =
                0 as libc::c_int as crate::src::qcommon::q_shared::vec_t
        }
        //
        dir[0 as libc::c_int as usize] =
            bestend[0 as libc::c_int as usize] - beststart[0 as libc::c_int as usize];
        dir[1 as libc::c_int as usize] =
            bestend[1 as libc::c_int as usize] - beststart[1 as libc::c_int as usize];
        dir[2 as libc::c_int as usize] =
            bestend[2 as libc::c_int as usize] - beststart[2 as libc::c_int as usize];
        dir[2 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
        CrossProduct(
            dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            up.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            sidewards.as_mut_ptr(),
        );
        //
        stopevent = 1 as libc::c_int
            | 4 as libc::c_int
            | 8 as libc::c_int
            | 16 as libc::c_int
            | 32 as libc::c_int;
        if AAS_AreaClusterPortal(area1num) == 0 && AAS_AreaClusterPortal(area2num) == 0 {
            stopevent |= 4096 as libc::c_int
        }
        //
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            //
            if i == 1 as libc::c_int {
                testend[0 as libc::c_int as usize] =
                    testend[0 as libc::c_int as usize] + sidewards[0 as libc::c_int as usize];
                testend[1 as libc::c_int as usize] =
                    testend[1 as libc::c_int as usize] + sidewards[1 as libc::c_int as usize];
                testend[2 as libc::c_int as usize] =
                    testend[2 as libc::c_int as usize] + sidewards[2 as libc::c_int as usize]
            } else if i == 2 as libc::c_int {
                testend[0 as libc::c_int as usize] =
                    bestend[0 as libc::c_int as usize] - sidewards[0 as libc::c_int as usize];
                testend[1 as libc::c_int as usize] =
                    bestend[1 as libc::c_int as usize] - sidewards[1 as libc::c_int as usize];
                testend[2 as libc::c_int as usize] =
                    bestend[2 as libc::c_int as usize] - sidewards[2 as libc::c_int as usize]
            } else {
                testend[0 as libc::c_int as usize] = bestend[0 as libc::c_int as usize];
                testend[1 as libc::c_int as usize] = bestend[1 as libc::c_int as usize];
                testend[2 as libc::c_int as usize] = bestend[2 as libc::c_int as usize]
            }
            dir[0 as libc::c_int as usize] =
                testend[0 as libc::c_int as usize] - beststart[0 as libc::c_int as usize];
            dir[1 as libc::c_int as usize] =
                testend[1 as libc::c_int as usize] - beststart[1 as libc::c_int as usize];
            dir[2 as libc::c_int as usize] =
                testend[2 as libc::c_int as usize] - beststart[2 as libc::c_int as usize];
            dir[2 as libc::c_int as usize] =
                0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
            crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
            velocity[0 as libc::c_int as usize] = dir[0 as libc::c_int as usize] * speed;
            velocity[1 as libc::c_int as usize] = dir[1 as libc::c_int as usize] * speed;
            velocity[2 as libc::c_int as usize] = dir[2 as libc::c_int as usize] * speed;
            //
            crate::src::botlib::be_aas_move::AAS_PredictClientMovement(
                &mut move_0,
                -(1 as libc::c_int),
                beststart.as_mut_ptr(),
                2 as libc::c_int,
                crate::src::qcommon::q_shared::qtrue as libc::c_int,
                velocity.as_mut_ptr(),
                cmdmove.as_mut_ptr(),
                3 as libc::c_int,
                30 as libc::c_int,
                0.1f32,
                stopevent,
                0 as libc::c_int,
                crate::src::qcommon::q_shared::qfalse as libc::c_int,
            );
            // if prediction time wasn't enough to fully predict the movement
            if move_0.frames >= 30 as libc::c_int {
                return crate::src::qcommon::q_shared::qfalse as libc::c_int;
            }
            // don't enter slime or lava and don't fall from too high
            if move_0.stopevent & (8 as libc::c_int | 16 as libc::c_int) != 0 {
                return crate::src::qcommon::q_shared::qfalse as libc::c_int;
            }
            // never jump or fall through a cluster portal
            if move_0.stopevent & 4096 as libc::c_int != 0 {
                return crate::src::qcommon::q_shared::qfalse as libc::c_int;
            }
            //the end position should be in area2, also test a little bit back
            //because the predicted jump could have rushed through the area
            teststart[0 as libc::c_int as usize] = move_0.endpos[0 as libc::c_int as usize]
                + dir[0 as libc::c_int as usize] * -(64 as libc::c_int) as libc::c_float; //end for
            teststart[1 as libc::c_int as usize] = move_0.endpos[1 as libc::c_int as usize]
                + dir[1 as libc::c_int as usize] * -(64 as libc::c_int) as libc::c_float;
            teststart[2 as libc::c_int as usize] = move_0.endpos[2 as libc::c_int as usize]
                + dir[2 as libc::c_int as usize] * -(64 as libc::c_int) as libc::c_float;
            teststart[2 as libc::c_int as usize] += 1 as libc::c_int as libc::c_float;
            numareas = crate::src::botlib::be_aas_sample::AAS_TraceAreas(
                move_0.endpos.as_mut_ptr(),
                teststart.as_mut_ptr(),
                areas.as_mut_ptr(),
                0 as *mut crate::src::qcommon::q_shared::vec3_t,
                (::std::mem::size_of::<[libc::c_int; 10]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    as libc::c_int,
            );
            j = 0 as libc::c_int;
            while j < numareas {
                if areas[j as usize] == area2num {
                    break;
                }
                j += 1
            }
            if j < numareas {
                break;
            }
            i += 1
        }
        if i >= 3 as libc::c_int {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        //
        //REACH_DEBUG
        //create a new reachability link
        lreach = AAS_AllocReachability(); //end if
        if lreach.is_null() {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        (*lreach).areanum = area2num;
        (*lreach).facenum = 0 as libc::c_int;
        (*lreach).edgenum = 0 as libc::c_int;
        (*lreach).start[0 as libc::c_int as usize] = beststart[0 as libc::c_int as usize];
        (*lreach).start[1 as libc::c_int as usize] = beststart[1 as libc::c_int as usize];
        (*lreach).start[2 as libc::c_int as usize] = beststart[2 as libc::c_int as usize];
        (*lreach).end[0 as libc::c_int as usize] = bestend[0 as libc::c_int as usize];
        (*lreach).end[1 as libc::c_int as usize] = bestend[1 as libc::c_int as usize];
        (*lreach).end[2 as libc::c_int as usize] = bestend[2 as libc::c_int as usize];
        (*lreach).traveltype = traveltype;
        dir[0 as libc::c_int as usize] =
            bestend[0 as libc::c_int as usize] - beststart[0 as libc::c_int as usize];
        dir[1 as libc::c_int as usize] =
            bestend[1 as libc::c_int as usize] - beststart[1 as libc::c_int as usize];
        dir[2 as libc::c_int as usize] =
            bestend[2 as libc::c_int as usize] - beststart[2 as libc::c_int as usize];
        height = dir[2 as libc::c_int as usize];
        dir[2 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        if traveltype & 0xffffff as libc::c_int == 7 as libc::c_int
            && height
                > VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t)
        {
            (*lreach).traveltime = (crate::src::botlib::be_aas_move::aassettings
                .rs_startwalkoffledge
                + height * 50 as libc::c_int as libc::c_float
                    / crate::src::botlib::be_aas_move::aassettings.phys_gravity)
                as libc::c_ushort
        } else {
            (*lreach).traveltime = (crate::src::botlib::be_aas_move::aassettings.rs_startjump
                + VectorDistance(bestend.as_mut_ptr(), beststart.as_mut_ptr())
                    * 240 as libc::c_int as libc::c_float
                    / crate::src::botlib::be_aas_move::aassettings.phys_maxwalkvelocity)
                as libc::c_ushort
        }
        //
        if AAS_AreaJumpPad(area2num) == 0 {
            if AAS_FallDelta(
                beststart[2 as libc::c_int as usize] - bestend[2 as libc::c_int as usize],
            ) > crate::src::botlib::be_aas_move::aassettings.phys_falldelta5
            {
                //end if
                (*lreach).traveltime = ((*lreach).traveltime as libc::c_float
                    + crate::src::botlib::be_aas_move::aassettings.rs_falldamage5)
                    as libc::c_ushort
            } else if AAS_FallDelta(
                beststart[2 as libc::c_int as usize] - bestend[2 as libc::c_int as usize],
            ) > crate::src::botlib::be_aas_move::aassettings.phys_falldelta10
            {
                (*lreach).traveltime = ((*lreach).traveltime as libc::c_float
                    + crate::src::botlib::be_aas_move::aassettings.rs_falldamage10)
                    as libc::c_ushort
            } //end if
              //end if
        }
        (*lreach).next = *areareachability.offset(area1num as isize);
        let ref mut fresh12 = *areareachability.offset(area1num as isize);
        *fresh12 = lreach;
        //
        if traveltype & 0xffffff as libc::c_int == 5 as libc::c_int {
            reach_jump += 1
        } else {
            reach_walkoffledge += 1
        }
    }
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//end of the function AAS_Reachability_Jump
//===========================================================================
// create a possible ladder reachability from area1 to area2
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Reachability_Ladder(
    mut area1num: libc::c_int,
    mut area2num: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut edge1num: libc::c_int = 0;
    let mut edge2num: libc::c_int = 0;
    let mut sharededgenum: libc::c_int = 0 as libc::c_int;
    let mut lowestedgenum: libc::c_int = 0 as libc::c_int;
    let mut face1num: libc::c_int = 0;
    let mut face2num: libc::c_int = 0;
    let mut ladderface1num: libc::c_int = 0 as libc::c_int;
    let mut ladderface2num: libc::c_int = 0 as libc::c_int;
    let mut ladderface1vertical: libc::c_int = 0;
    let mut ladderface2vertical: libc::c_int = 0;
    let mut firstv: libc::c_int = 0;
    let mut face1area: libc::c_float = 0.;
    let mut face2area: libc::c_float = 0.;
    let mut bestface1area: libc::c_float = -(9999 as libc::c_int) as libc::c_float;
    let mut bestface2area: libc::c_float = -(9999 as libc::c_int) as libc::c_float;
    let mut phys_jumpvel: libc::c_float = 0.;
    let mut maxjumpheight: libc::c_float = 0.;
    let mut area1point: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut area2point: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut mid: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut lowestpoint: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0.,
    ];
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut sharededgevec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut area1: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut area2: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut face1: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut face2: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut ladderface1: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut ladderface2: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut plane1: *mut crate::aasfile_h::aas_plane_t = 0 as *mut crate::aasfile_h::aas_plane_t;
    let mut plane2: *mut crate::aasfile_h::aas_plane_t = 0 as *mut crate::aasfile_h::aas_plane_t;
    let mut sharededge: *mut crate::aasfile_h::aas_edge_t = 0 as *mut crate::aasfile_h::aas_edge_t;
    let mut edge1: *mut crate::aasfile_h::aas_edge_t = 0 as *mut crate::aasfile_h::aas_edge_t;
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut trace: crate::be_aas_h::aas_trace_t = crate::be_aas_h::aas_trace_t {
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        ent: 0,
        lastarea: 0,
        area: 0,
        planenum: 0,
    };
    if AAS_AreaLadder(area1num) == 0 || AAS_AreaLadder(area2num) == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //
    phys_jumpvel = crate::src::botlib::be_aas_move::aassettings.phys_jumpvel;
    //maximum height a player can jump with the given initial z velocity
    maxjumpheight = AAS_MaxJumpHeight(phys_jumpvel); //end for
    area1 = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(area1num as isize) as *mut crate::aasfile_h::aas_area_t;
    area2 = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(area2num as isize) as *mut crate::aasfile_h::aas_area_t;
    i = 0 as libc::c_int;
    while i < (*area1).numfaces {
        face1num = *crate::src::botlib::be_aas_main::aasworld
            .faceindex
            .offset(((*area1).firstface + i) as isize);
        face1 = &mut *crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(
                (crate::stdlib::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(
                    face1num,
                ) as isize,
            ) as *mut crate::aasfile_h::aas_face_t;
        //end for
        //if not a ladder face
        if !((*face1).faceflags & 2 as libc::c_int == 0) {
            //
            j = 0 as libc::c_int;
            while j < (*area2).numfaces {
                face2num = *crate::src::botlib::be_aas_main::aasworld
                    .faceindex
                    .offset(((*area2).firstface + j) as isize);
                face2 = &mut *crate::src::botlib::be_aas_main::aasworld.faces.offset(
                    (crate::stdlib::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(
                        face2num,
                    ) as isize,
                ) as *mut crate::aasfile_h::aas_face_t;
                //end for
                //if not a ladder face
                if !((*face2).faceflags & 2 as libc::c_int == 0) {
                    //check if the faces share an edge
                    k = 0 as libc::c_int; //end for
                    while k < (*face1).numedges {
                        edge1num = *crate::src::botlib::be_aas_main::aasworld
                            .edgeindex
                            .offset(((*face1).firstedge + k) as isize);
                        l = 0 as libc::c_int;
                        while l < (*face2).numedges {
                            edge2num = *crate::src::botlib::be_aas_main::aasworld
                                .edgeindex
                                .offset(((*face2).firstedge + l) as isize);
                            if crate::stdlib::abs(edge1num) == crate::stdlib::abs(edge2num) {
                                //end if
                                //get the face with the largest area
                                face1area = AAS_FaceArea(face1); //end if
                                face2area = AAS_FaceArea(face2);
                                if face1area > bestface1area && face2area > bestface2area {
                                    bestface1area = face1area;
                                    bestface2area = face2area;
                                    ladderface1 = face1;
                                    ladderface2 = face2;
                                    ladderface1num = face1num;
                                    ladderface2num = face2num;
                                    sharededgenum = edge1num
                                }
                                break;
                            } else {
                                l += 1
                            }
                        }
                        if l != (*face2).numedges {
                            break;
                        }
                        k += 1
                    }
                }
                j += 1
            }
        }
        i += 1
    }
    //
    if !ladderface1.is_null() && !ladderface2.is_null() {
        //end if
        //get the middle of the shared edge
        sharededge =
            &mut *crate::src::botlib::be_aas_main::aasworld
                .edges
                .offset(
                    (crate::stdlib::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(
                        sharededgenum,
                    ) as isize,
                ) as *mut crate::aasfile_h::aas_edge_t;
        firstv = (sharededgenum < 0 as libc::c_int) as libc::c_int;
        //end if
        v1[0 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .vertexes
            .offset((*sharededge).v[firstv as usize] as isize))[0 as libc::c_int as usize];
        v1[1 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .vertexes
            .offset((*sharededge).v[firstv as usize] as isize))[1 as libc::c_int as usize];
        v1[2 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .vertexes
            .offset((*sharededge).v[firstv as usize] as isize))[2 as libc::c_int as usize];
        v2[0 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .vertexes
            .offset((*sharededge).v[(firstv == 0) as libc::c_int as usize] as isize))
            [0 as libc::c_int as usize];
        v2[1 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .vertexes
            .offset((*sharededge).v[(firstv == 0) as libc::c_int as usize] as isize))
            [1 as libc::c_int as usize];
        v2[2 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
            .vertexes
            .offset((*sharededge).v[(firstv == 0) as libc::c_int as usize] as isize))
            [2 as libc::c_int as usize];
        area1point[0 as libc::c_int as usize] =
            v1[0 as libc::c_int as usize] + v2[0 as libc::c_int as usize];
        area1point[1 as libc::c_int as usize] =
            v1[1 as libc::c_int as usize] + v2[1 as libc::c_int as usize];
        area1point[2 as libc::c_int as usize] =
            v1[2 as libc::c_int as usize] + v2[2 as libc::c_int as usize];
        area1point[0 as libc::c_int as usize] =
            (area1point[0 as libc::c_int as usize] as libc::c_double * 0.5f64)
                as crate::src::qcommon::q_shared::vec_t;
        area1point[1 as libc::c_int as usize] =
            (area1point[1 as libc::c_int as usize] as libc::c_double * 0.5f64)
                as crate::src::qcommon::q_shared::vec_t;
        area1point[2 as libc::c_int as usize] =
            (area1point[2 as libc::c_int as usize] as libc::c_double * 0.5f64)
                as crate::src::qcommon::q_shared::vec_t;
        area2point[0 as libc::c_int as usize] = area1point[0 as libc::c_int as usize];
        area2point[1 as libc::c_int as usize] = area1point[1 as libc::c_int as usize];
        area2point[2 as libc::c_int as usize] = area1point[2 as libc::c_int as usize];
        plane1 = &mut *crate::src::botlib::be_aas_main::aasworld.planes.offset(
            ((*ladderface1).planenum ^ (ladderface1num < 0 as libc::c_int) as libc::c_int) as isize,
        ) as *mut crate::aasfile_h::aas_plane_t;
        plane2 = &mut *crate::src::botlib::be_aas_main::aasworld.planes.offset(
            ((*ladderface2).planenum ^ (ladderface2num < 0 as libc::c_int) as libc::c_int) as isize,
        ) as *mut crate::aasfile_h::aas_plane_t;
        sharededgevec[0 as libc::c_int as usize] =
            v2[0 as libc::c_int as usize] - v1[0 as libc::c_int as usize];
        sharededgevec[1 as libc::c_int as usize] =
            v2[1 as libc::c_int as usize] - v1[1 as libc::c_int as usize];
        sharededgevec[2 as libc::c_int as usize] =
            v2[2 as libc::c_int as usize] - v1[2 as libc::c_int as usize];
        CrossProduct(
            (*plane1).normal.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            sharededgevec.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            dir.as_mut_ptr(),
        );
        crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
        area1point[0 as libc::c_int as usize] = area1point[0 as libc::c_int as usize]
            + dir[0 as libc::c_int as usize] * -(32 as libc::c_int) as libc::c_float;
        area1point[1 as libc::c_int as usize] = area1point[1 as libc::c_int as usize]
            + dir[1 as libc::c_int as usize] * -(32 as libc::c_int) as libc::c_float;
        area1point[2 as libc::c_int as usize] = area1point[2 as libc::c_int as usize]
            + dir[2 as libc::c_int as usize] * -(32 as libc::c_int) as libc::c_float;
        area2point[0 as libc::c_int as usize] = area2point[0 as libc::c_int as usize]
            + dir[0 as libc::c_int as usize] * 32 as libc::c_int as libc::c_float;
        area2point[1 as libc::c_int as usize] = area2point[1 as libc::c_int as usize]
            + dir[1 as libc::c_int as usize] * 32 as libc::c_int as libc::c_float;
        area2point[2 as libc::c_int as usize] = area2point[2 as libc::c_int as usize]
            + dir[2 as libc::c_int as usize] * 32 as libc::c_int as libc::c_float;
        ladderface1vertical = ((crate::stdlib::fabsf(
            (*plane1).normal[0 as libc::c_int as usize] * up[0 as libc::c_int as usize]
                + (*plane1).normal[1 as libc::c_int as usize] * up[1 as libc::c_int as usize]
                + (*plane1).normal[2 as libc::c_int as usize] * up[2 as libc::c_int as usize],
        ) as libc::c_double)
            < 0.1f64) as libc::c_int;
        ladderface2vertical = ((crate::stdlib::fabsf(
            (*plane2).normal[0 as libc::c_int as usize] * up[0 as libc::c_int as usize]
                + (*plane2).normal[1 as libc::c_int as usize] * up[1 as libc::c_int as usize]
                + (*plane2).normal[2 as libc::c_int as usize] * up[2 as libc::c_int as usize],
        ) as libc::c_double)
            < 0.1f64) as libc::c_int;
        if ladderface1vertical == 0 && ladderface2vertical == 0 {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        if ladderface1vertical != 0
            && ladderface2vertical != 0
            && ((*plane1).normal[0 as libc::c_int as usize]
                * (*plane2).normal[0 as libc::c_int as usize]
                + (*plane1).normal[1 as libc::c_int as usize]
                    * (*plane2).normal[1 as libc::c_int as usize]
                + (*plane1).normal[2 as libc::c_int as usize]
                    * (*plane2).normal[2 as libc::c_int as usize]) as libc::c_double
                > 0.7f64
            && (crate::stdlib::fabsf(
                sharededgevec[0 as libc::c_int as usize] * up[0 as libc::c_int as usize]
                    + sharededgevec[1 as libc::c_int as usize] * up[1 as libc::c_int as usize]
                    + sharededgevec[2 as libc::c_int as usize] * up[2 as libc::c_int as usize],
            ) as libc::c_double)
                < 0.7f64
        {
            //
            //
            //if the face plane in area 1 is pretty much vertical
            //
            //get the points really into the areas
            //NOTE: 32 because that's larger than 16 (bot bbox x,y)
            //
            //there's only reachability between vertical ladder faces
            //if both vertical ladder faces
            //end if
            //create a new reachability link
            lreach = AAS_AllocReachability();
            if lreach.is_null() {
                return crate::src::qcommon::q_shared::qfalse as libc::c_int;
            }
            (*lreach).areanum = area2num;
            (*lreach).facenum = ladderface1num;
            (*lreach).edgenum = crate::stdlib::abs(sharededgenum);
            (*lreach).start[0 as libc::c_int as usize] = area1point[0 as libc::c_int as usize];
            (*lreach).start[1 as libc::c_int as usize] = area1point[1 as libc::c_int as usize];
            (*lreach).start[2 as libc::c_int as usize] = area1point[2 as libc::c_int as usize];
            //VectorCopy(area2point, lreach->end);
            (*lreach).end[0 as libc::c_int as usize] = area2point[0 as libc::c_int as usize]
                + (*plane1).normal[0 as libc::c_int as usize]
                    * -(3 as libc::c_int) as libc::c_float;
            (*lreach).end[1 as libc::c_int as usize] = area2point[1 as libc::c_int as usize]
                + (*plane1).normal[1 as libc::c_int as usize]
                    * -(3 as libc::c_int) as libc::c_float;
            (*lreach).end[2 as libc::c_int as usize] = area2point[2 as libc::c_int as usize]
                + (*plane1).normal[2 as libc::c_int as usize]
                    * -(3 as libc::c_int) as libc::c_float;
            (*lreach).traveltype = 6 as libc::c_int;
            (*lreach).traveltime = 10 as libc::c_int as libc::c_ushort;
            (*lreach).next = *areareachability.offset(area1num as isize);
            let ref mut fresh13 = *areareachability.offset(area1num as isize);
            *fresh13 = lreach;
            //
            reach_ladder += 1;
            //create a new reachability link
            lreach = AAS_AllocReachability();
            if lreach.is_null() {
                return crate::src::qcommon::q_shared::qfalse as libc::c_int;
            }
            (*lreach).areanum = area1num;
            (*lreach).facenum = ladderface2num;
            (*lreach).edgenum = crate::stdlib::abs(sharededgenum);
            (*lreach).start[0 as libc::c_int as usize] = area2point[0 as libc::c_int as usize];
            (*lreach).start[1 as libc::c_int as usize] = area2point[1 as libc::c_int as usize];
            (*lreach).start[2 as libc::c_int as usize] = area2point[2 as libc::c_int as usize];
            //VectorCopy(area1point, lreach->end);
            (*lreach).end[0 as libc::c_int as usize] = area1point[0 as libc::c_int as usize]
                + (*plane1).normal[0 as libc::c_int as usize]
                    * -(3 as libc::c_int) as libc::c_float;
            (*lreach).end[1 as libc::c_int as usize] = area1point[1 as libc::c_int as usize]
                + (*plane1).normal[1 as libc::c_int as usize]
                    * -(3 as libc::c_int) as libc::c_float;
            (*lreach).end[2 as libc::c_int as usize] = area1point[2 as libc::c_int as usize]
                + (*plane1).normal[2 as libc::c_int as usize]
                    * -(3 as libc::c_int) as libc::c_float;
            (*lreach).traveltype = 6 as libc::c_int;
            (*lreach).traveltime = 10 as libc::c_int as libc::c_ushort;
            (*lreach).next = *areareachability.offset(area2num as isize);
            let ref mut fresh14 = *areareachability.offset(area2num as isize);
            *fresh14 = lreach;
            //
            reach_ladder += 1;
            //
            return crate::src::qcommon::q_shared::qtrue as libc::c_int;
        }
        if ladderface1vertical != 0 && (*ladderface2).faceflags & 4 as libc::c_int != 0 {
            //if the second ladder face is also a ground face
            //create ladder end (just ladder) reachability and
            //walk off a ladder (ledge) reachability
            //end if
            //create a new reachability link
            lreach = AAS_AllocReachability();
            if lreach.is_null() {
                return crate::src::qcommon::q_shared::qfalse as libc::c_int;
            }
            (*lreach).areanum = area2num;
            (*lreach).facenum = ladderface1num;
            (*lreach).edgenum = crate::stdlib::abs(sharededgenum);
            (*lreach).start[0 as libc::c_int as usize] = area1point[0 as libc::c_int as usize];
            (*lreach).start[1 as libc::c_int as usize] = area1point[1 as libc::c_int as usize];
            (*lreach).start[2 as libc::c_int as usize] = area1point[2 as libc::c_int as usize];
            (*lreach).end[0 as libc::c_int as usize] = area2point[0 as libc::c_int as usize];
            (*lreach).end[1 as libc::c_int as usize] = area2point[1 as libc::c_int as usize];
            (*lreach).end[2 as libc::c_int as usize] = area2point[2 as libc::c_int as usize];
            (*lreach).end[2 as libc::c_int as usize] += 16 as libc::c_int as libc::c_float;
            (*lreach).end[0 as libc::c_int as usize] = (*lreach).end[0 as libc::c_int as usize]
                + (*plane1).normal[0 as libc::c_int as usize]
                    * -(15 as libc::c_int) as libc::c_float;
            (*lreach).end[1 as libc::c_int as usize] = (*lreach).end[1 as libc::c_int as usize]
                + (*plane1).normal[1 as libc::c_int as usize]
                    * -(15 as libc::c_int) as libc::c_float;
            (*lreach).end[2 as libc::c_int as usize] = (*lreach).end[2 as libc::c_int as usize]
                + (*plane1).normal[2 as libc::c_int as usize]
                    * -(15 as libc::c_int) as libc::c_float;
            (*lreach).traveltype = 6 as libc::c_int;
            (*lreach).traveltime = 10 as libc::c_int as libc::c_ushort;
            (*lreach).next = *areareachability.offset(area1num as isize);
            let ref mut fresh15 = *areareachability.offset(area1num as isize);
            *fresh15 = lreach;
            //
            reach_ladder += 1;
            //create a new reachability link
            lreach = AAS_AllocReachability();
            if lreach.is_null() {
                return crate::src::qcommon::q_shared::qfalse as libc::c_int;
            }
            (*lreach).areanum = area1num;
            (*lreach).facenum = ladderface2num;
            (*lreach).edgenum = crate::stdlib::abs(sharededgenum);
            (*lreach).start[0 as libc::c_int as usize] = area2point[0 as libc::c_int as usize];
            (*lreach).start[1 as libc::c_int as usize] = area2point[1 as libc::c_int as usize];
            (*lreach).start[2 as libc::c_int as usize] = area2point[2 as libc::c_int as usize];
            (*lreach).end[0 as libc::c_int as usize] = area1point[0 as libc::c_int as usize];
            (*lreach).end[1 as libc::c_int as usize] = area1point[1 as libc::c_int as usize];
            (*lreach).end[2 as libc::c_int as usize] = area1point[2 as libc::c_int as usize];
            (*lreach).traveltype = 7 as libc::c_int;
            (*lreach).traveltime = 10 as libc::c_int as libc::c_ushort;
            (*lreach).next = *areareachability.offset(area2num as isize);
            let ref mut fresh16 = *areareachability.offset(area2num as isize);
            *fresh16 = lreach;
            //
            reach_walkoffledge += 1;
            //
            return crate::src::qcommon::q_shared::qtrue as libc::c_int;
        }
        if ladderface1vertical != 0 {
            //
            //find lowest edge of the ladder face
            lowestpoint[2 as libc::c_int as usize] =
                99999 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
            //end if
            /*//if slime or lava below the ladder
            //try jump reachability from far towards the ladder
            if (aasworld.areasettings[area2num].contents & (AREACONTENTS_SLIME
                                                    | AREACONTENTS_LAVA))
            {
                for (i = 20; i <= 120; i += 20)
                {
                    //trace down in the middle of this edge
                    VectorMA(lowestpoint, i, plane1->normal, start);
                    VectorCopy(start, end);
                    start[2] += 5;
                    end[2] -= 100;
                    //trace without entity collision
                    trace = AAS_TraceClientBBox(start, end, PRESENCE_NORMAL, -1);
                    //
                    if (trace.startsolid) break;
                    trace.endpos[2] += 1;
                    area2num = AAS_PointAreaNum(trace.endpos);
                    if (area2num == area1num) continue;
                    //
                    if (start[2] - trace.endpos[2] > maxjumpheight) continue;
                    if (aasworld.areasettings[area2num].contents & (AREACONTENTS_SLIME
                                                | AREACONTENTS_LAVA)) continue;
                    //
                    //create a new reachability link
                    lreach = AAS_AllocReachability();
                    if (!lreach) return qfalse;
                    lreach->areanum = area1num;
                    lreach->facenum = ladderface1num;
                    lreach->edgenum = lowestedgenum;
                    VectorCopy(trace.endpos, lreach->start);
                    VectorCopy(lowestpoint, lreach->end);
                    lreach->end[2] += 5;
                    lreach->traveltype = TRAVEL_JUMP;
                    lreach->traveltime = 10;
                    lreach->next = areareachability[area2num];
                    areareachability[area2num] = lreach;
                    //
                    reach_jump++;
                    //
                    Log_Write("jump far to ladder reach between %d and %d\r\n", area2num, area1num);
                    //
                    break;
                } //end for
            } //end if*/
            i = 0 as libc::c_int; //end for
            while i < (*ladderface1).numedges {
                edge1num = crate::stdlib::abs(
                    *crate::src::botlib::be_aas_main::aasworld
                        .edgeindex
                        .offset(((*ladderface1).firstedge + i) as isize),
                );
                edge1 = &mut *crate::src::botlib::be_aas_main::aasworld
                    .edges
                    .offset(edge1num as isize)
                    as *mut crate::aasfile_h::aas_edge_t;
                //end if
                v1[0 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
                    .vertexes
                    .offset((*edge1).v[0 as libc::c_int as usize] as isize))
                    [0 as libc::c_int as usize];
                v1[1 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
                    .vertexes
                    .offset((*edge1).v[0 as libc::c_int as usize] as isize))
                    [1 as libc::c_int as usize];
                v1[2 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
                    .vertexes
                    .offset((*edge1).v[0 as libc::c_int as usize] as isize))
                    [2 as libc::c_int as usize];
                v2[0 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
                    .vertexes
                    .offset((*edge1).v[1 as libc::c_int as usize] as isize))
                    [0 as libc::c_int as usize];
                v2[1 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
                    .vertexes
                    .offset((*edge1).v[1 as libc::c_int as usize] as isize))
                    [1 as libc::c_int as usize];
                v2[2 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
                    .vertexes
                    .offset((*edge1).v[1 as libc::c_int as usize] as isize))
                    [2 as libc::c_int as usize];
                mid[0 as libc::c_int as usize] =
                    v1[0 as libc::c_int as usize] + v2[0 as libc::c_int as usize];
                mid[1 as libc::c_int as usize] =
                    v1[1 as libc::c_int as usize] + v2[1 as libc::c_int as usize];
                mid[2 as libc::c_int as usize] =
                    v1[2 as libc::c_int as usize] + v2[2 as libc::c_int as usize];
                mid[0 as libc::c_int as usize] = (mid[0 as libc::c_int as usize] as libc::c_double
                    * 0.5f64)
                    as crate::src::qcommon::q_shared::vec_t;
                mid[1 as libc::c_int as usize] = (mid[1 as libc::c_int as usize] as libc::c_double
                    * 0.5f64)
                    as crate::src::qcommon::q_shared::vec_t;
                mid[2 as libc::c_int as usize] = (mid[2 as libc::c_int as usize] as libc::c_double
                    * 0.5f64)
                    as crate::src::qcommon::q_shared::vec_t;
                if mid[2 as libc::c_int as usize] < lowestpoint[2 as libc::c_int as usize] {
                    lowestpoint[0 as libc::c_int as usize] = mid[0 as libc::c_int as usize];
                    lowestpoint[1 as libc::c_int as usize] = mid[1 as libc::c_int as usize];
                    lowestpoint[2 as libc::c_int as usize] = mid[2 as libc::c_int as usize];
                    lowestedgenum = edge1num
                }
                i += 1
            }
            plane1 = &mut *crate::src::botlib::be_aas_main::aasworld
                .planes
                .offset((*ladderface1).planenum as isize)
                as *mut crate::aasfile_h::aas_plane_t;
            start[0 as libc::c_int as usize] = lowestpoint[0 as libc::c_int as usize]
                + (*plane1).normal[0 as libc::c_int as usize] * 5 as libc::c_int as libc::c_float;
            start[1 as libc::c_int as usize] = lowestpoint[1 as libc::c_int as usize]
                + (*plane1).normal[1 as libc::c_int as usize] * 5 as libc::c_int as libc::c_float;
            start[2 as libc::c_int as usize] = lowestpoint[2 as libc::c_int as usize]
                + (*plane1).normal[2 as libc::c_int as usize] * 5 as libc::c_int as libc::c_float;
            end[0 as libc::c_int as usize] = start[0 as libc::c_int as usize];
            end[1 as libc::c_int as usize] = start[1 as libc::c_int as usize];
            end[2 as libc::c_int as usize] = start[2 as libc::c_int as usize];
            start[2 as libc::c_int as usize] += 5 as libc::c_int as libc::c_float;
            end[2 as libc::c_int as usize] -= 100 as libc::c_int as libc::c_float;
            trace = crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(
                start.as_mut_ptr(),
                end.as_mut_ptr(),
                2 as libc::c_int,
                -(1 as libc::c_int),
            );
            trace.endpos[2 as libc::c_int as usize] += 1 as libc::c_int as libc::c_float;
            area2num =
                crate::src::botlib::be_aas_sample::AAS_PointAreaNum(trace.endpos.as_mut_ptr());
            area2 = &mut *crate::src::botlib::be_aas_main::aasworld
                .areas
                .offset(area2num as isize) as *mut crate::aasfile_h::aas_area_t;
            i = 0 as libc::c_int;
            while i < (*area2).numfaces {
                face2num = *crate::src::botlib::be_aas_main::aasworld
                    .faceindex
                    .offset(((*area2).firstface + i) as isize);
                face2 = &mut *crate::src::botlib::be_aas_main::aasworld.faces.offset(
                    (crate::stdlib::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(
                        face2num,
                    ) as isize,
                ) as *mut crate::aasfile_h::aas_face_t;
                //
                //
                //
                //
                //trace down in the middle of this edge
                //trace without entity collision
                //
                //
                //REACH_DEBUG
                //
                //
                //end for
                //end if
                //
                if (*face2).faceflags & 2 as libc::c_int != 0 {
                    plane2 = &mut *crate::src::botlib::be_aas_main::aasworld
                        .planes
                        .offset((*face2).planenum as isize)
                        as *mut crate::aasfile_h::aas_plane_t;
                    if (crate::stdlib::fabsf(
                        (*plane2).normal[0 as libc::c_int as usize] * up[0 as libc::c_int as usize]
                            + (*plane2).normal[1 as libc::c_int as usize]
                                * up[1 as libc::c_int as usize]
                            + (*plane2).normal[2 as libc::c_int as usize]
                                * up[2 as libc::c_int as usize],
                    ) as libc::c_double)
                        < 0.1f64
                    {
                        break;
                    }
                }
                i += 1
            }
            if i >= (*area2).numfaces
                && area2num != area1num
                && AAS_ReachabilityExists(area1num, area2num) as u64 == 0
                && AAS_ReachabilityExists(area2num, area1num) as u64 == 0
            {
                //if from another area without vertical ladder faces
                //if the height is jumpable
                if start[2 as libc::c_int as usize] - trace.endpos[2 as libc::c_int as usize]
                    < maxjumpheight
                {
                    //create a new reachability link
                    lreach = AAS_AllocReachability();
                    if lreach.is_null() {
                        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                    }
                    (*lreach).areanum = area2num;
                    (*lreach).facenum = ladderface1num;
                    (*lreach).edgenum = lowestedgenum;
                    (*lreach).start[0 as libc::c_int as usize] =
                        lowestpoint[0 as libc::c_int as usize];
                    (*lreach).start[1 as libc::c_int as usize] =
                        lowestpoint[1 as libc::c_int as usize];
                    (*lreach).start[2 as libc::c_int as usize] =
                        lowestpoint[2 as libc::c_int as usize];
                    (*lreach).end[0 as libc::c_int as usize] =
                        trace.endpos[0 as libc::c_int as usize];
                    (*lreach).end[1 as libc::c_int as usize] =
                        trace.endpos[1 as libc::c_int as usize];
                    (*lreach).end[2 as libc::c_int as usize] =
                        trace.endpos[2 as libc::c_int as usize];
                    (*lreach).traveltype = 6 as libc::c_int;
                    (*lreach).traveltime = 10 as libc::c_int as libc::c_ushort;
                    (*lreach).next = *areareachability.offset(area1num as isize);
                    let ref mut fresh17 = *areareachability.offset(area1num as isize);
                    *fresh17 = lreach;
                    //REACH_DEBUG
                    reach_ladder += 1;
                    lreach = AAS_AllocReachability();
                    if lreach.is_null() {
                        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                    }
                    (*lreach).areanum = area1num;
                    (*lreach).facenum = ladderface1num;
                    (*lreach).edgenum = lowestedgenum;
                    (*lreach).start[0 as libc::c_int as usize] =
                        trace.endpos[0 as libc::c_int as usize];
                    (*lreach).start[1 as libc::c_int as usize] =
                        trace.endpos[1 as libc::c_int as usize];
                    (*lreach).start[2 as libc::c_int as usize] =
                        trace.endpos[2 as libc::c_int as usize];
                    (*lreach).end[0 as libc::c_int as usize] = lowestpoint
                        [0 as libc::c_int as usize]
                        + (*plane1).normal[0 as libc::c_int as usize]
                            * -(5 as libc::c_int) as libc::c_float;
                    (*lreach).end[1 as libc::c_int as usize] = lowestpoint
                        [1 as libc::c_int as usize]
                        + (*plane1).normal[1 as libc::c_int as usize]
                            * -(5 as libc::c_int) as libc::c_float;
                    (*lreach).end[2 as libc::c_int as usize] = lowestpoint
                        [2 as libc::c_int as usize]
                        + (*plane1).normal[2 as libc::c_int as usize]
                            * -(5 as libc::c_int) as libc::c_float;
                    (*lreach).end[2 as libc::c_int as usize] += 10 as libc::c_int as libc::c_float;
                    (*lreach).traveltype = 5 as libc::c_int;
                    (*lreach).traveltime = 10 as libc::c_int as libc::c_ushort;
                    (*lreach).next = *areareachability.offset(area2num as isize);
                    let ref mut fresh18 = *areareachability.offset(area2num as isize);
                    *fresh18 = lreach;
                    reach_jump += 1;
                    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
                }
                //
                //create a new reachability link
                //get the end point a little bit into the ladder
                //get the end point a little higher
                //
                //
                //end if
                //REACH_DEBUG
            }
        }
    }
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//end of the function AAS_Reachability_Ladder
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_TravelFlagsForTeam(mut ent: libc::c_int) -> libc::c_int {
    let mut notteam: libc::c_int = 0;
    if crate::src::botlib::be_aas_bspq3::AAS_IntForBSPEpairKey(
        ent,
        b"bot_notteam\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut notteam,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if notteam == 1 as libc::c_int {
        return (1 as libc::c_int) << 24 as libc::c_int;
    }
    if notteam == 2 as libc::c_int {
        return (2 as libc::c_int) << 24 as libc::c_int;
    }
    return 0 as libc::c_int;
}
//end of the function AAS_TravelFlagsForTeam
//===========================================================================
// create possible teleporter reachabilities
// this is very game dependent.... :(
//
// classname = trigger_multiple or trigger_teleport
// target = "t1"
//
// classname = target_teleporter
// targetname = "t1"
// target = "t2"
//
// classname = misc_teleporter_dest
// targetname = "t2"
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Reachability_Teleport() {
    let mut area1num: libc::c_int = 0; //end else
    let mut area2num: libc::c_int = 0;
    let mut target: [libc::c_char; 128] = [0; 128];
    let mut targetname: [libc::c_char; 128] = [0; 128];
    let mut classname: [libc::c_char; 128] = [0; 128];
    let mut model: [libc::c_char; 128] = [0; 128];
    let mut ent: libc::c_int = 0;
    let mut dest: libc::c_int = 0;
    let mut angle: libc::c_float = 0.;
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut destorigin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut mid: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut cmdmove: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut move_0: crate::be_aas_h::aas_clientmove_t = crate::be_aas_h::aas_clientmove_t {
        endpos: [0.; 3],
        endarea: 0,
        velocity: [0.; 3],
        trace: crate::be_aas_h::aas_trace_t {
            startsolid: crate::src::qcommon::q_shared::qfalse,
            fraction: 0.,
            endpos: [0.; 3],
            ent: 0,
            lastarea: 0,
            area: 0,
            planenum: 0,
        },
        presencetype: 0,
        stopevent: 0,
        endcontents: 0,
        time: 0.,
        frames: 0,
    };
    let mut trace: crate::be_aas_h::aas_trace_t = crate::be_aas_h::aas_trace_t {
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        ent: 0,
        lastarea: 0,
        area: 0,
        planenum: 0,
    };
    let mut areas: *mut crate::be_aas_def_h::aas_link_t = 0 as *mut crate::be_aas_def_h::aas_link_t;
    let mut link: *mut crate::be_aas_def_h::aas_link_t = 0 as *mut crate::be_aas_def_h::aas_link_t;
    let mut current_block_61: u64;
    ent = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(0 as libc::c_int);
    while ent != 0 {
        if !(crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
            ent,
            b"classname\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            classname.as_mut_ptr(),
            128 as libc::c_int,
        ) == 0)
        {
            if crate::stdlib::strcmp(
                classname.as_mut_ptr(),
                b"trigger_multiple\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
                    ent,
                    b"model\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    model.as_mut_ptr(),
                    128 as libc::c_int,
                );
                //end if
                //#ifdef REACH_DEBUG
                botimport.Print.expect("non-null function pointer")(
                    1 as libc::c_int,
                    b"trigger_multiple model = \"%s\"\n\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    model.as_mut_ptr(),
                );
                //#endif REACH_DEBUG
                angles[2 as libc::c_int as usize] =
                    0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
                angles[1 as libc::c_int as usize] = angles[2 as libc::c_int as usize];
                angles[0 as libc::c_int as usize] = angles[1 as libc::c_int as usize];
                crate::src::botlib::be_aas_bspq3::AAS_BSPModelMinsMaxsOrigin(
                    crate::stdlib::atoi(model.as_mut_ptr().offset(1 as libc::c_int as isize)),
                    angles.as_mut_ptr(),
                    mins.as_mut_ptr(),
                    maxs.as_mut_ptr(),
                    origin.as_mut_ptr(),
                );
                if crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
                    ent,
                    b"target\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    target.as_mut_ptr(),
                    128 as libc::c_int,
                ) == 0
                {
                    botimport.Print.expect("non-null function pointer")(
                        3 as libc::c_int,
                        b"trigger_multiple at %1.0f %1.0f %1.0f without target\n\x00" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        origin[0 as libc::c_int as usize] as libc::c_double,
                        origin[1 as libc::c_int as usize] as libc::c_double,
                        origin[2 as libc::c_int as usize] as libc::c_double,
                    );
                    current_block_61 = 7351195479953500246;
                } else {
                    //
                    //end if
                    dest = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(0 as libc::c_int); //end for
                    while dest != 0 {
                        if !(crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
                            dest,
                            b"classname\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            classname.as_mut_ptr(),
                            128 as libc::c_int,
                        ) == 0)
                        {
                            if crate::stdlib::strcmp(
                                classname.as_mut_ptr(),
                                b"target_teleporter\x00" as *const u8 as *const libc::c_char,
                            ) == 0
                            {
                                if !(crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
                                    dest,
                                    b"targetname\x00" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                    targetname.as_mut_ptr(),
                                    128 as libc::c_int,
                                ) == 0)
                                {
                                    if crate::stdlib::strcmp(
                                        targetname.as_mut_ptr(),
                                        target.as_mut_ptr(),
                                    ) == 0
                                    {
                                        break;
                                    }
                                }
                                //end if
                            }
                        }
                        dest = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(dest)
                        //end if
                    } //end if
                    if dest == 0 {
                        current_block_61 = 7351195479953500246;
                    } else if crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
                        dest,
                        b"target\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        target.as_mut_ptr(),
                        128 as libc::c_int,
                    ) == 0
                    {
                        botimport.Print.expect("non-null function pointer")(
                            3 as libc::c_int,
                            b"target_teleporter without target\n\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        current_block_61 = 7351195479953500246;
                    } else {
                        current_block_61 = 3123434771885419771;
                    }
                }
            } else if crate::stdlib::strcmp(
                classname.as_mut_ptr(),
                b"trigger_teleport\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
                    ent,
                    b"model\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    model.as_mut_ptr(),
                    128 as libc::c_int,
                );
                //end if
                //#ifdef REACH_DEBUG
                botimport.Print.expect("non-null function pointer")(
                    1 as libc::c_int,
                    b"trigger_teleport model = \"%s\"\n\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    model.as_mut_ptr(),
                );
                //#endif REACH_DEBUG
                angles[2 as libc::c_int as usize] =
                    0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
                angles[1 as libc::c_int as usize] = angles[2 as libc::c_int as usize];
                angles[0 as libc::c_int as usize] = angles[1 as libc::c_int as usize];
                crate::src::botlib::be_aas_bspq3::AAS_BSPModelMinsMaxsOrigin(
                    crate::stdlib::atoi(model.as_mut_ptr().offset(1 as libc::c_int as isize)),
                    angles.as_mut_ptr(),
                    mins.as_mut_ptr(),
                    maxs.as_mut_ptr(),
                    origin.as_mut_ptr(),
                );
                if crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
                    ent,
                    b"target\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    target.as_mut_ptr(),
                    128 as libc::c_int,
                ) == 0
                {
                    botimport.Print.expect("non-null function pointer")(
                        3 as libc::c_int,
                        b"trigger_teleport at %1.0f %1.0f %1.0f without target\n\x00" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        origin[0 as libc::c_int as usize] as libc::c_double,
                        origin[1 as libc::c_int as usize] as libc::c_double,
                        origin[2 as libc::c_int as usize] as libc::c_double,
                    );
                    current_block_61 = 7351195479953500246;
                } else {
                    current_block_61 = 3123434771885419771;
                }
            } else {
                current_block_61 = 7351195479953500246;
            }
            match current_block_61 {
                7351195479953500246 => {}
                _ => {
                    //
                    //
                    dest = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(0 as libc::c_int); //end for
                    while dest != 0 {
                        //classname should be misc_teleporter_dest
                        //but I've also seen target_position and actually any
                        //entity could be used... burp
                        if crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
                            dest,
                            b"targetname\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            targetname.as_mut_ptr(),
                            128 as libc::c_int,
                        ) != 0
                        {
                            if crate::stdlib::strcmp(targetname.as_mut_ptr(), target.as_mut_ptr())
                                == 0
                            {
                                break;
                                //end if
                            }
                        }
                        dest = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(dest)
                        //end if
                    } //end if
                    if dest == 0 {
                        botimport.Print.expect("non-null function pointer")(
                            3 as libc::c_int,
                            b"teleporter without misc_teleporter_dest (%s)\n\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                            target.as_mut_ptr(),
                        ); //end if
                    } else if crate::src::botlib::be_aas_bspq3::AAS_VectorForBSPEpairKey(
                        dest,
                        b"origin\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        destorigin.as_mut_ptr(),
                    ) == 0
                    {
                        botimport.Print.expect("non-null function pointer")(
                            3 as libc::c_int,
                            b"teleporter destination (%s) without origin\n\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                            target.as_mut_ptr(),
                        );
                    } else {
                        //
                        area2num = crate::src::botlib::be_aas_sample::AAS_PointAreaNum(
                            destorigin.as_mut_ptr(),
                        );
                        //if not teleported into a teleporter or into a jumppad
                        if AAS_AreaTeleporter(area2num) == 0 && AAS_AreaJumpPad(area2num) == 0 {
                            end[0 as libc::c_int as usize] = destorigin[0 as libc::c_int as usize]; //end if
                            end[1 as libc::c_int as usize] = destorigin[1 as libc::c_int as usize];
                            end[2 as libc::c_int as usize] = destorigin[2 as libc::c_int as usize];
                            end[2 as libc::c_int as usize] -= 64 as libc::c_int as libc::c_float;
                            trace = crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(
                                destorigin.as_mut_ptr(),
                                end.as_mut_ptr(),
                                4 as libc::c_int,
                                -(1 as libc::c_int),
                            );
                            //end else
                            if trace.startsolid as u64 != 0 {
                                botimport.Print.expect("non-null function pointer")(
                                    3 as libc::c_int,
                                    b"teleporter destination (%s) in solid\n\x00" as *const u8
                                        as *const libc::c_char
                                        as *mut libc::c_char,
                                    target.as_mut_ptr(),
                                ); //end if
                                current_block_61 = 7351195479953500246;
                            } else {
                                /*
                                area2num = AAS_PointAreaNum(trace.endpos);
                                //
                                if (!AAS_AreaTeleporter(area2num) &&
                                    !AAS_AreaJumpPad(area2num) &&
                                    !AAS_AreaGrounded(area2num))
                                {
                                    VectorCopy(trace.endpos, destorigin);
                                }
                                else*/
                                //predict where you'll end up
                                crate::src::botlib::be_aas_bspq3::AAS_FloatForBSPEpairKey(
                                    dest,
                                    b"angle\x00" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                    &mut angle,
                                ); //end if
                                if angle != 0. {
                                    angles[0 as libc::c_int as usize] =
                                        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t; //end else
                                    angles[1 as libc::c_int as usize] = angle; //qtrue);
                                    angles[2 as libc::c_int as usize] =
                                        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t; //end if
                                    crate::src::qcommon::q_math::AngleVectors(
                                        angles.as_mut_ptr()
                                            as *const crate::src::qcommon::q_shared::vec_t,
                                        velocity.as_mut_ptr(),
                                        0 as *mut crate::src::qcommon::q_shared::vec_t,
                                        0 as *mut crate::src::qcommon::q_shared::vec_t,
                                    );
                                    velocity[0 as libc::c_int as usize] = velocity
                                        [0 as libc::c_int as usize]
                                        * 400 as libc::c_int as libc::c_float;
                                    velocity[1 as libc::c_int as usize] = velocity
                                        [1 as libc::c_int as usize]
                                        * 400 as libc::c_int as libc::c_float;
                                    velocity[2 as libc::c_int as usize] = velocity
                                        [2 as libc::c_int as usize]
                                        * 400 as libc::c_int as libc::c_float
                                } else {
                                    velocity[2 as libc::c_int as usize] =
                                        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
                                    velocity[1 as libc::c_int as usize] =
                                        velocity[2 as libc::c_int as usize];
                                    velocity[0 as libc::c_int as usize] =
                                        velocity[1 as libc::c_int as usize]
                                }
                                cmdmove[2 as libc::c_int as usize] =
                                    0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
                                cmdmove[1 as libc::c_int as usize] =
                                    cmdmove[2 as libc::c_int as usize];
                                cmdmove[0 as libc::c_int as usize] =
                                    cmdmove[1 as libc::c_int as usize];
                                crate::src::botlib::be_aas_move::AAS_PredictClientMovement(
                                    &mut move_0,
                                    -(1 as libc::c_int),
                                    destorigin.as_mut_ptr(),
                                    2 as libc::c_int,
                                    crate::src::qcommon::q_shared::qfalse as libc::c_int,
                                    velocity.as_mut_ptr(),
                                    cmdmove.as_mut_ptr(),
                                    0 as libc::c_int,
                                    30 as libc::c_int,
                                    0.1f32,
                                    1 as libc::c_int
                                        | 4 as libc::c_int
                                        | 8 as libc::c_int
                                        | 16 as libc::c_int
                                        | 32 as libc::c_int
                                        | 128 as libc::c_int
                                        | 256 as libc::c_int,
                                    0 as libc::c_int,
                                    crate::src::qcommon::q_shared::qfalse as libc::c_int,
                                );
                                area2num = crate::src::botlib::be_aas_sample::AAS_PointAreaNum(
                                    move_0.endpos.as_mut_ptr(),
                                );
                                if move_0.stopevent & (8 as libc::c_int | 16 as libc::c_int) != 0 {
                                    botimport.Print.expect("non-null function pointer")(
                                        2 as libc::c_int,
                                        b"teleported into slime or lava at dest %s\n\x00"
                                            as *const u8
                                            as *const libc::c_char
                                            as *mut libc::c_char,
                                        target.as_mut_ptr(),
                                    );
                                }
                                destorigin[0 as libc::c_int as usize] =
                                    move_0.endpos[0 as libc::c_int as usize];
                                destorigin[1 as libc::c_int as usize] =
                                    move_0.endpos[1 as libc::c_int as usize];
                                destorigin[2 as libc::c_int as usize] =
                                    move_0.endpos[2 as libc::c_int as usize];
                                current_block_61 = 7178192492338286402;
                            }
                        } else {
                            current_block_61 = 7178192492338286402;
                        }
                        match current_block_61 {
                            7351195479953500246 => {}
                            _ => {
                                //
                                //botimport.Print(PRT_MESSAGE, "teleporter brush origin at %f %f %f\n", origin[0], origin[1], origin[2]);
                                //botimport.Print(PRT_MESSAGE, "teleporter brush mins = %f %f %f\n", mins[0], mins[1], mins[2]);
                                //botimport.Print(PRT_MESSAGE, "teleporter brush maxs = %f %f %f\n", maxs[0], maxs[1], maxs[2]);
                                mins[0 as libc::c_int as usize] = origin[0 as libc::c_int as usize]
                                    + mins[0 as libc::c_int as usize];
                                mins[1 as libc::c_int as usize] = origin[1 as libc::c_int as usize]
                                    + mins[1 as libc::c_int as usize];
                                mins[2 as libc::c_int as usize] = origin[2 as libc::c_int as usize]
                                    + mins[2 as libc::c_int as usize];
                                maxs[0 as libc::c_int as usize] = origin[0 as libc::c_int as usize]
                                    + maxs[0 as libc::c_int as usize];
                                maxs[1 as libc::c_int as usize] = origin[1 as libc::c_int as usize]
                                    + maxs[1 as libc::c_int as usize];
                                maxs[2 as libc::c_int as usize] = origin[2 as libc::c_int as usize]
                                    + maxs[2 as libc::c_int as usize];
                                //
                                mid[0 as libc::c_int as usize] = mins[0 as libc::c_int as usize]
                                    + maxs[0 as libc::c_int as usize];
                                mid[1 as libc::c_int as usize] = mins[1 as libc::c_int as usize]
                                    + maxs[1 as libc::c_int as usize];
                                mid[2 as libc::c_int as usize] = mins[2 as libc::c_int as usize]
                                    + maxs[2 as libc::c_int as usize];
                                mid[0 as libc::c_int as usize] =
                                    (mid[0 as libc::c_int as usize] as libc::c_double * 0.5f64)
                                        as crate::src::qcommon::q_shared::vec_t;
                                mid[1 as libc::c_int as usize] =
                                    (mid[1 as libc::c_int as usize] as libc::c_double * 0.5f64)
                                        as crate::src::qcommon::q_shared::vec_t;
                                mid[2 as libc::c_int as usize] =
                                    (mid[2 as libc::c_int as usize] as libc::c_double * 0.5f64)
                                        as crate::src::qcommon::q_shared::vec_t;
                                //link an invalid (-1) entity
                                areas = crate::src::botlib::be_aas_sample::AAS_LinkEntityClientBBox(
                                    mins.as_mut_ptr(),
                                    maxs.as_mut_ptr(),
                                    -(1 as libc::c_int),
                                    4 as libc::c_int,
                                );
                                if areas.is_null() {
                                    botimport.Print.expect("non-null function pointer")(
                                        1 as libc::c_int,
                                        b"trigger_multiple not in any area\n\x00" as *const u8
                                            as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                }
                                //
                                link = areas; //end for
                                while !link.is_null() {
                                    //if (!AAS_AreaGrounded(link->areanum)) continue;
                                    if !(AAS_AreaTeleporter((*link).areanum) == 0) {
                                        //
                                        area1num = (*link).areanum;
                                        //create a new reachability link
                                        lreach = AAS_AllocReachability();
                                        if lreach.is_null() {
                                            break;
                                        }
                                        (*lreach).areanum = area2num;
                                        (*lreach).facenum = 0 as libc::c_int;
                                        (*lreach).edgenum = 0 as libc::c_int;
                                        (*lreach).start[0 as libc::c_int as usize] =
                                            mid[0 as libc::c_int as usize];
                                        (*lreach).start[1 as libc::c_int as usize] =
                                            mid[1 as libc::c_int as usize];
                                        (*lreach).start[2 as libc::c_int as usize] =
                                            mid[2 as libc::c_int as usize];
                                        (*lreach).end[0 as libc::c_int as usize] =
                                            destorigin[0 as libc::c_int as usize];
                                        (*lreach).end[1 as libc::c_int as usize] =
                                            destorigin[1 as libc::c_int as usize];
                                        (*lreach).end[2 as libc::c_int as usize] =
                                            destorigin[2 as libc::c_int as usize];
                                        (*lreach).traveltype = 10 as libc::c_int;
                                        (*lreach).traveltype |= AAS_TravelFlagsForTeam(ent);
                                        (*lreach).traveltime =
                                            crate::src::botlib::be_aas_move::aassettings.rs_teleport
                                                as libc::c_ushort;
                                        (*lreach).next =
                                            *areareachability.offset(area1num as isize);
                                        let ref mut fresh19 =
                                            *areareachability.offset(area1num as isize);
                                        *fresh19 = lreach;
                                        //
                                        reach_teleport += 1
                                    }
                                    link = (*link).next_area
                                }
                                //unlink the invalid entity
                                crate::src::botlib::be_aas_sample::AAS_UnlinkFromAreas(areas);
                            }
                        }
                    }
                }
            }
        }
        ent = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(ent)
    }
    //end for
}
//end of the function AAS_Reachability_Teleport
//===========================================================================
// create possible elevator (func_plat) reachabilities
// this is very game dependent.... :(
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Reachability_Elevator() {
    let mut area1num: libc::c_int = 0;
    let mut area2num: libc::c_int = 0;
    let mut modelnum: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut lip: libc::c_float = 0.;
    let mut height: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut model: [libc::c_char; 128] = [0; 128];
    let mut classname: [libc::c_char; 128] = [0; 128];
    let mut ent: libc::c_int = 0;
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut pos1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut pos2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut mids: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut platbottom: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut plattop: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut bottomorg: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut toporg: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut xvals: [crate::src::qcommon::q_shared::vec_t; 8] = [0.; 8];
    let mut yvals: [crate::src::qcommon::q_shared::vec_t; 8] = [0.; 8];
    let mut xvals_top: [crate::src::qcommon::q_shared::vec_t; 8] = [0.; 8];
    let mut yvals_top: [crate::src::qcommon::q_shared::vec_t; 8] = [0.; 8];
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut trace: crate::be_aas_h::aas_trace_t = crate::be_aas_h::aas_trace_t {
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        ent: 0,
        lastarea: 0,
        area: 0,
        planenum: 0,
    };
    //REACH_DEBUG
    ent = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(0 as libc::c_int);
    while ent != 0 {
        if !(crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
            ent,
            b"classname\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            classname.as_mut_ptr(),
            128 as libc::c_int,
        ) == 0)
        {
            if crate::stdlib::strcmp(
                classname.as_mut_ptr(),
                b"func_plat\x00" as *const u8 as *const libc::c_char,
            ) == 0
            {
                //REACH_DEBUG
                if crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
                    ent,
                    b"model\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    model.as_mut_ptr(),
                    128 as libc::c_int,
                ) == 0
                {
                    botimport.Print.expect("non-null function pointer")(
                        3 as libc::c_int,
                        b"func_plat without model\n\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ); //end if
                } else {
                    //get the model number, and skip the leading *
                    modelnum =
                        crate::stdlib::atoi(model.as_mut_ptr().offset(1 as libc::c_int as isize)); //end if
                    if modelnum <= 0 as libc::c_int {
                        botimport.Print.expect("non-null function pointer")(
                            3 as libc::c_int,
                            b"func_plat with invalid model number\n\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    } else {
                        //get the mins, maxs and origin of the model
                        //NOTE: the origin is usually (0,0,0) and the mins and maxs
                        //      are the absolute mins and maxs
                        crate::src::botlib::be_aas_bspq3::AAS_BSPModelMinsMaxsOrigin(
                            modelnum,
                            angles.as_mut_ptr(),
                            mins.as_mut_ptr(),
                            maxs.as_mut_ptr(),
                            origin.as_mut_ptr(),
                        );
                        //
                        crate::src::botlib::be_aas_bspq3::AAS_VectorForBSPEpairKey(
                            ent,
                            b"origin\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            origin.as_mut_ptr(),
                        );
                        //pos1 is the top position, pos2 is the bottom
                        pos1[0 as libc::c_int as usize] = origin[0 as libc::c_int as usize];
                        pos1[1 as libc::c_int as usize] = origin[1 as libc::c_int as usize];
                        pos1[2 as libc::c_int as usize] = origin[2 as libc::c_int as usize];
                        pos2[0 as libc::c_int as usize] = origin[0 as libc::c_int as usize];
                        pos2[1 as libc::c_int as usize] = origin[1 as libc::c_int as usize];
                        pos2[2 as libc::c_int as usize] = origin[2 as libc::c_int as usize];
                        //get the lip of the plat
                        crate::src::botlib::be_aas_bspq3::AAS_FloatForBSPEpairKey(
                            ent,
                            b"lip\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            &mut lip,
                        );
                        if lip == 0. {
                            lip = 8 as libc::c_int as libc::c_float
                        }
                        //get the movement height of the plat
                        crate::src::botlib::be_aas_bspq3::AAS_FloatForBSPEpairKey(
                            ent,
                            b"height\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            &mut height,
                        );
                        if height == 0. {
                            height = maxs[2 as libc::c_int as usize]
                                - mins[2 as libc::c_int as usize]
                                - lip
                        }
                        //get the speed of the plat
                        crate::src::botlib::be_aas_bspq3::AAS_FloatForBSPEpairKey(
                            ent,
                            b"speed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            &mut speed,
                        );
                        if speed == 0. {
                            speed = 200 as libc::c_int as libc::c_float
                        }
                        //get bottom position below pos1
                        pos2[2 as libc::c_int as usize] -= height;
                        //
                        //get a point just above the plat in the bottom position
                        mids[0 as libc::c_int as usize] =
                            mins[0 as libc::c_int as usize] + maxs[0 as libc::c_int as usize];
                        mids[1 as libc::c_int as usize] =
                            mins[1 as libc::c_int as usize] + maxs[1 as libc::c_int as usize];
                        mids[2 as libc::c_int as usize] =
                            mins[2 as libc::c_int as usize] + maxs[2 as libc::c_int as usize];
                        platbottom[0 as libc::c_int as usize] = (pos2[0 as libc::c_int as usize]
                            as libc::c_double
                            + mids[0 as libc::c_int as usize] as libc::c_double * 0.5f64)
                            as crate::src::qcommon::q_shared::vec_t;
                        platbottom[1 as libc::c_int as usize] = (pos2[1 as libc::c_int as usize]
                            as libc::c_double
                            + mids[1 as libc::c_int as usize] as libc::c_double * 0.5f64)
                            as crate::src::qcommon::q_shared::vec_t;
                        platbottom[2 as libc::c_int as usize] = (pos2[2 as libc::c_int as usize]
                            as libc::c_double
                            + mids[2 as libc::c_int as usize] as libc::c_double * 0.5f64)
                            as crate::src::qcommon::q_shared::vec_t;
                        platbottom[2 as libc::c_int as usize] = maxs[2 as libc::c_int as usize]
                            - (pos1[2 as libc::c_int as usize] - pos2[2 as libc::c_int as usize])
                            + 2 as libc::c_int as libc::c_float;
                        //get a point just above the plat in the top position
                        mids[0 as libc::c_int as usize] =
                            mins[0 as libc::c_int as usize] + maxs[0 as libc::c_int as usize];
                        mids[1 as libc::c_int as usize] =
                            mins[1 as libc::c_int as usize] + maxs[1 as libc::c_int as usize];
                        mids[2 as libc::c_int as usize] =
                            mins[2 as libc::c_int as usize] + maxs[2 as libc::c_int as usize];
                        plattop[0 as libc::c_int as usize] = (pos2[0 as libc::c_int as usize]
                            as libc::c_double
                            + mids[0 as libc::c_int as usize] as libc::c_double * 0.5f64)
                            as crate::src::qcommon::q_shared::vec_t;
                        plattop[1 as libc::c_int as usize] = (pos2[1 as libc::c_int as usize]
                            as libc::c_double
                            + mids[1 as libc::c_int as usize] as libc::c_double * 0.5f64)
                            as crate::src::qcommon::q_shared::vec_t;
                        plattop[2 as libc::c_int as usize] = (pos2[2 as libc::c_int as usize]
                            as libc::c_double
                            + mids[2 as libc::c_int as usize] as libc::c_double * 0.5f64)
                            as crate::src::qcommon::q_shared::vec_t;
                        plattop[2 as libc::c_int as usize] =
                            maxs[2 as libc::c_int as usize] + 2 as libc::c_int as libc::c_float;
                        //
                        /*if (!area1num)
                        {
                            Log_Write("no grounded area near plat bottom\r\n");
                            continue;
                        } //end if*/
                        //get the mins and maxs a little larger
                        i = 0 as libc::c_int; //end for
                        while i < 3 as libc::c_int {
                            mins[i as usize] -= 1 as libc::c_int as libc::c_float;
                            maxs[i as usize] += 1 as libc::c_int as libc::c_float;
                            i += 1
                        }
                        //
                        //botimport.Print(PRT_MESSAGE, "platbottom[2] = %1.1f plattop[2] = %1.1f\n", platbottom[2], plattop[2]);
                        //
                        mids[0 as libc::c_int as usize] =
                            mins[0 as libc::c_int as usize] + maxs[0 as libc::c_int as usize];
                        mids[1 as libc::c_int as usize] =
                            mins[1 as libc::c_int as usize] + maxs[1 as libc::c_int as usize];
                        mids[2 as libc::c_int as usize] =
                            mins[2 as libc::c_int as usize] + maxs[2 as libc::c_int as usize];
                        mids[0 as libc::c_int as usize] =
                            (mids[0 as libc::c_int as usize] as libc::c_double * 0.5f64)
                                as crate::src::qcommon::q_shared::vec_t;
                        mids[1 as libc::c_int as usize] =
                            (mids[1 as libc::c_int as usize] as libc::c_double * 0.5f64)
                                as crate::src::qcommon::q_shared::vec_t;
                        mids[2 as libc::c_int as usize] =
                            (mids[2 as libc::c_int as usize] as libc::c_double * 0.5f64)
                                as crate::src::qcommon::q_shared::vec_t;
                        //
                        xvals[0 as libc::c_int as usize] = mins[0 as libc::c_int as usize];
                        xvals[1 as libc::c_int as usize] = mids[0 as libc::c_int as usize];
                        xvals[2 as libc::c_int as usize] = maxs[0 as libc::c_int as usize];
                        xvals[3 as libc::c_int as usize] = mids[0 as libc::c_int as usize];
                        yvals[0 as libc::c_int as usize] = mids[1 as libc::c_int as usize];
                        yvals[1 as libc::c_int as usize] = maxs[1 as libc::c_int as usize];
                        yvals[2 as libc::c_int as usize] = mids[1 as libc::c_int as usize];
                        yvals[3 as libc::c_int as usize] = mins[1 as libc::c_int as usize];
                        //
                        xvals[4 as libc::c_int as usize] = mins[0 as libc::c_int as usize];
                        xvals[5 as libc::c_int as usize] = maxs[0 as libc::c_int as usize];
                        xvals[6 as libc::c_int as usize] = maxs[0 as libc::c_int as usize];
                        xvals[7 as libc::c_int as usize] = mins[0 as libc::c_int as usize];
                        yvals[4 as libc::c_int as usize] = maxs[1 as libc::c_int as usize];
                        yvals[5 as libc::c_int as usize] = maxs[1 as libc::c_int as usize];
                        yvals[6 as libc::c_int as usize] = mins[1 as libc::c_int as usize];
                        yvals[7 as libc::c_int as usize] = mins[1 as libc::c_int as usize];
                        let mut current_block_119: u64;
                        //find adjacent areas around the bottom of the plat
                        i = 0 as libc::c_int; //end else
                        while i < 9 as libc::c_int {
                            if i < 8 as libc::c_int {
                                //check at the sides of the plat
                                bottomorg[0 as libc::c_int as usize] =
                                    origin[0 as libc::c_int as usize] + xvals[i as usize];
                                bottomorg[1 as libc::c_int as usize] =
                                    origin[1 as libc::c_int as usize] + yvals[i as usize]; //end if
                                bottomorg[2 as libc::c_int as usize] = platbottom
                                    [2 as libc::c_int as usize]
                                    + 16 as libc::c_int as libc::c_float;
                                //end if
                                //get a grounded or swim area near the plat in the bottom position
                                area1num = crate::src::botlib::be_aas_sample::AAS_PointAreaNum(
                                    bottomorg.as_mut_ptr(),
                                ); //end if
                                k = 0 as libc::c_int; //end if
                                while k < 16 as libc::c_int {
                                    if area1num != 0 {
                                        if AAS_AreaGrounded(area1num) != 0
                                            || AAS_AreaSwim(area1num) != 0
                                        {
                                            break;
                                        }
                                    }
                                    bottomorg[2 as libc::c_int as usize] +=
                                        4 as libc::c_int as libc::c_float;
                                    area1num = crate::src::botlib::be_aas_sample::AAS_PointAreaNum(
                                        bottomorg.as_mut_ptr(),
                                    );
                                    k += 1
                                }
                                if k >= 16 as libc::c_int {
                                    current_block_119 = 5892776923941496671;
                                } else {
                                    current_block_119 = 7385833325316299293;
                                }
                            } else {
                                //if in solid
                                //at the middle of the plat
                                bottomorg[0 as libc::c_int as usize] =
                                    plattop[0 as libc::c_int as usize];
                                bottomorg[1 as libc::c_int as usize] =
                                    plattop[1 as libc::c_int as usize];
                                bottomorg[2 as libc::c_int as usize] =
                                    plattop[2 as libc::c_int as usize];
                                bottomorg[2 as libc::c_int as usize] +=
                                    24 as libc::c_int as libc::c_float;
                                area1num = crate::src::botlib::be_aas_sample::AAS_PointAreaNum(
                                    bottomorg.as_mut_ptr(),
                                );
                                if area1num == 0 {
                                    current_block_119 = 5892776923941496671;
                                } else {
                                    bottomorg[0 as libc::c_int as usize] =
                                        platbottom[0 as libc::c_int as usize];
                                    bottomorg[1 as libc::c_int as usize] =
                                        platbottom[1 as libc::c_int as usize];
                                    bottomorg[2 as libc::c_int as usize] =
                                        platbottom[2 as libc::c_int as usize];
                                    bottomorg[2 as libc::c_int as usize] +=
                                        24 as libc::c_int as libc::c_float;
                                    current_block_119 = 7385833325316299293;
                                }
                            }
                            match current_block_119 {
                                7385833325316299293 => {
                                    //look at adjacent areas around the top of the plat
                                    //make larger steps to outside the plat every time
                                    n = 0 as libc::c_int; //end for
                                    while n < 3 as libc::c_int {
                                        k = 0 as libc::c_int;
                                        while k < 3 as libc::c_int {
                                            mins[k as usize] -= 4 as libc::c_int as libc::c_float;
                                            maxs[k as usize] += 4 as libc::c_int as libc::c_float;
                                            k += 1
                                        }
                                        xvals_top[0 as libc::c_int as usize] =
                                            mins[0 as libc::c_int as usize];
                                        xvals_top[1 as libc::c_int as usize] =
                                            mids[0 as libc::c_int as usize];
                                        xvals_top[2 as libc::c_int as usize] =
                                            maxs[0 as libc::c_int as usize];
                                        xvals_top[3 as libc::c_int as usize] =
                                            mids[0 as libc::c_int as usize];
                                        yvals_top[0 as libc::c_int as usize] =
                                            mids[1 as libc::c_int as usize];
                                        yvals_top[1 as libc::c_int as usize] =
                                            maxs[1 as libc::c_int as usize];
                                        yvals_top[2 as libc::c_int as usize] =
                                            mids[1 as libc::c_int as usize];
                                        yvals_top[3 as libc::c_int as usize] =
                                            mins[1 as libc::c_int as usize];
                                        //end for
                                        xvals_top[4 as libc::c_int as usize] =
                                            mins[0 as libc::c_int as usize];
                                        xvals_top[5 as libc::c_int as usize] =
                                            maxs[0 as libc::c_int as usize];
                                        xvals_top[6 as libc::c_int as usize] =
                                            maxs[0 as libc::c_int as usize];
                                        xvals_top[7 as libc::c_int as usize] =
                                            mins[0 as libc::c_int as usize];
                                        yvals_top[4 as libc::c_int as usize] =
                                            maxs[1 as libc::c_int as usize];
                                        yvals_top[5 as libc::c_int as usize] =
                                            maxs[1 as libc::c_int as usize];
                                        yvals_top[6 as libc::c_int as usize] =
                                            mins[1 as libc::c_int as usize];
                                        yvals_top[7 as libc::c_int as usize] =
                                            mins[1 as libc::c_int as usize];
                                        j = 0 as libc::c_int;
                                        while j < 8 as libc::c_int {
                                            toporg[0 as libc::c_int as usize] = origin
                                                [0 as libc::c_int as usize]
                                                + xvals_top[j as usize];
                                            toporg[1 as libc::c_int as usize] = origin
                                                [1 as libc::c_int as usize]
                                                + yvals_top[j as usize];
                                            toporg[2 as libc::c_int as usize] = plattop
                                                [2 as libc::c_int as usize]
                                                + 16 as libc::c_int as libc::c_float;
                                            //
                                            //
                                            //get a grounded or swim area near the plat in the top position
                                            area2num =
                                                crate::src::botlib::be_aas_sample::AAS_PointAreaNum(
                                                    toporg.as_mut_ptr(),
                                                ); //end if
                                            l = 0 as libc::c_int; //end if
                                            while l < 16 as libc::c_int {
                                                if area2num != 0 {
                                                    if AAS_AreaGrounded(area2num) != 0
                                                        || AAS_AreaSwim(area2num) != 0
                                                    {
                                                        start[0 as libc::c_int as usize] =
                                                            plattop[0 as libc::c_int as usize];
                                                        start[1 as libc::c_int as usize] =
                                                            plattop[1 as libc::c_int as usize];
                                                        start[2 as libc::c_int as usize] =
                                                            plattop[2 as libc::c_int as usize];
                                                        start[2 as libc::c_int as usize] +=
                                                            32 as libc::c_int as libc::c_float;
                                                        end[0 as libc::c_int as usize] =
                                                            toporg[0 as libc::c_int as usize];
                                                        end[1 as libc::c_int as usize] =
                                                            toporg[1 as libc::c_int as usize];
                                                        end[2 as libc::c_int as usize] =
                                                            toporg[2 as libc::c_int as usize];
                                                        end[2 as libc::c_int as usize] +=
                                                            1 as libc::c_int as libc::c_float;
                                                        trace =
                                                            crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(start.as_mut_ptr(),
                                                                                end.as_mut_ptr(),
                                                                                4
                                                                                    as
                                                                                    libc::c_int,
                                                                                -(1
                                                                                      as
                                                                                      libc::c_int));
                                                        if trace.fraction
                                                            >= 1 as libc::c_int as libc::c_float
                                                        {
                                                            break;
                                                        }
                                                    }
                                                    //end if
                                                }
                                                toporg[2 as libc::c_int as usize] +=
                                                    4 as libc::c_int as libc::c_float;
                                                area2num =
                                                    crate::src::botlib::be_aas_sample::AAS_PointAreaNum(toporg.as_mut_ptr());
                                                l += 1
                                            }
                                            //if in solid
                                            if !(l >= 16 as libc::c_int) {
                                                //never create a reachability in the same area
                                                if !(area2num == area1num) {
                                                    //if the area isn't grounded
                                                    if !(AAS_AreaGrounded(area2num) == 0) {
                                                        //if there already exists reachability between the areas
                                                        if !(AAS_ReachabilityExists(
                                                            area1num, area2num,
                                                        )
                                                            as u64
                                                            != 0)
                                                        {
                                                            //if the reachability start is within the elevator bounding box
                                                            dir[0 as libc::c_int as usize] =
                                                                bottomorg
                                                                    [0 as libc::c_int as usize]
                                                                    - platbottom
                                                                        [0 as libc::c_int as usize];
                                                            dir[1 as libc::c_int as usize] =
                                                                bottomorg
                                                                    [1 as libc::c_int as usize]
                                                                    - platbottom
                                                                        [1 as libc::c_int as usize];
                                                            dir[2 as libc::c_int as usize] =
                                                                bottomorg
                                                                    [2 as libc::c_int as usize]
                                                                    - platbottom
                                                                        [2 as libc::c_int as usize];
                                                            crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
                                                            dir[0 as libc::c_int as usize] =
                                                                bottomorg
                                                                    [0 as libc::c_int as usize]
                                                                    + 24 as libc::c_int
                                                                        as libc::c_float
                                                                        * dir[0 as libc::c_int
                                                                            as usize];
                                                            dir[1 as libc::c_int as usize] =
                                                                bottomorg
                                                                    [1 as libc::c_int as usize]
                                                                    + 24 as libc::c_int
                                                                        as libc::c_float
                                                                        * dir[1 as libc::c_int
                                                                            as usize];
                                                            dir[2 as libc::c_int as usize] =
                                                                bottomorg
                                                                    [2 as libc::c_int as usize];
                                                            //
                                                            p = 0 as libc::c_int;
                                                            while p < 3 as libc::c_int {
                                                                if dir[p as usize]
                                                                    < origin[p as usize]
                                                                        + mins[p as usize]
                                                                    || dir[p as usize]
                                                                        > origin[p as usize]
                                                                            + maxs[p as usize]
                                                                {
                                                                    break;
                                                                }
                                                                p += 1
                                                            }
                                                            if !(p >= 3 as libc::c_int) {
                                                                //create a new reachability link
                                                                lreach = AAS_AllocReachability();
                                                                if !lreach.is_null() {
                                                                    (*lreach).areanum = area2num;
                                                                    //the facenum is the model number
                                                                    (*lreach).facenum = modelnum;
                                                                    //the edgenum is the height
                                                                    (*lreach).edgenum =
                                                                        height as libc::c_int;
                                                                    //
                                                                    (*lreach).start[0 as libc::c_int
                                                                        as usize] = dir
                                                                        [0 as libc::c_int as usize];
                                                                    (*lreach).start[1 as libc::c_int
                                                                        as usize] = dir
                                                                        [1 as libc::c_int as usize];
                                                                    (*lreach).start[2 as libc::c_int
                                                                        as usize] = dir
                                                                        [2 as libc::c_int as usize];
                                                                    (*lreach).end[0 as libc::c_int
                                                                        as usize] = toporg
                                                                        [0 as libc::c_int as usize];
                                                                    (*lreach).end[1 as libc::c_int
                                                                        as usize] = toporg
                                                                        [1 as libc::c_int as usize];
                                                                    (*lreach).end[2 as libc::c_int
                                                                        as usize] = toporg
                                                                        [2 as libc::c_int as usize];
                                                                    (*lreach).traveltype =
                                                                        11 as libc::c_int;
                                                                    (*lreach).traveltype |=
                                                                        AAS_TravelFlagsForTeam(ent);
                                                                    (*lreach).traveltime
                                                                        =
                                                                        (crate::src::botlib::be_aas_move::aassettings.rs_startelevator
                                                                             +
                                                                             height
                                                                                 *
                                                                                 100
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_float
                                                                                 /
                                                                                 speed)
                                                                            as
                                                                            libc::c_ushort;
                                                                    (*lreach).next =
                                                                        *areareachability.offset(
                                                                            area1num as isize,
                                                                        );
                                                                    let ref mut fresh20 =
                                                                        *areareachability.offset(
                                                                            area1num as isize,
                                                                        );
                                                                    *fresh20 = lreach;
                                                                    //don't go any further to the outside
                                                                    n = 9999 as libc::c_int;
                                                                    //
                                                                    //REACH_DEBUG
                                                                    //
                                                                    reach_elevator += 1
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            j += 1
                                        }
                                        n += 1
                                    }
                                }
                                _ => {}
                            }
                            i += 1
                            //end for
                        }
                    }
                }
                //end for
            }
        }
        ent = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(ent)
        //end if
    }
    //end for
}
//end of the function AAS_Reachability_Elevator
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_FindFaceReachabilities(
    mut facepoints: *mut crate::src::qcommon::q_shared::vec3_t,
    mut numpoints: libc::c_int,
    mut plane: *mut crate::aasfile_h::aas_plane_t,
    mut towardsface: libc::c_int,
) -> *mut aas_lreachability_t {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut facenum: libc::c_int = 0;
    let mut edgenum: libc::c_int = 0;
    let mut bestfacenum: libc::c_int = 0;
    let mut v1: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut v2: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut v3: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut v4: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut bestdist: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut hordist: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    let mut beststart: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0.,
        0.,
    ];
    let mut beststart2: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0.,
        0.,
    ];
    let mut bestend: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0.,
        0.,
    ];
    let mut bestend2: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0.,
        0.,
    ];
    let mut tmp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut testpoint: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut lreachabilities: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut area: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut face: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut edge: *mut crate::aasfile_h::aas_edge_t = 0 as *mut crate::aasfile_h::aas_edge_t;
    let mut faceplane: *mut crate::aasfile_h::aas_plane_t = 0 as *mut crate::aasfile_h::aas_plane_t;
    let mut bestfaceplane: *mut crate::aasfile_h::aas_plane_t =
        0 as *mut crate::aasfile_h::aas_plane_t;
    //
    lreachabilities = 0 as *mut aas_lreachability_t;
    bestfacenum = 0 as libc::c_int;
    bestfaceplane = 0 as *mut crate::aasfile_h::aas_plane_t;
    let mut current_block_61: u64;
    //
    i = 1 as libc::c_int; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        area = &mut *crate::src::botlib::be_aas_main::aasworld
            .areas
            .offset(i as isize) as *mut crate::aasfile_h::aas_area_t;
        // get the shortest distance between one of the func_bob start edges and
        // one of the face edges of area1
        bestdist = 999999 as libc::c_int as libc::c_float; //end for
        j = 0 as libc::c_int;
        while j < (*area).numfaces {
            facenum = *crate::src::botlib::be_aas_main::aasworld
                .faceindex
                .offset(((*area).firstface + j) as isize);
            face = &mut *crate::src::botlib::be_aas_main::aasworld
                .faces
                .offset(
                    (crate::stdlib::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(
                        facenum,
                    ) as isize,
                ) as *mut crate::aasfile_h::aas_face_t;
            //end for
            //if not a ground face
            if !((*face).faceflags & 4 as libc::c_int == 0) {
                //get the ground planes
                faceplane = &mut *crate::src::botlib::be_aas_main::aasworld
                    .planes
                    .offset((*face).planenum as isize)
                    as *mut crate::aasfile_h::aas_plane_t;
                //
                k = 0 as libc::c_int;
                while k < (*face).numedges {
                    edgenum = crate::stdlib::abs(
                        *crate::src::botlib::be_aas_main::aasworld
                            .edgeindex
                            .offset(((*face).firstedge + k) as isize),
                    );
                    edge = &mut *crate::src::botlib::be_aas_main::aasworld
                        .edges
                        .offset(edgenum as isize)
                        as *mut crate::aasfile_h::aas_edge_t;
                    //end for
                    v1 = (*crate::src::botlib::be_aas_main::aasworld
                        .vertexes
                        .offset((*edge).v[0 as libc::c_int as usize] as isize))
                    .as_mut_ptr();
                    v2 = (*crate::src::botlib::be_aas_main::aasworld
                        .vertexes
                        .offset((*edge).v[1 as libc::c_int as usize] as isize))
                    .as_mut_ptr();
                    l = 0 as libc::c_int;
                    while l < numpoints {
                        v3 = (*facepoints.offset(l as isize)).as_mut_ptr();
                        v4 = (*facepoints.offset(((l + 1 as libc::c_int) % numpoints) as isize))
                            .as_mut_ptr();
                        dist = AAS_ClosestEdgePoints(
                            v1,
                            v2,
                            v3,
                            v4,
                            faceplane,
                            plane,
                            beststart.as_mut_ptr(),
                            bestend.as_mut_ptr(),
                            beststart2.as_mut_ptr(),
                            bestend2.as_mut_ptr(),
                            bestdist,
                        );
                        if dist < bestdist {
                            bestfacenum = facenum;
                            bestfaceplane = faceplane;
                            bestdist = dist
                        }
                        l += 1
                        //calculate the minimum distance between the two edges
                        //
                        //end if
                    }
                    k += 1
                }
            }
            j += 1
        }
        //
        if !(bestdist > 192 as libc::c_int as libc::c_float) {
            //
            VectorMiddle(
                beststart.as_mut_ptr(),
                beststart2.as_mut_ptr(),
                beststart.as_mut_ptr(),
            );
            VectorMiddle(
                bestend.as_mut_ptr(),
                bestend2.as_mut_ptr(),
                bestend.as_mut_ptr(),
            );
            //
            if towardsface == 0 {
                tmp[0 as libc::c_int as usize] = beststart[0 as libc::c_int as usize]; //end if
                tmp[1 as libc::c_int as usize] = beststart[1 as libc::c_int as usize];
                tmp[2 as libc::c_int as usize] = beststart[2 as libc::c_int as usize];
                beststart[0 as libc::c_int as usize] = bestend[0 as libc::c_int as usize];
                beststart[1 as libc::c_int as usize] = bestend[1 as libc::c_int as usize];
                beststart[2 as libc::c_int as usize] = bestend[2 as libc::c_int as usize];
                bestend[0 as libc::c_int as usize] = tmp[0 as libc::c_int as usize];
                bestend[1 as libc::c_int as usize] = tmp[1 as libc::c_int as usize];
                bestend[2 as libc::c_int as usize] = tmp[2 as libc::c_int as usize]
            }
            //
            hordir[0 as libc::c_int as usize] =
                bestend[0 as libc::c_int as usize] - beststart[0 as libc::c_int as usize];
            hordir[1 as libc::c_int as usize] =
                bestend[1 as libc::c_int as usize] - beststart[1 as libc::c_int as usize];
            hordir[2 as libc::c_int as usize] =
                bestend[2 as libc::c_int as usize] - beststart[2 as libc::c_int as usize];
            hordir[2 as libc::c_int as usize] =
                0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
            hordist =
                VectorLength(hordir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
            //
            if !(hordist
                > 2 as libc::c_int as libc::c_float
                    * AAS_MaxJumpDistance(
                        crate::src::botlib::be_aas_move::aassettings.phys_jumpvel,
                    ))
            {
                //the end point should not be significantly higher than the start point
                if !(bestend[2 as libc::c_int as usize] - 32 as libc::c_int as libc::c_float
                    > beststart[2 as libc::c_int as usize])
                {
                    //don't fall down too far
                    if !(bestend[2 as libc::c_int as usize]
                        < beststart[2 as libc::c_int as usize]
                            - 128 as libc::c_int as libc::c_float)
                    {
                        //the distance should not be too far
                        if hordist > 32 as libc::c_int as libc::c_float {
                            //end if
                            //check for walk off ledge
                            if crate::src::botlib::be_aas_move::AAS_HorizontalVelocityForJump(
                                0 as libc::c_int as libc::c_float,
                                beststart.as_mut_ptr(),
                                bestend.as_mut_ptr(),
                                &mut speed,
                            ) == 0
                            {
                                current_block_61 = 3276175668257526147;
                            } else {
                                current_block_61 = 11793792312832361944;
                            }
                        } else {
                            current_block_61 = 11793792312832361944;
                        }
                        match current_block_61 {
                            3276175668257526147 => {}
                            _ => {
                                //
                                beststart[2 as libc::c_int as usize] +=
                                    1 as libc::c_int as libc::c_float;
                                bestend[2 as libc::c_int as usize] +=
                                    1 as libc::c_int as libc::c_float;
                                //
                                if towardsface != 0 {
                                    testpoint[0 as libc::c_int as usize] =
                                        bestend[0 as libc::c_int as usize];
                                    testpoint[1 as libc::c_int as usize] =
                                        bestend[1 as libc::c_int as usize];
                                    testpoint[2 as libc::c_int as usize] =
                                        bestend[2 as libc::c_int as usize]
                                } else {
                                    testpoint[0 as libc::c_int as usize] =
                                        beststart[0 as libc::c_int as usize];
                                    testpoint[1 as libc::c_int as usize] =
                                        beststart[1 as libc::c_int as usize];
                                    testpoint[2 as libc::c_int as usize] =
                                        beststart[2 as libc::c_int as usize]
                                }
                                if !bestfaceplane.is_null() {
                                    testpoint[2 as libc::c_int as usize] = ((*bestfaceplane).dist
                                        - ((*bestfaceplane).normal[0 as libc::c_int as usize]
                                            * testpoint[0 as libc::c_int as usize]
                                            + (*bestfaceplane).normal[1 as libc::c_int as usize]
                                                * testpoint[1 as libc::c_int as usize]
                                            + (*bestfaceplane).normal[2 as libc::c_int as usize]
                                                * testpoint[2 as libc::c_int as usize]))
                                        / (*bestfaceplane).normal[2 as libc::c_int as usize]
                                } else {
                                    testpoint[2 as libc::c_int as usize] =
                                        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t
                                }
                                //
                                if crate::src::botlib::be_aas_sample::AAS_PointInsideFace(
                                    bestfacenum,
                                    testpoint.as_mut_ptr(),
                                    0.1f32,
                                ) as u64
                                    == 0
                                {
                                    //end if
                                    //if the faces are not overlapping then only go down
                                    if bestend[2 as libc::c_int as usize]
                                        - 16 as libc::c_int as libc::c_float
                                        > beststart[2 as libc::c_int as usize]
                                    {
                                        current_block_61 = 3276175668257526147;
                                    } else {
                                        current_block_61 = 6545907279487748450;
                                    }
                                } else {
                                    current_block_61 = 6545907279487748450;
                                }
                                match current_block_61 {
                                    3276175668257526147 => {}
                                    _ => {
                                        lreach = AAS_AllocReachability();
                                        if lreach.is_null() {
                                            return lreachabilities;
                                        }
                                        (*lreach).areanum = i;
                                        (*lreach).facenum = 0 as libc::c_int;
                                        (*lreach).edgenum = 0 as libc::c_int;
                                        (*lreach).start[0 as libc::c_int as usize] =
                                            beststart[0 as libc::c_int as usize];
                                        (*lreach).start[1 as libc::c_int as usize] =
                                            beststart[1 as libc::c_int as usize];
                                        (*lreach).start[2 as libc::c_int as usize] =
                                            beststart[2 as libc::c_int as usize];
                                        (*lreach).end[0 as libc::c_int as usize] =
                                            bestend[0 as libc::c_int as usize];
                                        (*lreach).end[1 as libc::c_int as usize] =
                                            bestend[1 as libc::c_int as usize];
                                        (*lreach).end[2 as libc::c_int as usize] =
                                            bestend[2 as libc::c_int as usize];
                                        (*lreach).traveltype = 0 as libc::c_int;
                                        (*lreach).traveltime = 0 as libc::c_int as libc::c_ushort;
                                        (*lreach).next = lreachabilities;
                                        lreachabilities = lreach;
                                        if towardsface != 0 {
                                            crate::src::botlib::be_aas_debug::AAS_PermanentLine(
                                                (*lreach).start.as_mut_ptr(),
                                                (*lreach).end.as_mut_ptr(),
                                                1 as libc::c_int,
                                            );
                                        } else {
                                            crate::src::botlib::be_aas_debug::AAS_PermanentLine(
                                                (*lreach).start.as_mut_ptr(),
                                                (*lreach).end.as_mut_ptr(),
                                                2 as libc::c_int,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    return lreachabilities;
}
//end of the function AAS_FindFaceReachabilities
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Reachability_FuncBobbing() {
    let mut ent: libc::c_int = 0;
    let mut spawnflags: libc::c_int = 0;
    let mut modelnum: libc::c_int = 0;
    let mut axis: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut numareas: libc::c_int = 0;
    let mut areas: [libc::c_int; 10] = [0; 10];
    let mut classname: [libc::c_char; 128] = [0; 128];
    let mut model: [libc::c_char; 128] = [0; 128];
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut move_end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut move_start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut move_start_top: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut move_end_top: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut start_edgeverts: [crate::src::qcommon::q_shared::vec3_t; 4] = [[0.; 3]; 4];
    let mut end_edgeverts: [crate::src::qcommon::q_shared::vec3_t; 4] = [[0.; 3]; 4];
    let mut mid: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut org: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut points: [crate::src::qcommon::q_shared::vec3_t; 10] = [[0.; 3]; 10];
    let mut height: libc::c_float = 0.;
    let mut start_plane: crate::aasfile_h::aas_plane_t = crate::aasfile_h::aas_plane_t {
        normal: [0.; 3],
        dist: 0.,
        type_0: 0,
    };
    let mut end_plane: crate::aasfile_h::aas_plane_t = crate::aasfile_h::aas_plane_t {
        normal: [0.; 3],
        dist: 0.,
        type_0: 0,
    };
    let mut startreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut endreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut nextstartreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut nextendreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut firststartreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut firstendreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    ent = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(0 as libc::c_int);
    while ent != 0 {
        if !(crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
            ent,
            b"classname\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            classname.as_mut_ptr(),
            128 as libc::c_int,
        ) == 0)
        {
            if !(crate::stdlib::strcmp(
                classname.as_mut_ptr(),
                b"func_bobbing\x00" as *const u8 as *const libc::c_char,
            ) != 0)
            {
                crate::src::botlib::be_aas_bspq3::AAS_FloatForBSPEpairKey(
                    ent,
                    b"height\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    &mut height,
                );
                if height == 0. {
                    height = 32 as libc::c_int as libc::c_float
                }
                //
                if crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
                    ent,
                    b"model\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    model.as_mut_ptr(),
                    128 as libc::c_int,
                ) == 0
                {
                    botimport.Print.expect("non-null function pointer")(
                        3 as libc::c_int,
                        b"func_bobbing without model\n\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ); //end if
                } else {
                    //get the model number, and skip the leading *
                    modelnum =
                        crate::stdlib::atoi(model.as_mut_ptr().offset(1 as libc::c_int as isize)); //end if
                    if modelnum <= 0 as libc::c_int {
                        botimport.Print.expect("non-null function pointer")(
                            3 as libc::c_int,
                            b"func_bobbing with invalid model number\n\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    } else {
                        //if the entity has an origin set then use it
                        if crate::src::botlib::be_aas_bspq3::AAS_VectorForBSPEpairKey(
                            ent,
                            b"origin\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            origin.as_mut_ptr(),
                        ) == 0
                        {
                            origin[0 as libc::c_int as usize] =
                                0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
                            origin[1 as libc::c_int as usize] =
                                0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
                            origin[2 as libc::c_int as usize] =
                                0 as libc::c_int as crate::src::qcommon::q_shared::vec_t
                        }
                        //
                        crate::src::botlib::be_aas_bspq3::AAS_BSPModelMinsMaxsOrigin(
                            modelnum,
                            angles.as_mut_ptr(),
                            mins.as_mut_ptr(),
                            maxs.as_mut_ptr(),
                            0 as *mut crate::src::qcommon::q_shared::vec_t,
                        );
                        //
                        mins[0 as libc::c_int as usize] =
                            mins[0 as libc::c_int as usize] + origin[0 as libc::c_int as usize];
                        mins[1 as libc::c_int as usize] =
                            mins[1 as libc::c_int as usize] + origin[1 as libc::c_int as usize];
                        mins[2 as libc::c_int as usize] =
                            mins[2 as libc::c_int as usize] + origin[2 as libc::c_int as usize];
                        maxs[0 as libc::c_int as usize] =
                            maxs[0 as libc::c_int as usize] + origin[0 as libc::c_int as usize];
                        maxs[1 as libc::c_int as usize] =
                            maxs[1 as libc::c_int as usize] + origin[1 as libc::c_int as usize];
                        maxs[2 as libc::c_int as usize] =
                            maxs[2 as libc::c_int as usize] + origin[2 as libc::c_int as usize];
                        //
                        mid[0 as libc::c_int as usize] =
                            mins[0 as libc::c_int as usize] + maxs[0 as libc::c_int as usize];
                        mid[1 as libc::c_int as usize] =
                            mins[1 as libc::c_int as usize] + maxs[1 as libc::c_int as usize];
                        mid[2 as libc::c_int as usize] =
                            mins[2 as libc::c_int as usize] + maxs[2 as libc::c_int as usize];
                        mid[0 as libc::c_int as usize] =
                            (mid[0 as libc::c_int as usize] as libc::c_double * 0.5f64)
                                as crate::src::qcommon::q_shared::vec_t;
                        mid[1 as libc::c_int as usize] =
                            (mid[1 as libc::c_int as usize] as libc::c_double * 0.5f64)
                                as crate::src::qcommon::q_shared::vec_t;
                        mid[2 as libc::c_int as usize] =
                            (mid[2 as libc::c_int as usize] as libc::c_double * 0.5f64)
                                as crate::src::qcommon::q_shared::vec_t;
                        origin[0 as libc::c_int as usize] = mid[0 as libc::c_int as usize];
                        origin[1 as libc::c_int as usize] = mid[1 as libc::c_int as usize];
                        origin[2 as libc::c_int as usize] = mid[2 as libc::c_int as usize];
                        //
                        move_end[0 as libc::c_int as usize] = origin[0 as libc::c_int as usize];
                        move_end[1 as libc::c_int as usize] = origin[1 as libc::c_int as usize];
                        move_end[2 as libc::c_int as usize] = origin[2 as libc::c_int as usize];
                        move_start[0 as libc::c_int as usize] = origin[0 as libc::c_int as usize];
                        move_start[1 as libc::c_int as usize] = origin[1 as libc::c_int as usize];
                        move_start[2 as libc::c_int as usize] = origin[2 as libc::c_int as usize];
                        //
                        crate::src::botlib::be_aas_bspq3::AAS_IntForBSPEpairKey(
                            ent,
                            b"spawnflags\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            &mut spawnflags,
                        );
                        // set the axis of bobbing
                        if spawnflags & 1 as libc::c_int != 0 {
                            axis = 0 as libc::c_int
                        } else if spawnflags & 2 as libc::c_int != 0 {
                            axis = 1 as libc::c_int
                        } else {
                            axis = 2 as libc::c_int
                        }
                        //
                        move_start[axis as usize] -= height;
                        move_end[axis as usize] += height;
                        //
                        crate::src::botlib::l_log::Log_Write(b"funcbob model %d, start = {%1.1f, %1.1f, %1.1f} end = {%1.1f, %1.1f, %1.1f}\n\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char, modelnum,
                                  move_start[0 as libc::c_int as usize] as
                                      libc::c_double,
                                  move_start[1 as libc::c_int as usize] as
                                      libc::c_double,
                                  move_start[2 as libc::c_int as usize] as
                                      libc::c_double,
                                  move_end[0 as libc::c_int as usize] as
                                      libc::c_double,
                                  move_end[1 as libc::c_int as usize] as
                                      libc::c_double,
                                  move_end[2 as libc::c_int as usize] as
                                      libc::c_double);
                        //
                        /*
                        AAS_DrawPermanentCross(move_start, 4, 1);
                        AAS_DrawPermanentCross(move_end, 4, 2);
                        */
                        //
                        i = 0 as libc::c_int; //end for
                        while i < 4 as libc::c_int {
                            start_edgeverts[i as usize][0 as libc::c_int as usize] =
                                move_start[0 as libc::c_int as usize];
                            start_edgeverts[i as usize][1 as libc::c_int as usize] =
                                move_start[1 as libc::c_int as usize];
                            start_edgeverts[i as usize][2 as libc::c_int as usize] =
                                move_start[2 as libc::c_int as usize];
                            //+ player origin to ground dist
                            start_edgeverts[i as usize][2 as libc::c_int as usize] +=
                                maxs[2 as libc::c_int as usize] - mid[2 as libc::c_int as usize]; //+ bbox maxs z
                            start_edgeverts[i as usize][2 as libc::c_int as usize] +=
                                24 as libc::c_int as libc::c_float;
                            i += 1
                        }
                        start_edgeverts[0 as libc::c_int as usize][0 as libc::c_int as usize] +=
                            maxs[0 as libc::c_int as usize] - mid[0 as libc::c_int as usize];
                        start_edgeverts[0 as libc::c_int as usize][1 as libc::c_int as usize] +=
                            maxs[1 as libc::c_int as usize] - mid[1 as libc::c_int as usize];
                        start_edgeverts[1 as libc::c_int as usize][0 as libc::c_int as usize] +=
                            maxs[0 as libc::c_int as usize] - mid[0 as libc::c_int as usize];
                        start_edgeverts[1 as libc::c_int as usize][1 as libc::c_int as usize] +=
                            mins[1 as libc::c_int as usize] - mid[1 as libc::c_int as usize];
                        start_edgeverts[2 as libc::c_int as usize][0 as libc::c_int as usize] +=
                            mins[0 as libc::c_int as usize] - mid[0 as libc::c_int as usize];
                        start_edgeverts[2 as libc::c_int as usize][1 as libc::c_int as usize] +=
                            mins[1 as libc::c_int as usize] - mid[1 as libc::c_int as usize];
                        start_edgeverts[3 as libc::c_int as usize][0 as libc::c_int as usize] +=
                            mins[0 as libc::c_int as usize] - mid[0 as libc::c_int as usize];
                        start_edgeverts[3 as libc::c_int as usize][1 as libc::c_int as usize] +=
                            maxs[1 as libc::c_int as usize] - mid[1 as libc::c_int as usize];
                        //
                        start_plane.dist =
                            start_edgeverts[0 as libc::c_int as usize][2 as libc::c_int as usize];
                        start_plane.normal[0 as libc::c_int as usize] =
                            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
                        start_plane.normal[1 as libc::c_int as usize] =
                            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
                        start_plane.normal[2 as libc::c_int as usize] =
                            1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
                        //
                        i = 0 as libc::c_int; //end for
                        while i < 4 as libc::c_int {
                            end_edgeverts[i as usize][0 as libc::c_int as usize] =
                                move_end[0 as libc::c_int as usize];
                            end_edgeverts[i as usize][1 as libc::c_int as usize] =
                                move_end[1 as libc::c_int as usize];
                            end_edgeverts[i as usize][2 as libc::c_int as usize] =
                                move_end[2 as libc::c_int as usize];
                            //+ player origin to ground dist
                            end_edgeverts[i as usize][2 as libc::c_int as usize] +=
                                maxs[2 as libc::c_int as usize] - mid[2 as libc::c_int as usize]; //+ bbox maxs z
                            end_edgeverts[i as usize][2 as libc::c_int as usize] +=
                                24 as libc::c_int as libc::c_float;
                            i += 1
                        }
                        end_edgeverts[0 as libc::c_int as usize][0 as libc::c_int as usize] +=
                            maxs[0 as libc::c_int as usize] - mid[0 as libc::c_int as usize];
                        end_edgeverts[0 as libc::c_int as usize][1 as libc::c_int as usize] +=
                            maxs[1 as libc::c_int as usize] - mid[1 as libc::c_int as usize];
                        end_edgeverts[1 as libc::c_int as usize][0 as libc::c_int as usize] +=
                            maxs[0 as libc::c_int as usize] - mid[0 as libc::c_int as usize];
                        end_edgeverts[1 as libc::c_int as usize][1 as libc::c_int as usize] +=
                            mins[1 as libc::c_int as usize] - mid[1 as libc::c_int as usize];
                        end_edgeverts[2 as libc::c_int as usize][0 as libc::c_int as usize] +=
                            mins[0 as libc::c_int as usize] - mid[0 as libc::c_int as usize];
                        end_edgeverts[2 as libc::c_int as usize][1 as libc::c_int as usize] +=
                            mins[1 as libc::c_int as usize] - mid[1 as libc::c_int as usize];
                        end_edgeverts[3 as libc::c_int as usize][0 as libc::c_int as usize] +=
                            mins[0 as libc::c_int as usize] - mid[0 as libc::c_int as usize];
                        end_edgeverts[3 as libc::c_int as usize][1 as libc::c_int as usize] +=
                            maxs[1 as libc::c_int as usize] - mid[1 as libc::c_int as usize];
                        //
                        end_plane.dist =
                            end_edgeverts[0 as libc::c_int as usize][2 as libc::c_int as usize];
                        end_plane.normal[0 as libc::c_int as usize] =
                            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
                        end_plane.normal[1 as libc::c_int as usize] =
                            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
                        end_plane.normal[2 as libc::c_int as usize] =
                            1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
                        //
                        move_start_top[0 as libc::c_int as usize] =
                            move_start[0 as libc::c_int as usize]; //+ bbox maxs z
                        move_start_top[1 as libc::c_int as usize] =
                            move_start[1 as libc::c_int as usize]; //+ bbox maxs z
                        move_start_top[2 as libc::c_int as usize] =
                            move_start[2 as libc::c_int as usize];
                        move_start_top[2 as libc::c_int as usize] +=
                            maxs[2 as libc::c_int as usize] - mid[2 as libc::c_int as usize]
                                + 24 as libc::c_int as libc::c_float;
                        move_end_top[0 as libc::c_int as usize] =
                            move_end[0 as libc::c_int as usize];
                        move_end_top[1 as libc::c_int as usize] =
                            move_end[1 as libc::c_int as usize];
                        move_end_top[2 as libc::c_int as usize] =
                            move_end[2 as libc::c_int as usize];
                        move_end_top[2 as libc::c_int as usize] += maxs[2 as libc::c_int as usize]
                            - mid[2 as libc::c_int as usize]
                            + 24 as libc::c_int as libc::c_float;
                        //
                        if !(crate::src::botlib::be_aas_sample::AAS_PointAreaNum(
                            move_start_top.as_mut_ptr(),
                        ) == 0)
                        {
                            if !(crate::src::botlib::be_aas_sample::AAS_PointAreaNum(
                                move_end_top.as_mut_ptr(),
                            ) == 0)
                            {
                                //
                                i = 0 as libc::c_int;
                                while i < 2 as libc::c_int {
                                    //
                                    if i == 0 as libc::c_int {
                                        //end else
                                        firststartreach = AAS_FindFaceReachabilities(
                                            start_edgeverts.as_mut_ptr(),
                                            4 as libc::c_int,
                                            &mut start_plane,
                                            crate::src::qcommon::q_shared::qtrue as libc::c_int,
                                        ); //end if
                                        firstendreach = AAS_FindFaceReachabilities(
                                            end_edgeverts.as_mut_ptr(),
                                            4 as libc::c_int,
                                            &mut end_plane,
                                            crate::src::qcommon::q_shared::qfalse as libc::c_int,
                                        )
                                    } else {
                                        firststartreach = AAS_FindFaceReachabilities(
                                            end_edgeverts.as_mut_ptr(),
                                            4 as libc::c_int,
                                            &mut end_plane,
                                            crate::src::qcommon::q_shared::qtrue as libc::c_int,
                                        );
                                        firstendreach = AAS_FindFaceReachabilities(
                                            start_edgeverts.as_mut_ptr(),
                                            4 as libc::c_int,
                                            &mut start_plane,
                                            crate::src::qcommon::q_shared::qfalse as libc::c_int,
                                        )
                                    }
                                    //
                                    //create reachabilities from start to end
                                    startreach = firststartreach; //end for
                                    while !startreach.is_null() {
                                        nextstartreach = (*startreach).next;
                                        //end for
                                        endreach = firstendreach;
                                        while !endreach.is_null() {
                                            nextendreach = (*endreach).next;
                                            //
                                            //trace = AAS_TraceClientBBox(startreach->start, move_start_top, PRESENCE_NORMAL, -1);
                                            //if (trace.fraction < 1) continue;
                                            //
                                            //
                                            //
                                            //trace = AAS_TraceClientBBox(endreach->end, move_end_top, PRESENCE_NORMAL, -1);
                                            //if (trace.fraction < 1) continue;
                                            //
                                            crate::src::botlib::l_log::Log_Write(
                                                b"funcbob reach from area %d to %d\n\x00"
                                                    as *const u8
                                                    as *const libc::c_char
                                                    as *mut libc::c_char,
                                                (*startreach).areanum,
                                                (*endreach).areanum,
                                            );
                                            //
                                            //
                                            if i == 0 as libc::c_int {
                                                org[0 as libc::c_int as usize] =
                                                    move_start_top[0 as libc::c_int as usize];
                                                org[1 as libc::c_int as usize] =
                                                    move_start_top[1 as libc::c_int as usize];
                                                org[2 as libc::c_int as usize] =
                                                    move_start_top[2 as libc::c_int as usize]
                                            } else {
                                                org[0 as libc::c_int as usize] =
                                                    move_end_top[0 as libc::c_int as usize];
                                                org[1 as libc::c_int as usize] =
                                                    move_end_top[1 as libc::c_int as usize];
                                                org[2 as libc::c_int as usize] =
                                                    move_end_top[2 as libc::c_int as usize]
                                            }
                                            dir[0 as libc::c_int as usize] = (*startreach).start
                                                [0 as libc::c_int as usize]
                                                - org[0 as libc::c_int as usize];
                                            dir[1 as libc::c_int as usize] = (*startreach).start
                                                [1 as libc::c_int as usize]
                                                - org[1 as libc::c_int as usize];
                                            dir[2 as libc::c_int as usize] = (*startreach).start
                                                [2 as libc::c_int as usize]
                                                - org[2 as libc::c_int as usize];
                                            dir[2 as libc::c_int as usize] = 0 as libc::c_int
                                                as crate::src::qcommon::q_shared::vec_t;
                                            crate::src::qcommon::q_math::VectorNormalize(
                                                dir.as_mut_ptr(),
                                            );
                                            start[0 as libc::c_int as usize] =
                                                (*startreach).start[0 as libc::c_int as usize];
                                            start[1 as libc::c_int as usize] =
                                                (*startreach).start[1 as libc::c_int as usize];
                                            start[2 as libc::c_int as usize] =
                                                (*startreach).start[2 as libc::c_int as usize];
                                            start[0 as libc::c_int as usize] = (*startreach).start
                                                [0 as libc::c_int as usize]
                                                + dir[0 as libc::c_int as usize]
                                                    * 1 as libc::c_int as libc::c_float;
                                            start[1 as libc::c_int as usize] = (*startreach).start
                                                [1 as libc::c_int as usize]
                                                + dir[1 as libc::c_int as usize]
                                                    * 1 as libc::c_int as libc::c_float;
                                            start[2 as libc::c_int as usize] = (*startreach).start
                                                [2 as libc::c_int as usize]
                                                + dir[2 as libc::c_int as usize]
                                                    * 1 as libc::c_int as libc::c_float;
                                            start[2 as libc::c_int as usize] +=
                                                1 as libc::c_int as libc::c_float;
                                            end[0 as libc::c_int as usize] = (*startreach).start
                                                [0 as libc::c_int as usize]
                                                + dir[0 as libc::c_int as usize]
                                                    * 16 as libc::c_int as libc::c_float;
                                            end[1 as libc::c_int as usize] = (*startreach).start
                                                [1 as libc::c_int as usize]
                                                + dir[1 as libc::c_int as usize]
                                                    * 16 as libc::c_int as libc::c_float;
                                            end[2 as libc::c_int as usize] = (*startreach).start
                                                [2 as libc::c_int as usize]
                                                + dir[2 as libc::c_int as usize]
                                                    * 16 as libc::c_int as libc::c_float;
                                            end[2 as libc::c_int as usize] +=
                                                1 as libc::c_int as libc::c_float;
                                            //
                                            numareas =
                                                crate::src::botlib::be_aas_sample::AAS_TraceAreas(
                                                    start.as_mut_ptr(),
                                                    end.as_mut_ptr(),
                                                    areas.as_mut_ptr(),
                                                    points.as_mut_ptr(),
                                                    10 as libc::c_int,
                                                );
                                            if !(numareas <= 0 as libc::c_int) {
                                                if numareas > 1 as libc::c_int {
                                                    (*startreach).start
                                                        [0 as libc::c_int as usize] = points
                                                        [1 as libc::c_int as usize]
                                                        [0 as libc::c_int as usize];
                                                    (*startreach).start
                                                        [1 as libc::c_int as usize] = points
                                                        [1 as libc::c_int as usize]
                                                        [1 as libc::c_int as usize];
                                                    (*startreach).start[2 as libc::c_int as usize] =
                                                        points[1 as libc::c_int as usize]
                                                            [2 as libc::c_int as usize]
                                                } else {
                                                    (*startreach).start
                                                        [0 as libc::c_int as usize] =
                                                        end[0 as libc::c_int as usize];
                                                    (*startreach).start
                                                        [1 as libc::c_int as usize] =
                                                        end[1 as libc::c_int as usize];
                                                    (*startreach).start[2 as libc::c_int as usize] =
                                                        end[2 as libc::c_int as usize]
                                                }
                                                //
                                                if !(crate::src::botlib::be_aas_sample::AAS_PointAreaNum((*startreach).start.as_mut_ptr())
                                                         == 0) {
                                                    if !(crate::src::botlib::be_aas_sample::AAS_PointAreaNum((*endreach).end.as_mut_ptr())
                                                             == 0) {
                                                        //
                                                        lreach =
                                                            AAS_AllocReachability();
                                                        (*lreach).areanum =
                                                            (*endreach).areanum;
                                                        if i ==
                                                               0 as
                                                                   libc::c_int
                                                           {
                                                            (*lreach).edgenum
                                                                =
                                                                (move_start[axis
                                                                                as
                                                                                usize]
                                                                     as
                                                                     libc::c_int)
                                                                    <<
                                                                    16 as
                                                                        libc::c_int
                                                                    |
                                                                    move_end[axis
                                                                                 as
                                                                                 usize]
                                                                        as
                                                                        libc::c_int
                                                                        &
                                                                        0xffff
                                                                            as
                                                                            libc::c_int
                                                        } else {
                                                            (*lreach).edgenum
                                                                =
                                                                (move_end[axis
                                                                              as
                                                                              usize]
                                                                     as
                                                                     libc::c_int)
                                                                    <<
                                                                    16 as
                                                                        libc::c_int
                                                                    |
                                                                    move_start[axis
                                                                                   as
                                                                                   usize]
                                                                        as
                                                                        libc::c_int
                                                                        &
                                                                        0xffff
                                                                            as
                                                                            libc::c_int
                                                        }
                                                        (*lreach).facenum =
                                                            spawnflags <<
                                                                16 as
                                                                    libc::c_int
                                                                | modelnum;
                                                        (*lreach).start[0 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                                            =
                                                            (*startreach).start[0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    usize];
                                                        (*lreach).start[1 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                                            =
                                                            (*startreach).start[1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    usize];
                                                        (*lreach).start[2 as
                                                                            libc::c_int
                                                                            as
                                                                            usize]
                                                            =
                                                            (*startreach).start[2
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    usize];
                                                        (*lreach).end[0 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                                            =
                                                            (*endreach).end[0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize];
                                                        (*lreach).end[1 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                                            =
                                                            (*endreach).end[1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize];
                                                        (*lreach).end[2 as
                                                                          libc::c_int
                                                                          as
                                                                          usize]
                                                            =
                                                            (*endreach).end[2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize];
                                                        //					AAS_DrawArrow(lreach->start, lreach->end, LINECOLOR_BLUE, LINECOLOR_YELLOW);
//					AAS_PermanentLine(lreach->start, lreach->end, 1);
                                                        (*lreach).traveltype =
                                                            19 as
                                                                libc::c_int; //end for
                                                        (*lreach).traveltype
                                                            |=
                                                            AAS_TravelFlagsForTeam(ent); //end for
                                                        (*lreach).traveltime =
                                                            crate::src::botlib::be_aas_move::aassettings.rs_funcbob
                                                                as
                                                                libc::c_ushort;
                                                        reach_funcbob += 1;
                                                        (*lreach).next =
                                                            *areareachability.offset((*startreach).areanum
                                                                                         as
                                                                                         isize);
                                                        let ref mut fresh21 =
                                                            *areareachability.offset((*startreach).areanum
                                                                                         as
                                                                                         isize);
                                                        *fresh21 = lreach
                                                    }
                                                }
                                            }
                                            endreach = nextendreach
                                        }
                                        startreach = nextstartreach
                                    }
                                    startreach = firststartreach;
                                    while !startreach.is_null() {
                                        nextstartreach = (*startreach).next;
                                        AAS_FreeReachability(startreach);
                                        startreach = nextstartreach
                                    }
                                    endreach = firstendreach;
                                    while !endreach.is_null() {
                                        nextendreach = (*endreach).next;
                                        AAS_FreeReachability(endreach);
                                        endreach = nextendreach
                                    }
                                    //only go up with func_bobbing entities that go up and down
                                    if spawnflags & 1 as libc::c_int == 0
                                        && spawnflags & 2 as libc::c_int == 0
                                    {
                                        break;
                                    }
                                    i += 1
                                }
                            }
                        }
                    }
                }
            }
        }
        ent = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(ent)
        //end for
    }
    //end for
}
//end of the function AAS_Reachability_FuncBobbing
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Reachability_JumpPad() {
    let mut face2num: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut area2num: libc::c_int = 0;
    let mut visualize: libc::c_int = 0;
    let mut ent: libc::c_int = 0;
    let mut bot_visualizejumppads: libc::c_int = 0;
    //int modelnum, ent2;
    //float dist, time, height, gravity, forward;
    let mut speed: libc::c_float = 0.;
    let mut zvel: libc::c_float = 0.;
    //float hordist;
    let mut face2: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut area2: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut areastart: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut facecenter: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut cmdmove: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut absmins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut absmaxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    //vec3_t origin, ent2origin, angles, teststart;
    let mut move_0: crate::be_aas_h::aas_clientmove_t = crate::be_aas_h::aas_clientmove_t {
        endpos: [0.; 3],
        endarea: 0,
        velocity: [0.; 3],
        trace: crate::be_aas_h::aas_trace_t {
            startsolid: crate::src::qcommon::q_shared::qfalse,
            fraction: 0.,
            endpos: [0.; 3],
            ent: 0,
            lastarea: 0,
            area: 0,
            planenum: 0,
        },
        presencetype: 0,
        stopevent: 0,
        endcontents: 0,
        time: 0.,
        frames: 0,
    };
    //aas_trace_t trace;
    let mut areas: *mut crate::be_aas_def_h::aas_link_t = 0 as *mut crate::be_aas_def_h::aas_link_t;
    let mut link: *mut crate::be_aas_def_h::aas_link_t = 0 as *mut crate::be_aas_def_h::aas_link_t;
    //char target[MAX_EPAIRKEY], targetname[MAX_EPAIRKEY], model[MAX_EPAIRKEY];
    let mut classname: [libc::c_char; 128] = [0; 128];
    bot_visualizejumppads = crate::src::botlib::l_libvar::LibVarValue(
        b"bot_visualizejumppads\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    ent = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(0 as libc::c_int);
    while ent != 0 {
        if !(crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
            ent,
            b"classname\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            classname.as_mut_ptr(),
            128 as libc::c_int,
        ) == 0)
        {
            if !(crate::stdlib::strcmp(
                classname.as_mut_ptr(),
                b"trigger_push\x00" as *const u8 as *const libc::c_char,
            ) != 0)
            {
                //
                if !(AAS_GetJumpPadInfo(
                    ent,
                    areastart.as_mut_ptr(),
                    absmins.as_mut_ptr(),
                    absmaxs.as_mut_ptr(),
                    velocity.as_mut_ptr(),
                ) == 0)
                {
                    /*
                            //
                            AAS_FloatForBSPEpairKey(ent, "speed", &speed);
                            if (!speed) speed = 1000;
                    //		AAS_VectorForBSPEpairKey(ent, "angles", angles);
                    //		AAS_SetMovedir(angles, velocity);
                    //		VectorScale(velocity, speed, velocity);
                            VectorClear(angles);
                            //get the mins, maxs and origin of the model
                            AAS_ValueForBSPEpairKey(ent, "model", model, MAX_EPAIRKEY);
                            if (model[0]) modelnum = atoi(model+1);
                            else modelnum = 0;
                            AAS_BSPModelMinsMaxsOrigin(modelnum, angles, absmins, absmaxs, origin);
                            VectorAdd(origin, absmins, absmins);
                            VectorAdd(origin, absmaxs, absmaxs);
                            //
                    #ifdef REACH_DEBUG
                            botimport.Print(PRT_MESSAGE, "absmins = %f %f %f\n", absmins[0], absmins[1], absmins[2]);
                            botimport.Print(PRT_MESSAGE, "absmaxs = %f %f %f\n", absmaxs[0], absmaxs[1], absmaxs[2]);
                    #endif REACH_DEBUG
                            VectorAdd(absmins, absmaxs, origin);
                            VectorScale (origin, 0.5, origin);

                            //get the start areas
                            VectorCopy(origin, teststart);
                            teststart[2] += 64;
                            trace = AAS_TraceClientBBox(teststart, origin, PRESENCE_CROUCH, -1);
                            if (trace.startsolid)
                            {
                                botimport.Print(PRT_MESSAGE, "trigger_push start solid\n");
                                VectorCopy(origin, areastart);
                            } //end if
                            else
                            {
                                VectorCopy(trace.endpos, areastart);
                            } //end else
                            areastart[2] += 0.125;
                            //
                            //AAS_DrawPermanentCross(origin, 4, 4);
                            //get the target entity
                            AAS_ValueForBSPEpairKey(ent, "target", target, MAX_EPAIRKEY);
                            for (ent2 = AAS_NextBSPEntity(0); ent2; ent2 = AAS_NextBSPEntity(ent2))
                            {
                                if (!AAS_ValueForBSPEpairKey(ent2, "targetname", targetname, MAX_EPAIRKEY)) continue;
                                if (!strcmp(targetname, target)) break;
                            } //end for
                            if (!ent2)
                            {
                                botimport.Print(PRT_MESSAGE, "trigger_push without target entity %s\n", target);
                                continue;
                            } //end if
                            AAS_VectorForBSPEpairKey(ent2, "origin", ent2origin);
                            //
                            height = ent2origin[2] - origin[2];
                            gravity = aassettings.sv_gravity;
                            time = sqrt( height / ( 0.5 * gravity ) );
                            if (!time)
                            {
                                botimport.Print(PRT_MESSAGE, "trigger_push without time\n");
                                continue;
                            } //end if
                            // set s.origin2 to the push velocity
                            VectorSubtract ( ent2origin, origin, velocity);
                            dist = VectorNormalize( velocity);
                            forward = dist / time;
                            //FIXME: why multiply by 1.1
                            forward *= 1.1;
                            VectorScale(velocity, forward, velocity);
                            velocity[2] = time * gravity;
                            */
                    //get the areas the jump pad brush is in
                    areas = crate::src::botlib::be_aas_sample::AAS_LinkEntityClientBBox(
                        absmins.as_mut_ptr(),
                        absmaxs.as_mut_ptr(),
                        -(1 as libc::c_int),
                        4 as libc::c_int,
                    );
                    /*
                    for (link = areas; link; link = link->next_area)
                    {
                        if (link->areanum == 563)
                        {
                            ret = qfalse;
                        }
                    }
                    */
                    link = areas; //end for
                    while !link.is_null() {
                        if AAS_AreaJumpPad((*link).areanum) != 0 {
                            break; //end if
                        }
                        link = (*link).next_area
                    }
                    if link.is_null() {
                        botimport.Print.expect("non-null function pointer")(
                            1 as libc::c_int,
                            b"trigger_push not in any jump pad area\n\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        crate::src::botlib::be_aas_sample::AAS_UnlinkFromAreas(areas);
                    } else {
                        //
                        botimport.Print.expect("non-null function pointer")(
                            1 as libc::c_int,
                            b"found a trigger_push with velocity %f %f %f\n\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                            velocity[0 as libc::c_int as usize] as libc::c_double,
                            velocity[1 as libc::c_int as usize] as libc::c_double,
                            velocity[2 as libc::c_int as usize] as libc::c_double,
                        );
                        //if there is a horizontal velocity check for a reachability without air control
                        if velocity[0 as libc::c_int as usize] != 0.
                            || velocity[1 as libc::c_int as usize] != 0.
                        {
                            cmdmove[0 as libc::c_int as usize] =
                                0 as libc::c_int as crate::src::qcommon::q_shared::vec_t; //end if
                            cmdmove[1 as libc::c_int as usize] =
                                0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
                            cmdmove[2 as libc::c_int as usize] =
                                0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
                            //end if
                            crate::stdlib::memset(
                                &mut move_0 as *mut crate::be_aas_h::aas_clientmove_t
                                    as *mut libc::c_void,
                                0 as libc::c_int,
                                ::std::mem::size_of::<crate::be_aas_h::aas_clientmove_t>()
                                    as libc::c_ulong,
                            );
                            area2num = 0 as libc::c_int;
                            i = 0 as libc::c_int;
                            while i < 20 as libc::c_int {
                                crate::src::botlib::be_aas_move::AAS_PredictClientMovement(
                                    &mut move_0,
                                    -(1 as libc::c_int),
                                    areastart.as_mut_ptr(),
                                    2 as libc::c_int,
                                    crate::src::qcommon::q_shared::qfalse as libc::c_int,
                                    velocity.as_mut_ptr(),
                                    cmdmove.as_mut_ptr(),
                                    0 as libc::c_int,
                                    30 as libc::c_int,
                                    0.1f32,
                                    1 as libc::c_int
                                        | 4 as libc::c_int
                                        | 8 as libc::c_int
                                        | 16 as libc::c_int
                                        | 32 as libc::c_int
                                        | 128 as libc::c_int
                                        | 256 as libc::c_int,
                                    0 as libc::c_int,
                                    bot_visualizejumppads,
                                );
                                area2num = move_0.endarea;
                                //VectorCopy(velocity, cmdmove);
                                //cmdmove[2] = 0;
                                //end for
                                link = areas; //end if
                                while !link.is_null() {
                                    if !(AAS_AreaJumpPad((*link).areanum) == 0) {
                                        if (*link).areanum == area2num {
                                            break;
                                        }
                                    }
                                    link = (*link).next_area
                                }
                                if link.is_null() {
                                    break;
                                }
                                areastart[0 as libc::c_int as usize] =
                                    move_0.endpos[0 as libc::c_int as usize];
                                areastart[1 as libc::c_int as usize] =
                                    move_0.endpos[1 as libc::c_int as usize];
                                areastart[2 as libc::c_int as usize] =
                                    move_0.endpos[2 as libc::c_int as usize];
                                velocity[0 as libc::c_int as usize] =
                                    move_0.velocity[0 as libc::c_int as usize];
                                velocity[1 as libc::c_int as usize] =
                                    move_0.velocity[1 as libc::c_int as usize];
                                velocity[2 as libc::c_int as usize] =
                                    move_0.velocity[2 as libc::c_int as usize];
                                i += 1
                            }
                            if area2num != 0 && i < 20 as libc::c_int {
                                link = areas;
                                while !link.is_null() {
                                    if !(AAS_AreaJumpPad((*link).areanum) == 0) {
                                        if !(AAS_ReachabilityExists((*link).areanum, area2num)
                                            as u64
                                            != 0)
                                        {
                                            //create a rocket or bfg jump reachability from area1 to area2
                                            lreach = AAS_AllocReachability(); //end if
                                            if lreach.is_null() {
                                                crate::src::botlib::be_aas_sample::AAS_UnlinkFromAreas(areas);
                                                return;
                                            }
                                            (*lreach).areanum = area2num;
                                            //NOTE: the facenum is the Z velocity
                                            (*lreach).facenum =
                                                velocity[2 as libc::c_int as usize] as libc::c_int;
                                            //NOTE: the edgenum is the horizontal velocity
                                            (*lreach).edgenum = crate::stdlib::sqrt(
                                                (velocity[0 as libc::c_int as usize]
                                                    * velocity[0 as libc::c_int as usize]
                                                    + velocity[1 as libc::c_int as usize]
                                                        * velocity[1 as libc::c_int as usize])
                                                    as libc::c_double,
                                            )
                                                as libc::c_int;
                                            (*lreach).start[0 as libc::c_int as usize] =
                                                areastart[0 as libc::c_int as usize];
                                            (*lreach).start[1 as libc::c_int as usize] =
                                                areastart[1 as libc::c_int as usize];
                                            (*lreach).start[2 as libc::c_int as usize] =
                                                areastart[2 as libc::c_int as usize];
                                            (*lreach).end[0 as libc::c_int as usize] =
                                                move_0.endpos[0 as libc::c_int as usize];
                                            (*lreach).end[1 as libc::c_int as usize] =
                                                move_0.endpos[1 as libc::c_int as usize];
                                            (*lreach).end[2 as libc::c_int as usize] =
                                                move_0.endpos[2 as libc::c_int as usize];
                                            (*lreach).traveltype = 18 as libc::c_int;
                                            (*lreach).traveltype |= AAS_TravelFlagsForTeam(ent);
                                            (*lreach).traveltime =
                                                crate::src::botlib::be_aas_move::aassettings
                                                    .rs_jumppad
                                                    as libc::c_ushort;
                                            (*lreach).next =
                                                *areareachability.offset((*link).areanum as isize);
                                            let ref mut fresh22 =
                                                *areareachability.offset((*link).areanum as isize);
                                            *fresh22 = lreach;
                                            //
                                            reach_jumppad += 1
                                        }
                                    }
                                    link = (*link).next_area
                                }
                                //end for
                            }
                        }
                        //
                        if !(crate::stdlib::fabs(
                            velocity[0 as libc::c_int as usize] as libc::c_double,
                        ) > 100 as libc::c_int as libc::c_double
                            || crate::stdlib::fabs(
                                velocity[1 as libc::c_int as usize] as libc::c_double,
                            ) > 100 as libc::c_int as libc::c_double)
                        {
                            //check for areas we can reach with air control
                            area2num = 1 as libc::c_int; //end for
                            while area2num < crate::src::botlib::be_aas_main::aasworld.numareas {
                                visualize = crate::src::qcommon::q_shared::qfalse as libc::c_int;
                                //end for
                                /*
                                if (area2num == 3568)
                                {
                                    for (link = areas; link; link = link->next_area)
                                    {
                                        if (link->areanum == 3380)
                                        {
                                            visualize = qtrue;
                                            botimport.Print(PRT_MESSAGE, "bah\n");
                                        } //end if
                                    } //end for
                                } //end if*/
                                //never try to go back to one of the original jumppad areas
                                //and don't create reachabilities if they already exist
                                link = areas; //end if
                                while !link.is_null() {
                                    if AAS_ReachabilityExists((*link).areanum, area2num) as u64 != 0
                                    {
                                        break;
                                        //end if
                                    }
                                    if AAS_AreaJumpPad((*link).areanum) != 0 {
                                        if (*link).areanum == area2num {
                                            break;
                                        }
                                    }
                                    link = (*link).next_area
                                }
                                if link.is_null() {
                                    //
                                    area2 = &mut *crate::src::botlib::be_aas_main::aasworld
                                        .areas
                                        .offset(area2num as isize)
                                        as *mut crate::aasfile_h::aas_area_t;
                                    i = 0 as libc::c_int;
                                    while i < (*area2).numfaces {
                                        face2num = *crate::src::botlib::be_aas_main::aasworld
                                            .faceindex
                                            .offset(((*area2).firstface + i) as isize);
                                        face2 = &mut *crate::src::botlib::be_aas_main::aasworld
                                            .faces
                                            .offset((crate::stdlib::abs
                                                as unsafe extern "C" fn(
                                                    _: libc::c_int,
                                                )
                                                    -> libc::c_int)(
                                                face2num
                                            )
                                                as isize)
                                            as *mut crate::aasfile_h::aas_face_t;
                                        //end for
                                        //if it is not a ground face
                                        if !((*face2).faceflags & 4 as libc::c_int == 0) {
                                            //get the center of the face
                                            AAS_FaceCenter(face2num, facecenter.as_mut_ptr());
                                            //only go higher up
                                            if !(facecenter[2 as libc::c_int as usize]
                                                < areastart[2 as libc::c_int as usize])
                                            {
                                                //get the jumppad jump z velocity
                                                zvel = velocity[2 as libc::c_int as usize];
                                                //get the horizontal speed for the jump, if it isn't possible to calculate this
                                                //speed
                                                ret =
                                                    crate::src::botlib::be_aas_move::AAS_HorizontalVelocityForJump(zvel,
                                                                                  areastart.as_mut_ptr(),
                                                                                  facecenter.as_mut_ptr(),
                                                                                  &mut speed);
                                                if ret != 0
                                                    && speed < 150 as libc::c_int as libc::c_float
                                                {
                                                    //direction towards the face center
                                                    dir[0 as libc::c_int as usize] = facecenter
                                                        [0 as libc::c_int as usize]
                                                        - areastart[0 as libc::c_int as usize];
                                                    dir[1 as libc::c_int as usize] = facecenter
                                                        [1 as libc::c_int as usize]
                                                        - areastart[1 as libc::c_int as usize];
                                                    dir[2 as libc::c_int as usize] = facecenter
                                                        [2 as libc::c_int as usize]
                                                        - areastart[2 as libc::c_int as usize];
                                                    dir[2 as libc::c_int as usize] = 0
                                                        as libc::c_int
                                                        as crate::src::qcommon::q_shared::vec_t;
                                                    //end if
                                                    //hordist = VectorNormalize(dir);
                                                    //if (hordist < 1.6 * facecenter[2] - areastart[2])
                                                    //get command movement
                                                    cmdmove[0 as libc::c_int as usize] =
                                                        dir[0 as libc::c_int as usize] * speed;
                                                    cmdmove[1 as libc::c_int as usize] =
                                                        dir[1 as libc::c_int as usize] * speed;
                                                    cmdmove[2 as libc::c_int as usize] =
                                                        dir[2 as libc::c_int as usize] * speed;
                                                    crate::src::botlib::be_aas_move::AAS_PredictClientMovement(&mut move_0,
                                                                              -(1
                                                                                    as
                                                                                    libc::c_int),
                                                                              areastart.as_mut_ptr(),
                                                                              2
                                                                                  as
                                                                                  libc::c_int,
                                                                              crate::src::qcommon::q_shared::qfalse
                                                                                  as
                                                                                  libc::c_int,
                                                                              velocity.as_mut_ptr(),
                                                                              cmdmove.as_mut_ptr(),
                                                                              30
                                                                                  as
                                                                                  libc::c_int,
                                                                              30
                                                                                  as
                                                                                  libc::c_int,
                                                                              0.1f32,
                                                                              4
                                                                                  as
                                                                                  libc::c_int
                                                                                  |
                                                                                  8
                                                                                      as
                                                                                      libc::c_int
                                                                                  |
                                                                                  16
                                                                                      as
                                                                                      libc::c_int
                                                                                  |
                                                                                  32
                                                                                      as
                                                                                      libc::c_int
                                                                                  |
                                                                                  128
                                                                                      as
                                                                                      libc::c_int
                                                                                  |
                                                                                  256
                                                                                      as
                                                                                      libc::c_int
                                                                                  |
                                                                                  1024
                                                                                      as
                                                                                      libc::c_int,
                                                                              area2num,
                                                                              visualize);
                                                    if move_0.frames < 30 as libc::c_int
                                                        && move_0.stopevent
                                                            & (8 as libc::c_int
                                                                | 16 as libc::c_int
                                                                | 32 as libc::c_int)
                                                            == 0
                                                        && move_0.stopevent
                                                            & (1024 as libc::c_int
                                                                | 128 as libc::c_int
                                                                | 256 as libc::c_int)
                                                            != 0
                                                    {
                                                        //end if
                                                        //
                                                        //never go back to the same jumppad
                                                        link = areas;
                                                        while !link.is_null() {
                                                            if (*link).areanum == move_0.endarea {
                                                                break;
                                                            }
                                                            link = (*link).next_area
                                                        }
                                                        if link.is_null() {
                                                            link = areas;
                                                            while !link.is_null() {
                                                                if !(AAS_AreaJumpPad(
                                                                    (*link).areanum,
                                                                ) == 0)
                                                                {
                                                                    if !(AAS_ReachabilityExists(
                                                                        (*link).areanum,
                                                                        area2num,
                                                                    )
                                                                        as u64
                                                                        != 0)
                                                                    {
                                                                        //create a jumppad reachability from area1 to area2
                                                                        lreach =
                                                                            AAS_AllocReachability(); //end if
                                                                        if lreach.is_null() {
                                                                            crate::src::botlib::be_aas_sample::AAS_UnlinkFromAreas(areas);
                                                                            return;
                                                                        }
                                                                        (*lreach).areanum =
                                                                            move_0.endarea;
                                                                        //NOTE: the facenum is the Z velocity
                                                                        (*lreach).facenum = velocity
                                                                            [2 as libc::c_int
                                                                                as usize]
                                                                            as libc::c_int;
                                                                        //NOTE: the edgenum is the horizontal velocity
                                                                        (*lreach).edgenum
                                                                            =
                                                                            crate::stdlib::sqrt((cmdmove[0
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              usize]
                                                                                      *
                                                                                      cmdmove[0
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize]
                                                                                      +
                                                                                      cmdmove[1
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize]
                                                                                          *
                                                                                          cmdmove[1
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      usize])
                                                                                     as
                                                                                     libc::c_double)
                                                                                as
                                                                                libc::c_int;
                                                                        (*lreach).start[0
                                                                            as libc::c_int
                                                                            as usize] = areastart[0
                                                                            as libc::c_int
                                                                            as usize];
                                                                        (*lreach).start[1
                                                                            as libc::c_int
                                                                            as usize] = areastart[1
                                                                            as libc::c_int
                                                                            as usize];
                                                                        (*lreach).start[2
                                                                            as libc::c_int
                                                                            as usize] = areastart[2
                                                                            as libc::c_int
                                                                            as usize];
                                                                        (*lreach).end[0
                                                                            as libc::c_int
                                                                            as usize] = facecenter
                                                                            [0 as libc::c_int
                                                                                as usize];
                                                                        (*lreach).end[1
                                                                            as libc::c_int
                                                                            as usize] = facecenter
                                                                            [1 as libc::c_int
                                                                                as usize];
                                                                        (*lreach).end[2
                                                                            as libc::c_int
                                                                            as usize] = facecenter
                                                                            [2 as libc::c_int
                                                                                as usize];
                                                                        (*lreach).traveltype =
                                                                            18 as libc::c_int;
                                                                        (*lreach).traveltype |=
                                                                            AAS_TravelFlagsForTeam(
                                                                                ent,
                                                                            );
                                                                        (*lreach).traveltime
                                                                            =
                                                                            crate::src::botlib::be_aas_move::aassettings.rs_aircontrolledjumppad
                                                                                as
                                                                                libc::c_ushort;
                                                                        (*lreach).next =
                                                                            *areareachability
                                                                                .offset(
                                                                                    (*link).areanum
                                                                                        as isize,
                                                                                );
                                                                        let ref mut fresh23 =
                                                                            *areareachability
                                                                                .offset(
                                                                                    (*link).areanum
                                                                                        as isize,
                                                                                );
                                                                        *fresh23 = lreach;
                                                                        //
                                                                        reach_jumppad += 1
                                                                    }
                                                                }
                                                                link = (*link).next_area
                                                            }
                                                            //end for
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        i += 1
                                    }
                                }
                                area2num += 1
                            }
                            crate::src::botlib::be_aas_sample::AAS_UnlinkFromAreas(areas);
                        }
                    }
                }
            }
        }
        ent = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(ent)
    }
    //if prediction time wasn't enough to fully predict the movement
    //don't enter slime or lava and don't fall from too high
    //end for
}
//end of the function AAS_Reachability_JumpPad
//===========================================================================
// never point at ground faces
// always a higher and pretty far area
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Reachability_Grapple(
    mut area1num: libc::c_int,
    mut area2num: libc::c_int,
) -> libc::c_int {
    let mut face2num: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut areanum: libc::c_int = 0;
    let mut numareas: libc::c_int = 0;
    let mut areas: [libc::c_int; 20] = [0; 20];
    let mut mingrappleangle: libc::c_float = 0.;
    let mut z: libc::c_float = 0.;
    let mut hordist: libc::c_float = 0.;
    let mut bsptrace: crate::botlib_h::bsp_trace_t = crate::botlib_h::bsp_trace_t {
        allsolid: crate::src::qcommon::q_shared::qfalse,
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: crate::src::qcommon::q_shared::cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        exp_dist: 0.,
        sidenum: 0,
        surface: crate::botlib_h::bsp_surface_t {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    };
    let mut trace: crate::be_aas_h::aas_trace_t = crate::be_aas_h::aas_trace_t {
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        ent: 0,
        lastarea: 0,
        area: 0,
        planenum: 0,
    };
    let mut face2: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut area1: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut area2: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut areastart: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut facecenter: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut down: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        -(1 as libc::c_int) as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut v: *mut crate::src::qcommon::q_shared::vec_t =
        0 as *mut crate::src::qcommon::q_shared::vec_t;
    //only grapple when on the ground or swimming
    if AAS_AreaGrounded(area1num) == 0 && AAS_AreaSwim(area1num) == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //don't grapple from a crouch area
    if crate::src::botlib::be_aas_sample::AAS_AreaPresenceType(area1num) & 2 as libc::c_int == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //NOTE: disabled area swim it doesn't work right
    if AAS_AreaSwim(area1num) != 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //
    area1 = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(area1num as isize) as *mut crate::aasfile_h::aas_area_t;
    area2 = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(area2num as isize) as *mut crate::aasfile_h::aas_area_t;
    //don't grapple towards way lower areas
    if (*area2).maxs[2 as libc::c_int as usize] < (*area1).mins[2 as libc::c_int as usize] {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //
    start[0 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(area1num as isize))
    .center[0 as libc::c_int as usize];
    start[1 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(area1num as isize))
    .center[1 as libc::c_int as usize];
    start[2 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(area1num as isize))
    .center[2 as libc::c_int as usize];
    //if not a swim area
    if AAS_AreaSwim(area1num) == 0 {
        //end else
        if crate::src::botlib::be_aas_sample::AAS_PointAreaNum(start.as_mut_ptr()) == 0 {
            crate::src::botlib::l_log::Log_Write(
                b"area %d center %f %f %f in solid?\r\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                area1num,
                start[0 as libc::c_int as usize] as libc::c_double,
                start[1 as libc::c_int as usize] as libc::c_double,
                start[2 as libc::c_int as usize] as libc::c_double,
            ); //end if
        }
        end[0 as libc::c_int as usize] = start[0 as libc::c_int as usize];
        end[1 as libc::c_int as usize] = start[1 as libc::c_int as usize];
        end[2 as libc::c_int as usize] = start[2 as libc::c_int as usize];
        end[2 as libc::c_int as usize] -= 1000 as libc::c_int as libc::c_float;
        trace = crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(
            start.as_mut_ptr(),
            end.as_mut_ptr(),
            4 as libc::c_int,
            -(1 as libc::c_int),
        );
        if trace.startsolid as u64 != 0 {
            return crate::src::qcommon::q_shared::qfalse as libc::c_int;
        }
        areastart[0 as libc::c_int as usize] = trace.endpos[0 as libc::c_int as usize];
        areastart[1 as libc::c_int as usize] = trace.endpos[1 as libc::c_int as usize];
        areastart[2 as libc::c_int as usize] = trace.endpos[2 as libc::c_int as usize]
    } else if crate::src::botlib::be_aas_bspq3::AAS_PointContents(start.as_mut_ptr())
        & (8 as libc::c_int | 16 as libc::c_int | 32 as libc::c_int)
        == 0
    {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //
    //start is now the start point
    //
    i = 0 as libc::c_int; //end for
    while i < (*area2).numfaces {
        face2num = *crate::src::botlib::be_aas_main::aasworld
            .faceindex
            .offset(((*area2).firstface + i) as isize);
        face2 = &mut *crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(
                (crate::stdlib::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(
                    face2num,
                ) as isize,
            ) as *mut crate::aasfile_h::aas_face_t;
        //if it is not a solid face
        if !((*face2).faceflags & 1 as libc::c_int == 0) {
            //direction towards the first vertex of the face
            v = (*crate::src::botlib::be_aas_main::aasworld.vertexes.offset(
                (*crate::src::botlib::be_aas_main::aasworld
                    .edges
                    .offset(crate::stdlib::abs(
                        *crate::src::botlib::be_aas_main::aasworld
                            .edgeindex
                            .offset((*face2).firstedge as isize),
                    ) as isize))
                .v[0 as libc::c_int as usize] as isize,
            ))
            .as_mut_ptr();
            dir[0 as libc::c_int as usize] =
                *v.offset(0 as libc::c_int as isize) - areastart[0 as libc::c_int as usize];
            dir[1 as libc::c_int as usize] =
                *v.offset(1 as libc::c_int as isize) - areastart[1 as libc::c_int as usize];
            dir[2 as libc::c_int as usize] =
                *v.offset(2 as libc::c_int as isize) - areastart[2 as libc::c_int as usize];
            //if the face plane is facing away
            if !((*crate::src::botlib::be_aas_main::aasworld
                .planes
                .offset((*face2).planenum as isize))
            .normal[0 as libc::c_int as usize]
                * dir[0 as libc::c_int as usize]
                + (*crate::src::botlib::be_aas_main::aasworld
                    .planes
                    .offset((*face2).planenum as isize))
                .normal[1 as libc::c_int as usize]
                    * dir[1 as libc::c_int as usize]
                + (*crate::src::botlib::be_aas_main::aasworld
                    .planes
                    .offset((*face2).planenum as isize))
                .normal[2 as libc::c_int as usize]
                    * dir[2 as libc::c_int as usize]
                > 0 as libc::c_int as libc::c_float)
            {
                //get the center of the face
                AAS_FaceCenter(face2num, facecenter.as_mut_ptr());
                //only go higher up with the grapple
                if !(facecenter[2 as libc::c_int as usize]
                    < areastart[2 as libc::c_int as usize] + 64 as libc::c_int as libc::c_float)
                {
                    //only use vertical faces or downward facing faces
                    if !((*crate::src::botlib::be_aas_main::aasworld
                        .planes
                        .offset((*face2).planenum as isize))
                    .normal[0 as libc::c_int as usize]
                        * down[0 as libc::c_int as usize]
                        + (*crate::src::botlib::be_aas_main::aasworld
                            .planes
                            .offset((*face2).planenum as isize))
                        .normal[1 as libc::c_int as usize]
                            * down[1 as libc::c_int as usize]
                        + (*crate::src::botlib::be_aas_main::aasworld
                            .planes
                            .offset((*face2).planenum as isize))
                        .normal[2 as libc::c_int as usize]
                            * down[2 as libc::c_int as usize]
                        < 0 as libc::c_int as libc::c_float)
                    {
                        //direction towards the face center
                        dir[0 as libc::c_int as usize] = facecenter[0 as libc::c_int as usize]
                            - areastart[0 as libc::c_int as usize];
                        dir[1 as libc::c_int as usize] = facecenter[1 as libc::c_int as usize]
                            - areastart[1 as libc::c_int as usize];
                        dir[2 as libc::c_int as usize] = facecenter[2 as libc::c_int as usize]
                            - areastart[2 as libc::c_int as usize];
                        //
                        z = dir[2 as libc::c_int as usize];
                        dir[2 as libc::c_int as usize] =
                            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
                        hordist = VectorLength(
                            dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
                        );
                        if !(hordist == 0.) {
                            //if too far
                            if !(hordist > 2000 as libc::c_int as libc::c_float) {
                                //check the minimal angle of the movement
                                mingrappleangle = 15 as libc::c_int as libc::c_float; //15 degrees
                                if !(((z / hordist) as libc::c_double)
                                    < crate::stdlib::tan(
                                        2 as libc::c_int as libc::c_double
                                            * 3.14159265358979323846f64
                                            * mingrappleangle as libc::c_double
                                            / 360 as libc::c_int as libc::c_double,
                                    ))
                                {
                                    //
                                    start[0 as libc::c_int as usize] =
                                        facecenter[0 as libc::c_int as usize];
                                    start[1 as libc::c_int as usize] =
                                        facecenter[1 as libc::c_int as usize];
                                    start[2 as libc::c_int as usize] =
                                        facecenter[2 as libc::c_int as usize];
                                    end[0 as libc::c_int as usize] = facecenter
                                        [0 as libc::c_int as usize]
                                        + (*crate::src::botlib::be_aas_main::aasworld
                                            .planes
                                            .offset((*face2).planenum as isize))
                                        .normal
                                            [0 as libc::c_int as usize]
                                            * -(500 as libc::c_int) as libc::c_float;
                                    end[1 as libc::c_int as usize] = facecenter
                                        [1 as libc::c_int as usize]
                                        + (*crate::src::botlib::be_aas_main::aasworld
                                            .planes
                                            .offset((*face2).planenum as isize))
                                        .normal
                                            [1 as libc::c_int as usize]
                                            * -(500 as libc::c_int) as libc::c_float;
                                    end[2 as libc::c_int as usize] = facecenter
                                        [2 as libc::c_int as usize]
                                        + (*crate::src::botlib::be_aas_main::aasworld
                                            .planes
                                            .offset((*face2).planenum as isize))
                                        .normal
                                            [2 as libc::c_int as usize]
                                            * -(500 as libc::c_int) as libc::c_float;
                                    //
                                    bsptrace = crate::src::botlib::be_aas_bspq3::AAS_Trace(
                                        start.as_mut_ptr(),
                                        0 as *mut crate::src::qcommon::q_shared::vec_t,
                                        0 as *mut crate::src::qcommon::q_shared::vec_t,
                                        end.as_mut_ptr(),
                                        0 as libc::c_int,
                                        1 as libc::c_int,
                                    );
                                    //the grapple won't stick to the sky and the grapple point should be near the AAS wall
                                    if !(bsptrace.surface.flags & 0x4 as libc::c_int != 0
                                        || bsptrace.fraction * 500 as libc::c_int as libc::c_float
                                            > 32 as libc::c_int as libc::c_float)
                                    {
                                        //trace a full bounding box from the area center on the ground to
                                        //the center of the face
                                        dir[0 as libc::c_int as usize] = facecenter
                                            [0 as libc::c_int as usize]
                                            - areastart[0 as libc::c_int as usize];
                                        dir[1 as libc::c_int as usize] = facecenter
                                            [1 as libc::c_int as usize]
                                            - areastart[1 as libc::c_int as usize];
                                        dir[2 as libc::c_int as usize] = facecenter
                                            [2 as libc::c_int as usize]
                                            - areastart[2 as libc::c_int as usize];
                                        crate::src::qcommon::q_math::VectorNormalize(
                                            dir.as_mut_ptr(),
                                        );
                                        start[0 as libc::c_int as usize] = areastart
                                            [0 as libc::c_int as usize]
                                            + dir[0 as libc::c_int as usize]
                                                * 4 as libc::c_int as libc::c_float;
                                        start[1 as libc::c_int as usize] = areastart
                                            [1 as libc::c_int as usize]
                                            + dir[1 as libc::c_int as usize]
                                                * 4 as libc::c_int as libc::c_float;
                                        start[2 as libc::c_int as usize] = areastart
                                            [2 as libc::c_int as usize]
                                            + dir[2 as libc::c_int as usize]
                                                * 4 as libc::c_int as libc::c_float;
                                        end[0 as libc::c_int as usize] =
                                            bsptrace.endpos[0 as libc::c_int as usize];
                                        end[1 as libc::c_int as usize] =
                                            bsptrace.endpos[1 as libc::c_int as usize];
                                        end[2 as libc::c_int as usize] =
                                            bsptrace.endpos[2 as libc::c_int as usize];
                                        trace =
                                            crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(
                                                start.as_mut_ptr(),
                                                end.as_mut_ptr(),
                                                2 as libc::c_int,
                                                -(1 as libc::c_int),
                                            );
                                        dir[0 as libc::c_int as usize] = trace.endpos
                                            [0 as libc::c_int as usize]
                                            - facecenter[0 as libc::c_int as usize];
                                        dir[1 as libc::c_int as usize] = trace.endpos
                                            [1 as libc::c_int as usize]
                                            - facecenter[1 as libc::c_int as usize];
                                        dir[2 as libc::c_int as usize] = trace.endpos
                                            [2 as libc::c_int as usize]
                                            - facecenter[2 as libc::c_int as usize];
                                        if !(VectorLength(dir.as_mut_ptr()
                                            as *const crate::src::qcommon::q_shared::vec_t)
                                            > 24 as libc::c_int as libc::c_float)
                                        {
                                            //
                                            start[0 as libc::c_int as usize] =
                                                trace.endpos[0 as libc::c_int as usize];
                                            start[1 as libc::c_int as usize] =
                                                trace.endpos[1 as libc::c_int as usize];
                                            start[2 as libc::c_int as usize] =
                                                trace.endpos[2 as libc::c_int as usize];
                                            end[0 as libc::c_int as usize] =
                                                trace.endpos[0 as libc::c_int as usize];
                                            end[1 as libc::c_int as usize] =
                                                trace.endpos[1 as libc::c_int as usize];
                                            end[2 as libc::c_int as usize] =
                                                trace.endpos[2 as libc::c_int as usize];
                                            end[2 as libc::c_int as usize] -=
                                                AAS_FallDamageDistance() as libc::c_float;
                                            trace =
                                                crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(start.as_mut_ptr(),
                                                                    end.as_mut_ptr(),
                                                                    2 as
                                                                        libc::c_int,
                                                                    -(1 as
                                                                          libc::c_int));
                                            if !(trace.fraction
                                                >= 1 as libc::c_int as libc::c_float)
                                            {
                                                //area to end in
                                                areanum =
                                                    crate::src::botlib::be_aas_sample::AAS_PointAreaNum(trace.endpos.as_mut_ptr());
                                                //if not in lava or slime
                                                if !((*crate::src::botlib::be_aas_main::aasworld
                                                    .areasettings
                                                    .offset(areanum as isize))
                                                .contents
                                                    & (4 as libc::c_int | 2 as libc::c_int)
                                                    != 0)
                                                {
                                                    //end if
                                                    //do not go the the source area
                                                    if !(areanum == area1num) {
                                                        //don't create reachabilities if they already exist
                                                        if !(AAS_ReachabilityExists(
                                                            area1num, areanum,
                                                        )
                                                            as u64
                                                            != 0)
                                                        {
                                                            //only end in areas we can stand
                                                            if !(AAS_AreaGrounded(areanum) == 0) {
                                                                //never go through cluster portals!!
                                                                numareas =
                                                                    crate::src::botlib::be_aas_sample::AAS_TraceAreas(areastart.as_mut_ptr(),
                                                                                   bsptrace.endpos.as_mut_ptr(),
                                                                                   areas.as_mut_ptr(),
                                                                                   0
                                                                                       as
                                                                                       *mut crate::src::qcommon::q_shared::vec3_t,
                                                                                   20
                                                                                       as
                                                                                       libc::c_int); //end for
                                                                if !(numareas >= 20 as libc::c_int)
                                                                {
                                                                    j = 0 as libc::c_int;
                                                                    while j < numareas {
                                                                        if (*crate::src::botlib::be_aas_main::aasworld.areasettings.offset(areas[j
                                                                                                                    as
                                                                                                                    usize]
                                                                                                              as
                                                                                                              isize)).contents
                                                                               &
                                                                               8
                                                                                   as
                                                                                   libc::c_int
                                                                               !=
                                                                               0
                                                                           {
                                                                            break
                                                                                ;
                                                                        }
                                                                        j += 1
                                                                    }
                                                                    if !(j < numareas) {
                                                                        //create a new reachability link
                                                                        lreach =
                                                                            AAS_AllocReachability();
                                                                        if lreach.is_null() {
                                                                            return crate::src::qcommon::q_shared::qfalse
                                                                                       as
                                                                                       libc::c_int;
                                                                        }
                                                                        (*lreach).areanum = areanum;
                                                                        (*lreach).facenum =
                                                                            face2num;
                                                                        (*lreach).edgenum =
                                                                            0 as libc::c_int;
                                                                        (*lreach).start[0
                                                                            as libc::c_int
                                                                            as usize] = areastart[0
                                                                            as libc::c_int
                                                                            as usize];
                                                                        (*lreach).start[1
                                                                            as libc::c_int
                                                                            as usize] = areastart[1
                                                                            as libc::c_int
                                                                            as usize];
                                                                        (*lreach).start[2
                                                                            as libc::c_int
                                                                            as usize] = areastart[2
                                                                            as libc::c_int
                                                                            as usize];
                                                                        //VectorCopy(facecenter, lreach->end);
                                                                        (*lreach).end[0
                                                                            as libc::c_int
                                                                            as usize] = bsptrace
                                                                            .endpos
                                                                            [0 as libc::c_int
                                                                                as usize];
                                                                        (*lreach).end[1
                                                                            as libc::c_int
                                                                            as usize] = bsptrace
                                                                            .endpos
                                                                            [1 as libc::c_int
                                                                                as usize];
                                                                        (*lreach).end[2
                                                                            as libc::c_int
                                                                            as usize] = bsptrace
                                                                            .endpos
                                                                            [2 as libc::c_int
                                                                                as usize];
                                                                        (*lreach).traveltype =
                                                                            14 as libc::c_int;
                                                                        dir[0 as libc::c_int
                                                                            as usize] = (*lreach)
                                                                            .end
                                                                            [0 as libc::c_int
                                                                                as usize]
                                                                            - (*lreach).start[0
                                                                                as libc::c_int
                                                                                as usize];
                                                                        dir[1 as libc::c_int
                                                                            as usize] = (*lreach)
                                                                            .end
                                                                            [1 as libc::c_int
                                                                                as usize]
                                                                            - (*lreach).start[1
                                                                                as libc::c_int
                                                                                as usize];
                                                                        dir[2 as libc::c_int
                                                                            as usize] = (*lreach)
                                                                            .end
                                                                            [2 as libc::c_int
                                                                                as usize]
                                                                            - (*lreach).start[2
                                                                                as libc::c_int
                                                                                as usize];
                                                                        (*lreach).traveltime
                                                                            =
                                                                            (crate::src::botlib::be_aas_move::aassettings.rs_startgrapple
                                                                                 as
                                                                                 libc::c_double
                                                                                 +
                                                                                 VectorLength(dir.as_mut_ptr()
                                                                                                  as
                                                                                                  *const crate::src::qcommon::q_shared::vec_t)
                                                                                     as
                                                                                     libc::c_double
                                                                                     *
                                                                                     0.25f64)
                                                                                as
                                                                                libc::c_ushort;
                                                                        (*lreach).next =
                                                                            *areareachability
                                                                                .offset(
                                                                                    area1num
                                                                                        as isize,
                                                                                );
                                                                        let ref mut fresh24 =
                                                                            *areareachability
                                                                                .offset(
                                                                                    area1num
                                                                                        as isize,
                                                                                );
                                                                        *fresh24 = lreach;
                                                                        //
                                                                        reach_grapple += 1
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    //
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//end of the function AAS_Reachability_Grapple
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_SetWeaponJumpAreaFlags() {
    let mut ent: libc::c_int = 0; //end for
    let mut i: libc::c_int = 0;
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [
        -(15 as libc::c_int) as crate::src::qcommon::q_shared::vec_t,
        -(15 as libc::c_int) as crate::src::qcommon::q_shared::vec_t,
        -(15 as libc::c_int) as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [
        15 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        15 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        15 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut areanum: libc::c_int = 0;
    let mut weaponjumpareas: libc::c_int = 0;
    let mut spawnflags: libc::c_int = 0;
    let mut classname: [libc::c_char; 128] = [0; 128];
    weaponjumpareas = 0 as libc::c_int;
    ent = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(0 as libc::c_int);
    while ent != 0 {
        if !(crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey(
            ent,
            b"classname\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            classname.as_mut_ptr(),
            128 as libc::c_int,
        ) == 0)
        {
            if crate::stdlib::strcmp(
                classname.as_mut_ptr(),
                b"item_armor_body\x00" as *const u8 as *const libc::c_char,
            ) == 0
                || crate::stdlib::strcmp(
                    classname.as_mut_ptr(),
                    b"item_armor_combat\x00" as *const u8 as *const libc::c_char,
                ) == 0
                || crate::stdlib::strcmp(
                    classname.as_mut_ptr(),
                    b"item_health_mega\x00" as *const u8 as *const libc::c_char,
                ) == 0
                || crate::stdlib::strcmp(
                    classname.as_mut_ptr(),
                    b"weapon_grenadelauncher\x00" as *const u8 as *const libc::c_char,
                ) == 0
                || crate::stdlib::strcmp(
                    classname.as_mut_ptr(),
                    b"weapon_rocketlauncher\x00" as *const u8 as *const libc::c_char,
                ) == 0
                || crate::stdlib::strcmp(
                    classname.as_mut_ptr(),
                    b"weapon_lightning\x00" as *const u8 as *const libc::c_char,
                ) == 0
                || crate::stdlib::strcmp(
                    classname.as_mut_ptr(),
                    b"weapon_plasmagun\x00" as *const u8 as *const libc::c_char,
                ) == 0
                || crate::stdlib::strcmp(
                    classname.as_mut_ptr(),
                    b"weapon_railgun\x00" as *const u8 as *const libc::c_char,
                ) == 0
                || crate::stdlib::strcmp(
                    classname.as_mut_ptr(),
                    b"weapon_bfg\x00" as *const u8 as *const libc::c_char,
                ) == 0
                || crate::stdlib::strcmp(
                    classname.as_mut_ptr(),
                    b"item_quad\x00" as *const u8 as *const libc::c_char,
                ) == 0
                || crate::stdlib::strcmp(
                    classname.as_mut_ptr(),
                    b"item_regen\x00" as *const u8 as *const libc::c_char,
                ) == 0
                || crate::stdlib::strcmp(
                    classname.as_mut_ptr(),
                    b"item_invulnerability\x00" as *const u8 as *const libc::c_char,
                ) == 0
            {
                if crate::src::botlib::be_aas_bspq3::AAS_VectorForBSPEpairKey(
                    ent,
                    b"origin\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    origin.as_mut_ptr(),
                ) != 0
                {
                    spawnflags = 0 as libc::c_int;
                    crate::src::botlib::be_aas_bspq3::AAS_IntForBSPEpairKey(
                        ent,
                        b"spawnflags\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        &mut spawnflags,
                    );
                    //if not a stationary item
                    if spawnflags & 1 as libc::c_int == 0 {
                        if crate::src::botlib::be_aas_move::AAS_DropToFloor(
                            origin.as_mut_ptr(),
                            mins.as_mut_ptr(),
                            maxs.as_mut_ptr(),
                        ) == 0
                        {
                            botimport.Print.expect("non-null function pointer")(
                                1 as libc::c_int,
                                b"%s in solid at (%1.1f %1.1f %1.1f)\n\x00" as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char,
                                classname.as_mut_ptr(),
                                origin[0 as libc::c_int as usize] as libc::c_double,
                                origin[1 as libc::c_int as usize] as libc::c_double,
                                origin[2 as libc::c_int as usize] as libc::c_double,
                            ); //end if
                        }
                        //end if
                    }
                    //areanum = AAS_PointAreaNum(origin);
                    areanum = AAS_BestReachableArea(
                        origin.as_mut_ptr(),
                        mins.as_mut_ptr(),
                        maxs.as_mut_ptr(),
                        origin.as_mut_ptr(),
                    );
                    //the bot may rocket jump towards this area
                    (*crate::src::botlib::be_aas_main::aasworld
                        .areasettings
                        .offset(areanum as isize))
                    .areaflags |= 8192 as libc::c_int;
                    //
                    //if (!AAS_AreaGrounded(areanum))
                    //	botimport.Print(PRT_MESSAGE, "area not grounded\n");
                    //
                    weaponjumpareas += 1
                }
                //end if
            }
        }
        ent = crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity(ent)
        //end if
    } //end for
    i = 1 as libc::c_int;
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        if (*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .contents
            & 128 as libc::c_int
            != 0
        {
            (*crate::src::botlib::be_aas_main::aasworld
                .areasettings
                .offset(i as isize))
            .areaflags |= 8192 as libc::c_int;
            weaponjumpareas += 1
        }
        i += 1
        //end if
    }
    botimport.Print.expect("non-null function pointer")(
        1 as libc::c_int,
        b"%d weapon jump areas\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        weaponjumpareas,
    );
}
//end of the function AAS_SetWeaponJumpAreaFlags
//===========================================================================
// create a possible weapon jump reachability from area1 to area2
//
// check if there's a cool item in the second area
// check if area1 is lower than area2
// check if the bot can rocketjump from area1 to area2
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Reachability_WeaponJump(
    mut area1num: libc::c_int,
    mut area2num: libc::c_int,
) -> libc::c_int {
    let mut face2num: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut visualize: libc::c_int = 0;
    let mut speed: libc::c_float = 0.;
    let mut zvel: libc::c_float = 0.;
    //float hordist;
    let mut face2: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t; // teststart;
    let mut area1: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut area2: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut areastart: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut facecenter: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut cmdmove: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut move_0: crate::be_aas_h::aas_clientmove_t = crate::be_aas_h::aas_clientmove_t {
        endpos: [0.; 3],
        endarea: 0,
        velocity: [0.; 3],
        trace: crate::be_aas_h::aas_trace_t {
            startsolid: crate::src::qcommon::q_shared::qfalse,
            fraction: 0.,
            endpos: [0.; 3],
            ent: 0,
            lastarea: 0,
            area: 0,
            planenum: 0,
        },
        presencetype: 0,
        stopevent: 0,
        endcontents: 0,
        time: 0.,
        frames: 0,
    };
    let mut trace: crate::be_aas_h::aas_trace_t = crate::be_aas_h::aas_trace_t {
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        ent: 0,
        lastarea: 0,
        area: 0,
        planenum: 0,
    };
    visualize = crate::src::qcommon::q_shared::qfalse as libc::c_int;
    //	if (area1num == 4436 && area2num == 4318)
    //	{
    //		visualize = qtrue;
    //	}
    if AAS_AreaGrounded(area1num) == 0 || AAS_AreaSwim(area1num) != 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    if AAS_AreaGrounded(area2num) == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //NOTE: only weapon jump towards areas with an interesting item in it??
    if (*crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(area2num as isize))
    .areaflags
        & 8192 as libc::c_int
        == 0
    {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //
    area1 = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(area1num as isize) as *mut crate::aasfile_h::aas_area_t;
    area2 = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(area2num as isize) as *mut crate::aasfile_h::aas_area_t;
    //don't weapon jump towards way lower areas
    if (*area2).maxs[2 as libc::c_int as usize] < (*area1).mins[2 as libc::c_int as usize] {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //
    start[0 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(area1num as isize))
    .center[0 as libc::c_int as usize];
    start[1 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(area1num as isize))
    .center[1 as libc::c_int as usize];
    start[2 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(area1num as isize))
    .center[2 as libc::c_int as usize];
    //if not a swim area
    if crate::src::botlib::be_aas_sample::AAS_PointAreaNum(start.as_mut_ptr()) == 0 {
        crate::src::botlib::l_log::Log_Write(
            b"area %d center %f %f %f in solid?\r\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            area1num,
            start[0 as libc::c_int as usize] as libc::c_double,
            start[1 as libc::c_int as usize] as libc::c_double,
            start[2 as libc::c_int as usize] as libc::c_double,
        );
    }
    end[0 as libc::c_int as usize] = start[0 as libc::c_int as usize];
    end[1 as libc::c_int as usize] = start[1 as libc::c_int as usize];
    end[2 as libc::c_int as usize] = start[2 as libc::c_int as usize];
    end[2 as libc::c_int as usize] -= 1000 as libc::c_int as libc::c_float;
    trace = crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(
        start.as_mut_ptr(),
        end.as_mut_ptr(),
        4 as libc::c_int,
        -(1 as libc::c_int),
    );
    if trace.startsolid as u64 != 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    areastart[0 as libc::c_int as usize] = trace.endpos[0 as libc::c_int as usize];
    areastart[1 as libc::c_int as usize] = trace.endpos[1 as libc::c_int as usize];
    areastart[2 as libc::c_int as usize] = trace.endpos[2 as libc::c_int as usize];
    //
    //areastart is now the start point
    //
    i = 0 as libc::c_int; //end for
    while i < (*area2).numfaces {
        face2num = *crate::src::botlib::be_aas_main::aasworld
            .faceindex
            .offset(((*area2).firstface + i) as isize);
        face2 = &mut *crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(
                (crate::stdlib::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(
                    face2num,
                ) as isize,
            ) as *mut crate::aasfile_h::aas_face_t;
        //end for
        //if it is not a solid face
        if !((*face2).faceflags & 4 as libc::c_int == 0) {
            //get the center of the face
            AAS_FaceCenter(face2num, facecenter.as_mut_ptr());
            //only go higher up with weapon jumps
            if !(facecenter[2 as libc::c_int as usize]
                < areastart[2 as libc::c_int as usize] + 64 as libc::c_int as libc::c_float)
            {
                //NOTE: set to 2 to allow bfg jump reachabilities
                n = 0 as libc::c_int;
                while n < 1 as libc::c_int {
                    //get the rocket jump z velocity
                    if n != 0 {
                        zvel = crate::src::botlib::be_aas_move::AAS_BFGJumpZVelocity(
                            areastart.as_mut_ptr(),
                        )
                    } else {
                        zvel = crate::src::botlib::be_aas_move::AAS_RocketJumpZVelocity(
                            areastart.as_mut_ptr(),
                        )
                    }
                    //end if
                    ret = crate::src::botlib::be_aas_move::AAS_HorizontalVelocityForJump(
                        zvel,
                        areastart.as_mut_ptr(),
                        facecenter.as_mut_ptr(),
                        &mut speed,
                    );
                    if ret != 0 && speed < 300 as libc::c_int as libc::c_float {
                        //get the horizontal speed for the jump, if it isn't possible to calculate this
                        //speed (the jump is not possible) then there's no jump reachability created
                        //direction towards the face center
                        dir[0 as libc::c_int as usize] = facecenter[0 as libc::c_int as usize]
                            - areastart[0 as libc::c_int as usize];
                        dir[1 as libc::c_int as usize] = facecenter[1 as libc::c_int as usize]
                            - areastart[1 as libc::c_int as usize];
                        dir[2 as libc::c_int as usize] = facecenter[2 as libc::c_int as usize]
                            - areastart[2 as libc::c_int as usize];
                        dir[2 as libc::c_int as usize] =
                            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
                        //end if
                        //hordist = VectorNormalize(dir);
                        //if (hordist < 1.6 * (facecenter[2] - areastart[2]))
                        //get command movement
                        cmdmove[0 as libc::c_int as usize] = dir[0 as libc::c_int as usize] * speed;
                        cmdmove[1 as libc::c_int as usize] = dir[1 as libc::c_int as usize] * speed;
                        cmdmove[2 as libc::c_int as usize] = dir[2 as libc::c_int as usize] * speed;
                        velocity[0 as libc::c_int as usize] =
                            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
                        velocity[1 as libc::c_int as usize] =
                            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
                        velocity[2 as libc::c_int as usize] = zvel;
                        crate::src::botlib::be_aas_move::AAS_PredictClientMovement(
                            &mut move_0,
                            -(1 as libc::c_int),
                            areastart.as_mut_ptr(),
                            2 as libc::c_int,
                            crate::src::qcommon::q_shared::qtrue as libc::c_int,
                            velocity.as_mut_ptr(),
                            cmdmove.as_mut_ptr(),
                            30 as libc::c_int,
                            30 as libc::c_int,
                            0.1f32,
                            4 as libc::c_int
                                | 8 as libc::c_int
                                | 16 as libc::c_int
                                | 32 as libc::c_int
                                | 128 as libc::c_int
                                | 1 as libc::c_int
                                | 1024 as libc::c_int,
                            area2num,
                            visualize,
                        );
                        if move_0.frames < 30 as libc::c_int
                            && move_0.stopevent
                                & (8 as libc::c_int | 16 as libc::c_int | 32 as libc::c_int)
                                == 0
                            && move_0.stopevent & (1024 as libc::c_int | 128 as libc::c_int) != 0
                        {
                            //end if
                            /*
                            //get command movement
                            VectorScale(dir, speed, velocity);
                            velocity[2] = zvel;
                            VectorSet(cmdmove, 0, 0, 0);
                            */
                            //
                            //create a rocket or bfg jump reachability from area1 to area2
                            lreach = AAS_AllocReachability(); //end else
                            if lreach.is_null() {
                                return crate::src::qcommon::q_shared::qfalse as libc::c_int;
                            } //end if
                            (*lreach).areanum = area2num;
                            (*lreach).facenum = 0 as libc::c_int;
                            (*lreach).edgenum = 0 as libc::c_int;
                            (*lreach).start[0 as libc::c_int as usize] =
                                areastart[0 as libc::c_int as usize];
                            (*lreach).start[1 as libc::c_int as usize] =
                                areastart[1 as libc::c_int as usize];
                            (*lreach).start[2 as libc::c_int as usize] =
                                areastart[2 as libc::c_int as usize];
                            (*lreach).end[0 as libc::c_int as usize] =
                                facecenter[0 as libc::c_int as usize];
                            (*lreach).end[1 as libc::c_int as usize] =
                                facecenter[1 as libc::c_int as usize];
                            (*lreach).end[2 as libc::c_int as usize] =
                                facecenter[2 as libc::c_int as usize];
                            if n != 0 {
                                (*lreach).traveltype = 13 as libc::c_int;
                                (*lreach).traveltime = crate::src::botlib::be_aas_move::aassettings
                                    .rs_bfgjump
                                    as libc::c_ushort
                            } else {
                                (*lreach).traveltype = 12 as libc::c_int;
                                (*lreach).traveltime = crate::src::botlib::be_aas_move::aassettings
                                    .rs_rocketjump
                                    as libc::c_ushort
                            }
                            (*lreach).next = *areareachability.offset(area1num as isize);
                            let ref mut fresh25 = *areareachability.offset(area1num as isize);
                            *fresh25 = lreach;
                            //
                            reach_rocketjump += 1;
                            return crate::src::qcommon::q_shared::qtrue as libc::c_int;
                        }
                    }
                    n += 1
                }
            }
        }
        i += 1
    }
    //if prediction time wasn't enough to fully predict the movement
    //don't enter slime or lava and don't fall from too high
    //
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//end of the function AAS_Reachability_WeaponJump
//===========================================================================
// calculates additional walk off ledge reachabilities for the given area
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Reachability_WalkOffLedge(mut areanum: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut areas: [libc::c_int; 10] = [0; 10];
    let mut numareas: libc::c_int = 0;
    let mut face1num: libc::c_int = 0;
    let mut face2num: libc::c_int = 0;
    let mut face3num: libc::c_int = 0;
    let mut edge1num: libc::c_int = 0;
    let mut edge2num: libc::c_int = 0;
    let mut edge3num: libc::c_int = 0;
    let mut otherareanum: libc::c_int = 0;
    let mut gap: libc::c_int = 0;
    let mut reachareanum: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut area: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut area2: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut face1: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut face2: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut face3: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut edge: *mut crate::aasfile_h::aas_edge_t = 0 as *mut crate::aasfile_h::aas_edge_t;
    let mut plane: *mut crate::aasfile_h::aas_plane_t = 0 as *mut crate::aasfile_h::aas_plane_t;
    let mut v1: *mut crate::src::qcommon::q_shared::vec_t =
        0 as *mut crate::src::qcommon::q_shared::vec_t;
    let mut v2: *mut crate::src::qcommon::q_shared::vec_t =
        0 as *mut crate::src::qcommon::q_shared::vec_t;
    let mut sharededgevec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut mid: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut testend: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut trace: crate::be_aas_h::aas_trace_t = crate::be_aas_h::aas_trace_t {
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        ent: 0,
        lastarea: 0,
        area: 0,
        planenum: 0,
    };
    if AAS_AreaGrounded(areanum) == 0 || AAS_AreaSwim(areanum) != 0 {
        return;
    }
    //
    area = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize) as *mut crate::aasfile_h::aas_area_t;
    //
    i = 0 as libc::c_int;
    while i < (*area).numfaces {
        face1num = *crate::src::botlib::be_aas_main::aasworld
            .faceindex
            .offset(((*area).firstface + i) as isize);
        face1 = &mut *crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(
                (crate::stdlib::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(
                    face1num,
                ) as isize,
            ) as *mut crate::aasfile_h::aas_face_t;
        //end for
        //face 1 must be a ground face
        if !((*face1).faceflags & 4 as libc::c_int == 0) {
            //go through all the edges of this ground face
            k = 0 as libc::c_int;
            while k < (*face1).numedges {
                edge1num = *crate::src::botlib::be_aas_main::aasworld
                    .edgeindex
                    .offset(((*face1).firstedge + k) as isize);
                //end for
                j = 0 as libc::c_int;
                while j < (*area).numfaces {
                    face2num = *crate::src::botlib::be_aas_main::aasworld
                        .faceindex
                        .offset(((*area).firstface + j) as isize);
                    face2 = &mut *crate::src::botlib::be_aas_main::aasworld.faces.offset(
                        (crate::stdlib::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(
                            face2num,
                        ) as isize,
                    ) as *mut crate::aasfile_h::aas_face_t;
                    //find another not ground face using this same edge
                    //end for
                    //face 2 may not be a ground face
                    if !((*face2).faceflags & 4 as libc::c_int != 0) {
                        //compare all the edges
                        l = 0 as libc::c_int;
                        while l < (*face2).numedges {
                            edge2num = *crate::src::botlib::be_aas_main::aasworld
                                .edgeindex
                                .offset(((*face2).firstedge + l) as isize);
                            if crate::stdlib::abs(edge1num) == crate::stdlib::abs(edge2num) {
                                //end if
                                //get the area at the other side of the face
                                if (*face2).frontarea == areanum {
                                    otherareanum = (*face2).backarea
                                } else {
                                    otherareanum = (*face2).frontarea
                                }
                                //
                                area2 = &mut *crate::src::botlib::be_aas_main::aasworld
                                    .areas
                                    .offset(otherareanum as isize)
                                    as *mut crate::aasfile_h::aas_area_t;
                                //if the other area is grounded!
                                if (*crate::src::botlib::be_aas_main::aasworld
                                    .areasettings
                                    .offset(otherareanum as isize))
                                .areaflags
                                    & 1 as libc::c_int
                                    != 0
                                {
                                    //end if
                                    //check for a possible gap
                                    gap = crate::src::qcommon::q_shared::qfalse as libc::c_int; //end for
                                    n = 0 as libc::c_int;
                                    while n < (*area2).numfaces {
                                        face3num = *crate::src::botlib::be_aas_main::aasworld
                                            .faceindex
                                            .offset(((*area2).firstface + n) as isize);
                                        //may not be the shared face of the two areas
                                        if !(crate::stdlib::abs(face3num)
                                            == crate::stdlib::abs(face2num))
                                        {
                                            //
                                            face3 = &mut *crate::src::botlib::be_aas_main::aasworld
                                                .faces
                                                .offset((crate::stdlib::abs
                                                    as unsafe extern "C" fn(
                                                        _: libc::c_int,
                                                    )
                                                        -> libc::c_int)(
                                                    face3num
                                                )
                                                    as isize)
                                                as *mut crate::aasfile_h::aas_face_t;
                                            //find an edge shared by all three faces
                                            m = 0 as libc::c_int; //end for
                                            while m < (*face3).numedges {
                                                edge3num =
                                                    *crate::src::botlib::be_aas_main::aasworld
                                                        .edgeindex
                                                        .offset(((*face3).firstedge + m) as isize);
                                                //end if
                                                if crate::stdlib::abs(edge3num)
                                                    == crate::stdlib::abs(edge1num)
                                                {
                                                    //but the edge should be shared by all three faces
                                                    if (*face3).faceflags & 1 as libc::c_int == 0 {
                                                        gap = crate::src::qcommon::q_shared::qtrue
                                                            as libc::c_int; //end if
                                                        break;
                                                    } else if (*face3).faceflags & 4 as libc::c_int
                                                        != 0
                                                    {
                                                        gap = crate::src::qcommon::q_shared::qfalse
                                                            as libc::c_int;
                                                        break;
                                                    } else {
                                                        //
                                                        //end if
                                                        //FIXME: there are more situations to be handled
                                                        gap = crate::src::qcommon::q_shared::qtrue
                                                            as libc::c_int;
                                                        break;
                                                    }
                                                } else {
                                                    m += 1
                                                }
                                            }
                                            if m < (*face3).numedges {
                                                break;
                                            }
                                        }
                                        n += 1
                                    }
                                    if gap == 0 {
                                        break;
                                    }
                                }
                                //check for a walk off ledge reachability
                                edge = &mut *crate::src::botlib::be_aas_main::aasworld.edges.offset(
                                    (crate::stdlib::abs
                                        as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(
                                        edge1num,
                                    ) as isize,
                                )
                                    as *mut crate::aasfile_h::aas_edge_t;
                                side = (edge1num < 0 as libc::c_int) as libc::c_int;
                                //
                                v1 = (*crate::src::botlib::be_aas_main::aasworld
                                    .vertexes
                                    .offset((*edge).v[side as usize] as isize))
                                .as_mut_ptr();
                                v2 = (*crate::src::botlib::be_aas_main::aasworld.vertexes.offset(
                                    (*edge).v[(side == 0) as libc::c_int as usize] as isize,
                                ))
                                .as_mut_ptr();
                                //
                                plane = &mut *crate::src::botlib::be_aas_main::aasworld
                                    .planes
                                    .offset((*face1).planenum as isize)
                                    as *mut crate::aasfile_h::aas_plane_t;
                                //get the points really into the areas
                                sharededgevec[0 as libc::c_int as usize] = *v2
                                    .offset(0 as libc::c_int as isize)
                                    - *v1.offset(0 as libc::c_int as isize);
                                sharededgevec[1 as libc::c_int as usize] = *v2
                                    .offset(1 as libc::c_int as isize)
                                    - *v1.offset(1 as libc::c_int as isize);
                                sharededgevec[2 as libc::c_int as usize] = *v2
                                    .offset(2 as libc::c_int as isize)
                                    - *v1.offset(2 as libc::c_int as isize);
                                CrossProduct(
                                    (*plane).normal.as_mut_ptr()
                                        as *const crate::src::qcommon::q_shared::vec_t,
                                    sharededgevec.as_mut_ptr()
                                        as *const crate::src::qcommon::q_shared::vec_t,
                                    dir.as_mut_ptr(),
                                );
                                crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
                                //
                                mid[0 as libc::c_int as usize] = *v1
                                    .offset(0 as libc::c_int as isize)
                                    + *v2.offset(0 as libc::c_int as isize);
                                mid[1 as libc::c_int as usize] = *v1
                                    .offset(1 as libc::c_int as isize)
                                    + *v2.offset(1 as libc::c_int as isize);
                                mid[2 as libc::c_int as usize] = *v1
                                    .offset(2 as libc::c_int as isize)
                                    + *v2.offset(2 as libc::c_int as isize);
                                mid[0 as libc::c_int as usize] =
                                    (mid[0 as libc::c_int as usize] as libc::c_double * 0.5f64)
                                        as crate::src::qcommon::q_shared::vec_t;
                                mid[1 as libc::c_int as usize] =
                                    (mid[1 as libc::c_int as usize] as libc::c_double * 0.5f64)
                                        as crate::src::qcommon::q_shared::vec_t;
                                mid[2 as libc::c_int as usize] =
                                    (mid[2 as libc::c_int as usize] as libc::c_double * 0.5f64)
                                        as crate::src::qcommon::q_shared::vec_t;
                                mid[0 as libc::c_int as usize] = mid[0 as libc::c_int as usize]
                                    + dir[0 as libc::c_int as usize]
                                        * 8 as libc::c_int as libc::c_float;
                                mid[1 as libc::c_int as usize] = mid[1 as libc::c_int as usize]
                                    + dir[1 as libc::c_int as usize]
                                        * 8 as libc::c_int as libc::c_float;
                                mid[2 as libc::c_int as usize] = mid[2 as libc::c_int as usize]
                                    + dir[2 as libc::c_int as usize]
                                        * 8 as libc::c_int as libc::c_float;
                                //
                                testend[0 as libc::c_int as usize] = mid[0 as libc::c_int as usize];
                                testend[1 as libc::c_int as usize] = mid[1 as libc::c_int as usize];
                                testend[2 as libc::c_int as usize] = mid[2 as libc::c_int as usize];
                                testend[2 as libc::c_int as usize] -=
                                    1000 as libc::c_int as libc::c_float;
                                trace = crate::src::botlib::be_aas_sample::AAS_TraceClientBBox(
                                    mid.as_mut_ptr(),
                                    testend.as_mut_ptr(),
                                    4 as libc::c_int,
                                    -(1 as libc::c_int),
                                );
                                //
                                if trace.startsolid as u64 != 0 {
                                    //end if
                                    //Log_Write("area %d: trace.startsolid\r\n", areanum);
                                    break; //end if
                                } else {
                                    reachareanum =
                                        crate::src::botlib::be_aas_sample::AAS_PointAreaNum(
                                            trace.endpos.as_mut_ptr(),
                                        );
                                    if reachareanum == areanum {
                                        //Log_Write("area %d: same area\r\n", areanum);
                                        break; //end if
                                    } else if AAS_ReachabilityExists(areanum, reachareanum) as u64
                                        != 0
                                    {
                                        //Log_Write("area %d: reachability already exists\r\n", areanum);
                                        break; //end if
                                    } else if AAS_AreaGrounded(reachareanum) == 0
                                        && AAS_AreaSwim(reachareanum) == 0
                                    {
                                        //Log_Write("area %d, reach area %d: not grounded and not swim\r\n", areanum, reachareanum);
                                        break;
                                    } else {
                                        //
                                        if (*crate::src::botlib::be_aas_main::aasworld
                                            .areasettings
                                            .offset(reachareanum as isize))
                                        .contents
                                            & (4 as libc::c_int | 2 as libc::c_int)
                                            != 0
                                        {
                                            break; //end if
                                        }
                                        //if not going through a cluster portal
                                        numareas =
                                            crate::src::botlib::be_aas_sample::AAS_TraceAreas(
                                                mid.as_mut_ptr(),
                                                testend.as_mut_ptr(),
                                                areas.as_mut_ptr(),
                                                0 as *mut crate::src::qcommon::q_shared::vec3_t,
                                                (::std::mem::size_of::<[libc::c_int; 10]>()
                                                    as libc::c_ulong)
                                                    .wrapping_div(
                                                        ::std::mem::size_of::<libc::c_int>()
                                                            as libc::c_ulong,
                                                    )
                                                    as libc::c_int,
                                            );
                                        p = 0 as libc::c_int;
                                        while p < numareas {
                                            if AAS_AreaClusterPortal(areas[p as usize]) != 0 {
                                                break;
                                            }
                                            p += 1
                                        }
                                        if p < numareas {
                                            break;
                                        }
                                        // if a maximum fall height is set and the bot would fall down further
                                        if crate::src::botlib::be_aas_move::aassettings
                                            .rs_maxfallheight
                                            != 0.
                                            && crate::stdlib::fabs(
                                                (mid[2 as libc::c_int as usize]
                                                    - trace.endpos[2 as libc::c_int as usize])
                                                    as libc::c_double,
                                            ) > crate::src::botlib::be_aas_move::aassettings
                                                .rs_maxfallheight
                                                as libc::c_double
                                        {
                                            break;
                                        }
                                        //
                                        lreach = AAS_AllocReachability(); //end if
                                        if lreach.is_null() {
                                            break; //end if
                                        }
                                        (*lreach).areanum = reachareanum;
                                        (*lreach).facenum = 0 as libc::c_int;
                                        (*lreach).edgenum = edge1num;
                                        (*lreach).start[0 as libc::c_int as usize] =
                                            mid[0 as libc::c_int as usize];
                                        (*lreach).start[1 as libc::c_int as usize] =
                                            mid[1 as libc::c_int as usize];
                                        (*lreach).start[2 as libc::c_int as usize] =
                                            mid[2 as libc::c_int as usize];
                                        (*lreach).end[0 as libc::c_int as usize] =
                                            trace.endpos[0 as libc::c_int as usize];
                                        (*lreach).end[1 as libc::c_int as usize] =
                                            trace.endpos[1 as libc::c_int as usize];
                                        (*lreach).end[2 as libc::c_int as usize] =
                                            trace.endpos[2 as libc::c_int as usize];
                                        (*lreach).traveltype = 7 as libc::c_int;
                                        (*lreach).traveltime =
                                            (crate::src::botlib::be_aas_move::aassettings
                                                .rs_startwalkoffledge
                                                as libc::c_double
                                                + crate::stdlib::fabs(
                                                    (mid[2 as libc::c_int as usize]
                                                        - trace.endpos[2 as libc::c_int as usize])
                                                        as libc::c_double,
                                                ) * 50 as libc::c_int as libc::c_double
                                                    / crate::src::botlib::be_aas_move::aassettings
                                                        .phys_gravity
                                                        as libc::c_double)
                                                as libc::c_ushort;
                                        if AAS_AreaSwim(reachareanum) == 0
                                            && AAS_AreaJumpPad(reachareanum) == 0
                                        {
                                            if AAS_FallDelta(
                                                mid[2 as libc::c_int as usize]
                                                    - trace.endpos[2 as libc::c_int as usize],
                                            ) > crate::src::botlib::be_aas_move::aassettings
                                                .phys_falldelta5
                                            {
                                                (*lreach).traveltime = ((*lreach).traveltime
                                                    as libc::c_float
                                                    + crate::src::botlib::be_aas_move::aassettings
                                                        .rs_falldamage5)
                                                    as libc::c_ushort
                                            } else if AAS_FallDelta(
                                                mid[2 as libc::c_int as usize]
                                                    - trace.endpos[2 as libc::c_int as usize],
                                            )
                                                > crate::src::botlib::be_aas_move::aassettings
                                                    .phys_falldelta10
                                            {
                                                (*lreach).traveltime = ((*lreach).traveltime
                                                    as libc::c_float
                                                    + crate::src::botlib::be_aas_move::aassettings
                                                        .rs_falldamage10)
                                                    as libc::c_ushort
                                            }
                                            //end if
                                        }
                                        (*lreach).next = *areareachability.offset(areanum as isize);
                                        let ref mut fresh26 =
                                            *areareachability.offset(areanum as isize);
                                        *fresh26 = lreach;
                                        //we've got another walk off ledge reachability
                                        reach_walkoffledge += 1
                                    }
                                }
                            }
                            l += 1
                        }
                    }
                    j += 1
                }
                k += 1
            }
        }
        i += 1
    }
    //end for
}
//end of the function AAS_Reachability_WalkOffLedge
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_StoreReachability() {
    let mut i: libc::c_int = 0; //end for
    let mut areasettings: *mut crate::aasfile_h::aas_areasettings_t =
        0 as *mut crate::aasfile_h::aas_areasettings_t;
    let mut lreach: *mut aas_lreachability_t = 0 as *mut aas_lreachability_t;
    let mut reach: *mut crate::aasfile_h::aas_reachability_t =
        0 as *mut crate::aasfile_h::aas_reachability_t;
    if !crate::src::botlib::be_aas_main::aasworld
        .reachability
        .is_null()
    {
        crate::src::botlib::l_memory::FreeMemory(
            crate::src::botlib::be_aas_main::aasworld.reachability as *mut libc::c_void,
        );
    }
    crate::src::botlib::be_aas_main::aasworld.reachability =
        crate::src::botlib::l_memory::GetClearedMemory(
            ((numlreachabilities + 10 as libc::c_int) as libc::c_ulong).wrapping_mul(
                ::std::mem::size_of::<crate::aasfile_h::aas_reachability_t>() as libc::c_ulong,
            ),
        ) as *mut crate::aasfile_h::aas_reachability_t;
    crate::src::botlib::be_aas_main::aasworld.reachabilitysize = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < crate::src::botlib::be_aas_main::aasworld.numareas {
        areasettings = &mut *crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize) as *mut crate::aasfile_h::aas_areasettings_t;
        (*areasettings).firstreachablearea =
            crate::src::botlib::be_aas_main::aasworld.reachabilitysize;
        (*areasettings).numreachableareas = 0 as libc::c_int;
        lreach = *areareachability.offset(i as isize);
        while !lreach.is_null() {
            reach = &mut *crate::src::botlib::be_aas_main::aasworld
                .reachability
                .offset(
                    ((*areasettings).firstreachablearea + (*areasettings).numreachableareas)
                        as isize,
                ) as *mut crate::aasfile_h::aas_reachability_t;
            (*reach).areanum = (*lreach).areanum;
            (*reach).facenum = (*lreach).facenum;
            (*reach).edgenum = (*lreach).edgenum;
            (*reach).start[0 as libc::c_int as usize] = (*lreach).start[0 as libc::c_int as usize];
            (*reach).start[1 as libc::c_int as usize] = (*lreach).start[1 as libc::c_int as usize];
            (*reach).start[2 as libc::c_int as usize] = (*lreach).start[2 as libc::c_int as usize];
            (*reach).end[0 as libc::c_int as usize] = (*lreach).end[0 as libc::c_int as usize];
            (*reach).end[1 as libc::c_int as usize] = (*lreach).end[1 as libc::c_int as usize];
            (*reach).end[2 as libc::c_int as usize] = (*lreach).end[2 as libc::c_int as usize];
            (*reach).traveltype = (*lreach).traveltype;
            (*reach).traveltime = (*lreach).traveltime;
            //
            (*areasettings).numreachableareas += 1;
            lreach = (*lreach).next
        }
        crate::src::botlib::be_aas_main::aasworld.reachabilitysize +=
            (*areasettings).numreachableareas;
        i += 1
    }
    //end for
}
//continue calculating the reachabilities
//end of the function AAS_StoreReachability
//===========================================================================
//
// TRAVEL_WALK					100%	equal floor height + steps
// TRAVEL_CROUCH				100%
// TRAVEL_BARRIERJUMP			100%
// TRAVEL_JUMP					 80%
// TRAVEL_LADDER				100%	+ fall down from ladder + jump up to ladder
// TRAVEL_WALKOFFLEDGE			 90%	walk off very steep walls?
// TRAVEL_SWIM					100%
// TRAVEL_WATERJUMP				100%
// TRAVEL_TELEPORT				100%
// TRAVEL_ELEVATOR				100%
// TRAVEL_GRAPPLEHOOK			100%
// TRAVEL_DOUBLEJUMP			  0%
// TRAVEL_RAMPJUMP				  0%
// TRAVEL_STRAFEJUMP			  0%
// TRAVEL_ROCKETJUMP			100%	(currently limited towards areas with items)
// TRAVEL_BFGJUMP				  0%	(currently disabled)
// TRAVEL_JUMPPAD				100%
// TRAVEL_FUNCBOB				100%
//
// Parameter:			-
// Returns:				true if NOT finished
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ContinueInitReachability(mut time: libc::c_float) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut todo: libc::c_int = 0;
    let mut start_time: libc::c_int = 0;
    static mut framereachability: libc::c_float = 0.;
    static mut reachability_delay: libc::c_float = 0.;
    static mut lastpercentage: libc::c_int = 0;
    if crate::src::botlib::be_aas_main::aasworld.loaded == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //if reachability is calculated for all areas
    if crate::src::botlib::be_aas_main::aasworld.numreachabilityareas
        >= crate::src::botlib::be_aas_main::aasworld.numareas + 2 as libc::c_int
    {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //if starting with area 1 (area 0 is a dummy)
    if crate::src::botlib::be_aas_main::aasworld.numreachabilityareas == 1 as libc::c_int {
        botimport.Print.expect("non-null function pointer")(
            1 as libc::c_int,
            b"calculating reachability...\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ); //end if
        lastpercentage = 0 as libc::c_int;
        framereachability = 2000 as libc::c_int as libc::c_float;
        reachability_delay = 1000 as libc::c_int as libc::c_float
    }
    //number of areas to calculate reachability for this cycle
    todo = crate::src::botlib::be_aas_main::aasworld.numreachabilityareas
        + framereachability as libc::c_int;
    start_time = Sys_MilliSeconds();
    //loop over the areas
    i = crate::src::botlib::be_aas_main::aasworld.numreachabilityareas; //end for
    while i < crate::src::botlib::be_aas_main::aasworld.numareas && i < todo {
        crate::src::botlib::be_aas_main::aasworld.numreachabilityareas += 1;
        //only create jumppad reachabilities from jumppad areas
        if !((*crate::src::botlib::be_aas_main::aasworld
            .areasettings
            .offset(i as isize))
        .contents
            & 128 as libc::c_int
            != 0)
        {
            let mut current_block_14: u64; //end if
                                           //loop over the areas
            j = 1 as libc::c_int; //end for
            while j < crate::src::botlib::be_aas_main::aasworld.numareas {
                if !(i == j) {
                    //never create reachabilities from teleporter or jumppad areas to regular areas
                    if (*crate::src::botlib::be_aas_main::aasworld
                        .areasettings
                        .offset(i as isize))
                    .contents
                        & (64 as libc::c_int | 128 as libc::c_int)
                        != 0
                    {
                        if (*crate::src::botlib::be_aas_main::aasworld
                            .areasettings
                            .offset(j as isize))
                        .contents
                            & (64 as libc::c_int | 128 as libc::c_int)
                            == 0
                        {
                            current_block_14 = 4956146061682418353; //end if
                        } else {
                            current_block_14 = 1109700713171191020;
                        }
                    //end if
                    } else {
                        current_block_14 = 1109700713171191020;
                    }
                    match current_block_14 {
                        4956146061682418353 => {}
                        _ =>
                        //if there already is a reachability link from area i to j
                        {
                            if !(AAS_ReachabilityExists(i, j) as u64 != 0) {
                                //check for a swim reachability
                                if !(AAS_Reachability_Swim(i, j) != 0) {
                                    //check for a simple walk on equal floor height reachability
                                    if !(AAS_Reachability_EqualFloorHeight(i, j) != 0) {
                                        //check for step, barrier, waterjump and walk off ledge reachabilities
                                        if !(AAS_Reachability_Step_Barrier_WaterJump_WalkOffLedge(
                                            i, j,
                                        ) != 0)
                                        {
                                            //check for ladder reachabilities
                                            if !(AAS_Reachability_Ladder(i, j) != 0) {
                                                //check for a jump reachability
                                                (AAS_Reachability_Jump(i, j)) != 0;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                j += 1
            }
            //never create these reachabilities from teleporter or jumppad areas
            if !((*crate::src::botlib::be_aas_main::aasworld
                .areasettings
                .offset(i as isize))
            .contents
                & (64 as libc::c_int | 128 as libc::c_int)
                != 0)
            {
                //end if
                //loop over the areas
                j = 1 as libc::c_int; //end for
                while j < crate::src::botlib::be_aas_main::aasworld.numareas {
                    if !(i == j) {
                        //
                        if !(AAS_ReachabilityExists(i, j) as u64 != 0) {
                            //check for a grapple hook reachability
                            if calcgrapplereach != 0 {
                                AAS_Reachability_Grapple(i, j);
                            }
                            //check for a weapon jump reachability
                            AAS_Reachability_WeaponJump(i, j);
                        }
                    }
                    j += 1
                }
                //if the calculation took more time than the max reachability delay
                if Sys_MilliSeconds() - start_time > reachability_delay as libc::c_int {
                    break;
                }
                //
                if crate::src::botlib::be_aas_main::aasworld.numreachabilityareas
                    * 1000 as libc::c_int
                    / crate::src::botlib::be_aas_main::aasworld.numareas
                    > lastpercentage
                {
                    break;
                }
            }
        }
        i += 1
    }
    //
    if crate::src::botlib::be_aas_main::aasworld.numreachabilityareas
        == crate::src::botlib::be_aas_main::aasworld.numareas
    {
        //end else
        botimport.Print.expect("non-null function pointer")(
            1 as libc::c_int,
            b"\r%6.1f%%\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            100.0f64 as libc::c_float as libc::c_double,
        ); //end if
        botimport.Print.expect("non-null function pointer")(
            1 as libc::c_int,
            b"\nplease wait while storing reachability...\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        crate::src::botlib::be_aas_main::aasworld.numreachabilityareas += 1
    } else if crate::src::botlib::be_aas_main::aasworld.numreachabilityareas
        == crate::src::botlib::be_aas_main::aasworld.numareas + 1 as libc::c_int
    {
        //if this is the last step in the reachability calculations
        i = 1 as libc::c_int; //end if
        while i < crate::src::botlib::be_aas_main::aasworld.numareas {
            //create additional walk off ledge reachabilities for every area
            //end for
            //only create jumppad reachabilities from jumppad areas
            if !((*crate::src::botlib::be_aas_main::aasworld
                .areasettings
                .offset(i as isize))
            .contents
                & 128 as libc::c_int
                != 0)
            {
                AAS_Reachability_WalkOffLedge(i); //end if
            }
            i += 1
        }
        //create jump pad reachabilities
        AAS_Reachability_JumpPad();
        //create teleporter reachabilities
        AAS_Reachability_Teleport();
        //create elevator (func_plat) reachabilities
        AAS_Reachability_Elevator();
        //create func_bobbing reachabilities
        AAS_Reachability_FuncBobbing();
        //
        //*/
        //store all the reachabilities
        AAS_StoreReachability();
        //free the reachability link heap
        AAS_ShutDownReachabilityHeap();
        //
        crate::src::botlib::l_memory::FreeMemory(areareachability as *mut libc::c_void);
        //
        crate::src::botlib::be_aas_main::aasworld.numreachabilityareas += 1;
        //
        botimport.Print.expect("non-null function pointer")(
            1 as libc::c_int,
            b"calculating clusters...\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    } else {
        lastpercentage = crate::src::botlib::be_aas_main::aasworld.numreachabilityareas
            * 1000 as libc::c_int
            / crate::src::botlib::be_aas_main::aasworld.numareas;
        botimport.Print.expect("non-null function pointer")(
            1 as libc::c_int,
            b"\r%6.1f%%\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (lastpercentage as libc::c_float / 10 as libc::c_int as libc::c_float)
                as libc::c_double,
        );
    }
    //not yet finished
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
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
/* ****************************************************************************
 * name:		be_aas_reach.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_reach.h $
 *
 *****************************************************************************/
/* ****************************************************************************
 * name:		be_aas_reach.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_reach.h $
 *
 *****************************************************************************/
//initialize calculating the reachabilities
//end of the function AAS_ContinueInitReachability
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_InitReachability() {
    if crate::src::botlib::be_aas_main::aasworld.loaded == 0 {
        return;
    } //end if
    if crate::src::botlib::be_aas_main::aasworld.reachabilitysize != 0 {
        if crate::src::botlib::l_libvar::LibVarGetValue(
            b"forcereachability\x00" as *const u8 as *const libc::c_char,
        ) as libc::c_int
            == 0
        {
            crate::src::botlib::be_aas_main::aasworld.numreachabilityareas =
                crate::src::botlib::be_aas_main::aasworld.numareas + 2 as libc::c_int;
            return;
        }
        //end if
        //BSPC
    }
    calcgrapplereach = crate::src::botlib::l_libvar::LibVarGetValue(
        b"grapplereach\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    crate::src::botlib::be_aas_main::aasworld.savefile =
        crate::src::qcommon::q_shared::qtrue as libc::c_int;
    //start with area 1 because area zero is a dummy
    crate::src::botlib::be_aas_main::aasworld.numreachabilityareas = 1 as libc::c_int;
    // //aasworld.numreachabilityareas = aasworld.numareas + 1;		//only calculate entity reachabilities
    //setup the heap with reachability links
    AAS_SetupReachabilityHeap();
    //allocate area reachability link array
    areareachability = crate::src::botlib::l_memory::GetClearedMemory(
        (crate::src::botlib::be_aas_main::aasworld.numareas as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut aas_lreachability_t>() as libc::c_ulong),
    ) as *mut *mut aas_lreachability_t;
    //
    AAS_SetWeaponJumpAreaFlags();
}
//end of the function AAS_InitReachable
