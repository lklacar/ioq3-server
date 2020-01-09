use ::libc;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::qcommon_h::completionFunc_t;
pub use crate::qcommon_h::xcommand_t;
pub use crate::src::qcommon::cmd::Cmd_AddCommand;
pub use crate::src::qcommon::cmd::Cmd_Argc;
pub use crate::src::qcommon::cmd::Cmd_Args;
pub use crate::src::qcommon::cmd::Cmd_ArgsFrom;
pub use crate::src::qcommon::cmd::Cmd_Argv;
pub use crate::src::qcommon::cmd::Cmd_SetCommandCompletionFunc;
pub use crate::src::qcommon::common::com_errorEntered;
pub use crate::src::qcommon::common::Com_DPrintf;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Filter;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::CopyString;
pub use crate::src::qcommon::common::Field_CompleteCommand;
pub use crate::src::qcommon::common::Z_Free;
pub use crate::src::qcommon::files::FS_Write;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::Com_SkipTokens;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Info_SetValueForKey;
pub use crate::src::qcommon::q_shared::Info_SetValueForKey_Big;
pub use crate::src::qcommon::q_shared::Q_isanumber;
pub use crate::src::qcommon::q_shared::Q_isintegral;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
use crate::stdlib::__assert_fail;
use crate::stdlib::atof;
use crate::stdlib::atoi;
use crate::stdlib::memset;
use crate::stdlib::strchr;
use crate::stdlib::strcmp;
use crate::stdlib::strlen;
use crate::stdlib::tolower;
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
// cvar.c -- dynamic variable tracking
#[no_mangle]

pub static mut cvar_vars: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cvar_cheats: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *const crate::src::qcommon::q_shared::cvar_t as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut cvar_modifiedFlags: libc::c_int = 0;
#[no_mangle]

pub static mut cvar_indexes: [crate::src::qcommon::q_shared::cvar_t; 2048] =
    [crate::src::qcommon::q_shared::cvar_t {
        name: 0 as *const libc::c_char as *mut libc::c_char,
        string: 0 as *const libc::c_char as *mut libc::c_char,
        resetString: 0 as *const libc::c_char as *mut libc::c_char,
        latchedString: 0 as *const libc::c_char as *mut libc::c_char,
        flags: 0,
        modified: crate::src::qcommon::q_shared::qfalse,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        validate: crate::src::qcommon::q_shared::qfalse,
        integral: crate::src::qcommon::q_shared::qfalse,
        min: 0.,
        max: 0.,
        description: 0 as *const libc::c_char as *mut libc::c_char,
        next: 0 as *const crate::src::qcommon::q_shared::cvar_t
            as *mut crate::src::qcommon::q_shared::cvar_t,
        prev: 0 as *const crate::src::qcommon::q_shared::cvar_t
            as *mut crate::src::qcommon::q_shared::cvar_t,
        hashNext: 0 as *const crate::src::qcommon::q_shared::cvar_t
            as *mut crate::src::qcommon::q_shared::cvar_t,
        hashPrev: 0 as *const crate::src::qcommon::q_shared::cvar_t
            as *mut crate::src::qcommon::q_shared::cvar_t,
        hashIndex: 0,
    }; 2048];
#[no_mangle]

pub static mut cvar_numIndexes: libc::c_int = 0;

static mut hashTable: [*mut crate::src::qcommon::q_shared::cvar_t; 256] =
    [0 as *const crate::src::qcommon::q_shared::cvar_t
        as *mut crate::src::qcommon::q_shared::cvar_t; 256];
/*
================
return a hash value for the filename
================
*/

unsafe extern "C" fn generateHashValue(mut fname: *const libc::c_char) -> libc::c_long {
    let mut i: libc::c_int = 0;
    let mut hash: libc::c_long = 0;
    let mut letter: libc::c_char = 0;
    hash = 0 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while *fname.offset(i as isize) as libc::c_int != '\u{0}' as i32 {
        letter = crate::stdlib::tolower(*fname.offset(i as isize) as libc::c_int) as libc::c_char;
        hash += letter as libc::c_long * (i + 119 as libc::c_int) as libc::c_long;
        i += 1
    }
    hash &= (256 as libc::c_int - 1 as libc::c_int) as libc::c_long;
    return hash;
}
/*
============
Cvar_ValidateString
============
*/

