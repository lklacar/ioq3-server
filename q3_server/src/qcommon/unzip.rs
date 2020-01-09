// =============== BEGIN unzip_h ================
pub type unzFile = crate::stdlib::voidp;

pub type unz_file_info = crate::src::qcommon::unzip::unz_file_info_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unz_file_info_s {
    pub version: crate::stdlib::uLong,
    pub version_needed: crate::stdlib::uLong,
    pub flag: crate::stdlib::uLong,
    pub compression_method: crate::stdlib::uLong,
    pub dosDate: crate::stdlib::uLong,
    pub crc: crate::stdlib::uLong,
    pub compressed_size: crate::stdlib::uLong,
    pub uncompressed_size: crate::stdlib::uLong,
    pub size_filename: crate::stdlib::uLong,
    pub size_file_extra: crate::stdlib::uLong,
    pub size_file_comment: crate::stdlib::uLong,
    pub disk_num_start: crate::stdlib::uLong,
    pub internal_fa: crate::stdlib::uLong,
    pub external_fa: crate::stdlib::uLong,
    pub tmu_date: crate::src::qcommon::unzip::tm_unz,
}

pub type tm_unz = crate::src::qcommon::unzip::tm_unz_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tm_unz_s {
    pub tm_sec: crate::stdlib::uInt,
    pub tm_min: crate::stdlib::uInt,
    pub tm_hour: crate::stdlib::uInt,
    pub tm_mday: crate::stdlib::uInt,
    pub tm_mon: crate::stdlib::uInt,
    pub tm_year: crate::stdlib::uInt,
}

pub type unz_global_info = crate::src::qcommon::unzip::unz_global_info_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unz_global_info_s {
    pub number_entry: crate::stdlib::uLong,
    pub size_comment: crate::stdlib::uLong,
}

pub type unz_file_pos = crate::src::qcommon::unzip::unz_file_pos_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unz_file_pos_s {
    pub pos_in_zip_directory: crate::stdlib::uLong,
    pub num_of_file: crate::stdlib::uLong,
}
use ::libc;

pub use crate::stdlib::__off_t;
pub use crate::stdlib::off_t;

use crate::src::qcommon::common::Z_Free;
use crate::src::qcommon::common::Z_MallocDebug;
pub use crate::src::qcommon::ioapi::close_file_func;
pub use crate::src::qcommon::ioapi::fill_fopen_filefunc;
pub use crate::src::qcommon::ioapi::open_file_func;
pub use crate::src::qcommon::ioapi::read_file_func;
pub use crate::src::qcommon::ioapi::seek_file_func;
pub use crate::src::qcommon::ioapi::tell_file_func;
pub use crate::src::qcommon::ioapi::testerror_file_func;
pub use crate::src::qcommon::ioapi::write_file_func;
pub use crate::src::qcommon::ioapi::zlib_filefunc_def;
pub use crate::src::qcommon::ioapi::zlib_filefunc_def_s;
pub use crate::stdlib::alloc_func;
pub use crate::stdlib::crc32;
pub use crate::stdlib::free_func;
pub use crate::stdlib::inflate;
pub use crate::stdlib::inflateEnd;
pub use crate::stdlib::inflateInit2_;
pub use crate::stdlib::internal_state;
use crate::stdlib::strcmp;
use crate::stdlib::strlen;
pub use crate::stdlib::uInt;
pub use crate::stdlib::uLong;
pub use crate::stdlib::voidp;
pub use crate::stdlib::voidpf;
pub use crate::stdlib::z_stream;
pub use crate::stdlib::z_stream_s;
pub use crate::stdlib::z_streamp;
pub use crate::stdlib::Byte;
pub use crate::stdlib::Bytef;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_in_zip_read_info_s {
    pub read_buffer: *mut libc::c_char,
    pub stream: crate::stdlib::z_stream,
    pub pos_in_zipfile: crate::stdlib::uLong,
    pub stream_initialised: crate::stdlib::uLong,
    pub offset_local_extrafield: crate::stdlib::uLong,
    pub size_local_extrafield: crate::stdlib::uInt,
    pub pos_local_extrafield: crate::stdlib::uLong,
    pub crc32: crate::stdlib::uLong,
    pub crc32_wait: crate::stdlib::uLong,
    pub rest_read_compressed: crate::stdlib::uLong,
    pub rest_read_uncompressed: crate::stdlib::uLong,
    pub z_filefunc: crate::src::qcommon::ioapi::zlib_filefunc_def,
    pub filestream: crate::stdlib::voidpf,
    pub compression_method: crate::stdlib::uLong,
    pub byte_before_the_zipfile: crate::stdlib::uLong,
    pub raw: libc::c_int,
}
/* unz_file_info_interntal contain internal info about a file in zipfile*/

