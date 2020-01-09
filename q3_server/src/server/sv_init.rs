use ::libc;

pub use crate::stdlib::intptr_t;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::g_public_h::entityShared_t;
pub use crate::g_public_h::sharedEntity_t;
pub use crate::g_public_h::BOTAI_START_FRAME;
pub use crate::g_public_h::GAME_CLIENT_BEGIN;
pub use crate::g_public_h::GAME_CLIENT_COMMAND;
pub use crate::g_public_h::GAME_CLIENT_CONNECT;
pub use crate::g_public_h::GAME_CLIENT_DISCONNECT;
pub use crate::g_public_h::GAME_CLIENT_THINK;
pub use crate::g_public_h::GAME_CLIENT_USERINFO_CHANGED;
pub use crate::g_public_h::GAME_CONSOLE_COMMAND;
pub use crate::g_public_h::GAME_INIT;
pub use crate::g_public_h::GAME_RUN_FRAME;
pub use crate::g_public_h::GAME_SHUTDOWN;
pub use crate::qcommon_h::msg_t;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::netchan_t;
pub use crate::qcommon_h::netsrc_t;
pub use crate::qcommon_h::vm_t;
pub use crate::qcommon_h::NA_BAD;
pub use crate::qcommon_h::NA_BOT;
pub use crate::qcommon_h::NA_BROADCAST;
pub use crate::qcommon_h::NA_IP;
pub use crate::qcommon_h::NA_IP6;
pub use crate::qcommon_h::NA_LOOPBACK;
pub use crate::qcommon_h::NA_MULTICAST6;
pub use crate::qcommon_h::NA_UNSPEC;
pub use crate::qcommon_h::NS_CLIENT;
pub use crate::qcommon_h::NS_SERVER;
pub use crate::server_h::challenge_t;
pub use crate::server_h::clientSnapshot_t;
pub use crate::server_h::clientState_t;
pub use crate::server_h::client_s;
pub use crate::server_h::client_t;
pub use crate::server_h::netchan_buffer_s;
pub use crate::server_h::netchan_buffer_t;
pub use crate::server_h::serverState_t;
pub use crate::server_h::serverStatic_t;
pub use crate::server_h::server_t;
pub use crate::server_h::svEntity_s;
pub use crate::server_h::svEntity_t;
pub use crate::server_h::worldSector_s;
pub use crate::server_h::CS_ACTIVE;
pub use crate::server_h::CS_CONNECTED;
pub use crate::server_h::CS_FREE;
pub use crate::server_h::CS_PRIMED;
pub use crate::server_h::CS_ZOMBIE;
pub use crate::server_h::SS_DEAD;
pub use crate::server_h::SS_GAME;
pub use crate::server_h::SS_LOADING;
pub use crate::src::null::null_client::CL_Disconnect;
pub use crate::src::null::null_client::CL_MapLoading;
pub use crate::src::null::null_client::CL_ShutdownAll;
pub use crate::src::qcommon::cmd::Cbuf_AddText;
pub use crate::src::qcommon::common::com_dedicated;
pub use crate::src::qcommon::common::com_errorEntered;
pub use crate::src::qcommon::common::com_frameTime;
pub use crate::src::qcommon::common::com_sv_running;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Milliseconds;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::CopyString;
pub use crate::src::qcommon::common::Hunk_AllocDebug;
pub use crate::src::qcommon::common::Hunk_AllocateTempMemory;
pub use crate::src::qcommon::common::Hunk_Clear;
pub use crate::src::qcommon::common::Hunk_FreeTempMemory;
pub use crate::src::qcommon::common::Hunk_SetMark;
pub use crate::src::qcommon::common::Z_Free;
pub use crate::src::qcommon::common::Z_MallocDebug;
pub use crate::src::qcommon::cvar::cvar_modifiedFlags;
pub use crate::src::qcommon::cvar::Cvar_Get;
pub use crate::src::qcommon::cvar::Cvar_InfoString;
pub use crate::src::qcommon::cvar::Cvar_InfoString_Big;
pub use crate::src::qcommon::cvar::Cvar_Set;
pub use crate::src::qcommon::cvar::Cvar_VariableValue;
pub use crate::src::qcommon::files::FS_ClearPakReferences;
pub use crate::src::qcommon::files::FS_FCloseFile;
pub use crate::src::qcommon::files::FS_FOpenFileRead;
pub use crate::src::qcommon::files::FS_LoadedPakChecksums;
pub use crate::src::qcommon::files::FS_LoadedPakNames;
pub use crate::src::qcommon::files::FS_ReferencedPakChecksums;
pub use crate::src::qcommon::files::FS_ReferencedPakNames;
pub use crate::src::qcommon::files::FS_Restart;
pub use crate::src::qcommon::net_ip::NET_JoinMulticast6;
pub use crate::src::qcommon::net_ip::NET_LeaveMulticast6;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::h_dontcare;
pub use crate::src::qcommon::q_shared::h_high;
pub use crate::src::qcommon::q_shared::h_low;
pub use crate::src::qcommon::q_shared::ha_pref;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
pub use crate::src::server::sv_bot::SV_BotFrame;
pub use crate::src::server::sv_bot::SV_BotInitBotLib;
pub use crate::src::server::sv_bot::SV_BotInitCvars;
pub use crate::src::server::sv_ccmds::SV_AddOperatorCommands;
pub use crate::src::server::sv_ccmds::SV_Heartbeat_f;
pub use crate::src::server::sv_ccmds::SV_RemoveOperatorCommands;
pub use crate::src::server::sv_client::SV_DropClient;
pub use crate::src::server::sv_client::SV_FreeClient;
pub use crate::src::server::sv_game::SV_GentityNum;
pub use crate::src::server::sv_game::SV_InitGameProgs;
pub use crate::src::server::sv_game::SV_ShutdownGameProgs;
pub use crate::src::server::sv_main::gvm;
pub use crate::src::server::sv_main::sv;
pub use crate::src::server::sv_main::sv_allowDownload;
pub use crate::src::server::sv_main::sv_banFile;
pub use crate::src::server::sv_main::sv_dlRate;
pub use crate::src::server::sv_main::sv_floodProtect;
pub use crate::src::server::sv_main::sv_fps;
pub use crate::src::server::sv_main::sv_gametype;
pub use crate::src::server::sv_main::sv_hostname;
pub use crate::src::server::sv_main::sv_killserver;
pub use crate::src::server::sv_main::sv_lanForceRate;
pub use crate::src::server::sv_main::sv_mapChecksum;
pub use crate::src::server::sv_main::sv_mapname;
pub use crate::src::server::sv_main::sv_master;
pub use crate::src::server::sv_main::sv_maxPing;
pub use crate::src::server::sv_main::sv_maxRate;
pub use crate::src::server::sv_main::sv_maxclients;
pub use crate::src::server::sv_main::sv_minPing;
pub use crate::src::server::sv_main::sv_minRate;
pub use crate::src::server::sv_main::sv_padPackets;
pub use crate::src::server::sv_main::sv_privateClients;
pub use crate::src::server::sv_main::sv_privatePassword;
pub use crate::src::server::sv_main::sv_pure;
pub use crate::src::server::sv_main::sv_rconPassword;
pub use crate::src::server::sv_main::sv_reconnectlimit;
pub use crate::src::server::sv_main::sv_serverid;
pub use crate::src::server::sv_main::sv_showloss;
pub use crate::src::server::sv_main::sv_strictAuth;
pub use crate::src::server::sv_main::sv_timeout;
pub use crate::src::server::sv_main::sv_zombietime;
pub use crate::src::server::sv_main::svs;
pub use crate::src::server::sv_main::SV_MasterShutdown;
pub use crate::src::server::sv_main::SV_SendServerCommand;
pub use crate::src::server::sv_snapshot::SV_SendClientSnapshot;
pub use crate::src::server::sv_world::SV_ClearWorld;
pub use crate::src::vm::vm::VM_Call;
pub use crate::src::vm::vm::VM_ExplicitArgPtr;
use crate::stdlib::rand;
pub use crate::vm_local_h::vm_s;

