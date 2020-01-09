#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(asm)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(main)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]

pub mod cm_local_h {
    pub type leafList_t = crate::cm_local_h::leafList_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct leafList_s {
        pub count: libc::c_int,
        pub maxcount: libc::c_int,
        pub overflowed: crate::src::qcommon::q_shared::qboolean,
        pub list: *mut libc::c_int,
        pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
        pub lastLeaf: libc::c_int,
        pub storeLeafs: Option<
            unsafe extern "C" fn(_: *mut crate::cm_local_h::leafList_s, _: libc::c_int) -> (),
        >,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct sphere_t {
        pub use_0: crate::src::qcommon::q_shared::qboolean,
        pub radius: libc::c_float,
        pub halfheight: libc::c_float,
        pub offset: crate::src::qcommon::q_shared::vec3_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct traceWork_t {
        pub start: crate::src::qcommon::q_shared::vec3_t,
        pub end: crate::src::qcommon::q_shared::vec3_t,
        pub size: [crate::src::qcommon::q_shared::vec3_t; 2],
        pub offsets: [crate::src::qcommon::q_shared::vec3_t; 8],
        pub maxOffset: libc::c_float,
        pub extents: crate::src::qcommon::q_shared::vec3_t,
        pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
        pub modelOrigin: crate::src::qcommon::q_shared::vec3_t,
        pub contents: libc::c_int,
        pub isPoint: crate::src::qcommon::q_shared::qboolean,
        pub trace: crate::src::qcommon::q_shared::trace_t,
        pub sphere: crate::cm_local_h::sphere_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct clipMap_t {
        pub name: [libc::c_char; 64],
        pub numShaders: libc::c_int,
        pub shaders: *mut crate::qfiles_h::dshader_t,
        pub numBrushSides: libc::c_int,
        pub brushsides: *mut crate::cm_local_h::cbrushside_t,
        pub numPlanes: libc::c_int,
        pub planes: *mut crate::src::qcommon::q_shared::cplane_t,
        pub numNodes: libc::c_int,
        pub nodes: *mut crate::cm_local_h::cNode_t,
        pub numLeafs: libc::c_int,
        pub leafs: *mut crate::cm_local_h::cLeaf_t,
        pub numLeafBrushes: libc::c_int,
        pub leafbrushes: *mut libc::c_int,
        pub numLeafSurfaces: libc::c_int,
        pub leafsurfaces: *mut libc::c_int,
        pub numSubModels: libc::c_int,
        pub cmodels: *mut crate::cm_local_h::cmodel_t,
        pub numBrushes: libc::c_int,
        pub brushes: *mut crate::cm_local_h::cbrush_t,
        pub numClusters: libc::c_int,
        pub clusterBytes: libc::c_int,
        pub visibility: *mut crate::src::qcommon::q_shared::byte,
        pub vised: crate::src::qcommon::q_shared::qboolean,
        pub numEntityChars: libc::c_int,
        pub entityString: *mut libc::c_char,
        pub numAreas: libc::c_int,
        pub areas: *mut crate::cm_local_h::cArea_t,
        pub areaPortals: *mut libc::c_int,
        pub numSurfaces: libc::c_int,
        pub surfaces: *mut *mut crate::cm_local_h::cPatch_t,
        pub floodvalid: libc::c_int,
        pub checkcount: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct cPatch_t {
        pub checkcount: libc::c_int,
        pub surfaceFlags: libc::c_int,
        pub contents: libc::c_int,
        pub pc: *mut crate::src::qcommon::cm_patch::patchCollide_s,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct cArea_t {
        pub floodnum: libc::c_int,
        pub floodvalid: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct cbrush_t {
        pub shaderNum: libc::c_int,
        pub contents: libc::c_int,
        pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
        pub numsides: libc::c_int,
        pub sides: *mut crate::cm_local_h::cbrushside_t,
        pub checkcount: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct cbrushside_t {
        pub plane: *mut crate::src::qcommon::q_shared::cplane_t,
        pub surfaceFlags: libc::c_int,
        pub shaderNum: libc::c_int,
    }

    pub type cmodel_t = crate::cm_local_h::cmodel_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct cmodel_s {
        pub mins: crate::src::qcommon::q_shared::vec3_t,
        pub maxs: crate::src::qcommon::q_shared::vec3_t,
        pub leaf: crate::cm_local_h::cLeaf_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct cLeaf_t {
        pub cluster: libc::c_int,
        pub area: libc::c_int,
        pub firstLeafBrush: libc::c_int,
        pub numLeafBrushes: libc::c_int,
        pub firstLeafSurface: libc::c_int,
        pub numLeafSurfaces: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct cNode_t {
        pub plane: *mut crate::src::qcommon::q_shared::cplane_t,
        pub children: [libc::c_int; 2],
    }
}
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
}
pub mod stdarg_h {
    pub type va_list = crate::internal::__builtin_va_list;
}
pub mod internal {
    pub type __builtin_va_list = [crate::internal::__va_list_tag; 1];

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}
pub mod aasfile_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_lump_t {
        pub fileofs: libc::c_int,
        pub filelen: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_header_s {
        pub ident: libc::c_int,
        pub version: libc::c_int,
        pub bspchecksum: libc::c_int,
        pub lumps: [crate::aasfile_h::aas_lump_t; 14],
    }
    //child nodes of this node, or areas as leaves when negative
    //when a child is zero it's a solid leaf
    //aas file header

    pub type aas_header_t = crate::aasfile_h::aas_header_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_bbox_s {
        pub presencetype: libc::c_int,
        pub flags: libc::c_int,
        pub mins: crate::src::qcommon::q_shared::vec3_t,
        pub maxs: crate::src::qcommon::q_shared::vec3_t,
    }
    //bot can stand on the ground
    //area contains one or more ladder faces
    //area contains a liquid
    //area is disabled for routing when set
    //area ontop of a bridge
    //aas file header lumps
    //========== bounding box =========
    //bounding box

    pub type aas_bbox_t = crate::aasfile_h::aas_bbox_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_reachability_s {
        pub areanum: libc::c_int,
        pub facenum: libc::c_int,
        pub edgenum: libc::c_int,
        pub start: crate::src::qcommon::q_shared::vec3_t,
        pub end: crate::src::qcommon::q_shared::vec3_t,
        pub traveltype: libc::c_int,
        pub traveltime: libc::c_ushort,
    }
    //============ settings ===========
    //reachability to another area

    pub type aas_reachability_t = crate::aasfile_h::aas_reachability_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_areasettings_s {
        pub contents: libc::c_int,
        pub areaflags: libc::c_int,
        pub presencetype: libc::c_int,
        pub cluster: libc::c_int,
        pub clusterareanum: libc::c_int,
        pub numreachableareas: libc::c_int,
        pub firstreachablearea: libc::c_int,
    }
    //number of the reachable area
    //number of the face towards the other area
    //number of the edge towards the other area
    //start point of inter area movement
    //end point of inter area movement
    //type of travel required to get to the area
    //travel time of the inter area movement
    //area settings

    pub type aas_areasettings_t = crate::aasfile_h::aas_areasettings_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_portal_s {
        pub areanum: libc::c_int,
        pub frontcluster: libc::c_int,
        pub backcluster: libc::c_int,
        pub clusterareanum: [libc::c_int; 2],
    }
    //could also add all kind of statistic fields
    //contents of the area
    //several area flags
    //how a bot can be present in this area
    //cluster the area belongs to, if negative it's a portal
    //number of the area in the cluster
    //number of reachable areas from this one
    //first reachable area in the reachable area index
    //cluster portal

    pub type aas_portal_t = crate::aasfile_h::aas_portal_s;
    //area that is the actual portal
    //cluster at front of portal
    //cluster at back of portal
    //number of the area in the front and back cluster
    //cluster portal index

    pub type aas_portalindex_t = libc::c_int;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_cluster_s {
        pub numareas: libc::c_int,
        pub numreachabilityareas: libc::c_int,
        pub numportals: libc::c_int,
        pub firstportal: libc::c_int,
    }
    //cluster

    pub type aas_cluster_t = crate::aasfile_h::aas_cluster_s;
    //number of areas in the cluster
    //number of areas with reachabilities
    //number of cluster portals
    //first cluster portal in the index
    //============ 3d definition ============

    pub type aas_vertex_t = crate::src::qcommon::q_shared::vec3_t;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_plane_s {
        pub normal: crate::src::qcommon::q_shared::vec3_t,
        pub dist: libc::c_float,
        pub type_0: libc::c_int,
    }
    //just a plane in the third dimension

    pub type aas_plane_t = crate::aasfile_h::aas_plane_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_edge_s {
        pub v: [libc::c_int; 2],
    }
    //normal vector of the plane
    //distance of the plane (normal vector * distance = point in plane)
    //edge

    pub type aas_edge_t = crate::aasfile_h::aas_edge_s;
    //numbers of the vertexes of this edge
    //edge index, negative if vertexes are reversed

    pub type aas_edgeindex_t = libc::c_int;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_face_s {
        pub planenum: libc::c_int,
        pub faceflags: libc::c_int,
        pub numedges: libc::c_int,
        pub firstedge: libc::c_int,
        pub frontarea: libc::c_int,
        pub backarea: libc::c_int,
    }
    //a face bounds an area, often it will also separate two areas

    pub type aas_face_t = crate::aasfile_h::aas_face_s;
    //number of the plane this face is in
    //face flags (no use to create face settings for just this field)
    //number of edges in the boundary of the face
    //first edge in the edge index
    //area at the front of this face
    //area at the back of this face
    //face index, stores a negative index if backside of face

    pub type aas_faceindex_t = libc::c_int;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_area_s {
        pub areanum: libc::c_int,
        pub numfaces: libc::c_int,
        pub firstface: libc::c_int,
        pub mins: crate::src::qcommon::q_shared::vec3_t,
        pub maxs: crate::src::qcommon::q_shared::vec3_t,
        pub center: crate::src::qcommon::q_shared::vec3_t,
    }
    //area with a boundary of faces

    pub type aas_area_t = crate::aasfile_h::aas_area_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_node_s {
        pub planenum: libc::c_int,
        pub children: [libc::c_int; 2],
    }
    //number of this area
    //3d definition
    //number of faces used for the boundary of the area
    //first face in the face index used for the boundary of the area
    //mins of the area
    //maxs of the area
    //'center' of the area
    //nodes of the bsp tree

    pub type aas_node_t = crate::aasfile_h::aas_node_s;
}
pub mod be_aas_def_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct bsp_link_s {
        pub entnum: libc::c_int,
        pub leafnum: libc::c_int,
        pub next_ent: *mut crate::be_aas_def_h::bsp_link_s,
        pub prev_ent: *mut crate::be_aas_def_h::bsp_link_s,
        pub next_leaf: *mut crate::be_aas_def_h::bsp_link_s,
        pub prev_leaf: *mut crate::be_aas_def_h::bsp_link_s,
    }

    pub type bsp_link_t = crate::be_aas_def_h::bsp_link_s;

    pub type aas_settings_t = crate::be_aas_def_h::aas_settings_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_settings_s {
        pub phys_gravitydirection: crate::src::qcommon::q_shared::vec3_t,
        pub phys_friction: libc::c_float,
        pub phys_stopspeed: libc::c_float,
        pub phys_gravity: libc::c_float,
        pub phys_waterfriction: libc::c_float,
        pub phys_watergravity: libc::c_float,
        pub phys_maxvelocity: libc::c_float,
        pub phys_maxwalkvelocity: libc::c_float,
        pub phys_maxcrouchvelocity: libc::c_float,
        pub phys_maxswimvelocity: libc::c_float,
        pub phys_walkaccelerate: libc::c_float,
        pub phys_airaccelerate: libc::c_float,
        pub phys_swimaccelerate: libc::c_float,
        pub phys_maxstep: libc::c_float,
        pub phys_maxsteepness: libc::c_float,
        pub phys_maxwaterjump: libc::c_float,
        pub phys_maxbarrier: libc::c_float,
        pub phys_jumpvel: libc::c_float,
        pub phys_falldelta5: libc::c_float,
        pub phys_falldelta10: libc::c_float,
        pub rs_waterjump: libc::c_float,
        pub rs_teleport: libc::c_float,
        pub rs_barrierjump: libc::c_float,
        pub rs_startcrouch: libc::c_float,
        pub rs_startgrapple: libc::c_float,
        pub rs_startwalkoffledge: libc::c_float,
        pub rs_startjump: libc::c_float,
        pub rs_rocketjump: libc::c_float,
        pub rs_bfgjump: libc::c_float,
        pub rs_jumppad: libc::c_float,
        pub rs_aircontrolledjumppad: libc::c_float,
        pub rs_funcbob: libc::c_float,
        pub rs_startelevator: libc::c_float,
        pub rs_falldamage5: libc::c_float,
        pub rs_falldamage10: libc::c_float,
        pub rs_maxfallheight: libc::c_float,
        pub rs_maxjumpfallheight: libc::c_float,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_link_s {
        pub entnum: libc::c_int,
        pub areanum: libc::c_int,
        pub next_ent: *mut crate::be_aas_def_h::aas_link_s,
        pub prev_ent: *mut crate::be_aas_def_h::aas_link_s,
        pub next_area: *mut crate::be_aas_def_h::aas_link_s,
        pub prev_area: *mut crate::be_aas_def_h::aas_link_s,
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
    /* ****************************************************************************
     * name:		be_aas_def.h
     *
     * desc:		AAS
     *
     * $Archive: /source/code/botlib/be_aas_def.h $
     *
     *****************************************************************************/
    //debugging on
    //structure to link entities to areas and areas to entities

    pub type aas_link_t = crate::be_aas_def_h::aas_link_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_entity_s {
        pub i: crate::be_aas_h::aas_entityinfo_t,
        pub areas: *mut crate::be_aas_def_h::aas_link_t,
        pub leaves: *mut crate::be_aas_def_h::bsp_link_t,
    }
    //entity

    pub type aas_entity_t = crate::be_aas_def_h::aas_entity_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_routingcache_s {
        pub type_0: crate::src::qcommon::q_shared::byte,
        pub time: libc::c_float,
        pub size: libc::c_int,
        pub cluster: libc::c_int,
        pub areanum: libc::c_int,
        pub origin: crate::src::qcommon::q_shared::vec3_t,
        pub starttraveltime: libc::c_float,
        pub travelflags: libc::c_int,
        pub prev: *mut crate::be_aas_def_h::aas_routingcache_s,
        pub next: *mut crate::be_aas_def_h::aas_routingcache_s,
        pub time_prev: *mut crate::be_aas_def_h::aas_routingcache_s,
        pub time_next: *mut crate::be_aas_def_h::aas_routingcache_s,
        pub reachabilities: *mut libc::c_uchar,
        pub traveltimes: [libc::c_ushort; 1],
    }
    //entity info
    //links into the AAS areas
    //links into the BSP leaves
    //routing cache

    pub type aas_routingcache_t = crate::be_aas_def_h::aas_routingcache_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_routingupdate_s {
        pub cluster: libc::c_int,
        pub areanum: libc::c_int,
        pub start: crate::src::qcommon::q_shared::vec3_t,
        pub tmptraveltime: libc::c_ushort,
        pub areatraveltimes: *mut libc::c_ushort,
        pub inlist: crate::src::qcommon::q_shared::qboolean,
        pub next: *mut crate::be_aas_def_h::aas_routingupdate_s,
        pub prev: *mut crate::be_aas_def_h::aas_routingupdate_s,
    }
    //portal or area cache
    //last time accessed or updated
    //size of the routing cache
    //cluster the cache is for
    //area the cache is created for
    //origin within the area
    //travel time to start with
    //combinations of the travel flags
    //reachabilities used for routing
    //travel time for every area (variable sized)
    //fields for the routing algorithm

    pub type aas_routingupdate_t = crate::be_aas_def_h::aas_routingupdate_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_reversedlink_s {
        pub linknum: libc::c_int,
        pub areanum: libc::c_int,
        pub next: *mut crate::be_aas_def_h::aas_reversedlink_s,
    }
    //area number of the update
    //start point the area was entered
    //temporary travel time
    //travel times within the area
    //true if the update is in the list
    //reversed reachability link

    pub type aas_reversedlink_t = crate::be_aas_def_h::aas_reversedlink_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_reversedreachability_s {
        pub numlinks: libc::c_int,
        pub first: *mut crate::be_aas_def_h::aas_reversedlink_t,
    }
    //the aas_areareachability_t
    //reachable from this area
    //next link
    //reversed area reachability

    pub type aas_reversedreachability_t = crate::be_aas_def_h::aas_reversedreachability_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_reachabilityareas_s {
        pub firstarea: libc::c_int,
        pub numareas: libc::c_int,
    }
    //areas a reachability goes through

    pub type aas_reachabilityareas_t = crate::be_aas_def_h::aas_reachabilityareas_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_s {
        pub loaded: libc::c_int,
        pub initialized: libc::c_int,
        pub savefile: libc::c_int,
        pub bspchecksum: libc::c_int,
        pub time: libc::c_float,
        pub numframes: libc::c_int,
        pub filename: [libc::c_char; 64],
        pub mapname: [libc::c_char; 64],
        pub numbboxes: libc::c_int,
        pub bboxes: *mut crate::aasfile_h::aas_bbox_t,
        pub numvertexes: libc::c_int,
        pub vertexes: *mut crate::aasfile_h::aas_vertex_t,
        pub numplanes: libc::c_int,
        pub planes: *mut crate::aasfile_h::aas_plane_t,
        pub numedges: libc::c_int,
        pub edges: *mut crate::aasfile_h::aas_edge_t,
        pub edgeindexsize: libc::c_int,
        pub edgeindex: *mut crate::aasfile_h::aas_edgeindex_t,
        pub numfaces: libc::c_int,
        pub faces: *mut crate::aasfile_h::aas_face_t,
        pub faceindexsize: libc::c_int,
        pub faceindex: *mut crate::aasfile_h::aas_faceindex_t,
        pub numareas: libc::c_int,
        pub areas: *mut crate::aasfile_h::aas_area_t,
        pub numareasettings: libc::c_int,
        pub areasettings: *mut crate::aasfile_h::aas_areasettings_t,
        pub reachabilitysize: libc::c_int,
        pub reachability: *mut crate::aasfile_h::aas_reachability_t,
        pub numnodes: libc::c_int,
        pub nodes: *mut crate::aasfile_h::aas_node_t,
        pub numportals: libc::c_int,
        pub portals: *mut crate::aasfile_h::aas_portal_t,
        pub portalindexsize: libc::c_int,
        pub portalindex: *mut crate::aasfile_h::aas_portalindex_t,
        pub numclusters: libc::c_int,
        pub clusters: *mut crate::aasfile_h::aas_cluster_t,
        pub numreachabilityareas: libc::c_int,
        pub reachabilitytime: libc::c_float,
        pub linkheap: *mut crate::be_aas_def_h::aas_link_t,
        pub linkheapsize: libc::c_int,
        pub freelinks: *mut crate::be_aas_def_h::aas_link_t,
        pub arealinkedentities: *mut *mut crate::be_aas_def_h::aas_link_t,
        pub maxentities: libc::c_int,
        pub maxclients: libc::c_int,
        pub entities: *mut crate::be_aas_def_h::aas_entity_t,
        pub travelflagfortype: [libc::c_int; 32],
        pub areacontentstravelflags: *mut libc::c_int,
        pub areaupdate: *mut crate::be_aas_def_h::aas_routingupdate_t,
        pub portalupdate: *mut crate::be_aas_def_h::aas_routingupdate_t,
        pub frameroutingupdates: libc::c_int,
        pub reversedreachability: *mut crate::be_aas_def_h::aas_reversedreachability_t,
        pub areatraveltimes: *mut *mut *mut libc::c_ushort,
        pub clusterareacache: *mut *mut *mut crate::be_aas_def_h::aas_routingcache_t,
        pub portalcache: *mut *mut crate::be_aas_def_h::aas_routingcache_t,
        pub oldestcache: *mut crate::be_aas_def_h::aas_routingcache_t,
        pub newestcache: *mut crate::be_aas_def_h::aas_routingcache_t,
        pub portalmaxtraveltimes: *mut libc::c_int,
        pub reachabilityareaindex: *mut libc::c_int,
        pub reachabilityareas: *mut crate::be_aas_def_h::aas_reachabilityareas_t,
    }

    pub type aas_t = crate::be_aas_def_h::aas_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct bsp_entdata_s {
        pub origin: crate::src::qcommon::q_shared::vec3_t,
        pub angles: crate::src::qcommon::q_shared::vec3_t,
        pub absmins: crate::src::qcommon::q_shared::vec3_t,
        pub absmaxs: crate::src::qcommon::q_shared::vec3_t,
        pub solid: libc::c_int,
        pub modelnum: libc::c_int,
    }

    pub type bsp_entdata_t = crate::be_aas_def_h::bsp_entdata_s;
}
pub mod botlib_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct bot_input_s {
        pub thinktime: libc::c_float,
        pub dir: crate::src::qcommon::q_shared::vec3_t,
        pub speed: libc::c_float,
        pub viewangles: crate::src::qcommon::q_shared::vec3_t,
        pub actionflags: libc::c_int,
        pub weapon: libc::c_int,
    }

    pub type bot_input_t = crate::botlib_h::bot_input_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct bsp_surface_s {
        pub name: [libc::c_char; 16],
        pub flags: libc::c_int,
        pub value: libc::c_int,
    }
    //bsp_trace_t hit surface

    pub type bsp_surface_t = crate::botlib_h::bsp_surface_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct bsp_trace_s {
        pub allsolid: crate::src::qcommon::q_shared::qboolean,
        pub startsolid: crate::src::qcommon::q_shared::qboolean,
        pub fraction: libc::c_float,
        pub endpos: crate::src::qcommon::q_shared::vec3_t,
        pub plane: crate::src::qcommon::q_shared::cplane_t,
        pub exp_dist: libc::c_float,
        pub sidenum: libc::c_int,
        pub surface: crate::botlib_h::bsp_surface_t,
        pub contents: libc::c_int,
        pub ent: libc::c_int,
    }
    //remove the bsp_trace_s structure definition l8r on
    //a trace is returned when a box is swept through the world

    pub type bsp_trace_t = crate::botlib_h::bsp_trace_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct botlib_import_s {
        pub Print: Option<unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_char, _: ...) -> ()>,
        pub Trace: Option<
            unsafe extern "C" fn(
                _: *mut crate::botlib_h::bsp_trace_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: libc::c_int,
                _: libc::c_int,
            ) -> (),
        >,
        pub EntityTrace: Option<
            unsafe extern "C" fn(
                _: *mut crate::botlib_h::bsp_trace_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: libc::c_int,
                _: libc::c_int,
            ) -> (),
        >,
        pub PointContents: Option<
            unsafe extern "C" fn(_: *mut crate::src::qcommon::q_shared::vec_t) -> libc::c_int,
        >,
        pub inPVS: Option<
            unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> libc::c_int,
        >,
        pub BSPEntityData: Option<unsafe extern "C" fn() -> *mut libc::c_char>,
        pub BSPModelMinsMaxsOrigin: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> (),
        >,
        pub BotClientCommand:
            Option<unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_char) -> ()>,
        pub GetMemory: Option<unsafe extern "C" fn(_: libc::c_int) -> *mut libc::c_void>,
        pub FreeMemory: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        pub AvailableMemory: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub HunkAlloc: Option<unsafe extern "C" fn(_: libc::c_int) -> *mut libc::c_void>,
        pub FS_FOpenFile: Option<
            unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *mut crate::src::qcommon::q_shared::fileHandle_t,
                _: crate::src::qcommon::q_shared::fsMode_t,
            ) -> libc::c_int,
        >,
        pub FS_Read: Option<
            unsafe extern "C" fn(
                _: *mut libc::c_void,
                _: libc::c_int,
                _: crate::src::qcommon::q_shared::fileHandle_t,
            ) -> libc::c_int,
        >,
        pub FS_Write: Option<
            unsafe extern "C" fn(
                _: *const libc::c_void,
                _: libc::c_int,
                _: crate::src::qcommon::q_shared::fileHandle_t,
            ) -> libc::c_int,
        >,
        pub FS_FCloseFile:
            Option<unsafe extern "C" fn(_: crate::src::qcommon::q_shared::fileHandle_t) -> ()>,
        pub FS_Seek: Option<
            unsafe extern "C" fn(
                _: crate::src::qcommon::q_shared::fileHandle_t,
                _: libc::c_long,
                _: libc::c_int,
            ) -> libc::c_int,
        >,
        pub DebugLineCreate: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub DebugLineDelete: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub DebugLineShow: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: libc::c_int,
            ) -> (),
        >,
        pub DebugPolygonCreate: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: libc::c_int,
                _: *mut crate::src::qcommon::q_shared::vec3_t,
            ) -> libc::c_int,
        >,
        pub DebugPolygonDelete: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    }
    // if true, plane is not valid
    // if true, the initial point was in a solid area
    // time completed, 1.0 = didn't hit anything
    // final position
    // surface normal at impact
    // expanded plane distance
    // number of the brush side hit
    // the hit point surface
    // contents on other side of surface hit
    // number of entity hit
    //bot AI library exported functions

    pub type botlib_import_t = crate::botlib_h::botlib_import_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct bot_entitystate_s {
        pub type_0: libc::c_int,
        pub flags: libc::c_int,
        pub origin: crate::src::qcommon::q_shared::vec3_t,
        pub angles: crate::src::qcommon::q_shared::vec3_t,
        pub old_origin: crate::src::qcommon::q_shared::vec3_t,
        pub mins: crate::src::qcommon::q_shared::vec3_t,
        pub maxs: crate::src::qcommon::q_shared::vec3_t,
        pub groundent: libc::c_int,
        pub solid: libc::c_int,
        pub modelindex: libc::c_int,
        pub modelindex2: libc::c_int,
        pub frame: libc::c_int,
        pub event: libc::c_int,
        pub eventParm: libc::c_int,
        pub powerups: libc::c_int,
        pub weapon: libc::c_int,
        pub legsAnim: libc::c_int,
        pub torsoAnim: libc::c_int,
    }
    // if true, plane is not valid
    // if true, the initial point was in a solid area
    // time completed, 1.0 = didn't hit anything
    // final position
    // surface normal at impact
    // expanded plane distance
    // number of the brush side hit
    // the hit point surface
    // contents on other side of surface hit
    // number of entity hit
    // BSPTRACE
    //entity state

    pub type bot_entitystate_t = crate::botlib_h::bot_entitystate_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_export_s {
        pub AAS_EntityInfo: Option<
            unsafe extern "C" fn(_: libc::c_int, _: *mut crate::be_aas_h::aas_entityinfo_s) -> (),
        >,
        pub AAS_Initialized: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub AAS_PresenceTypeBoundingBox: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> (),
        >,
        pub AAS_Time: Option<unsafe extern "C" fn() -> libc::c_float>,
        pub AAS_PointAreaNum: Option<
            unsafe extern "C" fn(_: *mut crate::src::qcommon::q_shared::vec_t) -> libc::c_int,
        >,
        pub AAS_PointReachabilityAreaIndex: Option<
            unsafe extern "C" fn(_: *mut crate::src::qcommon::q_shared::vec_t) -> libc::c_int,
        >,
        pub AAS_TraceAreas: Option<
            unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut libc::c_int,
                _: *mut crate::src::qcommon::q_shared::vec3_t,
                _: libc::c_int,
            ) -> libc::c_int,
        >,
        pub AAS_BBoxAreas: Option<
            unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut libc::c_int,
                _: libc::c_int,
            ) -> libc::c_int,
        >,
        pub AAS_AreaInfo: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut crate::be_aas_h::aas_areainfo_s,
            ) -> libc::c_int,
        >,
        pub AAS_PointContents: Option<
            unsafe extern "C" fn(_: *mut crate::src::qcommon::q_shared::vec_t) -> libc::c_int,
        >,
        pub AAS_NextBSPEntity: Option<unsafe extern "C" fn(_: libc::c_int) -> libc::c_int>,
        pub AAS_ValueForBSPEpairKey: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: libc::c_int,
            ) -> libc::c_int,
        >,
        pub AAS_VectorForBSPEpairKey: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut libc::c_char,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> libc::c_int,
        >,
        pub AAS_FloatForBSPEpairKey: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut libc::c_char,
                _: *mut libc::c_float,
            ) -> libc::c_int,
        >,
        pub AAS_IntForBSPEpairKey: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut libc::c_char,
                _: *mut libc::c_int,
            ) -> libc::c_int,
        >,
        pub AAS_AreaReachability: Option<unsafe extern "C" fn(_: libc::c_int) -> libc::c_int>,
        pub AAS_AreaTravelTimeToGoalArea: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: libc::c_int,
                _: libc::c_int,
            ) -> libc::c_int,
        >,
        pub AAS_EnableRoutingArea:
            Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int) -> libc::c_int>,
        pub AAS_PredictRoute: Option<
            unsafe extern "C" fn(
                _: *mut crate::be_aas_h::aas_predictroute_s,
                _: libc::c_int,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: libc::c_int,
                _: libc::c_int,
                _: libc::c_int,
                _: libc::c_int,
                _: libc::c_int,
                _: libc::c_int,
                _: libc::c_int,
                _: libc::c_int,
            ) -> libc::c_int,
        >,
        pub AAS_AlternativeRouteGoals: Option<
            unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: libc::c_int,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: libc::c_int,
                _: libc::c_int,
                _: *mut crate::be_aas_h::aas_altroutegoal_s,
                _: libc::c_int,
                _: libc::c_int,
            ) -> libc::c_int,
        >,
        pub AAS_Swimming: Option<
            unsafe extern "C" fn(_: *mut crate::src::qcommon::q_shared::vec_t) -> libc::c_int,
        >,
        pub AAS_PredictClientMovement: Option<
            unsafe extern "C" fn(
                _: *mut crate::be_aas_h::aas_clientmove_s,
                _: libc::c_int,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: libc::c_int,
                _: libc::c_int,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: libc::c_int,
                _: libc::c_int,
                _: libc::c_float,
                _: libc::c_int,
                _: libc::c_int,
                _: libc::c_int,
            ) -> libc::c_int,
        >,
    }

    pub type aas_export_t = crate::botlib_h::aas_export_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ea_export_s {
        pub EA_Command: Option<unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_char) -> ()>,
        pub EA_Say: Option<unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_char) -> ()>,
        pub EA_SayTeam: Option<unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_char) -> ()>,
        pub EA_Action: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int) -> ()>,
        pub EA_Gesture: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_Talk: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_Attack: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_Use: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_Respawn: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_MoveUp: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_MoveDown: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_MoveForward: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_MoveBack: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_MoveLeft: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_MoveRight: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_Crouch: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_SelectWeapon: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int) -> ()>,
        pub EA_Jump: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_DelayedJump: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_Move: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: libc::c_float,
            ) -> (),
        >,
        pub EA_View: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> (),
        >,
        pub EA_EndRegular: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_float) -> ()>,
        pub EA_GetInput: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: libc::c_float,
                _: *mut crate::botlib_h::bot_input_t,
            ) -> (),
        >,
        pub EA_ResetInput: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    }

    pub type ea_export_t = crate::botlib_h::ea_export_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ai_export_s {
        pub BotLoadCharacter:
            Option<unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_float) -> libc::c_int>,
        pub BotFreeCharacter: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub Characteristic_Float:
            Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int) -> libc::c_float>,
        pub Characteristic_BFloat: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: libc::c_int,
                _: libc::c_float,
                _: libc::c_float,
            ) -> libc::c_float,
        >,
        pub Characteristic_Integer:
            Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int) -> libc::c_int>,
        pub Characteristic_BInteger: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: libc::c_int,
                _: libc::c_int,
                _: libc::c_int,
            ) -> libc::c_int,
        >,
        pub Characteristic_String: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: libc::c_int,
                _: *mut libc::c_char,
                _: libc::c_int,
            ) -> (),
        >,
        pub BotAllocChatState: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub BotFreeChatState: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub BotQueueConsoleMessage: Option<
            unsafe extern "C" fn(_: libc::c_int, _: libc::c_int, _: *mut libc::c_char) -> (),
        >,
        pub BotRemoveConsoleMessage:
            Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int) -> ()>,
        pub BotNextConsoleMessage: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut crate::src::botlib::be_ai_chat::bot_consolemessage_s,
            ) -> libc::c_int,
        >,
        pub BotNumConsoleMessages: Option<unsafe extern "C" fn(_: libc::c_int) -> libc::c_int>,
        pub BotInitialChat: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut libc::c_char,
                _: libc::c_int,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
            ) -> (),
        >,
        pub BotNumInitialChats:
            Option<unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_char) -> libc::c_int>,
        pub BotReplyChat: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut libc::c_char,
                _: libc::c_int,
                _: libc::c_int,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
            ) -> libc::c_int,
        >,
        pub BotChatLength: Option<unsafe extern "C" fn(_: libc::c_int) -> libc::c_int>,
        pub BotEnterChat:
            Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int, _: libc::c_int) -> ()>,
        pub BotGetChatMessage: Option<
            unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_char, _: libc::c_int) -> (),
        >,
        pub StringContains: Option<
            unsafe extern "C" fn(
                _: *mut libc::c_char,
                _: *mut libc::c_char,
                _: libc::c_int,
            ) -> libc::c_int,
        >,
        pub BotFindMatch: Option<
            unsafe extern "C" fn(
                _: *mut libc::c_char,
                _: *mut crate::src::botlib::be_ai_chat::bot_match_s,
                _: libc::c_ulong,
            ) -> libc::c_int,
        >,
        pub BotMatchVariable: Option<
            unsafe extern "C" fn(
                _: *mut crate::src::botlib::be_ai_chat::bot_match_s,
                _: libc::c_int,
                _: *mut libc::c_char,
                _: libc::c_int,
            ) -> (),
        >,
        pub UnifyWhiteSpaces: Option<unsafe extern "C" fn(_: *mut libc::c_char) -> ()>,
        pub BotReplaceSynonyms:
            Option<unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_ulong) -> ()>,
        pub BotLoadChatFile: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut libc::c_char,
                _: *mut libc::c_char,
            ) -> libc::c_int,
        >,
        pub BotSetChatGender: Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int) -> ()>,
        pub BotSetChatName: Option<
            unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_char, _: libc::c_int) -> (),
        >,
        pub BotResetGoalState: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub BotResetAvoidGoals: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub BotRemoveFromAvoidGoals:
            Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int) -> ()>,
        pub BotPushGoal: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_s,
            ) -> (),
        >,
        pub BotPopGoal: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub BotEmptyGoalStack: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub BotDumpAvoidGoals: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub BotDumpGoalStack: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub BotGoalName: Option<
            unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_char, _: libc::c_int) -> (),
        >,
        pub BotGetTopGoal: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_s,
            ) -> libc::c_int,
        >,
        pub BotGetSecondGoal: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_s,
            ) -> libc::c_int,
        >,
        pub BotChooseLTGItem: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut libc::c_int,
                _: libc::c_int,
            ) -> libc::c_int,
        >,
        pub BotChooseNBGItem: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut libc::c_int,
                _: libc::c_int,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_s,
                _: libc::c_float,
            ) -> libc::c_int,
        >,
        pub BotTouchingGoal: Option<
            unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_s,
            ) -> libc::c_int,
        >,
        pub BotItemGoalInVisButNotVisible: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_s,
            ) -> libc::c_int,
        >,
        pub BotGetLevelItemGoal: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut libc::c_char,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_s,
            ) -> libc::c_int,
        >,
        pub BotGetNextCampSpotGoal: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_s,
            ) -> libc::c_int,
        >,
        pub BotGetMapLocationGoal: Option<
            unsafe extern "C" fn(
                _: *mut libc::c_char,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_s,
            ) -> libc::c_int,
        >,
        pub BotAvoidGoalTime:
            Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int) -> libc::c_float>,
        pub BotSetAvoidGoalTime:
            Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int, _: libc::c_float) -> ()>,
        pub BotInitLevelItems: Option<unsafe extern "C" fn() -> ()>,
        pub BotUpdateEntityItems: Option<unsafe extern "C" fn() -> ()>,
        pub BotLoadItemWeights:
            Option<unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_char) -> libc::c_int>,
        pub BotFreeItemWeights: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub BotInterbreedGoalFuzzyLogic:
            Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int, _: libc::c_int) -> ()>,
        pub BotSaveGoalFuzzyLogic:
            Option<unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_char) -> ()>,
        pub BotMutateGoalFuzzyLogic:
            Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_float) -> ()>,
        pub BotAllocGoalState: Option<unsafe extern "C" fn(_: libc::c_int) -> libc::c_int>,
        pub BotFreeGoalState: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub BotResetMoveState: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub BotMoveToGoal: Option<
            unsafe extern "C" fn(
                _: *mut crate::src::botlib::be_ai_move::bot_moveresult_s,
                _: libc::c_int,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_s,
                _: libc::c_int,
            ) -> (),
        >,
        pub BotMoveInDirection: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: libc::c_float,
                _: libc::c_int,
            ) -> libc::c_int,
        >,
        pub BotResetAvoidReach: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub BotResetLastAvoidReach: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub BotReachabilityArea: Option<
            unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: libc::c_int,
            ) -> libc::c_int,
        >,
        pub BotMovementViewTarget: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_s,
                _: libc::c_int,
                _: libc::c_float,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> libc::c_int,
        >,
        pub BotPredictVisiblePosition: Option<
            unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: libc::c_int,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_s,
                _: libc::c_int,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> libc::c_int,
        >,
        pub BotAllocMoveState: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub BotFreeMoveState: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub BotInitMoveState: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut crate::src::botlib::be_ai_move::bot_initmove_s,
            ) -> (),
        >,
        pub BotAddAvoidSpot: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: libc::c_float,
                _: libc::c_int,
            ) -> (),
        >,
        pub BotChooseBestFightWeapon:
            Option<unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_int) -> libc::c_int>,
        pub BotGetWeaponInfo: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: libc::c_int,
                _: *mut crate::src::botlib::be_ai_weap::weaponinfo_s,
            ) -> (),
        >,
        pub BotLoadWeaponWeights:
            Option<unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_char) -> libc::c_int>,
        pub BotAllocWeaponState: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub BotFreeWeaponState: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub BotResetWeaponState: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub GeneticParentsAndChildSelection: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut libc::c_float,
                _: *mut libc::c_int,
                _: *mut libc::c_int,
                _: *mut libc::c_int,
            ) -> libc::c_int,
        >,
    }

    pub type ai_export_t = crate::botlib_h::ai_export_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct botlib_export_s {
        pub aas: crate::botlib_h::aas_export_t,
        pub ea: crate::botlib_h::ea_export_t,
        pub ai: crate::botlib_h::ai_export_t,
        pub BotLibSetup: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub BotLibShutdown: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub BotLibVarSet: Option<
            unsafe extern "C" fn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int,
        >,
        pub BotLibVarGet: Option<
            unsafe extern "C" fn(
                _: *const libc::c_char,
                _: *mut libc::c_char,
                _: libc::c_int,
            ) -> libc::c_int,
        >,
        pub PC_AddGlobalDefine: Option<unsafe extern "C" fn(_: *mut libc::c_char) -> libc::c_int>,
        pub PC_LoadSourceHandle:
            Option<unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_int>,
        pub PC_FreeSourceHandle: Option<unsafe extern "C" fn(_: libc::c_int) -> libc::c_int>,
        pub PC_ReadTokenHandle: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut crate::src::qcommon::q_shared::pc_token_t,
            ) -> libc::c_int,
        >,
        pub PC_SourceFileAndLine: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut libc::c_char,
                _: *mut libc::c_int,
            ) -> libc::c_int,
        >,
        pub BotLibStartFrame: Option<unsafe extern "C" fn(_: libc::c_float) -> libc::c_int>,
        pub BotLibLoadMap: Option<unsafe extern "C" fn(_: *const libc::c_char) -> libc::c_int>,
        pub BotLibUpdateEntity: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut crate::botlib_h::bot_entitystate_t,
            ) -> libc::c_int,
        >,
        pub Test: Option<
            unsafe extern "C" fn(
                _: libc::c_int,
                _: *mut libc::c_char,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> libc::c_int,
        >,
    }

    pub type botlib_export_t = crate::botlib_h::botlib_export_s;
}
pub mod vm_local_h {
    pub const OP_CVFI: crate::be_aas_h::C2RustUnnamed_0 = 59;

