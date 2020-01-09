use ::libc;

pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
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
// snddma_null.c
// all other sound mixing is portable
#[no_mangle]

pub unsafe extern "C" fn SNDDMA_Init() -> crate::src::qcommon::q_shared::qboolean {
    return crate::src::qcommon::q_shared::qfalse;
}
#[no_mangle]

pub unsafe extern "C" fn SNDDMA_GetDMAPos() -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn SNDDMA_Shutdown() {}
#[no_mangle]

pub unsafe extern "C" fn SNDDMA_BeginPainting() {}
#[no_mangle]

pub unsafe extern "C" fn SNDDMA_Submit() {}
#[no_mangle]

pub unsafe extern "C" fn S_RegisterSound(
    mut name: *const libc::c_char,
    mut compressed: crate::src::qcommon::q_shared::qboolean,
) -> crate::src::qcommon::q_shared::sfxHandle_t {
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn S_StartLocalSound(
    mut sfxHandle: crate::src::qcommon::q_shared::sfxHandle_t,
    mut channelNum: libc::c_int,
) {
}
// for writing the config files
#[no_mangle]

pub unsafe extern "C" fn S_ClearSoundBuffer() {}