unsafe extern "C" fn Cvar_ValidateString(
    mut s: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    if s.is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if !crate::stdlib::strchr(s, '\\' as i32).is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if !crate::stdlib::strchr(s, '\"' as i32).is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if !crate::stdlib::strchr(s, ';' as i32).is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
============
Cvar_FindVar
============
*/

unsafe extern "C" fn Cvar_FindVar(
    mut var_name: *const libc::c_char,
) -> *mut crate::src::qcommon::q_shared::cvar_t {
    let mut var: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    let mut hash: libc::c_long = 0;
    hash = generateHashValue(var_name);
    var = hashTable[hash as usize];
    while !var.is_null() {
        if crate::src::qcommon::q_shared::Q_stricmp(var_name, (*var).name) == 0 {
            return var;
        }
        var = (*var).hashNext
    }
    return 0 as *mut crate::src::qcommon::q_shared::cvar_t;
}
// expands value to a string and calls Cvar_Set/Cvar_SetSafe
/*
============
Cvar_VariableValue
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_VariableValue(mut var_name: *const libc::c_char) -> libc::c_float {
    let mut var: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    var = Cvar_FindVar(var_name);
    if var.is_null() {
        return 0 as libc::c_int as libc::c_float;
    }
    return (*var).value;
}
/*
============
Cvar_VariableIntegerValue
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_VariableIntegerValue(
    mut var_name: *const libc::c_char,
) -> libc::c_int {
    let mut var: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    var = Cvar_FindVar(var_name);
    if var.is_null() {
        return 0 as libc::c_int;
    }
    return (*var).integer;
}
// returns 0 if not defined or non numeric
/*
============
Cvar_VariableString
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_VariableString(
    mut var_name: *const libc::c_char,
) -> *mut libc::c_char {
    let mut var: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    var = Cvar_FindVar(var_name);
    if var.is_null() {
        return b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    return (*var).string;
}
/*
============
Cvar_VariableStringBuffer
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_VariableStringBuffer(
    mut var_name: *const libc::c_char,
    mut buffer: *mut libc::c_char,
    mut bufsize: libc::c_int,
) {
    let mut var: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    var = Cvar_FindVar(var_name);
    if var.is_null() {
        *buffer = 0 as libc::c_int as libc::c_char
    } else {
        crate::src::qcommon::q_shared::Q_strncpyz(buffer, (*var).string, bufsize);
    };
}
// returns an empty string if not defined
/*
============
Cvar_Flags
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_Flags(mut var_name: *const libc::c_char) -> libc::c_int {
    let mut var: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    var = Cvar_FindVar(var_name);
    if var.is_null() {
        return 0x80000000 as libc::c_uint as libc::c_int;
    } else if (*var).modified as u64 != 0 {
        return (*var).flags | 0x40000000 as libc::c_int;
    } else {
        return (*var).flags;
    };
}
// returns CVAR_NONEXISTENT if cvar doesn't exist or the flags of that particular CVAR.
/*
============
Cvar_CommandCompletion
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_CommandCompletion(
    mut callback: Option<unsafe extern "C" fn(_: *const libc::c_char) -> ()>,
) {
    let mut cvar: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    cvar = cvar_vars;
    while !cvar.is_null() {
        if !(*cvar).name.is_null() {
            callback.expect("non-null function pointer")((*cvar).name);
        }
        cvar = (*cvar).next
    }
}
/*
============
Cvar_Validate
============
*/

unsafe extern "C" fn Cvar_Validate(
    mut var: *mut crate::src::qcommon::q_shared::cvar_t,
    mut value: *const libc::c_char,
    mut warn: crate::src::qcommon::q_shared::qboolean,
) -> *const libc::c_char {
    static mut s: [libc::c_char; 256] = [0; 256];
    let mut valuef: libc::c_float = 0.;
    let mut changed: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    if (*var).validate as u64 == 0 {
        return value;
    }
    if value.is_null() {
        return value;
    }
    if crate::src::qcommon::q_shared::Q_isanumber(value) as u64 != 0 {
        valuef = crate::stdlib::atof(value) as libc::c_float;
        if (*var).integral as u64 != 0 {
            if crate::src::qcommon::q_shared::Q_isintegral(valuef) as u64 == 0 {
                if warn as u64 != 0 {
                    crate::src::qcommon::common::Com_Printf(
                        b"WARNING: cvar \'%s\' must be integral\x00" as *const u8
                            as *const libc::c_char,
                        (*var).name,
                    );
                }
                valuef = valuef as libc::c_int as libc::c_float;
                changed = crate::src::qcommon::q_shared::qtrue
            }
        }
    } else {
        if warn as u64 != 0 {
            crate::src::qcommon::common::Com_Printf(
                b"WARNING: cvar \'%s\' must be numeric\x00" as *const u8 as *const libc::c_char,
                (*var).name,
            );
        }
        valuef = crate::stdlib::atof((*var).resetString) as libc::c_float;
        changed = crate::src::qcommon::q_shared::qtrue
    }
    if valuef < (*var).min {
        if warn as u64 != 0 {
            if changed as u64 != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b" and is\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                crate::src::qcommon::common::Com_Printf(
                    b"WARNING: cvar \'%s\'\x00" as *const u8 as *const libc::c_char,
                    (*var).name,
                );
            }
            if crate::src::qcommon::q_shared::Q_isintegral((*var).min) as u64 != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b" out of range (min %d)\x00" as *const u8 as *const libc::c_char,
                    (*var).min as libc::c_int,
                );
            } else {
                crate::src::qcommon::common::Com_Printf(
                    b" out of range (min %f)\x00" as *const u8 as *const libc::c_char,
                    (*var).min as libc::c_double,
                );
            }
        }
        valuef = (*var).min;
        changed = crate::src::qcommon::q_shared::qtrue
    } else if valuef > (*var).max {
        if warn as u64 != 0 {
            if changed as u64 != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b" and is\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                crate::src::qcommon::common::Com_Printf(
                    b"WARNING: cvar \'%s\'\x00" as *const u8 as *const libc::c_char,
                    (*var).name,
                );
            }
            if crate::src::qcommon::q_shared::Q_isintegral((*var).max) as u64 != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b" out of range (max %d)\x00" as *const u8 as *const libc::c_char,
                    (*var).max as libc::c_int,
                );
            } else {
                crate::src::qcommon::common::Com_Printf(
                    b" out of range (max %f)\x00" as *const u8 as *const libc::c_char,
                    (*var).max as libc::c_double,
                );
            }
        }
        valuef = (*var).max;
        changed = crate::src::qcommon::q_shared::qtrue
    }
    if changed as u64 != 0 {
        if crate::src::qcommon::q_shared::Q_isintegral(valuef) as u64 != 0 {
            crate::src::qcommon::q_shared::Com_sprintf(
                s.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"%d\x00" as *const u8 as *const libc::c_char,
                valuef as libc::c_int,
            );
            if warn as u64 != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b", setting to %d\n\x00" as *const u8 as *const libc::c_char,
                    valuef as libc::c_int,
                );
            }
        } else {
            crate::src::qcommon::q_shared::Com_sprintf(
                s.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"%f\x00" as *const u8 as *const libc::c_char,
                valuef as libc::c_double,
            );
            if warn as u64 != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b", setting to %f\n\x00" as *const u8 as *const libc::c_char,
                    valuef as libc::c_double,
                );
            }
        }
        return s.as_mut_ptr();
    } else {
        return value;
    };
}
// Parses a single line of text into arguments and tries to execute it
// as if it was typed at the console
/*
==============================================================

CVAR

==============================================================
*/
/*

cvar_t variables are used to hold scalar or string variables that can be changed
or displayed at the console or prog code as well as accessed directly
in C code.

The user can access cvars from the console in three ways:
r_draworder			prints the current value
r_draworder 0		sets the current value to 0
set r_draworder 0	as above, but creates the cvar if not present

Cvars are restricted from having the same names as commands to keep this
interface from being ambiguous.

The are also occasionally used to communicated information between different
modules of the program.

*/
/*
============
Cvar_Get

If the variable already exists, the value will not be set unless CVAR_ROM
The flags will be or'ed in if the variable exists.
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_Get(
    mut var_name: *const libc::c_char,
    mut var_value: *const libc::c_char,
    mut flags: libc::c_int,
) -> *mut crate::src::qcommon::q_shared::cvar_t {
    let mut var: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    let mut hash: libc::c_long = 0;
    let mut index: libc::c_int = 0;
    if var_name.is_null() || var_value.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"Cvar_Get: NULL parameter\x00" as *const u8 as *const libc::c_char,
        );
    }
    if Cvar_ValidateString(var_name) as u64 == 0 {
        crate::src::qcommon::common::Com_Printf(
            b"invalid cvar name string: %s\n\x00" as *const u8 as *const libc::c_char,
            var_name,
        );
        var_name = b"BADNAME\x00" as *const u8 as *const libc::c_char
    }
    // FIXME: values with backslash happen
    var = Cvar_FindVar(var_name);
    if !var.is_null() {
        var_value = Cvar_Validate(var, var_value, crate::src::qcommon::q_shared::qfalse);
        // Make sure the game code cannot mark engine-added variables as gamecode vars
        if (*var).flags & 0x1000 as libc::c_int != 0 {
            if flags & 0x1000 as libc::c_int == 0 {
                (*var).flags &= !(0x1000 as libc::c_int)
            }
        } else if (*var).flags & 0x80 as libc::c_int == 0 {
            if flags & 0x1000 as libc::c_int != 0 {
                flags &= !(0x1000 as libc::c_int)
            }
        }
        // if the C code is now specifying a variable that the user already
        // set a value for, take the new value as the reset value
        if (*var).flags & 0x80 as libc::c_int != 0 {
            (*var).flags &= !(0x80 as libc::c_int);
            crate::src::qcommon::common::Z_Free((*var).resetString as *mut libc::c_void);
            (*var).resetString = crate::src::qcommon::common::CopyString(var_value);
            if flags & 0x40 as libc::c_int != 0 {
                // this variable was set by the user,
                // so force it to value given by the engine.
                if !(*var).latchedString.is_null() {
                    crate::src::qcommon::common::Z_Free((*var).latchedString as *mut libc::c_void);
                }
                (*var).latchedString = crate::src::qcommon::common::CopyString(var_value)
            }
        }
        // Make sure servers cannot mark engine-added variables as SERVER_CREATED
        if (*var).flags & 0x800 as libc::c_int != 0 {
            if flags & 0x800 as libc::c_int == 0 {
                (*var).flags &= !(0x800 as libc::c_int)
            }
        } else if flags & 0x800 as libc::c_int != 0 {
            flags &= !(0x800 as libc::c_int)
        }
        (*var).flags |= flags;
        // only allow one non-empty reset string without a warning
        if *(*var).resetString.offset(0 as libc::c_int as isize) == 0 {
            // we don't have a reset string yet
            crate::src::qcommon::common::Z_Free((*var).resetString as *mut libc::c_void);
            (*var).resetString = crate::src::qcommon::common::CopyString(var_value)
        } else if *var_value.offset(0 as libc::c_int as isize) as libc::c_int != 0
            && crate::stdlib::strcmp((*var).resetString, var_value) != 0
        {
            crate::src::qcommon::common::Com_DPrintf(
                b"Warning: cvar \"%s\" given initial values: \"%s\" and \"%s\"\n\x00" as *const u8
                    as *const libc::c_char,
                var_name,
                (*var).resetString,
                var_value,
            );
        }
        // if we have a latched string, take that value now
        if !(*var).latchedString.is_null() {
            let mut s: *mut libc::c_char = 0 as *mut libc::c_char; // otherwise cvar_set2 would free it
            s = (*var).latchedString;
            (*var).latchedString = 0 as *mut libc::c_char;
            Cvar_Set2(var_name, s, crate::src::qcommon::q_shared::qtrue);
            crate::src::qcommon::common::Z_Free(s as *mut libc::c_void);
        }
        // ZOID--needs to be set so that cvars the game sets as
        // SERVERINFO get sent to clients
        cvar_modifiedFlags |= flags;
        return var;
    }
    //
    // allocate a new cvar
    //
    // find a free cvar
    index = 0 as libc::c_int;
    while index < 2048 as libc::c_int {
        if cvar_indexes[index as usize].name.is_null() {
            break;
        }
        index += 1
    }
    if index >= 2048 as libc::c_int {
        if crate::src::qcommon::common::com_errorEntered as u64 == 0 {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
                b"Error: Too many cvars, cannot create a new one!\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0 as *mut crate::src::qcommon::q_shared::cvar_t;
    }
    var = &mut *cvar_indexes.as_mut_ptr().offset(index as isize)
        as *mut crate::src::qcommon::q_shared::cvar_t;
    if index >= cvar_numIndexes {
        cvar_numIndexes = index + 1 as libc::c_int
    }
    (*var).name = crate::src::qcommon::common::CopyString(var_name);
    (*var).string = crate::src::qcommon::common::CopyString(var_value);
    (*var).modified = crate::src::qcommon::q_shared::qtrue;
    (*var).modificationCount = 1 as libc::c_int;
    (*var).value = crate::stdlib::atof((*var).string) as libc::c_float;
    (*var).integer = crate::stdlib::atoi((*var).string);
    (*var).resetString = crate::src::qcommon::common::CopyString(var_value);
    (*var).validate = crate::src::qcommon::q_shared::qfalse;
    (*var).description = 0 as *mut libc::c_char;
    // link the variable in
    (*var).next = cvar_vars;
    if !cvar_vars.is_null() {
        (*cvar_vars).prev = var
    }
    (*var).prev = 0 as *mut crate::src::qcommon::q_shared::cvar_t;
    cvar_vars = var;
    (*var).flags = flags;
    // note what types of cvars have been modified (userinfo, archive, serverinfo, systeminfo)
    cvar_modifiedFlags |= (*var).flags;
    hash = generateHashValue(var_name);
    (*var).hashIndex = hash as libc::c_int;
    (*var).hashNext = hashTable[hash as usize];
    if !hashTable[hash as usize].is_null() {
        (*hashTable[hash as usize]).hashPrev = var
    }
    (*var).hashPrev = 0 as *mut crate::src::qcommon::q_shared::cvar_t;
    hashTable[hash as usize] = var;
    return var;
}
/*
============
Cvar_Print

Prints the value, default, and latched string of the given variable
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_Print(mut v: *mut crate::src::qcommon::q_shared::cvar_t) {
    crate::src::qcommon::common::Com_Printf(
        b"\"%s\" is:\"%s^7\"\x00" as *const u8 as *const libc::c_char,
        (*v).name,
        (*v).string,
    );
    if (*v).flags & 0x40 as libc::c_int == 0 {
        if crate::src::qcommon::q_shared::Q_stricmp((*v).string, (*v).resetString) == 0 {
            crate::src::qcommon::common::Com_Printf(
                b", the default\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            crate::src::qcommon::common::Com_Printf(
                b" default:\"%s^7\"\x00" as *const u8 as *const libc::c_char,
                (*v).resetString,
            );
        }
    }
    crate::src::qcommon::common::Com_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
    if !(*v).latchedString.is_null() {
        crate::src::qcommon::common::Com_Printf(
            b"latched: \"%s\"\n\x00" as *const u8 as *const libc::c_char,
            (*v).latchedString,
        );
    }
    if !(*v).description.is_null() {
        crate::src::qcommon::common::Com_Printf(
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            (*v).description,
        );
    };
}
// will create the variable with no flags if it doesn't exist
/*
============
Cvar_Set2
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_Set2(
    mut var_name: *const libc::c_char,
    mut value: *const libc::c_char,
    mut force: crate::src::qcommon::q_shared::qboolean,
) -> *mut crate::src::qcommon::q_shared::cvar_t {
    let mut var: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    //	Com_DPrintf( "Cvar_Set2: %s %s\n", var_name, value );
    if Cvar_ValidateString(var_name) as u64 == 0 {
        crate::src::qcommon::common::Com_Printf(
            b"invalid cvar name string: %s\n\x00" as *const u8 as *const libc::c_char,
            var_name,
        );
        var_name = b"BADNAME\x00" as *const u8 as *const libc::c_char
    }
    // FIXME
    var = Cvar_FindVar(var_name);
    if var.is_null() {
        if value.is_null() {
            return 0 as *mut crate::src::qcommon::q_shared::cvar_t;
        }
        // create it
        if force as u64 == 0 {
            return Cvar_Get(var_name, value, 0x80 as libc::c_int);
        } else {
            return Cvar_Get(var_name, value, 0 as libc::c_int);
        }
    }
    if value.is_null() {
        value = (*var).resetString
    }
    value = Cvar_Validate(var, value, crate::src::qcommon::q_shared::qtrue);
    if (*var).flags & 0x20 as libc::c_int != 0 && !(*var).latchedString.is_null() {
        if crate::stdlib::strcmp(value, (*var).string) == 0 {
            crate::src::qcommon::common::Z_Free((*var).latchedString as *mut libc::c_void);
            (*var).latchedString = 0 as *mut libc::c_char;
            return var;
        }
        if crate::stdlib::strcmp(value, (*var).latchedString) == 0 {
            return var;
        }
    } else if crate::stdlib::strcmp(value, (*var).string) == 0 {
        return var;
    }
    // note what types of cvars have been modified (userinfo, archive, serverinfo, systeminfo)
    cvar_modifiedFlags |= (*var).flags; // not changed
    if force as u64 == 0 {
        if (*var).flags & 0x40 as libc::c_int != 0 {
            crate::src::qcommon::common::Com_Printf(
                b"%s is read only.\n\x00" as *const u8 as *const libc::c_char,
                var_name,
            ); // free the old value string
            return var;
        }
        if (*var).flags & 0x10 as libc::c_int != 0 {
            crate::src::qcommon::common::Com_Printf(
                b"%s is write protected.\n\x00" as *const u8 as *const libc::c_char,
                var_name,
            );
            return var;
        }
        if (*var).flags & 0x200 as libc::c_int != 0 && (*cvar_cheats).integer == 0 {
            crate::src::qcommon::common::Com_Printf(
                b"%s is cheat protected.\n\x00" as *const u8 as *const libc::c_char,
                var_name,
            );
            return var;
        }
        if (*var).flags & 0x20 as libc::c_int != 0 {
            if !(*var).latchedString.is_null() {
                if crate::stdlib::strcmp(value, (*var).latchedString) == 0 as libc::c_int {
                    return var;
                }
                crate::src::qcommon::common::Z_Free((*var).latchedString as *mut libc::c_void);
            } else if crate::stdlib::strcmp(value, (*var).string) == 0 as libc::c_int {
                return var;
            }
            crate::src::qcommon::common::Com_Printf(
                b"%s will be changed upon restarting.\n\x00" as *const u8 as *const libc::c_char,
                var_name,
            );
            (*var).latchedString = crate::src::qcommon::common::CopyString(value);
            (*var).modified = crate::src::qcommon::q_shared::qtrue;
            (*var).modificationCount += 1;
            return var;
        }
    } else if !(*var).latchedString.is_null() {
        crate::src::qcommon::common::Z_Free((*var).latchedString as *mut libc::c_void);
        (*var).latchedString = 0 as *mut libc::c_char
    }
    if crate::stdlib::strcmp(value, (*var).string) == 0 {
        return var;
    }
    (*var).modified = crate::src::qcommon::q_shared::qtrue;
    (*var).modificationCount += 1;
    crate::src::qcommon::common::Z_Free((*var).string as *mut libc::c_void);
    (*var).string = crate::src::qcommon::common::CopyString(value);
    (*var).value = crate::stdlib::atof((*var).string) as libc::c_float;
    (*var).integer = crate::stdlib::atoi((*var).string);
    return var;
}
// updates an interpreted modules' version of a cvar
/*
============
Cvar_Set
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_Set(
    mut var_name: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    Cvar_Set2(var_name, value, crate::src::qcommon::q_shared::qtrue);
}
// same as Cvar_Set, but allows more control over setting of cvar
/*
============
Cvar_SetSafe
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_SetSafe(
    mut var_name: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    let mut flags: libc::c_int = Cvar_Flags(var_name);
    if flags as libc::c_uint != 0x80000000 as libc::c_uint && flags & 0x2000 as libc::c_int != 0 {
        if !value.is_null() {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                b"Restricted source tried to set \"%s\" to \"%s\"\x00" as *const u8
                    as *const libc::c_char,
                var_name,
                value,
            );
        } else {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                b"Restricted source tried to modify \"%s\"\x00" as *const u8 as *const libc::c_char,
                var_name,
            );
        }
    }
    Cvar_Set(var_name, value);
}
// sometimes we set variables from an untrusted source: fail if flags & CVAR_PROTECTED
/*
============
Cvar_SetLatched
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_SetLatched(
    mut var_name: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    Cvar_Set2(var_name, value, crate::src::qcommon::q_shared::qfalse);
}
// don't set the cvar immediately
/*
============
Cvar_SetValue
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_SetValue(
    mut var_name: *const libc::c_char,
    mut value: libc::c_float,
) {
    let mut val: [libc::c_char; 32] = [0; 32];
    if value == value as libc::c_int as libc::c_float {
        crate::src::qcommon::q_shared::Com_sprintf(
            val.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
            b"%i\x00" as *const u8 as *const libc::c_char,
            value as libc::c_int,
        );
    } else {
        crate::src::qcommon::q_shared::Com_sprintf(
            val.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
            b"%f\x00" as *const u8 as *const libc::c_char,
            value as libc::c_double,
        );
    }
    Cvar_Set(var_name, val.as_mut_ptr());
}
/*
============
Cvar_SetValueSafe
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_SetValueSafe(
    mut var_name: *const libc::c_char,
    mut value: libc::c_float,
) {
    let mut val: [libc::c_char; 32] = [0; 32];
    if crate::src::qcommon::q_shared::Q_isintegral(value) as u64 != 0 {
        crate::src::qcommon::q_shared::Com_sprintf(
            val.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
            b"%i\x00" as *const u8 as *const libc::c_char,
            value as libc::c_int,
        );
    } else {
        crate::src::qcommon::q_shared::Com_sprintf(
            val.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
            b"%f\x00" as *const u8 as *const libc::c_char,
            value as libc::c_double,
        );
    }
    Cvar_SetSafe(var_name, val.as_mut_ptr());
}
// callback with each valid string
/*
============
Cvar_Reset
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_Reset(mut var_name: *const libc::c_char) {
    Cvar_Set2(
        var_name,
        0 as *const libc::c_char,
        crate::src::qcommon::q_shared::qfalse,
    );
}
/*
============
Cvar_ForceReset
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_ForceReset(mut var_name: *const libc::c_char) {
    Cvar_Set2(
        var_name,
        0 as *const libc::c_char,
        crate::src::qcommon::q_shared::qtrue,
    );
}
/*
============
Cvar_SetCheatState

Any testing variables will be reset to the safe values
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_SetCheatState() {
    let mut var: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    // set all default vars to the safe value
    var = cvar_vars;
    while !var.is_null() {
        if (*var).flags & 0x200 as libc::c_int != 0 {
            // the CVAR_LATCHED|CVAR_CHEAT vars might escape the reset here
            // because of a different var->latchedString
            if !(*var).latchedString.is_null() {
                crate::src::qcommon::common::Z_Free((*var).latchedString as *mut libc::c_void);
                (*var).latchedString = 0 as *mut libc::c_char
            }
            if crate::stdlib::strcmp((*var).resetString, (*var).string) != 0 {
                Cvar_Set((*var).name, (*var).resetString);
            }
        }
        var = (*var).next
    }
}
// reset all testing vars to a safe value
/*
============
Cvar_Command

Handles variable inspection and changing from the console
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_Command() -> crate::src::qcommon::q_shared::qboolean {
    let mut v: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    // check variables
    v = Cvar_FindVar(crate::src::qcommon::cmd::Cmd_Argv(0 as libc::c_int));
    if v.is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    // perform a variable print or set
    if crate::src::qcommon::cmd::Cmd_Argc() == 1 as libc::c_int {
        Cvar_Print(v);
        return crate::src::qcommon::q_shared::qtrue;
    }
    // set the value if forcing isn't required
    Cvar_Set2(
        (*v).name,
        crate::src::qcommon::cmd::Cmd_Args(),
        crate::src::qcommon::q_shared::qfalse,
    );
    return crate::src::qcommon::q_shared::qtrue;
}
/*
============
Cvar_Print_f

Prints the contents of a cvar
(preferred over Cvar_Command where cvar names and commands conflict)
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_Print_f() {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cv: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    if crate::src::qcommon::cmd::Cmd_Argc() != 2 as libc::c_int {
        crate::src::qcommon::common::Com_Printf(
            b"usage: print <variable>\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    name = crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int);
    cv = Cvar_FindVar(name);
    if !cv.is_null() {
        Cvar_Print(cv);
    } else {
        crate::src::qcommon::common::Com_Printf(
            b"Cvar %s does not exist.\n\x00" as *const u8 as *const libc::c_char,
            name,
        );
    };
}
/*
============
Cvar_Toggle_f

Toggles a cvar for easy single key binding, optionally through a list of
given values
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_Toggle_f() {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = crate::src::qcommon::cmd::Cmd_Argc();
    let mut curval: *mut libc::c_char = 0 as *mut libc::c_char;
    if c < 2 as libc::c_int {
        crate::src::qcommon::common::Com_Printf(
            b"usage: toggle <variable> [value1, value2, ...]\n\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if c == 2 as libc::c_int {
        Cvar_Set2(
            crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int),
            crate::src::qcommon::q_shared::va(
                b"%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (Cvar_VariableValue(crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int)) == 0.)
                    as libc::c_int,
            ),
            crate::src::qcommon::q_shared::qfalse,
        );
        return;
    }
    if c == 3 as libc::c_int {
        crate::src::qcommon::common::Com_Printf(
            b"toggle: nothing to toggle to\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    curval = Cvar_VariableString(crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int));
    // don't bother checking the last arg for a match since the desired
    // behaviour is the same as no match (set to the first argument)
    i = 2 as libc::c_int;
    while (i + 1 as libc::c_int) < c {
        if crate::stdlib::strcmp(curval, crate::src::qcommon::cmd::Cmd_Argv(i)) == 0 as libc::c_int
        {
            Cvar_Set2(
                crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int),
                crate::src::qcommon::cmd::Cmd_Argv(i + 1 as libc::c_int),
                crate::src::qcommon::q_shared::qfalse,
            );
            return;
        }
        i += 1
    }
    // fallback
    Cvar_Set2(
        crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int),
        crate::src::qcommon::cmd::Cmd_Argv(2 as libc::c_int),
        crate::src::qcommon::q_shared::qfalse,
    );
}
/*
============
Cvar_Set_f

Allows setting and defining of arbitrary cvars from console, even if they
weren't declared in C code.
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_Set_f() {
    let mut c: libc::c_int = 0;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    c = crate::src::qcommon::cmd::Cmd_Argc();
    cmd = crate::src::qcommon::cmd::Cmd_Argv(0 as libc::c_int);
    if c < 2 as libc::c_int {
        crate::src::qcommon::common::Com_Printf(
            b"usage: %s <variable> <value>\n\x00" as *const u8 as *const libc::c_char,
            cmd,
        );
        return;
    }
    if c == 2 as libc::c_int {
        Cvar_Print_f();
        return;
    }
    v = Cvar_Set2(
        crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int),
        crate::src::qcommon::cmd::Cmd_ArgsFrom(2 as libc::c_int),
        crate::src::qcommon::q_shared::qfalse,
    );
    if v.is_null() {
        return;
    }
    match *cmd.offset(3 as libc::c_int as isize) as libc::c_int {
        97 => {
            if (*v).flags & 0x1 as libc::c_int == 0 {
                (*v).flags |= 0x1 as libc::c_int;
                cvar_modifiedFlags |= 0x1 as libc::c_int
            }
        }
        117 => {
            if (*v).flags & 0x2 as libc::c_int == 0 {
                (*v).flags |= 0x2 as libc::c_int;
                cvar_modifiedFlags |= 0x2 as libc::c_int
            }
        }
        115 => {
            if (*v).flags & 0x4 as libc::c_int == 0 {
                (*v).flags |= 0x4 as libc::c_int;
                cvar_modifiedFlags |= 0x4 as libc::c_int
            }
        }
        _ => {}
    };
}
/*
============
Cvar_Reset_f
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_Reset_f() {
    if crate::src::qcommon::cmd::Cmd_Argc() != 2 as libc::c_int {
        crate::src::qcommon::common::Com_Printf(
            b"usage: reset <variable>\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    Cvar_Reset(crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int));
}
// called by Cmd_ExecuteString when Cmd_Argv(0) doesn't match a known
// command.  Returns true if the command was a variable reference that
// was handled. (print or change)
/*
============
Cvar_WriteVariables

Appends lines containing "set variable value" for all variables
with the archive flag set to qtrue.
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_WriteVariables(mut f: crate::src::qcommon::q_shared::fileHandle_t) {
    let mut var: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut current_block_5: u64;
    var = cvar_vars;
    while !var.is_null() {
        if !((*var).name.is_null()
            || crate::src::qcommon::q_shared::Q_stricmp(
                (*var).name,
                b"cl_cdkey\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int)
        {
            if (*var).flags & 0x1 as libc::c_int != 0 {
                // write the latched value, even if it hasn't taken effect yet
                if !(*var).latchedString.is_null() {
                    if crate::stdlib::strlen((*var).name)
                        .wrapping_add(crate::stdlib::strlen((*var).latchedString))
                        .wrapping_add(10 as libc::c_int as libc::c_ulong)
                        > ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                    {
                        crate::src::qcommon::common::Com_Printf(
                            b"^3WARNING: value of variable \"%s\" too long to write to file\n\x00"
                                as *const u8 as *const libc::c_char,
                            (*var).name,
                        );
                        current_block_5 = 16559507199688588974;
                    } else {
                        crate::src::qcommon::q_shared::Com_sprintf(
                            buffer.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                                as libc::c_int,
                            b"seta %s \"%s\"\n\x00" as *const u8 as *const libc::c_char,
                            (*var).name,
                            (*var).latchedString,
                        );
                        current_block_5 = 12599329904712511516;
                    }
                } else if crate::stdlib::strlen((*var).name)
                    .wrapping_add(crate::stdlib::strlen((*var).string))
                    .wrapping_add(10 as libc::c_int as libc::c_ulong)
                    > ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                {
                    crate::src::qcommon::common::Com_Printf(
                        b"^3WARNING: value of variable \"%s\" too long to write to file\n\x00"
                            as *const u8 as *const libc::c_char,
                        (*var).name,
                    );
                    current_block_5 = 16559507199688588974;
                } else {
                    crate::src::qcommon::q_shared::Com_sprintf(
                        buffer.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                            as libc::c_int,
                        b"seta %s \"%s\"\n\x00" as *const u8 as *const libc::c_char,
                        (*var).name,
                        (*var).string,
                    );
                    current_block_5 = 12599329904712511516;
                }
                match current_block_5 {
                    16559507199688588974 => {}
                    _ => {
                        crate::src::qcommon::files::FS_Write(
                            buffer.as_mut_ptr() as *const libc::c_void,
                            crate::stdlib::strlen(buffer.as_mut_ptr()) as libc::c_int,
                            f,
                        );
                    }
                }
            }
        }
        var = (*var).next
    }
}
/*
============
Cvar_List_f
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_List_f() {
    let mut var: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    let mut i: libc::c_int = 0;
    let mut match_0: *mut libc::c_char = 0 as *mut libc::c_char;
    if crate::src::qcommon::cmd::Cmd_Argc() > 1 as libc::c_int {
        match_0 = crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int)
    } else {
        match_0 = 0 as *mut libc::c_char
    }
    i = 0 as libc::c_int;
    var = cvar_vars;
    while !var.is_null() {
        if !((*var).name.is_null()
            || !match_0.is_null()
                && crate::src::qcommon::common::Com_Filter(
                    match_0,
                    (*var).name,
                    crate::src::qcommon::q_shared::qfalse as libc::c_int,
                ) == 0)
        {
            if (*var).flags & 0x4 as libc::c_int != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"S\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                crate::src::qcommon::common::Com_Printf(
                    b" \x00" as *const u8 as *const libc::c_char,
                );
            }
            if (*var).flags & 0x8 as libc::c_int != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"s\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                crate::src::qcommon::common::Com_Printf(
                    b" \x00" as *const u8 as *const libc::c_char,
                );
            }
            if (*var).flags & 0x2 as libc::c_int != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"U\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                crate::src::qcommon::common::Com_Printf(
                    b" \x00" as *const u8 as *const libc::c_char,
                );
            }
            if (*var).flags & 0x40 as libc::c_int != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"R\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                crate::src::qcommon::common::Com_Printf(
                    b" \x00" as *const u8 as *const libc::c_char,
                );
            }
            if (*var).flags & 0x10 as libc::c_int != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"I\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                crate::src::qcommon::common::Com_Printf(
                    b" \x00" as *const u8 as *const libc::c_char,
                );
            }
            if (*var).flags & 0x1 as libc::c_int != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"A\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                crate::src::qcommon::common::Com_Printf(
                    b" \x00" as *const u8 as *const libc::c_char,
                );
            }
            if (*var).flags & 0x20 as libc::c_int != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"L\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                crate::src::qcommon::common::Com_Printf(
                    b" \x00" as *const u8 as *const libc::c_char,
                );
            }
            if (*var).flags & 0x200 as libc::c_int != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"C\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                crate::src::qcommon::common::Com_Printf(
                    b" \x00" as *const u8 as *const libc::c_char,
                );
            }
            if (*var).flags & 0x80 as libc::c_int != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"?\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                crate::src::qcommon::common::Com_Printf(
                    b" \x00" as *const u8 as *const libc::c_char,
                );
            }
            crate::src::qcommon::common::Com_Printf(
                b" %s \"%s\"\n\x00" as *const u8 as *const libc::c_char,
                (*var).name,
                (*var).string,
            );
        }
        var = (*var).next;
        i += 1
    }
    crate::src::qcommon::common::Com_Printf(
        b"\n%i total cvars\n\x00" as *const u8 as *const libc::c_char,
        i,
    );
    crate::src::qcommon::common::Com_Printf(
        b"%i cvar indexes\n\x00" as *const u8 as *const libc::c_char,
        cvar_numIndexes,
    );
}
/*
============
Cvar_ListModified_f
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_ListModified_f() {
    let mut var: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    let mut totalModified: libc::c_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut match_0: *mut libc::c_char = 0 as *mut libc::c_char;
    if crate::src::qcommon::cmd::Cmd_Argc() > 1 as libc::c_int {
        match_0 = crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int)
    } else {
        match_0 = 0 as *mut libc::c_char
    }
    totalModified = 0 as libc::c_int;
    var = cvar_vars;
    while !var.is_null() {
        if !((*var).name.is_null() || (*var).modificationCount == 0) {
            value = if !(*var).latchedString.is_null() {
                (*var).latchedString
            } else {
                (*var).string
            };
            if !(crate::stdlib::strcmp(value, (*var).resetString) == 0) {
                totalModified += 1;
                if !(!match_0.is_null()
                    && crate::src::qcommon::common::Com_Filter(
                        match_0,
                        (*var).name,
                        crate::src::qcommon::q_shared::qfalse as libc::c_int,
                    ) == 0)
                {
                    if (*var).flags & 0x4 as libc::c_int != 0 {
                        crate::src::qcommon::common::Com_Printf(
                            b"S\x00" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        crate::src::qcommon::common::Com_Printf(
                            b" \x00" as *const u8 as *const libc::c_char,
                        );
                    }
                    if (*var).flags & 0x8 as libc::c_int != 0 {
                        crate::src::qcommon::common::Com_Printf(
                            b"s\x00" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        crate::src::qcommon::common::Com_Printf(
                            b" \x00" as *const u8 as *const libc::c_char,
                        );
                    }
                    if (*var).flags & 0x2 as libc::c_int != 0 {
                        crate::src::qcommon::common::Com_Printf(
                            b"U\x00" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        crate::src::qcommon::common::Com_Printf(
                            b" \x00" as *const u8 as *const libc::c_char,
                        );
                    }
                    if (*var).flags & 0x40 as libc::c_int != 0 {
                        crate::src::qcommon::common::Com_Printf(
                            b"R\x00" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        crate::src::qcommon::common::Com_Printf(
                            b" \x00" as *const u8 as *const libc::c_char,
                        );
                    }
                    if (*var).flags & 0x10 as libc::c_int != 0 {
                        crate::src::qcommon::common::Com_Printf(
                            b"I\x00" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        crate::src::qcommon::common::Com_Printf(
                            b" \x00" as *const u8 as *const libc::c_char,
                        );
                    }
                    if (*var).flags & 0x1 as libc::c_int != 0 {
                        crate::src::qcommon::common::Com_Printf(
                            b"A\x00" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        crate::src::qcommon::common::Com_Printf(
                            b" \x00" as *const u8 as *const libc::c_char,
                        );
                    }
                    if (*var).flags & 0x20 as libc::c_int != 0 {
                        crate::src::qcommon::common::Com_Printf(
                            b"L\x00" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        crate::src::qcommon::common::Com_Printf(
                            b" \x00" as *const u8 as *const libc::c_char,
                        );
                    }
                    if (*var).flags & 0x200 as libc::c_int != 0 {
                        crate::src::qcommon::common::Com_Printf(
                            b"C\x00" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        crate::src::qcommon::common::Com_Printf(
                            b" \x00" as *const u8 as *const libc::c_char,
                        );
                    }
                    if (*var).flags & 0x80 as libc::c_int != 0 {
                        crate::src::qcommon::common::Com_Printf(
                            b"?\x00" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        crate::src::qcommon::common::Com_Printf(
                            b" \x00" as *const u8 as *const libc::c_char,
                        );
                    }
                    crate::src::qcommon::common::Com_Printf(
                        b" %s \"%s\", default \"%s\"\n\x00" as *const u8 as *const libc::c_char,
                        (*var).name,
                        value,
                        (*var).resetString,
                    );
                }
            }
        }
        var = (*var).next
    }
    crate::src::qcommon::common::Com_Printf(
        b"\n%i total modified cvars\n\x00" as *const u8 as *const libc::c_char,
        totalModified,
    );
}
/*
============
Cvar_Unset

Unsets a cvar
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_Unset(
    mut cv: *mut crate::src::qcommon::q_shared::cvar_t,
) -> *mut crate::src::qcommon::q_shared::cvar_t {
    let mut next: *mut crate::src::qcommon::q_shared::cvar_t = (*cv).next;
    // note what types of cvars have been modified (userinfo, archive, serverinfo, systeminfo)
    cvar_modifiedFlags |= (*cv).flags;
    if !(*cv).name.is_null() {
        crate::src::qcommon::common::Z_Free((*cv).name as *mut libc::c_void);
    }
    if !(*cv).string.is_null() {
        crate::src::qcommon::common::Z_Free((*cv).string as *mut libc::c_void);
    }
    if !(*cv).latchedString.is_null() {
        crate::src::qcommon::common::Z_Free((*cv).latchedString as *mut libc::c_void);
    }
    if !(*cv).resetString.is_null() {
        crate::src::qcommon::common::Z_Free((*cv).resetString as *mut libc::c_void);
    }
    if !(*cv).description.is_null() {
        crate::src::qcommon::common::Z_Free((*cv).description as *mut libc::c_void);
    }
    if !(*cv).prev.is_null() {
        (*(*cv).prev).next = (*cv).next
    } else {
        cvar_vars = (*cv).next
    }
    if !(*cv).next.is_null() {
        (*(*cv).next).prev = (*cv).prev
    }
    if !(*cv).hashPrev.is_null() {
        (*(*cv).hashPrev).hashNext = (*cv).hashNext
    } else {
        hashTable[(*cv).hashIndex as usize] = (*cv).hashNext
    }
    if !(*cv).hashNext.is_null() {
        (*(*cv).hashNext).hashPrev = (*cv).hashPrev
    }
    crate::stdlib::memset(
        cv as *mut libc::c_void,
        '\u{0}' as i32,
        ::std::mem::size_of::<crate::src::qcommon::q_shared::cvar_t>() as libc::c_ulong,
    );
    return next;
}
/*
============
Cvar_Unset_f

Unsets a userdefined cvar
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_Unset_f() {
    let mut cv: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    if crate::src::qcommon::cmd::Cmd_Argc() != 2 as libc::c_int {
        crate::src::qcommon::common::Com_Printf(
            b"Usage: %s <varname>\n\x00" as *const u8 as *const libc::c_char,
            crate::src::qcommon::cmd::Cmd_Argv(0 as libc::c_int),
        );
        return;
    }
    cv = Cvar_FindVar(crate::src::qcommon::cmd::Cmd_Argv(1 as libc::c_int));
    if cv.is_null() {
        return;
    }
    if (*cv).flags & 0x80 as libc::c_int != 0 {
        Cvar_Unset(cv);
    } else {
        crate::src::qcommon::common::Com_Printf(
            b"Error: %s: Variable %s is not user created.\n\x00" as *const u8
                as *const libc::c_char,
            crate::src::qcommon::cmd::Cmd_Argv(0 as libc::c_int),
            (*cv).name,
        );
    };
}
/*
============
Cvar_Restart

Resets all cvars to their hardcoded values and removes userdefined variables
and variables added via the VMs if requested.
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_Restart(mut unsetVM: crate::src::qcommon::q_shared::qboolean) {
    let mut curvar: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    curvar = cvar_vars;
    while !curvar.is_null() {
        if (*curvar).flags & 0x80 as libc::c_int != 0
            || unsetVM as libc::c_uint != 0 && (*curvar).flags & 0x1000 as libc::c_int != 0
        {
            // throw out any variables the user/vm created
            curvar = Cvar_Unset(curvar)
        } else {
            if (*curvar).flags & (0x40 as libc::c_int | 0x10 as libc::c_int | 0x400 as libc::c_int)
                == 0
            {
                // Just reset the rest to their default values.
                Cvar_Set2(
                    (*curvar).name,
                    (*curvar).resetString,
                    crate::src::qcommon::q_shared::qfalse,
                );
            }
            curvar = (*curvar).next
        }
    }
}
/*
============
Cvar_Restart_f

Resets all cvars to their hardcoded values
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_Restart_f() {
    Cvar_Restart(crate::src::qcommon::q_shared::qfalse);
}
/*
=====================
Cvar_InfoString
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_InfoString(mut bit: libc::c_int) -> *mut libc::c_char {
    static mut info: [libc::c_char; 1024] = [0; 1024];
    let mut var: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    info[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    var = cvar_vars;
    while !var.is_null() {
        if !(*var).name.is_null() && (*var).flags & bit != 0 {
            crate::src::qcommon::q_shared::Info_SetValueForKey(
                info.as_mut_ptr(),
                (*var).name,
                (*var).string,
            );
        }
        var = (*var).next
    }
    return info.as_mut_ptr();
}
/*
=====================
Cvar_InfoString_Big

  handles large info strings ( CS_SYSTEMINFO )
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_InfoString_Big(mut bit: libc::c_int) -> *mut libc::c_char {
    static mut info: [libc::c_char; 8192] = [0; 8192];
    let mut var: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    info[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    var = cvar_vars;
    while !var.is_null() {
        if !(*var).name.is_null() && (*var).flags & bit != 0 {
            crate::src::qcommon::q_shared::Info_SetValueForKey_Big(
                info.as_mut_ptr(),
                (*var).name,
                (*var).string,
            );
        }
        var = (*var).next
    }
    return info.as_mut_ptr();
}
// returns an info string containing all the cvars that have the given bit set
// in their flags ( CVAR_USERINFO, CVAR_SERVERINFO, CVAR_SYSTEMINFO, etc )
/*
=====================
Cvar_InfoStringBuffer
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_InfoStringBuffer(
    mut bit: libc::c_int,
    mut buff: *mut libc::c_char,
    mut buffsize: libc::c_int,
) {
    crate::src::qcommon::q_shared::Q_strncpyz(buff, Cvar_InfoString(bit), buffsize);
}
/*
=====================
Cvar_CheckRange
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_CheckRange(
    mut var: *mut crate::src::qcommon::q_shared::cvar_t,
    mut min: libc::c_float,
    mut max: libc::c_float,
    mut integral: crate::src::qcommon::q_shared::qboolean,
) {
    (*var).validate = crate::src::qcommon::q_shared::qtrue;
    (*var).min = min;
    (*var).max = max;
    (*var).integral = integral;
    // Force an initial range check
    Cvar_Set((*var).name, (*var).string);
}
/*
=====================
Cvar_SetDescription
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_SetDescription(
    mut var: *mut crate::src::qcommon::q_shared::cvar_t,
    mut var_description: *const libc::c_char,
) {
    if !var_description.is_null()
        && *var_description.offset(0 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
    {
        if !(*var).description.is_null() {
            crate::src::qcommon::common::Z_Free((*var).description as *mut libc::c_void);
        }
        (*var).description = crate::src::qcommon::common::CopyString(var_description)
    };
}
// creates the variable if it doesn't exist, or returns the existing one
// if it exists, the value will not be changed, but flags will be ORed in
// that allows variables to be unarchived without needing bitflags
// if value is "", the value will not override a previously set value.
/*
=====================
Cvar_Register

basically a slightly modified Cvar_Get for the interpreted modules
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_Register(
    mut vmCvar: *mut crate::src::qcommon::q_shared::vmCvar_t,
    mut varName: *const libc::c_char,
    mut defaultValue: *const libc::c_char,
    mut flags: libc::c_int,
) {
    let mut cv: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    // There is code in Cvar_Get to prevent CVAR_ROM cvars being changed by the
    // user. In other words CVAR_ARCHIVE and CVAR_ROM are mutually exclusive
    // flags. Unfortunately some historical game code (including single player
    // baseq3) sets both flags. We unset CVAR_ROM for such cvars.
    if flags & (0x1 as libc::c_int | 0x40 as libc::c_int)
        == 0x1 as libc::c_int | 0x40 as libc::c_int
    {
        crate::src::qcommon::common::Com_DPrintf(
            b"^3WARNING: Unsetting CVAR_ROM from cvar \'%s\', since it is also CVAR_ARCHIVE\n\x00"
                as *const u8 as *const libc::c_char,
            varName,
        );
        flags &= !(0x40 as libc::c_int)
    }
    // Don't allow VM to specific a different creator or other internal flags.
    if flags & 0x80 as libc::c_int != 0 {
        crate::src::qcommon::common::Com_DPrintf(
            b"^3WARNING: VM tried to set CVAR_USER_CREATED on cvar \'%s\'\n\x00" as *const u8
                as *const libc::c_char,
            varName,
        );
        flags &= !(0x80 as libc::c_int)
    }
    if flags & 0x800 as libc::c_int != 0 {
        crate::src::qcommon::common::Com_DPrintf(
            b"^3WARNING: VM tried to set CVAR_SERVER_CREATED on cvar \'%s\'\n\x00" as *const u8
                as *const libc::c_char,
            varName,
        );
        flags &= !(0x800 as libc::c_int)
    }
    if flags & 0x2000 as libc::c_int != 0 {
        crate::src::qcommon::common::Com_DPrintf(
            b"^3WARNING: VM tried to set CVAR_PROTECTED on cvar \'%s\'\n\x00" as *const u8
                as *const libc::c_char,
            varName,
        );
        flags &= !(0x2000 as libc::c_int)
    }
    if flags & 0x40000000 as libc::c_int != 0 {
        crate::src::qcommon::common::Com_DPrintf(
            b"^3WARNING: VM tried to set CVAR_MODIFIED on cvar \'%s\'\n\x00" as *const u8
                as *const libc::c_char,
            varName,
        );
        flags &= !(0x40000000 as libc::c_int)
    }
    if flags as libc::c_uint & 0x80000000 as libc::c_uint != 0 {
        crate::src::qcommon::common::Com_DPrintf(
            b"^3WARNING: VM tried to set CVAR_NONEXISTENT on cvar \'%s\'\n\x00" as *const u8
                as *const libc::c_char,
            varName,
        );
        flags = (flags as libc::c_uint & !(0x80000000 as libc::c_uint)) as libc::c_int
    }
    cv = Cvar_FindVar(varName);
    // Don't modify cvar if it's protected.
    if !cv.is_null() && (*cv).flags & 0x2000 as libc::c_int != 0 {
        crate::src::qcommon::common::Com_DPrintf(
            b"^3WARNING: VM tried to register protected cvar \'%s\' with value \'%s\'%s\n\x00"
                as *const u8 as *const libc::c_char,
            varName,
            defaultValue,
            if flags & !(*cv).flags != 0 as libc::c_int {
                b" and new flags\x00" as *const u8 as *const libc::c_char
            } else {
                b"\x00" as *const u8 as *const libc::c_char
            },
        );
    } else {
        cv = Cvar_Get(varName, defaultValue, flags | 0x1000 as libc::c_int)
    }
    if vmCvar.is_null() {
        return;
    }
    (*vmCvar).handle = cv.wrapping_offset_from(cvar_indexes.as_mut_ptr()) as libc::c_long
        as crate::src::qcommon::q_shared::cvarHandle_t;
    (*vmCvar).modificationCount = -(1 as libc::c_int);
    Cvar_Update(vmCvar);
}
// basically a slightly modified Cvar_Get for the interpreted modules
/*
=====================
Cvar_Update

updates an interpreted modules' version of a cvar
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_Update(mut vmCvar: *mut crate::src::qcommon::q_shared::vmCvar_t) {
    let mut cv: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    if !vmCvar.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"vmCvar\x00" as *const u8 as *const libc::c_char,
            b"/home/luka/Projects/ioq3-server/src/qcommon/cvar.c\x00" as *const u8
                as *const libc::c_char,
            1389 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                b"void Cvar_Update(vmCvar_t *)\x00",
            ))
            .as_ptr(),
        );
    }
    if (*vmCvar).handle as libc::c_uint >= cvar_numIndexes as libc::c_uint {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"Cvar_Update: handle out of range\x00" as *const u8 as *const libc::c_char,
        );
    }
    cv = cvar_indexes.as_mut_ptr().offset((*vmCvar).handle as isize);
    if (*cv).modificationCount == (*vmCvar).modificationCount {
        return;
    }
    if (*cv).string.is_null() {
        return;
        // variable might have been cleared by a cvar_restart
    }
    (*vmCvar).modificationCount = (*cv).modificationCount;
    if crate::stdlib::strlen((*cv).string).wrapping_add(1 as libc::c_int as libc::c_ulong)
        > 256 as libc::c_int as libc::c_ulong
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"Cvar_Update: src %s length %u exceeds MAX_CVAR_VALUE_STRING\x00" as *const u8
                as *const libc::c_char,
            (*cv).string,
            crate::stdlib::strlen((*cv).string) as libc::c_uint,
        );
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*vmCvar).string.as_mut_ptr(),
        (*cv).string,
        256 as libc::c_int,
    );
    (*vmCvar).value = (*cv).value;
    (*vmCvar).integer = (*cv).integer;
}
/*
==================
Cvar_CompleteCvarName
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_CompleteCvarName(
    mut args: *mut libc::c_char,
    mut argNum: libc::c_int,
) {
    if argNum == 2 as libc::c_int {
        // Skip "<cmd> "
        let mut p: *mut libc::c_char = crate::src::qcommon::q_shared::Com_SkipTokens(
            args,
            1 as libc::c_int,
            b" \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if p > args {
            crate::src::qcommon::common::Field_CompleteCommand(
                p,
                crate::src::qcommon::q_shared::qfalse,
                crate::src::qcommon::q_shared::qtrue,
            );
        }
    };
}
// writes lines containing "set variable value" for all variables
// with the archive flag set to true.
/*
============
Cvar_Init

Reads in all archived cvars
============
*/
#[no_mangle]

