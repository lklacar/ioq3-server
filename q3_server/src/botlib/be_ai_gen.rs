use ::libc;

pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
use crate::src::botlib::be_interface::botimport;
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
use crate::stdlib::memcpy;
use crate::stdlib::rand;
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
 * name:		be_ai_gen.c
 *
 * desc:		genetic selection
 *
 * $Archive: /MissionPack/code/botlib/be_ai_gen.c $
 *
 *****************************************************************************/
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn GeneticSelection(
    mut numranks: libc::c_int,
    mut rankings: *mut libc::c_float,
) -> libc::c_int {
    let mut sum: libc::c_float = 0.; //end for
    let mut i: libc::c_int = 0; //end if
    let mut index: libc::c_int = 0;
    sum = 0 as libc::c_int as libc::c_float;
    i = 0 as libc::c_int;
    while i < numranks {
        if !(*rankings.offset(i as isize) < 0 as libc::c_int as libc::c_float) {
            sum += *rankings.offset(i as isize)
        }
        i += 1
    }
    if sum > 0 as libc::c_int as libc::c_float {
        //select a bot where the ones with the highest rankings have
        //the highest chance of being selected
        //sum *= random();
        i = 0 as libc::c_int;
        while i < numranks {
            if !(*rankings.offset(i as isize) < 0 as libc::c_int as libc::c_float) {
                sum -= *rankings.offset(i as isize);
                if sum <= 0 as libc::c_int as libc::c_float {
                    return i;
                }
            }
            i += 1
        }
        //end for
    }
    //select a bot randomly
    index = ((crate::stdlib::rand() & 0x7fff as libc::c_int) as libc::c_float
        / 0x7fff as libc::c_int as libc::c_float
        * numranks as libc::c_float) as libc::c_int; //end for
    i = 0 as libc::c_int;
    while i < numranks {
        if *rankings.offset(index as isize) >= 0 as libc::c_int as libc::c_float {
            return index;
        }
        index = (index + 1 as libc::c_int) % numranks;
        i += 1
    }
    return 0 as libc::c_int;
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
//
/* ****************************************************************************
 * name:		be_ai_gen.h
 *
 * desc:		genetic selection
 *
 * $Archive: /source/code/botlib/be_ai_gen.h $
 *
 *****************************************************************************/
//end of the function GeneticSelection
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn GeneticParentsAndChildSelection(
    mut numranks: libc::c_int,
    mut ranks: *mut libc::c_float,
    mut parent1: *mut libc::c_int,
    mut parent2: *mut libc::c_int,
    mut child: *mut libc::c_int,
) -> libc::c_int {
    let mut rankings: [libc::c_float; 256] = [0.; 256]; //end if
    let mut max: libc::c_float = 0.; //end for
    let mut i: libc::c_int = 0; //end if
    if numranks > 256 as libc::c_int {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            2 as libc::c_int,
            b"GeneticParentsAndChildSelection: too many bots\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        *child = 0 as libc::c_int;
        *parent2 = *child;
        *parent1 = *parent2;
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    max = 0 as libc::c_int as libc::c_float;
    i = 0 as libc::c_int;
    while i < numranks {
        if !(*ranks.offset(i as isize) < 0 as libc::c_int as libc::c_float) {
            max += 1.
        }
        i += 1
    }
    if max < 3 as libc::c_int as libc::c_float {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            2 as libc::c_int,
            b"GeneticParentsAndChildSelection: too few valid bots\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        *child = 0 as libc::c_int;
        *parent2 = *child;
        *parent1 = *parent2;
        return crate::src::qcommon::q_shared::qfalse as libc::c_int;
    }
    crate::stdlib::memcpy(
        rankings.as_mut_ptr() as *mut libc::c_void,
        ranks as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(numranks as libc::c_ulong),
    );
    //select first parent
    *parent1 = GeneticSelection(numranks, rankings.as_mut_ptr());
    rankings[*parent1 as usize] = -(1 as libc::c_int) as libc::c_float;
    //select second parent
    *parent2 = GeneticSelection(numranks, rankings.as_mut_ptr());
    rankings[*parent2 as usize] = -(1 as libc::c_int) as libc::c_float;
    //reverse the rankings
    max = 0 as libc::c_int as libc::c_float; //end for
    i = 0 as libc::c_int; //end for
    while i < numranks {
        if !(rankings[i as usize] < 0 as libc::c_int as libc::c_float) {
            if rankings[i as usize] > max {
                max = rankings[i as usize]
            }
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < numranks {
        if !(rankings[i as usize] < 0 as libc::c_int as libc::c_float) {
            rankings[i as usize] = max - rankings[i as usize]
        }
        i += 1
    }
    //select child
    *child = GeneticSelection(numranks, rankings.as_mut_ptr());
    return crate::src::qcommon::q_shared::qtrue as libc::c_int;
}
//end of the function GeneticParentsAndChildSelection
