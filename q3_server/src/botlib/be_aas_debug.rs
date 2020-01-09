use ::libc;

pub mod q_shared_h {

    #[inline]

    pub unsafe extern "C" fn CrossProduct(
        mut v1: *const crate::src::qcommon::q_shared::vec_t,
        mut v2: *const crate::src::qcommon::q_shared::vec_t,
        mut cross: *mut crate::src::qcommon::q_shared::vec_t,
    ) {
        *cross.offset(0 as libc::c_int as isize) = *v1.offset(1 as libc::c_int as isize)
            * *v2.offset(2 as libc::c_int as isize)
            - *v1.offset(2 as libc::c_int as isize) * *v2.offset(1 as libc::c_int as isize);
        *cross.offset(1 as libc::c_int as isize) = *v1.offset(2 as libc::c_int as isize)
            * *v2.offset(0 as libc::c_int as isize)
            - *v1.offset(0 as libc::c_int as isize) * *v2.offset(2 as libc::c_int as isize);
        *cross.offset(2 as libc::c_int as isize) = *v1.offset(0 as libc::c_int as isize)
            * *v2.offset(1 as libc::c_int as isize)
            - *v1.offset(1 as libc::c_int as isize) * *v2.offset(0 as libc::c_int as isize);
    }

    // __Q_SHARED_H
}

pub use crate::aasfile_h::aas_area_s;
pub use crate::aasfile_h::aas_area_t;
pub use crate::aasfile_h::aas_areasettings_s;
pub use crate::aasfile_h::aas_areasettings_t;
pub use crate::aasfile_h::aas_bbox_s;
pub use crate::aasfile_h::aas_bbox_t;
pub use crate::aasfile_h::aas_cluster_s;
pub use crate::aasfile_h::aas_cluster_t;
pub use crate::aasfile_h::aas_edge_s;
pub use crate::aasfile_h::aas_edge_t;
pub use crate::aasfile_h::aas_edgeindex_t;
pub use crate::aasfile_h::aas_face_s;
pub use crate::aasfile_h::aas_face_t;
pub use crate::aasfile_h::aas_faceindex_t;
pub use crate::aasfile_h::aas_node_s;
pub use crate::aasfile_h::aas_node_t;
pub use crate::aasfile_h::aas_plane_s;
pub use crate::aasfile_h::aas_plane_t;
pub use crate::aasfile_h::aas_portal_s;
pub use crate::aasfile_h::aas_portal_t;
pub use crate::aasfile_h::aas_portalindex_t;
pub use crate::aasfile_h::aas_reachability_s;
pub use crate::aasfile_h::aas_reachability_t;
pub use crate::aasfile_h::aas_vertex_t;
pub use crate::be_aas_def_h::aas_entity_s;
pub use crate::be_aas_def_h::aas_entity_t;
pub use crate::be_aas_def_h::aas_link_s;
pub use crate::be_aas_def_h::aas_link_t;
pub use crate::be_aas_def_h::aas_reachabilityareas_s;
pub use crate::be_aas_def_h::aas_reachabilityareas_t;
pub use crate::be_aas_def_h::aas_reversedlink_s;
pub use crate::be_aas_def_h::aas_reversedlink_t;
pub use crate::be_aas_def_h::aas_reversedreachability_s;
pub use crate::be_aas_def_h::aas_reversedreachability_t;
pub use crate::be_aas_def_h::aas_routingcache_s;
pub use crate::be_aas_def_h::aas_routingcache_t;
pub use crate::be_aas_def_h::aas_routingupdate_s;
pub use crate::be_aas_def_h::aas_routingupdate_t;
pub use crate::be_aas_def_h::aas_s;
pub use crate::be_aas_def_h::aas_settings_s;
pub use crate::be_aas_def_h::aas_settings_t;
pub use crate::be_aas_def_h::aas_t;
pub use crate::be_aas_def_h::bsp_link_s;
pub use crate::be_aas_def_h::bsp_link_t;
pub use crate::be_aas_h::aas_clientmove_s;
pub use crate::be_aas_h::aas_clientmove_t;
pub use crate::be_aas_h::aas_entityinfo_s;
pub use crate::be_aas_h::aas_entityinfo_t;
pub use crate::be_aas_h::aas_trace_s;
pub use crate::be_aas_h::aas_trace_t;
pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
pub use crate::src::botlib::be_aas_debug::q_shared_h::CrossProduct;
use crate::src::botlib::be_interface::botimport;
use crate::src::botlib::l_memory::FreeMemory;
use crate::src::botlib::l_memory::GetClearedMemory;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;
use crate::stdlib::abs;
use crate::stdlib::memcpy;

use crate::src::botlib::be_aas_main::aasworld;
use crate::src::botlib::be_aas_main::AAS_Time;
use crate::src::botlib::be_aas_move::aassettings;
use crate::src::botlib::be_aas_move::AAS_HorizontalVelocityForJump;
use crate::src::botlib::be_aas_move::AAS_JumpReachRunStart;
use crate::src::botlib::be_aas_move::AAS_PredictClientMovement;
use crate::src::botlib::be_aas_move::AAS_RocketJumpZVelocity;
use crate::src::botlib::be_aas_sample::AAS_AreaCluster;
use crate::src::botlib::be_aas_sample::AAS_PointAreaNum;
#[no_mangle]

pub static mut debuglines: [libc::c_int; 1024] = [0; 1024];
#[no_mangle]

pub static mut debuglinevisible: [libc::c_int; 1024] = [0; 1024];
#[no_mangle]

pub static mut numdebuglines: libc::c_int = 0;