pub type unz_file_info_internal = unz_file_info_internal_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unz_file_info_internal_s {
    pub offset_curfile: crate::stdlib::uLong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unz_s {
    pub z_filefunc: crate::src::qcommon::ioapi::zlib_filefunc_def,
    pub filestream: crate::stdlib::voidpf,
    pub gi: crate::src::qcommon::unzip::unz_global_info,
    pub byte_before_the_zipfile: crate::stdlib::uLong,
    pub num_file: crate::stdlib::uLong,
    pub pos_in_central_dir: crate::stdlib::uLong,
    pub current_file_ok: crate::stdlib::uLong,
    pub central_pos: crate::stdlib::uLong,
    pub size_central_dir: crate::stdlib::uLong,
    pub offset_central_dir: crate::stdlib::uLong,
    pub cur_file_info: crate::src::qcommon::unzip::unz_file_info,
    pub cur_file_info_internal: unz_file_info_internal,
    pub pfile_in_zip_read: *mut file_in_zip_read_info_s,
    pub encrypted: libc::c_int,
}
#[no_mangle]

pub static mut unz_copyright: [libc::c_char; 81] = [
    32, 117, 110, 122, 105, 112, 32, 49, 46, 48, 49, 32, 67, 111, 112, 121, 114, 105, 103, 104,
    116, 32, 49, 57, 57, 56, 45, 50, 48, 48, 52, 32, 71, 105, 108, 108, 101, 115, 32, 86, 111, 108,
    108, 97, 110, 116, 32, 45, 32, 104, 116, 116, 112, 58, 47, 47, 119, 119, 119, 46, 119, 105,
    110, 105, 109, 97, 103, 101, 46, 99, 111, 109, 47, 122, 76, 105, 98, 68, 108, 108, 0,
];
/* relative offset of local header 4 bytes */
/* ===========================================================================
     Read a byte from a gz_stream; update next_in and avail_in. Return EOF
   for end of file.
   IN assertion: the stream s has been successfully opened for reading.
*/

unsafe extern "C" fn unzlocal_getByte(
    mut pzlib_filefunc_def: *const crate::src::qcommon::ioapi::zlib_filefunc_def,
    mut filestream: crate::stdlib::voidpf,
    mut pi: *mut libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_uchar = 0;
    let mut err: libc::c_int = Some(
        (*pzlib_filefunc_def)
            .zread_file
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        (*pzlib_filefunc_def).opaque,
        filestream,
        &mut c as *mut libc::c_uchar as *mut libc::c_void,
        1 as libc::c_int as crate::stdlib::uLong,
    ) as libc::c_int;
    if err == 1 as libc::c_int {
        *pi = c as libc::c_int;
        return 0 as libc::c_int;
    } else if Some(
        (*pzlib_filefunc_def)
            .zerror_file
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")((*pzlib_filefunc_def).opaque, filestream)
        != 0
    {
        return -(1 as libc::c_int);
    } else {
        return 0 as libc::c_int;
    };
}
/* ===========================================================================
   Reads a long in LSB order from the given gz_stream. Sets
*/

unsafe extern "C" fn unzlocal_getShort(
    mut pzlib_filefunc_def: *const crate::src::qcommon::ioapi::zlib_filefunc_def,
    mut filestream: crate::stdlib::voidpf,
    mut pX: *mut crate::stdlib::uLong,
) -> libc::c_int {
    let mut x: crate::stdlib::uLong = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut err: libc::c_int = 0;
    err = unzlocal_getByte(pzlib_filefunc_def, filestream, &mut i);
    x = i as crate::stdlib::uLong;
    if err == 0 as libc::c_int {
        err = unzlocal_getByte(pzlib_filefunc_def, filestream, &mut i)
    }
    x = (x as libc::c_ulong).wrapping_add((i as crate::stdlib::uLong) << 8 as libc::c_int)
        as crate::stdlib::uLong as crate::stdlib::uLong;
    if err == 0 as libc::c_int {
        *pX = x
    } else {
        *pX = 0 as libc::c_int as crate::stdlib::uLong
    }
    return err;
}

unsafe extern "C" fn unzlocal_getLong(
    mut pzlib_filefunc_def: *const crate::src::qcommon::ioapi::zlib_filefunc_def,
    mut filestream: crate::stdlib::voidpf,
    mut pX: *mut crate::stdlib::uLong,
) -> libc::c_int {
    let mut x: crate::stdlib::uLong = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut err: libc::c_int = 0;
    err = unzlocal_getByte(pzlib_filefunc_def, filestream, &mut i);
    x = i as crate::stdlib::uLong;
    if err == 0 as libc::c_int {
        err = unzlocal_getByte(pzlib_filefunc_def, filestream, &mut i)
    }
    x = (x as libc::c_ulong).wrapping_add((i as crate::stdlib::uLong) << 8 as libc::c_int)
        as crate::stdlib::uLong as crate::stdlib::uLong;
    if err == 0 as libc::c_int {
        err = unzlocal_getByte(pzlib_filefunc_def, filestream, &mut i)
    }
    x = (x as libc::c_ulong).wrapping_add((i as crate::stdlib::uLong) << 16 as libc::c_int)
        as crate::stdlib::uLong as crate::stdlib::uLong;
    if err == 0 as libc::c_int {
        err = unzlocal_getByte(pzlib_filefunc_def, filestream, &mut i)
    }
    x = (x as libc::c_ulong).wrapping_add((i as crate::stdlib::uLong) << 24 as libc::c_int)
        as crate::stdlib::uLong as crate::stdlib::uLong;
    if err == 0 as libc::c_int {
        *pX = x
    } else {
        *pX = 0 as libc::c_int as crate::stdlib::uLong
    }
    return err;
}
/* My own strcmpi / strcasecmp */

unsafe extern "C" fn strcmpcasenosensitive_internal(
    mut fileName1: *const libc::c_char,
    mut fileName2: *const libc::c_char,
) -> libc::c_int {
    loop {
        let fresh0 = fileName1;
        fileName1 = fileName1.offset(1);
        let mut c1: libc::c_char = *fresh0;
        let fresh1 = fileName2;
        fileName2 = fileName2.offset(1);
        let mut c2: libc::c_char = *fresh1;
        if c1 as libc::c_int >= 'a' as i32 && c1 as libc::c_int <= 'z' as i32 {
            c1 = (c1 as libc::c_int - 0x20 as libc::c_int) as libc::c_char
        }
        if c2 as libc::c_int >= 'a' as i32 && c2 as libc::c_int <= 'z' as i32 {
            c2 = (c2 as libc::c_int - 0x20 as libc::c_int) as libc::c_char
        }
        if c1 as libc::c_int == '\u{0}' as i32 {
            return if c2 as libc::c_int == '\u{0}' as i32 {
                0 as libc::c_int
            } else {
                -(1 as libc::c_int)
            };
        }
        if c2 as libc::c_int == '\u{0}' as i32 {
            return 1 as libc::c_int;
        }
        if (c1 as libc::c_int) < c2 as libc::c_int {
            return -(1 as libc::c_int);
        }
        if c1 as libc::c_int > c2 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
}
/*
   Compare two filename (fileName1,fileName2).
   If iCaseSenisivity = 1, comparison is case sensitivity (like strcmp)
   If iCaseSenisivity = 2, comparison is not case sensitivity (like strcmpi
                                                                or strcasecmp)
   If iCaseSenisivity = 0, case sensitivity is defaut of your operating system
        (like 1 on Unix, 2 on Windows)

*/
#[no_mangle]

pub unsafe extern "C" fn unzStringFileNameCompare(
    mut fileName1: *const libc::c_char,
    mut fileName2: *const libc::c_char,
    mut iCaseSensitivity: libc::c_int,
) -> libc::c_int {
    if iCaseSensitivity == 0 as libc::c_int {
        iCaseSensitivity = 1 as libc::c_int
    }
    if iCaseSensitivity == 1 as libc::c_int {
        return crate::stdlib::strcmp(fileName1, fileName2);
    }
    return strcmpcasenosensitive_internal(fileName1, fileName2);
}
/*
  Locate the Central directory of a zipfile (at the end, just before
    the global comment)
*/

unsafe extern "C" fn unzlocal_SearchCentralDir(
    mut pzlib_filefunc_def: *const crate::src::qcommon::ioapi::zlib_filefunc_def,
    mut filestream: crate::stdlib::voidpf,
) -> crate::stdlib::uLong {
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar; /* maximum size of global comment */
    let mut uSizeFile: crate::stdlib::uLong = 0;
    let mut uBackRead: crate::stdlib::uLong = 0;
    let mut uMaxBack: crate::stdlib::uLong = 0xffff as libc::c_int as crate::stdlib::uLong;
    let mut uPosFound: crate::stdlib::uLong = 0 as libc::c_int as crate::stdlib::uLong;
    if Some(
        (*pzlib_filefunc_def)
            .zseek_file
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        (*pzlib_filefunc_def).opaque,
        filestream,
        0 as libc::c_int as crate::stdlib::uLong,
        2 as libc::c_int,
    ) != 0 as libc::c_int as libc::c_long
    {
        return 0 as libc::c_int as crate::stdlib::uLong;
    }
    uSizeFile = Some(
        (*pzlib_filefunc_def)
            .ztell_file
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")((*pzlib_filefunc_def).opaque, filestream)
        as crate::stdlib::uLong;
    if uMaxBack > uSizeFile {
        uMaxBack = uSizeFile
    }
    buf = crate::src::qcommon::common::Z_MallocDebug(
        0x400 as libc::c_int + 4 as libc::c_int,
        b"(0x400)+4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/qcommon/unzip.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        345 as libc::c_int,
    ) as *mut libc::c_uchar;
    if buf.is_null() {
        return 0 as libc::c_int as crate::stdlib::uLong;
    }
    uBackRead = 4 as libc::c_int as crate::stdlib::uLong;
    while uBackRead < uMaxBack {
        let mut uReadSize: crate::stdlib::uLong = 0;
        let mut uReadPos: crate::stdlib::uLong = 0;
        let mut i: libc::c_int = 0;
        if uBackRead.wrapping_add(0x400 as libc::c_int as libc::c_ulong) > uMaxBack {
            uBackRead = uMaxBack
        } else {
            uBackRead = (uBackRead as libc::c_ulong)
                .wrapping_add(0x400 as libc::c_int as libc::c_ulong)
                as crate::stdlib::uLong as crate::stdlib::uLong
        }
        uReadPos = uSizeFile.wrapping_sub(uBackRead);
        uReadSize = if ((0x400 as libc::c_int + 4 as libc::c_int) as libc::c_ulong)
            < uSizeFile.wrapping_sub(uReadPos)
        {
            (0x400 as libc::c_int + 4 as libc::c_int) as libc::c_ulong
        } else {
            uSizeFile.wrapping_sub(uReadPos)
        };
        if Some(
            (*pzlib_filefunc_def)
                .zseek_file
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            (*pzlib_filefunc_def).opaque,
            filestream,
            uReadPos,
            0 as libc::c_int,
        ) != 0 as libc::c_int as libc::c_long
        {
            break;
        }
        if Some(
            (*pzlib_filefunc_def)
                .zread_file
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            (*pzlib_filefunc_def).opaque,
            filestream,
            buf as *mut libc::c_void,
            uReadSize,
        ) != uReadSize
        {
            break;
        }
        i = uReadSize as libc::c_int - 3 as libc::c_int;
        loop {
            let fresh2 = i;
            i = i - 1;
            if !(fresh2 > 0 as libc::c_int) {
                break;
            }
            if !(*buf.offset(i as isize) as libc::c_int == 0x50 as libc::c_int
                && *buf.offset(i as isize).offset(1 as libc::c_int as isize) as libc::c_int
                    == 0x4b as libc::c_int
                && *buf.offset(i as isize).offset(2 as libc::c_int as isize) as libc::c_int
                    == 0x5 as libc::c_int
                && *buf.offset(i as isize).offset(3 as libc::c_int as isize) as libc::c_int
                    == 0x6 as libc::c_int)
            {
                continue;
            }
            uPosFound = uReadPos.wrapping_add(i as libc::c_ulong);
            break;
        }
        if uPosFound != 0 as libc::c_int as libc::c_ulong {
            break;
        }
    }
    if !buf.is_null() {
        crate::src::qcommon::common::Z_Free(buf as *mut libc::c_void);
    }
    return uPosFound;
}
/*
  Open a Zip file. path contain the full pathname (by example,
     on a Windows NT computer "c:\\test\\zlib114.zip" or on a Unix computer
     "zlib/zlib114.zip".
     If the zipfile cannot be opened (file doesn't exist or in not valid), the
       return value is NULL.
     Else, the return value is an unzFile Handle, usable with other function
       of this unzip package.
*/
#[no_mangle]

pub unsafe extern "C" fn unzOpen2(
    mut path: *const libc::c_char,
    mut pzlib_filefunc_def: *mut crate::src::qcommon::ioapi::zlib_filefunc_def,
) -> crate::src::qcommon::unzip::unzFile {
    let mut us: unz_s = unz_s {
        z_filefunc: crate::src::qcommon::ioapi::zlib_filefunc_def {
            zopen_file: None,
            zread_file: None,
            zwrite_file: None,
            ztell_file: None,
            zseek_file: None,
            zclose_file: None,
            zerror_file: None,
            opaque: 0 as *mut libc::c_void,
        },
        filestream: 0 as *mut libc::c_void,
        gi: crate::src::qcommon::unzip::unz_global_info {
            number_entry: 0,
            size_comment: 0,
        },
        byte_before_the_zipfile: 0,
        num_file: 0,
        pos_in_central_dir: 0,
        current_file_ok: 0,
        central_pos: 0,
        size_central_dir: 0,
        offset_central_dir: 0,
        cur_file_info: crate::src::qcommon::unzip::unz_file_info {
            version: 0,
            version_needed: 0,
            flag: 0,
            compression_method: 0,
            dosDate: 0,
            crc: 0,
            compressed_size: 0,
            uncompressed_size: 0,
            size_filename: 0,
            size_file_extra: 0,
            size_file_comment: 0,
            disk_num_start: 0,
            internal_fa: 0,
            external_fa: 0,
            tmu_date: crate::src::qcommon::unzip::tm_unz {
                tm_sec: 0,
                tm_min: 0,
                tm_hour: 0,
                tm_mday: 0,
                tm_mon: 0,
                tm_year: 0,
            },
        },
        cur_file_info_internal: unz_file_info_internal { offset_curfile: 0 },
        pfile_in_zip_read: 0 as *mut file_in_zip_read_info_s,
        encrypted: 0,
    }; /* number of the current dist, used for
       spaning ZIP, unsupported, always 0*/
    let mut s: *mut unz_s = 0 as *mut unz_s; /* number the the disk with central dir, used
                                             for spaning ZIP, unsupported, always 0*/
    let mut central_pos: crate::stdlib::uLong = 0; /* total number of entries in
                                                   the central dir
                                                   (same than number_entry on nospan) */
    let mut uL: crate::stdlib::uLong = 0;
    let mut number_disk: crate::stdlib::uLong = 0;
    let mut number_disk_with_CD: crate::stdlib::uLong = 0;
    let mut number_entry_CD: crate::stdlib::uLong = 0;
    let mut err: libc::c_int = 0 as libc::c_int;
    if unz_copyright[0 as libc::c_int as usize] as libc::c_int != ' ' as i32 {
        return 0 as *mut libc::c_void;
    }
    if pzlib_filefunc_def.is_null() {
        crate::src::qcommon::ioapi::fill_fopen_filefunc(&mut us.z_filefunc);
    } else {
        us.z_filefunc = *pzlib_filefunc_def
    }
    us.filestream = Some(us.z_filefunc.zopen_file.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        us.z_filefunc.opaque,
        path,
        1 as libc::c_int | 4 as libc::c_int,
    );
    if us.filestream.is_null() {
        return 0 as *mut libc::c_void;
    }
    central_pos = unzlocal_SearchCentralDir(&mut us.z_filefunc, us.filestream);
    if central_pos == 0 as libc::c_int as libc::c_ulong {
        err = -(1 as libc::c_int)
    }
    if Some(us.z_filefunc.zseek_file.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        us.z_filefunc.opaque,
        us.filestream,
        central_pos,
        0 as libc::c_int,
    ) != 0 as libc::c_int as libc::c_long
    {
        err = -(1 as libc::c_int)
    }
    /* the signature, already checked */
    if unzlocal_getLong(&mut us.z_filefunc, us.filestream, &mut uL) != 0 as libc::c_int {
        err = -(1 as libc::c_int)
    }
    /* number of this disk */
    if unzlocal_getShort(&mut us.z_filefunc, us.filestream, &mut number_disk) != 0 as libc::c_int {
        err = -(1 as libc::c_int)
    }
    /* number of the disk with the start of the central directory */
    if unzlocal_getShort(&mut us.z_filefunc, us.filestream, &mut number_disk_with_CD)
        != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    }
    /* total number of entries in the central dir on this disk */
    if unzlocal_getShort(&mut us.z_filefunc, us.filestream, &mut us.gi.number_entry)
        != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    }
    /* total number of entries in the central dir */
    if unzlocal_getShort(&mut us.z_filefunc, us.filestream, &mut number_entry_CD)
        != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    }
    if number_entry_CD != us.gi.number_entry
        || number_disk_with_CD != 0 as libc::c_int as libc::c_ulong
        || number_disk != 0 as libc::c_int as libc::c_ulong
    {
        err = -(103 as libc::c_int)
    }
    /* size of the central directory */
    if unzlocal_getLong(&mut us.z_filefunc, us.filestream, &mut us.size_central_dir)
        != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    }
    /* offset of start of central directory with respect to the
    starting disk number */
    if unzlocal_getLong(
        &mut us.z_filefunc,
        us.filestream,
        &mut us.offset_central_dir,
    ) != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    }
    /* zipfile comment length */
    if unzlocal_getShort(&mut us.z_filefunc, us.filestream, &mut us.gi.size_comment)
        != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    }
    if central_pos < us.offset_central_dir.wrapping_add(us.size_central_dir)
        && err == 0 as libc::c_int
    {
        err = -(103 as libc::c_int)
    }
    if err != 0 as libc::c_int {
        Some(
            us.z_filefunc
                .zclose_file
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(us.z_filefunc.opaque, us.filestream);
        return 0 as *mut libc::c_void;
    }
    us.byte_before_the_zipfile =
        central_pos.wrapping_sub(us.offset_central_dir.wrapping_add(us.size_central_dir));
    us.central_pos = central_pos;
    us.pfile_in_zip_read = 0 as *mut file_in_zip_read_info_s;
    us.encrypted = 0 as libc::c_int;
    s = crate::src::qcommon::common::Z_MallocDebug(
        ::std::mem::size_of::<unz_s>() as libc::c_ulong as libc::c_int,
        b"sizeof(unz_s)\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/qcommon/unzip.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        488 as libc::c_int,
    ) as *mut unz_s;
    *s = us;
    unzGoToFirstFile(s as crate::src::qcommon::unzip::unzFile);
    return s as crate::src::qcommon::unzip::unzFile;
}
#[no_mangle]

