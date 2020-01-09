// =============== BEGIN ioapi_h ================
pub type open_file_func = Option<
    unsafe extern "C" fn(
        _: crate::stdlib::voidpf,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> crate::stdlib::voidpf,
>;

pub type read_file_func = Option<
    unsafe extern "C" fn(
        _: crate::stdlib::voidpf,
        _: crate::stdlib::voidpf,
        _: *mut libc::c_void,
        _: crate::stdlib::uLong,
    ) -> crate::stdlib::uLong,
>;

pub type write_file_func = Option<
    unsafe extern "C" fn(
        _: crate::stdlib::voidpf,
        _: crate::stdlib::voidpf,
        _: *const libc::c_void,
        _: crate::stdlib::uLong,
    ) -> crate::stdlib::uLong,
>;

pub type tell_file_func = Option<
    unsafe extern "C" fn(_: crate::stdlib::voidpf, _: crate::stdlib::voidpf) -> libc::c_long,
>;

pub type seek_file_func = Option<
    unsafe extern "C" fn(
        _: crate::stdlib::voidpf,
        _: crate::stdlib::voidpf,
        _: crate::stdlib::uLong,
        _: libc::c_int,
    ) -> libc::c_long,
>;

pub type close_file_func =
    Option<unsafe extern "C" fn(_: crate::stdlib::voidpf, _: crate::stdlib::voidpf) -> libc::c_int>;

pub type testerror_file_func =
    Option<unsafe extern "C" fn(_: crate::stdlib::voidpf, _: crate::stdlib::voidpf) -> libc::c_int>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zlib_filefunc_def_s {
    pub zopen_file: crate::src::qcommon::ioapi::open_file_func,
    pub zread_file: crate::src::qcommon::ioapi::read_file_func,
    pub zwrite_file: crate::src::qcommon::ioapi::write_file_func,
    pub ztell_file: crate::src::qcommon::ioapi::tell_file_func,
    pub zseek_file: crate::src::qcommon::ioapi::seek_file_func,
    pub zclose_file: crate::src::qcommon::ioapi::close_file_func,
    pub zerror_file: crate::src::qcommon::ioapi::testerror_file_func,
    pub opaque: crate::stdlib::voidpf,
}

pub type zlib_filefunc_def = crate::src::qcommon::ioapi::zlib_filefunc_def_s;
use ::libc;

pub use crate::stddef_h::size_t;

pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

use crate::stdlib::fclose;
use crate::stdlib::ferror;
use crate::stdlib::fopen;
use crate::stdlib::fread;
use crate::stdlib::fseek;
use crate::stdlib::ftell;
use crate::stdlib::fwrite;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidpf;
/* ioapi.c -- IO base function header for compress/uncompress .zip
   files using zlib + zip or unzip API

   Version 1.01e, February 12th, 2005

   Copyright (C) 1998-2005 Gilles Vollant
*/
/* I've found an old Unix (a SunOS 4.1.3_U1) without all SEEK_* defined.... */
#[no_mangle]

pub unsafe extern "C" fn fopen_file_func(
    mut opaque: crate::stdlib::voidpf,
    mut filename: *const libc::c_char,
    mut mode: libc::c_int,
) -> crate::stdlib::voidpf {
    let mut file: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
    let mut mode_fopen: *const libc::c_char = 0 as *const libc::c_char;
    if mode & 3 as libc::c_int == 1 as libc::c_int {
        mode_fopen = b"rb\x00" as *const u8 as *const libc::c_char
    } else if mode & 4 as libc::c_int != 0 {
        mode_fopen = b"r+b\x00" as *const u8 as *const libc::c_char
    } else if mode & 8 as libc::c_int != 0 {
        mode_fopen = b"wb\x00" as *const u8 as *const libc::c_char
    }
    if !filename.is_null() && !mode_fopen.is_null() {
        file = crate::stdlib::fopen(filename, mode_fopen)
    }
    return file as crate::stdlib::voidpf;
}
#[no_mangle]

pub unsafe extern "C" fn fread_file_func(
    mut opaque: crate::stdlib::voidpf,
    mut stream: crate::stdlib::voidpf,
    mut buf: *mut libc::c_void,
    mut size: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    let mut ret: crate::stdlib::uLong = 0;
    ret = crate::stdlib::fread(
        buf,
        1 as libc::c_int as crate::stddef_h::size_t,
        size,
        stream as *mut crate::stdlib::FILE,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn fwrite_file_func(
    mut opaque: crate::stdlib::voidpf,
    mut stream: crate::stdlib::voidpf,
    mut buf: *const libc::c_void,
    mut size: crate::stdlib::uLong,
) -> crate::stdlib::uLong {
    let mut ret: crate::stdlib::uLong = 0;
    ret = crate::stdlib::fwrite(
        buf,
        1 as libc::c_int as crate::stddef_h::size_t,
        size,
        stream as *mut crate::stdlib::FILE,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn ftell_file_func(
    mut opaque: crate::stdlib::voidpf,
    mut stream: crate::stdlib::voidpf,
) -> libc::c_long {
    let mut ret: libc::c_long = 0;
    ret = crate::stdlib::ftell(stream as *mut crate::stdlib::FILE);
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn fseek_file_func(
    mut opaque: crate::stdlib::voidpf,
    mut stream: crate::stdlib::voidpf,
    mut offset: crate::stdlib::uLong,
    mut origin: libc::c_int,
) -> libc::c_long {
    let mut fseek_origin: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_long = 0;
    match origin {
        1 => fseek_origin = 1 as libc::c_int,
        2 => fseek_origin = 2 as libc::c_int,
        0 => fseek_origin = 0 as libc::c_int,
        _ => return -(1 as libc::c_int) as libc::c_long,
    }
    ret = 0 as libc::c_int as libc::c_long;
    crate::stdlib::fseek(
        stream as *mut crate::stdlib::FILE,
        offset as libc::c_long,
        fseek_origin,
    );
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn fclose_file_func(
    mut opaque: crate::stdlib::voidpf,
    mut stream: crate::stdlib::voidpf,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = crate::stdlib::fclose(stream as *mut crate::stdlib::FILE);
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn ferror_file_func(
    mut opaque: crate::stdlib::voidpf,
    mut stream: crate::stdlib::voidpf,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = crate::stdlib::ferror(stream as *mut crate::stdlib::FILE);
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn fill_fopen_filefunc(
    mut pzlib_filefunc_def: *mut crate::src::qcommon::ioapi::zlib_filefunc_def,
) {
    (*pzlib_filefunc_def).zopen_file = ::std::mem::transmute::<
        Option<unsafe extern "C" fn() -> crate::stdlib::voidpf>,
        crate::src::qcommon::ioapi::open_file_func,
    >(Some(::std::mem::transmute::<
        unsafe extern "C" fn(
            _: crate::stdlib::voidpf,
            _: *const libc::c_char,
            _: libc::c_int,
        ) -> crate::stdlib::voidpf,
        unsafe extern "C" fn() -> crate::stdlib::voidpf,
    >(fopen_file_func)));
    (*pzlib_filefunc_def).zread_file = ::std::mem::transmute::<
        Option<unsafe extern "C" fn() -> crate::stdlib::uLong>,
        crate::src::qcommon::ioapi::read_file_func,
    >(Some(::std::mem::transmute::<
        unsafe extern "C" fn(
            _: crate::stdlib::voidpf,
            _: crate::stdlib::voidpf,
            _: *mut libc::c_void,
            _: crate::stdlib::uLong,
        ) -> crate::stdlib::uLong,
        unsafe extern "C" fn() -> crate::stdlib::uLong,
    >(fread_file_func)));
    (*pzlib_filefunc_def).zwrite_file = ::std::mem::transmute::<
        Option<unsafe extern "C" fn() -> crate::stdlib::uLong>,
        crate::src::qcommon::ioapi::write_file_func,
    >(Some(::std::mem::transmute::<
        unsafe extern "C" fn(
            _: crate::stdlib::voidpf,
            _: crate::stdlib::voidpf,
            _: *const libc::c_void,
            _: crate::stdlib::uLong,
        ) -> crate::stdlib::uLong,
        unsafe extern "C" fn() -> crate::stdlib::uLong,
    >(fwrite_file_func)));
    (*pzlib_filefunc_def).ztell_file = ::std::mem::transmute::<
        Option<unsafe extern "C" fn() -> libc::c_long>,
        crate::src::qcommon::ioapi::tell_file_func,
    >(Some(::std::mem::transmute::<
        unsafe extern "C" fn(_: crate::stdlib::voidpf, _: crate::stdlib::voidpf) -> libc::c_long,
        unsafe extern "C" fn() -> libc::c_long,
    >(ftell_file_func)));
    (*pzlib_filefunc_def).zseek_file = ::std::mem::transmute::<
        Option<unsafe extern "C" fn() -> libc::c_long>,
        crate::src::qcommon::ioapi::seek_file_func,
    >(Some(::std::mem::transmute::<
        unsafe extern "C" fn(
            _: crate::stdlib::voidpf,
            _: crate::stdlib::voidpf,
            _: crate::stdlib::uLong,
            _: libc::c_int,
        ) -> libc::c_long,
        unsafe extern "C" fn() -> libc::c_long,
    >(fseek_file_func)));
    (*pzlib_filefunc_def).zclose_file = ::std::mem::transmute::<
        Option<unsafe extern "C" fn() -> libc::c_int>,
        crate::src::qcommon::ioapi::close_file_func,
    >(Some(::std::mem::transmute::<
        unsafe extern "C" fn(_: crate::stdlib::voidpf, _: crate::stdlib::voidpf) -> libc::c_int,
        unsafe extern "C" fn() -> libc::c_int,
    >(fclose_file_func)));
    (*pzlib_filefunc_def).zerror_file = ::std::mem::transmute::<
        Option<unsafe extern "C" fn() -> libc::c_int>,
        crate::src::qcommon::ioapi::testerror_file_func,
    >(Some(::std::mem::transmute::<
        unsafe extern "C" fn(_: crate::stdlib::voidpf, _: crate::stdlib::voidpf) -> libc::c_int,
        unsafe extern "C" fn() -> libc::c_int,
    >(ferror_file_func)));
    (*pzlib_filefunc_def).opaque = 0 as *mut libc::c_void;
}
