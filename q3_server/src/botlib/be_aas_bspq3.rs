use ::libc;

pub use crate::be_aas_def_h::bsp_link_s;
pub use crate::be_aas_def_h::bsp_link_t;
pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
pub use crate::src::botlib::l_script::punctuation_s;
pub use crate::src::botlib::l_script::punctuation_t;
pub use crate::src::botlib::l_script::script_s;
pub use crate::src::botlib::l_script::script_t;
pub use crate::src::botlib::l_script::token_s;
pub use crate::src::botlib::l_script::token_t;
pub use crate::src::botlib::l_script::FreeScript;
pub use crate::src::botlib::l_script::LoadScriptMemory;
pub use crate::src::botlib::l_script::PS_ExpectTokenType;
pub use crate::src::botlib::l_script::PS_ReadToken;
pub use crate::src::botlib::l_script::ScriptError;
pub use crate::src::botlib::l_script::SetScriptFlags;
pub use crate::src::botlib::l_script::StripDoubleQuotes;
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
use crate::stdlib::sscanf;

use crate::src::botlib::l_memory::FreeMemory;
use crate::src::botlib::l_memory::GetClearedHunkMemory;
use crate::src::botlib::l_memory::GetHunkMemory;
use crate::stdlib::atof;
use crate::stdlib::atoi;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
use crate::stdlib::strcmp;
use crate::stdlib::strcpy;
use crate::stdlib::strlen;
use crate::stdlib::strncpy;
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
     * name:		be_aas_bspq3.c
     *
     * desc:		BSP, Environment Sampling
     *
     * $Archive: /MissionPack/code/botlib/be_aas_bspq3.c $
     *
     *****************************************************************************/
    #[no_mangle]
    pub static mut botimport: crate::botlib_h::botlib_import_t;
}
//id Software BSP data

pub type bsp_t = bsp_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bsp_s {
    pub loaded: libc::c_int,
    pub entdatasize: libc::c_int,
    pub dentdata: *mut libc::c_char,
    pub numentities: libc::c_int,
    pub entities: [bsp_entity_t; 2048],
}
//true when bsp file is loaded
//entity data
//bsp entities
//bsp data entity

pub type bsp_entity_t = bsp_entity_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bsp_entity_s {
    pub epairs: *mut bsp_epair_t,
}
//bsp entity epair

pub type bsp_epair_t = bsp_epair_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bsp_epair_s {
    pub key: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub next: *mut bsp_epair_s,
}
//global bsp
#[no_mangle]