pub unsafe extern "C" fn unzOpen(
    mut path: *const libc::c_char,
) -> crate::src::qcommon::unzip::unzFile {
    return unzOpen2(
        path,
        0 as *mut crate::src::qcommon::ioapi::zlib_filefunc_def,
    );
}
/*
Close a ZipFile opened with unzipOpen.
If there is files inside the .Zip opened with unzipOpenCurrentFile (see later),
  these files MUST be closed with unzipCloseCurrentFile before call unzipClose.
return UNZ_OK if there is no problem. */
#[no_mangle]

pub unsafe extern "C" fn unzClose(mut file: crate::src::qcommon::unzip::unzFile) -> libc::c_int {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    if file.is_null() {
        return -(102 as libc::c_int);
    }
    s = file as *mut unz_s;
    if !(*s).pfile_in_zip_read.is_null() {
        unzCloseCurrentFile(file);
    }
    Some(
        (*s).z_filefunc
            .zclose_file
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")((*s).z_filefunc.opaque, (*s).filestream);
    if !s.is_null() {
        crate::src::qcommon::common::Z_Free(s as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
/*
Write info about the ZipFile in the *pglobal_info structure.
No preparation of the structure is needed
return UNZ_OK if there is no problem. */
#[no_mangle]

pub unsafe extern "C" fn unzGetGlobalInfo(
    mut file: crate::src::qcommon::unzip::unzFile,
    mut pglobal_info: *mut crate::src::qcommon::unzip::unz_global_info,
) -> libc::c_int {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    if file.is_null() {
        return -(102 as libc::c_int);
    }
    s = file as *mut unz_s;
    *pglobal_info = (*s).gi;
    return 0 as libc::c_int;
}
/*
   Translate date/time from Dos format to tm_unz (readable more easilty)
*/

unsafe extern "C" fn unzlocal_DosDateToTmuDate(
    mut ulDosDate: crate::stdlib::uLong,
    mut ptm: *mut crate::src::qcommon::unzip::tm_unz,
) {
    let mut uDate: crate::stdlib::uLong = 0;
    uDate = ulDosDate >> 16 as libc::c_int;
    (*ptm).tm_mday = (uDate & 0x1f as libc::c_int as libc::c_ulong) as crate::stdlib::uInt;
    (*ptm).tm_mon = (uDate & 0x1e0 as libc::c_int as libc::c_ulong)
        .wrapping_div(0x20 as libc::c_int as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as crate::stdlib::uInt;
    (*ptm).tm_year = (uDate & 0xfe00 as libc::c_int as libc::c_ulong)
        .wrapping_div(0x200 as libc::c_int as libc::c_ulong)
        .wrapping_add(1980 as libc::c_int as libc::c_ulong)
        as crate::stdlib::uInt;
    (*ptm).tm_hour = (ulDosDate & 0xf800 as libc::c_int as libc::c_ulong)
        .wrapping_div(0x800 as libc::c_int as libc::c_ulong)
        as crate::stdlib::uInt;
    (*ptm).tm_min = (ulDosDate & 0x7e0 as libc::c_int as libc::c_ulong)
        .wrapping_div(0x20 as libc::c_int as libc::c_ulong)
        as crate::stdlib::uInt;
    (*ptm).tm_sec = (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul(ulDosDate & 0x1f as libc::c_int as libc::c_ulong)
        as crate::stdlib::uInt;
}
/*
  Get Info about the current file in the zipfile, with internal only info
*/

unsafe extern "C" fn unzlocal_GetCurrentFileInfoInternal(
    mut file: crate::src::qcommon::unzip::unzFile,
    mut pfile_info: *mut crate::src::qcommon::unzip::unz_file_info,
    mut pfile_info_internal: *mut unz_file_info_internal,
    mut szFileName: *mut libc::c_char,
    mut fileNameBufferSize: crate::stdlib::uLong,
    mut extraField: *mut libc::c_void,
    mut extraFieldBufferSize: crate::stdlib::uLong,
    mut szComment: *mut libc::c_char,
    mut commentBufferSize: crate::stdlib::uLong,
) -> libc::c_int {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut file_info: crate::src::qcommon::unzip::unz_file_info =
        crate::src::qcommon::unzip::unz_file_info {
            version: 0,
            version_needed: 0,
            flag: 0,
            compression_method: 0,
            dosDate: 0,
            crc: 0,
            compressed_size: 0,
            uncompressed_size: 0,
            size_filename: 0,
            size_file_extra: 0,
            size_file_comment: 0,
            disk_num_start: 0,
            internal_fa: 0,
            external_fa: 0,
            tmu_date: crate::src::qcommon::unzip::tm_unz {
                tm_sec: 0,
                tm_min: 0,
                tm_hour: 0,
                tm_mday: 0,
                tm_mon: 0,
                tm_year: 0,
            },
        };
    let mut file_info_internal: unz_file_info_internal =
        unz_file_info_internal { offset_curfile: 0 };
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut uMagic: crate::stdlib::uLong = 0;
    let mut lSeek: libc::c_long = 0 as libc::c_int as libc::c_long;
    if file.is_null() {
        return -(102 as libc::c_int);
    }
    s = file as *mut unz_s;
    if Some(
        (*s).z_filefunc
            .zseek_file
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        (*s).z_filefunc.opaque,
        (*s).filestream,
        (*s).pos_in_central_dir
            .wrapping_add((*s).byte_before_the_zipfile),
        0 as libc::c_int,
    ) != 0 as libc::c_int as libc::c_long
    {
        err = -(1 as libc::c_int)
    }
    /* we check the magic */
    if err == 0 as libc::c_int {
        if unzlocal_getLong(&mut (*s).z_filefunc, (*s).filestream, &mut uMagic) != 0 as libc::c_int
        {
            err = -(1 as libc::c_int)
        } else if uMagic != 0x2014b50 as libc::c_int as libc::c_ulong {
            err = -(103 as libc::c_int)
        }
    }
    if unzlocal_getShort(
        &mut (*s).z_filefunc,
        (*s).filestream,
        &mut file_info.version,
    ) != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    }
    if unzlocal_getShort(
        &mut (*s).z_filefunc,
        (*s).filestream,
        &mut file_info.version_needed,
    ) != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    }
    if unzlocal_getShort(&mut (*s).z_filefunc, (*s).filestream, &mut file_info.flag)
        != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    }
    if unzlocal_getShort(
        &mut (*s).z_filefunc,
        (*s).filestream,
        &mut file_info.compression_method,
    ) != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    }
    if unzlocal_getLong(
        &mut (*s).z_filefunc,
        (*s).filestream,
        &mut file_info.dosDate,
    ) != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    }
    unzlocal_DosDateToTmuDate(file_info.dosDate, &mut file_info.tmu_date);
    if unzlocal_getLong(&mut (*s).z_filefunc, (*s).filestream, &mut file_info.crc)
        != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    }
    if unzlocal_getLong(
        &mut (*s).z_filefunc,
        (*s).filestream,
        &mut file_info.compressed_size,
    ) != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    }
    if unzlocal_getLong(
        &mut (*s).z_filefunc,
        (*s).filestream,
        &mut file_info.uncompressed_size,
    ) != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    }
    if unzlocal_getShort(
        &mut (*s).z_filefunc,
        (*s).filestream,
        &mut file_info.size_filename,
    ) != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    }
    if unzlocal_getShort(
        &mut (*s).z_filefunc,
        (*s).filestream,
        &mut file_info.size_file_extra,
    ) != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    }
    if unzlocal_getShort(
        &mut (*s).z_filefunc,
        (*s).filestream,
        &mut file_info.size_file_comment,
    ) != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    }
    if unzlocal_getShort(
        &mut (*s).z_filefunc,
        (*s).filestream,
        &mut file_info.disk_num_start,
    ) != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    }
    if unzlocal_getShort(
        &mut (*s).z_filefunc,
        (*s).filestream,
        &mut file_info.internal_fa,
    ) != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    }
    if unzlocal_getLong(
        &mut (*s).z_filefunc,
        (*s).filestream,
        &mut file_info.external_fa,
    ) != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    }
    if unzlocal_getLong(
        &mut (*s).z_filefunc,
        (*s).filestream,
        &mut file_info_internal.offset_curfile,
    ) != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    }
    lSeek = (lSeek as libc::c_ulong).wrapping_add(file_info.size_filename) as libc::c_long
        as libc::c_long;
    if err == 0 as libc::c_int && !szFileName.is_null() {
        let mut uSizeRead: crate::stdlib::uLong = 0;
        if file_info.size_filename < fileNameBufferSize {
            *szFileName.offset(file_info.size_filename as isize) = '\u{0}' as i32 as libc::c_char;
            uSizeRead = file_info.size_filename
        } else {
            uSizeRead = fileNameBufferSize
        }
        if file_info.size_filename > 0 as libc::c_int as libc::c_ulong
            && fileNameBufferSize > 0 as libc::c_int as libc::c_ulong
        {
            if Some(
                (*s).z_filefunc
                    .zread_file
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                (*s).z_filefunc.opaque,
                (*s).filestream,
                szFileName as *mut libc::c_void,
                uSizeRead,
            ) != uSizeRead
            {
                err = -(1 as libc::c_int)
            }
        }
        lSeek = (lSeek as libc::c_ulong).wrapping_sub(uSizeRead) as libc::c_long as libc::c_long
    }
    if err == 0 as libc::c_int && !extraField.is_null() {
        let mut uSizeRead_0: crate::stdlib::uLong = 0;
        if file_info.size_file_extra < extraFieldBufferSize {
            uSizeRead_0 = file_info.size_file_extra
        } else {
            uSizeRead_0 = extraFieldBufferSize
        }
        if lSeek != 0 as libc::c_int as libc::c_long {
            if Some(
                (*s).z_filefunc
                    .zseek_file
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                (*s).z_filefunc.opaque,
                (*s).filestream,
                lSeek as crate::stdlib::uLong,
                1 as libc::c_int,
            ) == 0 as libc::c_int as libc::c_long
            {
                lSeek = 0 as libc::c_int as libc::c_long
            } else {
                err = -(1 as libc::c_int)
            }
        }
        if file_info.size_file_extra > 0 as libc::c_int as libc::c_ulong
            && extraFieldBufferSize > 0 as libc::c_int as libc::c_ulong
        {
            if Some(
                (*s).z_filefunc
                    .zread_file
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                (*s).z_filefunc.opaque,
                (*s).filestream,
                extraField,
                uSizeRead_0,
            ) != uSizeRead_0
            {
                err = -(1 as libc::c_int)
            }
        }
        lSeek = (lSeek as libc::c_ulong)
            .wrapping_add(file_info.size_file_extra.wrapping_sub(uSizeRead_0))
            as libc::c_long as libc::c_long
    } else {
        lSeek = (lSeek as libc::c_ulong).wrapping_add(file_info.size_file_extra) as libc::c_long
            as libc::c_long
    }
    if err == 0 as libc::c_int && !szComment.is_null() {
        let mut uSizeRead_1: crate::stdlib::uLong = 0;
        if file_info.size_file_comment < commentBufferSize {
            *szComment.offset(file_info.size_file_comment as isize) =
                '\u{0}' as i32 as libc::c_char;
            uSizeRead_1 = file_info.size_file_comment
        } else {
            uSizeRead_1 = commentBufferSize
        }
        if lSeek != 0 as libc::c_int as libc::c_long {
            if Some(
                (*s).z_filefunc
                    .zseek_file
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                (*s).z_filefunc.opaque,
                (*s).filestream,
                lSeek as crate::stdlib::uLong,
                1 as libc::c_int,
            ) != 0 as libc::c_int as libc::c_long
            {
                err = -(1 as libc::c_int)
            }
        }
        if file_info.size_file_comment > 0 as libc::c_int as libc::c_ulong
            && commentBufferSize > 0 as libc::c_int as libc::c_ulong
        {
            if Some(
                (*s).z_filefunc
                    .zread_file
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                (*s).z_filefunc.opaque,
                (*s).filestream,
                szComment as *mut libc::c_void,
                uSizeRead_1,
            ) != uSizeRead_1
            {
                err = -(1 as libc::c_int)
            }
        }
    }
    if err == 0 as libc::c_int && !pfile_info.is_null() {
        *pfile_info = file_info
    }
    if err == 0 as libc::c_int && !pfile_info_internal.is_null() {
        *pfile_info_internal = file_info_internal
    }
    return err;
}
/*
  Write info about the ZipFile in the *pglobal_info structure.
  No preparation of the structure is needed
  return UNZ_OK if there is no problem.
*/
#[no_mangle]

