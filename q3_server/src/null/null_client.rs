use ::libc;

pub use crate::qcommon_h::msg_t;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::NA_BAD;
pub use crate::qcommon_h::NA_BOT;
pub use crate::qcommon_h::NA_BROADCAST;
pub use crate::qcommon_h::NA_IP;
pub use crate::qcommon_h::NA_IP6;
pub use crate::qcommon_h::NA_LOOPBACK;
pub use crate::qcommon_h::NA_MULTICAST6;
pub use crate::qcommon_h::NA_UNSPEC;
pub use crate::src::qcommon::cvar::Cvar_Get;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
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
#[no_mangle]

pub static mut cl_shownet: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub unsafe extern "C" fn CL_Shutdown(
    mut finalmsg: *mut libc::c_char,
    mut disconnect: crate::src::qcommon::q_shared::qboolean,
    mut quit: crate::src::qcommon::q_shared::qboolean,
) {
}
// the keyboard binding interface must be setup before execing
// config files, but the rest of client startup will happen later
#[no_mangle]

pub unsafe extern "C" fn CL_Init() {
    cl_shownet = crate::src::qcommon::cvar::Cvar_Get(
        b"cl_shownet\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x100 as libc::c_int,
    );
}
// char events are for field typing, not game control
#[no_mangle]

pub unsafe extern "C" fn CL_MouseEvent(
    mut dx: libc::c_int,
    mut dy: libc::c_int,
    mut time: libc::c_int,
) {
}
// for keyname autocompletion
#[no_mangle]

pub unsafe extern "C" fn Key_WriteBindings(mut f: crate::src::qcommon::q_shared::fileHandle_t) {}
#[no_mangle]

pub unsafe extern "C" fn CL_Frame(mut msec: libc::c_int) {}
#[no_mangle]

pub unsafe extern "C" fn CL_PacketEvent(
    mut from: crate::qcommon_h::netadr_t,
    mut msg: *mut crate::qcommon_h::msg_t,
) {
}
#[no_mangle]

pub unsafe extern "C" fn CL_CharEvent(mut key: libc::c_int) {}
#[no_mangle]

pub unsafe extern "C" fn CL_Disconnect(mut showMainMenu: crate::src::qcommon::q_shared::qboolean) {}
#[no_mangle]

pub unsafe extern "C" fn CL_MapLoading() {}
#[no_mangle]

pub unsafe extern "C" fn CL_GameCommand() -> crate::src::qcommon::q_shared::qboolean {
    return crate::src::qcommon::q_shared::qfalse;
}
#[no_mangle]

pub unsafe extern "C" fn CL_KeyEvent(
    mut key: libc::c_int,
    mut down: crate::src::qcommon::q_shared::qboolean,
    mut time: libc::c_uint,
) {
}
//
// UI interface
//
#[no_mangle]

pub unsafe extern "C" fn UI_GameCommand() -> crate::src::qcommon::q_shared::qboolean {
    return crate::src::qcommon::q_shared::qfalse;
}
// do a screen update before starting to load a map
// when the server is going to load a new map, the entire hunk
// will be cleared, so the client must shutdown cgame, ui, and
// the renderer
#[no_mangle]

pub unsafe extern "C" fn CL_ForwardCommandToServer(mut string: *const libc::c_char) {}
#[no_mangle]

pub unsafe extern "C" fn CL_ConsolePrint(mut txt: *mut libc::c_char) {}
#[no_mangle]

pub unsafe extern "C" fn CL_JoystickEvent(
    mut axis: libc::c_int,
    mut value: libc::c_int,
    mut time: libc::c_int,
) {
}
/*
==============================================================

CLIENT / SERVER SYSTEMS

==============================================================
*/
//
// client interface
//
#[no_mangle]

pub unsafe extern "C" fn CL_InitKeyCommands() {}
// adds the current command line as a clc_clientCommand to the client message.
// things like godmode, noclip, etc, are commands directed to the server,
// so when they are typed in at the console, they will need to be forwarded.
#[no_mangle]

pub unsafe extern "C" fn CL_CDDialog() {}
// bring up the "need a cd to play" dialog
#[no_mangle]

pub unsafe extern "C" fn CL_FlushMemory() {}
// dump all memory on an error
#[no_mangle]

pub unsafe extern "C" fn CL_ShutdownAll(mut shutdownRef: crate::src::qcommon::q_shared::qboolean) {}
// initialize renderer interface
#[no_mangle]

pub unsafe extern "C" fn CL_StartHunkUsers(
    mut rendererOnly: crate::src::qcommon::q_shared::qboolean,
) {
}
// shutdown client
#[no_mangle]

pub unsafe extern "C" fn CL_InitRef() {}
// start all the client stuff using the hunk
#[no_mangle]

pub unsafe extern "C" fn CL_Snd_Shutdown() {}
#[no_mangle]

pub unsafe extern "C" fn CL_CDKeyValidate(
    mut key: *const libc::c_char,
    mut checksum: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    return crate::src::qcommon::q_shared::qtrue;
}
