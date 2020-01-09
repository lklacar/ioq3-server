use ::libc;

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdarg_h::va_list;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__blkcnt_t;
pub use crate::stdlib::__blksize_t;
pub use crate::stdlib::__dev_t;
pub use crate::stdlib::__gid_t;
pub use crate::stdlib::__ino_t;
pub use crate::stdlib::__mode_t;
pub use crate::stdlib::__nlink_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__sighandler_t;
pub use crate::stdlib::__syscall_slong_t;
pub use crate::stdlib::__time_t;
pub use crate::stdlib::__uid_t;
pub use crate::stdlib::intptr_t;
pub use crate::stdlib::signal;
pub use crate::stdlib::stat;
pub use crate::stdlib::timespec;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::qcommon_h::cpuFeatures_t;
pub use crate::qcommon_h::xcommand_t;
pub use crate::qcommon_h::CF_3DNOW;
pub use crate::qcommon_h::CF_3DNOW_EXT;
pub use crate::qcommon_h::CF_ALTIVEC;
pub use crate::qcommon_h::CF_MMX;
pub use crate::qcommon_h::CF_MMX_EXT;
pub use crate::qcommon_h::CF_RDTSC;
pub use crate::qcommon_h::CF_SSE;
pub use crate::qcommon_h::CF_SSE2;
pub use crate::src::null::null_input::IN_Restart;
pub use crate::src::qcommon::cmd::Cmd_AddCommand;
pub use crate::src::qcommon::common::com_fullyInitialized;
pub use crate::src::qcommon::common::Com_Frame;
pub use crate::src::qcommon::common::Com_Init;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::cvar::Cvar_Set;
pub use crate::src::qcommon::cvar::Cvar_VariableString;
pub use crate::src::qcommon::files::FS_CreatePath;
pub use crate::src::qcommon::files::FS_FilenameCompare;
pub use crate::src::qcommon::files::FS_GetCurrentGameDir;
pub use crate::src::qcommon::net_ip::NET_Init;
pub use crate::src::qcommon::net_ip::NET_Shutdown;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_IsColorString;
pub use crate::src::qcommon::q_shared::Q_strcat;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::server::sv_init::SV_Shutdown;
use crate::src::sys::con_log::CON_LogWrite;
use crate::src::sys::con_tty::CON_Init;
use crate::src::sys::con_tty::CON_Input;
use crate::src::sys::con_tty::CON_Print;
use crate::src::sys::con_tty::CON_Shutdown;
pub use crate::src::sys::sys_unix::Sys_Cwd;
pub use crate::src::sys::sys_unix::Sys_Dirname;
pub use crate::src::sys::sys_unix::Sys_DllExtension;
use crate::src::sys::sys_unix::Sys_ErrorDialog;
pub use crate::src::sys::sys_unix::Sys_GetCurrentUser;
pub use crate::src::sys::sys_unix::Sys_Milliseconds;
use crate::src::sys::sys_unix::Sys_PID;
use crate::src::sys::sys_unix::Sys_PIDIsRunning;
use crate::src::sys::sys_unix::Sys_PlatformExit;
use crate::src::sys::sys_unix::Sys_PlatformInit;
pub use crate::src::vm::vm::VM_Forced_Unload_Done;
pub use crate::src::vm::vm::VM_Forced_Unload_Start;
use crate::stdlib::__assert_fail;
use crate::stdlib::atoi;
use crate::stdlib::dlclose;
use crate::stdlib::dlerror;
use crate::stdlib::dlopen;
use crate::stdlib::dlsym;
use crate::stdlib::exit;
use crate::stdlib::fclose;
use crate::stdlib::fopen;
use crate::stdlib::fprintf;
use crate::stdlib::fputs;
use crate::stdlib::fread;
use crate::stdlib::remove;
use crate::stdlib::stderr;
use crate::stdlib::stdout;
use crate::stdlib::strchr;
use crate::stdlib::strcmp;
use crate::stdlib::vsnprintf;
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

static mut binaryPath: [libc::c_char; 4096] = [0; 4096];