use crate::src::qcommon::cm_load::CM_ClearMap;
use crate::src::qcommon::cm_load::CM_LoadMap;
use crate::stdlib::memset;
use crate::stdlib::strcmp;
use crate::stdlib::strlen;
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
===============
SV_SendConfigstring

Creates and sends the server command necessary to update the CS index for the
given client
===============
*/

unsafe extern "C" fn SV_SendConfigstring(
    mut client: *mut crate::server_h::client_t,
    mut index: libc::c_int,
) {
    let mut maxChunkSize: libc::c_int = 1024 as libc::c_int - 24 as libc::c_int;
    let mut len: libc::c_int = 0;
    len = crate::stdlib::strlen(crate::src::server::sv_main::sv.configstrings[index as usize])
        as libc::c_int;
    if len >= maxChunkSize {
        let mut sent: libc::c_int = 0 as libc::c_int;
        let mut remaining: libc::c_int = len;
        let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut buf: [libc::c_char; 1024] = [0; 1024];
        while remaining > 0 as libc::c_int {
            if sent == 0 as libc::c_int {
                cmd = b"bcs0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            } else if remaining < maxChunkSize {
                cmd = b"bcs2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            } else {
                cmd = b"bcs1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            crate::src::qcommon::q_shared::Q_strncpyz(
                buf.as_mut_ptr(),
                &mut *(*crate::src::server::sv_main::sv
                    .configstrings
                    .as_mut_ptr()
                    .offset(index as isize))
                .offset(sent as isize),
                maxChunkSize,
            );
            crate::src::server::sv_main::SV_SendServerCommand(
                client,
                b"%s %i \"%s\"\n\x00" as *const u8 as *const libc::c_char,
                cmd,
                index,
                buf.as_mut_ptr(),
            );
            sent += maxChunkSize - 1 as libc::c_int;
            remaining -= maxChunkSize - 1 as libc::c_int
        }
    } else {
        // standard cs, just send it
        crate::src::server::sv_main::SV_SendServerCommand(
            client,
            b"cs %i \"%s\"\n\x00" as *const u8 as *const libc::c_char,
            index,
            crate::src::server::sv_main::sv.configstrings[index as usize],
        );
    };
}
/*
===============
SV_UpdateConfigstrings

Called when a client goes from CS_PRIMED to CS_ACTIVE.  Updates all
Configstring indexes that have changed while the client was in CS_PRIMED
===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_UpdateConfigstrings(mut client: *mut crate::server_h::client_t) {
    let mut index: libc::c_int = 0;
    index = 0 as libc::c_int;
    while index < 1024 as libc::c_int {
        // if the CS hasn't changed since we went to CS_PRIMED, ignore
        if !((*client).csUpdated[index as usize] as u64 == 0) {
            // do not always send server info to all clients
            if !(index == 0 as libc::c_int
                && !(*client).gentity.is_null()
                && (*(*client).gentity).r.svFlags & 0x200 as libc::c_int != 0)
            {
                SV_SendConfigstring(client, index);
                (*client).csUpdated[index as usize] = crate::src::qcommon::q_shared::qfalse
            }
        }
        index += 1
    }
}
/*
===============
SV_SetConfigstring

===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_SetConfigstring(mut index: libc::c_int, mut val: *const libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut client: *mut crate::server_h::client_t = 0 as *mut crate::server_h::client_t;
    if index < 0 as libc::c_int || index >= 1024 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"SV_SetConfigstring: bad index %i\x00" as *const u8 as *const libc::c_char,
            index,
        );
    }
    if val.is_null() {
        val = b"\x00" as *const u8 as *const libc::c_char
    }
    // don't bother broadcasting an update if no change
    if crate::stdlib::strcmp(
        val,
        crate::src::server::sv_main::sv.configstrings[index as usize],
    ) == 0
    {
        return;
    }
    // change the string in sv
    crate::src::qcommon::common::Z_Free(
        crate::src::server::sv_main::sv.configstrings[index as usize] as *mut libc::c_void,
    );
    crate::src::server::sv_main::sv.configstrings[index as usize] =
        crate::src::qcommon::common::CopyString(val);
    // send it to all the clients if we aren't
    // spawning a new server
    if crate::src::server::sv_main::sv.state as libc::c_uint
        == crate::server_h::SS_GAME as libc::c_int as libc::c_uint
        || crate::src::server::sv_main::sv.restarting as libc::c_uint != 0
    {
        // send the data to all relevant clients
        i = 0 as libc::c_int;
        client = crate::src::server::sv_main::svs.clients;
        while i < (*crate::src::server::sv_main::sv_maxclients).integer {
            if ((*client).state as libc::c_uint)
                < crate::server_h::CS_ACTIVE as libc::c_int as libc::c_uint
            {
                if (*client).state as libc::c_uint
                    == crate::server_h::CS_PRIMED as libc::c_int as libc::c_uint
                {
                    (*client).csUpdated[index as usize] = crate::src::qcommon::q_shared::qtrue
                }
            } else if !(index == 0 as libc::c_int
                && !(*client).gentity.is_null()
                && (*(*client).gentity).r.svFlags & 0x200 as libc::c_int != 0)
            {
                SV_SendConfigstring(client, index);
            }
            i += 1;
            client = client.offset(1)
        }
    };
}
// do not always send server info to all clients
/*
===============
SV_GetConfigstring

===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_GetConfigstring(
    mut index: libc::c_int,
    mut buffer: *mut libc::c_char,
    mut bufferSize: libc::c_int,
) {
    if bufferSize < 1 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"SV_GetConfigstring: bufferSize == %i\x00" as *const u8 as *const libc::c_char,
            bufferSize,
        );
    }
    if index < 0 as libc::c_int || index >= 1024 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"SV_GetConfigstring: bad index %i\x00" as *const u8 as *const libc::c_char,
            index,
        );
    }
    if crate::src::server::sv_main::sv.configstrings[index as usize].is_null() {
        *buffer.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
        return;
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        buffer,
        crate::src::server::sv_main::sv.configstrings[index as usize],
        bufferSize,
    );
}
/*
===============
SV_SetUserinfo

===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_SetUserinfo(mut index: libc::c_int, mut val: *const libc::c_char) {
    if index < 0 as libc::c_int || index >= (*crate::src::server::sv_main::sv_maxclients).integer {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"SV_SetUserinfo: bad index %i\x00" as *const u8 as *const libc::c_char,
            index,
        );
    }
    if val.is_null() {
        val = b"\x00" as *const u8 as *const libc::c_char
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*crate::src::server::sv_main::svs
            .clients
            .offset(index as isize))
        .userinfo
        .as_mut_ptr(),
        val,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*crate::src::server::sv_main::svs
            .clients
            .offset(index as isize))
        .name
        .as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            val,
            b"name\x00" as *const u8 as *const libc::c_char,
        ),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
    );
}
/*
===============
SV_GetUserinfo

===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_GetUserinfo(
    mut index: libc::c_int,
    mut buffer: *mut libc::c_char,
    mut bufferSize: libc::c_int,
) {
    if bufferSize < 1 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"SV_GetUserinfo: bufferSize == %i\x00" as *const u8 as *const libc::c_char,
            bufferSize,
        );
    }
    if index < 0 as libc::c_int || index >= (*crate::src::server::sv_main::sv_maxclients).integer {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"SV_GetUserinfo: bad index %i\x00" as *const u8 as *const libc::c_char,
            index,
        );
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        buffer,
        (*crate::src::server::sv_main::svs
            .clients
            .offset(index as isize))
        .userinfo
        .as_mut_ptr(),
        bufferSize,
    );
}
/*
================
SV_CreateBaseline

Entity baselines are used to compress non-delta messages
to the clients -- only the fields that differ from the
baseline will be transmitted
================
*/