    pub const OP_CVIF: crate::be_aas_h::C2RustUnnamed_0 = 58;

    pub const OP_MULF: crate::be_aas_h::C2RustUnnamed_0 = 57;

    pub const OP_DIVF: crate::be_aas_h::C2RustUnnamed_0 = 56;

    pub const OP_SUBF: crate::be_aas_h::C2RustUnnamed_0 = 55;

    pub const OP_ADDF: crate::be_aas_h::C2RustUnnamed_0 = 54;

    pub const OP_NEGF: crate::be_aas_h::C2RustUnnamed_0 = 53;

    pub const OP_RSHU: crate::be_aas_h::C2RustUnnamed_0 = 52;

    pub const OP_RSHI: crate::be_aas_h::C2RustUnnamed_0 = 51;

    pub const OP_LSH: crate::be_aas_h::C2RustUnnamed_0 = 50;

    pub const OP_BCOM: crate::be_aas_h::C2RustUnnamed_0 = 49;

    pub const OP_BXOR: crate::be_aas_h::C2RustUnnamed_0 = 48;

    pub const OP_BOR: crate::be_aas_h::C2RustUnnamed_0 = 47;

    pub const OP_BAND: crate::be_aas_h::C2RustUnnamed_0 = 46;

    pub const OP_MULU: crate::be_aas_h::C2RustUnnamed_0 = 45;

