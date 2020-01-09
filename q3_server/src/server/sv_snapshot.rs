use ::libc;

pub mod q_shared_h {

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
    // for fast side tests: 0,1,2 = axial, 3 = nonaxial
    // signx + (signy<<1) + (signz<<2), used as lookup during collision
    // a trace is returned when a box is swept through the world
    // if true, plane is not valid
    // if true, the initial point was in a solid area
    // time completed, 1.0 = didn't hit anything
    // final position
    // surface normal at impact, transformed to world space
    // surface hit
    // contents on other side of surface hit
    // entity the contacted sirface is a part of
    // trace->entityNum can also be 0 to (MAX_GENTITIES-1)
    // or ENTITYNUM_NONE, ENTITYNUM_WORLD
    // markfragments are returned by R_MarkFragments()
    //=====================================================================
    // in order from highest priority to lowest
    // if none of the catchers are active, bound key strings will be executed
    // sound channels
    // channel 0 never willingly overrides
    // other channels will allways override a playing sound on that channel
    // menu sounds, etc
    // chat messages, etc
    // announcer voices, etc
    /*
    ========================================================================

      ELEMENTS COMMUNICATED ACROSS THE NET

    ========================================================================
    */
    // snapshot used during connection and for zombies
    // toggled every map_restart so transitions can be detected
    //
    // per-level limits
    //
    // absolute limit
    // don't need to send any more
    // entitynums are communicated with GENTITY_BITS, so any reserved
    // values that are going to be communcated over the net need to
    // also be in this range
    // these are sent over the net as 8 bits
    // so they cannot be blindly increased
    // these are the only configstrings that the system reserves, all the
    // other ones are strictly for servergame to clientgame communication
    // an info string with all the serverinfo cvars
    // an info string for server system to client system configuration (timescale, etc)
    // game can't modify below this, only the system can
    //=========================================================
    // bit field limits
    // playerState_t is the information needed by both the client and server
    // to predict player motion and actions
    // nothing outside of pmove should modify these, or some degree of prediction error
    // will occur
    // you can't add anything to this without modifying the code in msg.c
    // playerState_t is a full superset of entityState_t as it is used by players,
    // so if a playerState_t is transmitted, the entityState_t can be fully derived
    // from it.
    // cmd->serverTime of last executed command
    // for view bobbing and footstep generation
    // ducked, jump_held, etc
    // add to command angles to get view direction
    // changed by spawns, rotating objects, and teleporters
    // ENTITYNUM_NONE = in air
    // don't change low priority animations until this runs out
    // mask off ANIM_TOGGLEBIT
    // don't change low priority animations until this runs out
    // mask off ANIM_TOGGLEBIT
    // a number 0 to 7 that represents the relative angle
    // of movement to the view angle (axial and diagonals)
    // when at rest, the value will remain unchanged
    // used to twist the legs during strafing
    // location of grapple to pull towards if PMF_GRAPPLE_PULL
    // copied to entityState_t->eFlags
    // pmove generated events
    // events set on player from another source
    // ranges from 0 to MAX_CLIENTS-1
    // copied to entityState_t->weapon
    // for fixed views
    // damage feedback
    // when it changes, latch the other parms
    // stats that aren't cleared on death
    // level.time that the powerup runs out
    // jumppad entity hit this frame
    // not communicated over the net at all
    // server to game info for scoreboard
    //====================================================================
    //
    // usercmd_t->button bits, many of which are generated by the client system,
    // so they aren't game/cgame only definitions
    //
    // displays talk balloon and disables actions
    // walking can't just be inferred from MOVE_RUN
    // because a key pressed late in the frame will
    // only generate a small move value for that frame
    // walking will use different animations and
    // won't generate footsteps
    // any key whatsoever
    // if forwardmove or rightmove are >= MOVE_RUN,
    // then BUTTON_WALKING should be set
    // usercmd_t is sent to the server each client frame
    // weapon
    //===================================================================
    // if entityState->solid == SOLID_BMODEL, modelindex is an inline model number
    // non-parametric, but interpolate between snapshots
    // value = base + sin( time / duration ) * delta
    // if non 0, trTime + trDuration = stop time
    // velocity, etc
    // entityState_t is the information conveyed from the server
    // in an update message about entities that the client will
    // need to render in some way
    // Different eTypes may use the information in different ways
    // The messages are delta compressed, so it doesn't really matter if
    // the structure size is fairly large

    #[inline]

    pub unsafe extern "C" fn VectorLengthSquared(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return *v.offset(0 as libc::c_int as isize) * *v.offset(0 as libc::c_int as isize)
            + *v.offset(1 as libc::c_int as isize) * *v.offset(1 as libc::c_int as isize)
            + *v.offset(2 as libc::c_int as isize) * *v.offset(2 as libc::c_int as isize);
    }

    // entity index
    // entityType_t
    // for calculating position
    // for calculating angles
    // shotgun sources, etc
    // ENTITYNUM_NONE = in air
    // r + (g<<8) + (b<<16) + (intensity<<24)
    // constantly loop this sound
    // 0 to (MAX_CLIENTS - 1), for players and corpses
    // for client side prediction, trap_linkentity sets this properly
    // impulse events -- muzzle flashes, footsteps, etc
    // for players
    // bit flags
    // determines weapon and flash model, etc
    // mask off ANIM_TOGGLEBIT
    // mask off ANIM_TOGGLEBIT
    // __Q_SHARED_H
}