pub unsafe extern "C" fn unzGetCurrentFileInfo(
    mut file: crate::src::qcommon::unzip::unzFile,
    mut pfile_info: *mut crate::src::qcommon::unzip::unz_file_info,
    mut szFileName: *mut libc::c_char,
    mut fileNameBufferSize: crate::stdlib::uLong,
    mut extraField: *mut libc::c_void,
    mut extraFieldBufferSize: crate::stdlib::uLong,
    mut szComment: *mut libc::c_char,
    mut commentBufferSize: crate::stdlib::uLong,
) -> libc::c_int {
    return unzlocal_GetCurrentFileInfoInternal(
        file,
        pfile_info,
        0 as *mut unz_file_info_internal,
        szFileName,
        fileNameBufferSize,
        extraField,
        extraFieldBufferSize,
        szComment,
        commentBufferSize,
    );
}
/*
  Set the current file of the zipfile to the first file.
  return UNZ_OK if there is no problem
*/
#[no_mangle]

pub unsafe extern "C" fn unzGoToFirstFile(
    mut file: crate::src::qcommon::unzip::unzFile,
) -> libc::c_int {
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut s: *mut unz_s = 0 as *mut unz_s;
    if file.is_null() {
        return -(102 as libc::c_int);
    }
    s = file as *mut unz_s;
    (*s).pos_in_central_dir = (*s).offset_central_dir;
    (*s).num_file = 0 as libc::c_int as crate::stdlib::uLong;
    err = unzlocal_GetCurrentFileInfoInternal(
        file,
        &mut (*s).cur_file_info,
        &mut (*s).cur_file_info_internal,
        0 as *mut libc::c_char,
        0 as libc::c_int as crate::stdlib::uLong,
        0 as *mut libc::c_void,
        0 as libc::c_int as crate::stdlib::uLong,
        0 as *mut libc::c_char,
        0 as libc::c_int as crate::stdlib::uLong,
    );
    (*s).current_file_ok = (err == 0 as libc::c_int) as libc::c_int as crate::stdlib::uLong;
    return err;
}
/*
  Set the current file of the zipfile to the next file.
  return UNZ_OK if there is no problem
  return UNZ_END_OF_LIST_OF_FILE if the actual file was the latest.
*/
#[no_mangle]

pub unsafe extern "C" fn unzGoToNextFile(
    mut file: crate::src::qcommon::unzip::unzFile,
) -> libc::c_int {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut err: libc::c_int = 0;
    if file.is_null() {
        return -(102 as libc::c_int);
    }
    s = file as *mut unz_s;
    if (*s).current_file_ok == 0 {
        return -(100 as libc::c_int);
    }
    if (*s).gi.number_entry != 0xffff as libc::c_int as libc::c_ulong {
        /* 2^16 files overflow hack */
        if (*s)
            .num_file
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            == (*s).gi.number_entry
        {
            return -(100 as libc::c_int);
        }
    }
    (*s).pos_in_central_dir = ((*s).pos_in_central_dir as libc::c_ulong).wrapping_add(
        (0x2e as libc::c_int as libc::c_ulong)
            .wrapping_add((*s).cur_file_info.size_filename)
            .wrapping_add((*s).cur_file_info.size_file_extra)
            .wrapping_add((*s).cur_file_info.size_file_comment),
    ) as crate::stdlib::uLong as crate::stdlib::uLong;
    (*s).num_file = (*s).num_file.wrapping_add(1);
    err = unzlocal_GetCurrentFileInfoInternal(
        file,
        &mut (*s).cur_file_info,
        &mut (*s).cur_file_info_internal,
        0 as *mut libc::c_char,
        0 as libc::c_int as crate::stdlib::uLong,
        0 as *mut libc::c_void,
        0 as libc::c_int as crate::stdlib::uLong,
        0 as *mut libc::c_char,
        0 as libc::c_int as crate::stdlib::uLong,
    );
    (*s).current_file_ok = (err == 0 as libc::c_int) as libc::c_int as crate::stdlib::uLong;
    return err;
}
/*
  Try locate the file szFileName in the zipfile.
  For the iCaseSensitivity signification, see unzipStringFileNameCompare

  return value :
  UNZ_OK if the file is found. It becomes the current file.
  UNZ_END_OF_LIST_OF_FILE if the file is not found
*/
#[no_mangle]

pub unsafe extern "C" fn unzLocateFile(
    mut file: crate::src::qcommon::unzip::unzFile,
    mut szFileName: *const libc::c_char,
    mut iCaseSensitivity: libc::c_int,
) -> libc::c_int {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut err: libc::c_int = 0;
    /* We remember the 'current' position in the file so that we can jump
     * back there if we fail.
     */
    let mut cur_file_infoSaved: crate::src::qcommon::unzip::unz_file_info =
        crate::src::qcommon::unzip::unz_file_info {
            version: 0,
            version_needed: 0,
            flag: 0,
            compression_method: 0,
            dosDate: 0,
            crc: 0,
            compressed_size: 0,
            uncompressed_size: 0,
            size_filename: 0,
            size_file_extra: 0,
            size_file_comment: 0,
            disk_num_start: 0,
            internal_fa: 0,
            external_fa: 0,
            tmu_date: crate::src::qcommon::unzip::tm_unz {
                tm_sec: 0,
                tm_min: 0,
                tm_hour: 0,
                tm_mday: 0,
                tm_mon: 0,
                tm_year: 0,
            },
        };
    let mut cur_file_info_internalSaved: unz_file_info_internal =
        unz_file_info_internal { offset_curfile: 0 };
    let mut num_fileSaved: crate::stdlib::uLong = 0;
    let mut pos_in_central_dirSaved: crate::stdlib::uLong = 0;
    if file.is_null() {
        return -(102 as libc::c_int);
    }
    if crate::stdlib::strlen(szFileName) >= 256 as libc::c_int as libc::c_ulong {
        return -(102 as libc::c_int);
    }
    s = file as *mut unz_s;
    if (*s).current_file_ok == 0 {
        return -(100 as libc::c_int);
    }
    /* Save the current state */
    num_fileSaved = (*s).num_file;
    pos_in_central_dirSaved = (*s).pos_in_central_dir;
    cur_file_infoSaved = (*s).cur_file_info;
    cur_file_info_internalSaved = (*s).cur_file_info_internal;
    err = unzGoToFirstFile(file);
    while err == 0 as libc::c_int {
        let mut szCurrentFileName: [libc::c_char; 257] = [0; 257];
        err = unzGetCurrentFileInfo(
            file,
            0 as *mut crate::src::qcommon::unzip::unz_file_info,
            szCurrentFileName.as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_char; 257]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            0 as *mut libc::c_void,
            0 as libc::c_int as crate::stdlib::uLong,
            0 as *mut libc::c_char,
            0 as libc::c_int as crate::stdlib::uLong,
        );
        if err == 0 as libc::c_int {
            if unzStringFileNameCompare(
                szCurrentFileName.as_mut_ptr(),
                szFileName,
                iCaseSensitivity,
            ) == 0 as libc::c_int
            {
                return 0 as libc::c_int;
            }
            err = unzGoToNextFile(file)
        }
    }
    /* We failed, so restore the state of the 'current file' to where we
     * were.
     */
    (*s).num_file = num_fileSaved;
    (*s).pos_in_central_dir = pos_in_central_dirSaved;
    (*s).cur_file_info = cur_file_infoSaved;
    (*s).cur_file_info_internal = cur_file_info_internalSaved;
    return err;
}
/*
// /////////////////////////////////////////
// Contributed by Ryan Haksi (mailto://cryogen@infoserve.net)
// I need random access
//
// Further optimization could be realized by adding an ability
// to cache the directory in memory. The goal being a single
// comprehensive file read to put the file I need in a memory.
*/
/*
typedef struct unz_file_pos_s
{
    uLong pos_in_zip_directory;   // offset in file
    uLong num_of_file;            // # of file
} unz_file_pos;
*/
#[no_mangle]