unsafe extern "C" fn SV_CreateBaseline() {
    let mut svent: *mut crate::g_public_h::sharedEntity_t =
        0 as *mut crate::g_public_h::sharedEntity_t;
    let mut entnum: libc::c_int = 0;
    entnum = 1 as libc::c_int;
    while entnum < crate::src::server::sv_main::sv.num_entities {
        svent = crate::src::server::sv_game::SV_GentityNum(entnum);
        if !((*svent).r.linked as u64 == 0) {
            (*svent).s.number = entnum;
            //
            // take current state as baseline
            //
            crate::src::server::sv_main::sv.svEntities[entnum as usize].baseline = (*svent).s
        }
        entnum += 1
    }
}
/*
===============
SV_BoundMaxClients

===============
*/

unsafe extern "C" fn SV_BoundMaxClients(mut minimum: libc::c_int) {
    // get the current maxclients value
    crate::src::qcommon::cvar::Cvar_Get(
        b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
        b"8\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    (*crate::src::server::sv_main::sv_maxclients).modified = crate::src::qcommon::q_shared::qfalse;
    if (*crate::src::server::sv_main::sv_maxclients).integer < minimum {
        crate::src::qcommon::cvar::Cvar_Set(
            b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                minimum,
            ),
        );
    } else if (*crate::src::server::sv_main::sv_maxclients).integer > 64 as libc::c_int {
        crate::src::qcommon::cvar::Cvar_Set(
            b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                64 as libc::c_int,
            ),
        );
    };
}
/*
===============
SV_Startup

Called when a host starts a map when it wasn't running
one before.  Successive map or map_restart commands will
NOT cause this to be called, unless the game is exited to
the menu system first.
===============
*/

unsafe extern "C" fn SV_Startup() {
    if crate::src::server::sv_main::svs.initialized as u64 != 0 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"SV_Startup: svs.initialized\x00" as *const u8 as *const libc::c_char,
        );
    }
    SV_BoundMaxClients(1 as libc::c_int);
    crate::src::server::sv_main::svs.clients = crate::src::qcommon::common::Z_MallocDebug(
        (::std::mem::size_of::<crate::server_h::client_t>() as libc::c_ulong)
            .wrapping_mul((*crate::src::server::sv_main::sv_maxclients).integer as libc::c_ulong)
            as libc::c_int,
        b"sizeof(client_t) * sv_maxclients->integer\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/server/sv_init.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        272 as libc::c_int,
    ) as *mut crate::server_h::client_t;
    if (*crate::src::qcommon::common::com_dedicated).integer != 0 {
        crate::src::server::sv_main::svs.numSnapshotEntities =
            (*crate::src::server::sv_main::sv_maxclients).integer
                * 32 as libc::c_int
                * 256 as libc::c_int
    } else {
        // we don't need nearly as many when playing locally
        crate::src::server::sv_main::svs.numSnapshotEntities =
            (*crate::src::server::sv_main::sv_maxclients).integer
                * 4 as libc::c_int
                * 256 as libc::c_int
    }
    crate::src::server::sv_main::svs.initialized = crate::src::qcommon::q_shared::qtrue;
    // Don't respect sv_killserver unless a server is actually running
    if (*crate::src::server::sv_main::sv_killserver).integer != 0 {
        crate::src::qcommon::cvar::Cvar_Set(
            b"sv_killserver\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
    }
    crate::src::qcommon::cvar::Cvar_Set(
        b"sv_running\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
    );
    // Join the ipv6 multicast group now that a map is running so clients can scan for us on the local network.
    crate::src::qcommon::net_ip::NET_JoinMulticast6();
}
/*
==================
SV_ChangeMaxClients
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_ChangeMaxClients() {
    let mut oldMaxClients: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut oldClients: *mut crate::server_h::client_t = 0 as *mut crate::server_h::client_t;
    let mut count: libc::c_int = 0;
    // get the highest client number in use
    count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*crate::src::server::sv_main::sv_maxclients).integer {
        if (*crate::src::server::sv_main::svs.clients.offset(i as isize)).state as libc::c_uint
            >= crate::server_h::CS_CONNECTED as libc::c_int as libc::c_uint
        {
            if i > count {
                count = i
            }
        }
        i += 1
    }
    count += 1;
    oldMaxClients = (*crate::src::server::sv_main::sv_maxclients).integer;
    // never go below the highest client number in use
    SV_BoundMaxClients(count);
    // if still the same
    if (*crate::src::server::sv_main::sv_maxclients).integer == oldMaxClients {
        return;
    }
    oldClients = crate::src::qcommon::common::Hunk_AllocateTempMemory(
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::server_h::client_t>() as libc::c_ulong)
            as libc::c_int,
    ) as *mut crate::server_h::client_t;
    // copy the clients to hunk memory
    i = 0 as libc::c_int;
    while i < count {
        if (*crate::src::server::sv_main::svs.clients.offset(i as isize)).state as libc::c_uint
            >= crate::server_h::CS_CONNECTED as libc::c_int as libc::c_uint
        {
            *oldClients.offset(i as isize) =
                *crate::src::server::sv_main::svs.clients.offset(i as isize)
        } else {
            crate::stdlib::memset(
                &mut *oldClients.offset(i as isize) as *mut crate::server_h::client_t
                    as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<crate::server_h::client_t>() as libc::c_ulong,
            );
        }
        i += 1
    }
    // free old clients arrays
    crate::src::qcommon::common::Z_Free(
        crate::src::server::sv_main::svs.clients as *mut libc::c_void,
    );
    // allocate new clients
    crate::src::server::sv_main::svs.clients = crate::src::qcommon::common::Z_MallocDebug(
        ((*crate::src::server::sv_main::sv_maxclients).integer as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::server_h::client_t>() as libc::c_ulong)
            as libc::c_int,
        b"sv_maxclients->integer * sizeof(client_t)\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/server/sv_init.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        337 as libc::c_int,
    ) as *mut crate::server_h::client_t;
    crate::stdlib::memset(
        crate::src::server::sv_main::svs.clients as *mut libc::c_void,
        0 as libc::c_int,
        ((*crate::src::server::sv_main::sv_maxclients).integer as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::server_h::client_t>() as libc::c_ulong),
    );
    // copy the clients over
    i = 0 as libc::c_int;
    while i < count {
        if (*oldClients.offset(i as isize)).state as libc::c_uint
            >= crate::server_h::CS_CONNECTED as libc::c_int as libc::c_uint
        {
            *crate::src::server::sv_main::svs.clients.offset(i as isize) =
                *oldClients.offset(i as isize)
        }
        i += 1
    }
    // free the old clients on the hunk
    crate::src::qcommon::common::Hunk_FreeTempMemory(oldClients as *mut libc::c_void);
    // allocate new snapshot entities
    if (*crate::src::qcommon::common::com_dedicated).integer != 0 {
        crate::src::server::sv_main::svs.numSnapshotEntities =
            (*crate::src::server::sv_main::sv_maxclients).integer
                * 32 as libc::c_int
                * 256 as libc::c_int
    } else {
        // we don't need nearly as many when playing locally
        crate::src::server::sv_main::svs.numSnapshotEntities =
            (*crate::src::server::sv_main::sv_maxclients).integer
                * 4 as libc::c_int
                * 256 as libc::c_int
    };
}
/*
================
SV_ClearServer
================
*/

