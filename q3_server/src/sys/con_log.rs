use ::libc;

use crate::stdlib::memcpy;
use crate::stdlib::strlen;

static mut consoleLog: [libc::c_char; 32768] = [0; 32768];

static mut writePos: libc::c_uint = 0 as libc::c_int as libc::c_uint;

static mut readPos: libc::c_uint = 0 as libc::c_int as libc::c_uint;
/*
==================
CON_LogSize
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CON_LogSize() -> libc::c_uint {
    if readPos <= writePos {
        return writePos.wrapping_sub(readPos);
    } else {
        return writePos
            .wrapping_add(32768 as libc::c_int as libc::c_uint)
            .wrapping_sub(readPos);
    };
}
/*
==================
CON_LogFree
==================
*/

unsafe extern "C" fn CON_LogFree() -> libc::c_uint {
    return (32768 as libc::c_int as libc::c_uint)
        .wrapping_sub(CON_LogSize())
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
}
/*
==================
CON_LogWrite
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CON_LogWrite(mut in_0: *const libc::c_char) -> libc::c_uint {
    let mut length: libc::c_uint = crate::stdlib::strlen(in_0) as libc::c_uint;
    let mut firstChunk: libc::c_uint = 0;
    let mut secondChunk: libc::c_uint = 0;
    while CON_LogFree() < length && CON_LogSize() > 0 as libc::c_int as libc::c_uint {
        // Free enough space
        while consoleLog[readPos as usize] as libc::c_int != '\n' as i32
            && CON_LogSize() > 1 as libc::c_int as libc::c_uint
        {
            readPos = readPos
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_rem(32768 as libc::c_int as libc::c_uint)
        }
        // Skip past the '\n'
        readPos = readPos
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_rem(32768 as libc::c_int as libc::c_uint)
    }
    if CON_LogFree() < length {
        return 0 as libc::c_int as libc::c_uint;
    }
    if writePos.wrapping_add(length) > 32768 as libc::c_int as libc::c_uint {
        firstChunk = (32768 as libc::c_int as libc::c_uint).wrapping_sub(writePos);
        secondChunk = length.wrapping_sub(firstChunk)
    } else {
        firstChunk = length;
        secondChunk = 0 as libc::c_int as libc::c_uint
    }
    crate::stdlib::memcpy(
        consoleLog.as_mut_ptr().offset(writePos as isize) as *mut libc::c_void,
        in_0 as *const libc::c_void,
        firstChunk as libc::c_ulong,
    );
    crate::stdlib::memcpy(
        consoleLog.as_mut_ptr() as *mut libc::c_void,
        in_0.offset(firstChunk as isize) as *const libc::c_void,
        secondChunk as libc::c_ulong,
    );
    writePos = writePos
        .wrapping_add(length)
        .wrapping_rem(32768 as libc::c_int as libc::c_uint);
    return length;
}
/*
==================
CON_LogRead
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CON_LogRead(
    mut out: *mut libc::c_char,
    mut outSize: libc::c_uint,
) -> libc::c_uint {
    let mut firstChunk: libc::c_uint = 0;
    let mut secondChunk: libc::c_uint = 0;
    if CON_LogSize() < outSize {
        outSize = CON_LogSize()
    }
    if readPos.wrapping_add(outSize) > 32768 as libc::c_int as libc::c_uint {
        firstChunk = (32768 as libc::c_int as libc::c_uint).wrapping_sub(readPos);
        secondChunk = outSize.wrapping_sub(firstChunk)
    } else {
        firstChunk = outSize;
        secondChunk = 0 as libc::c_int as libc::c_uint
    }
    crate::stdlib::memcpy(
        out as *mut libc::c_void,
        consoleLog.as_mut_ptr().offset(readPos as isize) as *const libc::c_void,
        firstChunk as libc::c_ulong,
    );
    crate::stdlib::memcpy(
        out.offset(firstChunk as isize) as *mut libc::c_void,
        consoleLog.as_mut_ptr() as *const libc::c_void,
        secondChunk as libc::c_ulong,
    );
    readPos = readPos
        .wrapping_add(outSize)
        .wrapping_rem(32768 as libc::c_int as libc::c_uint);
    return outSize;
}