pub unsafe extern "C" fn unzGetFilePos(
    mut file: crate::src::qcommon::unzip::unzFile,
    mut file_pos: *mut crate::src::qcommon::unzip::unz_file_pos,
) -> libc::c_int {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    if file.is_null() || file_pos.is_null() {
        return -(102 as libc::c_int);
    }
    s = file as *mut unz_s;
    if (*s).current_file_ok == 0 {
        return -(100 as libc::c_int);
    }
    (*file_pos).pos_in_zip_directory = (*s).pos_in_central_dir;
    (*file_pos).num_of_file = (*s).num_file;
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn unzGoToFilePos(
    mut file: crate::src::qcommon::unzip::unzFile,
    mut file_pos: *mut crate::src::qcommon::unzip::unz_file_pos,
) -> libc::c_int {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut err: libc::c_int = 0;
    if file.is_null() || file_pos.is_null() {
        return -(102 as libc::c_int);
    }
    s = file as *mut unz_s;
    /* jump to the right spot */
    (*s).pos_in_central_dir = (*file_pos).pos_in_zip_directory;
    (*s).num_file = (*file_pos).num_of_file;
    /* set the current file */
    err = unzlocal_GetCurrentFileInfoInternal(
        file,
        &mut (*s).cur_file_info,
        &mut (*s).cur_file_info_internal,
        0 as *mut libc::c_char,
        0 as libc::c_int as crate::stdlib::uLong,
        0 as *mut libc::c_void,
        0 as libc::c_int as crate::stdlib::uLong,
        0 as *mut libc::c_char,
        0 as libc::c_int as crate::stdlib::uLong,
    );
    /* return results */
    (*s).current_file_ok = (err == 0 as libc::c_int) as libc::c_int as crate::stdlib::uLong;
    return err;
}
/*
// Unzip Helper Functions - should be here?
// /////////////////////////////////////////
*/
/*
  Read the local header of the current zipfile
  Check the coherency of the local header and info in the end of central
        directory about this file
  store in *piSizeVar the size of extra info in local header
        (filename and size of extra field data)
*/

unsafe extern "C" fn unzlocal_CheckCurrentFileCoherencyHeader(
    mut s: *mut unz_s,
    mut piSizeVar: *mut crate::stdlib::uInt,
    mut poffset_local_extrafield: *mut crate::stdlib::uLong,
    mut psize_local_extrafield: *mut crate::stdlib::uInt,
) -> libc::c_int {
    let mut uMagic: crate::stdlib::uLong = 0;
    let mut uData: crate::stdlib::uLong = 0;
    let mut uFlags: crate::stdlib::uLong = 0;
    let mut size_filename: crate::stdlib::uLong = 0;
    let mut size_extra_field: crate::stdlib::uLong = 0;
    let mut err: libc::c_int = 0 as libc::c_int;
    *piSizeVar = 0 as libc::c_int as crate::stdlib::uInt;
    *poffset_local_extrafield = 0 as libc::c_int as crate::stdlib::uLong;
    *psize_local_extrafield = 0 as libc::c_int as crate::stdlib::uInt;
    if Some(
        (*s).z_filefunc
            .zseek_file
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        (*s).z_filefunc.opaque,
        (*s).filestream,
        (*s).cur_file_info_internal
            .offset_curfile
            .wrapping_add((*s).byte_before_the_zipfile),
        0 as libc::c_int,
    ) != 0 as libc::c_int as libc::c_long
    {
        return -(1 as libc::c_int);
    }
    if err == 0 as libc::c_int {
        if unzlocal_getLong(&mut (*s).z_filefunc, (*s).filestream, &mut uMagic) != 0 as libc::c_int
        {
            err = -(1 as libc::c_int)
        } else if uMagic != 0x4034b50 as libc::c_int as libc::c_ulong {
            err = -(103 as libc::c_int)
        }
    }
    if unzlocal_getShort(&mut (*s).z_filefunc, (*s).filestream, &mut uData) != 0 as libc::c_int {
        err = -(1 as libc::c_int)
    }
    /*
        else if ((err==UNZ_OK) && (uData!=s->cur_file_info.wVersion))
            err=UNZ_BADZIPFILE;
    */
    if unzlocal_getShort(&mut (*s).z_filefunc, (*s).filestream, &mut uFlags) != 0 as libc::c_int {
        err = -(1 as libc::c_int)
    }
    if unzlocal_getShort(&mut (*s).z_filefunc, (*s).filestream, &mut uData) != 0 as libc::c_int {
        err = -(1 as libc::c_int)
    } else if err == 0 as libc::c_int && uData != (*s).cur_file_info.compression_method {
        err = -(103 as libc::c_int)
    }
    if err == 0 as libc::c_int
        && (*s).cur_file_info.compression_method != 0 as libc::c_int as libc::c_ulong
        && (*s).cur_file_info.compression_method != 8 as libc::c_int as libc::c_ulong
    {
        err = -(103 as libc::c_int)
    }
    if unzlocal_getLong(&mut (*s).z_filefunc, (*s).filestream, &mut uData) != 0 as libc::c_int {
        /* date/time */
        err = -(1 as libc::c_int)
    }
    if unzlocal_getLong(&mut (*s).z_filefunc, (*s).filestream, &mut uData) != 0 as libc::c_int {
        /* crc */
        err = -(1 as libc::c_int)
    } else if err == 0 as libc::c_int
        && uData != (*s).cur_file_info.crc
        && uFlags & 8 as libc::c_int as libc::c_ulong == 0 as libc::c_int as libc::c_ulong
    {
        err = -(103 as libc::c_int)
    }
    if unzlocal_getLong(&mut (*s).z_filefunc, (*s).filestream, &mut uData) != 0 as libc::c_int {
        /* size compr */
        err = -(1 as libc::c_int)
    } else if err == 0 as libc::c_int
        && uData != (*s).cur_file_info.compressed_size
        && uFlags & 8 as libc::c_int as libc::c_ulong == 0 as libc::c_int as libc::c_ulong
    {
        err = -(103 as libc::c_int)
    }
    if unzlocal_getLong(&mut (*s).z_filefunc, (*s).filestream, &mut uData) != 0 as libc::c_int {
        /* size uncompr */
        err = -(1 as libc::c_int)
    } else if err == 0 as libc::c_int
        && uData != (*s).cur_file_info.uncompressed_size
        && uFlags & 8 as libc::c_int as libc::c_ulong == 0 as libc::c_int as libc::c_ulong
    {
        err = -(103 as libc::c_int)
    }
    if unzlocal_getShort(&mut (*s).z_filefunc, (*s).filestream, &mut size_filename)
        != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    } else if err == 0 as libc::c_int && size_filename != (*s).cur_file_info.size_filename {
        err = -(103 as libc::c_int)
    }
    *piSizeVar = (*piSizeVar as libc::c_uint).wrapping_add(size_filename as crate::stdlib::uInt)
        as crate::stdlib::uInt as crate::stdlib::uInt;
    if unzlocal_getShort(&mut (*s).z_filefunc, (*s).filestream, &mut size_extra_field)
        != 0 as libc::c_int
    {
        err = -(1 as libc::c_int)
    }
    *poffset_local_extrafield = (*s)
        .cur_file_info_internal
        .offset_curfile
        .wrapping_add(0x1e as libc::c_int as libc::c_ulong)
        .wrapping_add(size_filename);
    *psize_local_extrafield = size_extra_field as crate::stdlib::uInt;
    *piSizeVar = (*piSizeVar as libc::c_uint).wrapping_add(size_extra_field as crate::stdlib::uInt)
        as crate::stdlib::uInt as crate::stdlib::uInt;
    return err;
}
/*
  Open for reading data the current file in the zipfile.
  If there is no error and the file is opened, the return value is UNZ_OK.
*/
#[no_mangle]

