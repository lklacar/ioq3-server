use ::libc;

pub use crate::g_public_h::entityShared_t;
pub use crate::g_public_h::sharedEntity_t;
pub use crate::qcommon_h::msg_t;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::netchan_t;
pub use crate::qcommon_h::netsrc_t;
pub use crate::qcommon_h::svc_EOF;
pub use crate::qcommon_h::svc_bad;
pub use crate::qcommon_h::svc_baseline;
pub use crate::qcommon_h::svc_configstring;
pub use crate::qcommon_h::svc_download;
pub use crate::qcommon_h::svc_gamestate;
pub use crate::qcommon_h::svc_nop;
pub use crate::qcommon_h::svc_ops_e;
pub use crate::qcommon_h::svc_serverCommand;
pub use crate::qcommon_h::svc_snapshot;
pub use crate::qcommon_h::svc_voipOpus;
pub use crate::qcommon_h::svc_voipSpeex;
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
pub use crate::server_h::clientSnapshot_t;
pub use crate::server_h::clientState_t;
pub use crate::server_h::client_s;
pub use crate::server_h::client_t;
pub use crate::server_h::netchan_buffer_s;
pub use crate::server_h::netchan_buffer_t;
pub use crate::server_h::CS_ACTIVE;
pub use crate::server_h::CS_CONNECTED;
pub use crate::server_h::CS_FREE;
pub use crate::server_h::CS_PRIMED;
pub use crate::server_h::CS_ZOMBIE;
pub use crate::src::qcommon::common::Com_DPrintf;
pub use crate::src::qcommon::common::Z_Free;
pub use crate::src::qcommon::common::Z_MallocDebug;
pub use crate::src::qcommon::msg::MSG_Copy;
pub use crate::src::qcommon::msg::MSG_ReadLong;
pub use crate::src::qcommon::msg::MSG_WriteByte;
pub use crate::src::qcommon::net_chan::Netchan_Process;
pub use crate::src::qcommon::net_chan::Netchan_Transmit;
pub use crate::src::qcommon::net_chan::Netchan_TransmitNextFragment;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
pub use crate::src::server::sv_main::SV_RateMsec;
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
==============
SV_Netchan_Encode

    // first four bytes of the data are always:
    long reliableAcknowledge;

==============
*/

unsafe extern "C" fn SV_Netchan_Encode(
    mut client: *mut crate::server_h::client_t,
    mut msg: *mut crate::qcommon_h::msg_t,
    mut clientCommandString: *const libc::c_char,
) {
    let mut i: libc::c_long = 0;
    let mut index: libc::c_long = 0;
    let mut key: crate::src::qcommon::q_shared::byte = 0;
    let mut string: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut srdc: libc::c_int = 0;
    let mut sbit: libc::c_int = 0;
    let mut soob: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    if (*msg).cursize < 4 as libc::c_int {
        return;
    }
    srdc = (*msg).readcount;
    sbit = (*msg).bit;
    soob = (*msg).oob;
    (*msg).bit = 0 as libc::c_int;
    (*msg).readcount = 0 as libc::c_int;
    (*msg).oob = crate::src::qcommon::q_shared::qfalse;
    /* reliableAcknowledge = */
    crate::src::qcommon::msg::MSG_ReadLong(msg);
    (*msg).oob = soob;
    (*msg).bit = sbit;
    (*msg).readcount = srdc;
    string = clientCommandString as *mut crate::src::qcommon::q_shared::byte;
    index = 0 as libc::c_int as libc::c_long;
    // xor the client challenge with the netchan sequence number
    key = ((*client).challenge ^ (*client).netchan.outgoingSequence)
        as crate::src::qcommon::q_shared::byte;
    i = 4 as libc::c_int as libc::c_long;
    while i < (*msg).cursize as libc::c_long {
        // modify the key with the last received and with this message acknowledged client command
        if *string.offset(index as isize) == 0 {
            index = 0 as libc::c_int as libc::c_long
        }
        if *string.offset(index as isize) as libc::c_int > 127 as libc::c_int
            || *string.offset(index as isize) as libc::c_int == '%' as i32
        {
            key = (key as libc::c_int ^ ('.' as i32) << (i & 1 as libc::c_int as libc::c_long))
                as crate::src::qcommon::q_shared::byte
        } else {
            key = (key as libc::c_int
                ^ (*string.offset(index as isize) as libc::c_int)
                    << (i & 1 as libc::c_int as libc::c_long))
                as crate::src::qcommon::q_shared::byte
        }
        index += 1;
        // encode the data with this key
        *(*msg).data.offset(i as isize) = (*(*msg).data.offset(i as isize) as libc::c_int
            ^ key as libc::c_int)
            as crate::src::qcommon::q_shared::byte;
        i += 1
    }
}
/*
==============
SV_Netchan_Decode

    // first 12 bytes of the data are always:
    long serverId;
    long messageAcknowledge;
    long reliableAcknowledge;

==============
*/