    pub const OP_MULI: crate::be_aas_h::C2RustUnnamed_0 = 44;

    pub const OP_MODU: crate::be_aas_h::C2RustUnnamed_0 = 43;

    pub const OP_MODI: crate::be_aas_h::C2RustUnnamed_0 = 42;

    pub const OP_DIVU: crate::be_aas_h::C2RustUnnamed_0 = 41;

    pub const OP_DIVI: crate::be_aas_h::C2RustUnnamed_0 = 40;

    pub const OP_SUB: crate::be_aas_h::C2RustUnnamed_0 = 39;

    pub const OP_ADD: crate::be_aas_h::C2RustUnnamed_0 = 38;

    pub const OP_NEGI: crate::be_aas_h::C2RustUnnamed_0 = 37;

    pub const OP_SEX16: crate::be_aas_h::C2RustUnnamed_0 = 36;

    pub const OP_SEX8: crate::be_aas_h::C2RustUnnamed_0 = 35;

    pub const OP_BLOCK_COPY: crate::be_aas_h::C2RustUnnamed_0 = 34;

    pub const OP_ARG: crate::be_aas_h::C2RustUnnamed_0 = 33;

    pub const OP_STORE4: crate::be_aas_h::C2RustUnnamed_0 = 32;

    pub const OP_STORE2: crate::be_aas_h::C2RustUnnamed_0 = 31;

    pub const OP_STORE1: crate::be_aas_h::C2RustUnnamed_0 = 30;

    pub const OP_LOAD4: crate::be_aas_h::C2RustUnnamed_0 = 29;

    pub const OP_LOAD2: crate::be_aas_h::C2RustUnnamed_0 = 28;

    pub const OP_LOAD1: crate::be_aas_h::C2RustUnnamed_0 = 27;

    pub const OP_GEF: crate::be_aas_h::C2RustUnnamed_0 = 26;

    pub const OP_GTF: crate::be_aas_h::C2RustUnnamed_0 = 25;

    pub const OP_LEF: crate::be_aas_h::C2RustUnnamed_0 = 24;

    pub const OP_LTF: crate::be_aas_h::C2RustUnnamed_0 = 23;

    pub const OP_NEF: crate::be_aas_h::C2RustUnnamed_0 = 22;

    pub const OP_EQF: crate::be_aas_h::C2RustUnnamed_0 = 21;

    pub const OP_GEU: crate::be_aas_h::C2RustUnnamed_0 = 20;

    pub const OP_GTU: crate::be_aas_h::C2RustUnnamed_0 = 19;

    pub const OP_LEU: crate::be_aas_h::C2RustUnnamed_0 = 18;

    pub const OP_LTU: crate::be_aas_h::C2RustUnnamed_0 = 17;

    pub const OP_GEI: crate::be_aas_h::C2RustUnnamed_0 = 16;

    pub const OP_GTI: crate::be_aas_h::C2RustUnnamed_0 = 15;

    pub const OP_LEI: crate::be_aas_h::C2RustUnnamed_0 = 14;

    pub const OP_LTI: crate::be_aas_h::C2RustUnnamed_0 = 13;

    pub const OP_NE: crate::be_aas_h::C2RustUnnamed_0 = 12;

    pub const OP_EQ: crate::be_aas_h::C2RustUnnamed_0 = 11;

    pub const OP_JUMP: crate::be_aas_h::C2RustUnnamed_0 = 10;

    pub const OP_LOCAL: crate::be_aas_h::C2RustUnnamed_0 = 9;

    pub const OP_CONST: crate::be_aas_h::C2RustUnnamed_0 = 8;

    pub const OP_POP: crate::be_aas_h::C2RustUnnamed_0 = 7;

    pub const OP_PUSH: crate::be_aas_h::C2RustUnnamed_0 = 6;

    pub const OP_CALL: crate::be_aas_h::C2RustUnnamed_0 = 5;

    pub const OP_LEAVE: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const OP_ENTER: crate::be_aas_h::C2RustUnnamed_0 = 3;

    pub const OP_BREAK: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const OP_IGNORE: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const OP_UNDEF: crate::be_aas_h::C2RustUnnamed_0 = 0;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vm_s {
        pub programStack: libc::c_int,
        pub systemCall: Option<
            unsafe extern "C" fn(_: *mut crate::stdlib::intptr_t) -> crate::stdlib::intptr_t,
        >,
        pub name: [libc::c_char; 64],
        pub searchPath: *mut libc::c_void,
        pub dllHandle: *mut libc::c_void,
        pub entryPoint:
            Option<unsafe extern "C" fn(_: libc::c_int, _: ...) -> crate::stdlib::intptr_t>,
        pub destroy: Option<unsafe extern "C" fn(_: *mut crate::qcommon_h::vm_t) -> ()>,
        pub currentlyInterpreting: crate::src::qcommon::q_shared::qboolean,
        pub compiled: crate::src::qcommon::q_shared::qboolean,
        pub codeBase: *mut crate::src::qcommon::q_shared::byte,
        pub entryOfs: libc::c_int,
        pub codeLength: libc::c_int,
        pub instructionPointers: *mut crate::stdlib::intptr_t,
        pub instructionCount: libc::c_int,
        pub dataBase: *mut crate::src::qcommon::q_shared::byte,
        pub dataMask: libc::c_int,
        pub dataAlloc: libc::c_int,
        pub stackBottom: libc::c_int,
        pub numSymbols: libc::c_int,
        pub symbols: *mut crate::vm_local_h::vmSymbol_s,
        pub callLevel: libc::c_int,
        pub breakFunction: libc::c_int,
        pub breakCount: libc::c_int,
        pub jumpTableTargets: *mut crate::src::qcommon::q_shared::byte,
        pub numJumpTableTargets: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vmSymbol_s {
        pub next: *mut crate::vm_local_h::vmSymbol_s,
        pub symValue: libc::c_int,
        pub profileCount: libc::c_int,
        pub symName: [libc::c_char; 1],
    }

    pub type vmSymbol_t = crate::vm_local_h::vmSymbol_s;
}
pub mod bg_public_h {
    pub const GT_FFA: crate::be_aas_h::C2RustUnnamed_0 = 0;

    pub const GT_SINGLE_PLAYER: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const GT_MAX_GAME_TYPE: crate::be_aas_h::C2RustUnnamed_0 = 8;

    pub const GT_HARVESTER: crate::be_aas_h::C2RustUnnamed_0 = 7;

    pub const GT_OBELISK: crate::be_aas_h::C2RustUnnamed_0 = 6;

    pub const GT_1FCTF: crate::be_aas_h::C2RustUnnamed_0 = 5;

    pub const GT_CTF: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const GT_TEAM: crate::be_aas_h::C2RustUnnamed_0 = 3;

    pub const GT_TOURNAMENT: crate::be_aas_h::C2RustUnnamed_0 = 1;
}
pub mod server_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct svEntity_s {
        pub worldSector: *mut crate::server_h::worldSector_s,
        pub nextEntityInWorldSector: *mut crate::server_h::svEntity_s,
        pub baseline: crate::src::qcommon::q_shared::entityState_t,
        pub numClusters: libc::c_int,
        pub clusternums: [libc::c_int; 16],
        pub lastCluster: libc::c_int,
        pub areanum: libc::c_int,
        pub areanum2: libc::c_int,
        pub snapshotCounter: libc::c_int,
    }

    pub type svEntity_t = crate::server_h::svEntity_s;