pub unsafe extern "C" fn Cvar_Init() {
    crate::stdlib::memset(
        cvar_indexes.as_mut_ptr() as *mut libc::c_void,
        '\u{0}' as i32,
        ::std::mem::size_of::<[crate::src::qcommon::q_shared::cvar_t; 2048]>() as libc::c_ulong,
    );
    crate::stdlib::memset(
        hashTable.as_mut_ptr() as *mut libc::c_void,
        '\u{0}' as i32,
        ::std::mem::size_of::<[*mut crate::src::qcommon::q_shared::cvar_t; 256]>() as libc::c_ulong,
    );
    cvar_cheats = Cvar_Get(
        b"sv_cheats\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x40 as libc::c_int | 0x8 as libc::c_int,
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"print\x00" as *const u8 as *const libc::c_char,
        Some(Cvar_Print_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"toggle\x00" as *const u8 as *const libc::c_char,
        Some(Cvar_Toggle_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_SetCommandCompletionFunc(
        b"toggle\x00" as *const u8 as *const libc::c_char,
        Some(
            Cvar_CompleteCvarName
                as unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int) -> (),
        ),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"set\x00" as *const u8 as *const libc::c_char,
        Some(Cvar_Set_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_SetCommandCompletionFunc(
        b"set\x00" as *const u8 as *const libc::c_char,
        Some(
            Cvar_CompleteCvarName
                as unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int) -> (),
        ),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"sets\x00" as *const u8 as *const libc::c_char,
        Some(Cvar_Set_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_SetCommandCompletionFunc(
        b"sets\x00" as *const u8 as *const libc::c_char,
        Some(
            Cvar_CompleteCvarName
                as unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int) -> (),
        ),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"setu\x00" as *const u8 as *const libc::c_char,
        Some(Cvar_Set_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_SetCommandCompletionFunc(
        b"setu\x00" as *const u8 as *const libc::c_char,
        Some(
            Cvar_CompleteCvarName
                as unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int) -> (),
        ),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"seta\x00" as *const u8 as *const libc::c_char,
        Some(Cvar_Set_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_SetCommandCompletionFunc(
        b"seta\x00" as *const u8 as *const libc::c_char,
        Some(
            Cvar_CompleteCvarName
                as unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int) -> (),
        ),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"reset\x00" as *const u8 as *const libc::c_char,
        Some(Cvar_Reset_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_SetCommandCompletionFunc(
        b"reset\x00" as *const u8 as *const libc::c_char,
        Some(
            Cvar_CompleteCvarName
                as unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int) -> (),
        ),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"unset\x00" as *const u8 as *const libc::c_char,
        Some(Cvar_Unset_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_SetCommandCompletionFunc(
        b"unset\x00" as *const u8 as *const libc::c_char,
        Some(
            Cvar_CompleteCvarName
                as unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int) -> (),
        ),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"cvarlist\x00" as *const u8 as *const libc::c_char,
        Some(Cvar_List_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"cvar_modified\x00" as *const u8 as *const libc::c_char,
        Some(Cvar_ListModified_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"cvar_restart\x00" as *const u8 as *const libc::c_char,
        Some(Cvar_Restart_f as unsafe extern "C" fn() -> ()),
    );
}