unsafe extern "C" fn SV_Netchan_Decode(
    mut client: *mut crate::server_h::client_t,
    mut msg: *mut crate::qcommon_h::msg_t,
) {
    let mut serverId: libc::c_int = 0;
    let mut messageAcknowledge: libc::c_int = 0;
    let mut reliableAcknowledge: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut srdc: libc::c_int = 0;
    let mut sbit: libc::c_int = 0;
    let mut soob: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut key: crate::src::qcommon::q_shared::byte = 0;
    let mut string: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    srdc = (*msg).readcount;
    sbit = (*msg).bit;
    soob = (*msg).oob;
    (*msg).oob = crate::src::qcommon::q_shared::qfalse;
    serverId = crate::src::qcommon::msg::MSG_ReadLong(msg);
    messageAcknowledge = crate::src::qcommon::msg::MSG_ReadLong(msg);
    reliableAcknowledge = crate::src::qcommon::msg::MSG_ReadLong(msg);
    (*msg).oob = soob;
    (*msg).bit = sbit;
    (*msg).readcount = srdc;
    string = (*client).reliableCommands
        [(reliableAcknowledge & 64 as libc::c_int - 1 as libc::c_int) as usize]
        .as_mut_ptr() as *mut crate::src::qcommon::q_shared::byte;
    index = 0 as libc::c_int;
    //
    key = ((*client).challenge ^ serverId ^ messageAcknowledge)
        as crate::src::qcommon::q_shared::byte;
    i = (*msg).readcount + 12 as libc::c_int;
    while i < (*msg).cursize {
        // modify the key with the last sent and acknowledged server command
        if *string.offset(index as isize) == 0 {
            index = 0 as libc::c_int
        }
        if *string.offset(index as isize) as libc::c_int > 127 as libc::c_int
            || *string.offset(index as isize) as libc::c_int == '%' as i32
        {
            key = (key as libc::c_int ^ ('.' as i32) << (i & 1 as libc::c_int))
                as crate::src::qcommon::q_shared::byte
        } else {
            key = (key as libc::c_int
                ^ (*string.offset(index as isize) as libc::c_int) << (i & 1 as libc::c_int))
                as crate::src::qcommon::q_shared::byte
        }
        index += 1;
        // decode the data with this key
        *(*msg).data.offset(i as isize) = (*(*msg).data.offset(i as isize) as libc::c_int
            ^ key as libc::c_int)
            as crate::src::qcommon::q_shared::byte;
        i += 1
    }
}
/*
=================
SV_Netchan_FreeQueue
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_Netchan_FreeQueue(mut client: *mut crate::server_h::client_t) {
    let mut netbuf: *mut crate::server_h::netchan_buffer_t =
        0 as *mut crate::server_h::netchan_buffer_t;
    let mut next: *mut crate::server_h::netchan_buffer_t =
        0 as *mut crate::server_h::netchan_buffer_t;
    netbuf = (*client).netchan_start_queue;
    while !netbuf.is_null() {
        next = (*netbuf).next;
        crate::src::qcommon::common::Z_Free(netbuf as *mut libc::c_void);
        netbuf = next
    }
    (*client).netchan_start_queue = 0 as *mut crate::server_h::netchan_buffer_t;
    (*client).netchan_end_queue = &mut (*client).netchan_start_queue;
}
/*
=================
SV_Netchan_TransmitNextInQueue
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_Netchan_TransmitNextInQueue(
    mut client: *mut crate::server_h::client_t,
) {
    let mut netbuf: *mut crate::server_h::netchan_buffer_t =
        0 as *mut crate::server_h::netchan_buffer_t;
    crate::src::qcommon::common::Com_DPrintf(
        b"#462 Netchan_TransmitNextFragment: popping a queued message for transmit\n\x00"
            as *const u8 as *const libc::c_char,
    );
    netbuf = (*client).netchan_start_queue;
    if (*client).compat as u64 != 0 {
        SV_Netchan_Encode(
            client,
            &mut (*netbuf).msg,
            (*netbuf).clientCommandString.as_mut_ptr(),
        );
    }
    crate::src::qcommon::net_chan::Netchan_Transmit(
        &mut (*client).netchan,
        (*netbuf).msg.cursize,
        (*netbuf).msg.data,
    );
    // pop from queue
    (*client).netchan_start_queue = (*netbuf).next;
    if (*client).netchan_start_queue.is_null() {
        crate::src::qcommon::common::Com_DPrintf(
            b"#462 Netchan_TransmitNextFragment: emptied queue\n\x00" as *const u8
                as *const libc::c_char,
        );
        (*client).netchan_end_queue = &mut (*client).netchan_start_queue
    } else {
        crate::src::qcommon::common::Com_DPrintf(
            b"#462 Netchan_TransmitNextFragment: remaining queued message\n\x00" as *const u8
                as *const libc::c_char,
        );
    }
    crate::src::qcommon::common::Z_Free(netbuf as *mut libc::c_void);
}
/*
=================
SV_Netchan_TransmitNextFragment
Transmit the next fragment and the next queued packet
Return number of ms until next message can be sent based on throughput given by client rate,
-1 if no packet was sent.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_Netchan_TransmitNextFragment(
    mut client: *mut crate::server_h::client_t,
) -> libc::c_int {
    if (*client).netchan.unsentFragments as u64 != 0 {
        crate::src::qcommon::net_chan::Netchan_TransmitNextFragment(&mut (*client).netchan);
        return crate::src::server::sv_main::SV_RateMsec(client);
    } else {
        if !(*client).netchan_start_queue.is_null() {
            SV_Netchan_TransmitNextInQueue(client);
            return crate::src::server::sv_main::SV_RateMsec(client);
        }
    }
    return -(1 as libc::c_int);
}
/*
===============
SV_Netchan_Transmit
TTimo
https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=462
if there are some unsent fragments (which may happen if the snapshots
and the gamestate are fragmenting, and collide on send for instance)
then buffer them and make sure they get sent in correct order
================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_Netchan_Transmit(
    mut client: *mut crate::server_h::client_t,
    mut msg: *mut crate::qcommon_h::msg_t,
) {
    crate::src::qcommon::msg::MSG_WriteByte(msg, crate::qcommon_h::svc_EOF as libc::c_int);
    if (*client).netchan.unsentFragments as libc::c_uint != 0
        || !(*client).netchan_start_queue.is_null()
    {
        let mut netbuf: *mut crate::server_h::netchan_buffer_t =
            0 as *mut crate::server_h::netchan_buffer_t;
        crate::src::qcommon::common::Com_DPrintf(
            b"#462 SV_Netchan_Transmit: unsent fragments, stacked\n\x00" as *const u8
                as *const libc::c_char,
        );
        netbuf = crate::src::qcommon::common::Z_MallocDebug(
            ::std::mem::size_of::<crate::server_h::netchan_buffer_t>() as libc::c_ulong
                as libc::c_int,
            b"sizeof(netchan_buffer_t)\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            b"/home/luka/Projects/ioq3-server/src/server/sv_net_chan.c\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            232 as libc::c_int,
        ) as *mut crate::server_h::netchan_buffer_t;
        // store the msg, we can't store it encoded, as the encoding depends on stuff we still have to finish sending
        crate::src::qcommon::msg::MSG_Copy(
            &mut (*netbuf).msg,
            (*netbuf).msgBuffer.as_mut_ptr(),
            ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 16384]>() as libc::c_ulong
                as libc::c_int,
            msg,
        );
        if (*client).compat as u64 != 0 {
            crate::src::qcommon::q_shared::Q_strncpyz(
                (*netbuf).clientCommandString.as_mut_ptr(),
                (*client).lastClientCommandString.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            );
        }
        (*netbuf).next = 0 as *mut crate::server_h::netchan_buffer_s;
        // insert it in the queue, the message will be encoded and sent later
        *(*client).netchan_end_queue = netbuf;
        (*client).netchan_end_queue = &mut (**(*client).netchan_end_queue).next
    } else {
        if (*client).compat as u64 != 0 {
            SV_Netchan_Encode(client, msg, (*client).lastClientCommandString.as_mut_ptr());
        }
        crate::src::qcommon::net_chan::Netchan_Transmit(
            &mut (*client).netchan,
            (*msg).cursize,
            (*msg).data,
        );
    };
}
// fills in a table of entity numbers with entities that have bounding boxes
// that intersect the given area.  It is possible for a non-axial bmodel
// to be returned that doesn't actually intersect the area on an exact
// test.
// returns the number of pointers filled in
// The world entity is never returned in this list.
// returns the CONTENTS_* value from the world and all entities at the given point.
// mins and maxs are relative
// if the entire move stays in a solid volume, trace.allsolid will be set,
// trace.startsolid will be set, and trace.fraction will be 0
// if the starting point is in a solid, it will be allowed to move out
// to an open area
// passEntityNum is explicitly excluded from clipping checks (normally ENTITYNUM_NONE)
// clip to a specific entity
//
// sv_net_chan.c
//
/*
=================
Netchan_SV_Process
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_Netchan_Process(
    mut client: *mut crate::server_h::client_t,
    mut msg: *mut crate::qcommon_h::msg_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut ret: libc::c_int = 0;
    ret =
        crate::src::qcommon::net_chan::Netchan_Process(&mut (*client).netchan, msg) as libc::c_int;
    if ret == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*client).compat as u64 != 0 {
        SV_Netchan_Decode(client, msg);
    }
    return crate::src::qcommon::q_shared::qtrue;
}
