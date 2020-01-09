use ::libc;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::cm_local_h::cArea_t;
pub use crate::cm_local_h::cLeaf_t;
pub use crate::cm_local_h::cNode_t;
pub use crate::cm_local_h::cPatch_t;
pub use crate::cm_local_h::cbrush_t;
pub use crate::cm_local_h::cbrushside_t;
pub use crate::cm_local_h::clipMap_t;
pub use crate::cm_local_h::cmodel_s;
pub use crate::cm_local_h::cmodel_t;
pub use crate::qfiles_h::dbrush_t;
pub use crate::qfiles_h::dbrushside_t;
pub use crate::qfiles_h::dheader_t;
pub use crate::qfiles_h::dleaf_t;
pub use crate::qfiles_h::dmodel_t;
pub use crate::qfiles_h::dnode_t;
pub use crate::qfiles_h::dplane_t;
pub use crate::qfiles_h::drawVert_t;
pub use crate::qfiles_h::dshader_t;
pub use crate::qfiles_h::dsurface_t;
pub use crate::qfiles_h::lump_t;
pub use crate::qfiles_h::MST_BAD;
pub use crate::qfiles_h::MST_FLARE;
pub use crate::qfiles_h::MST_PATCH;
pub use crate::qfiles_h::MST_PLANAR;
pub use crate::qfiles_h::MST_TRIANGLE_SOUP;
pub use crate::src::qcommon::cm_patch::patchCollide_s;
pub use crate::src::qcommon::cm_patch::CM_ClearLevelPatches;
pub use crate::src::qcommon::cm_patch::CM_GeneratePatchCollide;
use crate::src::qcommon::common::Com_DPrintf;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Hunk_AllocDebug;
use crate::src::qcommon::cvar::Cvar_Get;
use crate::src::qcommon::files::FS_FreeFile;
use crate::src::qcommon::files::FS_ReadFile;
use crate::src::qcommon::md4::Com_BlockChecksum;
pub use crate::src::qcommon::q_math::SetPlaneSignbits;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::clipHandle_t;
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
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
use crate::stdlib::strcmp;
extern "C" {
    #[no_mangle]
    pub fn CM_FloodAreaConnections();
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed_7 {
    pub i: *mut libc::c_int,
    pub v: *mut libc::c_void,
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
// cmodel.c -- model loading
//BSPC
// to allow boxes to be treated as brush models, we allocate
// some extra indexes along with those needed by the map
#[no_mangle]

pub static mut cm: crate::cm_local_h::clipMap_t = crate::cm_local_h::clipMap_t {
    name: [0; 64],
    numShaders: 0,
    shaders: 0 as *const crate::qfiles_h::dshader_t as *mut crate::qfiles_h::dshader_t,
    numBrushSides: 0,
    brushsides: 0 as *const crate::cm_local_h::cbrushside_t as *mut crate::cm_local_h::cbrushside_t,
    numPlanes: 0,
    planes: 0 as *const crate::src::qcommon::q_shared::cplane_t
        as *mut crate::src::qcommon::q_shared::cplane_t,
    numNodes: 0,
    nodes: 0 as *const crate::cm_local_h::cNode_t as *mut crate::cm_local_h::cNode_t,
    numLeafs: 0,
    leafs: 0 as *const crate::cm_local_h::cLeaf_t as *mut crate::cm_local_h::cLeaf_t,
    numLeafBrushes: 0,
    leafbrushes: 0 as *const libc::c_int as *mut libc::c_int,
    numLeafSurfaces: 0,
    leafsurfaces: 0 as *const libc::c_int as *mut libc::c_int,
    numSubModels: 0,
    cmodels: 0 as *const crate::cm_local_h::cmodel_t as *mut crate::cm_local_h::cmodel_t,
    numBrushes: 0,
    brushes: 0 as *const crate::cm_local_h::cbrush_t as *mut crate::cm_local_h::cbrush_t,
    numClusters: 0,
    clusterBytes: 0,
    visibility: 0 as *const crate::src::qcommon::q_shared::byte
        as *mut crate::src::qcommon::q_shared::byte,
    vised: crate::src::qcommon::q_shared::qfalse,
    numEntityChars: 0,
    entityString: 0 as *const libc::c_char as *mut libc::c_char,
    numAreas: 0,
    areas: 0 as *const crate::cm_local_h::cArea_t as *mut crate::cm_local_h::cArea_t,
    areaPortals: 0 as *const libc::c_int as *mut libc::c_int,
    numSurfaces: 0,
    surfaces: 0 as *const *mut crate::cm_local_h::cPatch_t as *mut *mut crate::cm_local_h::cPatch_t,
    floodvalid: 0,
    checkcount: 0,
};
#[no_mangle]

pub static mut c_pointcontents: libc::c_int = 0;
#[no_mangle]

pub static mut c_traces: libc::c_int = 0;
#[no_mangle]

pub static mut c_brush_traces: libc::c_int = 0;
#[no_mangle]

pub static mut c_patch_traces: libc::c_int = 0;
#[no_mangle]

pub static mut cmod_base: *mut crate::src::qcommon::q_shared::byte =
    0 as *const crate::src::qcommon::q_shared::byte as *mut crate::src::qcommon::q_shared::byte;
#[no_mangle]

pub static mut cm_noAreas: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cm_noCurves: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cm_playerCurveClip: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut box_model: crate::cm_local_h::cmodel_t = crate::cm_local_h::cmodel_t {
    mins: [0.; 3],
    maxs: [0.; 3],
    leaf: crate::cm_local_h::cLeaf_t {
        cluster: 0,
        area: 0,
        firstLeafBrush: 0,
        numLeafBrushes: 0,
        firstLeafSurface: 0,
        numLeafSurfaces: 0,
    },
};
#[no_mangle]

pub static mut box_planes: *mut crate::src::qcommon::q_shared::cplane_t = 0
    as *const crate::src::qcommon::q_shared::cplane_t
    as *mut crate::src::qcommon::q_shared::cplane_t;
#[no_mangle]

pub static mut box_brush: *mut crate::cm_local_h::cbrush_t =
    0 as *const crate::cm_local_h::cbrush_t as *mut crate::cm_local_h::cbrush_t;
/*
===============================================================================

                    MAP LOADING

===============================================================================
*/
/*
=================
CMod_LoadShaders
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadShaders(mut l: *mut crate::qfiles_h::lump_t) {
    let mut in_0: *mut crate::qfiles_h::dshader_t = 0 as *mut crate::qfiles_h::dshader_t;
    let mut out: *mut crate::qfiles_h::dshader_t = 0 as *mut crate::qfiles_h::dshader_t;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void
        as *mut crate::qfiles_h::dshader_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<crate::qfiles_h::dshader_t>() as libc::c_ulong)
        != 0
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"CMod_LoadShaders: funny lump size\x00" as *const u8 as *const libc::c_char,
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::qfiles_h::dshader_t>() as libc::c_ulong)
        as libc::c_int;
    if count < 1 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"Map with no shaders\x00" as *const u8 as *const libc::c_char,
        );
    }
    cm.shaders = crate::src::qcommon::common::Hunk_AllocDebug(
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::qfiles_h::dshader_t>() as libc::c_ulong)
            as libc::c_int,
        crate::src::qcommon::q_shared::h_high,
        b"count * sizeof( *cm.shaders )\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/qcommon/cm_load.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        103 as libc::c_int,
    ) as *mut crate::qfiles_h::dshader_t;
    cm.numShaders = count;
    crate::stdlib::memcpy(
        cm.shaders as *mut libc::c_void,
        in_0 as *const libc::c_void,
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::qfiles_h::dshader_t>() as libc::c_ulong),
    );
    out = cm.shaders;
    i = 0 as libc::c_int;
    while i < count {
        (*out).contentFlags = (*out).contentFlags;
        (*out).surfaceFlags = (*out).surfaceFlags;
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1)
    }
}
/*
=================
CMod_LoadSubmodels
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadSubmodels(mut l: *mut crate::qfiles_h::lump_t) {
    let mut in_0: *mut crate::qfiles_h::dmodel_t = 0 as *mut crate::qfiles_h::dmodel_t;
    let mut out: *mut crate::cm_local_h::cmodel_t = 0 as *mut crate::cm_local_h::cmodel_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut indexes: *mut libc::c_int = 0 as *mut libc::c_int;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void
        as *mut crate::qfiles_h::dmodel_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<crate::qfiles_h::dmodel_t>() as libc::c_ulong)
        != 0
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"CMod_LoadSubmodels: funny lump size\x00" as *const u8 as *const libc::c_char,
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::qfiles_h::dmodel_t>() as libc::c_ulong)
        as libc::c_int;
    if count < 1 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"Map with no models\x00" as *const u8 as *const libc::c_char,
        );
    }
    cm.cmodels = crate::src::qcommon::common::Hunk_AllocDebug(
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::cm_local_h::cmodel_t>() as libc::c_ulong)
            as libc::c_int,
        crate::src::qcommon::q_shared::h_high,
        b"count * sizeof( *cm.cmodels )\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/qcommon/cm_load.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        134 as libc::c_int,
    ) as *mut crate::cm_local_h::cmodel_t;
    cm.numSubModels = count;
    if count > 256 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"MAX_SUBMODELS exceeded\x00" as *const u8 as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < count {
        out = &mut *cm.cmodels.offset(i as isize) as *mut crate::cm_local_h::cmodel_t;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            // spread the mins / maxs by a pixel
            (*out).mins[j as usize] = (*in_0).mins[j as usize] - 1 as libc::c_int as libc::c_float;
            (*out).maxs[j as usize] = (*in_0).maxs[j as usize] + 1 as libc::c_int as libc::c_float;
            j += 1
        }
        if !(i == 0 as libc::c_int) {
            // make a "leaf" just to hold the model's brushes and surfaces
            (*out).leaf.numLeafBrushes = (*in_0).numBrushes;
            indexes = crate::src::qcommon::common::Hunk_AllocDebug(
                (*out).leaf.numLeafBrushes * 4 as libc::c_int,
                crate::src::qcommon::q_shared::h_high,
                b"out->leaf.numLeafBrushes * 4\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                b"/home/luka/Projects/ioq3-server/src/qcommon/cm_load.c\x00" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                157 as libc::c_int,
            ) as *mut libc::c_int;
            (*out).leaf.firstLeafBrush =
                indexes.wrapping_offset_from(cm.leafbrushes) as libc::c_long as libc::c_int;
            j = 0 as libc::c_int;
            while j < (*out).leaf.numLeafBrushes {
                *indexes.offset(j as isize) = (*in_0).firstBrush + j;
                j += 1
            }
            (*out).leaf.numLeafSurfaces = (*in_0).numSurfaces;
            indexes = crate::src::qcommon::common::Hunk_AllocDebug(
                (*out).leaf.numLeafSurfaces * 4 as libc::c_int,
                crate::src::qcommon::q_shared::h_high,
                b"out->leaf.numLeafSurfaces * 4\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                b"/home/luka/Projects/ioq3-server/src/qcommon/cm_load.c\x00" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                164 as libc::c_int,
            ) as *mut libc::c_int;
            (*out).leaf.firstLeafSurface =
                indexes.wrapping_offset_from(cm.leafsurfaces) as libc::c_long as libc::c_int;
            j = 0 as libc::c_int;
            while j < (*out).leaf.numLeafSurfaces {
                *indexes.offset(j as isize) = (*in_0).firstSurface + j;
                j += 1
            }
        }
        i += 1;
        in_0 = in_0.offset(1)
        // world model doesn't need other info
    }
}
/*
=================
CMod_LoadNodes

=================
*/
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadNodes(mut l: *mut crate::qfiles_h::lump_t) {
    let mut in_0: *mut crate::qfiles_h::dnode_t = 0 as *mut crate::qfiles_h::dnode_t;
    let mut child: libc::c_int = 0;
    let mut out: *mut crate::cm_local_h::cNode_t = 0 as *mut crate::cm_local_h::cNode_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void
        as *mut crate::qfiles_h::dnode_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<crate::qfiles_h::dnode_t>() as libc::c_ulong)
        != 0
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as *const libc::c_char,
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::qfiles_h::dnode_t>() as libc::c_ulong)
        as libc::c_int;
    if count < 1 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"Map has no nodes\x00" as *const u8 as *const libc::c_char,
        );
    }
    cm.nodes = crate::src::qcommon::common::Hunk_AllocDebug(
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::cm_local_h::cNode_t>() as libc::c_ulong)
            as libc::c_int,
        crate::src::qcommon::q_shared::h_high,
        b"count * sizeof( *cm.nodes )\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/qcommon/cm_load.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        192 as libc::c_int,
    ) as *mut crate::cm_local_h::cNode_t;
    cm.numNodes = count;
    out = cm.nodes;
    i = 0 as libc::c_int;
    while i < count {
        (*out).plane = cm.planes.offset((*in_0).planeNum as isize);
        j = 0 as libc::c_int;
        while j < 2 as libc::c_int {
            child = (*in_0).children[j as usize];
            (*out).children[j as usize] = child;
            j += 1
        }
        i += 1;
        out = out.offset(1);
        in_0 = in_0.offset(1)
    }
}
/*
=================
CM_BoundBrush

=================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_BoundBrush(mut b: *mut crate::cm_local_h::cbrush_t) {
    (*b).bounds[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        -(*(*(*b).sides.offset(0 as libc::c_int as isize)).plane).dist;
    (*b).bounds[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        (*(*(*b).sides.offset(1 as libc::c_int as isize)).plane).dist;
    (*b).bounds[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        -(*(*(*b).sides.offset(2 as libc::c_int as isize)).plane).dist;
    (*b).bounds[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        (*(*(*b).sides.offset(3 as libc::c_int as isize)).plane).dist;
    (*b).bounds[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        -(*(*(*b).sides.offset(4 as libc::c_int as isize)).plane).dist;
    (*b).bounds[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        (*(*(*b).sides.offset(5 as libc::c_int as isize)).plane).dist;
}
/*
=================
CMod_LoadBrushes

=================
*/
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadBrushes(mut l: *mut crate::qfiles_h::lump_t) {
    let mut in_0: *mut crate::qfiles_h::dbrush_t = 0 as *mut crate::qfiles_h::dbrush_t;
    let mut out: *mut crate::cm_local_h::cbrush_t = 0 as *mut crate::cm_local_h::cbrush_t;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void
        as *mut crate::qfiles_h::dbrush_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<crate::qfiles_h::dbrush_t>() as libc::c_ulong)
        != 0
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as *const libc::c_char,
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::qfiles_h::dbrush_t>() as libc::c_ulong)
        as libc::c_int;
    cm.brushes = crate::src::qcommon::common::Hunk_AllocDebug(
        ((1 as libc::c_int + count) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::cm_local_h::cbrush_t>() as libc::c_ulong)
            as libc::c_int,
        crate::src::qcommon::q_shared::h_high,
        b"( BOX_BRUSHES + count ) * sizeof( *cm.brushes )\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/qcommon/cm_load.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        244 as libc::c_int,
    ) as *mut crate::cm_local_h::cbrush_t;
    cm.numBrushes = count;
    out = cm.brushes;
    i = 0 as libc::c_int;
    while i < count {
        (*out).sides = cm.brushsides.offset((*in_0).firstSide as isize);
        (*out).numsides = (*in_0).numSides;
        (*out).shaderNum = (*in_0).shaderNum;
        if (*out).shaderNum < 0 as libc::c_int || (*out).shaderNum >= cm.numShaders {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                b"CMod_LoadBrushes: bad shaderNum: %i\x00" as *const u8 as *const libc::c_char,
                (*out).shaderNum,
            );
        }
        (*out).contents = (*cm.shaders.offset((*out).shaderNum as isize)).contentFlags;
        CM_BoundBrush(out);
        i += 1;
        out = out.offset(1);
        in_0 = in_0.offset(1)
    }
}
/*
=================
CMod_LoadLeafs
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadLeafs(mut l: *mut crate::qfiles_h::lump_t) {
    let mut i: libc::c_int = 0;
    let mut out: *mut crate::cm_local_h::cLeaf_t = 0 as *mut crate::cm_local_h::cLeaf_t;
    let mut in_0: *mut crate::qfiles_h::dleaf_t = 0 as *mut crate::qfiles_h::dleaf_t;
    let mut count: libc::c_int = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void
        as *mut crate::qfiles_h::dleaf_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<crate::qfiles_h::dleaf_t>() as libc::c_ulong)
        != 0
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as *const libc::c_char,
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::qfiles_h::dleaf_t>() as libc::c_ulong)
        as libc::c_int;
    if count < 1 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"Map with no leafs\x00" as *const u8 as *const libc::c_char,
        );
    }
    cm.leafs = crate::src::qcommon::common::Hunk_AllocDebug(
        ((2 as libc::c_int + count) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::cm_local_h::cLeaf_t>() as libc::c_ulong)
            as libc::c_int,
        crate::src::qcommon::q_shared::h_high,
        b"( BOX_LEAFS + count ) * sizeof( *cm.leafs )\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/qcommon/cm_load.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        284 as libc::c_int,
    ) as *mut crate::cm_local_h::cLeaf_t;
    cm.numLeafs = count;
    out = cm.leafs;
    i = 0 as libc::c_int;
    while i < count {
        (*out).cluster = (*in_0).cluster;
        (*out).area = (*in_0).area;
        (*out).firstLeafBrush = (*in_0).firstLeafBrush;
        (*out).numLeafBrushes = (*in_0).numLeafBrushes;
        (*out).firstLeafSurface = (*in_0).firstLeafSurface;
        (*out).numLeafSurfaces = (*in_0).numLeafSurfaces;
        if (*out).cluster >= cm.numClusters {
            cm.numClusters = (*out).cluster + 1 as libc::c_int
        }
        if (*out).area >= cm.numAreas {
            cm.numAreas = (*out).area + 1 as libc::c_int
        }
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1)
    }
    cm.areas = crate::src::qcommon::common::Hunk_AllocDebug(
        (cm.numAreas as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::cm_local_h::cArea_t>() as libc::c_ulong)
            as libc::c_int,
        crate::src::qcommon::q_shared::h_high,
        b"cm.numAreas * sizeof( *cm.areas )\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/qcommon/cm_load.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        303 as libc::c_int,
    ) as *mut crate::cm_local_h::cArea_t;
    cm.areaPortals = crate::src::qcommon::common::Hunk_AllocDebug(
        ((cm.numAreas * cm.numAreas) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
        crate::src::qcommon::q_shared::h_high,
        b"cm.numAreas * cm.numAreas * sizeof( *cm.areaPortals )\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/qcommon/cm_load.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        304 as libc::c_int,
    ) as *mut libc::c_int;
}
/*
=================
CMod_LoadPlanes
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadPlanes(mut l: *mut crate::qfiles_h::lump_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut out: *mut crate::src::qcommon::q_shared::cplane_t =
        0 as *mut crate::src::qcommon::q_shared::cplane_t;
    let mut in_0: *mut crate::qfiles_h::dplane_t = 0 as *mut crate::qfiles_h::dplane_t;
    let mut count: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void
        as *mut crate::qfiles_h::dplane_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<crate::qfiles_h::dplane_t>() as libc::c_ulong)
        != 0
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as *const libc::c_char,
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::qfiles_h::dplane_t>() as libc::c_ulong)
        as libc::c_int;
    if count < 1 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"Map with no planes\x00" as *const u8 as *const libc::c_char,
        );
    }
    cm.planes = crate::src::qcommon::common::Hunk_AllocDebug(
        ((12 as libc::c_int + count) as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            crate::src::qcommon::q_shared::cplane_t,
        >() as libc::c_ulong) as libc::c_int,
        crate::src::qcommon::q_shared::h_high,
        b"( BOX_PLANES + count ) * sizeof( *cm.planes )\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/qcommon/cm_load.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        327 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::cplane_t;
    cm.numPlanes = count;
    out = cm.planes;
    i = 0 as libc::c_int;
    while i < count {
        bits = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*out).normal[j as usize] = (*in_0).normal[j as usize];
            if (*out).normal[j as usize] < 0 as libc::c_int as libc::c_float {
                bits |= (1 as libc::c_int) << j
            }
            j += 1
        }
        (*out).dist = (*in_0).dist;
        (*out).type_0 = if (*out).normal[0 as libc::c_int as usize] as libc::c_double == 1.0f64 {
            0 as libc::c_int
        } else if (*out).normal[1 as libc::c_int as usize] as libc::c_double == 1.0f64 {
            1 as libc::c_int
        } else if (*out).normal[2 as libc::c_int as usize] as libc::c_double == 1.0f64 {
            2 as libc::c_int
        } else {
            3 as libc::c_int
        } as crate::src::qcommon::q_shared::byte;
        (*out).signbits = bits as crate::src::qcommon::q_shared::byte;
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1)
    }
}
/*
=================
CMod_LoadLeafBrushes
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadLeafBrushes(mut l: *mut crate::qfiles_h::lump_t) {
    let mut i: libc::c_int = 0;
    let mut out: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut in_0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut count: libc::c_int = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut libc::c_int;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        != 0
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as *const libc::c_char,
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int;
    cm.leafbrushes = crate::src::qcommon::common::Hunk_AllocDebug(
        ((count + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
        crate::src::qcommon::q_shared::h_high,
        b"(count + BOX_BRUSHES) * sizeof( *cm.leafbrushes )\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/qcommon/cm_load.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        365 as libc::c_int,
    ) as *mut libc::c_int;
    cm.numLeafBrushes = count;
    out = cm.leafbrushes;
    i = 0 as libc::c_int;
    while i < count {
        *out = *in_0;
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1)
    }
}
/*
=================
CMod_LoadLeafSurfaces
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadLeafSurfaces(mut l: *mut crate::qfiles_h::lump_t) {
    let mut i: libc::c_int = 0;
    let mut out: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut in_0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut count: libc::c_int = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut libc::c_int;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        != 0
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as *const libc::c_char,
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int;
    cm.leafsurfaces = crate::src::qcommon::common::Hunk_AllocDebug(
        (count as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
        crate::src::qcommon::q_shared::h_high,
        b"count * sizeof( *cm.leafsurfaces )\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/qcommon/cm_load.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        392 as libc::c_int,
    ) as *mut libc::c_int;
    cm.numLeafSurfaces = count;
    out = cm.leafsurfaces;
    i = 0 as libc::c_int;
    while i < count {
        *out = *in_0;
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1)
    }
}
/*
=================
CMod_LoadBrushSides
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadBrushSides(mut l: *mut crate::qfiles_h::lump_t) {
    let mut i: libc::c_int = 0;
    let mut out: *mut crate::cm_local_h::cbrushside_t = 0 as *mut crate::cm_local_h::cbrushside_t;
    let mut in_0: *mut crate::qfiles_h::dbrushside_t = 0 as *mut crate::qfiles_h::dbrushside_t;
    let mut count: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void
        as *mut crate::qfiles_h::dbrushside_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<crate::qfiles_h::dbrushside_t>() as libc::c_ulong)
        != 0
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as *const libc::c_char,
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::qfiles_h::dbrushside_t>() as libc::c_ulong)
        as libc::c_int;
    cm.brushsides = crate::src::qcommon::common::Hunk_AllocDebug(
        ((6 as libc::c_int + count) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::cm_local_h::cbrushside_t>() as libc::c_ulong)
            as libc::c_int,
        crate::src::qcommon::q_shared::h_high,
        b"( BOX_SIDES + count ) * sizeof( *cm.brushsides )\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/qcommon/cm_load.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        421 as libc::c_int,
    ) as *mut crate::cm_local_h::cbrushside_t;
    cm.numBrushSides = count;
    out = cm.brushsides;
    i = 0 as libc::c_int;
    while i < count {
        num = (*in_0).planeNum;
        (*out).plane =
            &mut *cm.planes.offset(num as isize) as *mut crate::src::qcommon::q_shared::cplane_t;
        (*out).shaderNum = (*in_0).shaderNum;
        if (*out).shaderNum < 0 as libc::c_int || (*out).shaderNum >= cm.numShaders {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                b"CMod_LoadBrushSides: bad shaderNum: %i\x00" as *const u8 as *const libc::c_char,
                (*out).shaderNum,
            );
        }
        (*out).surfaceFlags = (*cm.shaders.offset((*out).shaderNum as isize)).surfaceFlags;
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1)
    }
}
/*
=================
CMod_LoadEntityString
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadEntityString(mut l: *mut crate::qfiles_h::lump_t) {
    cm.entityString = crate::src::qcommon::common::Hunk_AllocDebug(
        (*l).filelen,
        crate::src::qcommon::q_shared::h_high,
        b"l->filelen\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/qcommon/cm_load.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        444 as libc::c_int,
    ) as *mut libc::c_char;
    cm.numEntityChars = (*l).filelen;
    crate::stdlib::memcpy(
        cm.entityString as *mut libc::c_void,
        cmod_base.offset((*l).fileofs as isize) as *const libc::c_void,
        (*l).filelen as libc::c_ulong,
    );
}
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadVisibility(mut l: *mut crate::qfiles_h::lump_t) {
    let mut len: libc::c_int = 0;
    let mut buf: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    len = (*l).filelen;
    if len == 0 {
        cm.clusterBytes = cm.numClusters + 31 as libc::c_int & !(31 as libc::c_int);
        cm.visibility = crate::src::qcommon::common::Hunk_AllocDebug(
            cm.clusterBytes,
            crate::src::qcommon::q_shared::h_high,
            b"cm.clusterBytes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"/home/luka/Projects/ioq3-server/src/qcommon/cm_load.c\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            462 as libc::c_int,
        ) as *mut crate::src::qcommon::q_shared::byte;
        crate::stdlib::memset(
            cm.visibility as *mut libc::c_void,
            255 as libc::c_int,
            cm.clusterBytes as libc::c_ulong,
        );
        return;
    }
    buf = cmod_base.offset((*l).fileofs as isize);
    cm.vised = crate::src::qcommon::q_shared::qtrue;
    cm.visibility = crate::src::qcommon::common::Hunk_AllocDebug(
        len,
        crate::src::qcommon::q_shared::h_high,
        b"len\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/qcommon/cm_load.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        469 as libc::c_int,
    ) as *mut crate::src::qcommon::q_shared::byte;
    cm.numClusters = *(buf as *mut libc::c_int).offset(0 as libc::c_int as isize);
    cm.clusterBytes = *(buf as *mut libc::c_int).offset(1 as libc::c_int as isize);
    crate::stdlib::memcpy(
        cm.visibility as *mut libc::c_void,
        buf.offset(8 as libc::c_int as isize) as *const libc::c_void,
        (len - 8 as libc::c_int) as libc::c_ulong,
    );
}
#[no_mangle]

pub unsafe extern "C" fn CMod_LoadPatches(
    mut surfs: *mut crate::qfiles_h::lump_t,
    mut verts: *mut crate::qfiles_h::lump_t,
) {
    let mut dv: *mut crate::qfiles_h::drawVert_t = 0 as *mut crate::qfiles_h::drawVert_t;
    let mut dv_p: *mut crate::qfiles_h::drawVert_t = 0 as *mut crate::qfiles_h::drawVert_t;
    let mut in_0: *mut crate::qfiles_h::dsurface_t = 0 as *mut crate::qfiles_h::dsurface_t;
    let mut count: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut patch: *mut crate::cm_local_h::cPatch_t = 0 as *mut crate::cm_local_h::cPatch_t;
    let mut points: [crate::src::qcommon::q_shared::vec3_t; 1024] = [[0.; 3]; 1024];
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut shaderNum: libc::c_int = 0;
    in_0 = cmod_base.offset((*surfs).fileofs as isize) as *mut libc::c_void
        as *mut crate::qfiles_h::dsurface_t;
    if ((*surfs).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<crate::qfiles_h::dsurface_t>() as libc::c_ulong)
        != 0
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as *const libc::c_char,
        );
    }
    count = ((*surfs).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<crate::qfiles_h::dsurface_t>() as libc::c_ulong)
        as libc::c_int;
    cm.numSurfaces = count;
    cm.surfaces =
        crate::src::qcommon::common::Hunk_AllocDebug(
            (cm.numSurfaces as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
                *mut crate::cm_local_h::cPatch_t,
            >() as libc::c_ulong) as libc::c_int,
            crate::src::qcommon::q_shared::h_high,
            b"cm.numSurfaces * sizeof( cm.surfaces[0] )\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            b"/home/luka/Projects/ioq3-server/src/qcommon/cm_load.c\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            499 as libc::c_int,
        ) as *mut *mut crate::cm_local_h::cPatch_t;
    dv = cmod_base.offset((*verts).fileofs as isize) as *mut libc::c_void
        as *mut crate::qfiles_h::drawVert_t;
    if ((*verts).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<crate::qfiles_h::drawVert_t>() as libc::c_ulong)
        != 0
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as *const libc::c_char,
        );
    }
    // scan through all the surfaces, but only load patches,
    // not planar faces
    i = 0 as libc::c_int;
    while i < count {
        if !((*in_0).surfaceType != crate::qfiles_h::MST_PATCH as libc::c_int) {
            // FIXME: check for non-colliding patches
            patch = crate::src::qcommon::common::Hunk_AllocDebug(
                ::std::mem::size_of::<crate::cm_local_h::cPatch_t>() as libc::c_ulong
                    as libc::c_int,
                crate::src::qcommon::q_shared::h_high,
                b"sizeof( *patch )\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"/home/luka/Projects/ioq3-server/src/qcommon/cm_load.c\x00" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                513 as libc::c_int,
            ) as *mut crate::cm_local_h::cPatch_t;
            let ref mut fresh0 = *cm.surfaces.offset(i as isize);
            *fresh0 = patch;
            // load the full drawverts onto the stack
            width = (*in_0).patchWidth;
            height = (*in_0).patchHeight;
            c = width * height;
            if c > 1024 as libc::c_int {
                crate::src::qcommon::common::Com_Error(
                    crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                    b"ParseMesh: MAX_PATCH_VERTS\x00" as *const u8 as *const libc::c_char,
                );
            }
            dv_p = dv.offset((*in_0).firstVert as isize);
            j = 0 as libc::c_int;
            while j < c {
                points[j as usize][0 as libc::c_int as usize] =
                    (*dv_p).xyz[0 as libc::c_int as usize];
                points[j as usize][1 as libc::c_int as usize] =
                    (*dv_p).xyz[1 as libc::c_int as usize];
                points[j as usize][2 as libc::c_int as usize] =
                    (*dv_p).xyz[2 as libc::c_int as usize];
                j += 1;
                dv_p = dv_p.offset(1)
            }
            shaderNum = (*in_0).shaderNum;
            (*patch).contents = (*cm.shaders.offset(shaderNum as isize)).contentFlags;
            (*patch).surfaceFlags = (*cm.shaders.offset(shaderNum as isize)).surfaceFlags;
            // create the internal facet structure
            (*patch).pc = crate::src::qcommon::cm_patch::CM_GeneratePatchCollide(
                width,
                height,
                points.as_mut_ptr(),
            )
        }
        i += 1;
        in_0 = in_0.offset(1)
        // ignore other surfaces
    }
}
//==================================================================
#[no_mangle]

pub unsafe extern "C" fn CM_LumpChecksum(mut lump: *mut crate::qfiles_h::lump_t) -> libc::c_uint {
    return crate::src::qcommon::md4::Com_BlockChecksum(
        cmod_base.offset((*lump).fileofs as isize) as *const libc::c_void,
        (*lump).filelen,
    );
}
#[no_mangle]

pub unsafe extern "C" fn CM_Checksum(mut header: *mut crate::qfiles_h::dheader_t) -> libc::c_uint {
    let mut checksums: [libc::c_uint; 16] = [0; 16];
    checksums[0 as libc::c_int as usize] = CM_LumpChecksum(
        &mut *(*header)
            .lumps
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize),
    );
    checksums[1 as libc::c_int as usize] = CM_LumpChecksum(
        &mut *(*header)
            .lumps
            .as_mut_ptr()
            .offset(4 as libc::c_int as isize),
    );
    checksums[2 as libc::c_int as usize] = CM_LumpChecksum(
        &mut *(*header)
            .lumps
            .as_mut_ptr()
            .offset(6 as libc::c_int as isize),
    );
    checksums[3 as libc::c_int as usize] = CM_LumpChecksum(
        &mut *(*header)
            .lumps
            .as_mut_ptr()
            .offset(5 as libc::c_int as isize),
    );
    checksums[4 as libc::c_int as usize] = CM_LumpChecksum(
        &mut *(*header)
            .lumps
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize),
    );
    checksums[5 as libc::c_int as usize] = CM_LumpChecksum(
        &mut *(*header)
            .lumps
            .as_mut_ptr()
            .offset(9 as libc::c_int as isize),
    );
    checksums[6 as libc::c_int as usize] = CM_LumpChecksum(
        &mut *(*header)
            .lumps
            .as_mut_ptr()
            .offset(8 as libc::c_int as isize),
    );
    checksums[7 as libc::c_int as usize] = CM_LumpChecksum(
        &mut *(*header)
            .lumps
            .as_mut_ptr()
            .offset(7 as libc::c_int as isize),
    );
    checksums[8 as libc::c_int as usize] = CM_LumpChecksum(
        &mut *(*header)
            .lumps
            .as_mut_ptr()
            .offset(3 as libc::c_int as isize),
    );
    checksums[9 as libc::c_int as usize] = CM_LumpChecksum(
        &mut *(*header)
            .lumps
            .as_mut_ptr()
            .offset(13 as libc::c_int as isize),
    );
    checksums[10 as libc::c_int as usize] = CM_LumpChecksum(
        &mut *(*header)
            .lumps
            .as_mut_ptr()
            .offset(10 as libc::c_int as isize),
    );
    return crate::src::qcommon::md4::Com_BlockChecksum(
        checksums.as_mut_ptr() as *const libc::c_void,
        11 as libc::c_int * 4 as libc::c_int,
    );
}
/*
==================
CM_LoadMap

Loads in the map and all submodels
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_LoadMap(
    mut name: *const libc::c_char,
    mut clientload: crate::src::qcommon::q_shared::qboolean,
    mut checksum: *mut libc::c_int,
) {
    let mut buf: C2RustUnnamed_7 = C2RustUnnamed_7 {
        i: 0 as *mut libc::c_int,
    };
    let mut i: libc::c_int = 0;
    let mut header: crate::qfiles_h::dheader_t = crate::qfiles_h::dheader_t {
        ident: 0,
        version: 0,
        lumps: [crate::qfiles_h::lump_t {
            fileofs: 0,
            filelen: 0,
        }; 17],
    };
    let mut length: libc::c_int = 0;
    static mut last_checksum: libc::c_uint = 0;
    if name.is_null() || *name.offset(0 as libc::c_int as isize) == 0 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"CM_LoadMap: NULL name\x00" as *const u8 as *const libc::c_char,
        );
    }
    cm_noAreas = crate::src::qcommon::cvar::Cvar_Get(
        b"cm_noAreas\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as libc::c_int,
    );
    cm_noCurves = crate::src::qcommon::cvar::Cvar_Get(
        b"cm_noCurves\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200 as libc::c_int,
    );
    cm_playerCurveClip = crate::src::qcommon::cvar::Cvar_Get(
        b"cm_playerCurveClip\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int | 0x200 as libc::c_int,
    );
    crate::src::qcommon::common::Com_DPrintf(
        b"CM_LoadMap( %s, %i )\n\x00" as *const u8 as *const libc::c_char,
        name,
        clientload as libc::c_uint,
    );
    if crate::stdlib::strcmp(cm.name.as_mut_ptr(), name) == 0 && clientload as libc::c_uint != 0 {
        *checksum = last_checksum as libc::c_int;
        return;
    }
    // free old stuff
    crate::stdlib::memset(
        &mut cm as *mut crate::cm_local_h::clipMap_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::cm_local_h::clipMap_t>() as libc::c_ulong,
    );
    crate::src::qcommon::cm_patch::CM_ClearLevelPatches();
    if *name.offset(0 as libc::c_int as isize) == 0 {
        cm.numLeafs = 1 as libc::c_int;
        cm.numClusters = 1 as libc::c_int;
        cm.numAreas = 1 as libc::c_int;
        cm.cmodels = crate::src::qcommon::common::Hunk_AllocDebug(
            ::std::mem::size_of::<crate::cm_local_h::cmodel_t>() as libc::c_ulong as libc::c_int,
            crate::src::qcommon::q_shared::h_high,
            b"sizeof( *cm.cmodels )\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"/home/luka/Projects/ioq3-server/src/qcommon/cm_load.c\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            603 as libc::c_int,
        ) as *mut crate::cm_local_h::cmodel_t;
        *checksum = 0 as libc::c_int;
        return;
    }
    //
    // load the file
    //
    length = crate::src::qcommon::files::FS_ReadFile(name, &mut buf.v) as libc::c_int;
    if buf.i.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"Couldn\'t load %s\x00" as *const u8 as *const libc::c_char,
            name,
        );
    }
    last_checksum =
        crate::src::qcommon::md4::Com_BlockChecksum(buf.i as *const libc::c_void, length);
    *checksum = last_checksum as libc::c_int;
    header = *(buf.i as *mut crate::qfiles_h::dheader_t);
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<crate::qfiles_h::dheader_t>() as libc::c_ulong)
            .wrapping_div(4 as libc::c_int as libc::c_ulong)
    {
        *(&mut header as *mut crate::qfiles_h::dheader_t as *mut libc::c_int).offset(i as isize) =
            *(&mut header as *mut crate::qfiles_h::dheader_t as *mut libc::c_int)
                .offset(i as isize);
        i += 1
    }
    if header.version != 46 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"CM_LoadMap: %s has wrong version number (%i should be %i)\x00" as *const u8
                as *const libc::c_char,
            name,
            header.version,
            46 as libc::c_int,
        );
    }
    cmod_base = buf.i as *mut crate::src::qcommon::q_shared::byte;
    // load into heap
    CMod_LoadShaders(&mut *header.lumps.as_mut_ptr().offset(1 as libc::c_int as isize));
    CMod_LoadLeafs(&mut *header.lumps.as_mut_ptr().offset(4 as libc::c_int as isize));
    CMod_LoadLeafBrushes(&mut *header.lumps.as_mut_ptr().offset(6 as libc::c_int as isize));
    CMod_LoadLeafSurfaces(&mut *header.lumps.as_mut_ptr().offset(5 as libc::c_int as isize));
    CMod_LoadPlanes(&mut *header.lumps.as_mut_ptr().offset(2 as libc::c_int as isize));
    CMod_LoadBrushSides(&mut *header.lumps.as_mut_ptr().offset(9 as libc::c_int as isize));
    CMod_LoadBrushes(&mut *header.lumps.as_mut_ptr().offset(8 as libc::c_int as isize));
    CMod_LoadSubmodels(&mut *header.lumps.as_mut_ptr().offset(7 as libc::c_int as isize));
    CMod_LoadNodes(&mut *header.lumps.as_mut_ptr().offset(3 as libc::c_int as isize));
    CMod_LoadEntityString(&mut *header.lumps.as_mut_ptr().offset(0 as libc::c_int as isize));
    CMod_LoadVisibility(&mut *header.lumps.as_mut_ptr().offset(16 as libc::c_int as isize));
    CMod_LoadPatches(
        &mut *header.lumps.as_mut_ptr().offset(13 as libc::c_int as isize),
        &mut *header.lumps.as_mut_ptr().offset(10 as libc::c_int as isize),
    );
    // we are NOT freeing the file, because it is cached for the ref
    crate::src::qcommon::files::FS_FreeFile(buf.v);
    CM_InitBoxHull();
    CM_FloodAreaConnections();
    // allow this to be cached if it is loaded by the server
    if clientload as u64 == 0 {
        crate::src::qcommon::q_shared::Q_strncpyz(
            cm.name.as_mut_ptr(),
            name,
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        );
    };
}
/*
==================
CM_ClearMap
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_ClearMap() {
    crate::stdlib::memset(
        &mut cm as *mut crate::cm_local_h::clipMap_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::cm_local_h::clipMap_t>() as libc::c_ulong,
    );
    crate::src::qcommon::cm_patch::CM_ClearLevelPatches();
}
/*
==================
CM_ClipHandleToModel
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_ClipHandleToModel(
    mut handle: crate::src::qcommon::q_shared::clipHandle_t,
) -> *mut crate::cm_local_h::cmodel_t {
    if handle < 0 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"CM_ClipHandleToModel: bad handle %i\x00" as *const u8 as *const libc::c_char,
            handle,
        );
    }
    if handle < cm.numSubModels {
        return &mut *cm.cmodels.offset(handle as isize) as *mut crate::cm_local_h::cmodel_t;
    }
    if handle == 255 as libc::c_int {
        return &mut box_model;
    }
    if handle < 256 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"CM_ClipHandleToModel: bad handle %i < %i < %i\x00" as *const u8
                as *const libc::c_char,
            cm.numSubModels,
            handle,
            256 as libc::c_int,
        );
    }
    crate::src::qcommon::common::Com_Error(
        crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
        b"CM_ClipHandleToModel: bad handle %i\x00" as *const u8 as *const libc::c_char,
        handle + 256 as libc::c_int,
    );
}
/*
==================
CM_InlineModel
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_InlineModel(
    mut index: libc::c_int,
) -> crate::src::qcommon::q_shared::clipHandle_t {
    if index < 0 as libc::c_int || index >= cm.numSubModels {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"CM_InlineModel: bad number\x00" as *const u8 as *const libc::c_char,
        );
    }
    return index;
}
#[no_mangle]

pub unsafe extern "C" fn CM_NumClusters() -> libc::c_int {
    return cm.numClusters;
}
#[no_mangle]

pub unsafe extern "C" fn CM_NumInlineModels() -> libc::c_int {
    return cm.numSubModels;
}
#[no_mangle]

pub unsafe extern "C" fn CM_EntityString() -> *mut libc::c_char {
    return cm.entityString;
}
#[no_mangle]

pub unsafe extern "C" fn CM_LeafCluster(mut leafnum: libc::c_int) -> libc::c_int {
    if leafnum < 0 as libc::c_int || leafnum >= cm.numLeafs {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"CM_LeafCluster: bad number\x00" as *const u8 as *const libc::c_char,
        );
    }
    return (*cm.leafs.offset(leafnum as isize)).cluster;
}
// returns an ORed contents mask
// only returns non-solid leafs
// overflow if return listsize and if *lastLeaf != list[listsize-1]
#[no_mangle]

pub unsafe extern "C" fn CM_LeafArea(mut leafnum: libc::c_int) -> libc::c_int {
    if leafnum < 0 as libc::c_int || leafnum >= cm.numLeafs {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"CM_LeafArea: bad number\x00" as *const u8 as *const libc::c_char,
        );
    }
    return (*cm.leafs.offset(leafnum as isize)).area;
}
//=======================================================================
/*
===================
CM_InitBoxHull

Set up the planes and nodes so that the six floats of a bounding box
can just be stored out and get a proper clipping hull structure.
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_InitBoxHull() {
    let mut i: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut p: *mut crate::src::qcommon::q_shared::cplane_t =
        0 as *mut crate::src::qcommon::q_shared::cplane_t;
    let mut s: *mut crate::cm_local_h::cbrushside_t = 0 as *mut crate::cm_local_h::cbrushside_t;
    box_planes = &mut *cm.planes.offset(cm.numPlanes as isize)
        as *mut crate::src::qcommon::q_shared::cplane_t;
    box_brush = &mut *cm.brushes.offset(cm.numBrushes as isize) as *mut crate::cm_local_h::cbrush_t;
    (*box_brush).numsides = 6 as libc::c_int;
    (*box_brush).sides = cm.brushsides.offset(cm.numBrushSides as isize);
    (*box_brush).contents = 0x2000000 as libc::c_int;
    box_model.leaf.numLeafBrushes = 1 as libc::c_int;
    //	box_model.leaf.firstLeafBrush = cm.numBrushes;
    box_model.leaf.firstLeafBrush = cm.numLeafBrushes;
    *cm.leafbrushes.offset(cm.numLeafBrushes as isize) = cm.numBrushes;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        side = i & 1 as libc::c_int;
        // brush sides
        s = &mut *cm.brushsides.offset((cm.numBrushSides + i) as isize)
            as *mut crate::cm_local_h::cbrushside_t;
        (*s).plane = cm
            .planes
            .offset((cm.numPlanes + i * 2 as libc::c_int + side) as isize);
        (*s).surfaceFlags = 0 as libc::c_int;
        // planes
        p = &mut *box_planes.offset((i * 2 as libc::c_int) as isize)
            as *mut crate::src::qcommon::q_shared::cplane_t;
        (*p).type_0 = (i >> 1 as libc::c_int) as crate::src::qcommon::q_shared::byte;
        (*p).signbits = 0 as libc::c_int as crate::src::qcommon::q_shared::byte;
        (*p).normal[2 as libc::c_int as usize] =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        (*p).normal[1 as libc::c_int as usize] = (*p).normal[2 as libc::c_int as usize];
        (*p).normal[0 as libc::c_int as usize] = (*p).normal[1 as libc::c_int as usize];
        (*p).normal[(i >> 1 as libc::c_int) as usize] =
            1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        p = &mut *box_planes.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize)
            as *mut crate::src::qcommon::q_shared::cplane_t;
        (*p).type_0 =
            (3 as libc::c_int + (i >> 1 as libc::c_int)) as crate::src::qcommon::q_shared::byte;
        (*p).signbits = 0 as libc::c_int as crate::src::qcommon::q_shared::byte;
        (*p).normal[2 as libc::c_int as usize] =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        (*p).normal[1 as libc::c_int as usize] = (*p).normal[2 as libc::c_int as usize];
        (*p).normal[0 as libc::c_int as usize] = (*p).normal[1 as libc::c_int as usize];
        (*p).normal[(i >> 1 as libc::c_int) as usize] =
            -(1 as libc::c_int) as crate::src::qcommon::q_shared::vec_t;
        crate::src::qcommon::q_math::SetPlaneSignbits(p);
        i += 1
    }
}
/*
===================
CM_TempBoxModel

To keep everything totally uniform, bounding boxes are turned into small
BSP trees instead of being compared directly.
Capsules are handled differently though.
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_TempBoxModel(
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut capsule: libc::c_int,
) -> crate::src::qcommon::q_shared::clipHandle_t {
    box_model.mins[0 as libc::c_int as usize] = *mins.offset(0 as libc::c_int as isize);
    box_model.mins[1 as libc::c_int as usize] = *mins.offset(1 as libc::c_int as isize);
    box_model.mins[2 as libc::c_int as usize] = *mins.offset(2 as libc::c_int as isize);
    box_model.maxs[0 as libc::c_int as usize] = *maxs.offset(0 as libc::c_int as isize);
    box_model.maxs[1 as libc::c_int as usize] = *maxs.offset(1 as libc::c_int as isize);
    box_model.maxs[2 as libc::c_int as usize] = *maxs.offset(2 as libc::c_int as isize);
    if capsule != 0 {
        return 254 as libc::c_int;
    }
    (*box_planes.offset(0 as libc::c_int as isize)).dist = *maxs.offset(0 as libc::c_int as isize);
    (*box_planes.offset(1 as libc::c_int as isize)).dist = -*maxs.offset(0 as libc::c_int as isize);
    (*box_planes.offset(2 as libc::c_int as isize)).dist = *mins.offset(0 as libc::c_int as isize);
    (*box_planes.offset(3 as libc::c_int as isize)).dist = -*mins.offset(0 as libc::c_int as isize);
    (*box_planes.offset(4 as libc::c_int as isize)).dist = *maxs.offset(1 as libc::c_int as isize);
    (*box_planes.offset(5 as libc::c_int as isize)).dist = -*maxs.offset(1 as libc::c_int as isize);
    (*box_planes.offset(6 as libc::c_int as isize)).dist = *mins.offset(1 as libc::c_int as isize);
    (*box_planes.offset(7 as libc::c_int as isize)).dist = -*mins.offset(1 as libc::c_int as isize);
    (*box_planes.offset(8 as libc::c_int as isize)).dist = *maxs.offset(2 as libc::c_int as isize);
    (*box_planes.offset(9 as libc::c_int as isize)).dist = -*maxs.offset(2 as libc::c_int as isize);
    (*box_planes.offset(10 as libc::c_int as isize)).dist = *mins.offset(2 as libc::c_int as isize);
    (*box_planes.offset(11 as libc::c_int as isize)).dist =
        -*mins.offset(2 as libc::c_int as isize);
    (*box_brush).bounds[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        *mins.offset(0 as libc::c_int as isize);
    (*box_brush).bounds[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        *mins.offset(1 as libc::c_int as isize);
    (*box_brush).bounds[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        *mins.offset(2 as libc::c_int as isize);
    (*box_brush).bounds[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        *maxs.offset(0 as libc::c_int as isize);
    (*box_brush).bounds[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        *maxs.offset(1 as libc::c_int as isize);
    (*box_brush).bounds[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        *maxs.offset(2 as libc::c_int as isize);
    return 255 as libc::c_int;
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
/*
===================
CM_ModelBounds
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_ModelBounds(
    mut model: crate::src::qcommon::q_shared::clipHandle_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut cmod: *mut crate::cm_local_h::cmodel_t = 0 as *mut crate::cm_local_h::cmodel_t;
    cmod = CM_ClipHandleToModel(model);
    *mins.offset(0 as libc::c_int as isize) = (*cmod).mins[0 as libc::c_int as usize];
    *mins.offset(1 as libc::c_int as isize) = (*cmod).mins[1 as libc::c_int as usize];
    *mins.offset(2 as libc::c_int as isize) = (*cmod).mins[2 as libc::c_int as usize];
    *maxs.offset(0 as libc::c_int as isize) = (*cmod).maxs[0 as libc::c_int as usize];
    *maxs.offset(1 as libc::c_int as isize) = (*cmod).maxs[1 as libc::c_int as usize];
    *maxs.offset(2 as libc::c_int as isize) = (*cmod).maxs[2 as libc::c_int as usize];
}
