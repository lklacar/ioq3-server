use ::libc;

pub use crate::src::qcommon::q_shared::byte;
#[no_mangle]

pub static mut crctable: [libc::c_ushort; 257] = [
    0 as libc::c_int as libc::c_ushort,
    0x1021 as libc::c_int as libc::c_ushort,
    0x2042 as libc::c_int as libc::c_ushort,
    0x3063 as libc::c_int as libc::c_ushort,
    0x4084 as libc::c_int as libc::c_ushort,
    0x50a5 as libc::c_int as libc::c_ushort,
    0x60c6 as libc::c_int as libc::c_ushort,
    0x70e7 as libc::c_int as libc::c_ushort,
    0x8108 as libc::c_int as libc::c_ushort,
    0x9129 as libc::c_int as libc::c_ushort,
    0xa14a as libc::c_int as libc::c_ushort,
    0xb16b as libc::c_int as libc::c_ushort,
    0xc18c as libc::c_int as libc::c_ushort,
    0xd1ad as libc::c_int as libc::c_ushort,
    0xe1ce as libc::c_int as libc::c_ushort,
    0xf1ef as libc::c_int as libc::c_ushort,
    0x1231 as libc::c_int as libc::c_ushort,
    0x210 as libc::c_int as libc::c_ushort,
    0x3273 as libc::c_int as libc::c_ushort,
    0x2252 as libc::c_int as libc::c_ushort,
    0x52b5 as libc::c_int as libc::c_ushort,
    0x4294 as libc::c_int as libc::c_ushort,
    0x72f7 as libc::c_int as libc::c_ushort,
    0x62d6 as libc::c_int as libc::c_ushort,
    0x9339 as libc::c_int as libc::c_ushort,
    0x8318 as libc::c_int as libc::c_ushort,
    0xb37b as libc::c_int as libc::c_ushort,
    0xa35a as libc::c_int as libc::c_ushort,
    0xd3bd as libc::c_int as libc::c_ushort,
    0xc39c as libc::c_int as libc::c_ushort,
    0xf3ff as libc::c_int as libc::c_ushort,
    0xe3de as libc::c_int as libc::c_ushort,
    0x2462 as libc::c_int as libc::c_ushort,
    0x3443 as libc::c_int as libc::c_ushort,
    0x420 as libc::c_int as libc::c_ushort,
    0x1401 as libc::c_int as libc::c_ushort,
    0x64e6 as libc::c_int as libc::c_ushort,
    0x74c7 as libc::c_int as libc::c_ushort,
    0x44a4 as libc::c_int as libc::c_ushort,
    0x5485 as libc::c_int as libc::c_ushort,
    0xa56a as libc::c_int as libc::c_ushort,
    0xb54b as libc::c_int as libc::c_ushort,
    0x8528 as libc::c_int as libc::c_ushort,
    0x9509 as libc::c_int as libc::c_ushort,
    0xe5ee as libc::c_int as libc::c_ushort,
    0xf5cf as libc::c_int as libc::c_ushort,
    0xc5ac as libc::c_int as libc::c_ushort,
    0xd58d as libc::c_int as libc::c_ushort,
    0x3653 as libc::c_int as libc::c_ushort,
    0x2672 as libc::c_int as libc::c_ushort,
    0x1611 as libc::c_int as libc::c_ushort,
    0x630 as libc::c_int as libc::c_ushort,
    0x76d7 as libc::c_int as libc::c_ushort,
    0x66f6 as libc::c_int as libc::c_ushort,
    0x5695 as libc::c_int as libc::c_ushort,
    0x46b4 as libc::c_int as libc::c_ushort,
    0xb75b as libc::c_int as libc::c_ushort,
    0xa77a as libc::c_int as libc::c_ushort,
    0x9719 as libc::c_int as libc::c_ushort,
    0x8738 as libc::c_int as libc::c_ushort,
    0xf7df as libc::c_int as libc::c_ushort,
    0xe7fe as libc::c_int as libc::c_ushort,
    0xd79d as libc::c_int as libc::c_ushort,
    0xc7bc as libc::c_int as libc::c_ushort,
    0x48c4 as libc::c_int as libc::c_ushort,
    0x58e5 as libc::c_int as libc::c_ushort,
    0x6886 as libc::c_int as libc::c_ushort,
    0x78a7 as libc::c_int as libc::c_ushort,
    0x840 as libc::c_int as libc::c_ushort,
    0x1861 as libc::c_int as libc::c_ushort,
    0x2802 as libc::c_int as libc::c_ushort,
    0x3823 as libc::c_int as libc::c_ushort,
    0xc9cc as libc::c_int as libc::c_ushort,
    0xd9ed as libc::c_int as libc::c_ushort,
    0xe98e as libc::c_int as libc::c_ushort,
    0xf9af as libc::c_int as libc::c_ushort,
    0x8948 as libc::c_int as libc::c_ushort,
    0x9969 as libc::c_int as libc::c_ushort,
    0xa90a as libc::c_int as libc::c_ushort,
    0xb92b as libc::c_int as libc::c_ushort,
    0x5af5 as libc::c_int as libc::c_ushort,
    0x4ad4 as libc::c_int as libc::c_ushort,
    0x7ab7 as libc::c_int as libc::c_ushort,
    0x6a96 as libc::c_int as libc::c_ushort,
    0x1a71 as libc::c_int as libc::c_ushort,
    0xa50 as libc::c_int as libc::c_ushort,
    0x3a33 as libc::c_int as libc::c_ushort,
    0x2a12 as libc::c_int as libc::c_ushort,
    0xdbfd as libc::c_int as libc::c_ushort,
    0xcbdc as libc::c_int as libc::c_ushort,
    0xfbbf as libc::c_int as libc::c_ushort,
    0xeb9e as libc::c_int as libc::c_ushort,
    0x9b79 as libc::c_int as libc::c_ushort,
    0x8b58 as libc::c_int as libc::c_ushort,
    0xbb3b as libc::c_int as libc::c_ushort,
    0xab1a as libc::c_int as libc::c_ushort,
    0x6ca6 as libc::c_int as libc::c_ushort,
    0x7c87 as libc::c_int as libc::c_ushort,
    0x4ce4 as libc::c_int as libc::c_ushort,
    0x5cc5 as libc::c_int as libc::c_ushort,
    0x2c22 as libc::c_int as libc::c_ushort,
    0x3c03 as libc::c_int as libc::c_ushort,
    0xc60 as libc::c_int as libc::c_ushort,
    0x1c41 as libc::c_int as libc::c_ushort,
    0xedae as libc::c_int as libc::c_ushort,
    0xfd8f as libc::c_int as libc::c_ushort,
    0xcdec as libc::c_int as libc::c_ushort,
    0xddcd as libc::c_int as libc::c_ushort,
    0xad2a as libc::c_int as libc::c_ushort,
    0xbd0b as libc::c_int as libc::c_ushort,
    0x8d68 as libc::c_int as libc::c_ushort,
    0x9d49 as libc::c_int as libc::c_ushort,
    0x7e97 as libc::c_int as libc::c_ushort,
    0x6eb6 as libc::c_int as libc::c_ushort,
    0x5ed5 as libc::c_int as libc::c_ushort,
    0x4ef4 as libc::c_int as libc::c_ushort,
    0x3e13 as libc::c_int as libc::c_ushort,
    0x2e32 as libc::c_int as libc::c_ushort,
    0x1e51 as libc::c_int as libc::c_ushort,
    0xe70 as libc::c_int as libc::c_ushort,
    0xff9f as libc::c_int as libc::c_ushort,
    0xefbe as libc::c_int as libc::c_ushort,
    0xdfdd as libc::c_int as libc::c_ushort,
    0xcffc as libc::c_int as libc::c_ushort,
    0xbf1b as libc::c_int as libc::c_ushort,
    0xaf3a as libc::c_int as libc::c_ushort,
    0x9f59 as libc::c_int as libc::c_ushort,
    0x8f78 as libc::c_int as libc::c_ushort,
    0x9188 as libc::c_int as libc::c_ushort,
    0x81a9 as libc::c_int as libc::c_ushort,
    0xb1ca as libc::c_int as libc::c_ushort,
    0xa1eb as libc::c_int as libc::c_ushort,
    0xd10c as libc::c_int as libc::c_ushort,
    0xc12d as libc::c_int as libc::c_ushort,
    0xf14e as libc::c_int as libc::c_ushort,
    0xe16f as libc::c_int as libc::c_ushort,
    0x1080 as libc::c_int as libc::c_ushort,
    0xa1 as libc::c_int as libc::c_ushort,
    0x30c2 as libc::c_int as libc::c_ushort,
    0x20e3 as libc::c_int as libc::c_ushort,
    0x5004 as libc::c_int as libc::c_ushort,
    0x4025 as libc::c_int as libc::c_ushort,
    0x7046 as libc::c_int as libc::c_ushort,
    0x6067 as libc::c_int as libc::c_ushort,
    0x83b9 as libc::c_int as libc::c_ushort,
    0x9398 as libc::c_int as libc::c_ushort,
    0xa3fb as libc::c_int as libc::c_ushort,
    0xb3da as libc::c_int as libc::c_ushort,
    0xc33d as libc::c_int as libc::c_ushort,
    0xd31c as libc::c_int as libc::c_ushort,
    0xe37f as libc::c_int as libc::c_ushort,
    0xf35e as libc::c_int as libc::c_ushort,
    0x2b1 as libc::c_int as libc::c_ushort,
    0x1290 as libc::c_int as libc::c_ushort,
    0x22f3 as libc::c_int as libc::c_ushort,
    0x32d2 as libc::c_int as libc::c_ushort,
    0x4235 as libc::c_int as libc::c_ushort,
    0x5214 as libc::c_int as libc::c_ushort,
    0x6277 as libc::c_int as libc::c_ushort,
    0x7256 as libc::c_int as libc::c_ushort,
    0xb5ea as libc::c_int as libc::c_ushort,
    0xa5cb as libc::c_int as libc::c_ushort,
    0x95a8 as libc::c_int as libc::c_ushort,
    0x8589 as libc::c_int as libc::c_ushort,
    0xf56e as libc::c_int as libc::c_ushort,
    0xe54f as libc::c_int as libc::c_ushort,
    0xd52c as libc::c_int as libc::c_ushort,
    0xc50d as libc::c_int as libc::c_ushort,
    0x34e2 as libc::c_int as libc::c_ushort,
    0x24c3 as libc::c_int as libc::c_ushort,
    0x14a0 as libc::c_int as libc::c_ushort,
    0x481 as libc::c_int as libc::c_ushort,
    0x7466 as libc::c_int as libc::c_ushort,
    0x6447 as libc::c_int as libc::c_ushort,
    0x5424 as libc::c_int as libc::c_ushort,
    0x4405 as libc::c_int as libc::c_ushort,
    0xa7db as libc::c_int as libc::c_ushort,
    0xb7fa as libc::c_int as libc::c_ushort,
    0x8799 as libc::c_int as libc::c_ushort,
    0x97b8 as libc::c_int as libc::c_ushort,
    0xe75f as libc::c_int as libc::c_ushort,
    0xf77e as libc::c_int as libc::c_ushort,
    0xc71d as libc::c_int as libc::c_ushort,
    0xd73c as libc::c_int as libc::c_ushort,
    0x26d3 as libc::c_int as libc::c_ushort,
    0x36f2 as libc::c_int as libc::c_ushort,
    0x691 as libc::c_int as libc::c_ushort,
    0x16b0 as libc::c_int as libc::c_ushort,
    0x6657 as libc::c_int as libc::c_ushort,
    0x7676 as libc::c_int as libc::c_ushort,
    0x4615 as libc::c_int as libc::c_ushort,
    0x5634 as libc::c_int as libc::c_ushort,
    0xd94c as libc::c_int as libc::c_ushort,
    0xc96d as libc::c_int as libc::c_ushort,
    0xf90e as libc::c_int as libc::c_ushort,
    0xe92f as libc::c_int as libc::c_ushort,
    0x99c8 as libc::c_int as libc::c_ushort,
    0x89e9 as libc::c_int as libc::c_ushort,
    0xb98a as libc::c_int as libc::c_ushort,
    0xa9ab as libc::c_int as libc::c_ushort,
    0x5844 as libc::c_int as libc::c_ushort,
    0x4865 as libc::c_int as libc::c_ushort,
    0x7806 as libc::c_int as libc::c_ushort,
    0x6827 as libc::c_int as libc::c_ushort,
    0x18c0 as libc::c_int as libc::c_ushort,
    0x8e1 as libc::c_int as libc::c_ushort,
    0x3882 as libc::c_int as libc::c_ushort,
    0x28a3 as libc::c_int as libc::c_ushort,
    0xcb7d as libc::c_int as libc::c_ushort,
    0xdb5c as libc::c_int as libc::c_ushort,
    0xeb3f as libc::c_int as libc::c_ushort,
    0xfb1e as libc::c_int as libc::c_ushort,
    0x8bf9 as libc::c_int as libc::c_ushort,
    0x9bd8 as libc::c_int as libc::c_ushort,
    0xabbb as libc::c_int as libc::c_ushort,
    0xbb9a as libc::c_int as libc::c_ushort,
    0x4a75 as libc::c_int as libc::c_ushort,
    0x5a54 as libc::c_int as libc::c_ushort,
    0x6a37 as libc::c_int as libc::c_ushort,
    0x7a16 as libc::c_int as libc::c_ushort,
    0xaf1 as libc::c_int as libc::c_ushort,
    0x1ad0 as libc::c_int as libc::c_ushort,
    0x2ab3 as libc::c_int as libc::c_ushort,
    0x3a92 as libc::c_int as libc::c_ushort,
    0xfd2e as libc::c_int as libc::c_ushort,
    0xed0f as libc::c_int as libc::c_ushort,
    0xdd6c as libc::c_int as libc::c_ushort,
    0xcd4d as libc::c_int as libc::c_ushort,
    0xbdaa as libc::c_int as libc::c_ushort,
    0xad8b as libc::c_int as libc::c_ushort,
    0x9de8 as libc::c_int as libc::c_ushort,
    0x8dc9 as libc::c_int as libc::c_ushort,
    0x7c26 as libc::c_int as libc::c_ushort,
    0x6c07 as libc::c_int as libc::c_ushort,
    0x5c64 as libc::c_int as libc::c_ushort,
    0x4c45 as libc::c_int as libc::c_ushort,
    0x3ca2 as libc::c_int as libc::c_ushort,
    0x2c83 as libc::c_int as libc::c_ushort,
    0x1ce0 as libc::c_int as libc::c_ushort,
    0xcc1 as libc::c_int as libc::c_ushort,
    0xef1f as libc::c_int as libc::c_ushort,
    0xff3e as libc::c_int as libc::c_ushort,
    0xcf5d as libc::c_int as libc::c_ushort,
    0xdf7c as libc::c_int as libc::c_ushort,
    0xaf9b as libc::c_int as libc::c_ushort,
    0xbfba as libc::c_int as libc::c_ushort,
    0x8fd9 as libc::c_int as libc::c_ushort,
    0x9ff8 as libc::c_int as libc::c_ushort,
    0x6e17 as libc::c_int as libc::c_ushort,
    0x7e36 as libc::c_int as libc::c_ushort,
    0x4e55 as libc::c_int as libc::c_ushort,
    0x5e74 as libc::c_int as libc::c_ushort,
    0x2e93 as libc::c_int as libc::c_ushort,
    0x3eb2 as libc::c_int as libc::c_ushort,
    0xed1 as libc::c_int as libc::c_ushort,
    0x1ef0 as libc::c_int as libc::c_ushort,
    0,
];
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn CRC_Init(mut crcvalue: *mut libc::c_ushort) {
    *crcvalue = 0xffff as libc::c_int as libc::c_ushort;
}
//end of the function CRC_Init
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn CRC_ProcessByte(
    mut crcvalue: *mut libc::c_ushort,
    mut data: crate::src::qcommon::q_shared::byte,
) {
    *crcvalue = ((*crcvalue as libc::c_int) << 8 as libc::c_int
        ^ crctable[(*crcvalue as libc::c_int >> 8 as libc::c_int ^ data as libc::c_int) as usize]
            as libc::c_int) as libc::c_ushort;
}
//end of the function CRC_ProcessByte
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn CRC_Value(mut crcvalue: libc::c_ushort) -> libc::c_ushort {
    return (crcvalue as libc::c_int ^ 0 as libc::c_int) as libc::c_ushort;
}
//end of the function CRC_Value
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn CRC_ProcessString(
    mut data: *mut libc::c_uchar,
    mut length: libc::c_int,
) -> libc::c_ushort {
    let mut crcvalue: libc::c_ushort = 0; //end for
    let mut i: libc::c_int = 0;
    let mut ind: libc::c_int = 0;
    CRC_Init(&mut crcvalue);
    i = 0 as libc::c_int;
    while i < length {
        ind = crcvalue as libc::c_int >> 8 as libc::c_int ^ *data.offset(i as isize) as libc::c_int;
        if ind < 0 as libc::c_int || ind > 256 as libc::c_int {
            ind = 0 as libc::c_int
        }
        crcvalue = ((crcvalue as libc::c_int) << 8 as libc::c_int
            ^ crctable[ind as usize] as libc::c_int) as libc::c_ushort;
        i += 1
    }
    return CRC_Value(crcvalue);
}
//end of the function CRC_ProcessString
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn CRC_ContinueProcessString(
    mut crc: *mut libc::c_ushort,
    mut data: *mut libc::c_char,
    mut length: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < length {
        *crc = ((*crc as libc::c_int) << 8 as libc::c_int
            ^ crctable[(*crc as libc::c_int >> 8 as libc::c_int
                ^ *data.offset(i as isize) as libc::c_int) as usize] as libc::c_int)
            as libc::c_ushort;
        i += 1
    }
    //end for
}
//end of the function CRC_ProcessString