pub unsafe extern "C" fn unzOpenCurrentFile3(
    mut file: crate::src::qcommon::unzip::unzFile,
    mut method: *mut libc::c_int,
    mut level: *mut libc::c_int,
    mut raw: libc::c_int,
    mut password: *const libc::c_char,
) -> libc::c_int {
    let mut err: libc::c_int = 0 as libc::c_int; /* offset of the local extra field */
    let mut iSizeVar: crate::stdlib::uInt = 0; /* size of the local extra field */
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut pfile_in_zip_read_info: *mut file_in_zip_read_info_s =
        0 as *mut file_in_zip_read_info_s;
    let mut offset_local_extrafield: crate::stdlib::uLong = 0;
    let mut size_local_extrafield: crate::stdlib::uInt = 0;
    if !password.is_null() {
        return -(102 as libc::c_int);
    }
    if file.is_null() {
        return -(102 as libc::c_int);
    }
    s = file as *mut unz_s;
    if (*s).current_file_ok == 0 {
        return -(102 as libc::c_int);
    }
    if !(*s).pfile_in_zip_read.is_null() {
        unzCloseCurrentFile(file);
    }
    if unzlocal_CheckCurrentFileCoherencyHeader(
        s,
        &mut iSizeVar,
        &mut offset_local_extrafield,
        &mut size_local_extrafield,
    ) != 0 as libc::c_int
    {
        return -(103 as libc::c_int);
    }
    pfile_in_zip_read_info = crate::src::qcommon::common::Z_MallocDebug(
        ::std::mem::size_of::<file_in_zip_read_info_s>() as libc::c_ulong as libc::c_int,
        b"sizeof(file_in_zip_read_info_s)\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/qcommon/unzip.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        1093 as libc::c_int,
    ) as *mut file_in_zip_read_info_s;
    if pfile_in_zip_read_info.is_null() {
        return -(104 as libc::c_int);
    }
    (*pfile_in_zip_read_info).read_buffer = crate::src::qcommon::common::Z_MallocDebug(
        16384 as libc::c_int,
        b"(16384)\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"/home/luka/Projects/ioq3-server/src/qcommon/unzip.c\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        1097 as libc::c_int,
    ) as *mut libc::c_char;
    (*pfile_in_zip_read_info).offset_local_extrafield = offset_local_extrafield;
    (*pfile_in_zip_read_info).size_local_extrafield = size_local_extrafield;
    (*pfile_in_zip_read_info).pos_local_extrafield = 0 as libc::c_int as crate::stdlib::uLong;
    (*pfile_in_zip_read_info).raw = raw;
    if (*pfile_in_zip_read_info).read_buffer.is_null() {
        if !pfile_in_zip_read_info.is_null() {
            crate::src::qcommon::common::Z_Free(pfile_in_zip_read_info as *mut libc::c_void);
        }
        return -(104 as libc::c_int);
    }
    (*pfile_in_zip_read_info).stream_initialised = 0 as libc::c_int as crate::stdlib::uLong;
    if !method.is_null() {
        *method = (*s).cur_file_info.compression_method as libc::c_int
    }
    if !level.is_null() {
        *level = 6 as libc::c_int;
        match (*s).cur_file_info.flag & 0x6 as libc::c_int as libc::c_ulong {
            6 => *level = 1 as libc::c_int,
            4 => *level = 2 as libc::c_int,
            2 => *level = 9 as libc::c_int,
            _ => {}
        }
    }
    if (*s).cur_file_info.compression_method != 0 as libc::c_int as libc::c_ulong
        && (*s).cur_file_info.compression_method != 8 as libc::c_int as libc::c_ulong
    {
        err = -(103 as libc::c_int)
    }
    (*pfile_in_zip_read_info).crc32_wait = (*s).cur_file_info.crc;
    (*pfile_in_zip_read_info).crc32 = 0 as libc::c_int as crate::stdlib::uLong;
    (*pfile_in_zip_read_info).compression_method = (*s).cur_file_info.compression_method;
    (*pfile_in_zip_read_info).filestream = (*s).filestream;
    (*pfile_in_zip_read_info).z_filefunc = (*s).z_filefunc;
    (*pfile_in_zip_read_info).byte_before_the_zipfile = (*s).byte_before_the_zipfile;
    (*pfile_in_zip_read_info).stream.total_out = 0 as libc::c_int as crate::stdlib::uLong;
    if (*s).cur_file_info.compression_method == 8 as libc::c_int as libc::c_ulong && raw == 0 {
        (*pfile_in_zip_read_info).stream.zalloc = None;
        (*pfile_in_zip_read_info).stream.zfree = None;
        (*pfile_in_zip_read_info).stream.opaque = 0 as crate::stdlib::voidpf;
        (*pfile_in_zip_read_info).stream.next_in = 0 as *mut crate::stdlib::Bytef;
        (*pfile_in_zip_read_info).stream.avail_in = 0 as libc::c_int as crate::stdlib::uInt;
        if crate::stdlib::inflateInit2_(
            &mut (*pfile_in_zip_read_info).stream,
            -(15 as libc::c_int),
            b"1.2.11\x00" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<crate::stdlib::z_stream>() as libc::c_ulong as libc::c_int,
        ) == 0 as libc::c_int
        {
            (*pfile_in_zip_read_info).stream_initialised = 1 as libc::c_int as crate::stdlib::uLong
        } else {
            if !pfile_in_zip_read_info.is_null() {
                crate::src::qcommon::common::Z_Free(pfile_in_zip_read_info as *mut libc::c_void);
            }
            return -(104 as libc::c_int);
        }
        /* windowBits is passed < 0 to tell that there is no zlib header.
         * Note that in this case inflate *requires* an extra "dummy" byte
         * after the compressed stream in order to complete decompression and
         * return Z_STREAM_END.
         * In unzip, i don't wait absolutely Z_STREAM_END because I known the
         * size of both compressed and uncompressed data
         */
    }
    (*pfile_in_zip_read_info).rest_read_compressed = (*s).cur_file_info.compressed_size;
    (*pfile_in_zip_read_info).rest_read_uncompressed = (*s).cur_file_info.uncompressed_size;
    (*pfile_in_zip_read_info).pos_in_zipfile = (*s)
        .cur_file_info_internal
        .offset_curfile
        .wrapping_add(0x1e as libc::c_int as libc::c_ulong)
        .wrapping_add(iSizeVar as libc::c_ulong);
    (*pfile_in_zip_read_info).stream.avail_in = 0 as libc::c_int as crate::stdlib::uInt;
    (*s).pfile_in_zip_read = pfile_in_zip_read_info;
    return err;
}
#[no_mangle]

pub unsafe extern "C" fn unzOpenCurrentFile(
    mut file: crate::src::qcommon::unzip::unzFile,
) -> libc::c_int {
    return unzOpenCurrentFile3(
        file,
        0 as *mut libc::c_int,
        0 as *mut libc::c_int,
        0 as libc::c_int,
        0 as *const libc::c_char,
    );
}
#[no_mangle]

pub unsafe extern "C" fn unzOpenCurrentFilePassword(
    mut file: crate::src::qcommon::unzip::unzFile,
    mut password: *const libc::c_char,
) -> libc::c_int {
    return unzOpenCurrentFile3(
        file,
        0 as *mut libc::c_int,
        0 as *mut libc::c_int,
        0 as libc::c_int,
        password,
    );
}
#[no_mangle]

pub unsafe extern "C" fn unzOpenCurrentFile2(
    mut file: crate::src::qcommon::unzip::unzFile,
    mut method: *mut libc::c_int,
    mut level: *mut libc::c_int,
    mut raw: libc::c_int,
) -> libc::c_int {
    return unzOpenCurrentFile3(file, method, level, raw, 0 as *const libc::c_char);
}
/*
  Read bytes from the current file.
  buf contain buffer where data must be copied
  len the size of buf.

  return the number of byte copied if somes bytes are copied
  return 0 if the end of file was reached
  return <0 with error code if there is an error
    (UNZ_ERRNO for IO error, or zLib error for uncompress error)
*/
#[no_mangle]