    pub type serverState_t = libc::c_uint;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct server_t {
        pub state: crate::server_h::serverState_t,
        pub restarting: crate::src::qcommon::q_shared::qboolean,
        pub serverId: libc::c_int,
        pub restartedServerId: libc::c_int,
        pub checksumFeed: libc::c_int,
        pub checksumFeedServerId: libc::c_int,
        pub snapshotCounter: libc::c_int,
        pub timeResidual: libc::c_int,
        pub nextFrameTime: libc::c_int,
        pub configstrings: [*mut libc::c_char; 1024],
        pub svEntities: [crate::server_h::svEntity_t; 1024],
        pub entityParsePoint: *mut libc::c_char,
        pub gentities: *mut crate::g_public_h::sharedEntity_t,
        pub gentitySize: libc::c_int,
        pub num_entities: libc::c_int,
        pub gameClients: *mut crate::src::qcommon::q_shared::playerState_t,
        pub gameClientSize: libc::c_int,
        pub restartTime: libc::c_int,
        pub time: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct serverBan_t {
        pub ip: crate::qcommon_h::netadr_t,
        pub subnet: libc::c_int,
        pub isexception: crate::src::qcommon::q_shared::qboolean,
    }

    pub const SS_GAME: crate::server_h::serverState_t = 2;

    pub const SS_LOADING: crate::server_h::serverState_t = 1;

    pub const SS_DEAD: crate::server_h::serverState_t = 0;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union C2RustUnnamed_38 {
        pub _4: [crate::src::qcommon::q_shared::byte; 4],
        pub _6: [crate::src::qcommon::q_shared::byte; 16],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct clientSnapshot_t {
        pub areabytes: libc::c_int,
        pub areabits: [crate::src::qcommon::q_shared::byte; 32],
        pub ps: crate::src::qcommon::q_shared::playerState_t,
        pub num_entities: libc::c_int,
        pub first_entity: libc::c_int,
        pub messageSent: libc::c_int,
        pub messageAcked: libc::c_int,
        pub messageSize: libc::c_int,
    }

    pub type clientState_t = libc::c_uint;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct netchan_buffer_s {
        pub msg: crate::qcommon_h::msg_t,
        pub msgBuffer: [crate::src::qcommon::q_shared::byte; 16384],
        pub clientCommandString: [libc::c_char; 1024],
        pub next: *mut crate::server_h::netchan_buffer_s,
    }

    pub type netchan_buffer_t = crate::server_h::netchan_buffer_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct client_s {
        pub state: crate::server_h::clientState_t,
        pub userinfo: [libc::c_char; 1024],
        pub reliableCommands: [[libc::c_char; 1024]; 64],
        pub reliableSequence: libc::c_int,
        pub reliableAcknowledge: libc::c_int,
        pub reliableSent: libc::c_int,
        pub messageAcknowledge: libc::c_int,
        pub gamestateMessageNum: libc::c_int,
        pub challenge: libc::c_int,
        pub lastUsercmd: crate::src::qcommon::q_shared::usercmd_t,
        pub lastMessageNum: libc::c_int,
        pub lastClientCommand: libc::c_int,
        pub lastClientCommandString: [libc::c_char; 1024],
        pub gentity: *mut crate::g_public_h::sharedEntity_t,
        pub name: [libc::c_char; 32],
        pub downloadName: [libc::c_char; 64],
        pub download: crate::src::qcommon::q_shared::fileHandle_t,
        pub downloadSize: libc::c_int,
        pub downloadCount: libc::c_int,
        pub downloadClientBlock: libc::c_int,
        pub downloadCurrentBlock: libc::c_int,
        pub downloadXmitBlock: libc::c_int,
        pub downloadBlocks: [*mut libc::c_uchar; 48],
        pub downloadBlockSize: [libc::c_int; 48],
        pub downloadEOF: crate::src::qcommon::q_shared::qboolean,
        pub downloadSendTime: libc::c_int,
        pub deltaMessage: libc::c_int,
        pub nextReliableTime: libc::c_int,
        pub lastPacketTime: libc::c_int,
        pub lastConnectTime: libc::c_int,
        pub lastSnapshotTime: libc::c_int,
        pub rateDelayed: crate::src::qcommon::q_shared::qboolean,
        pub timeoutCount: libc::c_int,
        pub frames: [crate::server_h::clientSnapshot_t; 32],
        pub ping: libc::c_int,
        pub rate: libc::c_int,
        pub snapshotMsec: libc::c_int,
        pub pureAuthentic: libc::c_int,
        pub gotCP: crate::src::qcommon::q_shared::qboolean,
        pub netchan: crate::qcommon_h::netchan_t,
        pub netchan_start_queue: *mut crate::server_h::netchan_buffer_t,
        pub netchan_end_queue: *mut *mut crate::server_h::netchan_buffer_t,
        pub oldServerTime: libc::c_int,
        pub csUpdated: [crate::src::qcommon::q_shared::qboolean; 1024],
        pub compat: crate::src::qcommon::q_shared::qboolean,
    }

    pub type client_t = crate::server_h::client_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct challenge_t {
        pub adr: crate::qcommon_h::netadr_t,
        pub challenge: libc::c_int,
        pub clientChallenge: libc::c_int,
        pub time: libc::c_int,
        pub pingTime: libc::c_int,
        pub firstTime: libc::c_int,
        pub wasrefused: crate::src::qcommon::q_shared::qboolean,
        pub connected: crate::src::qcommon::q_shared::qboolean,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct serverStatic_t {
        pub initialized: crate::src::qcommon::q_shared::qboolean,
        pub time: libc::c_int,
        pub snapFlagServerBit: libc::c_int,
        pub clients: *mut crate::server_h::client_t,
        pub numSnapshotEntities: libc::c_int,
        pub nextSnapshotEntities: libc::c_int,
        pub snapshotEntities: *mut crate::src::qcommon::q_shared::entityState_t,
        pub nextHeartbeatTime: libc::c_int,
        pub challenges: [crate::server_h::challenge_t; 2048],
        pub redirectAddress: crate::qcommon_h::netadr_t,
        pub authorizeAddress: crate::qcommon_h::netadr_t,
        pub masterResolveTime: [libc::c_int; 5],
    }

    pub use crate::src::server::sv_world::worldSector_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct leakyBucket_s {
        pub type_0: crate::qcommon_h::netadrtype_t,
        pub ipv: crate::server_h::C2RustUnnamed_38,
        pub lastTime: libc::c_int,
        pub burst: libc::c_schar,
        pub hash: libc::c_long,
        pub prev: *mut crate::server_h::leakyBucket_t,
        pub next: *mut crate::server_h::leakyBucket_t,
    }

    pub type leakyBucket_t = crate::server_h::leakyBucket_s;

    pub const CS_ACTIVE: crate::server_h::clientState_t = 4;

    pub const CS_PRIMED: crate::server_h::clientState_t = 3;

    pub const CS_CONNECTED: crate::server_h::clientState_t = 2;

    pub const CS_ZOMBIE: crate::server_h::clientState_t = 1;

    pub const CS_FREE: crate::server_h::clientState_t = 0;
}
pub mod g_public_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct entityShared_t {
        pub unused: crate::src::qcommon::q_shared::entityState_t,
        pub linked: crate::src::qcommon::q_shared::qboolean,
        pub linkcount: libc::c_int,
        pub svFlags: libc::c_int,
        pub singleClient: libc::c_int,
        pub bmodel: crate::src::qcommon::q_shared::qboolean,
        pub mins: crate::src::qcommon::q_shared::vec3_t,
        pub maxs: crate::src::qcommon::q_shared::vec3_t,
        pub contents: libc::c_int,
        pub absmin: crate::src::qcommon::q_shared::vec3_t,
        pub absmax: crate::src::qcommon::q_shared::vec3_t,
        pub currentOrigin: crate::src::qcommon::q_shared::vec3_t,
        pub currentAngles: crate::src::qcommon::q_shared::vec3_t,
        pub ownerNum: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct sharedEntity_t {
        pub s: crate::src::qcommon::q_shared::entityState_t,
        pub r: crate::g_public_h::entityShared_t,
    }

    pub const BOTAI_START_FRAME: crate::be_aas_h::C2RustUnnamed_0 = 10;

    pub const GAME_CONSOLE_COMMAND: crate::be_aas_h::C2RustUnnamed_0 = 9;

    pub const GAME_RUN_FRAME: crate::be_aas_h::C2RustUnnamed_0 = 8;

    pub const GAME_CLIENT_THINK: crate::be_aas_h::C2RustUnnamed_0 = 7;

    pub const GAME_CLIENT_COMMAND: crate::be_aas_h::C2RustUnnamed_0 = 6;

    pub const GAME_CLIENT_DISCONNECT: crate::be_aas_h::C2RustUnnamed_0 = 5;

    pub const GAME_CLIENT_USERINFO_CHANGED: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const GAME_CLIENT_BEGIN: crate::be_aas_h::C2RustUnnamed_0 = 3;

    pub const GAME_CLIENT_CONNECT: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const GAME_SHUTDOWN: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const GAME_INIT: crate::be_aas_h::C2RustUnnamed_0 = 0;

    pub const BOTLIB_PC_SOURCE_FILE_AND_LINE: crate::be_aas_h::C2RustUnnamed_0 = 581;

    pub const BOTLIB_PC_READ_TOKEN: crate::be_aas_h::C2RustUnnamed_0 = 580;

    pub const BOTLIB_PC_FREE_SOURCE: crate::be_aas_h::C2RustUnnamed_0 = 579;

    pub const BOTLIB_PC_LOAD_SOURCE: crate::be_aas_h::C2RustUnnamed_0 = 578;

    pub const BOTLIB_AAS_POINT_REACHABILITY_AREA_INDEX: crate::be_aas_h::C2RustUnnamed_0 = 577;

    pub const BOTLIB_AAS_PREDICT_ROUTE: crate::be_aas_h::C2RustUnnamed_0 = 576;

    pub const BOTLIB_AAS_ALTERNATIVE_ROUTE_GOAL: crate::be_aas_h::C2RustUnnamed_0 = 575;

    pub const BOTLIB_AI_ADD_AVOID_SPOT: crate::be_aas_h::C2RustUnnamed_0 = 574;

    pub const BOTLIB_AI_SET_AVOID_GOAL_TIME: crate::be_aas_h::C2RustUnnamed_0 = 573;

    pub const BOTLIB_AI_PREDICT_VISIBLE_POSITION: crate::be_aas_h::C2RustUnnamed_0 = 572;

    pub const BOTLIB_AI_REMOVE_FROM_AVOID_GOALS: crate::be_aas_h::C2RustUnnamed_0 = 571;

    pub const BOTLIB_AI_GET_CHAT_MESSAGE: crate::be_aas_h::C2RustUnnamed_0 = 570;

    pub const BOTLIB_AI_NUM_INITIAL_CHATS: crate::be_aas_h::C2RustUnnamed_0 = 569;

    pub const BOTLIB_AI_GET_MAP_LOCATION_GOAL: crate::be_aas_h::C2RustUnnamed_0 = 568;

    pub const BOTLIB_AI_GET_NEXT_CAMP_SPOT_GOAL: crate::be_aas_h::C2RustUnnamed_0 = 567;

    pub const BOTLIB_AI_MUTATE_GOAL_FUZZY_LOGIC: crate::be_aas_h::C2RustUnnamed_0 = 566;

    pub const BOTLIB_AI_INTERBREED_GOAL_FUZZY_LOGIC: crate::be_aas_h::C2RustUnnamed_0 = 565;

    pub const BOTLIB_AI_GENETIC_PARENTS_AND_CHILD_SELECTION: crate::be_aas_h::C2RustUnnamed_0 = 564;

    pub const BOTLIB_AI_RESET_WEAPON_STATE: crate::be_aas_h::C2RustUnnamed_0 = 563;

    pub const BOTLIB_AI_FREE_WEAPON_STATE: crate::be_aas_h::C2RustUnnamed_0 = 562;

    pub const BOTLIB_AI_ALLOC_WEAPON_STATE: crate::be_aas_h::C2RustUnnamed_0 = 561;

    pub const BOTLIB_AI_LOAD_WEAPON_WEIGHTS: crate::be_aas_h::C2RustUnnamed_0 = 560;

    pub const BOTLIB_AI_GET_WEAPON_INFO: crate::be_aas_h::C2RustUnnamed_0 = 559;

    pub const BOTLIB_AI_CHOOSE_BEST_FIGHT_WEAPON: crate::be_aas_h::C2RustUnnamed_0 = 558;

    pub const BOTLIB_AI_INIT_MOVE_STATE: crate::be_aas_h::C2RustUnnamed_0 = 557;

    pub const BOTLIB_AI_FREE_MOVE_STATE: crate::be_aas_h::C2RustUnnamed_0 = 556;

    pub const BOTLIB_AI_ALLOC_MOVE_STATE: crate::be_aas_h::C2RustUnnamed_0 = 555;

    pub const BOTLIB_AI_MOVEMENT_VIEW_TARGET: crate::be_aas_h::C2RustUnnamed_0 = 554;

    pub const BOTLIB_AI_REACHABILITY_AREA: crate::be_aas_h::C2RustUnnamed_0 = 553;

    pub const BOTLIB_AI_RESET_LAST_AVOID_REACH: crate::be_aas_h::C2RustUnnamed_0 = 552;

    pub const BOTLIB_AI_RESET_AVOID_REACH: crate::be_aas_h::C2RustUnnamed_0 = 551;

    pub const BOTLIB_AI_MOVE_IN_DIRECTION: crate::be_aas_h::C2RustUnnamed_0 = 550;

    pub const BOTLIB_AI_MOVE_TO_GOAL: crate::be_aas_h::C2RustUnnamed_0 = 549;

    pub const BOTLIB_AI_RESET_MOVE_STATE: crate::be_aas_h::C2RustUnnamed_0 = 548;

    pub const BOTLIB_AI_FREE_GOAL_STATE: crate::be_aas_h::C2RustUnnamed_0 = 547;

    pub const BOTLIB_AI_ALLOC_GOAL_STATE: crate::be_aas_h::C2RustUnnamed_0 = 546;

    pub const BOTLIB_AI_SAVE_GOAL_FUZZY_LOGIC: crate::be_aas_h::C2RustUnnamed_0 = 545;

    pub const BOTLIB_AI_FREE_ITEM_WEIGHTS: crate::be_aas_h::C2RustUnnamed_0 = 544;

    pub const BOTLIB_AI_LOAD_ITEM_WEIGHTS: crate::be_aas_h::C2RustUnnamed_0 = 543;

    pub const BOTLIB_AI_UPDATE_ENTITY_ITEMS: crate::be_aas_h::C2RustUnnamed_0 = 542;

    pub const BOTLIB_AI_INIT_LEVEL_ITEMS: crate::be_aas_h::C2RustUnnamed_0 = 541;

    pub const BOTLIB_AI_AVOID_GOAL_TIME: crate::be_aas_h::C2RustUnnamed_0 = 540;

    pub const BOTLIB_AI_GET_LEVEL_ITEM_GOAL: crate::be_aas_h::C2RustUnnamed_0 = 539;

    pub const BOTLIB_AI_ITEM_GOAL_IN_VIS_BUT_NOT_VISIBLE: crate::be_aas_h::C2RustUnnamed_0 = 538;

    pub const BOTLIB_AI_TOUCHING_GOAL: crate::be_aas_h::C2RustUnnamed_0 = 537;

    pub const BOTLIB_AI_CHOOSE_NBG_ITEM: crate::be_aas_h::C2RustUnnamed_0 = 536;

    pub const BOTLIB_AI_CHOOSE_LTG_ITEM: crate::be_aas_h::C2RustUnnamed_0 = 535;

    pub const BOTLIB_AI_GET_SECOND_GOAL: crate::be_aas_h::C2RustUnnamed_0 = 534;

    pub const BOTLIB_AI_GET_TOP_GOAL: crate::be_aas_h::C2RustUnnamed_0 = 533;

    pub const BOTLIB_AI_GOAL_NAME: crate::be_aas_h::C2RustUnnamed_0 = 532;

    pub const BOTLIB_AI_DUMP_GOAL_STACK: crate::be_aas_h::C2RustUnnamed_0 = 531;

    pub const BOTLIB_AI_DUMP_AVOID_GOALS: crate::be_aas_h::C2RustUnnamed_0 = 530;

    pub const BOTLIB_AI_EMPTY_GOAL_STACK: crate::be_aas_h::C2RustUnnamed_0 = 529;

    pub const BOTLIB_AI_POP_GOAL: crate::be_aas_h::C2RustUnnamed_0 = 528;

    pub const BOTLIB_AI_PUSH_GOAL: crate::be_aas_h::C2RustUnnamed_0 = 527;

    pub const BOTLIB_AI_RESET_AVOID_GOALS: crate::be_aas_h::C2RustUnnamed_0 = 526;

    pub const BOTLIB_AI_RESET_GOAL_STATE: crate::be_aas_h::C2RustUnnamed_0 = 525;

    pub const BOTLIB_AI_SET_CHAT_NAME: crate::be_aas_h::C2RustUnnamed_0 = 524;

    pub const BOTLIB_AI_SET_CHAT_GENDER: crate::be_aas_h::C2RustUnnamed_0 = 523;

    pub const BOTLIB_AI_LOAD_CHAT_FILE: crate::be_aas_h::C2RustUnnamed_0 = 522;

    pub const BOTLIB_AI_REPLACE_SYNONYMS: crate::be_aas_h::C2RustUnnamed_0 = 521;

    pub const BOTLIB_AI_UNIFY_WHITE_SPACES: crate::be_aas_h::C2RustUnnamed_0 = 520;

    pub const BOTLIB_AI_MATCH_VARIABLE: crate::be_aas_h::C2RustUnnamed_0 = 519;

    pub const BOTLIB_AI_FIND_MATCH: crate::be_aas_h::C2RustUnnamed_0 = 518;

    pub const BOTLIB_AI_STRING_CONTAINS: crate::be_aas_h::C2RustUnnamed_0 = 517;

    pub const BOTLIB_AI_ENTER_CHAT: crate::be_aas_h::C2RustUnnamed_0 = 516;

    pub const BOTLIB_AI_CHAT_LENGTH: crate::be_aas_h::C2RustUnnamed_0 = 515;

    pub const BOTLIB_AI_REPLY_CHAT: crate::be_aas_h::C2RustUnnamed_0 = 514;

    pub const BOTLIB_AI_INITIAL_CHAT: crate::be_aas_h::C2RustUnnamed_0 = 513;

    pub const BOTLIB_AI_NUM_CONSOLE_MESSAGE: crate::be_aas_h::C2RustUnnamed_0 = 512;

    pub const BOTLIB_AI_NEXT_CONSOLE_MESSAGE: crate::be_aas_h::C2RustUnnamed_0 = 511;

    pub const BOTLIB_AI_REMOVE_CONSOLE_MESSAGE: crate::be_aas_h::C2RustUnnamed_0 = 510;

    pub const BOTLIB_AI_QUEUE_CONSOLE_MESSAGE: crate::be_aas_h::C2RustUnnamed_0 = 509;

    pub const BOTLIB_AI_FREE_CHAT_STATE: crate::be_aas_h::C2RustUnnamed_0 = 508;

    pub const BOTLIB_AI_ALLOC_CHAT_STATE: crate::be_aas_h::C2RustUnnamed_0 = 507;

    pub const BOTLIB_AI_CHARACTERISTIC_STRING: crate::be_aas_h::C2RustUnnamed_0 = 506;

    pub const BOTLIB_AI_CHARACTERISTIC_BINTEGER: crate::be_aas_h::C2RustUnnamed_0 = 505;

    pub const BOTLIB_AI_CHARACTERISTIC_INTEGER: crate::be_aas_h::C2RustUnnamed_0 = 504;

    pub const BOTLIB_AI_CHARACTERISTIC_BFLOAT: crate::be_aas_h::C2RustUnnamed_0 = 503;

    pub const BOTLIB_AI_CHARACTERISTIC_FLOAT: crate::be_aas_h::C2RustUnnamed_0 = 502;

    pub const BOTLIB_AI_FREE_CHARACTER: crate::be_aas_h::C2RustUnnamed_0 = 501;

    pub const BOTLIB_AI_LOAD_CHARACTER: crate::be_aas_h::C2RustUnnamed_0 = 500;

    pub const BOTLIB_EA_RESET_INPUT: crate::be_aas_h::C2RustUnnamed_0 = 423;

    pub const BOTLIB_EA_GET_INPUT: crate::be_aas_h::C2RustUnnamed_0 = 422;

    pub const BOTLIB_EA_END_REGULAR: crate::be_aas_h::C2RustUnnamed_0 = 421;

    pub const BOTLIB_EA_VIEW: crate::be_aas_h::C2RustUnnamed_0 = 420;

    pub const BOTLIB_EA_MOVE: crate::be_aas_h::C2RustUnnamed_0 = 419;

    pub const BOTLIB_EA_DELAYED_JUMP: crate::be_aas_h::C2RustUnnamed_0 = 418;

    pub const BOTLIB_EA_JUMP: crate::be_aas_h::C2RustUnnamed_0 = 417;

    pub const BOTLIB_EA_SELECT_WEAPON: crate::be_aas_h::C2RustUnnamed_0 = 416;

    pub const BOTLIB_EA_MOVE_RIGHT: crate::be_aas_h::C2RustUnnamed_0 = 415;

    pub const BOTLIB_EA_MOVE_LEFT: crate::be_aas_h::C2RustUnnamed_0 = 414;

    pub const BOTLIB_EA_MOVE_BACK: crate::be_aas_h::C2RustUnnamed_0 = 413;

    pub const BOTLIB_EA_MOVE_FORWARD: crate::be_aas_h::C2RustUnnamed_0 = 412;

    pub const BOTLIB_EA_MOVE_DOWN: crate::be_aas_h::C2RustUnnamed_0 = 411;

    pub const BOTLIB_EA_MOVE_UP: crate::be_aas_h::C2RustUnnamed_0 = 410;

    pub const BOTLIB_EA_CROUCH: crate::be_aas_h::C2RustUnnamed_0 = 409;

    pub const BOTLIB_EA_RESPAWN: crate::be_aas_h::C2RustUnnamed_0 = 408;

    pub const BOTLIB_EA_USE: crate::be_aas_h::C2RustUnnamed_0 = 407;

    pub const BOTLIB_EA_ATTACK: crate::be_aas_h::C2RustUnnamed_0 = 406;

    pub const BOTLIB_EA_TALK: crate::be_aas_h::C2RustUnnamed_0 = 405;

    pub const BOTLIB_EA_GESTURE: crate::be_aas_h::C2RustUnnamed_0 = 404;

    pub const BOTLIB_EA_ACTION: crate::be_aas_h::C2RustUnnamed_0 = 403;

    pub const BOTLIB_EA_COMMAND: crate::be_aas_h::C2RustUnnamed_0 = 402;

    pub const BOTLIB_EA_SAY_TEAM: crate::be_aas_h::C2RustUnnamed_0 = 401;

    pub const BOTLIB_EA_SAY: crate::be_aas_h::C2RustUnnamed_0 = 400;

    pub const BOTLIB_AAS_PREDICT_CLIENT_MOVEMENT: crate::be_aas_h::C2RustUnnamed_0 = 318;

    pub const BOTLIB_AAS_SWIMMING: crate::be_aas_h::C2RustUnnamed_0 = 317;

    pub const BOTLIB_AAS_AREA_TRAVEL_TIME_TO_GOAL_AREA: crate::be_aas_h::C2RustUnnamed_0 = 316;

    pub const BOTLIB_AAS_AREA_REACHABILITY: crate::be_aas_h::C2RustUnnamed_0 = 315;

    pub const BOTLIB_AAS_INT_FOR_BSP_EPAIR_KEY: crate::be_aas_h::C2RustUnnamed_0 = 314;

    pub const BOTLIB_AAS_FLOAT_FOR_BSP_EPAIR_KEY: crate::be_aas_h::C2RustUnnamed_0 = 313;

    pub const BOTLIB_AAS_VECTOR_FOR_BSP_EPAIR_KEY: crate::be_aas_h::C2RustUnnamed_0 = 312;

    pub const BOTLIB_AAS_VALUE_FOR_BSP_EPAIR_KEY: crate::be_aas_h::C2RustUnnamed_0 = 311;

    pub const BOTLIB_AAS_NEXT_BSP_ENTITY: crate::be_aas_h::C2RustUnnamed_0 = 310;

    pub const BOTLIB_AAS_POINT_CONTENTS: crate::be_aas_h::C2RustUnnamed_0 = 309;

    pub const BOTLIB_AAS_TRACE_AREAS: crate::be_aas_h::C2RustUnnamed_0 = 308;

    pub const BOTLIB_AAS_POINT_AREA_NUM: crate::be_aas_h::C2RustUnnamed_0 = 307;

    pub const BOTLIB_AAS_TIME: crate::be_aas_h::C2RustUnnamed_0 = 306;

    pub const BOTLIB_AAS_PRESENCE_TYPE_BOUNDING_BOX: crate::be_aas_h::C2RustUnnamed_0 = 305;

    pub const BOTLIB_AAS_INITIALIZED: crate::be_aas_h::C2RustUnnamed_0 = 304;

    pub const BOTLIB_AAS_ENTITY_INFO: crate::be_aas_h::C2RustUnnamed_0 = 303;

    pub const BOTLIB_AAS_AREA_INFO: crate::be_aas_h::C2RustUnnamed_0 = 302;

    pub const BOTLIB_AAS_BBOX_AREAS: crate::be_aas_h::C2RustUnnamed_0 = 301;

    pub const BOTLIB_AAS_ENABLE_ROUTING_AREA: crate::be_aas_h::C2RustUnnamed_0 = 300;

    pub const BOTLIB_USER_COMMAND: crate::be_aas_h::C2RustUnnamed_0 = 211;

    pub const BOTLIB_GET_CONSOLE_MESSAGE: crate::be_aas_h::C2RustUnnamed_0 = 210;

    pub const BOTLIB_GET_SNAPSHOT_ENTITY: crate::be_aas_h::C2RustUnnamed_0 = 209;

    pub const BOTLIB_TEST: crate::be_aas_h::C2RustUnnamed_0 = 208;

    pub const BOTLIB_UPDATENTITY: crate::be_aas_h::C2RustUnnamed_0 = 207;

    pub const BOTLIB_LOAD_MAP: crate::be_aas_h::C2RustUnnamed_0 = 206;

    pub const BOTLIB_START_FRAME: crate::be_aas_h::C2RustUnnamed_0 = 205;

    pub const BOTLIB_PC_ADD_GLOBAL_DEFINE: crate::be_aas_h::C2RustUnnamed_0 = 204;

    pub const BOTLIB_LIBVAR_GET: crate::be_aas_h::C2RustUnnamed_0 = 203;

    pub const BOTLIB_LIBVAR_SET: crate::be_aas_h::C2RustUnnamed_0 = 202;

    pub const BOTLIB_SHUTDOWN: crate::be_aas_h::C2RustUnnamed_0 = 201;

    pub const BOTLIB_SETUP: crate::be_aas_h::C2RustUnnamed_0 = 200;

    pub const G_FS_SEEK: crate::be_aas_h::C2RustUnnamed_0 = 45;

    pub const G_ENTITY_CONTACTCAPSULE: crate::be_aas_h::C2RustUnnamed_0 = 44;

    pub const G_TRACECAPSULE: crate::be_aas_h::C2RustUnnamed_0 = 43;

    pub const G_SNAPVECTOR: crate::be_aas_h::C2RustUnnamed_0 = 42;

    pub const G_REAL_TIME: crate::be_aas_h::C2RustUnnamed_0 = 41;

    pub const G_DEBUG_POLYGON_DELETE: crate::be_aas_h::C2RustUnnamed_0 = 40;

    pub const G_DEBUG_POLYGON_CREATE: crate::be_aas_h::C2RustUnnamed_0 = 39;

    pub const G_FS_GETFILELIST: crate::be_aas_h::C2RustUnnamed_0 = 38;

    pub const G_GET_ENTITY_TOKEN: crate::be_aas_h::C2RustUnnamed_0 = 37;

    pub const G_GET_USERCMD: crate::be_aas_h::C2RustUnnamed_0 = 36;

    pub const G_BOT_FREE_CLIENT: crate::be_aas_h::C2RustUnnamed_0 = 35;

    pub const G_BOT_ALLOCATE_CLIENT: crate::be_aas_h::C2RustUnnamed_0 = 34;

    pub const G_ENTITY_CONTACT: crate::be_aas_h::C2RustUnnamed_0 = 33;

    pub const G_ENTITIES_IN_BOX: crate::be_aas_h::C2RustUnnamed_0 = 32;

    pub const G_UNLINKENTITY: crate::be_aas_h::C2RustUnnamed_0 = 31;

    pub const G_LINKENTITY: crate::be_aas_h::C2RustUnnamed_0 = 30;

    pub const G_AREAS_CONNECTED: crate::be_aas_h::C2RustUnnamed_0 = 29;

    pub const G_ADJUST_AREA_PORTAL_STATE: crate::be_aas_h::C2RustUnnamed_0 = 28;

    pub const G_IN_PVS_IGNORE_PORTALS: crate::be_aas_h::C2RustUnnamed_0 = 27;

    pub const G_IN_PVS: crate::be_aas_h::C2RustUnnamed_0 = 26;

    pub const G_POINT_CONTENTS: crate::be_aas_h::C2RustUnnamed_0 = 25;

    pub const G_TRACE: crate::be_aas_h::C2RustUnnamed_0 = 24;

    pub const G_SET_BRUSH_MODEL: crate::be_aas_h::C2RustUnnamed_0 = 23;

    pub const G_GET_SERVERINFO: crate::be_aas_h::C2RustUnnamed_0 = 22;

    pub const G_SET_USERINFO: crate::be_aas_h::C2RustUnnamed_0 = 21;

    pub const G_GET_USERINFO: crate::be_aas_h::C2RustUnnamed_0 = 20;

    pub const G_GET_CONFIGSTRING: crate::be_aas_h::C2RustUnnamed_0 = 19;

    pub const G_SET_CONFIGSTRING: crate::be_aas_h::C2RustUnnamed_0 = 18;

    pub const G_SEND_SERVER_COMMAND: crate::be_aas_h::C2RustUnnamed_0 = 17;

    pub const G_DROP_CLIENT: crate::be_aas_h::C2RustUnnamed_0 = 16;

    pub const G_LOCATE_GAME_DATA: crate::be_aas_h::C2RustUnnamed_0 = 15;

    pub const G_SEND_CONSOLE_COMMAND: crate::be_aas_h::C2RustUnnamed_0 = 14;

    pub const G_FS_FCLOSE_FILE: crate::be_aas_h::C2RustUnnamed_0 = 13;

    pub const G_FS_WRITE: crate::be_aas_h::C2RustUnnamed_0 = 12;

    pub const G_FS_READ: crate::be_aas_h::C2RustUnnamed_0 = 11;

    pub const G_FS_FOPEN_FILE: crate::be_aas_h::C2RustUnnamed_0 = 10;

    pub const G_ARGV: crate::be_aas_h::C2RustUnnamed_0 = 9;

    pub const G_ARGC: crate::be_aas_h::C2RustUnnamed_0 = 8;

    pub const G_CVAR_VARIABLE_STRING_BUFFER: crate::be_aas_h::C2RustUnnamed_0 = 7;

    pub const G_CVAR_VARIABLE_INTEGER_VALUE: crate::be_aas_h::C2RustUnnamed_0 = 6;

    pub const G_CVAR_SET: crate::be_aas_h::C2RustUnnamed_0 = 5;

    pub const G_CVAR_UPDATE: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const G_CVAR_REGISTER: crate::be_aas_h::C2RustUnnamed_0 = 3;

    pub const G_MILLISECONDS: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const G_ERROR: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const G_PRINT: crate::be_aas_h::C2RustUnnamed_0 = 0;
}
pub mod qfiles_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct lump_t {
        pub fileofs: libc::c_int,
        pub filelen: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct dheader_t {
        pub ident: libc::c_int,
        pub version: libc::c_int,
        pub lumps: [crate::qfiles_h::lump_t; 17],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct dmodel_t {
        pub mins: [libc::c_float; 3],
        pub maxs: [libc::c_float; 3],
        pub firstSurface: libc::c_int,
        pub numSurfaces: libc::c_int,
        pub firstBrush: libc::c_int,
        pub numBrushes: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct dshader_t {
        pub shader: [libc::c_char; 64],
        pub surfaceFlags: libc::c_int,
        pub contentFlags: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct dplane_t {
        pub normal: [libc::c_float; 3],
        pub dist: libc::c_float,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct dnode_t {
        pub planeNum: libc::c_int,
        pub children: [libc::c_int; 2],
        pub mins: [libc::c_int; 3],
        pub maxs: [libc::c_int; 3],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct dleaf_t {
        pub cluster: libc::c_int,
        pub area: libc::c_int,
        pub mins: [libc::c_int; 3],
        pub maxs: [libc::c_int; 3],
        pub firstLeafSurface: libc::c_int,
        pub numLeafSurfaces: libc::c_int,
        pub firstLeafBrush: libc::c_int,
        pub numLeafBrushes: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct dbrushside_t {
        pub planeNum: libc::c_int,
        pub shaderNum: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct dbrush_t {
        pub firstSide: libc::c_int,
        pub numSides: libc::c_int,
        pub shaderNum: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct drawVert_t {
        pub xyz: crate::src::qcommon::q_shared::vec3_t,
        pub st: [libc::c_float; 2],
        pub lightmap: [libc::c_float; 2],
        pub normal: crate::src::qcommon::q_shared::vec3_t,
        pub color: [crate::src::qcommon::q_shared::byte; 4],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct dsurface_t {
        pub shaderNum: libc::c_int,
        pub fogNum: libc::c_int,
        pub surfaceType: libc::c_int,
        pub firstVert: libc::c_int,
        pub numVerts: libc::c_int,
        pub firstIndex: libc::c_int,
        pub numIndexes: libc::c_int,
        pub lightmapNum: libc::c_int,
        pub lightmapX: libc::c_int,
        pub lightmapY: libc::c_int,
        pub lightmapWidth: libc::c_int,
        pub lightmapHeight: libc::c_int,
        pub lightmapOrigin: crate::src::qcommon::q_shared::vec3_t,
        pub lightmapVecs: [crate::src::qcommon::q_shared::vec3_t; 3],
        pub patchWidth: libc::c_int,
        pub patchHeight: libc::c_int,
    }

    pub const MST_FLARE: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const MST_TRIANGLE_SOUP: crate::be_aas_h::C2RustUnnamed_0 = 3;

    pub const MST_PATCH: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const MST_PLANAR: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const MST_BAD: crate::be_aas_h::C2RustUnnamed_0 = 0;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vmHeader_t {
        pub vmMagic: libc::c_int,
        pub instructionCount: libc::c_int,
        pub codeOffset: libc::c_int,
        pub codeLength: libc::c_int,
        pub dataOffset: libc::c_int,
        pub dataLength: libc::c_int,
        pub litLength: libc::c_int,
        pub bssLength: libc::c_int,
        pub jtrgLength: libc::c_int,
    }
}
pub mod qcommon_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct msg_t {
        pub allowoverflow: crate::src::qcommon::q_shared::qboolean,
        pub overflowed: crate::src::qcommon::q_shared::qboolean,
        pub oob: crate::src::qcommon::q_shared::qboolean,
        pub data: *mut crate::src::qcommon::q_shared::byte,
        pub maxsize: libc::c_int,
        pub cursize: libc::c_int,
        pub readcount: libc::c_int,
        pub bit: libc::c_int,
    }

    pub type netadrtype_t = libc::c_uint;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct netadr_t {
        pub type_0: crate::qcommon_h::netadrtype_t,
        pub ip: [crate::src::qcommon::q_shared::byte; 4],
        pub ip6: [crate::src::qcommon::q_shared::byte; 16],
        pub port: libc::c_ushort,
        pub scope_id: libc::c_ulong,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct netchan_t {
        pub sock: crate::qcommon_h::netsrc_t,
        pub dropped: libc::c_int,
        pub remoteAddress: crate::qcommon_h::netadr_t,
        pub qport: libc::c_int,
        pub incomingSequence: libc::c_int,
        pub outgoingSequence: libc::c_int,
        pub fragmentSequence: libc::c_int,
        pub fragmentLength: libc::c_int,
        pub fragmentBuffer: [crate::src::qcommon::q_shared::byte; 16384],
        pub unsentFragments: crate::src::qcommon::q_shared::qboolean,
        pub unsentFragmentStart: libc::c_int,
        pub unsentLength: libc::c_int,
        pub unsentBuffer: [crate::src::qcommon::q_shared::byte; 16384],
        pub challenge: libc::c_int,
        pub lastSentTime: libc::c_int,
        pub lastSentSize: libc::c_int,
        pub compat: crate::src::qcommon::q_shared::qboolean,
    }

    pub type vm_t = crate::vm_local_h::vm_s;

    pub type svc_ops_e = libc::c_uint;

    pub type clc_ops_e = libc::c_uint;

    pub type vmInterpret_t = libc::c_uint;

    pub type dialogType_t = libc::c_uint;

    pub type dialogResult_t = libc::c_uint;

    pub const NA_UNSPEC: crate::qcommon_h::netadrtype_t = 7;

    pub const NA_MULTICAST6: crate::qcommon_h::netadrtype_t = 6;

    pub const NA_IP6: crate::qcommon_h::netadrtype_t = 5;

    pub const NA_IP: crate::qcommon_h::netadrtype_t = 4;

    pub const NA_BROADCAST: crate::qcommon_h::netadrtype_t = 3;

    pub const NA_LOOPBACK: crate::qcommon_h::netadrtype_t = 2;

    pub const NA_BOT: crate::qcommon_h::netadrtype_t = 1;

    pub const NA_BAD: crate::qcommon_h::netadrtype_t = 0;

    pub const svc_voipOpus: crate::qcommon_h::svc_ops_e = 10;

    pub const svc_voipSpeex: crate::qcommon_h::svc_ops_e = 9;

    pub const svc_EOF: crate::qcommon_h::svc_ops_e = 8;

    pub const svc_snapshot: crate::qcommon_h::svc_ops_e = 7;

    pub const svc_download: crate::qcommon_h::svc_ops_e = 6;

    pub const svc_serverCommand: crate::qcommon_h::svc_ops_e = 5;

    pub const svc_baseline: crate::qcommon_h::svc_ops_e = 4;

    pub const svc_configstring: crate::qcommon_h::svc_ops_e = 3;

    pub const svc_gamestate: crate::qcommon_h::svc_ops_e = 2;

    pub const svc_nop: crate::qcommon_h::svc_ops_e = 1;

    pub const svc_bad: crate::qcommon_h::svc_ops_e = 0;

    pub const clc_voipOpus: crate::qcommon_h::clc_ops_e = 7;

    pub const clc_voipSpeex: crate::qcommon_h::clc_ops_e = 6;

    pub const clc_EOF: crate::qcommon_h::clc_ops_e = 5;

    pub const clc_clientCommand: crate::qcommon_h::clc_ops_e = 4;

    pub const clc_moveNoDelta: crate::qcommon_h::clc_ops_e = 3;

    pub const clc_move: crate::qcommon_h::clc_ops_e = 2;

    pub const clc_nop: crate::qcommon_h::clc_ops_e = 1;

    pub const clc_bad: crate::qcommon_h::clc_ops_e = 0;

    pub const TRAP_TESTPRINTFLOAT: crate::be_aas_h::C2RustUnnamed_0 = 113;

    pub const TRAP_TESTPRINTINT: crate::be_aas_h::C2RustUnnamed_0 = 112;

    pub const TRAP_CEIL: crate::be_aas_h::C2RustUnnamed_0 = 111;

    pub const TRAP_FLOOR: crate::be_aas_h::C2RustUnnamed_0 = 110;

    pub const TRAP_PERPENDICULARVECTOR: crate::be_aas_h::C2RustUnnamed_0 = 109;

    pub const TRAP_ANGLEVECTORS: crate::be_aas_h::C2RustUnnamed_0 = 108;

    pub const TRAP_MATRIXMULTIPLY: crate::be_aas_h::C2RustUnnamed_0 = 107;

    pub const TRAP_SQRT: crate::be_aas_h::C2RustUnnamed_0 = 106;

    pub const TRAP_ATAN2: crate::be_aas_h::C2RustUnnamed_0 = 105;

    pub const TRAP_COS: crate::be_aas_h::C2RustUnnamed_0 = 104;

    pub const TRAP_SIN: crate::be_aas_h::C2RustUnnamed_0 = 103;

    pub const TRAP_STRNCPY: crate::be_aas_h::C2RustUnnamed_0 = 102;

    pub const TRAP_MEMCPY: crate::be_aas_h::C2RustUnnamed_0 = 101;

    pub const TRAP_MEMSET: crate::be_aas_h::C2RustUnnamed_0 = 100;

    pub const DT_OK_CANCEL: crate::qcommon_h::dialogType_t = 4;

    pub const DT_YES_NO: crate::qcommon_h::dialogType_t = 3;

    pub const DT_ERROR: crate::qcommon_h::dialogType_t = 2;

    pub const DT_WARNING: crate::qcommon_h::dialogType_t = 1;

    pub const DT_INFO: crate::qcommon_h::dialogType_t = 0;

    pub const DR_CANCEL: crate::qcommon_h::dialogResult_t = 1;

    pub const DR_OK: crate::qcommon_h::dialogResult_t = 0;

    pub const DR_NO: crate::qcommon_h::dialogResult_t = 1;

    pub const DR_YES: crate::qcommon_h::dialogResult_t = 0;

    pub type xcommand_t = Option<unsafe extern "C" fn() -> ()>;

    pub type completionFunc_t =
        Option<unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int) -> ()>;

    pub type netsrc_t = libc::c_uint;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct field_t {
        pub cursor: libc::c_int,
        pub scroll: libc::c_int,
        pub widthInChars: libc::c_int,
        pub buffer: [libc::c_char; 256],
    }

    pub type cpuFeatures_t = libc::c_uint;

    pub type sysEventType_t = libc::c_uint;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct sysEvent_t {
        pub evTime: libc::c_int,
        pub evType: crate::qcommon_h::sysEventType_t,
        pub evValue: libc::c_int,
        pub evValue2: libc::c_int,
        pub evPtrLength: libc::c_int,
        pub evPtr: *mut libc::c_void,
    }

    pub const NS_SERVER: crate::qcommon_h::netsrc_t = 1;

    pub const NS_CLIENT: crate::qcommon_h::netsrc_t = 0;

    pub const CF_ALTIVEC: crate::qcommon_h::cpuFeatures_t = 128;

    pub const CF_SSE2: crate::qcommon_h::cpuFeatures_t = 64;

    pub const CF_SSE: crate::qcommon_h::cpuFeatures_t = 32;

    pub const CF_3DNOW_EXT: crate::qcommon_h::cpuFeatures_t = 16;

    pub const CF_3DNOW: crate::qcommon_h::cpuFeatures_t = 8;

    pub const CF_MMX_EXT: crate::qcommon_h::cpuFeatures_t = 4;

    pub const CF_MMX: crate::qcommon_h::cpuFeatures_t = 2;

    pub const CF_RDTSC: crate::qcommon_h::cpuFeatures_t = 1;

    pub const SE_CONSOLE: crate::qcommon_h::sysEventType_t = 5;

    pub const SE_JOYSTICK_AXIS: crate::qcommon_h::sysEventType_t = 4;

    pub const SE_MOUSE: crate::qcommon_h::sysEventType_t = 3;

    pub const SE_CHAR: crate::qcommon_h::sysEventType_t = 2;

    pub const SE_KEY: crate::qcommon_h::sysEventType_t = 1;

    pub const SE_NONE: crate::qcommon_h::sysEventType_t = 0;

    pub const TAG_SMALL: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const TAG_STATIC: crate::be_aas_h::C2RustUnnamed_0 = 5;

    pub const TAG_GENERAL: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const TAG_RENDERER: crate::be_aas_h::C2RustUnnamed_0 = 3;

    pub const TAG_BOTLIB: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const TAG_FREE: crate::be_aas_h::C2RustUnnamed_0 = 0;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct nodetype {
        pub left: *mut crate::qcommon_h::nodetype,
        pub right: *mut crate::qcommon_h::nodetype,
        pub parent: *mut crate::qcommon_h::nodetype,
        pub next: *mut crate::qcommon_h::nodetype,
        pub prev: *mut crate::qcommon_h::nodetype,
        pub head: *mut *mut crate::qcommon_h::nodetype,
        pub weight: libc::c_int,
        pub symbol: libc::c_int,
    }

    pub type node_t = crate::qcommon_h::nodetype;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct huff_t {
        pub blocNode: libc::c_int,
        pub blocPtrs: libc::c_int,
        pub tree: *mut crate::qcommon_h::node_t,
        pub lhead: *mut crate::qcommon_h::node_t,
        pub ltail: *mut crate::qcommon_h::node_t,
        pub loc: [*mut crate::qcommon_h::node_t; 257],
        pub freelist: *mut *mut crate::qcommon_h::node_t,
        pub nodeList: [crate::qcommon_h::node_t; 768],
        pub nodePtrs: [*mut crate::qcommon_h::node_t; 768],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct huffman_t {
        pub compressor: crate::qcommon_h::huff_t,
        pub decompressor: crate::qcommon_h::huff_t,
    }

    pub const VMI_COMPILED: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const VMI_BYTECODE: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const VMI_NATIVE: crate::be_aas_h::C2RustUnnamed_0 = 0;
}
pub mod be_aas_h {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_clientmove_s {
        pub endpos: crate::src::qcommon::q_shared::vec3_t,
        pub endarea: libc::c_int,
        pub velocity: crate::src::qcommon::q_shared::vec3_t,
        pub trace: crate::be_aas_h::aas_trace_t,
        pub presencetype: libc::c_int,
        pub stopevent: libc::c_int,
        pub endcontents: libc::c_int,
        pub time: libc::c_float,
        pub frames: libc::c_int,
    }
    //a trace is returned when a box is swept through the AAS world

    pub type aas_trace_t = crate::be_aas_h::aas_trace_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_trace_s {
        pub startsolid: crate::src::qcommon::q_shared::qboolean,
        pub fraction: libc::c_float,
        pub endpos: crate::src::qcommon::q_shared::vec3_t,
        pub ent: libc::c_int,
        pub lastarea: libc::c_int,
        pub area: libc::c_int,
        pub planenum: libc::c_int,
    }
    // true if updated this frame
    // entity type
    // entity flags
    // local time
    // time between last and current update
    // number of the entity
    // origin of the entity
    // angles of the model
    // for lerping
    // last visible origin
    // bounding box minimums
    // bounding box maximums
    // ground entity
    // solid type
    // model used
    // weapons, CTF flags, etc
    // model frame number
    // impulse events -- muzzle flashes, footsteps, etc
    // even parameter
    // bit flags
    // determines weapon and flash model, etc
    // mask off ANIM_TOGGLEBIT
    // mask off ANIM_TOGGLEBIT
    // a ground face in the area is hit
    // hit the specified bounding box
    // touching a cluster portal

    pub type aas_clientmove_t = crate::be_aas_h::aas_clientmove_s;

    pub type C2RustUnnamed_0 = libc::c_uint;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_altroutegoal_s {
        pub origin: crate::src::qcommon::q_shared::vec3_t,
        pub areanum: libc::c_int,
        pub starttraveltime: libc::c_ushort,
        pub goaltraveltime: libc::c_ushort,
        pub extratraveltime: libc::c_ushort,
    }

    pub type aas_altroutegoal_t = crate::be_aas_h::aas_altroutegoal_s;

    pub const SOLID_BSP: crate::be_aas_h::C2RustUnnamed_0 = 3;

    pub const SOLID_BBOX: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const SOLID_TRIGGER: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const SOLID_NOT: crate::be_aas_h::C2RustUnnamed_0 = 0;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_entityinfo_s {
        pub valid: libc::c_int,
        pub type_0: libc::c_int,
        pub flags: libc::c_int,
        pub ltime: libc::c_float,
        pub update_time: libc::c_float,
        pub number: libc::c_int,
        pub origin: crate::src::qcommon::q_shared::vec3_t,
        pub angles: crate::src::qcommon::q_shared::vec3_t,
        pub old_origin: crate::src::qcommon::q_shared::vec3_t,
        pub lastvisorigin: crate::src::qcommon::q_shared::vec3_t,
        pub mins: crate::src::qcommon::q_shared::vec3_t,
        pub maxs: crate::src::qcommon::q_shared::vec3_t,
        pub groundent: libc::c_int,
        pub solid: libc::c_int,
        pub modelindex: libc::c_int,
        pub modelindex2: libc::c_int,
        pub frame: libc::c_int,
        pub event: libc::c_int,
        pub eventParm: libc::c_int,
        pub powerups: libc::c_int,
        pub weapon: libc::c_int,
        pub legsAnim: libc::c_int,
        pub torsoAnim: libc::c_int,
    }
    /* Defined in botlib.h

    //bsp_trace_t hit surface
    typedef struct bsp_surface_s
    {
        char name[16];
        int flags;
        int value;
    } bsp_surface_t;

    //a trace is returned when a box is swept through the BSP world
    typedef struct bsp_trace_s
    {
        qboolean		allsolid;	// if true, plane is not valid
        qboolean		startsolid;	// if true, the initial point was in a solid area
        float			fraction;	// time completed, 1.0 = didn't hit anything
        vec3_t			endpos;		// final position
        cplane_t		plane;		// surface normal at impact
        float			exp_dist;	// expanded plane distance
        int				sidenum;	// number of the brush side hit
        bsp_surface_t	surface;	// hit surface
        int				contents;	// contents on other side of surface hit
        int				ent;		// number of entity hit
    } bsp_trace_t;
    //
    */
    //entity info

    pub type aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_areainfo_s {
        pub contents: libc::c_int,
        pub flags: libc::c_int,
        pub presencetype: libc::c_int,
        pub cluster: libc::c_int,
        pub mins: crate::src::qcommon::q_shared::vec3_t,
        pub maxs: crate::src::qcommon::q_shared::vec3_t,
        pub center: crate::src::qcommon::q_shared::vec3_t,
    }

    pub type aas_areainfo_t = crate::be_aas_h::aas_areainfo_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_predictroute_s {
        pub endpos: crate::src::qcommon::q_shared::vec3_t,
        pub endarea: libc::c_int,
        pub stopevent: libc::c_int,
        pub endcontents: libc::c_int,
        pub endtravelflags: libc::c_int,
        pub numareas: libc::c_int,
        pub time: libc::c_int,
    }
}
pub mod stdlib {
    extern "C" {
        #[no_mangle]
        pub fn __assert_fail(
            __assertion: *const libc::c_char,
            __file: *const libc::c_char,
            __line: libc::c_uint,
            __function: *const libc::c_char,
        ) -> !;
        #[no_mangle]
        pub fn tolower(_: libc::c_int) -> libc::c_int;

        #[no_mangle]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;

        #[no_mangle]
        pub fn toupper(_: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;

        #[no_mangle]
        pub fn dlclose(__handle: *mut libc::c_void) -> libc::c_int;

        #[no_mangle]
        pub fn dlsym(__handle: *mut libc::c_void, __name: *const libc::c_char)
            -> *mut libc::c_void;

        #[no_mangle]
        pub fn dlerror() -> *mut libc::c_char;
        #[no_mangle]
        pub fn __errno_location() -> *mut libc::c_int;
        #[no_mangle]
        pub fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;

        #[no_mangle]
        pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
        #[no_mangle]
        pub fn fesetround(__rounding_direction: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn if_nametoindex(__ifname: *const libc::c_char) -> libc::c_uint;
        #[no_mangle]
        pub fn getifaddrs(__ifap: *mut *mut crate::stdlib::ifaddrs) -> libc::c_int;

        #[no_mangle]
        pub fn freeifaddrs(__ifa: *mut crate::stdlib::ifaddrs);
        #[no_mangle]
        pub static in6addr_any: crate::stdlib::in6_addr;

        #[no_mangle]
        pub fn ntohs(__netshort: crate::stdlib::uint16_t) -> crate::stdlib::uint16_t;

        #[no_mangle]
        pub fn htons(__hostshort: crate::stdlib::uint16_t) -> crate::stdlib::uint16_t;
        pub type __dirstream;

        #[no_mangle]
        pub fn opendir(__name: *const libc::c_char) -> *mut crate::stdlib::DIR;

        #[no_mangle]
        pub fn closedir(__dirp: *mut crate::stdlib::DIR) -> libc::c_int;

        #[no_mangle]
        pub fn readdir(__dirp: *mut crate::stdlib::DIR) -> *mut crate::stdlib::dirent;
        #[no_mangle]
        pub fn _setjmp(_: *mut crate::stdlib::__jmp_buf_tag) -> libc::c_int;

        #[no_mangle]
        pub fn longjmp(_: *mut crate::stdlib::__jmp_buf_tag, _: libc::c_int) -> !;
        #[no_mangle]
        pub fn tcflush(__fd: libc::c_int, __queue_selector: libc::c_int) -> libc::c_int;

        #[no_mangle]
        pub fn tcsetattr(
            __fd: libc::c_int,
            __optional_actions: libc::c_int,
            __termios_p: *const crate::stdlib::termios,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn tcgetattr(
            __fd: libc::c_int,
            __termios_p: *mut crate::stdlib::termios,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
        #[no_mangle]
        pub fn dirname(__path: *mut libc::c_char) -> *mut libc::c_char;

        #[no_mangle]
        pub fn __xpg_basename(__path: *mut libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        pub fn ceil(_: libc::c_double) -> libc::c_double;

        #[no_mangle]
        pub fn floor(_: libc::c_double) -> libc::c_double;

        #[no_mangle]
        pub fn sqrt(_: libc::c_double) -> libc::c_double;

        #[no_mangle]
        pub fn fabsf(_: libc::c_float) -> libc::c_float;

        #[no_mangle]
        pub fn tan(_: libc::c_double) -> libc::c_double;

        #[no_mangle]
        pub fn fabs(_: libc::c_double) -> libc::c_double;

        #[no_mangle]
        pub fn acos(_: libc::c_double) -> libc::c_double;

        #[no_mangle]
        pub fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;

        #[no_mangle]
        pub fn cos(_: libc::c_double) -> libc::c_double;

        #[no_mangle]
        pub fn sin(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn mprotect(
            __addr: *mut libc::c_void,
            __len: crate::stddef_h::size_t,
            __prot: libc::c_int,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn mmap(
            __addr: *mut libc::c_void,
            __len: crate::stddef_h::size_t,
            __prot: libc::c_int,
            __flags: libc::c_int,
            __fd: libc::c_int,
            __offset: crate::stdlib::__off_t,
        ) -> *mut libc::c_void;

        #[no_mangle]
        pub fn munmap(__addr: *mut libc::c_void, __len: crate::stddef_h::size_t) -> libc::c_int;
        #[no_mangle]
        pub fn gethostbyname(__name: *const libc::c_char) -> *mut crate::stdlib::hostent;

        #[no_mangle]
        pub fn getaddrinfo(
            __name: *const libc::c_char,
            __service: *const libc::c_char,
            __req: *const crate::stdlib::addrinfo,
            __pai: *mut *mut crate::stdlib::addrinfo,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn getnameinfo(
            __sa: *const crate::stdlib::sockaddr,
            __salen: crate::stdlib::socklen_t,
            __host: *mut libc::c_char,
            __hostlen: crate::stdlib::socklen_t,
            __serv: *mut libc::c_char,
            __servlen: crate::stdlib::socklen_t,
            __flags: libc::c_int,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;

        #[no_mangle]
        pub fn freeaddrinfo(__ai: *mut crate::stdlib::addrinfo);
        #[no_mangle]
        pub fn getpwuid(__uid: crate::stdlib::__uid_t) -> *mut crate::stdlib::passwd;
        #[no_mangle]
        pub fn select(
            __nfds: libc::c_int,
            __readfds: *mut crate::stdlib::fd_set,
            __writefds: *mut crate::stdlib::fd_set,
            __exceptfds: *mut crate::stdlib::fd_set,
            __timeout: *mut crate::stdlib::timeval,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn kill(__pid: crate::stdlib::__pid_t, __sig: libc::c_int) -> libc::c_int;

        #[no_mangle]
        pub fn signal(
            __sig: libc::c_int,
            __handler: crate::stdlib::__sighandler_t,
        ) -> crate::stdlib::__sighandler_t;
        #[no_mangle]
        pub fn ferror(__stream: *mut crate::stdlib::FILE) -> libc::c_int;

        #[no_mangle]
        pub static mut stderr: *mut crate::stdlib::FILE;

        #[no_mangle]
        pub fn fputs(__s: *const libc::c_char, __stream: *mut crate::stdlib::FILE) -> libc::c_int;

        #[no_mangle]
        pub fn fileno(__stream: *mut crate::stdlib::FILE) -> libc::c_int;

        #[no_mangle]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;

        #[no_mangle]
        pub static mut stdout: *mut crate::stdlib::FILE;

        #[no_mangle]
        pub fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;

        #[no_mangle]
        pub fn vsnprintf(
            _: *mut libc::c_char,
            _: libc::c_ulong,
            _: *const libc::c_char,
            _: ::std::ffi::VaList,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn fprintf(_: *mut crate::stdlib::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;

        #[no_mangle]
        pub fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;

        #[no_mangle]
        pub fn remove(__filename: *const libc::c_char) -> libc::c_int;

        #[no_mangle]
        pub fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;

        #[no_mangle]
        pub fn setvbuf(
            __stream: *mut crate::stdlib::FILE,
            __buf: *mut libc::c_char,
            __modes: libc::c_int,
            __n: crate::stddef_h::size_t,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn fread(
            __ptr: *mut libc::c_void,
            __size: crate::stddef_h::size_t,
            __n: crate::stddef_h::size_t,
            __stream: *mut crate::stdlib::FILE,
        ) -> crate::stddef_h::size_t;

        #[no_mangle]
        pub fn fwrite(
            __ptr: *const libc::c_void,
            __size: crate::stddef_h::size_t,
            __n: crate::stddef_h::size_t,
            __s: *mut crate::stdlib::FILE,
        ) -> crate::stddef_h::size_t;

        #[no_mangle]
        pub fn fseek(
            __stream: *mut crate::stdlib::FILE,
            __off: libc::c_long,
            __whence: libc::c_int,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn ftell(__stream: *mut crate::stdlib::FILE) -> libc::c_long;

        #[no_mangle]
        pub fn fclose(__stream: *mut crate::stdlib::FILE) -> libc::c_int;

        #[no_mangle]
        pub fn fflush(__stream: *mut crate::stdlib::FILE) -> libc::c_int;

        #[no_mangle]
        pub fn fopen(
            __filename: *const libc::c_char,
            __modes: *const libc::c_char,
        ) -> *mut crate::stdlib::FILE;

        #[no_mangle]
        pub fn vfprintf(
            _: *mut crate::stdlib::FILE,
            _: *const libc::c_char,
            _: ::std::ffi::VaList,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn setenv(
            __name: *const libc::c_char,
            __value: *const libc::c_char,
            __replace: libc::c_int,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn unsetenv(__name: *const libc::c_char) -> libc::c_int;

        #[no_mangle]
        pub fn srand(__seed: libc::c_uint);

        #[no_mangle]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;

        #[no_mangle]
        pub fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;

        #[no_mangle]
        pub fn strtod(
            __nptr: *const libc::c_char,
            __endptr: *mut *mut libc::c_char,
        ) -> libc::c_double;

        #[no_mangle]
        pub fn exit(_: libc::c_int) -> !;

        #[no_mangle]
        pub fn atof(__nptr: *const libc::c_char) -> libc::c_double;

        #[no_mangle]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;

        #[no_mangle]
        pub fn abs(_: libc::c_int) -> libc::c_int;

        #[no_mangle]
        pub fn rand() -> libc::c_int;

        #[no_mangle]
        pub fn qsort(
            __base: *mut libc::c_void,
            __nmemb: crate::stddef_h::size_t,
            __size: crate::stddef_h::size_t,
            __compar: crate::stdlib::__compar_fn_t,
        );

        #[no_mangle]
        pub fn free(__ptr: *mut libc::c_void);

        #[no_mangle]
        pub fn labs(_: libc::c_long) -> libc::c_long;
        #[no_mangle]
        pub fn memcmp(
            _: *const libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;

        #[no_mangle]
        pub fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

        #[no_mangle]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

        #[no_mangle]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;

        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;

        #[no_mangle]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

        #[no_mangle]
        pub fn strncpy(
            _: *mut libc::c_char,
            _: *const libc::c_char,
            _: libc::c_ulong,
        ) -> *mut libc::c_char;

        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;

        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;

        #[no_mangle]
        pub fn memmove(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;

        #[no_mangle]
        pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

        #[no_mangle]
        pub fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

        #[no_mangle]
        pub fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

        #[no_mangle]
        pub fn strncmp(
            _: *const libc::c_char,
            _: *const libc::c_char,
            _: libc::c_ulong,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn strncat(
            _: *mut libc::c_char,
            _: *const libc::c_char,
            _: libc::c_ulong,
        ) -> *mut libc::c_char;
        pub type _IO_wide_data;

        pub type _IO_codecvt;

        pub type _IO_marker;
        #[no_mangle]
        pub fn connect(
            __fd: libc::c_int,
            __addr: *const crate::stdlib::sockaddr,
            __len: crate::stdlib::socklen_t,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn send(
            __fd: libc::c_int,
            __buf: *const libc::c_void,
            __n: crate::stddef_h::size_t,
            __flags: libc::c_int,
        ) -> crate::stdlib::ssize_t;

        #[no_mangle]
        pub fn recv(
            __fd: libc::c_int,
            __buf: *mut libc::c_void,
            __n: crate::stddef_h::size_t,
            __flags: libc::c_int,
        ) -> crate::stdlib::ssize_t;

        #[no_mangle]
        pub fn setsockopt(
            __fd: libc::c_int,
            __level: libc::c_int,
            __optname: libc::c_int,
            __optval: *const libc::c_void,
            __optlen: crate::stdlib::socklen_t,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn socket(
            __domain: libc::c_int,
            __type: libc::c_int,
            __protocol: libc::c_int,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn recvfrom(
            __fd: libc::c_int,
            __buf: *mut libc::c_void,
            __n: crate::stddef_h::size_t,
            __flags: libc::c_int,
            __addr: *mut crate::stdlib::sockaddr,
            __addr_len: *mut crate::stdlib::socklen_t,
        ) -> crate::stdlib::ssize_t;

        #[no_mangle]
        pub fn sendto(
            __fd: libc::c_int,
            __buf: *const libc::c_void,
            __n: crate::stddef_h::size_t,
            __flags: libc::c_int,
            __addr: *const crate::stdlib::sockaddr,
            __addr_len: crate::stdlib::socklen_t,
        ) -> crate::stdlib::ssize_t;

        #[no_mangle]
        pub fn bind(
            __fd: libc::c_int,
            __addr: *const crate::stdlib::sockaddr,
            __len: crate::stdlib::socklen_t,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn stat(__file: *const libc::c_char, __buf: *mut crate::stdlib::stat) -> libc::c_int;

        #[no_mangle]
        pub fn mkdir(__path: *const libc::c_char, __mode: crate::stdlib::__mode_t) -> libc::c_int;

        #[no_mangle]
        pub fn mkfifo(__path: *const libc::c_char, __mode: crate::stdlib::__mode_t) -> libc::c_int;
        #[no_mangle]
        pub fn localtime(__timer: *const crate::stdlib::time_t) -> *mut crate::stdlib::tm;

        #[no_mangle]
        pub fn asctime(__tp: *const crate::stdlib::tm) -> *mut libc::c_char;

        #[no_mangle]
        pub fn clock() -> crate::stdlib::clock_t;

        #[no_mangle]
        pub fn gettimeofday(
            __tv: *mut crate::stdlib::timeval,
            __tz: crate::stdlib::__timezone_ptr_t,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn time(__timer: *mut crate::stdlib::time_t) -> crate::stdlib::time_t;

        #[no_mangle]
        pub fn ctime(__timer: *const crate::stdlib::time_t) -> *mut libc::c_char;
        #[no_mangle]
        pub fn read(
            __fd: libc::c_int,
            __buf: *mut libc::c_void,
            __nbytes: crate::stddef_h::size_t,
        ) -> crate::stdlib::ssize_t;

        #[no_mangle]
        pub fn write(
            __fd: libc::c_int,
            __buf: *const libc::c_void,
            __n: crate::stddef_h::size_t,
        ) -> crate::stdlib::ssize_t;

        #[no_mangle]
        pub fn execvp(__file: *const libc::c_char, __argv: *const *mut libc::c_char)
            -> libc::c_int;

        #[no_mangle]
        pub fn getpid() -> crate::stdlib::__pid_t;

        #[no_mangle]
        pub fn fork() -> crate::stdlib::__pid_t;

        #[no_mangle]
        pub fn getuid() -> crate::stdlib::__uid_t;

        #[no_mangle]
        pub fn isatty(__fd: libc::c_int) -> libc::c_int;

        #[no_mangle]
        pub fn getcwd(
            __buf: *mut libc::c_char,
            __size: crate::stddef_h::size_t,
        ) -> *mut libc::c_char;

        #[no_mangle]
        pub fn usleep(__useconds: crate::stdlib::__useconds_t) -> libc::c_int;

        #[no_mangle]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn wait(__stat_loc: *mut libc::c_int) -> crate::stdlib::__pid_t;
        pub type internal_state;

        #[no_mangle]
        pub fn inflate(strm: crate::stdlib::z_streamp, flush: libc::c_int) -> libc::c_int;

        #[no_mangle]
        pub fn inflateEnd(strm: crate::stdlib::z_streamp) -> libc::c_int;

        #[no_mangle]
        pub fn crc32(
            crc: crate::stdlib::uLong,
            buf: *const crate::stdlib::Bytef,
            len: crate::stdlib::uInt,
        ) -> crate::stdlib::uLong;

        #[no_mangle]
        pub fn inflateInit2_(
            strm: crate::stdlib::z_streamp,
            windowBits: libc::c_int,
            version: *const libc::c_char,
            stream_size: libc::c_int,
        ) -> libc::c_int;
    }
    pub type FILE = crate::stdlib::_IO_FILE;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct __sigset_t {
        pub __val: [libc::c_ulong; 16],
    }
    pub type clock_t = crate::stdlib::__clock_t;
    pub const _ISalnum: crate::be_aas_h::C2RustUnnamed_0 = 8;

    pub const _ISpunct: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const _IScntrl: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const _ISblank: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const _ISgraph: crate::be_aas_h::C2RustUnnamed_0 = 32768;

    pub const _ISprint: crate::be_aas_h::C2RustUnnamed_0 = 16384;

    pub const _ISspace: crate::be_aas_h::C2RustUnnamed_0 = 8192;

    pub const _ISxdigit: crate::be_aas_h::C2RustUnnamed_0 = 4096;

    pub const _ISdigit: crate::be_aas_h::C2RustUnnamed_0 = 2048;

    pub const _ISalpha: crate::be_aas_h::C2RustUnnamed_0 = 1024;

    pub const _ISlower: crate::be_aas_h::C2RustUnnamed_0 = 512;

    pub const _ISupper: crate::be_aas_h::C2RustUnnamed_0 = 256;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct dirent {
        pub d_ino: crate::stdlib::__ino_t,
        pub d_off: crate::stdlib::__off_t,
        pub d_reclen: libc::c_ushort,
        pub d_type: libc::c_uchar,
        pub d_name: [libc::c_char; 256],
    }
    pub const IFF_UP: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const IFF_DYNAMIC: crate::be_aas_h::C2RustUnnamed_0 = 32768;

    pub const IFF_AUTOMEDIA: crate::be_aas_h::C2RustUnnamed_0 = 16384;

    pub const IFF_PORTSEL: crate::be_aas_h::C2RustUnnamed_0 = 8192;

    pub const IFF_MULTICAST: crate::be_aas_h::C2RustUnnamed_0 = 4096;

    pub const IFF_SLAVE: crate::be_aas_h::C2RustUnnamed_0 = 2048;

    pub const IFF_MASTER: crate::be_aas_h::C2RustUnnamed_0 = 1024;

    pub const IFF_ALLMULTI: crate::be_aas_h::C2RustUnnamed_0 = 512;

    pub const IFF_PROMISC: crate::be_aas_h::C2RustUnnamed_0 = 256;

    pub const IFF_NOARP: crate::be_aas_h::C2RustUnnamed_0 = 128;

    pub const IFF_RUNNING: crate::be_aas_h::C2RustUnnamed_0 = 64;

    pub const IFF_NOTRAILERS: crate::be_aas_h::C2RustUnnamed_0 = 32;

    pub const IFF_POINTOPOINT: crate::be_aas_h::C2RustUnnamed_0 = 16;

    pub const IFF_LOOPBACK: crate::be_aas_h::C2RustUnnamed_0 = 8;

    pub const IFF_DEBUG: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const IFF_BROADCAST: crate::be_aas_h::C2RustUnnamed_0 = 2;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union C2RustUnnamed_27 {
        pub ifu_broadaddr: *mut crate::stdlib::sockaddr,
        pub ifu_dstaddr: *mut crate::stdlib::sockaddr,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ifaddrs {
        pub ifa_next: *mut crate::stdlib::ifaddrs,
        pub ifa_name: *mut libc::c_char,
        pub ifa_flags: libc::c_uint,
        pub ifa_addr: *mut crate::stdlib::sockaddr,
        pub ifa_netmask: *mut crate::stdlib::sockaddr,
        pub ifa_ifu: crate::stdlib::C2RustUnnamed_27,
        pub ifa_data: *mut libc::c_void,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union C2RustUnnamed_25 {
        pub __u6_addr8: [crate::stdlib::uint8_t; 16],
        pub __u6_addr16: [crate::stdlib::uint16_t; 8],
        pub __u6_addr32: [crate::stdlib::uint32_t; 4],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ipv6_mreq {
        pub ipv6mr_multiaddr: crate::stdlib::in6_addr,
        pub ipv6mr_interface: libc::c_uint,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct in6_addr {
        pub __in6_u: crate::stdlib::C2RustUnnamed_25,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct sockaddr_in6 {
        pub sin6_family: crate::stdlib::sa_family_t,
        pub sin6_port: crate::stdlib::in_port_t,
        pub sin6_flowinfo: crate::stdlib::uint32_t,
        pub sin6_addr: crate::stdlib::in6_addr,
        pub sin6_scope_id: crate::stdlib::uint32_t,
    }

    pub type in_port_t = crate::stdlib::uint16_t;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct sockaddr_in {
        pub sin_family: crate::stdlib::sa_family_t,
        pub sin_port: crate::stdlib::in_port_t,
        pub sin_addr: crate::stdlib::in_addr,
        pub sin_zero: [libc::c_uchar; 8],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct in_addr {
        pub s_addr: crate::stdlib::in_addr_t,
    }

    pub type in_addr_t = crate::stdlib::uint32_t;

    pub const IPPROTO_TCP: crate::be_aas_h::C2RustUnnamed_0 = 6;

    pub const IPPROTO_UDP: crate::be_aas_h::C2RustUnnamed_0 = 17;

    pub const IPPROTO_IPV6: crate::be_aas_h::C2RustUnnamed_0 = 41;

    pub const IPPROTO_MAX: crate::be_aas_h::C2RustUnnamed_0 = 256;

    pub const IPPROTO_RAW: crate::be_aas_h::C2RustUnnamed_0 = 255;

    pub const IPPROTO_MPLS: crate::be_aas_h::C2RustUnnamed_0 = 137;

    pub const IPPROTO_UDPLITE: crate::be_aas_h::C2RustUnnamed_0 = 136;

    pub const IPPROTO_SCTP: crate::be_aas_h::C2RustUnnamed_0 = 132;

    pub const IPPROTO_COMP: crate::be_aas_h::C2RustUnnamed_0 = 108;

    pub const IPPROTO_PIM: crate::be_aas_h::C2RustUnnamed_0 = 103;

    pub const IPPROTO_ENCAP: crate::be_aas_h::C2RustUnnamed_0 = 98;

    pub const IPPROTO_BEETPH: crate::be_aas_h::C2RustUnnamed_0 = 94;

    pub const IPPROTO_MTP: crate::be_aas_h::C2RustUnnamed_0 = 92;

    pub const IPPROTO_AH: crate::be_aas_h::C2RustUnnamed_0 = 51;

    pub const IPPROTO_ESP: crate::be_aas_h::C2RustUnnamed_0 = 50;

    pub const IPPROTO_GRE: crate::be_aas_h::C2RustUnnamed_0 = 47;

    pub const IPPROTO_RSVP: crate::be_aas_h::C2RustUnnamed_0 = 46;

    pub const IPPROTO_DCCP: crate::be_aas_h::C2RustUnnamed_0 = 33;

    pub const IPPROTO_TP: crate::be_aas_h::C2RustUnnamed_0 = 29;

    pub const IPPROTO_IDP: crate::be_aas_h::C2RustUnnamed_0 = 22;

    pub const IPPROTO_PUP: crate::be_aas_h::C2RustUnnamed_0 = 12;

    pub const IPPROTO_EGP: crate::be_aas_h::C2RustUnnamed_0 = 8;

    pub const IPPROTO_IPIP: crate::be_aas_h::C2RustUnnamed_0 = 4;

    pub const IPPROTO_IGMP: crate::be_aas_h::C2RustUnnamed_0 = 2;

    pub const IPPROTO_ICMP: crate::be_aas_h::C2RustUnnamed_0 = 1;

    pub const IPPROTO_IP: crate::be_aas_h::C2RustUnnamed_0 = 0;
    pub type DIR = crate::stdlib::__dirstream;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct __jmp_buf_tag {
        pub __jmpbuf: crate::stdlib::__jmp_buf,
        pub __mask_was_saved: libc::c_int,
        pub __saved_mask: crate::stdlib::__sigset_t,
    }

    pub type jmp_buf = [crate::stdlib::__jmp_buf_tag; 1];
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct addrinfo {
        pub ai_flags: libc::c_int,
        pub ai_family: libc::c_int,
        pub ai_socktype: libc::c_int,
        pub ai_protocol: libc::c_int,
        pub ai_addrlen: crate::stdlib::socklen_t,
        pub ai_addr: *mut crate::stdlib::sockaddr,
        pub ai_canonname: *mut libc::c_char,
        pub ai_next: *mut crate::stdlib::addrinfo,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct hostent {
        pub h_name: *mut libc::c_char,
        pub h_aliases: *mut *mut libc::c_char,
        pub h_addrtype: libc::c_int,
        pub h_length: libc::c_int,
        pub h_addr_list: *mut *mut libc::c_char,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct passwd {
        pub pw_name: *mut libc::c_char,
        pub pw_passwd: *mut libc::c_char,
        pub pw_uid: crate::stdlib::__uid_t,
        pub pw_gid: crate::stdlib::__gid_t,
        pub pw_gecos: *mut libc::c_char,
        pub pw_dir: *mut libc::c_char,
        pub pw_shell: *mut libc::c_char,
    }
    pub type __fd_mask = libc::c_long;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct fd_set {
        pub __fds_bits: [crate::stdlib::__fd_mask; 16],
    }
    pub type __jmp_buf = [libc::c_long; 8];
    pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
    pub type sa_family_t = libc::c_ushort;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct sockaddr {
        pub sa_family: crate::stdlib::sa_family_t,
        pub sa_data: [libc::c_char; 14],
    }

    pub type socklen_t = crate::stdlib::__socklen_t;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct sockaddr_storage {
        pub ss_family: crate::stdlib::sa_family_t,
        pub __ss_padding: [libc::c_char; 118],
        pub __ss_align: libc::c_ulong,
    }
    pub type __socket_type = libc::c_uint;

    pub const SOCK_DGRAM: crate::stdlib::__socket_type = 2;

    pub const SOCK_STREAM: crate::stdlib::__socket_type = 1;

    pub const SOCK_NONBLOCK: crate::stdlib::__socket_type = 2048;

    pub const SOCK_CLOEXEC: crate::stdlib::__socket_type = 524288;

    pub const SOCK_PACKET: crate::stdlib::__socket_type = 10;

    pub const SOCK_DCCP: crate::stdlib::__socket_type = 6;

    pub const SOCK_SEQPACKET: crate::stdlib::__socket_type = 5;

    pub const SOCK_RDM: crate::stdlib::__socket_type = 4;

    pub const SOCK_RAW: crate::stdlib::__socket_type = 3;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct stat {
        pub st_dev: crate::stdlib::__dev_t,
        pub st_ino: crate::stdlib::__ino_t,
        pub st_nlink: crate::stdlib::__nlink_t,
        pub st_mode: crate::stdlib::__mode_t,
        pub st_uid: crate::stdlib::__uid_t,
        pub st_gid: crate::stdlib::__gid_t,
        pub __pad0: libc::c_int,
        pub st_rdev: crate::stdlib::__dev_t,
        pub st_size: crate::stdlib::__off_t,
        pub st_blksize: crate::stdlib::__blksize_t,
        pub st_blocks: crate::stdlib::__blkcnt_t,
        pub st_atim: crate::stdlib::timespec,
        pub st_mtim: crate::stdlib::timespec,
        pub st_ctim: crate::stdlib::timespec,
        pub __glibc_reserved: [crate::stdlib::__syscall_slong_t; 3],
    }
    pub type intptr_t = libc::c_long;
    pub type int32_t = crate::stdlib::__int32_t;
    pub type uint16_t = crate::stdlib::__uint16_t;

    pub type uint8_t = crate::stdlib::__uint8_t;

    pub type uint32_t = crate::stdlib::__uint32_t;
    pub type ssize_t = crate::stdlib::__ssize_t;

    pub type off_t = crate::stdlib::__off_t;
    pub type __compar_fn_t =
        Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut crate::stdlib::_IO_marker,
        pub _chain: *mut crate::stdlib::_IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: crate::stdlib::__off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: crate::stdlib::__off64_t,
        pub _codecvt: *mut crate::stdlib::_IO_codecvt,
        pub _wide_data: *mut crate::stdlib::_IO_wide_data,
        pub _freeres_list: *mut crate::stdlib::_IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: crate::stddef_h::size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }

    pub type _IO_lock_t = ();
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct timespec {
        pub tv_sec: crate::stdlib::__time_t,
        pub tv_nsec: crate::stdlib::__syscall_slong_t,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct timeval {
        pub tv_sec: crate::stdlib::__time_t,
        pub tv_usec: crate::stdlib::__suseconds_t,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct tm {
        pub tm_sec: libc::c_int,
        pub tm_min: libc::c_int,
        pub tm_hour: libc::c_int,
        pub tm_mday: libc::c_int,
        pub tm_mon: libc::c_int,
        pub tm_year: libc::c_int,
        pub tm_wday: libc::c_int,
        pub tm_yday: libc::c_int,
        pub tm_isdst: libc::c_int,
        pub tm_gmtoff: libc::c_long,
        pub tm_zone: *const libc::c_char,
    }
    pub type pid_t = crate::stdlib::__pid_t;
    pub type tcflag_t = libc::c_uint;

    pub type cc_t = libc::c_uchar;

    pub type speed_t = libc::c_uint;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct termios {
        pub c_iflag: crate::stdlib::tcflag_t,
        pub c_oflag: crate::stdlib::tcflag_t,
        pub c_cflag: crate::stdlib::tcflag_t,
        pub c_lflag: crate::stdlib::tcflag_t,
        pub c_line: crate::stdlib::cc_t,
        pub c_cc: [crate::stdlib::cc_t; 32],
        pub c_ispeed: crate::stdlib::speed_t,
        pub c_ospeed: crate::stdlib::speed_t,
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct timezone {
        pub tz_minuteswest: libc::c_int,
        pub tz_dsttime: libc::c_int,
    }

    pub type __timezone_ptr_t = *mut crate::stdlib::timezone;
    pub type time_t = crate::stdlib::__time_t;
    pub type __uint16_t = libc::c_ushort;

    pub type __suseconds_t = libc::c_long;

    pub type __ssize_t = libc::c_long;

    pub type __socklen_t = libc::c_uint;

    pub type __dev_t = libc::c_ulong;

    pub type __uid_t = libc::c_uint;

    pub type __gid_t = libc::c_uint;

    pub type __ino_t = libc::c_ulong;

    pub type __mode_t = libc::c_uint;

    pub type __nlink_t = libc::c_ulong;

    pub type __blksize_t = libc::c_long;

    pub type __blkcnt_t = libc::c_long;

    pub type __syscall_slong_t = libc::c_long;

    pub type __pid_t = libc::c_int;

    pub type __useconds_t = libc::c_uint;

    pub type __uint8_t = libc::c_uchar;

    pub type __int32_t = libc::c_int;

    pub type __uint32_t = libc::c_uint;

    pub type __off_t = libc::c_long;

    pub type __off64_t = libc::c_long;

    pub type __clock_t = libc::c_long;

    pub type __time_t = libc::c_long;
    pub type voidpf = *mut libc::c_void;

    pub type voidp = *mut libc::c_void;

    pub type uLong = libc::c_ulong;

    pub type uInt = libc::c_uint;

    pub type Byte = libc::c_uchar;

    pub type Bytef = crate::stdlib::Byte;
    pub type alloc_func = Option<
        unsafe extern "C" fn(
            _: crate::stdlib::voidpf,
            _: crate::stdlib::uInt,
            _: crate::stdlib::uInt,
        ) -> crate::stdlib::voidpf,
    >;

    pub type free_func =
        Option<unsafe extern "C" fn(_: crate::stdlib::voidpf, _: crate::stdlib::voidpf) -> ()>;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct z_stream_s {
        pub next_in: *mut crate::stdlib::Bytef,
        pub avail_in: crate::stdlib::uInt,
        pub total_in: crate::stdlib::uLong,
        pub next_out: *mut crate::stdlib::Bytef,
        pub avail_out: crate::stdlib::uInt,
        pub total_out: crate::stdlib::uLong,
        pub msg: *mut libc::c_char,
        pub state: *mut crate::stdlib::internal_state,
        pub zalloc: crate::stdlib::alloc_func,
        pub zfree: crate::stdlib::free_func,
        pub opaque: crate::stdlib::voidpf,
        pub data_type: libc::c_int,
        pub adler: crate::stdlib::uLong,
        pub reserved: crate::stdlib::uLong,
    }

    pub type z_stream = crate::stdlib::z_stream_s;

    pub type z_streamp = *mut crate::stdlib::z_stream;
}
#[macro_use]
extern crate c2rust_asm_casts;
extern crate libc;

pub mod src {
    pub mod asm {
        pub mod ftola;
        pub mod snapvector;
    } // mod asm
    pub mod botlib {
        pub mod be_aas_bspq3;
        pub mod be_aas_cluster;
        pub mod be_aas_debug;
        pub mod be_aas_entity;
        pub mod be_aas_file;
        pub mod be_aas_main;
        pub mod be_aas_move;
        pub mod be_aas_optimize;
        pub mod be_aas_reach;
        pub mod be_aas_route;
        pub mod be_aas_routealt;
        pub mod be_aas_sample;
        pub mod be_ai_char;
        pub mod be_ai_chat;
        pub mod be_ai_gen;
        pub mod be_ai_goal;
        pub mod be_ai_move;
        pub mod be_ai_weap;
        pub mod be_ai_weight;
        pub mod be_ea;
        pub mod be_interface;
        pub mod bg_lib;
        pub mod l_crc;
        pub mod l_libvar;
        pub mod l_log;
        pub mod l_memory;
        pub mod l_precomp;
        pub mod l_script;
        pub mod l_struct;
    } // mod botlib
    pub mod null {
        pub mod null_client;
        pub mod null_input;
        pub mod null_snddma;
    } // mod null
    pub mod qcommon {
        pub mod cm_load;
        pub mod cm_patch;
        pub mod cm_polylib;
        pub mod cm_test;
        pub mod cm_trace;
        pub mod cmd;
        pub mod common;
        pub mod cvar;
        pub mod files;
        pub mod huffman;
        pub mod ioapi;
        pub mod md4;
        pub mod msg;
        pub mod net_chan;
        pub mod net_ip;
        pub mod q_math;
        pub mod q_shared;
        pub mod unzip;
    } // mod qcommon
    pub mod server {
        pub mod sv_bot;
        pub mod sv_ccmds;
        pub mod sv_client;
        pub mod sv_game;
        pub mod sv_init;
        pub mod sv_main;
        pub mod sv_net_chan;
        pub mod sv_snapshot;
        pub mod sv_world;
    } // mod server
    pub mod sys {
        pub mod con_log;
        pub mod con_tty;
        pub mod sys_autoupdater;
        pub mod sys_main;
        pub mod sys_unix;
    } // mod sys
    pub mod vm {
        pub mod vm;
        pub mod vm_interpreted;
        pub mod vm_x86;
    } // mod vm
} // mod src