static mut debugpolygons: [libc::c_int; 8192] = [0; 8192];
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ClearShownPolygons() {
    let mut i: libc::c_int = 0;
    //*
    i = 0 as libc::c_int;
    while i < 8192 as libc::c_int {
        if debugpolygons[i as usize] != 0 {
            crate::src::botlib::be_interface::botimport
                .DebugPolygonDelete
                .expect("non-null function pointer")(debugpolygons[i as usize]);
        }
        debugpolygons[i as usize] = 0 as libc::c_int;
        i += 1
    }
    //end for
    //*/
    /*
        for (i = 0; i < MAX_DEBUGPOLYGONS; i++)
        {
            botimport.DebugPolygonDelete(i);
            debugpolygons[i] = 0;
        } //end for
    */
}
//end of the function AAS_ClearShownPolygons
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ShowPolygon(
    mut color: libc::c_int,
    mut numpoints: libc::c_int,
    mut points: *mut crate::src::qcommon::q_shared::vec3_t,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8192 as libc::c_int {
        if debugpolygons[i as usize] == 0 {
            debugpolygons[i as usize] =
                crate::src::botlib::be_interface::botimport
                    .DebugPolygonCreate
                    .expect("non-null function pointer")(color, numpoints, points);
            break;
        } else {
            i += 1
        }
        //end if
    }
    //end for
}
//end of the function AAS_ShowPolygon
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ClearShownDebugLines() {
    let mut i: libc::c_int = 0;
    //make all lines invisible
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        if debuglines[i as usize] != 0 {
            //botimport.DebugLineShow(debuglines[i], NULL, NULL, LINECOLOR_NONE);
            crate::src::botlib::be_interface::botimport
                .DebugLineDelete
                .expect("non-null function pointer")(debuglines[i as usize]);
            debuglines[i as usize] = 0 as libc::c_int;
            debuglinevisible[i as usize] = crate::src::qcommon::q_shared::qfalse as libc::c_int
        }
        i += 1
        //end if
    }
    //end for
}
//end of the function AAS_ClearShownDebugLines
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_DebugLine(
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut color: libc::c_int,
) {
    let mut line: libc::c_int = 0; //end if
    line = 0 as libc::c_int;
    while line < 1024 as libc::c_int {
        if debuglines[line as usize] == 0 {
            debuglines[line as usize] = crate::src::botlib::be_interface::botimport
                .DebugLineCreate
                .expect("non-null function pointer")();
            debuglinevisible[line as usize] = crate::src::qcommon::q_shared::qfalse as libc::c_int;
            numdebuglines += 1
        }
        if debuglinevisible[line as usize] == 0 {
            crate::src::botlib::be_interface::botimport
                .DebugLineShow
                .expect("non-null function pointer")(
                debuglines[line as usize], start, end, color
            );
            debuglinevisible[line as usize] = crate::src::qcommon::q_shared::qtrue as libc::c_int;
            return;
        }
        line += 1
        //end else
    }
    //end for
}
//end of the function AAS_DebugLine
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_PermanentLine(
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut color: libc::c_int,
) {
    let mut line: libc::c_int = 0;
    line = crate::src::botlib::be_interface::botimport
        .DebugLineCreate
        .expect("non-null function pointer")();
    crate::src::botlib::be_interface::botimport
        .DebugLineShow
        .expect("non-null function pointer")(line, start, end, color);
}
//end of the function AAS_PermenentLine
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_DrawPermanentCross(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut size: libc::c_float,
    mut color: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut debugline: libc::c_int = 0;
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        start[0 as libc::c_int as usize] = *origin.offset(0 as libc::c_int as isize);
        start[1 as libc::c_int as usize] = *origin.offset(1 as libc::c_int as isize);
        start[2 as libc::c_int as usize] = *origin.offset(2 as libc::c_int as isize);
        start[i as usize] += size;
        end[0 as libc::c_int as usize] = *origin.offset(0 as libc::c_int as isize);
        end[1 as libc::c_int as usize] = *origin.offset(1 as libc::c_int as isize);
        end[2 as libc::c_int as usize] = *origin.offset(2 as libc::c_int as isize);
        end[i as usize] -= size;
        AAS_DebugLine(start.as_mut_ptr(), end.as_mut_ptr(), color);
        debugline = crate::src::botlib::be_interface::botimport
            .DebugLineCreate
            .expect("non-null function pointer")();
        crate::src::botlib::be_interface::botimport
            .DebugLineShow
            .expect("non-null function pointer")(
            debugline,
            start.as_mut_ptr(),
            end.as_mut_ptr(),
            color,
        );
        i += 1
    }
    //end for
}
//end of the function AAS_DrawPermanentCross
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_DrawPlaneCross(
    mut point: *mut crate::src::qcommon::q_shared::vec_t,
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
    mut dist: libc::c_float,
    mut type_0: libc::c_int,
    mut color: libc::c_int,
) {
    let mut n0: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut line: libc::c_int = 0;
    let mut lines: [libc::c_int; 2] = [0; 2];
    let mut start1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    //make a cross in the hit plane at the hit point
    start1[0 as libc::c_int as usize] = *point.offset(0 as libc::c_int as isize); //end for
    start1[1 as libc::c_int as usize] = *point.offset(1 as libc::c_int as isize); //end if
    start1[2 as libc::c_int as usize] = *point.offset(2 as libc::c_int as isize);
    end1[0 as libc::c_int as usize] = *point.offset(0 as libc::c_int as isize);
    end1[1 as libc::c_int as usize] = *point.offset(1 as libc::c_int as isize);
    end1[2 as libc::c_int as usize] = *point.offset(2 as libc::c_int as isize);
    start2[0 as libc::c_int as usize] = *point.offset(0 as libc::c_int as isize);
    start2[1 as libc::c_int as usize] = *point.offset(1 as libc::c_int as isize);
    start2[2 as libc::c_int as usize] = *point.offset(2 as libc::c_int as isize);
    end2[0 as libc::c_int as usize] = *point.offset(0 as libc::c_int as isize);
    end2[1 as libc::c_int as usize] = *point.offset(1 as libc::c_int as isize);
    end2[2 as libc::c_int as usize] = *point.offset(2 as libc::c_int as isize);
    n0 = type_0 % 3 as libc::c_int;
    n1 = (type_0 + 1 as libc::c_int) % 3 as libc::c_int;
    n2 = (type_0 + 2 as libc::c_int) % 3 as libc::c_int;
    start1[n1 as usize] -= 6 as libc::c_int as libc::c_float;
    start1[n2 as usize] -= 6 as libc::c_int as libc::c_float;
    end1[n1 as usize] += 6 as libc::c_int as libc::c_float;
    end1[n2 as usize] += 6 as libc::c_int as libc::c_float;
    start2[n1 as usize] += 6 as libc::c_int as libc::c_float;
    start2[n2 as usize] -= 6 as libc::c_int as libc::c_float;
    end2[n1 as usize] -= 6 as libc::c_int as libc::c_float;
    end2[n2 as usize] += 6 as libc::c_int as libc::c_float;
    start1[n0 as usize] = (dist
        - (start1[n1 as usize] * *normal.offset(n1 as isize)
            + start1[n2 as usize] * *normal.offset(n2 as isize)))
        / *normal.offset(n0 as isize);
    end1[n0 as usize] = (dist
        - (end1[n1 as usize] * *normal.offset(n1 as isize)
            + end1[n2 as usize] * *normal.offset(n2 as isize)))
        / *normal.offset(n0 as isize);
    start2[n0 as usize] = (dist
        - (start2[n1 as usize] * *normal.offset(n1 as isize)
            + start2[n2 as usize] * *normal.offset(n2 as isize)))
        / *normal.offset(n0 as isize);
    end2[n0 as usize] = (dist
        - (end2[n1 as usize] * *normal.offset(n1 as isize)
            + end2[n2 as usize] * *normal.offset(n2 as isize)))
        / *normal.offset(n0 as isize);
    j = 0 as libc::c_int;
    line = 0 as libc::c_int;
    while j < 2 as libc::c_int && line < 1024 as libc::c_int {
        if debuglines[line as usize] == 0 {
            debuglines[line as usize] = crate::src::botlib::be_interface::botimport
                .DebugLineCreate
                .expect("non-null function pointer")();
            let fresh0 = j;
            j = j + 1;
            lines[fresh0 as usize] = debuglines[line as usize];
            debuglinevisible[line as usize] = crate::src::qcommon::q_shared::qtrue as libc::c_int;
            numdebuglines += 1
        } else if debuglinevisible[line as usize] == 0 {
            let fresh1 = j;
            j = j + 1;
            lines[fresh1 as usize] = debuglines[line as usize];
            debuglinevisible[line as usize] = crate::src::qcommon::q_shared::qtrue as libc::c_int
        }
        line += 1
        //end else
    }
    crate::src::botlib::be_interface::botimport
        .DebugLineShow
        .expect("non-null function pointer")(
        lines[0 as libc::c_int as usize],
        start1.as_mut_ptr(),
        end1.as_mut_ptr(),
        color,
    );
    crate::src::botlib::be_interface::botimport
        .DebugLineShow
        .expect("non-null function pointer")(
        lines[1 as libc::c_int as usize],
        start2.as_mut_ptr(),
        end2.as_mut_ptr(),
        color,
    );
}
//end of the function AAS_DrawPlaneCross
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ShowBoundingBox(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut bboxcorners: [crate::src::qcommon::q_shared::vec3_t; 8] = [[0.; 3]; 8];
    let mut lines: [libc::c_int; 3] = [0; 3];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut line: libc::c_int = 0;
    //upper corners
    bboxcorners[0 as libc::c_int as usize][0 as libc::c_int as usize] =
        *origin.offset(0 as libc::c_int as isize) + *maxs.offset(0 as libc::c_int as isize);
    bboxcorners[0 as libc::c_int as usize][1 as libc::c_int as usize] =
        *origin.offset(1 as libc::c_int as isize) + *maxs.offset(1 as libc::c_int as isize);
    bboxcorners[0 as libc::c_int as usize][2 as libc::c_int as usize] =
        *origin.offset(2 as libc::c_int as isize) + *maxs.offset(2 as libc::c_int as isize);
    //
    bboxcorners[1 as libc::c_int as usize][0 as libc::c_int as usize] =
        *origin.offset(0 as libc::c_int as isize) + *mins.offset(0 as libc::c_int as isize);
    bboxcorners[1 as libc::c_int as usize][1 as libc::c_int as usize] =
        *origin.offset(1 as libc::c_int as isize) + *maxs.offset(1 as libc::c_int as isize);
    bboxcorners[1 as libc::c_int as usize][2 as libc::c_int as usize] =
        *origin.offset(2 as libc::c_int as isize) + *maxs.offset(2 as libc::c_int as isize);
    //
    bboxcorners[2 as libc::c_int as usize][0 as libc::c_int as usize] =
        *origin.offset(0 as libc::c_int as isize) + *mins.offset(0 as libc::c_int as isize);
    bboxcorners[2 as libc::c_int as usize][1 as libc::c_int as usize] =
        *origin.offset(1 as libc::c_int as isize) + *mins.offset(1 as libc::c_int as isize);
    bboxcorners[2 as libc::c_int as usize][2 as libc::c_int as usize] =
        *origin.offset(2 as libc::c_int as isize) + *maxs.offset(2 as libc::c_int as isize);
    //
    bboxcorners[3 as libc::c_int as usize][0 as libc::c_int as usize] =
        *origin.offset(0 as libc::c_int as isize) + *maxs.offset(0 as libc::c_int as isize);
    bboxcorners[3 as libc::c_int as usize][1 as libc::c_int as usize] =
        *origin.offset(1 as libc::c_int as isize) + *mins.offset(1 as libc::c_int as isize);
    bboxcorners[3 as libc::c_int as usize][2 as libc::c_int as usize] =
        *origin.offset(2 as libc::c_int as isize) + *maxs.offset(2 as libc::c_int as isize);
    //lower corners
    crate::stdlib::memcpy(
        bboxcorners[4 as libc::c_int as usize].as_mut_ptr() as *mut libc::c_void,
        bboxcorners[0 as libc::c_int as usize].as_mut_ptr() as *const libc::c_void,
        (::std::mem::size_of::<crate::src::qcommon::q_shared::vec3_t>() as libc::c_ulong)
            .wrapping_mul(4 as libc::c_int as libc::c_ulong),
    );
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        bboxcorners[(4 as libc::c_int + i) as usize][2 as libc::c_int as usize] =
            *origin.offset(2 as libc::c_int as isize) + *mins.offset(2 as libc::c_int as isize);
        i += 1
    }
    //draw bounding box
    i = 0 as libc::c_int; //end for
    while i < 4 as libc::c_int {
        j = 0 as libc::c_int; //end if
        line = 0 as libc::c_int;
        while j < 3 as libc::c_int && line < 1024 as libc::c_int {
            if debuglines[line as usize] == 0 {
                debuglines[line as usize] = crate::src::botlib::be_interface::botimport
                    .DebugLineCreate
                    .expect("non-null function pointer")(
                );
                let fresh2 = j;
                j = j + 1;
                lines[fresh2 as usize] = debuglines[line as usize];
                debuglinevisible[line as usize] =
                    crate::src::qcommon::q_shared::qtrue as libc::c_int;
                numdebuglines += 1
            } else if debuglinevisible[line as usize] == 0 {
                let fresh3 = j;
                j = j + 1;
                lines[fresh3 as usize] = debuglines[line as usize];
                debuglinevisible[line as usize] =
                    crate::src::qcommon::q_shared::qtrue as libc::c_int
            }
            line += 1
            //end else
        }
        //top plane
        crate::src::botlib::be_interface::botimport
            .DebugLineShow
            .expect("non-null function pointer")(
            lines[0 as libc::c_int as usize],
            bboxcorners[i as usize].as_mut_ptr(),
            bboxcorners[(i + 1 as libc::c_int & 3 as libc::c_int) as usize].as_mut_ptr(),
            1 as libc::c_int,
        );
        //bottom plane
        crate::src::botlib::be_interface::botimport
            .DebugLineShow
            .expect("non-null function pointer")(
            lines[1 as libc::c_int as usize],
            bboxcorners[(4 as libc::c_int + i) as usize].as_mut_ptr(),
            bboxcorners[(4 as libc::c_int + (i + 1 as libc::c_int & 3 as libc::c_int)) as usize]
                .as_mut_ptr(),
            1 as libc::c_int,
        );
        //vertical lines
        crate::src::botlib::be_interface::botimport
            .DebugLineShow
            .expect("non-null function pointer")(
            lines[2 as libc::c_int as usize],
            bboxcorners[i as usize].as_mut_ptr(),
            bboxcorners[(4 as libc::c_int + i) as usize].as_mut_ptr(),
            1 as libc::c_int,
        );
        i += 1
    }
    //end for
}
//end of the function AAS_ShowBoundingBox
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ShowFace(mut facenum: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut color: libc::c_int = 0;
    let mut edgenum: libc::c_int = 0;
    let mut edge: *mut crate::aasfile_h::aas_edge_t = 0 as *mut crate::aasfile_h::aas_edge_t;
    let mut face: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut plane: *mut crate::aasfile_h::aas_plane_t = 0 as *mut crate::aasfile_h::aas_plane_t;
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    color = 4 as libc::c_int;
    //check if face number is in range
    if facenum >= crate::src::botlib::be_aas_main::aasworld.numfaces {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as libc::c_int,
            b"facenum %d out of range\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            facenum,
        ); //end if
    }
    face = &mut *crate::src::botlib::be_aas_main::aasworld
        .faces
        .offset(facenum as isize) as *mut crate::aasfile_h::aas_face_t;
    //walk through the edges of the face
    i = 0 as libc::c_int; //end for
    while i < (*face).numedges {
        //edge number
        edgenum = crate::stdlib::abs(
            *crate::src::botlib::be_aas_main::aasworld
                .edgeindex
                .offset(((*face).firstedge + i) as isize),
        );
        //check if edge number is in range
        if edgenum >= crate::src::botlib::be_aas_main::aasworld.numedges {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3 as libc::c_int,
                b"edgenum %d out of range\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                edgenum,
            ); //end if
        }
        edge = &mut *crate::src::botlib::be_aas_main::aasworld
            .edges
            .offset(edgenum as isize) as *mut crate::aasfile_h::aas_edge_t;
        if color == 1 as libc::c_int {
            color = 2 as libc::c_int
        } else if color == 2 as libc::c_int {
            color = 3 as libc::c_int
        } else if color == 3 as libc::c_int {
            color = 4 as libc::c_int
        } else {
            color = 1 as libc::c_int
        }
        AAS_DebugLine(
            (*crate::src::botlib::be_aas_main::aasworld
                .vertexes
                .offset((*edge).v[0 as libc::c_int as usize] as isize))
            .as_mut_ptr(),
            (*crate::src::botlib::be_aas_main::aasworld
                .vertexes
                .offset((*edge).v[1 as libc::c_int as usize] as isize))
            .as_mut_ptr(),
            color,
        );
        i += 1
    }
    plane = &mut *crate::src::botlib::be_aas_main::aasworld
        .planes
        .offset((*face).planenum as isize) as *mut crate::aasfile_h::aas_plane_t;
    edgenum = crate::stdlib::abs(
        *crate::src::botlib::be_aas_main::aasworld
            .edgeindex
            .offset((*face).firstedge as isize),
    );
    edge = &mut *crate::src::botlib::be_aas_main::aasworld
        .edges
        .offset(edgenum as isize) as *mut crate::aasfile_h::aas_edge_t;
    start[0 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
        .vertexes
        .offset((*edge).v[0 as libc::c_int as usize] as isize))[0 as libc::c_int as usize];
    start[1 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
        .vertexes
        .offset((*edge).v[0 as libc::c_int as usize] as isize))[1 as libc::c_int as usize];
    start[2 as libc::c_int as usize] = (*crate::src::botlib::be_aas_main::aasworld
        .vertexes
        .offset((*edge).v[0 as libc::c_int as usize] as isize))[2 as libc::c_int as usize];
    end[0 as libc::c_int as usize] = start[0 as libc::c_int as usize]
        + (*plane).normal[0 as libc::c_int as usize] * 20 as libc::c_int as libc::c_float;
    end[1 as libc::c_int as usize] = start[1 as libc::c_int as usize]
        + (*plane).normal[1 as libc::c_int as usize] * 20 as libc::c_int as libc::c_float;
    end[2 as libc::c_int as usize] = start[2 as libc::c_int as usize]
        + (*plane).normal[2 as libc::c_int as usize] * 20 as libc::c_int as libc::c_float;
    AAS_DebugLine(start.as_mut_ptr(), end.as_mut_ptr(), 1 as libc::c_int);
}
//end of the function AAS_ShowFace
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ShowFacePolygon(
    mut facenum: libc::c_int,
    mut color: libc::c_int,
    mut flip: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut edgenum: libc::c_int = 0;
    let mut numpoints: libc::c_int = 0;
    let mut points: [crate::src::qcommon::q_shared::vec3_t; 128] = [[0.; 3]; 128];
    let mut edge: *mut crate::aasfile_h::aas_edge_t = 0 as *mut crate::aasfile_h::aas_edge_t;
    let mut face: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    //check if face number is in range
    if facenum >= crate::src::botlib::be_aas_main::aasworld.numfaces {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as libc::c_int,
            b"facenum %d out of range\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            facenum,
        ); //end if
    }
    face = &mut *crate::src::botlib::be_aas_main::aasworld
        .faces
        .offset(facenum as isize) as *mut crate::aasfile_h::aas_face_t;
    //walk through the edges of the face
    numpoints = 0 as libc::c_int; //end else
    if flip != 0 {
        i = (*face).numedges - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            //edge number
            edgenum = *crate::src::botlib::be_aas_main::aasworld
                .edgeindex
                .offset(((*face).firstedge + i) as isize);
            edge = &mut *crate::src::botlib::be_aas_main::aasworld
                .edges
                .offset(
                    (crate::stdlib::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(
                        edgenum,
                    ) as isize,
                ) as *mut crate::aasfile_h::aas_edge_t;
            points[numpoints as usize][0 as libc::c_int as usize] =
                (*crate::src::botlib::be_aas_main::aasworld.vertexes.offset(
                    (*edge).v[(edgenum < 0 as libc::c_int) as libc::c_int as usize] as isize,
                ))[0 as libc::c_int as usize];
            points[numpoints as usize][1 as libc::c_int as usize] =
                (*crate::src::botlib::be_aas_main::aasworld.vertexes.offset(
                    (*edge).v[(edgenum < 0 as libc::c_int) as libc::c_int as usize] as isize,
                ))[1 as libc::c_int as usize];
            points[numpoints as usize][2 as libc::c_int as usize] =
                (*crate::src::botlib::be_aas_main::aasworld.vertexes.offset(
                    (*edge).v[(edgenum < 0 as libc::c_int) as libc::c_int as usize] as isize,
                ))[2 as libc::c_int as usize];
            numpoints += 1;
            i -= 1
        }
    //end for
    } else {
        i = 0 as libc::c_int;
        while i < (*face).numedges {
            //edge number
            edgenum = *crate::src::botlib::be_aas_main::aasworld
                .edgeindex
                .offset(((*face).firstedge + i) as isize);
            edge = &mut *crate::src::botlib::be_aas_main::aasworld
                .edges
                .offset(
                    (crate::stdlib::abs as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)(
                        edgenum,
                    ) as isize,
                ) as *mut crate::aasfile_h::aas_edge_t;
            points[numpoints as usize][0 as libc::c_int as usize] =
                (*crate::src::botlib::be_aas_main::aasworld.vertexes.offset(
                    (*edge).v[(edgenum < 0 as libc::c_int) as libc::c_int as usize] as isize,
                ))[0 as libc::c_int as usize];
            points[numpoints as usize][1 as libc::c_int as usize] =
                (*crate::src::botlib::be_aas_main::aasworld.vertexes.offset(
                    (*edge).v[(edgenum < 0 as libc::c_int) as libc::c_int as usize] as isize,
                ))[1 as libc::c_int as usize];
            points[numpoints as usize][2 as libc::c_int as usize] =
                (*crate::src::botlib::be_aas_main::aasworld.vertexes.offset(
                    (*edge).v[(edgenum < 0 as libc::c_int) as libc::c_int as usize] as isize,
                ))[2 as libc::c_int as usize];
            numpoints += 1;
            i += 1
        }
        //end for
    }
    AAS_ShowPolygon(color, numpoints, points.as_mut_ptr());
}
//end of the function AAS_ShowFacePolygon
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ShowArea(mut areanum: libc::c_int, mut groundfacesonly: libc::c_int) {
    let mut areaedges: [libc::c_int; 1024] = [0; 1024];
    let mut numareaedges: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut color: libc::c_int = 0 as libc::c_int;
    let mut line: libc::c_int = 0;
    let mut facenum: libc::c_int = 0;
    let mut edgenum: libc::c_int = 0;
    let mut area: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut face: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut edge: *mut crate::aasfile_h::aas_edge_t = 0 as *mut crate::aasfile_h::aas_edge_t;
    //
    numareaedges = 0 as libc::c_int;
    //
    if areanum < 0 as libc::c_int || areanum >= crate::src::botlib::be_aas_main::aasworld.numareas {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as libc::c_int,
            b"area %d out of range [0, %d]\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            areanum,
            crate::src::botlib::be_aas_main::aasworld.numareas,
        ); //end if
        return;
    }
    //pointer to the convex area
    area = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize) as *mut crate::aasfile_h::aas_area_t;
    let mut current_block_23: u64;
    //walk through the faces of the area
    i = 0 as libc::c_int; //end for
    while i < (*area).numfaces {
        facenum = crate::stdlib::abs(
            *crate::src::botlib::be_aas_main::aasworld
                .faceindex
                .offset(((*area).firstface + i) as isize),
        );
        //end for
        //AAS_ShowFace(facenum);
        //check if face number is in range
        if facenum >= crate::src::botlib::be_aas_main::aasworld.numfaces {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3 as libc::c_int,
                b"facenum %d out of range\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                facenum,
            ); //end if
        }
        face = &mut *crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(facenum as isize) as *mut crate::aasfile_h::aas_face_t;
        //ground faces only
        if groundfacesonly != 0 {
            if (*face).faceflags & (4 as libc::c_int | 2 as libc::c_int) == 0 {
                current_block_23 = 3276175668257526147; //end if
            } else {
                current_block_23 = 15652330335145281839;
            }
        } else {
            current_block_23 = 15652330335145281839;
        }
        match current_block_23 {
            15652330335145281839 => {
                //walk through the edges of the face
                j = 0 as libc::c_int;
                while j < (*face).numedges {
                    //edge number
                    edgenum = crate::stdlib::abs(
                        *crate::src::botlib::be_aas_main::aasworld
                            .edgeindex
                            .offset(((*face).firstedge + j) as isize),
                    );
                    //end if
                    if edgenum >= crate::src::botlib::be_aas_main::aasworld.numedges {
                        crate::src::botlib::be_interface::botimport
                            .Print
                            .expect("non-null function pointer")(
                            3 as libc::c_int,
                            b"edgenum %d out of range\n\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            edgenum,
                        );
                    }
                    n = 0 as libc::c_int;
                    while n < numareaedges {
                        if areaedges[n as usize] == edgenum {
                            break;
                        }
                        n += 1
                    }
                    if n == numareaedges && numareaedges < 1024 as libc::c_int {
                        let fresh4 = numareaedges;
                        numareaedges = numareaedges + 1;
                        areaedges[fresh4 as usize] = edgenum
                    }
                    j += 1
                }
            }
            _ => {}
        }
        i += 1
    }
    //check if edge number is in range
    //end if
    //check if the edge is stored already
    //end for
    //draw all the edges
    n = 0 as libc::c_int; //end for
    while n < numareaedges {
        line = 0 as libc::c_int; //end if
        while line < 1024 as libc::c_int {
            if debuglines[line as usize] == 0 {
                debuglines[line as usize] = crate::src::botlib::be_interface::botimport
                    .DebugLineCreate
                    .expect("non-null function pointer")(
                );
                debuglinevisible[line as usize] =
                    crate::src::qcommon::q_shared::qfalse as libc::c_int;
                numdebuglines += 1
            }
            if debuglinevisible[line as usize] == 0 {
                break;
            }
            line += 1
            //end else
        }
        if line >= 1024 as libc::c_int {
            return;
        }
        edge = &mut *crate::src::botlib::be_aas_main::aasworld
            .edges
            .offset(*areaedges.as_mut_ptr().offset(n as isize) as isize)
            as *mut crate::aasfile_h::aas_edge_t;
        if color == 1 as libc::c_int {
            color = 3 as libc::c_int
        } else if color == 3 as libc::c_int {
            color = 2 as libc::c_int
        } else if color == 2 as libc::c_int {
            color = 4 as libc::c_int
        } else {
            color = 1 as libc::c_int
        }
        crate::src::botlib::be_interface::botimport
            .DebugLineShow
            .expect("non-null function pointer")(
            debuglines[line as usize],
            (*crate::src::botlib::be_aas_main::aasworld
                .vertexes
                .offset((*edge).v[0 as libc::c_int as usize] as isize))
            .as_mut_ptr(),
            (*crate::src::botlib::be_aas_main::aasworld
                .vertexes
                .offset((*edge).v[1 as libc::c_int as usize] as isize))
            .as_mut_ptr(),
            color,
        );
        debuglinevisible[line as usize] = crate::src::qcommon::q_shared::qtrue as libc::c_int;
        n += 1
    }
    //end for*/
}
//end of the function AAS_ShowArea
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ShowAreaPolygons(
    mut areanum: libc::c_int,
    mut color: libc::c_int,
    mut groundfacesonly: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut facenum: libc::c_int = 0;
    let mut area: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut face: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    //
    if areanum < 0 as libc::c_int || areanum >= crate::src::botlib::be_aas_main::aasworld.numareas {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3 as libc::c_int,
            b"area %d out of range [0, %d]\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            areanum,
            crate::src::botlib::be_aas_main::aasworld.numareas,
        ); //end if
        return;
    }
    //pointer to the convex area
    area = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize) as *mut crate::aasfile_h::aas_area_t;
    let mut current_block_11: u64;
    //walk through the faces of the area
    i = 0 as libc::c_int;
    while i < (*area).numfaces {
        facenum = crate::stdlib::abs(
            *crate::src::botlib::be_aas_main::aasworld
                .faceindex
                .offset(((*area).firstface + i) as isize),
        );
        //check if face number is in range
        if facenum >= crate::src::botlib::be_aas_main::aasworld.numfaces {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3 as libc::c_int,
                b"facenum %d out of range\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                facenum,
            ); //end if
        }
        face = &mut *crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(facenum as isize) as *mut crate::aasfile_h::aas_face_t;
        //ground faces only
        if groundfacesonly != 0 {
            if (*face).faceflags & (4 as libc::c_int | 2 as libc::c_int) == 0 {
                current_block_11 = 2473556513754201174; //end if
            } else {
                current_block_11 = 8236137900636309791;
            }
        } else {
            current_block_11 = 8236137900636309791;
        }
        match current_block_11 {
            8236137900636309791 => {
                AAS_ShowFacePolygon(
                    facenum,
                    color,
                    ((*face).frontarea != areanum) as libc::c_int,
                );
            }
            _ => {}
        }
        i += 1
    }
    //end for
}
//end of the function AAS_ShowAreaPolygons
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_DrawCross(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut size: libc::c_float,
    mut color: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        start[0 as libc::c_int as usize] = *origin.offset(0 as libc::c_int as isize);
        start[1 as libc::c_int as usize] = *origin.offset(1 as libc::c_int as isize);
        start[2 as libc::c_int as usize] = *origin.offset(2 as libc::c_int as isize);
        start[i as usize] += size;
        end[0 as libc::c_int as usize] = *origin.offset(0 as libc::c_int as isize);
        end[1 as libc::c_int as usize] = *origin.offset(1 as libc::c_int as isize);
        end[2 as libc::c_int as usize] = *origin.offset(2 as libc::c_int as isize);
        end[i as usize] -= size;
        AAS_DebugLine(start.as_mut_ptr(), end.as_mut_ptr(), color);
        i += 1
    }
    //end for
}
//end of the function AAS_DrawCross
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_PrintTravelType(mut traveltype: libc::c_int) {}
//end of the function AAS_PrintTravelType
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_DrawArrow(
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut linecolor: libc::c_int,
    mut arrowcolor: libc::c_int,
) {
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut cross: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
        1 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    ];
    let mut dot: libc::c_float = 0.;
    dir[0 as libc::c_int as usize] =
        *end.offset(0 as libc::c_int as isize) - *start.offset(0 as libc::c_int as isize);
    dir[1 as libc::c_int as usize] =
        *end.offset(1 as libc::c_int as isize) - *start.offset(1 as libc::c_int as isize);
    dir[2 as libc::c_int as usize] =
        *end.offset(2 as libc::c_int as isize) - *start.offset(2 as libc::c_int as isize);
    crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
    dot = dir[0 as libc::c_int as usize] * up[0 as libc::c_int as usize]
        + dir[1 as libc::c_int as usize] * up[1 as libc::c_int as usize]
        + dir[2 as libc::c_int as usize] * up[2 as libc::c_int as usize];
    if dot as libc::c_double > 0.99f64 || (dot as libc::c_double) < -0.99f64 {
        cross[0 as libc::c_int as usize] = 1 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        cross[1 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        cross[2 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t
    } else {
        CrossProduct(
            dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            up.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            cross.as_mut_ptr(),
        );
    }
    p1[0 as libc::c_int as usize] = *end.offset(0 as libc::c_int as isize)
        + dir[0 as libc::c_int as usize] * -(6 as libc::c_int) as libc::c_float;
    p1[1 as libc::c_int as usize] = *end.offset(1 as libc::c_int as isize)
        + dir[1 as libc::c_int as usize] * -(6 as libc::c_int) as libc::c_float;
    p1[2 as libc::c_int as usize] = *end.offset(2 as libc::c_int as isize)
        + dir[2 as libc::c_int as usize] * -(6 as libc::c_int) as libc::c_float;
    p2[0 as libc::c_int as usize] = p1[0 as libc::c_int as usize];
    p2[1 as libc::c_int as usize] = p1[1 as libc::c_int as usize];
    p2[2 as libc::c_int as usize] = p1[2 as libc::c_int as usize];
    p1[0 as libc::c_int as usize] = p1[0 as libc::c_int as usize]
        + cross[0 as libc::c_int as usize] * 6 as libc::c_int as libc::c_float;
    p1[1 as libc::c_int as usize] = p1[1 as libc::c_int as usize]
        + cross[1 as libc::c_int as usize] * 6 as libc::c_int as libc::c_float;
    p1[2 as libc::c_int as usize] = p1[2 as libc::c_int as usize]
        + cross[2 as libc::c_int as usize] * 6 as libc::c_int as libc::c_float;
    p2[0 as libc::c_int as usize] = p2[0 as libc::c_int as usize]
        + cross[0 as libc::c_int as usize] * -(6 as libc::c_int) as libc::c_float;
    p2[1 as libc::c_int as usize] = p2[1 as libc::c_int as usize]
        + cross[1 as libc::c_int as usize] * -(6 as libc::c_int) as libc::c_float;
    p2[2 as libc::c_int as usize] = p2[2 as libc::c_int as usize]
        + cross[2 as libc::c_int as usize] * -(6 as libc::c_int) as libc::c_float;
    AAS_DebugLine(start, end, linecolor);
    AAS_DebugLine(p1.as_mut_ptr(), end, arrowcolor);
    AAS_DebugLine(p2.as_mut_ptr(), end, arrowcolor);
}
//end of the function AAS_DrawArrow
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ShowReachability(
    mut reach: *mut crate::aasfile_h::aas_reachability_t,
) {
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut cmdmove: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut speed: libc::c_float = 0.;
    let mut zvel: libc::c_float = 0.;
    let mut move_0: crate::be_aas_h::aas_clientmove_t = crate::be_aas_h::aas_clientmove_t {
        endpos: [0.; 3],
        endarea: 0,
        velocity: [0.; 3],
        trace: crate::be_aas_h::aas_trace_t {
            startsolid: crate::src::qcommon::q_shared::qfalse,
            fraction: 0.,
            endpos: [0.; 3],
            ent: 0,
            lastarea: 0,
            area: 0,
            planenum: 0,
        },
        presencetype: 0,
        stopevent: 0,
        endcontents: 0,
        time: 0.,
        frames: 0,
    };
    AAS_ShowAreaPolygons(
        (*reach).areanum,
        5 as libc::c_int,
        crate::src::qcommon::q_shared::qtrue as libc::c_int,
    );
    //AAS_ShowArea(reach->areanum, qtrue);
    AAS_DrawArrow(
        (*reach).start.as_mut_ptr(),
        (*reach).end.as_mut_ptr(),
        3 as libc::c_int,
        4 as libc::c_int,
    );
    //
    if (*reach).traveltype & 0xffffff as libc::c_int == 5 as libc::c_int
        || (*reach).traveltype & 0xffffff as libc::c_int == 7 as libc::c_int
    {
        crate::src::botlib::be_aas_move::AAS_HorizontalVelocityForJump(
            crate::src::botlib::be_aas_move::aassettings.phys_jumpvel,
            (*reach).start.as_mut_ptr(),
            (*reach).end.as_mut_ptr(),
            &mut speed,
        );
        //end if
        dir[0 as libc::c_int as usize] =
            (*reach).end[0 as libc::c_int as usize] - (*reach).start[0 as libc::c_int as usize];
        dir[1 as libc::c_int as usize] =
            (*reach).end[1 as libc::c_int as usize] - (*reach).start[1 as libc::c_int as usize];
        dir[2 as libc::c_int as usize] =
            (*reach).end[2 as libc::c_int as usize] - (*reach).start[2 as libc::c_int as usize];
        dir[2 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
        velocity[0 as libc::c_int as usize] = dir[0 as libc::c_int as usize] * speed;
        velocity[1 as libc::c_int as usize] = dir[1 as libc::c_int as usize] * speed;
        velocity[2 as libc::c_int as usize] = dir[2 as libc::c_int as usize] * speed;
        cmdmove[2 as libc::c_int as usize] =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        cmdmove[1 as libc::c_int as usize] = cmdmove[2 as libc::c_int as usize];
        cmdmove[0 as libc::c_int as usize] = cmdmove[1 as libc::c_int as usize];
        cmdmove[2 as libc::c_int as usize] =
            crate::src::botlib::be_aas_move::aassettings.phys_jumpvel;
        crate::src::botlib::be_aas_move::AAS_PredictClientMovement(
            &mut move_0,
            -(1 as libc::c_int),
            (*reach).start.as_mut_ptr(),
            2 as libc::c_int,
            crate::src::qcommon::q_shared::qtrue as libc::c_int,
            velocity.as_mut_ptr(),
            cmdmove.as_mut_ptr(),
            3 as libc::c_int,
            30 as libc::c_int,
            0.1f32,
            1 as libc::c_int
                | 4 as libc::c_int
                | 8 as libc::c_int
                | 16 as libc::c_int
                | 32 as libc::c_int,
            0 as libc::c_int,
            crate::src::qcommon::q_shared::qtrue as libc::c_int,
        );
        if (*reach).traveltype & 0xffffff as libc::c_int == 5 as libc::c_int {
            crate::src::botlib::be_aas_move::AAS_JumpReachRunStart(reach, dir.as_mut_ptr());
            AAS_DrawCross(
                dir.as_mut_ptr(),
                4 as libc::c_int as libc::c_float,
                3 as libc::c_int,
            );
        }
    } else if (*reach).traveltype & 0xffffff as libc::c_int == 12 as libc::c_int {
        //
        //set the velocity
        //set the command movement
        //
        //
        zvel =
            crate::src::botlib::be_aas_move::AAS_RocketJumpZVelocity((*reach).start.as_mut_ptr()); //end else if
        crate::src::botlib::be_aas_move::AAS_HorizontalVelocityForJump(
            zvel,
            (*reach).start.as_mut_ptr(),
            (*reach).end.as_mut_ptr(),
            &mut speed,
        );
        //
        dir[0 as libc::c_int as usize] =
            (*reach).end[0 as libc::c_int as usize] - (*reach).start[0 as libc::c_int as usize];
        dir[1 as libc::c_int as usize] =
            (*reach).end[1 as libc::c_int as usize] - (*reach).start[1 as libc::c_int as usize];
        dir[2 as libc::c_int as usize] =
            (*reach).end[2 as libc::c_int as usize] - (*reach).start[2 as libc::c_int as usize];
        dir[2 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
        //get command movement
        cmdmove[0 as libc::c_int as usize] = dir[0 as libc::c_int as usize] * speed;
        cmdmove[1 as libc::c_int as usize] = dir[1 as libc::c_int as usize] * speed;
        cmdmove[2 as libc::c_int as usize] = dir[2 as libc::c_int as usize] * speed;
        velocity[0 as libc::c_int as usize] =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        velocity[1 as libc::c_int as usize] =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        velocity[2 as libc::c_int as usize] = zvel;
        //
        crate::src::botlib::be_aas_move::AAS_PredictClientMovement(
            &mut move_0,
            -(1 as libc::c_int),
            (*reach).start.as_mut_ptr(),
            2 as libc::c_int,
            crate::src::qcommon::q_shared::qtrue as libc::c_int,
            velocity.as_mut_ptr(),
            cmdmove.as_mut_ptr(),
            30 as libc::c_int,
            30 as libc::c_int,
            0.1f32,
            4 as libc::c_int
                | 8 as libc::c_int
                | 16 as libc::c_int
                | 32 as libc::c_int
                | 128 as libc::c_int
                | 1024 as libc::c_int,
            (*reach).areanum,
            crate::src::qcommon::q_shared::qtrue as libc::c_int,
        );
    } else if (*reach).traveltype & 0xffffff as libc::c_int == 18 as libc::c_int {
        cmdmove[0 as libc::c_int as usize] =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        cmdmove[1 as libc::c_int as usize] =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        cmdmove[2 as libc::c_int as usize] =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        //
        dir[0 as libc::c_int as usize] =
            (*reach).end[0 as libc::c_int as usize] - (*reach).start[0 as libc::c_int as usize];
        dir[1 as libc::c_int as usize] =
            (*reach).end[1 as libc::c_int as usize] - (*reach).start[1 as libc::c_int as usize];
        dir[2 as libc::c_int as usize] =
            (*reach).end[2 as libc::c_int as usize] - (*reach).start[2 as libc::c_int as usize];
        dir[2 as libc::c_int as usize] = 0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
        //set the velocity
        //NOTE: the edgenum is the horizontal velocity
        velocity[0 as libc::c_int as usize] =
            dir[0 as libc::c_int as usize] * (*reach).edgenum as libc::c_float;
        velocity[1 as libc::c_int as usize] =
            dir[1 as libc::c_int as usize] * (*reach).edgenum as libc::c_float;
        velocity[2 as libc::c_int as usize] =
            dir[2 as libc::c_int as usize] * (*reach).edgenum as libc::c_float;
        //NOTE: the facenum is the Z velocity
        velocity[2 as libc::c_int as usize] =
            (*reach).facenum as crate::src::qcommon::q_shared::vec_t;
        //
        crate::src::botlib::be_aas_move::AAS_PredictClientMovement(
            &mut move_0,
            -(1 as libc::c_int),
            (*reach).start.as_mut_ptr(),
            2 as libc::c_int,
            crate::src::qcommon::q_shared::qtrue as libc::c_int,
            velocity.as_mut_ptr(),
            cmdmove.as_mut_ptr(),
            30 as libc::c_int,
            30 as libc::c_int,
            0.1f32,
            4 as libc::c_int
                | 8 as libc::c_int
                | 16 as libc::c_int
                | 32 as libc::c_int
                | 128 as libc::c_int
                | 1024 as libc::c_int,
            (*reach).areanum,
            crate::src::qcommon::q_shared::qtrue as libc::c_int,
        );
    };
    //end else if
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
/* ****************************************************************************
 * name:		be_aas_debug.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_debug.h $
 *
 *****************************************************************************/
/* ****************************************************************************
 * name:		be_aas_debug.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_debug.h $
 *
 *****************************************************************************/
//clear the shown debug lines
//clear the shown debug lines
//
//
//show a debug line
//show a debug line
//show a permenent line
//show a permenent line
//show a permanent cross
//show a permanent cross
//draw a cross in the plane
//draw a cross in the plane
//show a bounding box
//show a bounding box
//show a face
//show a face
//show an area
//show an area
//
//
//draw a cros
//draw a cros
//print the travel type
//print the travel type
//draw an arrow
//draw an arrow
//visualize the given reachability
//visualize the given reachability
//show the reachable areas from the given area
//show the reachable areas from the given area
//end of the function AAS_ShowReachability
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn AAS_ShowReachableAreas(mut areanum: libc::c_int) {
    let mut settings: *mut crate::aasfile_h::aas_areasettings_t =
        0 as *mut crate::aasfile_h::aas_areasettings_t; //end if
    static mut reach: crate::aasfile_h::aas_reachability_t = crate::aasfile_h::aas_reachability_t {
        areanum: 0,
        facenum: 0,
        edgenum: 0,
        start: [0.; 3],
        end: [0.; 3],
        traveltype: 0,
        traveltime: 0,
    };
    static mut index: libc::c_int = 0;
    static mut lastareanum: libc::c_int = 0;
    static mut lasttime: libc::c_float = 0.;
    if areanum != lastareanum {
        index = 0 as libc::c_int;
        lastareanum = areanum
    }
    settings = &mut *crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize) as *mut crate::aasfile_h::aas_areasettings_t;
    //
    if (*settings).numreachableareas == 0 {
        return;
    }
    //
    if index >= (*settings).numreachableareas {
        index = 0 as libc::c_int
    }
    //
    if (crate::src::botlib::be_aas_main::AAS_Time() - lasttime) as libc::c_double > 1.5f64 {
        crate::stdlib::memcpy(
            &mut reach as *mut crate::aasfile_h::aas_reachability_t as *mut libc::c_void,
            &mut *crate::src::botlib::be_aas_main::aasworld
                .reachability
                .offset(((*settings).firstreachablearea + index) as isize)
                as *mut crate::aasfile_h::aas_reachability_t as *const libc::c_void,
            ::std::mem::size_of::<crate::aasfile_h::aas_reachability_t>() as libc::c_ulong,
        ); //end if
        index += 1;
        lasttime = crate::src::botlib::be_aas_main::AAS_Time();
        AAS_PrintTravelType(reach.traveltype & 0xffffff as libc::c_int);
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            1 as libc::c_int,
            b"\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    AAS_ShowReachability(&mut reach);
}
//end of the function ShowReachableAreas
#[no_mangle]

pub unsafe extern "C" fn AAS_FloodAreas_r(
    mut areanum: libc::c_int,
    mut cluster: libc::c_int,
    mut done: *mut libc::c_int,
) {
    let mut nextareanum: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut facenum: libc::c_int = 0;
    let mut area: *mut crate::aasfile_h::aas_area_t = 0 as *mut crate::aasfile_h::aas_area_t;
    let mut face: *mut crate::aasfile_h::aas_face_t = 0 as *mut crate::aasfile_h::aas_face_t;
    let mut settings: *mut crate::aasfile_h::aas_areasettings_t =
        0 as *mut crate::aasfile_h::aas_areasettings_t;
    let mut reach: *mut crate::aasfile_h::aas_reachability_t =
        0 as *mut crate::aasfile_h::aas_reachability_t;
    AAS_ShowAreaPolygons(
        areanum,
        1 as libc::c_int,
        crate::src::qcommon::q_shared::qtrue as libc::c_int,
    );
    //pointer to the convex area
    area = &mut *crate::src::botlib::be_aas_main::aasworld
        .areas
        .offset(areanum as isize) as *mut crate::aasfile_h::aas_area_t;
    settings = &mut *crate::src::botlib::be_aas_main::aasworld
        .areasettings
        .offset(areanum as isize) as *mut crate::aasfile_h::aas_areasettings_t;
    //walk through the faces of the area
    i = 0 as libc::c_int; //end for
    while i < (*area).numfaces {
        facenum = crate::stdlib::abs(
            *crate::src::botlib::be_aas_main::aasworld
                .faceindex
                .offset(((*area).firstface + i) as isize),
        );
        face = &mut *crate::src::botlib::be_aas_main::aasworld
            .faces
            .offset(facenum as isize) as *mut crate::aasfile_h::aas_face_t;
        if (*face).frontarea == areanum {
            nextareanum = (*face).backarea
        } else {
            nextareanum = (*face).frontarea
        }
        if !(nextareanum == 0) {
            if !(*done.offset(nextareanum as isize) != 0) {
                *done.offset(nextareanum as isize) =
                    crate::src::qcommon::q_shared::qtrue as libc::c_int;
                if !((*crate::src::botlib::be_aas_main::aasworld
                    .areasettings
                    .offset(nextareanum as isize))
                .contents
                    & 512 as libc::c_int
                    != 0)
                {
                    if !(crate::src::botlib::be_aas_sample::AAS_AreaCluster(nextareanum) != cluster)
                    {
                        AAS_FloodAreas_r(nextareanum, cluster, done);
                    }
                }
            }
        }
        i += 1
    }
    //
    i = 0 as libc::c_int;
    while i < (*settings).numreachableareas {
        reach = &mut *crate::src::botlib::be_aas_main::aasworld
            .reachability
            .offset(((*settings).firstreachablearea + i) as isize)
            as *mut crate::aasfile_h::aas_reachability_t;
        nextareanum = (*reach).areanum;
        if !(nextareanum == 0) {
            if !(*done.offset(nextareanum as isize) != 0) {
                *done.offset(nextareanum as isize) =
                    crate::src::qcommon::q_shared::qtrue as libc::c_int;
                if !((*crate::src::botlib::be_aas_main::aasworld
                    .areasettings
                    .offset(nextareanum as isize))
                .contents
                    & 512 as libc::c_int
                    != 0)
                {
                    if !(crate::src::botlib::be_aas_sample::AAS_AreaCluster(nextareanum) != cluster)
                    {
                        /*
                        if ((reach->traveltype & TRAVELTYPE_MASK) == TRAVEL_WALKOFFLEDGE)
                        {
                            AAS_DebugLine(reach->start, reach->end, 1);
                        }
                        */
                        AAS_FloodAreas_r(nextareanum, cluster, done);
                    }
                }
            }
        }
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn AAS_FloodAreas(mut origin: *mut crate::src::qcommon::q_shared::vec_t) {
    let mut areanum: libc::c_int = 0;
    let mut cluster: libc::c_int = 0;
    let mut done: *mut libc::c_int = 0 as *mut libc::c_int;
    done = crate::src::botlib::l_memory::GetClearedMemory(
        (crate::src::botlib::be_aas_main::aasworld.numareas as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    areanum = crate::src::botlib::be_aas_sample::AAS_PointAreaNum(origin);
    cluster = crate::src::botlib::be_aas_sample::AAS_AreaCluster(areanum);
    AAS_FloodAreas_r(areanum, cluster, done);
    crate::src::botlib::l_memory::FreeMemory(done as *mut libc::c_void);
}