pub unsafe extern "C" fn unzReadCurrentFile(
    mut file: crate::src::qcommon::unzip::unzFile,
    mut buf: crate::stdlib::voidp,
    mut len: libc::c_uint,
) -> libc::c_int {
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut iRead: crate::stdlib::uInt = 0 as libc::c_int as crate::stdlib::uInt;
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut pfile_in_zip_read_info: *mut file_in_zip_read_info_s =
        0 as *mut file_in_zip_read_info_s;
    if file.is_null() {
        return -(102 as libc::c_int);
    }
    s = file as *mut unz_s;
    pfile_in_zip_read_info = (*s).pfile_in_zip_read;
    if pfile_in_zip_read_info.is_null() {
        return -(102 as libc::c_int);
    }
    if (*pfile_in_zip_read_info).read_buffer.is_null() {
        return -(100 as libc::c_int);
    }
    if len == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    (*pfile_in_zip_read_info).stream.next_out = buf as *mut crate::stdlib::Bytef;
    (*pfile_in_zip_read_info).stream.avail_out = len;
    if len as libc::c_ulong > (*pfile_in_zip_read_info).rest_read_uncompressed
        && (*pfile_in_zip_read_info).raw == 0
    {
        (*pfile_in_zip_read_info).stream.avail_out =
            (*pfile_in_zip_read_info).rest_read_uncompressed as crate::stdlib::uInt
    }
    if len as libc::c_ulong
        > (*pfile_in_zip_read_info)
            .rest_read_compressed
            .wrapping_add((*pfile_in_zip_read_info).stream.avail_in as libc::c_ulong)
        && (*pfile_in_zip_read_info).raw != 0
    {
        (*pfile_in_zip_read_info).stream.avail_out = ((*pfile_in_zip_read_info).rest_read_compressed
            as crate::stdlib::uInt)
            .wrapping_add((*pfile_in_zip_read_info).stream.avail_in)
    }
    while (*pfile_in_zip_read_info).stream.avail_out > 0 as libc::c_int as libc::c_uint {
        if (*pfile_in_zip_read_info).stream.avail_in == 0 as libc::c_int as libc::c_uint
            && (*pfile_in_zip_read_info).rest_read_compressed > 0 as libc::c_int as libc::c_ulong
        {
            let mut uReadThis: crate::stdlib::uInt = 16384 as libc::c_int as crate::stdlib::uInt;
            if (*pfile_in_zip_read_info).rest_read_compressed < uReadThis as libc::c_ulong {
                uReadThis = (*pfile_in_zip_read_info).rest_read_compressed as crate::stdlib::uInt
            }
            if uReadThis == 0 as libc::c_int as libc::c_uint {
                return 0 as libc::c_int;
            }
            if Some(
                (*pfile_in_zip_read_info)
                    .z_filefunc
                    .zseek_file
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                (*pfile_in_zip_read_info).z_filefunc.opaque,
                (*pfile_in_zip_read_info).filestream,
                (*pfile_in_zip_read_info)
                    .pos_in_zipfile
                    .wrapping_add((*pfile_in_zip_read_info).byte_before_the_zipfile),
                0 as libc::c_int,
            ) != 0 as libc::c_int as libc::c_long
            {
                return -(1 as libc::c_int);
            }
            if Some(
                (*pfile_in_zip_read_info)
                    .z_filefunc
                    .zread_file
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                (*pfile_in_zip_read_info).z_filefunc.opaque,
                (*pfile_in_zip_read_info).filestream,
                (*pfile_in_zip_read_info).read_buffer as *mut libc::c_void,
                uReadThis as crate::stdlib::uLong,
            ) != uReadThis as libc::c_ulong
            {
                return -(1 as libc::c_int);
            }
            (*pfile_in_zip_read_info).pos_in_zipfile =
                ((*pfile_in_zip_read_info).pos_in_zipfile as libc::c_ulong)
                    .wrapping_add(uReadThis as libc::c_ulong)
                    as crate::stdlib::uLong as crate::stdlib::uLong;
            (*pfile_in_zip_read_info).rest_read_compressed =
                ((*pfile_in_zip_read_info).rest_read_compressed as libc::c_ulong)
                    .wrapping_sub(uReadThis as libc::c_ulong)
                    as crate::stdlib::uLong as crate::stdlib::uLong;
            (*pfile_in_zip_read_info).stream.next_in =
                (*pfile_in_zip_read_info).read_buffer as *mut crate::stdlib::Bytef;
            (*pfile_in_zip_read_info).stream.avail_in = uReadThis
        }
        if (*pfile_in_zip_read_info).compression_method == 0 as libc::c_int as libc::c_ulong
            || (*pfile_in_zip_read_info).raw != 0
        {
            let mut uDoCopy: crate::stdlib::uInt = 0;
            let mut i: crate::stdlib::uInt = 0;
            if (*pfile_in_zip_read_info).stream.avail_in == 0 as libc::c_int as libc::c_uint
                && (*pfile_in_zip_read_info).rest_read_compressed
                    == 0 as libc::c_int as libc::c_ulong
            {
                return if iRead == 0 as libc::c_int as libc::c_uint {
                    0 as libc::c_int as libc::c_uint
                } else {
                    iRead
                } as libc::c_int;
            }
            if (*pfile_in_zip_read_info).stream.avail_out
                < (*pfile_in_zip_read_info).stream.avail_in
            {
                uDoCopy = (*pfile_in_zip_read_info).stream.avail_out
            } else {
                uDoCopy = (*pfile_in_zip_read_info).stream.avail_in
            }
            i = 0 as libc::c_int as crate::stdlib::uInt;
            while i < uDoCopy {
                *(*pfile_in_zip_read_info).stream.next_out.offset(i as isize) =
                    *(*pfile_in_zip_read_info).stream.next_in.offset(i as isize);
                i = i.wrapping_add(1)
            }
            (*pfile_in_zip_read_info).crc32 = crate::stdlib::crc32(
                (*pfile_in_zip_read_info).crc32,
                (*pfile_in_zip_read_info).stream.next_out,
                uDoCopy,
            );
            (*pfile_in_zip_read_info).rest_read_uncompressed =
                ((*pfile_in_zip_read_info).rest_read_uncompressed as libc::c_ulong)
                    .wrapping_sub(uDoCopy as libc::c_ulong) as crate::stdlib::uLong
                    as crate::stdlib::uLong;
            (*pfile_in_zip_read_info).stream.avail_in =
                ((*pfile_in_zip_read_info).stream.avail_in as libc::c_uint).wrapping_sub(uDoCopy)
                    as crate::stdlib::uInt as crate::stdlib::uInt;
            (*pfile_in_zip_read_info).stream.avail_out =
                ((*pfile_in_zip_read_info).stream.avail_out as libc::c_uint).wrapping_sub(uDoCopy)
                    as crate::stdlib::uInt as crate::stdlib::uInt;
            (*pfile_in_zip_read_info).stream.next_out = (*pfile_in_zip_read_info)
                .stream
                .next_out
                .offset(uDoCopy as isize);
            (*pfile_in_zip_read_info).stream.next_in = (*pfile_in_zip_read_info)
                .stream
                .next_in
                .offset(uDoCopy as isize);
            (*pfile_in_zip_read_info).stream.total_out =
                ((*pfile_in_zip_read_info).stream.total_out as libc::c_ulong)
                    .wrapping_add(uDoCopy as libc::c_ulong) as crate::stdlib::uLong
                    as crate::stdlib::uLong;
            iRead = (iRead as libc::c_uint).wrapping_add(uDoCopy) as crate::stdlib::uInt
                as crate::stdlib::uInt
        } else {
            let mut uTotalOutBefore: crate::stdlib::uLong = 0;
            let mut uTotalOutAfter: crate::stdlib::uLong = 0;
            let mut bufBefore: *const crate::stdlib::Bytef = 0 as *const crate::stdlib::Bytef;
            let mut uOutThis: crate::stdlib::uLong = 0;
            let mut flush: libc::c_int = 2 as libc::c_int;
            uTotalOutBefore = (*pfile_in_zip_read_info).stream.total_out;
            bufBefore = (*pfile_in_zip_read_info).stream.next_out;
            /*
            if ((pfile_in_zip_read_info->rest_read_uncompressed ==
                     pfile_in_zip_read_info->stream.avail_out) &&
                (pfile_in_zip_read_info->rest_read_compressed == 0))
                flush = Z_FINISH;
            */
            err = crate::stdlib::inflate(&mut (*pfile_in_zip_read_info).stream, flush);
            if err >= 0 as libc::c_int && !(*pfile_in_zip_read_info).stream.msg.is_null() {
                err = -(3 as libc::c_int)
            }
            uTotalOutAfter = (*pfile_in_zip_read_info).stream.total_out;
            uOutThis = uTotalOutAfter.wrapping_sub(uTotalOutBefore);
            (*pfile_in_zip_read_info).crc32 = crate::stdlib::crc32(
                (*pfile_in_zip_read_info).crc32,
                bufBefore,
                uOutThis as crate::stdlib::uInt,
            );
            (*pfile_in_zip_read_info).rest_read_uncompressed =
                ((*pfile_in_zip_read_info).rest_read_uncompressed as libc::c_ulong)
                    .wrapping_sub(uOutThis) as crate::stdlib::uLong
                    as crate::stdlib::uLong;
            iRead = (iRead as libc::c_uint)
                .wrapping_add(uTotalOutAfter.wrapping_sub(uTotalOutBefore) as crate::stdlib::uInt)
                as crate::stdlib::uInt as crate::stdlib::uInt;
            if err == 1 as libc::c_int {
                return if iRead == 0 as libc::c_int as libc::c_uint {
                    0 as libc::c_int as libc::c_uint
                } else {
                    iRead
                } as libc::c_int;
            }
            if err != 0 as libc::c_int {
                break;
            }
        }
    }
    if err == 0 as libc::c_int {
        return iRead as libc::c_int;
    }
    return err;
}
/*
  Give the current position in uncompressed data
*/
#[no_mangle]

pub unsafe extern "C" fn unztell(
    mut file: crate::src::qcommon::unzip::unzFile,
) -> crate::stdlib::off_t {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut pfile_in_zip_read_info: *mut file_in_zip_read_info_s =
        0 as *mut file_in_zip_read_info_s;
    if file.is_null() {
        return -(102 as libc::c_int) as crate::stdlib::off_t;
    }
    s = file as *mut unz_s;
    pfile_in_zip_read_info = (*s).pfile_in_zip_read;
    if pfile_in_zip_read_info.is_null() {
        return -(102 as libc::c_int) as crate::stdlib::off_t;
    }
    return (*pfile_in_zip_read_info).stream.total_out as crate::stdlib::off_t;
}
/* unzip.h -- IO for uncompress .zip files using zlib
   Version 1.01e, February 12th, 2005

   Copyright (C) 1998-2005 Gilles Vollant

   This unzip package allow extract file from .ZIP file, compatible with PKZip 2.04g
     WinZip, InfoZip tools and compatible.

   Multi volume ZipFile (span) are not supported.
   Encryption compatible with pkzip 2.04g only supported
   Old compressions used by old PKZip 1.x are not supported


   I WAIT FEEDBACK at mail info@winimage.com
   Visit also http://www.winimage.com/zLibDll/unzip.htm for evolution

   Condition of use and distribution are the same than zlib :

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.


*/
/* for more info about .ZIP format, see
      http://www.info-zip.org/pub/infozip/doc/appnote-981119-iz.zip
      http://www.info-zip.org/pub/infozip/doc/
   PkWare has also a specification at :
      ftp://ftp.pkware.com/probdesc.zip
*/
/* tm_unz contain date/time info */
/* seconds after the minute - [0,59] */
/* minutes after the hour - [0,59] */
/* hours since midnight - [0,23] */
/* day of the month - [1,31] */
/* months since January - [0,11] */
/* years - [1980..2044] */
/* unz_global_info structure contain global data about the ZIPfile
These data comes from the end of central dir */
/* total number of entries in
the central dir on this disk */
/* size of the global comment of the zipfile */
/* unz_file_info contain information about a file in the zipfile */
/* version made by                 2 bytes */
/* version needed to extract       2 bytes */
/* general purpose bit flag        2 bytes */
/* compression method              2 bytes */
/* last mod file date in Dos fmt   4 bytes */
/* crc-32                          4 bytes */
/* compressed size                 4 bytes */
/* uncompressed size               4 bytes */
/* filename length                 2 bytes */
/* extra field length              2 bytes */
/* file comment length             2 bytes */
/* disk number start               2 bytes */
/* internal file attributes        2 bytes */
/* external file attributes        4 bytes */
/*
   Compare two filename (fileName1,fileName2).
   If iCaseSenisivity = 1, comparison is case sensitivity (like strcmp)
   If iCaseSenisivity = 2, comparison is not case sensitivity (like strcmpi
                                or strcasecmp)
   If iCaseSenisivity = 0, case sensitivity is defaut of your operating system
    (like 1 on Unix, 2 on Windows)
*/
/*
  Open a Zip file. path contain the full pathname (by example,
     on a Windows XP computer "c:\\zlib\\zlib113.zip" or on a Unix computer
     "zlib/zlib113.zip".
     If the zipfile cannot be opened (file don't exist or in not valid), the
       return value is NULL.
     Else, the return value is an unzFile Handle, usable with other function
       of this unzip package.
*/
/*
   Open a Zip file, like unzOpen, but provide a set of file low level API
      for read/write the zip file (see ioapi.h)
*/
/*
Close a ZipFile opened with unzipOpen.
If there is files inside the .Zip opened with unzOpenCurrentFile (see later),
  these files MUST be closed with unzipCloseCurrentFile before call unzipClose.
return UNZ_OK if there is no problem. */
/*
Write info about the ZipFile in the *pglobal_info structure.
No preparation of the structure is needed
return UNZ_OK if there is no problem. */
/*
  Get the global comment string of the ZipFile, in the szComment buffer.
  uSizeBuf is the size of the szComment buffer.
  return the number of byte copied or an error code <0
*/
/* **************************************************************************/
/* Unzip package allow you browse the directory of the zipfile */
/*
  Set the current file of the zipfile to the first file.
  return UNZ_OK if there is no problem
*/
/*
  Set the current file of the zipfile to the next file.
  return UNZ_OK if there is no problem
  return UNZ_END_OF_LIST_OF_FILE if the actual file was the latest.
*/
/*
  Try locate the file szFileName in the zipfile.
  For the iCaseSensitivity signification, see unzStringFileNameCompare

  return value :
  UNZ_OK if the file is found. It becomes the current file.
  UNZ_END_OF_LIST_OF_FILE if the file is not found
*/
/* ****************************************** */
/* Ryan supplied functions */
/* unz_file_info contain information about a file in the zipfile */
/* offset in zip file directory */
/* # of file */
/* ****************************************** */
/*
  Get Info about the current file
  if pfile_info!=NULL, the *pfile_info structure will contain somes info about
        the current file
  if szFileName!=NULL, the filemane string will be copied in szFileName
            (fileNameBufferSize is the size of the buffer)
  if extraField!=NULL, the extra field information will be copied in extraField
            (extraFieldBufferSize is the size of the buffer).
            This is the Central-header version of the extra field
  if szComment!=NULL, the comment string of the file will be copied in szComment
            (commentBufferSize is the size of the buffer)
*/
/* **************************************************************************/
/* for reading the content of the current zipfile, you can open it, read data
from it, and close it (you can close it before reading all the file)
*/
/*
  Open for reading data the current file in the zipfile.
  If there is no error, the return value is UNZ_OK.
*/
/*
  Open for reading data the current file in the zipfile.
  password is a crypting password
  If there is no error, the return value is UNZ_OK.
*/
/*
  Same than unzOpenCurrentFile, but open for read raw the file (not uncompress)
    if raw==1
  *method will receive method of compression, *level will receive level of
     compression
  note : you can set level parameter as NULL (if you did not want known level,
         but you CANNOT set method parameter as NULL
*/
/*
  Same than unzOpenCurrentFile, but open for read raw the file (not uncompress)
    if raw==1
  *method will receive method of compression, *level will receive level of
     compression
  note : you can set level parameter as NULL (if you did not want known level,
         but you CANNOT set method parameter as NULL
*/
/*
  Close the file in zip opened with unzOpenCurrentFile
  Return UNZ_CRCERROR if all the file was read but the CRC is not good
*/
/*
  Read bytes from the current file (opened by unzOpenCurrentFile)
  buf contain buffer where data must be copied
  len the size of buf.

  return the number of byte copied if somes bytes are copied
  return 0 if the end of file was reached
  return <0 with error code if there is an error
    (UNZ_ERRNO for IO error, or zLib error for uncompress error)
*/
/*
  Give the current position in uncompressed data
*/
/*
  return 1 if the end of file was reached, 0 elsewhere
*/
#[no_mangle]