pub static mut bspworld: bsp_t = bsp_t {
    loaded: 0,
    entdatasize: 0,
    dentdata: 0 as *const libc::c_char as *mut libc::c_char,
    numentities: 0,
    entities: [bsp_entity_t {
        epairs: 0 as *const bsp_epair_t as *mut bsp_epair_t,
    }; 2048],
};
//trace through the world
//trace through the world
// BSP_DEBUG
//===========================================================================
// traces axial boxes of any size through the world
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_Trace(
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut passent: libc::c_int,
    mut contentmask: libc::c_int,
) -> crate::botlib_h::bsp_trace_t {
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
    botimport.Trace.expect("non-null function pointer")(
        &mut bsptrace,
        start,
        mins,
        maxs,
        end,
        passent,
        contentmask,
    );
    return bsptrace;
}
//returns the contents at the given point
//returns the contents at the given point
//end of the function AAS_Trace
//===========================================================================
// returns the contents at the given point
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_PointContents(
    mut point: *mut crate::src::qcommon::q_shared::vec_t,
) -> libc::c_int {
    return botimport.PointContents.expect("non-null function pointer")(point);
}
//calculates collision with given entity
//end of the function AAS_PointContents
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_EntityCollision(
    mut entnum: libc::c_int,
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut boxmins: *mut crate::src::qcommon::q_shared::vec_t,
    mut boxmaxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut contentmask: libc::c_int,
    mut trace: *mut crate::botlib_h::bsp_trace_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut enttrace: crate::botlib_h::bsp_trace_t = crate::botlib_h::bsp_trace_t {
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
    }; //end if
    botimport.EntityTrace.expect("non-null function pointer")(
        &mut enttrace,
        start,
        boxmins,
        boxmaxs,
        end,
        entnum,
        contentmask,
    );
    if enttrace.fraction < (*trace).fraction {
        crate::stdlib::memcpy(
            trace as *mut libc::c_void,
            &mut enttrace as *mut crate::botlib_h::bsp_trace_t as *const libc::c_void,
            ::std::mem::size_of::<crate::botlib_h::bsp_trace_t>() as libc::c_ulong,
        );
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
//returns true when p2 is in the PVS of p1
//returns true when p2 is in the PVS of p1
//end of the function AAS_EntityCollision
//===========================================================================
// returns true if in Potentially Hearable Set
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_inPVS(
    mut p1: *mut crate::src::qcommon::q_shared::vec_t,
    mut p2: *mut crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    return botimport.inPVS.expect("non-null function pointer")(p1, p2)
        as crate::src::qcommon::q_shared::qboolean;
}
//returns true when p2 is in the PHS of p1
//returns true when p2 is in the PHS of p1
//end of the function AAS_InPVS
//===========================================================================
// returns true if in Potentially Visible Set
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_inPHS(
    mut p1: *mut crate::src::qcommon::q_shared::vec_t,
    mut p2: *mut crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    return crate::src::qcommon::q_shared::qtrue;
}
//gets the mins, maxs and origin of a BSP model
//gets the mins, maxs and origin of a BSP model
//end of the function AAS_inPHS
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_BSPModelMinsMaxsOrigin(
    mut modelnum: libc::c_int,
    mut angles: *mut crate::src::qcommon::q_shared::vec_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
) {
    botimport
        .BSPModelMinsMaxsOrigin
        .expect("non-null function pointer")(modelnum, angles, mins, maxs, origin);
}
//unlink the given entity from the bsp tree leaves
//end of the function AAS_BSPModelMinsMaxs
//===========================================================================
// unlinks the entity from all leaves
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_UnlinkFromBSPLeaves(mut leaves: *mut crate::be_aas_def_h::bsp_link_t) {
}
//link the given entity to the bsp tree leaves of the given model
//end of the function AAS_UnlinkFromBSPLeaves
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_BSPLinkEntity(
    mut absmins: *mut crate::src::qcommon::q_shared::vec_t,
    mut absmaxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut entnum: libc::c_int,
    mut modelnum: libc::c_int,
) -> *mut crate::be_aas_def_h::bsp_link_t {
    return 0 as *mut crate::be_aas_def_h::bsp_link_t;
}
//creates a list with entities totally or partly within the given box
//creates a list with entities totally or partly within the given box
//end of the function AAS_BSPLinkEntity
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_BoxEntities(
    mut absmins: *mut crate::src::qcommon::q_shared::vec_t,
    mut absmaxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut list: *mut libc::c_int,
    mut maxcount: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
//end of the function AAS_BoxEntities
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_NextBSPEntity(mut ent: libc::c_int) -> libc::c_int {
    ent += 1;
    if ent >= 1 as libc::c_int && ent < bspworld.numentities {
        return ent;
    }
    return 0 as libc::c_int;
}
//end of the function AAS_NextBSPEntity
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_BSPEntityInRange(mut ent: libc::c_int) -> libc::c_int {
    if ent <= 0 as libc::c_int || ent >= bspworld.numentities {
        botimport.Print.expect("non-null function pointer")(
            1 as libc::c_int,
            b"bsp entity out of range\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function AAS_BSPEntityInRange
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ValueForBSPEpairKey(
    mut ent: libc::c_int,
    mut key: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut size: libc::c_int,
) -> libc::c_int {
    let mut epair: *mut bsp_epair_t = 0 as *mut bsp_epair_t; //end for
    *value.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    if AAS_BSPEntityInRange(ent) == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    epair = bspworld.entities[ent as usize].epairs;
    while !epair.is_null() {
        if crate::stdlib::strcmp((*epair).key, key) == 0 {
            crate::stdlib::strncpy(
                value,
                (*epair).value,
                (size - 1 as libc::c_int) as libc::c_ulong,
            );
            *value.offset((size - 1 as libc::c_int) as isize) = '\u{0}' as i32 as libc::c_char;
            return crate::src::qcommon::q_shared::qtrue as libc::c_int;
        }
        epair = (*epair).next
        //end if
    }
    return crate::src::qcommon::q_shared::qfalse as libc::c_int;
}
//end of the function AAS_FindBSPEpair
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_VectorForBSPEpairKey(
    mut ent: libc::c_int,
    mut key: *mut libc::c_char,
    mut v: *mut crate::src::qcommon::q_shared::vec_t,
) -> libc::c_int {
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut v1: libc::c_double = 0.;
    let mut v2: libc::c_double = 0.;
    let mut v3: libc::c_double = 0.;
    let ref mut fresh0 = *v.offset(2 as libc::c_int as isize);
    *fresh0 = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    let ref mut fresh1 = *v.offset(1 as libc::c_int as isize);
    *fresh1 = *fresh0;
    *v.offset(0 as libc::c_int as isize) = *fresh1;
    if AAS_ValueForBSPEpairKey(ent, key, buf.as_mut_ptr(), 128 as libc::c_int) == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    //scanf into doubles, then assign, so it is vec_t size independent
    v3 = 0 as libc::c_int as libc::c_double;
    v2 = v3;
    v1 = v2;
    crate::stdlib::sscanf(
        buf.as_mut_ptr(),
        b"%lf %lf %lf\x00" as *const u8 as *const libc::c_char,
        &mut v1 as *mut libc::c_double,
        &mut v2 as *mut libc::c_double,
        &mut v3 as *mut libc::c_double,
    );
    *v.offset(0 as libc::c_int as isize) = v1 as crate::src::qcommon::q_shared::vec_t;
    *v.offset(1 as libc::c_int as isize) = v2 as crate::src::qcommon::q_shared::vec_t;
    *v.offset(2 as libc::c_int as isize) = v3 as crate::src::qcommon::q_shared::vec_t;
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function AAS_VectorForBSPEpairKey
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_FloatForBSPEpairKey(
    mut ent: libc::c_int,
    mut key: *mut libc::c_char,
    mut value: *mut libc::c_float,
) -> libc::c_int {
    let mut buf: [libc::c_char; 128] = [0; 128];
    *value = 0 as libc::c_int as libc::c_float;
    if AAS_ValueForBSPEpairKey(ent, key, buf.as_mut_ptr(), 128 as libc::c_int) == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    *value = crate::stdlib::atof(buf.as_mut_ptr()) as libc::c_float;
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//handle to the next bsp entity
//handle to the next bsp entity
//return the value of the BSP epair key
//return the value of the BSP epair key
//get a vector for the BSP epair key
//get a vector for the BSP epair key
//get a float for the BSP epair key
//get a float for the BSP epair key
//get an integer for the BSP epair key
//get an integer for the BSP epair key
//end of the function AAS_FloatForBSPEpairKey
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_IntForBSPEpairKey(
    mut ent: libc::c_int,
    mut key: *mut libc::c_char,
    mut value: *mut libc::c_int,
) -> libc::c_int {
    let mut buf: [libc::c_char; 128] = [0; 128];
    *value = 0 as libc::c_int;
    if AAS_ValueForBSPEpairKey(ent, key, buf.as_mut_ptr(), 128 as libc::c_int) == 0 {
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    *value = crate::stdlib::atoi(buf.as_mut_ptr());
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function AAS_IntForBSPEpairKey
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_FreeBSPEntities() {
    let mut i: libc::c_int = 0; //end for
    let mut ent: *mut bsp_entity_t = 0 as *mut bsp_entity_t;
    let mut epair: *mut bsp_epair_t = 0 as *mut bsp_epair_t;
    let mut nextepair: *mut bsp_epair_t = 0 as *mut bsp_epair_t;
    i = 1 as libc::c_int;
    while i < bspworld.numentities {
        ent = &mut *bspworld.entities.as_mut_ptr().offset(i as isize) as *mut bsp_entity_t;
        epair = (*ent).epairs;
        while !epair.is_null() {
            nextepair = (*epair).next;
            //end for
            //
            if !(*epair).key.is_null() {
                crate::src::botlib::l_memory::FreeMemory((*epair).key as *mut libc::c_void);
            }
            if !(*epair).value.is_null() {
                crate::src::botlib::l_memory::FreeMemory((*epair).value as *mut libc::c_void);
            }
            crate::src::botlib::l_memory::FreeMemory(epair as *mut libc::c_void);
            epair = nextepair
        }
        i += 1
    }
    bspworld.numentities = 0 as libc::c_int;
}
//end of the function AAS_FreeBSPEntities
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ParseBSPEntities() {
    let mut script: *mut crate::src::botlib::l_script::script_t =
        0 as *mut crate::src::botlib::l_script::script_t; //SCFL_PRIMITIVE);
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut libc::c_char,
        endwhitespace_p: 0 as *mut libc::c_char,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    let mut ent: *mut bsp_entity_t = 0 as *mut bsp_entity_t;
    let mut epair: *mut bsp_epair_t = 0 as *mut bsp_epair_t;
    script = crate::src::botlib::l_script::LoadScriptMemory(
        bspworld.dentdata,
        bspworld.entdatasize,
        b"entdata\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    crate::src::botlib::l_script::SetScriptFlags(script, 0x4 as libc::c_int | 0x8 as libc::c_int);
    bspworld.numentities = 1 as libc::c_int;
    //end if
    while crate::src::botlib::l_script::PS_ReadToken(script, &mut token) != 0 {
        if crate::stdlib::strcmp(
            token.string.as_mut_ptr(),
            b"{\x00" as *const u8 as *const libc::c_char,
        ) != 0
        {
            crate::src::botlib::l_script::ScriptError(
                script,
                b"invalid %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                token.string.as_mut_ptr(),
            ); //end while
            AAS_FreeBSPEntities(); //end if
            crate::src::botlib::l_script::FreeScript(script); //end if
            return;
        } //end while
        if bspworld.numentities >= 2048 as libc::c_int {
            botimport.Print.expect("non-null function pointer")(
                1 as libc::c_int,
                b"too many entities in BSP file\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ); //end if
            break; //end if
        } else {
            ent = &mut *bspworld
                .entities
                .as_mut_ptr()
                .offset(bspworld.numentities as isize) as *mut bsp_entity_t;
            bspworld.numentities += 1;
            (*ent).epairs = 0 as *mut bsp_epair_t;
            while crate::src::botlib::l_script::PS_ReadToken(script, &mut token) != 0 {
                if crate::stdlib::strcmp(
                    token.string.as_mut_ptr(),
                    b"}\x00" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    break;
                }
                epair = crate::src::botlib::l_memory::GetClearedHunkMemory(::std::mem::size_of::<
                    bsp_epair_t,
                >()
                    as libc::c_ulong) as *mut bsp_epair_t;
                (*epair).next = (*ent).epairs;
                (*ent).epairs = epair;
                if token.type_0 != 1 as libc::c_int {
                    crate::src::botlib::l_script::ScriptError(
                        script,
                        b"invalid %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        token.string.as_mut_ptr(),
                    );
                    AAS_FreeBSPEntities();
                    crate::src::botlib::l_script::FreeScript(script);
                    return;
                }
                crate::src::botlib::l_script::StripDoubleQuotes(token.string.as_mut_ptr());
                (*epair).key = crate::src::botlib::l_memory::GetHunkMemory(
                    crate::stdlib::strlen(token.string.as_mut_ptr())
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                crate::stdlib::strcpy((*epair).key, token.string.as_mut_ptr());
                if crate::src::botlib::l_script::PS_ExpectTokenType(
                    script,
                    1 as libc::c_int,
                    0 as libc::c_int,
                    &mut token,
                ) == 0
                {
                    AAS_FreeBSPEntities();
                    crate::src::botlib::l_script::FreeScript(script);
                    return;
                }
                crate::src::botlib::l_script::StripDoubleQuotes(token.string.as_mut_ptr());
                (*epair).value = crate::src::botlib::l_memory::GetHunkMemory(
                    crate::stdlib::strlen(token.string.as_mut_ptr())
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                crate::stdlib::strcpy((*epair).value, token.string.as_mut_ptr());
            }
            if crate::stdlib::strcmp(
                token.string.as_mut_ptr(),
                b"}\x00" as *const u8 as *const libc::c_char,
            ) != 0
            {
                crate::src::botlib::l_script::ScriptError(
                    script,
                    b"missing }\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                AAS_FreeBSPEntities();
                crate::src::botlib::l_script::FreeScript(script);
                return;
            }
        }
    }
    crate::src::botlib::l_script::FreeScript(script);
}
//end of the function AAS_ParseBSPEntities
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_BSPTraceLight(
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut endpos: *mut crate::src::qcommon::q_shared::vec_t,
    mut red: *mut libc::c_int,
    mut green: *mut libc::c_int,
    mut blue: *mut libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
//dump the loaded BSP data
//end of the function AAS_BSPTraceLight
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_DumpBSPData() {
    AAS_FreeBSPEntities();
    if !bspworld.dentdata.is_null() {
        crate::src::botlib::l_memory::FreeMemory(bspworld.dentdata as *mut libc::c_void);
    }
    bspworld.dentdata = 0 as *mut libc::c_char;
    bspworld.entdatasize = 0 as libc::c_int;
    //
    bspworld.loaded = crate::src::qcommon::q_shared::qfalse as libc::c_int;
    crate::stdlib::memset(
        &mut bspworld as *mut bsp_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<bsp_t>() as libc::c_ulong,
    );
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
 * name:		be_aas_bsp.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_bsp.h $
 *
 *****************************************************************************/
/* ****************************************************************************
 * name:		be_aas_bsp.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_bsp.h $
 *
 *****************************************************************************/
//loads the given BSP file
//end of the function AAS_DumpBSPData
//===========================================================================
// load a .bsp file
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_LoadBSPFile() -> libc::c_int {
    AAS_DumpBSPData();
    bspworld.entdatasize =
        crate::stdlib::strlen(botimport.BSPEntityData.expect("non-null function pointer")())
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    bspworld.dentdata =
        crate::src::botlib::l_memory::GetClearedHunkMemory(bspworld.entdatasize as libc::c_ulong)
            as *mut libc::c_char;
    crate::stdlib::memcpy(
        bspworld.dentdata as *mut libc::c_void,
        botimport.BSPEntityData.expect("non-null function pointer")() as *const libc::c_void,
        bspworld.entdatasize as libc::c_ulong,
    );
    AAS_ParseBSPEntities();
    bspworld.loaded = crate::src::qcommon::q_shared::qtrue as libc::c_int;
    return 0 as libc::c_int;
}
//end of the function AAS_LoadBSPFile