static mut installPath: [libc::c_char; 4096] = [0; 4096];
/*
=================
Sys_SetBinaryPath
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_SetBinaryPath(mut path: *const libc::c_char) {
    crate::src::qcommon::q_shared::Q_strncpyz(
        binaryPath.as_mut_ptr(),
        path,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
    );
}
/*
=================
Sys_BinaryPath
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_BinaryPath() -> *mut libc::c_char {
    return binaryPath.as_mut_ptr();
}
/*
=================
Sys_SetDefaultInstallPath
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_SetDefaultInstallPath(mut path: *const libc::c_char) {
    crate::src::qcommon::q_shared::Q_strncpyz(
        installPath.as_mut_ptr(),
        path,
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
    );
}
/*
=================
Sys_DefaultInstallPath
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_DefaultInstallPath() -> *mut libc::c_char {
    if *installPath.as_mut_ptr() != 0 {
        return installPath.as_mut_ptr();
    } else {
        return crate::src::sys::sys_unix::Sys_Cwd();
    };
}
/*
=================
Sys_DefaultAppPath
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_DefaultAppPath() -> *mut libc::c_char {
    return Sys_BinaryPath();
}
/*
=================
Sys_In_Restart_f

Restart the input subsystem
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_In_Restart_f() {
    crate::src::null::null_input::IN_Restart();
}
/*
=================
Sys_ConsoleInput

Handle new console input
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_ConsoleInput() -> *mut libc::c_char {
    return crate::src::sys::con_tty::CON_Input();
}
/*
==================
Sys_GetClipboardData
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_GetClipboardData() -> *mut libc::c_char {
    return 0 as *mut libc::c_char;
}
/*
=================
Sys_PIDFileName
=================
*/