unsafe extern "C" fn SV_ClearServer() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        if !crate::src::server::sv_main::sv.configstrings[i as usize].is_null() {
            crate::src::qcommon::common::Z_Free(
                crate::src::server::sv_main::sv.configstrings[i as usize] as *mut libc::c_void,
            );
        }
        i += 1
    }
    crate::stdlib::memset(
        &mut crate::src::server::sv_main::sv as *mut crate::server_h::server_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::server_h::server_t>() as libc::c_ulong,
    );
}
/*
================
SV_TouchFile
================
*/

unsafe extern "C" fn SV_TouchFile(mut filename: *const libc::c_char) {
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    crate::src::qcommon::files::FS_FOpenFileRead(
        filename,
        &mut f,
        crate::src::qcommon::q_shared::qfalse,
    );
    if f != 0 {
        crate::src::qcommon::files::FS_FCloseFile(f);
    };
}
//
// sv_init.c
//
/*
================
SV_SpawnServer

Change the server to a new map, taking all connected
clients along with it.
This is NOT called for map_restart
================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_SpawnServer(
    mut server: *mut libc::c_char,
    mut killBots: crate::src::qcommon::q_shared::qboolean,
) {
    let mut i: libc::c_int = 0;
    let mut checksum: libc::c_int = 0;
    let mut isBot: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut systemInfo: [libc::c_char; 16384] = [0; 16384];
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    // shut down the existing game if it is running
    crate::src::server::sv_game::SV_ShutdownGameProgs();
    crate::src::qcommon::common::Com_Printf(
        b"------ Server Initialization ------\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::common::Com_Printf(
        b"Server: %s\n\x00" as *const u8 as *const libc::c_char,
        server,
    );
    // if not running a dedicated server CL_MapLoading will connect the client to the server
    // also print some status stuff
    crate::src::null::null_client::CL_MapLoading();
    // make sure all the client stuff is unloaded
    crate::src::null::null_client::CL_ShutdownAll(crate::src::qcommon::q_shared::qfalse);
    // clear the whole hunk because we're (re)loading the server
    crate::src::qcommon::common::Hunk_Clear();
    // clear collision map data
    crate::src::qcommon::cm_load::CM_ClearMap();
    // init client structures and svs.numSnapshotEntities
    if crate::src::qcommon::cvar::Cvar_VariableValue(
        b"sv_running\x00" as *const u8 as *const libc::c_char,
    ) == 0.
    {
        SV_Startup();
    } else if (*crate::src::server::sv_main::sv_maxclients).modified as u64 != 0 {
        SV_ChangeMaxClients();
    }
    // check for maxclients change
    // clear pak references
    crate::src::qcommon::files::FS_ClearPakReferences(0 as libc::c_int);
    // allocate the snapshot entities on the hunk
    crate::src::server::sv_main::svs.snapshotEntities = crate::src::qcommon::common::Hunk_AllocDebug(
        (::std::mem::size_of::<crate::src::qcommon::q_shared::entityState_t>() as libc::c_ulong)
            .wrapping_mul(crate::src::server::sv_main::svs.numSnapshotEntities as libc::c_ulong)
            as libc::c_int,
        crate::src::qcommon::q_shared::h_high,
        b"sizeof(entityState_t)*svs.numSnapshotEntities\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/server/sv_init.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        438 as libc::c_int,
    )
        as *mut crate::src::qcommon::q_shared::entityState_t;
    crate::src::server::sv_main::svs.nextSnapshotEntities = 0 as libc::c_int;
    // toggle the server bit so clients can detect that a
    // server has changed
    crate::src::server::sv_main::svs.snapFlagServerBit ^= 4 as libc::c_int;
    // set nextmap to the same map, but it may be overriden
    // by the game startup or another console command
    crate::src::qcommon::cvar::Cvar_Set(
        b"nextmap\x00" as *const u8 as *const libc::c_char,
        b"map_restart 0\x00" as *const u8 as *const libc::c_char,
    );
    //	Cvar_Set( "nextmap", va("map %s", server) );
    i = 0 as libc::c_int;
    while i < (*crate::src::server::sv_main::sv_maxclients).integer {
        // save when the server started for each client already connected
        if (*crate::src::server::sv_main::svs.clients.offset(i as isize)).state as libc::c_uint
            >= crate::server_h::CS_CONNECTED as libc::c_int as libc::c_uint
        {
            (*crate::src::server::sv_main::svs.clients.offset(i as isize)).oldServerTime =
                crate::src::server::sv_main::sv.time
        }
        i += 1
    }
    // wipe the entire per-level structure
    SV_ClearServer();
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        crate::src::server::sv_main::sv.configstrings[i as usize] =
            crate::src::qcommon::common::CopyString(b"\x00" as *const u8 as *const libc::c_char);
        i += 1
    }
    // make sure we are not paused
    crate::src::qcommon::cvar::Cvar_Set(
        b"cl_paused\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
    // get a new checksum feed and restart the file system
    crate::src::server::sv_main::sv.checksumFeed = ((crate::stdlib::rand() as libc::c_uint)
        << 16 as libc::c_int
        ^ crate::stdlib::rand() as libc::c_uint
        ^ crate::src::qcommon::common::Com_Milliseconds() as libc::c_uint)
        as libc::c_int;
    crate::src::qcommon::files::FS_Restart(crate::src::server::sv_main::sv.checksumFeed);
    crate::src::qcommon::cm_load::CM_LoadMap(
        crate::src::qcommon::q_shared::va(
            b"maps/%s.bsp\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            server,
        ),
        crate::src::qcommon::q_shared::qfalse,
        &mut checksum,
    );
    // set serverinfo visible name
    crate::src::qcommon::cvar::Cvar_Set(b"mapname\x00" as *const u8 as *const libc::c_char, server);
    crate::src::qcommon::cvar::Cvar_Set(
        b"sv_mapChecksum\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            checksum,
        ),
    );
    // serverid should be different each time
    crate::src::server::sv_main::sv.serverId = crate::src::qcommon::common::com_frameTime; // I suppose the init here is just to be safe
    crate::src::server::sv_main::sv.restartedServerId = crate::src::server::sv_main::sv.serverId;
    crate::src::server::sv_main::sv.checksumFeedServerId = crate::src::server::sv_main::sv.serverId;
    crate::src::qcommon::cvar::Cvar_Set(
        b"sv_serverid\x00" as *const u8 as *const libc::c_char,
        crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            crate::src::server::sv_main::sv.serverId,
        ),
    );
    // clear physics interaction links
    crate::src::server::sv_world::SV_ClearWorld();
    // media configstring setting should be done during
    // the loading stage, so connected clients don't have
    // to load during actual gameplay
    crate::src::server::sv_main::sv.state = crate::server_h::SS_LOADING;
    // load and spawn all other entities
    crate::src::server::sv_game::SV_InitGameProgs();
    // don't allow a map_restart if game is modified
    (*crate::src::server::sv_main::sv_gametype).modified = crate::src::qcommon::q_shared::qfalse;
    // run a few frames to allow everything to settle
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        crate::src::vm::vm::VM_Call(
            crate::src::server::sv_main::gvm,
            crate::g_public_h::GAME_RUN_FRAME as libc::c_int,
            crate::src::server::sv_main::sv.time,
        );
        crate::src::server::sv_bot::SV_BotFrame(crate::src::server::sv_main::sv.time);
        crate::src::server::sv_main::sv.time += 100 as libc::c_int;
        crate::src::server::sv_main::svs.time += 100 as libc::c_int;
        i += 1
    }
    // create a baseline for more efficient communications
    SV_CreateBaseline();
    let mut current_block_73: u64;
    i = 0 as libc::c_int;
    while i < (*crate::src::server::sv_main::sv_maxclients).integer {
        // send the new gamestate to all connected clients
        if (*crate::src::server::sv_main::svs.clients.offset(i as isize)).state as libc::c_uint
            >= crate::server_h::CS_CONNECTED as libc::c_int as libc::c_uint
        {
            let mut denied: *mut libc::c_char = 0 as *mut libc::c_char;
            if (*crate::src::server::sv_main::svs.clients.offset(i as isize))
                .netchan
                .remoteAddress
                .type_0 as libc::c_uint
                == crate::qcommon_h::NA_BOT as libc::c_int as libc::c_uint
            {
                if killBots as u64 != 0 {
                    crate::src::server::sv_client::SV_DropClient(
                        &mut *crate::src::server::sv_main::svs.clients.offset(i as isize),
                        b"\x00" as *const u8 as *const libc::c_char,
                    );
                    current_block_73 = 13460095289871124136;
                } else {
                    isBot = crate::src::qcommon::q_shared::qtrue;
                    current_block_73 = 6560072651652764009;
                }
            } else {
                isBot = crate::src::qcommon::q_shared::qfalse;
                current_block_73 = 6560072651652764009;
            }
            match current_block_73 {
                13460095289871124136 => {}
                _ => {
                    // connect the client again
                    denied = crate::src::vm::vm::VM_ExplicitArgPtr(
                        crate::src::server::sv_main::gvm,
                        crate::src::vm::vm::VM_Call(
                            crate::src::server::sv_main::gvm,
                            crate::g_public_h::GAME_CLIENT_CONNECT as libc::c_int,
                            i,
                            crate::src::qcommon::q_shared::qfalse as libc::c_int,
                            isBot as libc::c_uint,
                        ),
                    ) as *mut libc::c_char; // firstTime = qfalse
                    if !denied.is_null() {
                        // this generally shouldn't happen, because the client
                        // was connected before the level change
                        crate::src::server::sv_client::SV_DropClient(
                            &mut *crate::src::server::sv_main::svs.clients.offset(i as isize),
                            denied,
                        );
                    } else if isBot as u64 == 0 {
                        // when we get the next packet from a connected client,
                        // the new gamestate will be sent
                        (*crate::src::server::sv_main::svs.clients.offset(i as isize)).state =
                            crate::server_h::CS_CONNECTED
                    } else {
                        let mut client: *mut crate::server_h::client_t =
                            0 as *mut crate::server_h::client_t; // generate a snapshot immediately
                        let mut ent: *mut crate::g_public_h::sharedEntity_t =
                            0 as *mut crate::g_public_h::sharedEntity_t;
                        client = &mut *crate::src::server::sv_main::svs.clients.offset(i as isize)
                            as *mut crate::server_h::client_t;
                        (*client).state = crate::server_h::CS_ACTIVE;
                        ent = crate::src::server::sv_game::SV_GentityNum(i);
                        (*ent).s.number = i;
                        (*client).gentity = ent;
                        (*client).deltaMessage = -(1 as libc::c_int);
                        (*client).lastSnapshotTime = 0 as libc::c_int;
                        crate::src::vm::vm::VM_Call(
                            crate::src::server::sv_main::gvm,
                            crate::g_public_h::GAME_CLIENT_BEGIN as libc::c_int,
                            i,
                        );
                    }
                }
            }
        }
        i += 1
    }
    // run another frame to allow things to look at all the players
    crate::src::vm::vm::VM_Call(
        crate::src::server::sv_main::gvm,
        crate::g_public_h::GAME_RUN_FRAME as libc::c_int,
        crate::src::server::sv_main::sv.time,
    );
    crate::src::server::sv_bot::SV_BotFrame(crate::src::server::sv_main::sv.time);
    crate::src::server::sv_main::sv.time += 100 as libc::c_int;
    crate::src::server::sv_main::svs.time += 100 as libc::c_int;
    if (*crate::src::server::sv_main::sv_pure).integer != 0 {
        // the server sends these to the clients so they will only
        // load pk3s also loaded at the server
        p = crate::src::qcommon::files::FS_LoadedPakChecksums();
        crate::src::qcommon::cvar::Cvar_Set(b"sv_paks\x00" as *const u8 as *const libc::c_char, p);
        if crate::stdlib::strlen(p) == 0 as libc::c_int as libc::c_ulong {
            crate::src::qcommon::common::Com_Printf(
                b"WARNING: sv_pure set but no PK3 files loaded\n\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        p = crate::src::qcommon::files::FS_LoadedPakNames();
        crate::src::qcommon::cvar::Cvar_Set(
            b"sv_pakNames\x00" as *const u8 as *const libc::c_char,
            p,
        );
        // we need to touch the cgame and ui qvm because they could be in
        // separate pk3 files and the client will need to download the pk3
        // files with the latest cgame and ui qvm to pass the pure check
        SV_TouchFile(b"vm/cgame.qvm\x00" as *const u8 as *const libc::c_char);
        SV_TouchFile(b"vm/ui.qvm\x00" as *const u8 as *const libc::c_char);
    } else {
        crate::src::qcommon::cvar::Cvar_Set(
            b"sv_paks\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char,
        );
        crate::src::qcommon::cvar::Cvar_Set(
            b"sv_pakNames\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char,
        );
    }
    // the server sends these to the clients so they can figure
    // out which pk3s should be auto-downloaded
    p = crate::src::qcommon::files::FS_ReferencedPakChecksums();
    crate::src::qcommon::cvar::Cvar_Set(
        b"sv_referencedPaks\x00" as *const u8 as *const libc::c_char,
        p,
    );
    p = crate::src::qcommon::files::FS_ReferencedPakNames();
    crate::src::qcommon::cvar::Cvar_Set(
        b"sv_referencedPakNames\x00" as *const u8 as *const libc::c_char,
        p,
    );
    // save systeminfo and serverinfo strings
    crate::src::qcommon::q_shared::Q_strncpyz(
        systemInfo.as_mut_ptr(),
        crate::src::qcommon::cvar::Cvar_InfoString_Big(0x8 as libc::c_int),
        ::std::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong as libc::c_int,
    );
    crate::src::qcommon::cvar::cvar_modifiedFlags &= !(0x8 as libc::c_int);
    SV_SetConfigstring(1 as libc::c_int, systemInfo.as_mut_ptr());
    SV_SetConfigstring(
        0 as libc::c_int,
        crate::src::qcommon::cvar::Cvar_InfoString(0x4 as libc::c_int),
    );
    crate::src::qcommon::cvar::cvar_modifiedFlags &= !(0x4 as libc::c_int);
    // any media configstring setting now should issue a warning
    // and any configstring changes should be reliably transmitted
    // to all clients
    crate::src::server::sv_main::sv.state = crate::server_h::SS_GAME;
    // send a heartbeat now so the master will get up to date info
    crate::src::server::sv_ccmds::SV_Heartbeat_f();
    crate::src::qcommon::common::Hunk_SetMark();
    crate::src::qcommon::common::Com_Printf(
        b"-----------------------------------\n\x00" as *const u8 as *const libc::c_char,
    );
}
/*
===============
SV_Init

Only called at main exe startup, not for each game
===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_Init() {
    let mut index: libc::c_int = 0;
    crate::src::server::sv_ccmds::SV_AddOperatorCommands();
    // serverinfo vars
    crate::src::qcommon::cvar::Cvar_Get(
        b"dmflags\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x4 as libc::c_int,
    );
    crate::src::qcommon::cvar::Cvar_Get(
        b"fraglimit\x00" as *const u8 as *const libc::c_char,
        b"20\x00" as *const u8 as *const libc::c_char,
        0x4 as libc::c_int,
    );
    crate::src::qcommon::cvar::Cvar_Get(
        b"timelimit\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x4 as libc::c_int,
    );
    crate::src::server::sv_main::sv_gametype = crate::src::qcommon::cvar::Cvar_Get(
        b"g_gametype\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x4 as libc::c_int | 0x20 as libc::c_int,
    );
    crate::src::qcommon::cvar::Cvar_Get(
        b"sv_keywords\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x4 as libc::c_int,
    );
    crate::src::server::sv_main::sv_mapname = crate::src::qcommon::cvar::Cvar_Get(
        b"mapname\x00" as *const u8 as *const libc::c_char,
        b"nomap\x00" as *const u8 as *const libc::c_char,
        0x4 as libc::c_int | 0x40 as libc::c_int,
    );
    crate::src::server::sv_main::sv_privateClients = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_privateClients\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x4 as libc::c_int,
    );
    crate::src::server::sv_main::sv_hostname = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_hostname\x00" as *const u8 as *const libc::c_char,
        b"noname\x00" as *const u8 as *const libc::c_char,
        0x4 as libc::c_int | 0x1 as libc::c_int,
    );
    crate::src::server::sv_main::sv_maxclients = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
        b"8\x00" as *const u8 as *const libc::c_char,
        0x4 as libc::c_int | 0x20 as libc::c_int,
    );
    crate::src::server::sv_main::sv_minRate = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_minRate\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int | 0x4 as libc::c_int,
    );
    crate::src::server::sv_main::sv_maxRate = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_maxRate\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int | 0x4 as libc::c_int,
    );
    crate::src::server::sv_main::sv_dlRate = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_dlRate\x00" as *const u8 as *const libc::c_char,
        b"100\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int | 0x4 as libc::c_int,
    );
    crate::src::server::sv_main::sv_minPing = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_minPing\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int | 0x4 as libc::c_int,
    );
    crate::src::server::sv_main::sv_maxPing = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_maxPing\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int | 0x4 as libc::c_int,
    );
    crate::src::server::sv_main::sv_floodProtect = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_floodProtect\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int | 0x4 as libc::c_int,
    );
    // systeminfo
    crate::src::qcommon::cvar::Cvar_Get(
        b"sv_cheats\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x8 as libc::c_int | 0x40 as libc::c_int,
    );
    crate::src::server::sv_main::sv_serverid = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_serverid\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x8 as libc::c_int | 0x40 as libc::c_int,
    );
    crate::src::server::sv_main::sv_pure = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_pure\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x8 as libc::c_int,
    );
    crate::src::qcommon::cvar::Cvar_Get(
        b"sv_paks\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x8 as libc::c_int | 0x40 as libc::c_int,
    );
    crate::src::qcommon::cvar::Cvar_Get(
        b"sv_pakNames\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x8 as libc::c_int | 0x40 as libc::c_int,
    );
    crate::src::qcommon::cvar::Cvar_Get(
        b"sv_referencedPaks\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x8 as libc::c_int | 0x40 as libc::c_int,
    );
    crate::src::qcommon::cvar::Cvar_Get(
        b"sv_referencedPakNames\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x8 as libc::c_int | 0x40 as libc::c_int,
    );
    // server vars
    crate::src::server::sv_main::sv_rconPassword = crate::src::qcommon::cvar::Cvar_Get(
        b"rconPassword\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x100 as libc::c_int,
    );
    crate::src::server::sv_main::sv_privatePassword = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_privatePassword\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x100 as libc::c_int,
    );
    crate::src::server::sv_main::sv_fps = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_fps\x00" as *const u8 as *const libc::c_char,
        b"20\x00" as *const u8 as *const libc::c_char,
        0x100 as libc::c_int,
    );
    crate::src::server::sv_main::sv_timeout = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_timeout\x00" as *const u8 as *const libc::c_char,
        b"200\x00" as *const u8 as *const libc::c_char,
        0x100 as libc::c_int,
    );
    crate::src::server::sv_main::sv_zombietime = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_zombietime\x00" as *const u8 as *const libc::c_char,
        b"2\x00" as *const u8 as *const libc::c_char,
        0x100 as libc::c_int,
    );
    crate::src::qcommon::cvar::Cvar_Get(
        b"nextmap\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x100 as libc::c_int,
    );
    crate::src::server::sv_main::sv_allowDownload = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_allowDownload\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x4 as libc::c_int,
    );
    crate::src::qcommon::cvar::Cvar_Get(
        b"sv_dlURL\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x4 as libc::c_int | 0x1 as libc::c_int,
    );
    crate::src::server::sv_main::sv_master[0 as libc::c_int as usize] =
        crate::src::qcommon::cvar::Cvar_Get(
            b"sv_master1\x00" as *const u8 as *const libc::c_char,
            b"master.quake3arena.com\x00" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    crate::src::server::sv_main::sv_master[1 as libc::c_int as usize] =
        crate::src::qcommon::cvar::Cvar_Get(
            b"sv_master2\x00" as *const u8 as *const libc::c_char,
            b"master.ioquake3.org\x00" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    index = 2 as libc::c_int;
    while index < 5 as libc::c_int {
        crate::src::server::sv_main::sv_master[index as usize] =
            crate::src::qcommon::cvar::Cvar_Get(
                crate::src::qcommon::q_shared::va(
                    b"sv_master%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    index + 1 as libc::c_int,
                ),
                b"\x00" as *const u8 as *const libc::c_char,
                0x1 as libc::c_int,
            );
        index += 1
    }
    crate::src::server::sv_main::sv_reconnectlimit = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_reconnectlimit\x00" as *const u8 as *const libc::c_char,
        b"3\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    crate::src::server::sv_main::sv_showloss = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_showloss\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    crate::src::server::sv_main::sv_padPackets = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_padPackets\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    crate::src::server::sv_main::sv_killserver = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_killserver\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    crate::src::server::sv_main::sv_mapChecksum = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_mapChecksum\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x40 as libc::c_int,
    );
    crate::src::server::sv_main::sv_lanForceRate = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_lanForceRate\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    );
    crate::src::server::sv_main::sv_strictAuth = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_strictAuth\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    );
    crate::src::server::sv_main::sv_banFile = crate::src::qcommon::cvar::Cvar_Get(
        b"sv_banFile\x00" as *const u8 as *const libc::c_char,
        b"serverbans.dat\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int,
    );
    // initialize bot cvars so they are listed and can be set before loading the botlib
    crate::src::server::sv_bot::SV_BotInitCvars();
    // init the botlib here because we need the pre-compiler in the UI
    crate::src::server::sv_bot::SV_BotInitBotLib();
    // Load saved bans
    crate::src::qcommon::cmd::Cbuf_AddText(b"rehashbans\n\x00" as *const u8 as *const libc::c_char);
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
// server.h
//=============================================================================
// !!! MUST NOT CHANGE, SERVER AND
// GAME BOTH REFERENCE !!!
// for delta compression of initial sighting
// if -1, use headnode instead
// if all the clusters don't fit in clusternums
// used to prevent double adding from portal views
// no map loaded
// spawning level entities
// actively running
// if true, send configstring changes during SS_LOADING
// changes each server start
// serverId before a map_restart
// the feed key that we use to compute the pure checksum strings
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=475
// the serverId associated with the current checksumFeed (always <= serverId)
// incremented for each snapshot built
// <= 1000 / sv_frame->value
// when time > nextFrameTime, process world
// used during game VM init
// the game virtual machine will update these on init and changes
// current number, <= MAX_GENTITIES
// will be > sizeof(playerState_t) due to game private data
// portalarea visibility bits
// into the circular sv_packet_entities[]
// the entities MUST be in increasing state number
// order, otherwise the delta compression will fail
// time the message was transmitted
// time the message was acked
// used to rate drop packets
// can be reused for a new connection
// client has been disconnected, but don't reuse
// connection for a couple seconds
// has been assigned to a client_t, but no gamestate yet
// gamestate has been sent, but client hasn't sent a usercmd
// client is fully in game
// valid command string for SV_Netchan_Encode
// name, etc
// last added reliable message, not necessarily sent or acknowledged yet
// last acknowledged reliable message
// last sent reliable message, not necessarily acknowledged yet
// netchan->outgoingSequence of gamestate
// for delta compression
// reliable client message sequence
// SV_GentityNum(clientnum)
// extracted from userinfo, high bits masked
// downloading
// if not empty string, we are downloading
// file being downloaded
// total bytes (can't use EOF because of paks)
// bytes sent
// last block we sent to the client, awaiting ack
// current block number
// last block we xmited
// the buffers for the download blocks
// We have sent the EOF block
// time we last got an ack from the client
// frame last client usercmd message
// svs.time when another reliable command will be allowed
// svs.time when packet was last received
// svs.time when connection started
// svs.time of last sent snapshot
// true if nextSnapshotTime was set based on rate instead of snapshotMsec
// must timeout a few frames in a row so debugging doesn't break
// updates can be delta'd from here
// bytes / second
// requests a snapshot every snapshotMsec unless rate choked
// TTimo - additional flag to distinguish between a bad pure checksum, and no cp command at all
// TTimo
// queuing outgoing fragmented messages to send them properly, without udp packet bursts
// in case large fragmented messages are stacking up
// buffer them into this queue, and hand them out to netchan as needed
//=============================================================================
// MAX_CHALLENGES is made large to prevent a denial
// of service attack that could cycle all of them
// out before legitimate users connected
// Allow a certain amount of challenges to have the same IP address
// to make it a bit harder to DOS one single IP address from connecting
// while not allowing a single ip to grab all challenge resources
// challenge number coming from the client
// time the last packet was sent to the autherize server
// time the challenge response was sent to client
// time the adr was first used, for authorize timeout checks
// this structure will be cleared only when the game dll changes
// sv_init has completed
// will be strictly increasing across level changes
// ^= SNAPFLAG_SERVERCOUNT every SV_SpawnServer()
// [sv_maxclients->integer];
// sv_maxclients->integer*PACKET_BACKUP*MAX_SNAPSHOT_ENTITIES
// next snapshotEntities to use
// [numSnapshotEntities]
// to prevent invalid IPs from connecting
// for rcon return messages
// authorize server address
// next svs.time that server should do dns lookup for master server
// Structure for managing bans
// For a CIDR-Notation type suffix
//=============================================================================
// persistant server info across maps
// cleared each map
// game virtual machine
//===========================================================
//
// sv_main.c
//
/*
==================
SV_FinalMessage

Used by SV_Shutdown to send a final message to all
connected clients before the server goes down.  The messages are sent immediately,
not just stuck on the outgoing message list, because the server is going
to totally exit after returning from this function.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_FinalMessage(mut message: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cl: *mut crate::server_h::client_t = 0 as *mut crate::server_h::client_t;
    // send it twice, ignoring rate
    j = 0 as libc::c_int;
    while j < 2 as libc::c_int {
        i = 0 as libc::c_int;
        cl = crate::src::server::sv_main::svs.clients;
        while i < (*crate::src::server::sv_main::sv_maxclients).integer {
            if (*cl).state as libc::c_uint
                >= crate::server_h::CS_CONNECTED as libc::c_int as libc::c_uint
            {
                // don't send a disconnect to a local client
                if (*cl).netchan.remoteAddress.type_0 as libc::c_uint
                    != crate::qcommon_h::NA_LOOPBACK as libc::c_int as libc::c_uint
                {
                    crate::src::server::sv_main::SV_SendServerCommand(
                        cl,
                        b"print \"%s\n\"\n\x00" as *const u8 as *const libc::c_char,
                        message,
                    );
                    crate::src::server::sv_main::SV_SendServerCommand(
                        cl,
                        b"disconnect \"%s\"\x00" as *const u8 as *const libc::c_char,
                        message,
                    );
                }
                // force a snapshot to be sent
                (*cl).lastSnapshotTime = 0 as libc::c_int;
                crate::src::server::sv_snapshot::SV_SendClientSnapshot(cl);
            }
            i += 1;
            cl = cl.offset(1)
        }
        j += 1
    }
}
// will properly create any needed paths and deal with seperater character issues
// if uniqueFILE is true, then a new FILE will be fopened even if the file
// is found in an already open pak file.  If uniqueFILE is false, you must call
// FS_FCloseFile instead of fclose, otherwise the pak FILE would be improperly closed
// It is generally safe to always set uniqueFILE to true, because the majority of
// file IO goes through FS_ReadFile, which Does The Right Thing already.
// returns 1 if a file is in the PAK file, otherwise -1
// properly handles partial reads and reads from other dlls
// note: you can't just fclose from another DLL, due to MS libc issues
// returns the length of the file
// a null buffer will just return the file length without loading
// as a quick check for existence. -1 length == not present
// A 0 byte will always be appended at the end, so string ops are safe.
// the buffer should be considered read-only, because it may be cached
// for other uses.
// forces flush on files we're writing to.
// frees the memory returned by FS_ReadFile
// writes a complete file, creating any subdirectories needed
// doesn't work for files that are opened from a pack file
// where are we?
// like fprintf
// opens a file for reading, writing, or appending depending on the value of mode
// seek on a file
// Returns a space separated string containing the checksums of all loaded pk3 files.
// Servers with sv_pure set will get this string and pass it to clients.
// Returns a space separated string containing the checksums of all loaded
// AND referenced pk3 files. Servers with sv_pure set will get this string
// back from clients for pure validation
// clears referenced booleans on loaded pk3s
// If the string is empty, all data sources will be allowed.
// If not empty, only pk3 files that match one of the space
// separated checksums will be checked for files, with the
// sole exception of .cfg files.
/*
==============================================================

Edit fields and command line history/completion

==============================================================
*/
/*
==============================================================

MISC

==============================================================
*/
// centralizing the declarations for cl_cdkey
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=470
// returned by Sys_GetProcessorFeatures
// centralized and cleaned, that's the max string you can send to a Com_Printf / Com_DPrintf (above gets truncated)
// SE_NONE must be zero
// evTime is still valid
// evValue is a key code, evValue2 is the down flag
// evValue is an ascii char
// evValue and evValue2 are relative signed x / y moves
// evValue is an axis number and evValue2 is the current state (-127 to 127)
// evPtr is a char*
// bytes of data pointed to by evPtr, for journaling
// this must be manually freed if not NULL
// will be journaled properly
// checks for and removes command line "+set var arg" constructs
// if match is NULL, all set commands will be executed, otherwise
// only a set with the exact name.  Only used during startup.
// for building release pak files
// both client and server must agree to pause
// com_speeds times
// renderer backend time
/*

--- low memory ----
server vm
server clipmap
---mark---
renderer initialization (shaders, etc)
UI vm
cgame vm
renderer map
renderer models

---free---

temp file loading
--- high memory ---

*/
// NOT 0 filled memory
// returns 0 filled memory
// returns 0 filled memory
// commandLine should not include the executable name (argv[0])
/*
==============================================================

CLIENT / SERVER SYSTEMS

==============================================================
*/
//
// client interface
//
// the keyboard binding interface must be setup before execing
// config files, but the rest of client startup will happen later
// char events are for field typing, not game control
// do a screen update before starting to load a map
// when the server is going to load a new map, the entire hunk
// will be cleared, so the client must shutdown cgame, ui, and
// the renderer
// adds the current command line as a clc_clientCommand to the client message.
// things like godmode, noclip, etc, are commands directed to the server,
// so when they are typed in at the console, they will need to be forwarded.
// bring up the "need a cd to play" dialog
// dump all memory on an error
// shutdown client
// initialize renderer interface
// start all the client stuff using the hunk
// Restart sound subsystem
// for keyname autocompletion
// for writing the config files
// call before filesystem access
// FIXME: move logging to common?
// AVI files have the start of pixel lines 4 byte-aligned
//
// server interface
//
/*
================
SV_Shutdown

Called when each game quits,
before Sys_Quit or Sys_Error
================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_Shutdown(mut finalmsg: *mut libc::c_char) {
    if crate::src::qcommon::common::com_sv_running.is_null()
        || (*crate::src::qcommon::common::com_sv_running).integer == 0
    {
        return;
    }
    crate::src::qcommon::common::Com_Printf(
        b"----- Server Shutdown (%s) -----\n\x00" as *const u8 as *const libc::c_char,
        finalmsg,
    );
    crate::src::qcommon::net_ip::NET_LeaveMulticast6();
    if !crate::src::server::sv_main::svs.clients.is_null()
        && crate::src::qcommon::common::com_errorEntered as u64 == 0
    {
        SV_FinalMessage(finalmsg);
    }
    crate::src::server::sv_ccmds::SV_RemoveOperatorCommands();
    crate::src::server::sv_main::SV_MasterShutdown();
    crate::src::server::sv_game::SV_ShutdownGameProgs();
    // free current level
    SV_ClearServer();
    // free server static data
    if !crate::src::server::sv_main::svs.clients.is_null() {
        let mut index: libc::c_int = 0;
        index = 0 as libc::c_int;
        while index < (*crate::src::server::sv_main::sv_maxclients).integer {
            crate::src::server::sv_client::SV_FreeClient(
                &mut *crate::src::server::sv_main::svs
                    .clients
                    .offset(index as isize),
            );
            index += 1
        }
        crate::src::qcommon::common::Z_Free(
            crate::src::server::sv_main::svs.clients as *mut libc::c_void,
        );
    }
    crate::stdlib::memset(
        &mut crate::src::server::sv_main::svs as *mut crate::server_h::serverStatic_t
            as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<crate::server_h::serverStatic_t>() as libc::c_ulong,
    );
    crate::src::qcommon::cvar::Cvar_Set(
        b"sv_running\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cvar::Cvar_Set(
        b"ui_singlePlayerActive\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::common::Com_Printf(
        b"---------------------------\n\x00" as *const u8 as *const libc::c_char,
    );
    // disconnect any local clients
    if (*crate::src::server::sv_main::sv_killserver).integer != 2 as libc::c_int {
        crate::src::null::null_client::CL_Disconnect(crate::src::qcommon::q_shared::qfalse);
    };
}