pub unsafe extern "C" fn unzeof(mut file: crate::src::qcommon::unzip::unzFile) -> libc::c_int {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut pfile_in_zip_read_info: *mut file_in_zip_read_info_s =
        0 as *mut file_in_zip_read_info_s;
    if file.is_null() {
        return -(102 as libc::c_int);
    }
    s = file as *mut unz_s;
    pfile_in_zip_read_info = (*s).pfile_in_zip_read;
    if pfile_in_zip_read_info.is_null() {
        return -(102 as libc::c_int);
    }
    if (*pfile_in_zip_read_info).rest_read_uncompressed == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}
/*
  Read extra field from the current file (opened by unzOpenCurrentFile)
  This is the local-header version of the extra field (sometimes, there is
    more info in the local-header version than in the central-header)

  if buf==NULL, it return the size of the local extra field that can be read

  if buf!=NULL, len is the size of the buffer, the extra header is copied in
    buf.
  the return value is the number of bytes copied in buf, or (if <0)
    the error code
*/
#[no_mangle]

pub unsafe extern "C" fn unzGetLocalExtrafield(
    mut file: crate::src::qcommon::unzip::unzFile,
    mut buf: crate::stdlib::voidp,
    mut len: libc::c_uint,
) -> libc::c_int {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut pfile_in_zip_read_info: *mut file_in_zip_read_info_s =
        0 as *mut file_in_zip_read_info_s;
    let mut read_now: crate::stdlib::uInt = 0;
    let mut size_to_read: crate::stdlib::uLong = 0;
    if file.is_null() {
        return -(102 as libc::c_int);
    }
    s = file as *mut unz_s;
    pfile_in_zip_read_info = (*s).pfile_in_zip_read;
    if pfile_in_zip_read_info.is_null() {
        return -(102 as libc::c_int);
    }
    size_to_read = ((*pfile_in_zip_read_info).size_local_extrafield as libc::c_ulong)
        .wrapping_sub((*pfile_in_zip_read_info).pos_local_extrafield);
    if buf.is_null() {
        return size_to_read as libc::c_int;
    }
    if len as libc::c_ulong > size_to_read {
        read_now = size_to_read as crate::stdlib::uInt
    } else {
        read_now = len
    }
    if read_now == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if Some(
        (*pfile_in_zip_read_info)
            .z_filefunc
            .zseek_file
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        (*pfile_in_zip_read_info).z_filefunc.opaque,
        (*pfile_in_zip_read_info).filestream,
        (*pfile_in_zip_read_info)
            .offset_local_extrafield
            .wrapping_add((*pfile_in_zip_read_info).pos_local_extrafield),
        0 as libc::c_int,
    ) != 0 as libc::c_int as libc::c_long
    {
        return -(1 as libc::c_int);
    }
    if Some(
        (*pfile_in_zip_read_info)
            .z_filefunc
            .zread_file
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        (*pfile_in_zip_read_info).z_filefunc.opaque,
        (*pfile_in_zip_read_info).filestream,
        buf,
        read_now as crate::stdlib::uLong,
    ) != read_now as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    return read_now as libc::c_int;
}
/*
  Close the file in zip opened with unzipOpenCurrentFile
  Return UNZ_CRCERROR if all the file was read but the CRC is not good
*/
#[no_mangle]

pub unsafe extern "C" fn unzCloseCurrentFile(
    mut file: crate::src::qcommon::unzip::unzFile,
) -> libc::c_int {
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut pfile_in_zip_read_info: *mut file_in_zip_read_info_s =
        0 as *mut file_in_zip_read_info_s;
    if file.is_null() {
        return -(102 as libc::c_int);
    }
    s = file as *mut unz_s;
    pfile_in_zip_read_info = (*s).pfile_in_zip_read;
    if pfile_in_zip_read_info.is_null() {
        return -(102 as libc::c_int);
    }
    if (*pfile_in_zip_read_info).rest_read_uncompressed == 0 as libc::c_int as libc::c_ulong
        && (*pfile_in_zip_read_info).raw == 0
    {
        if (*pfile_in_zip_read_info).crc32 != (*pfile_in_zip_read_info).crc32_wait {
            err = -(105 as libc::c_int)
        }
    }
    if !(*pfile_in_zip_read_info).read_buffer.is_null() {
        crate::src::qcommon::common::Z_Free(
            (*pfile_in_zip_read_info).read_buffer as *mut libc::c_void,
        );
    }
    (*pfile_in_zip_read_info).read_buffer = 0 as *mut libc::c_char;
    if (*pfile_in_zip_read_info).stream_initialised != 0 {
        crate::stdlib::inflateEnd(&mut (*pfile_in_zip_read_info).stream);
    }
    (*pfile_in_zip_read_info).stream_initialised = 0 as libc::c_int as crate::stdlib::uLong;
    if !pfile_in_zip_read_info.is_null() {
        crate::src::qcommon::common::Z_Free(pfile_in_zip_read_info as *mut libc::c_void);
    }
    (*s).pfile_in_zip_read = 0 as *mut file_in_zip_read_info_s;
    return err;
}
/*
  Get the global comment string of the ZipFile, in the szComment buffer.
  uSizeBuf is the size of the szComment buffer.
  return the number of byte copied or an error code <0
*/
#[no_mangle]

pub unsafe extern "C" fn unzGetGlobalComment(
    mut file: crate::src::qcommon::unzip::unzFile,
    mut szComment: *mut libc::c_char,
    mut uSizeBuf: crate::stdlib::uLong,
) -> libc::c_int {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    let mut uReadThis: crate::stdlib::uLong = 0;
    if file.is_null() {
        return -(102 as libc::c_int);
    }
    s = file as *mut unz_s;
    uReadThis = uSizeBuf;
    if uReadThis > (*s).gi.size_comment {
        uReadThis = (*s).gi.size_comment
    }
    if Some(
        (*s).z_filefunc
            .zseek_file
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        (*s).z_filefunc.opaque,
        (*s).filestream,
        (*s).central_pos
            .wrapping_add(22 as libc::c_int as libc::c_ulong),
        0 as libc::c_int,
    ) != 0 as libc::c_int as libc::c_long
    {
        return -(1 as libc::c_int);
    }
    if uReadThis > 0 as libc::c_int as libc::c_ulong {
        *szComment = '\u{0}' as i32 as libc::c_char;
        if Some(
            (*s).z_filefunc
                .zread_file
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            (*s).z_filefunc.opaque,
            (*s).filestream,
            szComment as *mut libc::c_void,
            uReadThis,
        ) != uReadThis
        {
            return -(1 as libc::c_int);
        }
    }
    if !szComment.is_null() && uSizeBuf > (*s).gi.size_comment {
        *szComment.offset((*s).gi.size_comment as isize) = '\u{0}' as i32 as libc::c_char
    }
    return uReadThis as libc::c_int;
}
/* Additions by RX '2004 */
#[no_mangle]

pub unsafe extern "C" fn unzGetOffset(
    mut file: crate::src::qcommon::unzip::unzFile,
) -> crate::stdlib::uLong {
    let mut s: *mut unz_s = 0 as *mut unz_s;
    if file.is_null() {
        return -(102 as libc::c_int) as crate::stdlib::uLong;
    }
    s = file as *mut unz_s;
    if (*s).current_file_ok == 0 {
        return 0 as libc::c_int as crate::stdlib::uLong;
    }
    if (*s).gi.number_entry != 0 as libc::c_int as libc::c_ulong
        && (*s).gi.number_entry != 0xffff as libc::c_int as libc::c_ulong
    {
        if (*s).num_file == (*s).gi.number_entry {
            return 0 as libc::c_int as crate::stdlib::uLong;
        }
    }
    return (*s).pos_in_central_dir;
}
/*
  return 1 if the end of file was reached, 0 elsewhere
*/
/*
  Read extra field from the current file (opened by unzOpenCurrentFile)
  This is the local-header version of the extra field (sometimes, there is
    more info in the local-header version than in the central-header)

  if buf==NULL, it return the size of the local extra field

  if buf!=NULL, len is the size of the buffer, the extra header is copied in
    buf.
  the return value is the number of bytes copied in buf, or (if <0)
    the error code
*/
/* **************************************************************************/
/* Get the current file offset */
/* Set the current file offset */
#[no_mangle]

pub unsafe extern "C" fn unzSetOffset(
    mut file: crate::src::qcommon::unzip::unzFile,
    mut pos: crate::stdlib::uLong,
) -> libc::c_int {
    let mut s: *mut unz_s = 0 as *mut unz_s; /* hack */
    let mut err: libc::c_int = 0;
    if file.is_null() {
        return -(102 as libc::c_int);
    }
    s = file as *mut unz_s;
    (*s).pos_in_central_dir = pos;
    (*s).num_file = (*s).gi.number_entry;
    err = unzlocal_GetCurrentFileInfoInternal(
        file,
        &mut (*s).cur_file_info,
        &mut (*s).cur_file_info_internal,
        0 as *mut libc::c_char,
        0 as libc::c_int as crate::stdlib::uLong,
        0 as *mut libc::c_void,
        0 as libc::c_int as crate::stdlib::uLong,
        0 as *mut libc::c_char,
        0 as libc::c_int as crate::stdlib::uLong,
    );
    (*s).current_file_ok = (err == 0 as libc::c_int) as libc::c_int as crate::stdlib::uLong;
    return err;
}