unsafe extern "C" fn Sys_PIDFileName(mut gamedir: *const libc::c_char) -> *mut libc::c_char {
    let mut homePath: *const libc::c_char = crate::src::qcommon::cvar::Cvar_VariableString(
        b"fs_homepath\x00" as *const u8 as *const libc::c_char,
    );
    if *homePath as libc::c_int != '\u{0}' as i32 {
        return crate::src::qcommon::q_shared::va(
            b"%s/%s/%s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            homePath,
            gamedir,
            b"ioq3_server.pid\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as *mut libc::c_char;
}
/*
=================
Sys_RemovePIDFile
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_RemovePIDFile(mut gamedir: *const libc::c_char) {
    let mut pidFile: *mut libc::c_char = Sys_PIDFileName(gamedir);
    if !pidFile.is_null() {
        crate::stdlib::remove(pidFile);
    };
}
/*
=================
Sys_WritePIDFile

Return qtrue if there is an existing stale PID file
=================
*/

unsafe extern "C" fn Sys_WritePIDFile(
    mut gamedir: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut pidFile: *mut libc::c_char = Sys_PIDFileName(gamedir);
    let mut f: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    let mut stale: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    if pidFile.is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    // First, check if the pid file is already there
    f = crate::stdlib::fopen(pidFile, b"r\x00" as *const u8 as *const libc::c_char);
    if !f.is_null() {
        let mut pidBuffer: [libc::c_char; 64] = [0; 64];

        let mut pid: libc::c_int = 0;
        pid = crate::stdlib::fread(
            pidBuffer.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            (::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            f,
        ) as libc::c_int;
        crate::stdlib::fclose(f);
        if pid > 0 as libc::c_int {
            pid = crate::stdlib::atoi(pidBuffer.as_mut_ptr());
            if crate::src::sys::sys_unix::Sys_PIDIsRunning(pid) as u64 == 0 {
                stale = crate::src::qcommon::q_shared::qtrue
            }
        } else {
            stale = crate::src::qcommon::q_shared::qtrue
        }
    }
    if crate::src::qcommon::files::FS_CreatePath(pidFile) as u64 != 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    f = crate::stdlib::fopen(pidFile, b"w\x00" as *const u8 as *const libc::c_char);
    if !f.is_null() {
        crate::stdlib::fprintf(
            f,
            b"%d\x00" as *const u8 as *const libc::c_char,
            crate::src::sys::sys_unix::Sys_PID(),
        );
        crate::stdlib::fclose(f);
    } else {
        crate::src::qcommon::common::Com_Printf(
            b"^3Couldn\'t write %s.\n\x00" as *const u8 as *const libc::c_char,
            pidFile,
        );
    }
    return stale;
}
/*
=================
Sys_InitPIDFile
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_InitPIDFile(mut gamedir: *const libc::c_char) {
    (Sys_WritePIDFile(gamedir) as u64) != 0;
}
/*
=================
Sys_Exit

Single exit point (regular exit or in case of error)
=================
*/

unsafe extern "C" fn Sys_Exit(mut exitCode: libc::c_int) -> ! {
    crate::src::sys::con_tty::CON_Shutdown();
    if exitCode < 2 as libc::c_int
        && crate::src::qcommon::common::com_fullyInitialized as libc::c_uint != 0
    {
        // Normal exit
        Sys_RemovePIDFile(crate::src::qcommon::files::FS_GetCurrentGameDir());
    }
    crate::src::qcommon::net_ip::NET_Shutdown();
    crate::src::sys::sys_unix::Sys_PlatformExit();
    crate::stdlib::exit(exitCode);
}
/*
=================
Sys_Quit
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_Quit() -> ! {
    Sys_Exit(0 as libc::c_int);
}
/*
=================
Sys_GetProcessorFeatures
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_GetProcessorFeatures() -> crate::qcommon_h::cpuFeatures_t {
    let mut features: crate::qcommon_h::cpuFeatures_t = 0 as crate::qcommon_h::cpuFeatures_t;
    return features;
}
/*
=================
Sys_Init
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_Init() {
    crate::src::qcommon::cmd::Cmd_AddCommand(
        b"in_restart\x00" as *const u8 as *const libc::c_char,
        Some(Sys_In_Restart_f as unsafe extern "C" fn() -> ()),
    );
    crate::src::qcommon::cvar::Cvar_Set(
        b"arch\x00" as *const u8 as *const libc::c_char,
        b"linux x86_64\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::qcommon::cvar::Cvar_Set(
        b"username\x00" as *const u8 as *const libc::c_char,
        crate::src::sys::sys_unix::Sys_GetCurrentUser(),
    );
}
/*
=================
Sys_AnsiColorPrint

Transform Q3 colour codes to ANSI escape sequences
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_AnsiColorPrint(mut msg: *const libc::c_char) {
    static mut buffer: [libc::c_char; 4096] = [0; 4096];
    let mut length: libc::c_int = 0 as libc::c_int;
    static mut q3ToAnsi: [libc::c_int; 8] = [
        30 as libc::c_int,
        31 as libc::c_int,
        32 as libc::c_int,
        33 as libc::c_int,
        34 as libc::c_int,
        36 as libc::c_int,
        35 as libc::c_int,
        0 as libc::c_int,
    ];
    while *msg != 0 {
        if crate::src::qcommon::q_shared::Q_IsColorString(msg) as libc::c_uint != 0
            || *msg as libc::c_int == '\n' as i32
        {
            // First empty the buffer
            if length > 0 as libc::c_int {
                buffer[length as usize] = '\u{0}' as i32 as libc::c_char;
                crate::stdlib::fputs(buffer.as_mut_ptr(), crate::stdlib::stderr);
                length = 0 as libc::c_int
            }
            if *msg as libc::c_int == '\n' as i32 {
                // Issue a reset and then the newline
                crate::stdlib::fputs(
                    b"\x1b[0m\n\x00" as *const u8 as *const libc::c_char,
                    crate::stdlib::stderr,
                );
                msg = msg.offset(1)
            } else {
                // Print the color code
                crate::src::qcommon::q_shared::Com_sprintf(
                    buffer.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
                    b"\x1b[%dm\x00" as *const u8 as *const libc::c_char,
                    q3ToAnsi[(*msg.offset(1 as libc::c_int as isize) as libc::c_int - '0' as i32
                        & 0x7 as libc::c_int) as usize],
                );
                crate::stdlib::fputs(buffer.as_mut_ptr(), crate::stdlib::stderr);
                msg = msg.offset(2 as libc::c_int as isize)
            }
        } else {
            if length >= 4096 as libc::c_int - 1 as libc::c_int {
                break;
            }
            buffer[length as usize] = *msg;
            length += 1;
            msg = msg.offset(1)
        }
    }
    // Empty anything still left in the buffer
    if length > 0 as libc::c_int {
        buffer[length as usize] = '\u{0}' as i32 as libc::c_char;
        crate::stdlib::fputs(buffer.as_mut_ptr(), crate::stdlib::stderr);
    };
}
/*
=================
Sys_Print
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_Print(mut msg: *const libc::c_char) {
    crate::src::sys::con_log::CON_LogWrite(msg);
    crate::src::sys::con_tty::CON_Print(msg);
}
/*
=================
Sys_Error
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_Error(mut error: *const libc::c_char, mut args: ...) -> ! {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut string: [libc::c_char; 1024] = [0; 1024];
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        string.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        error,
        argptr.as_va_list(),
    );
    crate::src::sys::sys_unix::Sys_ErrorDialog(string.as_mut_ptr());
    Sys_Exit(3 as libc::c_int);
}
/*
============
Sys_FileTime

returns -1 if not present
============
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_FileTime(mut path: *mut libc::c_char) -> libc::c_int {
    let mut buf: crate::stdlib::stat = crate::stdlib::stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: crate::stdlib::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: crate::stdlib::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: crate::stdlib::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if crate::stdlib::stat(path, &mut buf) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    return buf.st_mtim.tv_sec as libc::c_int;
}
/*
=================
Sys_UnloadDll
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_UnloadDll(mut dllHandle: *mut libc::c_void) {
    if dllHandle.is_null() {
        crate::src::qcommon::common::Com_Printf(
            b"Sys_UnloadDll(NULL)\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    crate::stdlib::dlclose(dllHandle);
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
=================
Sys_LoadDll

First try to load library name from system library path,
from executable path, then fs_basepath.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_LoadDll(
    mut name: *const libc::c_char,
    mut useSystemLib: crate::src::qcommon::q_shared::qboolean,
) -> *mut libc::c_void {
    let mut dllhandle: *mut libc::c_void = 0 as *mut libc::c_void;
    if crate::src::sys::sys_unix::Sys_DllExtension(name) as u64 == 0 {
        crate::src::qcommon::common::Com_Printf(
            b"Refusing to attempt to load library \"%s\": Extension not allowed.\n\x00" as *const u8
                as *const libc::c_char,
            name,
        );
        return 0 as *mut libc::c_void;
    }
    if useSystemLib as u64 != 0 {
        crate::src::qcommon::common::Com_Printf(
            b"Trying to load \"%s\"...\n\x00" as *const u8 as *const libc::c_char,
            name,
        );
        dllhandle = crate::stdlib::dlopen(name, 0x2 as libc::c_int)
    }
    if dllhandle.is_null() {
        let mut topDir: *const libc::c_char = 0 as *const libc::c_char;
        let mut libPath: [libc::c_char; 4096] = [0; 4096];
        let mut len: libc::c_int = 0;
        topDir = Sys_BinaryPath();
        if *topDir == 0 {
            topDir = b".\x00" as *const u8 as *const libc::c_char
        }
        len = crate::src::qcommon::q_shared::Com_sprintf(
            libPath.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
            b"%s%c%s\x00" as *const u8 as *const libc::c_char,
            topDir,
            '/' as i32,
            name,
        );
        if (len as libc::c_ulong) < ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong {
            crate::src::qcommon::common::Com_Printf(
                b"Trying to load \"%s\" from \"%s\"...\n\x00" as *const u8 as *const libc::c_char,
                name,
                topDir,
            );
            dllhandle = crate::stdlib::dlopen(libPath.as_mut_ptr(), 0x2 as libc::c_int)
        } else {
            crate::src::qcommon::common::Com_Printf(
                b"Skipping trying to load \"%s\" from \"%s\", file name is too long.\n\x00"
                    as *const u8 as *const libc::c_char,
                name,
                topDir,
            );
        }
        if dllhandle.is_null() {
            let mut basePath: *const libc::c_char = crate::src::qcommon::cvar::Cvar_VariableString(
                b"fs_basepath\x00" as *const u8 as *const libc::c_char,
            );
            if basePath.is_null() || *basePath == 0 {
                basePath = b".\x00" as *const u8 as *const libc::c_char
            }
            if crate::src::qcommon::files::FS_FilenameCompare(topDir, basePath) as u64 != 0 {
                len = crate::src::qcommon::q_shared::Com_sprintf(
                    libPath.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
                    b"%s%c%s\x00" as *const u8 as *const libc::c_char,
                    basePath,
                    '/' as i32,
                    name,
                );
                if (len as libc::c_ulong)
                    < ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                {
                    crate::src::qcommon::common::Com_Printf(
                        b"Trying to load \"%s\" from \"%s\"...\n\x00" as *const u8
                            as *const libc::c_char,
                        name,
                        basePath,
                    );
                    dllhandle = crate::stdlib::dlopen(libPath.as_mut_ptr(), 0x2 as libc::c_int)
                } else {
                    crate::src::qcommon::common::Com_Printf(
                        b"Skipping trying to load \"%s\" from \"%s\", file name is too long.\n\x00"
                            as *const u8 as *const libc::c_char,
                        name,
                        basePath,
                    );
                }
            }
            if dllhandle.is_null() {
                crate::src::qcommon::common::Com_Printf(
                    b"Loading \"%s\" failed\n\x00" as *const u8 as *const libc::c_char,
                    name,
                );
            }
        }
    }
    return dllhandle;
}
/*
=================
Sys_LoadGameDll

Used to load a development dll instead of a virtual machine
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_LoadGameDll(
    mut name: *const libc::c_char,
    mut entryPoint: *mut Option<
        unsafe extern "C" fn(_: libc::c_int, _: ...) -> crate::stdlib::intptr_t,
    >,
    mut systemcalls: Option<
        unsafe extern "C" fn(_: crate::stdlib::intptr_t, _: ...) -> crate::stdlib::intptr_t,
    >,
) -> *mut libc::c_void {
    let mut libHandle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut dllEntry: Option<
        unsafe extern "C" fn(
            _: Option<
                unsafe extern "C" fn(_: crate::stdlib::intptr_t, _: ...) -> crate::stdlib::intptr_t,
            >,
        ) -> (),
    > = None;
    if !name.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"name\x00" as *const u8 as *const libc::c_char,
                      b"/home/luka/Projects/ioq3-server/src/sys/sys_main.c\x00"
                          as *const u8 as *const libc::c_char,
                      588 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 90],
                                                &[libc::c_char; 90]>(b"void *Sys_LoadGameDll(const char *, intptr_t (**)(int, ...), intptr_t (*)(intptr_t, ...))\x00")).as_ptr());
    }
    if crate::src::sys::sys_unix::Sys_DllExtension(name) as u64 == 0 {
        crate::src::qcommon::common::Com_Printf(
            b"Refusing to attempt to load library \"%s\": Extension not allowed.\n\x00" as *const u8
                as *const libc::c_char,
            name,
        );
        return 0 as *mut libc::c_void;
    }
    crate::src::qcommon::common::Com_Printf(
        b"Loading DLL file: %s\n\x00" as *const u8 as *const libc::c_char,
        name,
    );
    libHandle = crate::stdlib::dlopen(name, 0x2 as libc::c_int);
    if libHandle.is_null() {
        crate::src::qcommon::common::Com_Printf(
            b"Sys_LoadGameDll(%s) failed:\n\"%s\"\n\x00" as *const u8 as *const libc::c_char,
            name,
            crate::stdlib::dlerror(),
        );
        return 0 as *mut libc::c_void;
    }
    dllEntry = ::std::mem::transmute::<
        *mut libc::c_void,
        Option<
            unsafe extern "C" fn(
                _: Option<
                    unsafe extern "C" fn(
                        _: crate::stdlib::intptr_t,
                        _: ...
                    ) -> crate::stdlib::intptr_t,
                >,
            ) -> (),
        >,
    >(crate::stdlib::dlsym(
        libHandle,
        b"dllEntry\x00" as *const u8 as *const libc::c_char,
    ));
    *entryPoint = ::std::mem::transmute::<
        *mut libc::c_void,
        Option<unsafe extern "C" fn(_: libc::c_int, _: ...) -> crate::stdlib::intptr_t>,
    >(crate::stdlib::dlsym(
        libHandle,
        b"vmMain\x00" as *const u8 as *const libc::c_char,
    ));
    if (*entryPoint).is_none() || dllEntry.is_none() {
        crate::src::qcommon::common::Com_Printf(
            b"Sys_LoadGameDll(%s) failed to find vmMain function:\n\"%s\" !\n\x00" as *const u8
                as *const libc::c_char,
            name,
            crate::stdlib::dlerror(),
        );
        crate::stdlib::dlclose(libHandle);
        return 0 as *mut libc::c_void;
    }
    crate::src::qcommon::common::Com_Printf(
        b"Sys_LoadGameDll(%s) found vmMain function at %p\n\x00" as *const u8
            as *const libc::c_char,
        name,
        *entryPoint,
    );
    dllEntry.expect("non-null function pointer")(systemcalls);
    return libHandle;
}
/*
=================
Sys_ParseArgs
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_ParseArgs(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) {
    if argc == 2 as libc::c_int {
        if crate::stdlib::strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"--version\x00" as *const u8 as *const libc::c_char,
        ) == 0
            || crate::stdlib::strcmp(
                *argv.offset(1 as libc::c_int as isize),
                b"-v\x00" as *const u8 as *const libc::c_char,
            ) == 0
        {
            let mut date: *const libc::c_char =
                b"Jan  9 2020\x00" as *const u8 as *const libc::c_char;
            crate::stdlib::fprintf(
                crate::stdlib::stdout,
                b"ioq3 1.36 dedicated server (%s)\n\x00" as *const u8 as *const libc::c_char,
                date,
            );
            Sys_Exit(0 as libc::c_int);
        }
    };
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
// Console
/*
=================
Sys_SigHandler
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_SigHandler(mut signal_0: libc::c_int) -> ! {
    static mut signalcaught: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    if signalcaught as u64 != 0 {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"DOUBLE SIGNAL FAULT: Received signal %d, exiting...\n\x00" as *const u8
                as *const libc::c_char,
            signal_0,
        );
    } else {
        signalcaught = crate::src::qcommon::q_shared::qtrue;
        crate::src::vm::vm::VM_Forced_Unload_Start();
        crate::src::server::sv_init::SV_Shutdown(crate::src::qcommon::q_shared::va(
            b"Received signal %d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            signal_0,
        ));
        crate::src::vm::vm::VM_Forced_Unload_Done();
    }
    if signal_0 == 15 as libc::c_int || signal_0 == 2 as libc::c_int {
        Sys_Exit(1 as libc::c_int);
    } else {
        Sys_Exit(2 as libc::c_int);
    };
}
/*
=================
main
=================
*/

unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut commandLine: [libc::c_char; 1024] = [0; 1024];

    extern "C" {
        #[link_name = "Sys_LaunchAutoupdater"]
        pub fn Sys_LaunchAutoupdater_0(argc_0: libc::c_int, argv_0: *mut *mut libc::c_char);
    }
    Sys_LaunchAutoupdater_0(argc, argv);
    crate::src::sys::sys_unix::Sys_PlatformInit();
    // Set the initial time base
    crate::src::sys::sys_unix::Sys_Milliseconds();
    Sys_ParseArgs(argc, argv);
    Sys_SetBinaryPath(crate::src::sys::sys_unix::Sys_Dirname(
        *argv.offset(0 as libc::c_int as isize),
    ));
    Sys_SetDefaultInstallPath(Sys_BinaryPath());
    // Concatenate the command line for passing to Com_Init
    i = 1 as libc::c_int;
    while i < argc {
        let containsSpaces: crate::src::qcommon::q_shared::qboolean =
            (crate::stdlib::strchr(*argv.offset(i as isize), ' ' as i32)
                != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
                as crate::src::qcommon::q_shared::qboolean;
        if containsSpaces as u64 != 0 {
            crate::src::qcommon::q_shared::Q_strcat(
                commandLine.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
                b"\"\x00" as *const u8 as *const libc::c_char,
            );
        }
        crate::src::qcommon::q_shared::Q_strcat(
            commandLine.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            *argv.offset(i as isize),
        );
        if containsSpaces as u64 != 0 {
            crate::src::qcommon::q_shared::Q_strcat(
                commandLine.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
                b"\"\x00" as *const u8 as *const libc::c_char,
            );
        }
        crate::src::qcommon::q_shared::Q_strcat(
            commandLine.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b" \x00" as *const u8 as *const libc::c_char,
        );
        i += 1
    }
    crate::src::sys::con_tty::CON_Init();
    crate::src::qcommon::common::Com_Init(commandLine.as_mut_ptr());
    crate::src::qcommon::net_ip::NET_Init();
    loop {
        crate::src::qcommon::common::Com_Frame();
    }
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}