pub use crate::stddef_h::size_t;

pub use crate::be_aas_h::C2RustUnnamed_0;
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
use crate::src::qcommon::cm_load::CM_LeafArea;
use crate::src::qcommon::cm_load::CM_LeafCluster;
use crate::src::qcommon::cm_test::CM_AreasConnected;
use crate::src::qcommon::cm_test::CM_ClusterPVS;
use crate::src::qcommon::cm_test::CM_PointLeafnum;
use crate::src::qcommon::cm_test::CM_WriteAreaBits;
pub use crate::src::qcommon::common::com_timescale;
pub use crate::src::qcommon::common::Com_DPrintf;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::msg::MSG_Clear;
pub use crate::src::qcommon::msg::MSG_Init;
pub use crate::src::qcommon::msg::MSG_WriteBits;
pub use crate::src::qcommon::msg::MSG_WriteByte;
pub use crate::src::qcommon::msg::MSG_WriteData;
pub use crate::src::qcommon::msg::MSG_WriteDeltaEntity;
pub use crate::src::qcommon::msg::MSG_WriteDeltaPlayerstate;
pub use crate::src::qcommon::msg::MSG_WriteLong;
pub use crate::src::qcommon::msg::MSG_WriteString;
pub use crate::src::qcommon::net_ip::Sys_IsLANAddress;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
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
pub use crate::src::server::sv_game::SV_GameClientNum;
pub use crate::src::server::sv_game::SV_GentityNum;
pub use crate::src::server::sv_game::SV_SvEntityForGentity;
pub use crate::src::server::sv_main::sv;
pub use crate::src::server::sv_main::sv_lanForceRate;
pub use crate::src::server::sv_main::sv_maxclients;
pub use crate::src::server::sv_main::sv_padPackets;
pub use crate::src::server::sv_main::svs;
pub use crate::src::server::sv_main::SV_RateMsec;
pub use crate::src::server::sv_net_chan::SV_Netchan_Transmit;
pub use crate::src::server::sv_snapshot::q_shared_h::VectorLengthSquared;
pub use crate::stdlib::__compar_fn_t;
use crate::stdlib::memset;
pub use crate::stdlib::qsort;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snapshotEntityNumbers_t {
    pub numSnapshotEntities: libc::c_int,
    pub snapshotEntities: [libc::c_int; 256],
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
=============================================================================

Delta encode a client frame onto the network channel

A normal server packet will look like:

4	sequence number (high bit set if an oversize fragment)
<optional reliable commands>
1	svc_snapshot
4	last client reliable command
4	serverTime
1	lastframe for delta compression
1	snapFlags
1	areaBytes
<areabytes>
<playerstate>
<packetentities>

=============================================================================
*/
/*
=============
SV_EmitPacketEntities

Writes a delta update of an entityState_t list to the message.
=============
*/

unsafe extern "C" fn SV_EmitPacketEntities(
    mut from: *mut crate::server_h::clientSnapshot_t,
    mut to: *mut crate::server_h::clientSnapshot_t,
    mut msg: *mut crate::qcommon_h::msg_t,
) {
    let mut oldent: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    let mut newent: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    let mut oldindex: libc::c_int = 0;
    let mut newindex: libc::c_int = 0;
    let mut oldnum: libc::c_int = 0;
    let mut newnum: libc::c_int = 0;
    let mut from_num_entities: libc::c_int = 0;
    // generate the delta update
    if from.is_null() {
        from_num_entities = 0 as libc::c_int
    } else {
        from_num_entities = (*from).num_entities
    }
    newent = 0 as *mut crate::src::qcommon::q_shared::entityState_t;
    oldent = 0 as *mut crate::src::qcommon::q_shared::entityState_t;
    newindex = 0 as libc::c_int;
    oldindex = 0 as libc::c_int;
    while newindex < (*to).num_entities || oldindex < from_num_entities {
        if newindex >= (*to).num_entities {
            newnum = 9999 as libc::c_int
        } else {
            newent = &mut *crate::src::server::sv_main::svs.snapshotEntities.offset(
                (((*to).first_entity + newindex)
                    % crate::src::server::sv_main::svs.numSnapshotEntities)
                    as isize,
            ) as *mut crate::src::qcommon::q_shared::entityState_t;
            newnum = (*newent).number
        }
        if oldindex >= from_num_entities {
            oldnum = 9999 as libc::c_int
        } else {
            oldent = &mut *crate::src::server::sv_main::svs.snapshotEntities.offset(
                (((*from).first_entity + oldindex)
                    % crate::src::server::sv_main::svs.numSnapshotEntities)
                    as isize,
            ) as *mut crate::src::qcommon::q_shared::entityState_t;
            oldnum = (*oldent).number
        }
        if newnum == oldnum {
            // delta update from old position
            // because the force parm is qfalse, this will not result
            // in any bytes being emitted if the entity has not changed at all
            crate::src::qcommon::msg::MSG_WriteDeltaEntity(
                msg,
                oldent,
                newent,
                crate::src::qcommon::q_shared::qfalse,
            );
            oldindex += 1;
            newindex += 1
        } else if newnum < oldnum {
            // this is a new entity, send it from the baseline
            crate::src::qcommon::msg::MSG_WriteDeltaEntity(
                msg,
                &mut (*crate::src::server::sv_main::sv
                    .svEntities
                    .as_mut_ptr()
                    .offset(newnum as isize))
                .baseline,
                newent,
                crate::src::qcommon::q_shared::qtrue,
            );
            newindex += 1
        } else {
            if !(newnum > oldnum) {
                continue;
            }
            // the old entity isn't present in the new message
            crate::src::qcommon::msg::MSG_WriteDeltaEntity(
                msg,
                oldent,
                0 as *mut crate::src::qcommon::q_shared::entityState_s,
                crate::src::qcommon::q_shared::qtrue,
            );
            oldindex += 1
        }
    }
    crate::src::qcommon::msg::MSG_WriteBits(
        msg,
        ((1 as libc::c_int) << 10 as libc::c_int) - 1 as libc::c_int,
        10 as libc::c_int,
    );
    // end of packetentities
}
/*
==================
SV_WriteSnapshotToClient
==================
*/

unsafe extern "C" fn SV_WriteSnapshotToClient(
    mut client: *mut crate::server_h::client_t,
    mut msg: *mut crate::qcommon_h::msg_t,
) {
    let mut frame: *mut crate::server_h::clientSnapshot_t =
        0 as *mut crate::server_h::clientSnapshot_t;
    let mut oldframe: *mut crate::server_h::clientSnapshot_t =
        0 as *mut crate::server_h::clientSnapshot_t;
    let mut lastframe: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut snapFlags: libc::c_int = 0;
    // this is the snapshot we are creating
    frame = &mut *(*client).frames.as_mut_ptr().offset(
        ((*client).netchan.outgoingSequence & 32 as libc::c_int - 1 as libc::c_int) as isize,
    ) as *mut crate::server_h::clientSnapshot_t;
    // try to use a previous frame as the source for delta compressing the snapshot
    if (*client).deltaMessage <= 0 as libc::c_int
        || (*client).state as libc::c_uint
            != crate::server_h::CS_ACTIVE as libc::c_int as libc::c_uint
    {
        // client is asking for a retransmit
        oldframe = 0 as *mut crate::server_h::clientSnapshot_t;
        lastframe = 0 as libc::c_int
    } else if (*client).netchan.outgoingSequence - (*client).deltaMessage
        >= 32 as libc::c_int - 3 as libc::c_int
    {
        // client hasn't gotten a good message through in a long time
        crate::src::qcommon::common::Com_DPrintf(
            b"%s: Delta request from out of date packet.\n\x00" as *const u8 as *const libc::c_char,
            (*client).name.as_mut_ptr(),
        );
        oldframe = 0 as *mut crate::server_h::clientSnapshot_t;
        lastframe = 0 as libc::c_int
    } else {
        // we have a valid snapshot to delta from
        oldframe = &mut *(*client)
            .frames
            .as_mut_ptr()
            .offset(((*client).deltaMessage & 32 as libc::c_int - 1 as libc::c_int) as isize)
            as *mut crate::server_h::clientSnapshot_t;
        lastframe = (*client).netchan.outgoingSequence - (*client).deltaMessage;
        // the snapshot's entities may still have rolled off the buffer, though
        if (*oldframe).first_entity
            <= crate::src::server::sv_main::svs.nextSnapshotEntities
                - crate::src::server::sv_main::svs.numSnapshotEntities
        {
            crate::src::qcommon::common::Com_DPrintf(
                b"%s: Delta request from out of date entities.\n\x00" as *const u8
                    as *const libc::c_char,
                (*client).name.as_mut_ptr(),
            );
            oldframe = 0 as *mut crate::server_h::clientSnapshot_t;
            lastframe = 0 as libc::c_int
        }
    }
    crate::src::qcommon::msg::MSG_WriteByte(msg, crate::qcommon_h::svc_snapshot as libc::c_int);
    // NOTE, MRE: now sent at the start of every message from server to client
    // let the client know which reliable clientCommands we have received
    //MSG_WriteLong( msg, client->lastClientCommand );
    // send over the current server time so the client can drift
    // its view of time to try to match
    if (*client).oldServerTime != 0 {
        // The server has not yet got an acknowledgement of the
        // new gamestate from this client, so continue to send it
        // a time as if the server has not restarted. Note from
        // the client's perspective this time is strictly speaking
        // incorrect, but since it'll be busy loading a map at
        // the time it doesn't really matter.
        crate::src::qcommon::msg::MSG_WriteLong(
            msg,
            crate::src::server::sv_main::sv.time + (*client).oldServerTime,
        );
    } else {
        crate::src::qcommon::msg::MSG_WriteLong(msg, crate::src::server::sv_main::sv.time);
    }
    // what we are delta'ing from
    crate::src::qcommon::msg::MSG_WriteByte(msg, lastframe);
    snapFlags = crate::src::server::sv_main::svs.snapFlagServerBit;
    if (*client).rateDelayed as u64 != 0 {
        snapFlags |= 1 as libc::c_int
    }
    if (*client).state as libc::c_uint != crate::server_h::CS_ACTIVE as libc::c_int as libc::c_uint
    {
        snapFlags |= 2 as libc::c_int
    }
    crate::src::qcommon::msg::MSG_WriteByte(msg, snapFlags);
    // send over the areabits
    crate::src::qcommon::msg::MSG_WriteByte(msg, (*frame).areabytes);
    crate::src::qcommon::msg::MSG_WriteData(
        msg,
        (*frame).areabits.as_mut_ptr() as *const libc::c_void,
        (*frame).areabytes,
    );
    // delta encode the playerstate
    if !oldframe.is_null() {
        crate::src::qcommon::msg::MSG_WriteDeltaPlayerstate(
            msg,
            &mut (*oldframe).ps,
            &mut (*frame).ps,
        );
    } else {
        crate::src::qcommon::msg::MSG_WriteDeltaPlayerstate(
            msg,
            0 as *mut crate::src::qcommon::q_shared::playerState_s,
            &mut (*frame).ps,
        );
    }
    // delta encode the entities
    SV_EmitPacketEntities(oldframe, frame, msg);
    // padding for rate debugging
    if (*crate::src::server::sv_main::sv_padPackets).integer != 0 {
        i = 0 as libc::c_int;
        while i < (*crate::src::server::sv_main::sv_padPackets).integer {
            crate::src::qcommon::msg::MSG_WriteByte(msg, crate::qcommon_h::svc_nop as libc::c_int);
            i += 1
        }
    };
}
/*
==================
SV_UpdateServerCommandsToClient

(re)send all server commands the client hasn't acknowledged yet
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_UpdateServerCommandsToClient(
    mut client: *mut crate::server_h::client_t,
    mut msg: *mut crate::qcommon_h::msg_t,
) {
    let mut i: libc::c_int = 0;
    // write any unacknowledged serverCommands
    i = (*client).reliableAcknowledge + 1 as libc::c_int;
    while i <= (*client).reliableSequence {
        crate::src::qcommon::msg::MSG_WriteByte(
            msg,
            crate::qcommon_h::svc_serverCommand as libc::c_int,
        );
        crate::src::qcommon::msg::MSG_WriteLong(msg, i);
        crate::src::qcommon::msg::MSG_WriteString(
            msg,
            (*client).reliableCommands[(i & 64 as libc::c_int - 1 as libc::c_int) as usize]
                .as_mut_ptr(),
        );
        i += 1
    }
    (*client).reliableSent = (*client).reliableSequence;
}
/*
=======================
SV_QsortEntityNumbers
=======================
*/

unsafe extern "C" fn SV_QsortEntityNumbers(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut ea: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut eb: *mut libc::c_int = 0 as *mut libc::c_int;
    ea = a as *mut libc::c_int;
    eb = b as *mut libc::c_int;
    if *ea == *eb {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"SV_QsortEntityStates: duplicated entity\x00" as *const u8 as *const libc::c_char,
        );
    }
    if *ea < *eb {
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}
/*
===============
SV_AddEntToSnapshot
===============
*/

unsafe extern "C" fn SV_AddEntToSnapshot(
    mut svEnt: *mut crate::server_h::svEntity_t,
    mut gEnt: *mut crate::g_public_h::sharedEntity_t,
    mut eNums: *mut snapshotEntityNumbers_t,
) {
    // if we have already added this entity to this snapshot, don't add again
    if (*svEnt).snapshotCounter == crate::src::server::sv_main::sv.snapshotCounter {
        return;
    }
    (*svEnt).snapshotCounter = crate::src::server::sv_main::sv.snapshotCounter;
    // if we are full, silently discard entities
    if (*eNums).numSnapshotEntities == 256 as libc::c_int {
        return;
    }
    (*eNums).snapshotEntities[(*eNums).numSnapshotEntities as usize] = (*gEnt).s.number;
    (*eNums).numSnapshotEntities += 1;
}
/*
===============
SV_AddEntitiesVisibleFromPoint
===============
*/

unsafe extern "C" fn SV_AddEntitiesVisibleFromPoint(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut frame: *mut crate::server_h::clientSnapshot_t,
    mut eNums: *mut snapshotEntityNumbers_t,
    mut portal: crate::src::qcommon::q_shared::qboolean,
) {
    let mut e: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ent: *mut crate::g_public_h::sharedEntity_t =
        0 as *mut crate::g_public_h::sharedEntity_t;
    let mut svEnt: *mut crate::server_h::svEntity_t = 0 as *mut crate::server_h::svEntity_t;
    let mut l: libc::c_int = 0;
    let mut clientarea: libc::c_int = 0;
    let mut clientcluster: libc::c_int = 0;
    let mut leafnum: libc::c_int = 0;
    let mut clientpvs: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut bitvector: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    // during an error shutdown message we may need to transmit
    // the shutdown message after the server has shutdown, so
    // specfically check for it
    if crate::src::server::sv_main::sv.state as u64 == 0 {
        return;
    }
    leafnum = crate::src::qcommon::cm_test::CM_PointLeafnum(
        origin as *const crate::src::qcommon::q_shared::vec_t,
    );
    clientarea = crate::src::qcommon::cm_load::CM_LeafArea(leafnum);
    clientcluster = crate::src::qcommon::cm_load::CM_LeafCluster(leafnum);
    // calculate the visible areas
    (*frame).areabytes =
        crate::src::qcommon::cm_test::CM_WriteAreaBits((*frame).areabits.as_mut_ptr(), clientarea);
    clientpvs = crate::src::qcommon::cm_test::CM_ClusterPVS(clientcluster);
    let mut current_block_26: u64;
    e = 0 as libc::c_int;
    while e < crate::src::server::sv_main::sv.num_entities {
        ent = crate::src::server::sv_game::SV_GentityNum(e);
        // never send entities that aren't linked in
        if !((*ent).r.linked as u64 == 0) {
            if (*ent).s.number != e {
                crate::src::qcommon::common::Com_DPrintf(
                    b"FIXING ENT->S.NUMBER!!!\n\x00" as *const u8 as *const libc::c_char,
                );
                (*ent).s.number = e
            }
            // entities can be flagged to explicitly not be sent to the client
            if !((*ent).r.svFlags & 0x1 as libc::c_int != 0) {
                // entities can be flagged to be sent to only one client
                if (*ent).r.svFlags & 0x100 as libc::c_int != 0 {
                    if (*ent).r.singleClient != (*frame).ps.clientNum {
                        current_block_26 = 7651349459974463963;
                    } else {
                        current_block_26 = 13472856163611868459;
                    }
                } else {
                    current_block_26 = 13472856163611868459;
                }
                match current_block_26 {
                    7651349459974463963 => {}
                    _ =>
                    // entities can be flagged to be sent to everyone but one client
                    {
                        if (*ent).r.svFlags & 0x800 as libc::c_int != 0 {
                            if (*ent).r.singleClient == (*frame).ps.clientNum {
                                current_block_26 = 7651349459974463963;
                            } else {
                                current_block_26 = 13550086250199790493;
                            }
                        } else {
                            current_block_26 = 13550086250199790493;
                        }
                        match current_block_26 {
                            7651349459974463963 => {}
                            _ =>
                            // entities can be flagged to be sent to a given mask of clients
                            {
                                if (*ent).r.svFlags & 0x2 as libc::c_int != 0 {
                                    if (*frame).ps.clientNum >= 32 as libc::c_int {
                                        crate::src::qcommon::common::Com_Error(
                                            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                                            b"SVF_CLIENTMASK: clientNum >= 32\x00" as *const u8
                                                as *const libc::c_char,
                                        );
                                    }
                                    if !(*ent).r.singleClient
                                        & (1 as libc::c_int) << (*frame).ps.clientNum
                                        != 0
                                    {
                                        current_block_26 = 7651349459974463963;
                                    } else {
                                        current_block_26 = 652864300344834934;
                                    }
                                } else {
                                    current_block_26 = 652864300344834934;
                                }
                                match current_block_26 {
                                    7651349459974463963 => {}
                                    _ => {
                                        svEnt =
                                            crate::src::server::sv_game::SV_SvEntityForGentity(ent);
                                        // don't double add an entity through portals
                                        if !((*svEnt).snapshotCounter
                                            == crate::src::server::sv_main::sv.snapshotCounter)
                                        {
                                            // broadcast entities are always sent
                                            if (*ent).r.svFlags & 0x20 as libc::c_int != 0 {
                                                SV_AddEntToSnapshot(svEnt, ent, eNums);
                                            } else {
                                                // ignore if not touching a PV leaf
                                                // check area
                                                if crate::src::qcommon::cm_test::CM_AreasConnected(
                                                    clientarea,
                                                    (*svEnt).areanum,
                                                )
                                                    as u64
                                                    == 0
                                                {
                                                    // doors can legally straddle two areas, so
                                                    // we may need to check another one
                                                    if crate::src::qcommon::cm_test::CM_AreasConnected(clientarea,
                                                                         (*svEnt).areanum2)
                                                           as u64 == 0 {
                                                        current_block_26 =
                                                            7651349459974463963;
                                                        // blocked by a door
                                                    } else {
                                                        current_block_26 =
                                                            7427571413727699167;
                                                    }
                                                } else {
                                                    current_block_26 = 7427571413727699167;
                                                }
                                                match current_block_26 {
                                                    7651349459974463963 => {}
                                                    _ => {
                                                        bitvector = clientpvs;
                                                        // check individual leafs
                                                        if !((*svEnt).numClusters == 0) {
                                                            l = 0 as libc::c_int;
                                                            i = 0 as libc::c_int;
                                                            while i < (*svEnt).numClusters {
                                                                l = (*svEnt).clusternums
                                                                    [i as usize];
                                                                if *bitvector.offset(
                                                                    (l >> 3 as libc::c_int)
                                                                        as isize,
                                                                )
                                                                    as libc::c_int
                                                                    & (1 as libc::c_int)
                                                                        << (l & 7 as libc::c_int)
                                                                    != 0
                                                                {
                                                                    break;
                                                                }
                                                                i += 1
                                                            }
                                                            // if we haven't found it to be visible,
                                                            // check overflow clusters that coudln't be stored
                                                            if i == (*svEnt).numClusters {
                                                                if (*svEnt).lastCluster != 0 {
                                                                    while l <= (*svEnt).lastCluster
                                                                    {
                                                                        if *bitvector.offset(
                                                                            (l >> 3 as libc::c_int)
                                                                                as isize,
                                                                        )
                                                                            as libc::c_int
                                                                            & (1 as libc::c_int)
                                                                                << (l & 7
                                                                                    as libc::c_int)
                                                                            != 0
                                                                        {
                                                                            break;
                                                                        }
                                                                        l += 1
                                                                    }
                                                                    if l == (*svEnt).lastCluster {
                                                                        current_block_26 =
                                                                            7651349459974463963;
                                                                    } else {
                                                                        current_block_26 =
                                                                            9512719473022792396;
                                                                    }
                                                                } else {
                                                                    current_block_26 =
                                                                        7651349459974463963;
                                                                }
                                                            } else {
                                                                current_block_26 =
                                                                    9512719473022792396;
                                                            }
                                                            match current_block_26 {
                                                                7651349459974463963 => {}
                                                                _ => {
                                                                    // add it
                                                                    SV_AddEntToSnapshot(
                                                                        svEnt, ent, eNums,
                                                                    );
                                                                    // if it's a portal entity, add everything visible from its camera position
                                                                    if (*ent).r.svFlags
                                                                        & 0x40 as libc::c_int
                                                                        != 0
                                                                    {
                                                                        if (*ent).s.generic1 != 0 {
                                                                            let mut dir:
                                                                                    crate::src::qcommon::q_shared::vec3_t =
                                                                                [0.;
                                                                                    3];
                                                                            dir[0 as libc::c_int
                                                                                as usize] = (*ent)
                                                                                .s
                                                                                .origin
                                                                                [0 as libc::c_int
                                                                                    as usize]
                                                                                - *origin.offset(
                                                                                    0 as libc::c_int
                                                                                        as isize,
                                                                                );
                                                                            dir[1 as libc::c_int
                                                                                as usize] = (*ent)
                                                                                .s
                                                                                .origin
                                                                                [1 as libc::c_int
                                                                                    as usize]
                                                                                - *origin.offset(
                                                                                    1 as libc::c_int
                                                                                        as isize,
                                                                                );
                                                                            dir[2 as libc::c_int
                                                                                as usize] = (*ent)
                                                                                .s
                                                                                .origin
                                                                                [2 as libc::c_int
                                                                                    as usize]
                                                                                - *origin.offset(
                                                                                    2 as libc::c_int
                                                                                        as isize,
                                                                                );
                                                                            if VectorLengthSquared(dir.as_mut_ptr()
                                                                                                       as
                                                                                                       *const crate::src::qcommon::q_shared::vec_t)
                                                                                   >
                                                                                   (*ent).s.generic1
                                                                                       as
                                                                                       libc::c_float
                                                                                       *
                                                                                       (*ent).s.generic1
                                                                                           as
                                                                                           libc::c_float
                                                                               {
                                                                                current_block_26
                                                                                    =
                                                                                    7651349459974463963;
                                                                            } else {
                                                                                current_block_26
                                                                                    =
                                                                                    8869332144787829186;
                                                                            }
                                                                        } else {
                                                                            current_block_26 =
                                                                                8869332144787829186;
                                                                        }
                                                                        match current_block_26 {
                                                                            7651349459974463963 => {
                                                                            }
                                                                            _ => {
                                                                                SV_AddEntitiesVisibleFromPoint((*ent).s.origin2.as_mut_ptr(),
                                                                                                               frame,
                                                                                                               eNums,
                                                                                                               crate::src::qcommon::q_shared::qtrue);
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
            }
        }
        e += 1
        // not visible
    }
}
/*
=============
SV_BuildClientSnapshot

Decides which entities are going to be visible to the client, and
copies off the playerstate and areabits.

This properly handles multiple recursive portals, but the render
currently doesn't.

For viewing through other player's eyes, clent can be something other than client->gentity
=============
*/

unsafe extern "C" fn SV_BuildClientSnapshot(mut client: *mut crate::server_h::client_t) {
    let mut org: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut frame: *mut crate::server_h::clientSnapshot_t =
        0 as *mut crate::server_h::clientSnapshot_t;
    let mut entityNumbers: snapshotEntityNumbers_t = snapshotEntityNumbers_t {
        numSnapshotEntities: 0,
        snapshotEntities: [0; 256],
    };
    let mut i: libc::c_int = 0;
    let mut ent: *mut crate::g_public_h::sharedEntity_t =
        0 as *mut crate::g_public_h::sharedEntity_t;
    let mut state: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    let mut svEnt: *mut crate::server_h::svEntity_t = 0 as *mut crate::server_h::svEntity_t;
    let mut clent: *mut crate::g_public_h::sharedEntity_t =
        0 as *mut crate::g_public_h::sharedEntity_t;
    let mut clientNum: libc::c_int = 0;
    let mut ps: *mut crate::src::qcommon::q_shared::playerState_t =
        0 as *mut crate::src::qcommon::q_shared::playerState_t;
    // bump the counter used to prevent double adding
    crate::src::server::sv_main::sv.snapshotCounter += 1;
    // this is the frame we are creating
    frame = &mut *(*client).frames.as_mut_ptr().offset(
        ((*client).netchan.outgoingSequence & 32 as libc::c_int - 1 as libc::c_int) as isize,
    ) as *mut crate::server_h::clientSnapshot_t;
    // clear everything in this snapshot
    entityNumbers.numSnapshotEntities = 0 as libc::c_int;
    crate::stdlib::memset(
        (*frame).areabits.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 32]>() as libc::c_ulong,
    );
    // https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=62
    (*frame).num_entities = 0 as libc::c_int;
    clent = (*client).gentity;
    if clent.is_null()
        || (*client).state as libc::c_uint
            == crate::server_h::CS_ZOMBIE as libc::c_int as libc::c_uint
    {
        return;
    }
    // grab the current playerState_t
    ps = crate::src::server::sv_game::SV_GameClientNum(
        client.wrapping_offset_from(crate::src::server::sv_main::svs.clients) as libc::c_long
            as libc::c_int,
    );
    (*frame).ps = *ps;
    // never send client's own entity, because it can
    // be regenerated from the playerstate
    clientNum = (*frame).ps.clientNum;
    if clientNum < 0 as libc::c_int || clientNum >= (1 as libc::c_int) << 10 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"SV_SvEntityForGentity: bad gEnt\x00" as *const u8 as *const libc::c_char,
        );
    }
    svEnt = &mut *crate::src::server::sv_main::sv
        .svEntities
        .as_mut_ptr()
        .offset(clientNum as isize) as *mut crate::server_h::svEntity_t;
    (*svEnt).snapshotCounter = crate::src::server::sv_main::sv.snapshotCounter;
    // find the client's viewpoint
    org[0 as libc::c_int as usize] = (*ps).origin[0 as libc::c_int as usize];
    org[1 as libc::c_int as usize] = (*ps).origin[1 as libc::c_int as usize];
    org[2 as libc::c_int as usize] = (*ps).origin[2 as libc::c_int as usize];
    org[2 as libc::c_int as usize] += (*ps).viewheight as libc::c_float;
    // add all the entities directly visible to the eye, which
    // may include portal entities that merge other viewpoints
    SV_AddEntitiesVisibleFromPoint(
        org.as_mut_ptr(),
        frame,
        &mut entityNumbers,
        crate::src::qcommon::q_shared::qfalse,
    );
    // if there were portals visible, there may be out of order entities
    // in the list which will need to be resorted for the delta compression
    // to work correctly.  This also catches the error condition
    // of an entity being included twice.
    crate::stdlib::qsort(
        entityNumbers.snapshotEntities.as_mut_ptr() as *mut libc::c_void,
        entityNumbers.numSnapshotEntities as crate::stddef_h::size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        Some(
            SV_QsortEntityNumbers
                as unsafe extern "C" fn(
                    _: *const libc::c_void,
                    _: *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    // now that all viewpoint's areabits have been OR'd together, invert
    // all of them to make it a mask vector, which is what the renderer wants
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int / 4 as libc::c_int {
        *((*frame).areabits.as_mut_ptr() as *mut libc::c_int).offset(i as isize) =
            *((*frame).areabits.as_mut_ptr() as *mut libc::c_int).offset(i as isize)
                ^ -(1 as libc::c_int);
        i += 1
    }
    // copy the entity states out
    (*frame).num_entities = 0 as libc::c_int;
    (*frame).first_entity = crate::src::server::sv_main::svs.nextSnapshotEntities;
    i = 0 as libc::c_int;
    while i < entityNumbers.numSnapshotEntities {
        ent =
            crate::src::server::sv_game::SV_GentityNum(entityNumbers.snapshotEntities[i as usize]);
        state = &mut *crate::src::server::sv_main::svs.snapshotEntities.offset(
            (crate::src::server::sv_main::svs.nextSnapshotEntities
                % crate::src::server::sv_main::svs.numSnapshotEntities) as isize,
        ) as *mut crate::src::qcommon::q_shared::entityState_t;
        *state = (*ent).s;
        crate::src::server::sv_main::svs.nextSnapshotEntities += 1;
        // this should never hit, map should always be restarted first in SV_Frame
        if crate::src::server::sv_main::svs.nextSnapshotEntities >= 0x7ffffffe as libc::c_int {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
                b"svs.nextSnapshotEntities wrapped\x00" as *const u8 as *const libc::c_char,
            );
        }
        (*frame).num_entities += 1;
        i += 1
    }
}
/*
=======================
SV_SendMessageToClient

Called by SV_SendClientSnapshot and SV_SendClientGameState
=======================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_SendMessageToClient(
    mut msg: *mut crate::qcommon_h::msg_t,
    mut client: *mut crate::server_h::client_t,
) {
    // record information about the message
    (*client).frames
        [((*client).netchan.outgoingSequence & 32 as libc::c_int - 1 as libc::c_int) as usize]
        .messageSize = (*msg).cursize;
    (*client).frames
        [((*client).netchan.outgoingSequence & 32 as libc::c_int - 1 as libc::c_int) as usize]
        .messageSent = crate::src::server::sv_main::svs.time;
    (*client).frames
        [((*client).netchan.outgoingSequence & 32 as libc::c_int - 1 as libc::c_int) as usize]
        .messageAcked = -(1 as libc::c_int);
    // send the datagram
    crate::src::server::sv_net_chan::SV_Netchan_Transmit(client, msg);
}
/*
=======================
SV_SendClientSnapshot

Also called by SV_FinalMessage

=======================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_SendClientSnapshot(mut client: *mut crate::server_h::client_t) {
    let mut msg_buf: [crate::src::qcommon::q_shared::byte; 16384] = [0; 16384];
    let mut msg: crate::qcommon_h::msg_t = crate::qcommon_h::msg_t {
        allowoverflow: crate::src::qcommon::q_shared::qfalse,
        overflowed: crate::src::qcommon::q_shared::qfalse,
        oob: crate::src::qcommon::q_shared::qfalse,
        data: 0 as *mut crate::src::qcommon::q_shared::byte,
        maxsize: 0,
        cursize: 0,
        readcount: 0,
        bit: 0,
    };
    // build the snapshot
    SV_BuildClientSnapshot(client);
    // bots need to have their snapshots build, but
    // the query them directly without needing to be sent
    if !(*client).gentity.is_null() && (*(*client).gentity).r.svFlags & 0x8 as libc::c_int != 0 {
        return;
    }
    crate::src::qcommon::msg::MSG_Init(
        &mut msg,
        msg_buf.as_mut_ptr(),
        ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 16384]>() as libc::c_ulong
            as libc::c_int,
    );
    msg.allowoverflow = crate::src::qcommon::q_shared::qtrue;
    // NOTE, MRE: all server->client messages now acknowledge
    // let the client know which reliable clientCommands we have received
    crate::src::qcommon::msg::MSG_WriteLong(&mut msg, (*client).lastClientCommand);
    // (re)send any reliable server commands
    SV_UpdateServerCommandsToClient(client, &mut msg);
    // send over all the relevant entityState_t
    // and the playerState_t
    SV_WriteSnapshotToClient(client, &mut msg);
    // check for overflow
    if msg.overflowed as u64 != 0 {
        crate::src::qcommon::common::Com_Printf(
            b"WARNING: msg overflowed for %s\n\x00" as *const u8 as *const libc::c_char,
            (*client).name.as_mut_ptr(),
        );
        crate::src::qcommon::msg::MSG_Clear(&mut msg);
    }
    SV_SendMessageToClient(&mut msg, client);
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
//
// sv_init.c
//
//
// sv_client.c
//
//
// sv_ccmds.c
//
//
// sv_snapshot.c
//
/*
=======================
SV_SendClientMessages
=======================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_SendClientMessages() {
    let mut i: libc::c_int = 0;
    let mut c: *mut crate::server_h::client_t = 0 as *mut crate::server_h::client_t;
    let mut current_block_6: u64;
    // send a message to each connected client
    i = 0 as libc::c_int; // not connected
    while i < (*crate::src::server::sv_main::sv_maxclients).integer {
        c = &mut *crate::src::server::sv_main::svs.clients.offset(i as isize)
            as *mut crate::server_h::client_t; // It's not time yet
        if !((*c).state as u64 == 0) {
            if !(((crate::src::server::sv_main::svs.time - (*c).lastSnapshotTime) as libc::c_float)
                < (*c).snapshotMsec as libc::c_float
                    * (*crate::src::qcommon::common::com_timescale).value)
            {
                if !(*(*c).downloadName.as_mut_ptr() != 0) {
                    if (*c).netchan.unsentFragments as libc::c_uint != 0
                        || !(*c).netchan_start_queue.is_null()
                    {
                        (*c).rateDelayed = crate::src::qcommon::q_shared::qtrue // Client is downloading, don't send snapshots
                                                                                // Drop this snapshot if the packet queue is still full or delta compression will break
                    } else {
                        if !((*c).netchan.remoteAddress.type_0 as libc::c_uint
                            == crate::qcommon_h::NA_LOOPBACK as libc::c_int as libc::c_uint
                            || (*crate::src::server::sv_main::sv_lanForceRate).integer != 0
                                && crate::src::qcommon::net_ip::Sys_IsLANAddress(
                                    (*c).netchan.remoteAddress,
                                ) as libc::c_uint
                                    != 0)
                        {
                            // rate control for clients not on LAN
                            if crate::src::server::sv_main::SV_RateMsec(c) > 0 as libc::c_int {
                                // Not enough time since last packet passed through the line
                                (*c).rateDelayed = crate::src::qcommon::q_shared::qtrue;
                                current_block_6 = 16559507199688588974;
                            } else {
                                current_block_6 = 12599329904712511516;
                            }
                        } else {
                            current_block_6 = 12599329904712511516;
                        }
                        match current_block_6 {
                            16559507199688588974 => {}
                            _ => {
                                // generate and send a new message
                                SV_SendClientSnapshot(c);
                                (*c).lastSnapshotTime = crate::src::server::sv_main::svs.time;
                                (*c).rateDelayed = crate::src::qcommon::q_shared::qfalse
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
}
