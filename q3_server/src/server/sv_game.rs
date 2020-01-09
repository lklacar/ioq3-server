use ::libc;

pub mod qcommon_h {

    #[inline]

    pub unsafe extern "C" fn _vmf(mut x: crate::stdlib::intptr_t) -> libc::c_float {
        let mut fi: crate::src::qcommon::q_shared::floatint_t =
            crate::src::qcommon::q_shared::floatint_t { f: 0. };
        fi.i = x as libc::c_int;
        return fi.f;
    }

    use crate::src::qcommon::q_shared::floatint_t;
    // _QCOMMON_H_
    // flags for sv_allowDownload and cl_allowDownload
}

pub use crate::stdlib::intptr_t;

pub use crate::be_aas_h::aas_altroutegoal_s;
pub use crate::be_aas_h::aas_areainfo_s;
pub use crate::be_aas_h::aas_clientmove_s;
pub use crate::be_aas_h::aas_entityinfo_s;
pub use crate::be_aas_h::aas_predictroute_s;
pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::botlib_h::aas_export_s;
pub use crate::botlib_h::aas_export_t;
pub use crate::botlib_h::ai_export_s;
pub use crate::botlib_h::ai_export_t;
pub use crate::botlib_h::bot_entitystate_s;
pub use crate::botlib_h::bot_entitystate_t;
pub use crate::botlib_h::bot_input_s;
pub use crate::botlib_h::bot_input_t;
pub use crate::botlib_h::botlib_export_s;
pub use crate::botlib_h::botlib_export_t;
pub use crate::botlib_h::ea_export_s;
pub use crate::botlib_h::ea_export_t;
pub use crate::g_public_h::entityShared_t;
pub use crate::g_public_h::sharedEntity_t;
pub use crate::g_public_h::BOTAI_START_FRAME;
pub use crate::g_public_h::BOTLIB_AAS_ALTERNATIVE_ROUTE_GOAL;
pub use crate::g_public_h::BOTLIB_AAS_AREA_INFO;
pub use crate::g_public_h::BOTLIB_AAS_AREA_REACHABILITY;
pub use crate::g_public_h::BOTLIB_AAS_AREA_TRAVEL_TIME_TO_GOAL_AREA;
pub use crate::g_public_h::BOTLIB_AAS_BBOX_AREAS;
pub use crate::g_public_h::BOTLIB_AAS_ENABLE_ROUTING_AREA;
pub use crate::g_public_h::BOTLIB_AAS_ENTITY_INFO;
pub use crate::g_public_h::BOTLIB_AAS_FLOAT_FOR_BSP_EPAIR_KEY;
pub use crate::g_public_h::BOTLIB_AAS_INITIALIZED;
pub use crate::g_public_h::BOTLIB_AAS_INT_FOR_BSP_EPAIR_KEY;
pub use crate::g_public_h::BOTLIB_AAS_NEXT_BSP_ENTITY;
pub use crate::g_public_h::BOTLIB_AAS_POINT_AREA_NUM;
pub use crate::g_public_h::BOTLIB_AAS_POINT_CONTENTS;
pub use crate::g_public_h::BOTLIB_AAS_POINT_REACHABILITY_AREA_INDEX;
pub use crate::g_public_h::BOTLIB_AAS_PREDICT_CLIENT_MOVEMENT;
pub use crate::g_public_h::BOTLIB_AAS_PREDICT_ROUTE;
pub use crate::g_public_h::BOTLIB_AAS_PRESENCE_TYPE_BOUNDING_BOX;
pub use crate::g_public_h::BOTLIB_AAS_SWIMMING;
pub use crate::g_public_h::BOTLIB_AAS_TIME;
pub use crate::g_public_h::BOTLIB_AAS_TRACE_AREAS;
pub use crate::g_public_h::BOTLIB_AAS_VALUE_FOR_BSP_EPAIR_KEY;
pub use crate::g_public_h::BOTLIB_AAS_VECTOR_FOR_BSP_EPAIR_KEY;
pub use crate::g_public_h::BOTLIB_AI_ADD_AVOID_SPOT;
pub use crate::g_public_h::BOTLIB_AI_ALLOC_CHAT_STATE;
pub use crate::g_public_h::BOTLIB_AI_ALLOC_GOAL_STATE;
pub use crate::g_public_h::BOTLIB_AI_ALLOC_MOVE_STATE;
pub use crate::g_public_h::BOTLIB_AI_ALLOC_WEAPON_STATE;
pub use crate::g_public_h::BOTLIB_AI_AVOID_GOAL_TIME;
pub use crate::g_public_h::BOTLIB_AI_CHARACTERISTIC_BFLOAT;
pub use crate::g_public_h::BOTLIB_AI_CHARACTERISTIC_BINTEGER;
pub use crate::g_public_h::BOTLIB_AI_CHARACTERISTIC_FLOAT;
pub use crate::g_public_h::BOTLIB_AI_CHARACTERISTIC_INTEGER;
pub use crate::g_public_h::BOTLIB_AI_CHARACTERISTIC_STRING;
pub use crate::g_public_h::BOTLIB_AI_CHAT_LENGTH;
pub use crate::g_public_h::BOTLIB_AI_CHOOSE_BEST_FIGHT_WEAPON;
pub use crate::g_public_h::BOTLIB_AI_CHOOSE_LTG_ITEM;
pub use crate::g_public_h::BOTLIB_AI_CHOOSE_NBG_ITEM;
pub use crate::g_public_h::BOTLIB_AI_DUMP_AVOID_GOALS;
pub use crate::g_public_h::BOTLIB_AI_DUMP_GOAL_STACK;
pub use crate::g_public_h::BOTLIB_AI_EMPTY_GOAL_STACK;
pub use crate::g_public_h::BOTLIB_AI_ENTER_CHAT;
pub use crate::g_public_h::BOTLIB_AI_FIND_MATCH;
pub use crate::g_public_h::BOTLIB_AI_FREE_CHARACTER;
pub use crate::g_public_h::BOTLIB_AI_FREE_CHAT_STATE;
pub use crate::g_public_h::BOTLIB_AI_FREE_GOAL_STATE;
pub use crate::g_public_h::BOTLIB_AI_FREE_ITEM_WEIGHTS;
pub use crate::g_public_h::BOTLIB_AI_FREE_MOVE_STATE;
pub use crate::g_public_h::BOTLIB_AI_FREE_WEAPON_STATE;
pub use crate::g_public_h::BOTLIB_AI_GENETIC_PARENTS_AND_CHILD_SELECTION;
pub use crate::g_public_h::BOTLIB_AI_GET_CHAT_MESSAGE;
pub use crate::g_public_h::BOTLIB_AI_GET_LEVEL_ITEM_GOAL;
pub use crate::g_public_h::BOTLIB_AI_GET_MAP_LOCATION_GOAL;
pub use crate::g_public_h::BOTLIB_AI_GET_NEXT_CAMP_SPOT_GOAL;
pub use crate::g_public_h::BOTLIB_AI_GET_SECOND_GOAL;
pub use crate::g_public_h::BOTLIB_AI_GET_TOP_GOAL;
pub use crate::g_public_h::BOTLIB_AI_GET_WEAPON_INFO;
pub use crate::g_public_h::BOTLIB_AI_GOAL_NAME;
pub use crate::g_public_h::BOTLIB_AI_INITIAL_CHAT;
pub use crate::g_public_h::BOTLIB_AI_INIT_LEVEL_ITEMS;
pub use crate::g_public_h::BOTLIB_AI_INIT_MOVE_STATE;
pub use crate::g_public_h::BOTLIB_AI_INTERBREED_GOAL_FUZZY_LOGIC;
pub use crate::g_public_h::BOTLIB_AI_ITEM_GOAL_IN_VIS_BUT_NOT_VISIBLE;
pub use crate::g_public_h::BOTLIB_AI_LOAD_CHARACTER;
pub use crate::g_public_h::BOTLIB_AI_LOAD_CHAT_FILE;
pub use crate::g_public_h::BOTLIB_AI_LOAD_ITEM_WEIGHTS;
pub use crate::g_public_h::BOTLIB_AI_LOAD_WEAPON_WEIGHTS;
pub use crate::g_public_h::BOTLIB_AI_MATCH_VARIABLE;
pub use crate::g_public_h::BOTLIB_AI_MOVEMENT_VIEW_TARGET;
pub use crate::g_public_h::BOTLIB_AI_MOVE_IN_DIRECTION;
pub use crate::g_public_h::BOTLIB_AI_MOVE_TO_GOAL;
pub use crate::g_public_h::BOTLIB_AI_MUTATE_GOAL_FUZZY_LOGIC;
pub use crate::g_public_h::BOTLIB_AI_NEXT_CONSOLE_MESSAGE;
pub use crate::g_public_h::BOTLIB_AI_NUM_CONSOLE_MESSAGE;
pub use crate::g_public_h::BOTLIB_AI_NUM_INITIAL_CHATS;
pub use crate::g_public_h::BOTLIB_AI_POP_GOAL;
pub use crate::g_public_h::BOTLIB_AI_PREDICT_VISIBLE_POSITION;
pub use crate::g_public_h::BOTLIB_AI_PUSH_GOAL;
pub use crate::g_public_h::BOTLIB_AI_QUEUE_CONSOLE_MESSAGE;
pub use crate::g_public_h::BOTLIB_AI_REACHABILITY_AREA;
pub use crate::g_public_h::BOTLIB_AI_REMOVE_CONSOLE_MESSAGE;
pub use crate::g_public_h::BOTLIB_AI_REMOVE_FROM_AVOID_GOALS;
pub use crate::g_public_h::BOTLIB_AI_REPLACE_SYNONYMS;
pub use crate::g_public_h::BOTLIB_AI_REPLY_CHAT;
pub use crate::g_public_h::BOTLIB_AI_RESET_AVOID_GOALS;
pub use crate::g_public_h::BOTLIB_AI_RESET_AVOID_REACH;
pub use crate::g_public_h::BOTLIB_AI_RESET_GOAL_STATE;
pub use crate::g_public_h::BOTLIB_AI_RESET_LAST_AVOID_REACH;
pub use crate::g_public_h::BOTLIB_AI_RESET_MOVE_STATE;
pub use crate::g_public_h::BOTLIB_AI_RESET_WEAPON_STATE;
pub use crate::g_public_h::BOTLIB_AI_SAVE_GOAL_FUZZY_LOGIC;
pub use crate::g_public_h::BOTLIB_AI_SET_AVOID_GOAL_TIME;
pub use crate::g_public_h::BOTLIB_AI_SET_CHAT_GENDER;
pub use crate::g_public_h::BOTLIB_AI_SET_CHAT_NAME;
pub use crate::g_public_h::BOTLIB_AI_STRING_CONTAINS;
pub use crate::g_public_h::BOTLIB_AI_TOUCHING_GOAL;
pub use crate::g_public_h::BOTLIB_AI_UNIFY_WHITE_SPACES;
pub use crate::g_public_h::BOTLIB_AI_UPDATE_ENTITY_ITEMS;
pub use crate::g_public_h::BOTLIB_EA_ACTION;
pub use crate::g_public_h::BOTLIB_EA_ATTACK;
pub use crate::g_public_h::BOTLIB_EA_COMMAND;
pub use crate::g_public_h::BOTLIB_EA_CROUCH;
pub use crate::g_public_h::BOTLIB_EA_DELAYED_JUMP;
pub use crate::g_public_h::BOTLIB_EA_END_REGULAR;
pub use crate::g_public_h::BOTLIB_EA_GESTURE;
pub use crate::g_public_h::BOTLIB_EA_GET_INPUT;
pub use crate::g_public_h::BOTLIB_EA_JUMP;
pub use crate::g_public_h::BOTLIB_EA_MOVE;
pub use crate::g_public_h::BOTLIB_EA_MOVE_BACK;
pub use crate::g_public_h::BOTLIB_EA_MOVE_DOWN;
pub use crate::g_public_h::BOTLIB_EA_MOVE_FORWARD;
pub use crate::g_public_h::BOTLIB_EA_MOVE_LEFT;
pub use crate::g_public_h::BOTLIB_EA_MOVE_RIGHT;
pub use crate::g_public_h::BOTLIB_EA_MOVE_UP;
pub use crate::g_public_h::BOTLIB_EA_RESET_INPUT;
pub use crate::g_public_h::BOTLIB_EA_RESPAWN;
pub use crate::g_public_h::BOTLIB_EA_SAY;
pub use crate::g_public_h::BOTLIB_EA_SAY_TEAM;
pub use crate::g_public_h::BOTLIB_EA_SELECT_WEAPON;
pub use crate::g_public_h::BOTLIB_EA_TALK;
pub use crate::g_public_h::BOTLIB_EA_USE;
pub use crate::g_public_h::BOTLIB_EA_VIEW;
pub use crate::g_public_h::BOTLIB_GET_CONSOLE_MESSAGE;
pub use crate::g_public_h::BOTLIB_GET_SNAPSHOT_ENTITY;
pub use crate::g_public_h::BOTLIB_LIBVAR_GET;
pub use crate::g_public_h::BOTLIB_LIBVAR_SET;
pub use crate::g_public_h::BOTLIB_LOAD_MAP;
pub use crate::g_public_h::BOTLIB_PC_ADD_GLOBAL_DEFINE;
pub use crate::g_public_h::BOTLIB_PC_FREE_SOURCE;
pub use crate::g_public_h::BOTLIB_PC_LOAD_SOURCE;
pub use crate::g_public_h::BOTLIB_PC_READ_TOKEN;
pub use crate::g_public_h::BOTLIB_PC_SOURCE_FILE_AND_LINE;
pub use crate::g_public_h::BOTLIB_SETUP;
pub use crate::g_public_h::BOTLIB_SHUTDOWN;
pub use crate::g_public_h::BOTLIB_START_FRAME;
pub use crate::g_public_h::BOTLIB_TEST;
pub use crate::g_public_h::BOTLIB_UPDATENTITY;
pub use crate::g_public_h::BOTLIB_USER_COMMAND;
pub use crate::g_public_h::GAME_CLIENT_BEGIN;
pub use crate::g_public_h::GAME_CLIENT_COMMAND;
pub use crate::g_public_h::GAME_CLIENT_CONNECT;
pub use crate::g_public_h::GAME_CLIENT_DISCONNECT;
pub use crate::g_public_h::GAME_CLIENT_THINK;
pub use crate::g_public_h::GAME_CLIENT_USERINFO_CHANGED;
pub use crate::g_public_h::GAME_CONSOLE_COMMAND;
pub use crate::g_public_h::GAME_INIT;
pub use crate::g_public_h::GAME_RUN_FRAME;
pub use crate::g_public_h::GAME_SHUTDOWN;
pub use crate::g_public_h::G_ADJUST_AREA_PORTAL_STATE;
pub use crate::g_public_h::G_AREAS_CONNECTED;
pub use crate::g_public_h::G_ARGC;
pub use crate::g_public_h::G_ARGV;
pub use crate::g_public_h::G_BOT_ALLOCATE_CLIENT;
pub use crate::g_public_h::G_BOT_FREE_CLIENT;
pub use crate::g_public_h::G_CVAR_REGISTER;
pub use crate::g_public_h::G_CVAR_SET;
pub use crate::g_public_h::G_CVAR_UPDATE;
pub use crate::g_public_h::G_CVAR_VARIABLE_INTEGER_VALUE;
pub use crate::g_public_h::G_CVAR_VARIABLE_STRING_BUFFER;
pub use crate::g_public_h::G_DEBUG_POLYGON_CREATE;
pub use crate::g_public_h::G_DEBUG_POLYGON_DELETE;
pub use crate::g_public_h::G_DROP_CLIENT;
pub use crate::g_public_h::G_ENTITIES_IN_BOX;
pub use crate::g_public_h::G_ENTITY_CONTACT;
pub use crate::g_public_h::G_ENTITY_CONTACTCAPSULE;
pub use crate::g_public_h::G_ERROR;
pub use crate::g_public_h::G_FS_FCLOSE_FILE;
pub use crate::g_public_h::G_FS_FOPEN_FILE;
pub use crate::g_public_h::G_FS_GETFILELIST;
pub use crate::g_public_h::G_FS_READ;
pub use crate::g_public_h::G_FS_SEEK;
pub use crate::g_public_h::G_FS_WRITE;
pub use crate::g_public_h::G_GET_CONFIGSTRING;
pub use crate::g_public_h::G_GET_ENTITY_TOKEN;
pub use crate::g_public_h::G_GET_SERVERINFO;
pub use crate::g_public_h::G_GET_USERCMD;
pub use crate::g_public_h::G_GET_USERINFO;
pub use crate::g_public_h::G_IN_PVS;
pub use crate::g_public_h::G_IN_PVS_IGNORE_PORTALS;
pub use crate::g_public_h::G_LINKENTITY;
pub use crate::g_public_h::G_LOCATE_GAME_DATA;
pub use crate::g_public_h::G_MILLISECONDS;
pub use crate::g_public_h::G_POINT_CONTENTS;
pub use crate::g_public_h::G_PRINT;
pub use crate::g_public_h::G_REAL_TIME;
pub use crate::g_public_h::G_SEND_CONSOLE_COMMAND;
pub use crate::g_public_h::G_SEND_SERVER_COMMAND;
pub use crate::g_public_h::G_SET_BRUSH_MODEL;
pub use crate::g_public_h::G_SET_CONFIGSTRING;
pub use crate::g_public_h::G_SET_USERINFO;
pub use crate::g_public_h::G_SNAPVECTOR;
pub use crate::g_public_h::G_TRACE;
pub use crate::g_public_h::G_TRACECAPSULE;
pub use crate::g_public_h::G_UNLINKENTITY;
pub use crate::qcommon_h::msg_t;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::netchan_t;
pub use crate::qcommon_h::netsrc_t;
pub use crate::qcommon_h::vmInterpret_t;
pub use crate::qcommon_h::vm_t;
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
pub use crate::qcommon_h::TRAP_ANGLEVECTORS;
pub use crate::qcommon_h::TRAP_ATAN2;
pub use crate::qcommon_h::TRAP_CEIL;
pub use crate::qcommon_h::TRAP_COS;
pub use crate::qcommon_h::TRAP_FLOOR;
pub use crate::qcommon_h::TRAP_MATRIXMULTIPLY;
pub use crate::qcommon_h::TRAP_MEMCPY;
pub use crate::qcommon_h::TRAP_MEMSET;
pub use crate::qcommon_h::TRAP_PERPENDICULARVECTOR;
pub use crate::qcommon_h::TRAP_SIN;
pub use crate::qcommon_h::TRAP_SQRT;
pub use crate::qcommon_h::TRAP_STRNCPY;
pub use crate::qcommon_h::TRAP_TESTPRINTFLOAT;
pub use crate::qcommon_h::TRAP_TESTPRINTINT;
pub use crate::qcommon_h::VMI_BYTECODE;
pub use crate::qcommon_h::VMI_COMPILED;
pub use crate::qcommon_h::VMI_NATIVE;
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
pub use crate::src::asm::snapvector::qsnapvectorsse;
pub use crate::src::botlib::be_ai_chat::bot_consolemessage_s;
pub use crate::src::botlib::be_ai_chat::bot_match_s;
pub use crate::src::botlib::be_ai_goal::bot_goal_s;
pub use crate::src::botlib::be_ai_move::bot_initmove_s;
pub use crate::src::botlib::be_ai_move::bot_moveresult_s;
pub use crate::src::botlib::be_ai_weap::weaponinfo_s;
pub use crate::src::qcommon::cmd::Cbuf_ExecuteText;
pub use crate::src::qcommon::cmd::Cmd_Argc;
pub use crate::src::qcommon::cmd::Cmd_ArgvBuffer;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Milliseconds;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::Com_RealTime;
pub use crate::src::qcommon::cvar::Cvar_Get;
pub use crate::src::qcommon::cvar::Cvar_InfoString;
pub use crate::src::qcommon::cvar::Cvar_Register;
pub use crate::src::qcommon::cvar::Cvar_SetSafe;
pub use crate::src::qcommon::cvar::Cvar_Update;
pub use crate::src::qcommon::cvar::Cvar_VariableIntegerValue;
pub use crate::src::qcommon::cvar::Cvar_VariableStringBuffer;
pub use crate::src::qcommon::cvar::Cvar_VariableValue;
pub use crate::src::qcommon::files::FS_FCloseFile;
pub use crate::src::qcommon::files::FS_FOpenFileByMode;
pub use crate::src::qcommon::files::FS_GetFileList;
pub use crate::src::qcommon::files::FS_Read;
pub use crate::src::qcommon::files::FS_Seek;
pub use crate::src::qcommon::files::FS_Write;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_math::MatrixMultiply;
pub use crate::src::qcommon::q_math::PerpendicularVector;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::clipHandle_t;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::floatint_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::pc_token_s;
pub use crate::src::qcommon::q_shared::pc_token_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtime_s;
pub use crate::src::qcommon::q_shared::qtime_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trace_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::COM_Parse;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
pub use crate::src::server::sv_bot::BotImport_DebugPolygonCreate;
pub use crate::src::server::sv_bot::BotImport_DebugPolygonDelete;
pub use crate::src::server::sv_bot::SV_BotAllocateClient;
pub use crate::src::server::sv_bot::SV_BotFreeClient;
pub use crate::src::server::sv_bot::SV_BotGetConsoleMessage;
pub use crate::src::server::sv_bot::SV_BotGetSnapshotEntity;
pub use crate::src::server::sv_bot::SV_BotLibSetup;
pub use crate::src::server::sv_bot::SV_BotLibShutdown;
pub use crate::src::server::sv_client::SV_ClientThink;
pub use crate::src::server::sv_client::SV_DropClient;
pub use crate::src::server::sv_game::qcommon_h::_vmf;
pub use crate::src::server::sv_init::SV_GetConfigstring;
pub use crate::src::server::sv_init::SV_GetUserinfo;
pub use crate::src::server::sv_init::SV_SetConfigstring;
pub use crate::src::server::sv_init::SV_SetUserinfo;
pub use crate::src::server::sv_main::gvm;
pub use crate::src::server::sv_main::sv;
pub use crate::src::server::sv_main::sv_maxclients;
pub use crate::src::server::sv_main::svs;
pub use crate::src::server::sv_main::SV_SendServerCommand;
pub use crate::src::server::sv_world::SV_AreaEntities;
pub use crate::src::server::sv_world::SV_ClipHandleForEntity;
pub use crate::src::server::sv_world::SV_LinkEntity;
pub use crate::src::server::sv_world::SV_PointContents;
pub use crate::src::server::sv_world::SV_Trace;
pub use crate::src::server::sv_world::SV_UnlinkEntity;
pub use crate::src::sys::sys_unix::Sys_Milliseconds;
pub use crate::src::vm::vm::VM_ArgPtr;
pub use crate::src::vm::vm::VM_Call;
pub use crate::src::vm::vm::VM_Create;
pub use crate::src::vm::vm::VM_Free;
pub use crate::src::vm::vm::VM_Restart;
use crate::stdlib::atoi;
pub use crate::vm_local_h::vm_s;

use crate::src::qcommon::cm_load::CM_EntityString;
use crate::src::qcommon::cm_load::CM_InlineModel;
use crate::src::qcommon::cm_load::CM_LeafArea;
use crate::src::qcommon::cm_load::CM_LeafCluster;
use crate::src::qcommon::cm_load::CM_ModelBounds;
use crate::src::qcommon::cm_test::CM_AdjustAreaPortalState;
use crate::src::qcommon::cm_test::CM_AreasConnected;
use crate::src::qcommon::cm_test::CM_ClusterPVS;
use crate::src::qcommon::cm_test::CM_PointLeafnum;
use crate::src::qcommon::cm_trace::CM_TransformedBoxTrace;
use crate::stdlib::atan2;
use crate::stdlib::ceil;
use crate::stdlib::cos;
use crate::stdlib::floor;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
use crate::stdlib::sin;
use crate::stdlib::sqrt;
use crate::stdlib::strncpy;
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
// sv_game.c -- interface to the game dll
#[no_mangle]

pub static mut botlib_export: *mut crate::botlib_h::botlib_export_t =
    0 as *const crate::botlib_h::botlib_export_t as *mut crate::botlib_h::botlib_export_t;
// these functions must be used instead of pointer arithmetic, because
// the game allocates gentities with private information after the server shared part
#[no_mangle]

pub unsafe extern "C" fn SV_NumForGentity(
    mut ent: *mut crate::g_public_h::sharedEntity_t,
) -> libc::c_int {
    let mut num: libc::c_int = 0;
    num = ((ent as *mut crate::src::qcommon::q_shared::byte).wrapping_offset_from(
        crate::src::server::sv_main::sv.gentities as *mut crate::src::qcommon::q_shared::byte,
    ) as libc::c_long
        / crate::src::server::sv_main::sv.gentitySize as libc::c_long) as libc::c_int;
    return num;
}
#[no_mangle]

pub unsafe extern "C" fn SV_GentityNum(
    mut num: libc::c_int,
) -> *mut crate::g_public_h::sharedEntity_t {
    let mut ent: *mut crate::g_public_h::sharedEntity_t =
        0 as *mut crate::g_public_h::sharedEntity_t;
    ent = (crate::src::server::sv_main::sv.gentities as *mut crate::src::qcommon::q_shared::byte)
        .offset((crate::src::server::sv_main::sv.gentitySize * num) as isize)
        as *mut crate::g_public_h::sharedEntity_t;
    return ent;
}
#[no_mangle]

pub unsafe extern "C" fn SV_GameClientNum(
    mut num: libc::c_int,
) -> *mut crate::src::qcommon::q_shared::playerState_t {
    let mut ps: *mut crate::src::qcommon::q_shared::playerState_t =
        0 as *mut crate::src::qcommon::q_shared::playerState_t;
    ps = (crate::src::server::sv_main::sv.gameClients as *mut crate::src::qcommon::q_shared::byte)
        .offset((crate::src::server::sv_main::sv.gameClientSize * num) as isize)
        as *mut crate::src::qcommon::q_shared::playerState_t;
    return ps;
}
#[no_mangle]

pub unsafe extern "C" fn SV_SvEntityForGentity(
    mut gEnt: *mut crate::g_public_h::sharedEntity_t,
) -> *mut crate::server_h::svEntity_t {
    if gEnt.is_null()
        || (*gEnt).s.number < 0 as libc::c_int
        || (*gEnt).s.number >= (1 as libc::c_int) << 10 as libc::c_int
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"SV_SvEntityForGentity: bad gEnt\x00" as *const u8 as *const libc::c_char,
        );
    }
    return &mut *crate::src::server::sv_main::sv
        .svEntities
        .as_mut_ptr()
        .offset((*gEnt).s.number as isize) as *mut crate::server_h::svEntity_t;
}
#[no_mangle]

pub unsafe extern "C" fn SV_GEntityForSvEntity(
    mut svEnt: *mut crate::server_h::svEntity_t,
) -> *mut crate::g_public_h::sharedEntity_t {
    let mut num: libc::c_int = 0;
    num = svEnt.wrapping_offset_from(crate::src::server::sv_main::sv.svEntities.as_mut_ptr())
        as libc::c_long as libc::c_int;
    return SV_GentityNum(num);
}
/*
===============
SV_GameSendServerCommand

Sends a command string to a client
===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_GameSendServerCommand(
    mut clientNum: libc::c_int,
    mut text: *const libc::c_char,
) {
    if clientNum == -(1 as libc::c_int) {
        crate::src::server::sv_main::SV_SendServerCommand(
            0 as *mut crate::server_h::client_t,
            b"%s\x00" as *const u8 as *const libc::c_char,
            text,
        );
    } else {
        if clientNum < 0 as libc::c_int
            || clientNum >= (*crate::src::server::sv_main::sv_maxclients).integer
        {
            return;
        }
        crate::src::server::sv_main::SV_SendServerCommand(
            crate::src::server::sv_main::svs
                .clients
                .offset(clientNum as isize),
            b"%s\x00" as *const u8 as *const libc::c_char,
            text,
        );
    };
}
/*
===============
SV_GameDropClient

Disconnects the client with a message
===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_GameDropClient(
    mut clientNum: libc::c_int,
    mut reason: *const libc::c_char,
) {
    if clientNum < 0 as libc::c_int
        || clientNum >= (*crate::src::server::sv_main::sv_maxclients).integer
    {
        return;
    }
    crate::src::server::sv_client::SV_DropClient(
        crate::src::server::sv_main::svs
            .clients
            .offset(clientNum as isize),
        reason,
    );
}
/*
=================
SV_SetBrushModel

sets mins and maxs for inline bmodels
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_SetBrushModel(
    mut ent: *mut crate::g_public_h::sharedEntity_t,
    mut name: *const libc::c_char,
) {
    let mut h: crate::src::qcommon::q_shared::clipHandle_t = 0; // we don't know exactly what is in the brushes
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if name.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"SV_SetBrushModel: NULL\x00" as *const u8 as *const libc::c_char,
        );
    }
    if *name.offset(0 as libc::c_int as isize) as libc::c_int != '*' as i32 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"SV_SetBrushModel: %s isn\'t a brush model\x00" as *const u8 as *const libc::c_char,
            name,
        );
    }
    (*ent).s.modelindex = crate::stdlib::atoi(name.offset(1 as libc::c_int as isize));
    h = crate::src::qcommon::cm_load::CM_InlineModel((*ent).s.modelindex);
    crate::src::qcommon::cm_load::CM_ModelBounds(h, mins.as_mut_ptr(), maxs.as_mut_ptr());
    (*ent).r.mins[0 as libc::c_int as usize] = mins[0 as libc::c_int as usize];
    (*ent).r.mins[1 as libc::c_int as usize] = mins[1 as libc::c_int as usize];
    (*ent).r.mins[2 as libc::c_int as usize] = mins[2 as libc::c_int as usize];
    (*ent).r.maxs[0 as libc::c_int as usize] = maxs[0 as libc::c_int as usize];
    (*ent).r.maxs[1 as libc::c_int as usize] = maxs[1 as libc::c_int as usize];
    (*ent).r.maxs[2 as libc::c_int as usize] = maxs[2 as libc::c_int as usize];
    (*ent).r.bmodel = crate::src::qcommon::q_shared::qtrue;
    (*ent).r.contents = -(1 as libc::c_int);
    crate::src::server::sv_world::SV_LinkEntity(ent);
    // FIXME: remove
}
/*
=================
SV_inPVS

Also checks portalareas so that doors block sight
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_inPVS(
    mut p1: *const crate::src::qcommon::q_shared::vec_t,
    mut p2: *const crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut leafnum: libc::c_int = 0; // a door blocks sight
    let mut cluster: libc::c_int = 0;
    let mut area1: libc::c_int = 0;
    let mut area2: libc::c_int = 0;
    let mut mask: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    leafnum = crate::src::qcommon::cm_test::CM_PointLeafnum(p1);
    cluster = crate::src::qcommon::cm_load::CM_LeafCluster(leafnum);
    area1 = crate::src::qcommon::cm_load::CM_LeafArea(leafnum);
    mask = crate::src::qcommon::cm_test::CM_ClusterPVS(cluster);
    leafnum = crate::src::qcommon::cm_test::CM_PointLeafnum(p2);
    cluster = crate::src::qcommon::cm_load::CM_LeafCluster(leafnum);
    area2 = crate::src::qcommon::cm_load::CM_LeafArea(leafnum);
    if !mask.is_null()
        && *mask.offset((cluster >> 3 as libc::c_int) as isize) as libc::c_int
            & (1 as libc::c_int) << (cluster & 7 as libc::c_int)
            == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if crate::src::qcommon::cm_test::CM_AreasConnected(area1, area2) as u64 == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
=================
SV_inPVSIgnorePortals

Does NOT check portalareas
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_inPVSIgnorePortals(
    mut p1: *const crate::src::qcommon::q_shared::vec_t,
    mut p2: *const crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut leafnum: libc::c_int = 0;
    let mut cluster: libc::c_int = 0;
    let mut mask: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    leafnum = crate::src::qcommon::cm_test::CM_PointLeafnum(p1);
    cluster = crate::src::qcommon::cm_load::CM_LeafCluster(leafnum);
    mask = crate::src::qcommon::cm_test::CM_ClusterPVS(cluster);
    leafnum = crate::src::qcommon::cm_test::CM_PointLeafnum(p2);
    cluster = crate::src::qcommon::cm_load::CM_LeafCluster(leafnum);
    if !mask.is_null()
        && *mask.offset((cluster >> 3 as libc::c_int) as isize) as libc::c_int
            & (1 as libc::c_int) << (cluster & 7 as libc::c_int)
            == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
========================
SV_AdjustAreaPortalState
========================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_AdjustAreaPortalState(
    mut ent: *mut crate::g_public_h::sharedEntity_t,
    mut open: crate::src::qcommon::q_shared::qboolean,
) {
    let mut svEnt: *mut crate::server_h::svEntity_t = 0 as *mut crate::server_h::svEntity_t;
    svEnt = SV_SvEntityForGentity(ent);
    if (*svEnt).areanum2 == -(1 as libc::c_int) {
        return;
    }
    crate::src::qcommon::cm_test::CM_AdjustAreaPortalState(
        (*svEnt).areanum,
        (*svEnt).areanum2,
        open,
    );
}
/*
==================
SV_EntityContact
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_EntityContact(
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut gEnt: *const crate::g_public_h::sharedEntity_t,
    mut capsule: libc::c_int,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut origin: *const libc::c_float = 0 as *const libc::c_float;
    let mut angles: *const libc::c_float = 0 as *const libc::c_float;
    let mut ch: crate::src::qcommon::q_shared::clipHandle_t = 0;
    let mut trace: crate::src::qcommon::q_shared::trace_t =
        crate::src::qcommon::q_shared::trace_t {
            allsolid: crate::src::qcommon::q_shared::qfalse,
            startsolid: crate::src::qcommon::q_shared::qfalse,
            fraction: 0.,
            endpos: [0.; 3],
            plane: crate::src::qcommon::q_shared::cplane_t {
                normal: [0.; 3],
                dist: 0.,
                type_0: 0,
                signbits: 0,
                pad: [0; 2],
            },
            surfaceFlags: 0,
            contents: 0,
            entityNum: 0,
        };
    // check for exact collision
    origin = (*gEnt).r.currentOrigin.as_ptr();
    angles = (*gEnt).r.currentAngles.as_ptr();
    ch = crate::src::server::sv_world::SV_ClipHandleForEntity(gEnt);
    crate::src::qcommon::cm_trace::CM_TransformedBoxTrace(
        &mut trace,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        mins,
        maxs,
        ch,
        -(1 as libc::c_int),
        origin,
        angles,
        capsule,
    );
    return trace.startsolid;
}
/*
===============
SV_GetServerinfo

===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_GetServerinfo(
    mut buffer: *mut libc::c_char,
    mut bufferSize: libc::c_int,
) {
    if bufferSize < 1 as libc::c_int {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"SV_GetServerinfo: bufferSize == %i\x00" as *const u8 as *const libc::c_char,
            bufferSize,
        );
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        buffer,
        crate::src::qcommon::cvar::Cvar_InfoString(0x4 as libc::c_int),
        bufferSize,
    );
}
/*
===============
SV_LocateGameData

===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_LocateGameData(
    mut gEnts: *mut crate::g_public_h::sharedEntity_t,
    mut numGEntities: libc::c_int,
    mut sizeofGEntity_t: libc::c_int,
    mut clients: *mut crate::src::qcommon::q_shared::playerState_t,
    mut sizeofGameClient: libc::c_int,
) {
    crate::src::server::sv_main::sv.gentities = gEnts;
    crate::src::server::sv_main::sv.gentitySize = sizeofGEntity_t;
    crate::src::server::sv_main::sv.num_entities = numGEntities;
    crate::src::server::sv_main::sv.gameClients = clients;
    crate::src::server::sv_main::sv.gameClientSize = sizeofGameClient;
}
/*
===============
SV_GetUsercmd

===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_GetUsercmd(
    mut clientNum: libc::c_int,
    mut cmd: *mut crate::src::qcommon::q_shared::usercmd_t,
) {
    if clientNum < 0 as libc::c_int
        || clientNum >= (*crate::src::server::sv_main::sv_maxclients).integer
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
            b"SV_GetUsercmd: bad clientNum:%i\x00" as *const u8 as *const libc::c_char,
            clientNum,
        );
    }
    *cmd = (*crate::src::server::sv_main::svs
        .clients
        .offset(clientNum as isize))
    .lastUsercmd;
}
//==============================================

unsafe extern "C" fn FloatAsInt(mut f: libc::c_float) -> libc::c_int {
    let mut fi: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    fi.f = f;
    return fi.i;
}
/*
====================
SV_GameSystemCalls

The module is making a system call
====================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_GameSystemCalls(
    mut args: *mut crate::stdlib::intptr_t,
) -> crate::stdlib::intptr_t {
    match *args.offset(0 as libc::c_int as isize) {
        0 => {
            crate::src::qcommon::common::Com_Printf(
                b"%s\x00" as *const u8 as *const libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *const libc::c_char,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        1 => {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                b"%s\x00" as *const u8 as *const libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *const libc::c_char,
            );
        }
        2 => return crate::src::sys::sys_unix::Sys_Milliseconds() as crate::stdlib::intptr_t,
        3 => {
            crate::src::qcommon::cvar::Cvar_Register(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *const libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *const libc::c_char,
                *args.offset(4 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        4 => {
            crate::src::qcommon::cvar::Cvar_Update(crate::src::vm::vm::VM_ArgPtr(
                *args.offset(1 as libc::c_int as isize),
            )
                as *mut crate::src::qcommon::q_shared::vmCvar_t);
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        5 => {
            crate::src::qcommon::cvar::Cvar_SetSafe(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *const libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *const libc::c_char,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        6 => {
            return crate::src::qcommon::cvar::Cvar_VariableIntegerValue(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *const libc::c_char,
            ) as crate::stdlib::intptr_t
        }
        7 => {
            crate::src::qcommon::cvar::Cvar_VariableStringBuffer(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *const libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
                *args.offset(3 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        8 => return crate::src::qcommon::cmd::Cmd_Argc() as crate::stdlib::intptr_t,
        9 => {
            crate::src::qcommon::cmd::Cmd_ArgvBuffer(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
                *args.offset(3 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        14 => {
            crate::src::qcommon::cmd::Cbuf_ExecuteText(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *const libc::c_char,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        10 => {
            return crate::src::qcommon::files::FS_FOpenFileByMode(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *const libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::fileHandle_t,
                *args.offset(3 as libc::c_int as isize) as crate::src::qcommon::q_shared::fsMode_t,
            ) as crate::stdlib::intptr_t
        }
        11 => {
            crate::src::qcommon::files::FS_Read(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize)),
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
                *args.offset(3 as libc::c_int as isize)
                    as crate::src::qcommon::q_shared::fileHandle_t,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        12 => {
            crate::src::qcommon::files::FS_Write(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize)),
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
                *args.offset(3 as libc::c_int as isize)
                    as crate::src::qcommon::q_shared::fileHandle_t,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        13 => {
            crate::src::qcommon::files::FS_FCloseFile(*args.offset(1 as libc::c_int as isize)
                as crate::src::qcommon::q_shared::fileHandle_t);
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        38 => {
            return crate::src::qcommon::files::FS_GetFileList(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *const libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *const libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut libc::c_char,
                *args.offset(4 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        45 => {
            return crate::src::qcommon::files::FS_Seek(
                *args.offset(1 as libc::c_int as isize)
                    as crate::src::qcommon::q_shared::fileHandle_t,
                *args.offset(2 as libc::c_int as isize),
                *args.offset(3 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        15 => {
            SV_LocateGameData(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::g_public_h::sharedEntity_t,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
                *args.offset(3 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(4 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::playerState_t,
                *args.offset(5 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        16 => {
            SV_GameDropClient(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *const libc::c_char,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        17 => {
            SV_GameSendServerCommand(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *const libc::c_char,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        30 => {
            crate::src::server::sv_world::SV_LinkEntity(crate::src::vm::vm::VM_ArgPtr(
                *args.offset(1 as libc::c_int as isize),
            )
                as *mut crate::g_public_h::sharedEntity_t);
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        31 => {
            crate::src::server::sv_world::SV_UnlinkEntity(crate::src::vm::vm::VM_ArgPtr(
                *args.offset(1 as libc::c_int as isize),
            )
                as *mut crate::g_public_h::sharedEntity_t);
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        32 => {
            return crate::src::server::sv_world::SV_AreaEntities(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *const crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *const crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut libc::c_int,
                *args.offset(4 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        33 => {
            return SV_EntityContact(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *const crate::g_public_h::sharedEntity_t,
                crate::src::qcommon::q_shared::qfalse as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        44 => {
            return SV_EntityContact(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *const crate::g_public_h::sharedEntity_t,
                crate::src::qcommon::q_shared::qtrue as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        24 => {
            crate::src::server::sv_world::SV_Trace(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::trace_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *const crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(4 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(5 as libc::c_int as isize))
                    as *const crate::src::qcommon::q_shared::vec_t,
                *args.offset(6 as libc::c_int as isize) as libc::c_int,
                *args.offset(7 as libc::c_int as isize) as libc::c_int,
                crate::src::qcommon::q_shared::qfalse as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        43 => {
            crate::src::server::sv_world::SV_Trace(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::trace_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *const crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(4 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(5 as libc::c_int as isize))
                    as *const crate::src::qcommon::q_shared::vec_t,
                *args.offset(6 as libc::c_int as isize) as libc::c_int,
                *args.offset(7 as libc::c_int as isize) as libc::c_int,
                crate::src::qcommon::q_shared::qtrue as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        25 => {
            return crate::src::server::sv_world::SV_PointContents(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *const crate::src::qcommon::q_shared::vec_t,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        23 => {
            SV_SetBrushModel(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::g_public_h::sharedEntity_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *const libc::c_char,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        26 => {
            return SV_inPVS(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *const crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *const crate::src::qcommon::q_shared::vec_t,
            ) as crate::stdlib::intptr_t
        }
        27 => {
            return SV_inPVSIgnorePortals(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *const crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *const crate::src::qcommon::q_shared::vec_t,
            ) as crate::stdlib::intptr_t
        }
        18 => {
            crate::src::server::sv_init::SV_SetConfigstring(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *const libc::c_char,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        19 => {
            crate::src::server::sv_init::SV_GetConfigstring(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
                *args.offset(3 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        21 => {
            crate::src::server::sv_init::SV_SetUserinfo(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *const libc::c_char,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        20 => {
            crate::src::server::sv_init::SV_GetUserinfo(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
                *args.offset(3 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        22 => {
            SV_GetServerinfo(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut libc::c_char,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        28 => {
            SV_AdjustAreaPortalState(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::g_public_h::sharedEntity_t,
                *args.offset(2 as libc::c_int as isize) as crate::src::qcommon::q_shared::qboolean,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        29 => {
            return crate::src::qcommon::cm_test::CM_AreasConnected(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        34 => return crate::src::server::sv_bot::SV_BotAllocateClient() as crate::stdlib::intptr_t,
        35 => {
            crate::src::server::sv_bot::SV_BotFreeClient(
                *args.offset(1 as libc::c_int as isize) as libc::c_int
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        36 => {
            SV_GetUsercmd(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::usercmd_t,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        37 => {
            let mut s: *const libc::c_char = 0 as *const libc::c_char;
            s = crate::src::qcommon::q_shared::COM_Parse(
                &mut crate::src::server::sv_main::sv.entityParsePoint,
            );
            crate::src::qcommon::q_shared::Q_strncpyz(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut libc::c_char,
                s,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
            );
            if crate::src::server::sv_main::sv.entityParsePoint.is_null()
                && *s.offset(0 as libc::c_int as isize) == 0
            {
                return crate::src::qcommon::q_shared::qfalse as libc::c_int
                    as crate::stdlib::intptr_t;
            } else {
                return crate::src::qcommon::q_shared::qtrue as libc::c_int
                    as crate::stdlib::intptr_t;
            }
        }
        39 => {
            return crate::src::server::sv_bot::BotImport_DebugPolygonCreate(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec3_t,
            ) as crate::stdlib::intptr_t
        }
        40 => {
            crate::src::server::sv_bot::BotImport_DebugPolygonDelete(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        41 => {
            return crate::src::qcommon::common::Com_RealTime(crate::src::vm::vm::VM_ArgPtr(
                *args.offset(1 as libc::c_int as isize),
            )
                as *mut crate::src::qcommon::q_shared::qtime_t)
                as crate::stdlib::intptr_t
        }
        42 => {
            crate::src::asm::snapvector::qsnapvectorsse(crate::src::vm::vm::VM_ArgPtr(
                *args.offset(1 as libc::c_int as isize),
            )
                as *mut crate::src::qcommon::q_shared::vec_t);
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        200 => {
            //====================================
            return crate::src::server::sv_bot::SV_BotLibSetup() as crate::stdlib::intptr_t;
        }
        201 => return crate::src::server::sv_bot::SV_BotLibShutdown() as crate::stdlib::intptr_t,
        202 => {
            return (*botlib_export)
                .BotLibVarSet
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *const libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *const libc::c_char,
            ) as crate::stdlib::intptr_t
        }
        203 => {
            return (*botlib_export)
                .BotLibVarGet
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *const libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
                *args.offset(3 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        204 => {
            return (*botlib_export)
                .PC_AddGlobalDefine
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut libc::c_char,
            ) as crate::stdlib::intptr_t
        }
        578 => {
            return (*botlib_export)
                .PC_LoadSourceHandle
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *const libc::c_char,
            ) as crate::stdlib::intptr_t
        }
        579 => {
            return (*botlib_export)
                .PC_FreeSourceHandle
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        580 => {
            return (*botlib_export)
                .PC_ReadTokenHandle
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::pc_token_t,
            ) as crate::stdlib::intptr_t
        }
        581 => {
            return (*botlib_export)
                .PC_SourceFileAndLine
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        205 => {
            return (*botlib_export)
                .BotLibStartFrame
                .expect("non-null function pointer")(_vmf(
                *args.offset(1 as libc::c_int as isize),
            )) as crate::stdlib::intptr_t
        }
        206 => {
            return (*botlib_export)
                .BotLibLoadMap
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *const libc::c_char,
            ) as crate::stdlib::intptr_t
        }
        207 => {
            return (*botlib_export)
                .BotLibUpdateEntity
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::botlib_h::bot_entitystate_t,
            ) as crate::stdlib::intptr_t
        }
        208 => {
            return (*botlib_export).Test.expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(4 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
            ) as crate::stdlib::intptr_t
        }
        209 => {
            return crate::src::server::sv_bot::SV_BotGetSnapshotEntity(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        210 => {
            return crate::src::server::sv_bot::SV_BotGetConsoleMessage(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
                *args.offset(3 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        211 => {
            let mut clientNum: libc::c_int = *args.offset(1 as libc::c_int as isize) as libc::c_int;
            if clientNum >= 0 as libc::c_int
                && clientNum < (*crate::src::server::sv_main::sv_maxclients).integer
            {
                crate::src::server::sv_client::SV_ClientThink(
                    &mut *crate::src::server::sv_main::svs
                        .clients
                        .offset(clientNum as isize),
                    crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                        as *mut crate::src::qcommon::q_shared::usercmd_t,
                );
            }
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        301 => {
            return (*botlib_export)
                .aas
                .AAS_BBoxAreas
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut libc::c_int,
                *args.offset(4 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        302 => {
            return (*botlib_export)
                .aas
                .AAS_AreaInfo
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::be_aas_h::aas_areainfo_s,
            ) as crate::stdlib::intptr_t
        }
        575 => {
            return (*botlib_export)
                .aas
                .AAS_AlternativeRouteGoals
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                *args.offset(4 as libc::c_int as isize) as libc::c_int,
                *args.offset(5 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(6 as libc::c_int as isize))
                    as *mut crate::be_aas_h::aas_altroutegoal_s,
                *args.offset(7 as libc::c_int as isize) as libc::c_int,
                *args.offset(8 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        303 => {
            (*botlib_export)
                .aas
                .AAS_EntityInfo
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::be_aas_h::aas_entityinfo_s,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        304 => {
            return (*botlib_export)
                .aas
                .AAS_Initialized
                .expect("non-null function pointer")()
                as crate::stdlib::intptr_t
        }
        305 => {
            (*botlib_export)
                .aas
                .AAS_PresenceTypeBoundingBox
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        306 => {
            return FloatAsInt((*botlib_export)
                .aas
                .AAS_Time
                .expect("non-null function pointer")())
                as crate::stdlib::intptr_t
        }
        307 => {
            return (*botlib_export)
                .aas
                .AAS_PointAreaNum
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
            ) as crate::stdlib::intptr_t
        }
        577 => {
            return (*botlib_export)
                .aas
                .AAS_PointReachabilityAreaIndex
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
            ) as crate::stdlib::intptr_t
        }
        308 => {
            return (*botlib_export)
                .aas
                .AAS_TraceAreas
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(4 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec3_t,
                *args.offset(5 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        309 => {
            return (*botlib_export)
                .aas
                .AAS_PointContents
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
            ) as crate::stdlib::intptr_t
        }
        310 => {
            return (*botlib_export)
                .aas
                .AAS_NextBSPEntity
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        311 => {
            return (*botlib_export)
                .aas
                .AAS_ValueForBSPEpairKey
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut libc::c_char,
                *args.offset(4 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        312 => {
            return (*botlib_export)
                .aas
                .AAS_VectorForBSPEpairKey
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
            ) as crate::stdlib::intptr_t
        }
        313 => {
            return (*botlib_export)
                .aas
                .AAS_FloatForBSPEpairKey
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut libc::c_float,
            ) as crate::stdlib::intptr_t
        }
        314 => {
            return (*botlib_export)
                .aas
                .AAS_IntForBSPEpairKey
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        315 => {
            return (*botlib_export)
                .aas
                .AAS_AreaReachability
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        316 => {
            return (*botlib_export)
                .aas
                .AAS_AreaTravelTimeToGoalArea
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                *args.offset(3 as libc::c_int as isize) as libc::c_int,
                *args.offset(4 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        300 => {
            return (*botlib_export)
                .aas
                .AAS_EnableRoutingArea
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        576 => {
            return (*botlib_export)
                .aas
                .AAS_PredictRoute
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::be_aas_h::aas_predictroute_s,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                *args.offset(4 as libc::c_int as isize) as libc::c_int,
                *args.offset(5 as libc::c_int as isize) as libc::c_int,
                *args.offset(6 as libc::c_int as isize) as libc::c_int,
                *args.offset(7 as libc::c_int as isize) as libc::c_int,
                *args.offset(8 as libc::c_int as isize) as libc::c_int,
                *args.offset(9 as libc::c_int as isize) as libc::c_int,
                *args.offset(10 as libc::c_int as isize) as libc::c_int,
                *args.offset(11 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        317 => {
            return (*botlib_export)
                .aas
                .AAS_Swimming
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
            ) as crate::stdlib::intptr_t
        }
        318 => {
            return (*botlib_export)
                .aas
                .AAS_PredictClientMovement
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::be_aas_h::aas_clientmove_s,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                *args.offset(4 as libc::c_int as isize) as libc::c_int,
                *args.offset(5 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(6 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(7 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                *args.offset(8 as libc::c_int as isize) as libc::c_int,
                *args.offset(9 as libc::c_int as isize) as libc::c_int,
                _vmf(*args.offset(10 as libc::c_int as isize)),
                *args.offset(11 as libc::c_int as isize) as libc::c_int,
                *args.offset(12 as libc::c_int as isize) as libc::c_int,
                *args.offset(13 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        400 => {
            (*botlib_export)
                .ea
                .EA_Say
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        401 => {
            (*botlib_export)
                .ea
                .EA_SayTeam
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        402 => {
            (*botlib_export)
                .ea
                .EA_Command
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        403 => {
            (*botlib_export)
                .ea
                .EA_Action
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        404 => {
            (*botlib_export)
                .ea
                .EA_Gesture
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        405 => {
            (*botlib_export)
                .ea
                .EA_Talk
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        406 => {
            (*botlib_export)
                .ea
                .EA_Attack
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        407 => {
            (*botlib_export)
                .ea
                .EA_Use
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        408 => {
            (*botlib_export)
                .ea
                .EA_Respawn
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        409 => {
            (*botlib_export)
                .ea
                .EA_Crouch
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        410 => {
            (*botlib_export)
                .ea
                .EA_MoveUp
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        411 => {
            (*botlib_export)
                .ea
                .EA_MoveDown
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        412 => {
            (*botlib_export)
                .ea
                .EA_MoveForward
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        413 => {
            (*botlib_export)
                .ea
                .EA_MoveBack
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        414 => {
            (*botlib_export)
                .ea
                .EA_MoveLeft
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        415 => {
            (*botlib_export)
                .ea
                .EA_MoveRight
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        416 => {
            (*botlib_export)
                .ea
                .EA_SelectWeapon
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        417 => {
            (*botlib_export)
                .ea
                .EA_Jump
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        418 => {
            (*botlib_export)
                .ea
                .EA_DelayedJump
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        419 => {
            (*botlib_export)
                .ea
                .EA_Move
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                _vmf(*args.offset(3 as libc::c_int as isize)),
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        420 => {
            (*botlib_export)
                .ea
                .EA_View
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        421 => {
            (*botlib_export)
                .ea
                .EA_EndRegular
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                _vmf(*args.offset(2 as libc::c_int as isize)),
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        422 => {
            (*botlib_export)
                .ea
                .EA_GetInput
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                _vmf(*args.offset(2 as libc::c_int as isize)),
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut crate::botlib_h::bot_input_t,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        423 => {
            (*botlib_export)
                .ea
                .EA_ResetInput
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        500 => {
            return (*botlib_export)
                .ai
                .BotLoadCharacter
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut libc::c_char,
                _vmf(*args.offset(2 as libc::c_int as isize)),
            ) as crate::stdlib::intptr_t
        }
        501 => {
            (*botlib_export)
                .ai
                .BotFreeCharacter
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        502 => {
            return FloatAsInt((*botlib_export)
                .ai
                .Characteristic_Float
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
            )) as crate::stdlib::intptr_t
        }
        503 => {
            return FloatAsInt((*botlib_export)
                .ai
                .Characteristic_BFloat
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
                _vmf(*args.offset(3 as libc::c_int as isize)),
                _vmf(*args.offset(4 as libc::c_int as isize)),
            )) as crate::stdlib::intptr_t
        }
        504 => {
            return (*botlib_export)
                .ai
                .Characteristic_Integer
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        505 => {
            return (*botlib_export)
                .ai
                .Characteristic_BInteger
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
                *args.offset(3 as libc::c_int as isize) as libc::c_int,
                *args.offset(4 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        506 => {
            (*botlib_export)
                .ai
                .Characteristic_String
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut libc::c_char,
                *args.offset(4 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        507 => {
            return (*botlib_export)
                .ai
                .BotAllocChatState
                .expect("non-null function pointer")()
                as crate::stdlib::intptr_t
        }
        508 => {
            (*botlib_export)
                .ai
                .BotFreeChatState
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        509 => {
            (*botlib_export)
                .ai
                .BotQueueConsoleMessage
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut libc::c_char,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        510 => {
            (*botlib_export)
                .ai
                .BotRemoveConsoleMessage
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        511 => {
            return (*botlib_export)
                .ai
                .BotNextConsoleMessage
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::botlib::be_ai_chat::bot_consolemessage_s,
            ) as crate::stdlib::intptr_t
        }
        512 => {
            return (*botlib_export)
                .ai
                .BotNumConsoleMessages
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        513 => {
            (*botlib_export)
                .ai
                .BotInitialChat
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
                *args.offset(3 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(4 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(5 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(6 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(7 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(8 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(9 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(10 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(11 as libc::c_int as isize))
                    as *mut libc::c_char,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        569 => {
            return (*botlib_export)
                .ai
                .BotNumInitialChats
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
            ) as crate::stdlib::intptr_t
        }
        514 => {
            return (*botlib_export)
                .ai
                .BotReplyChat
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
                *args.offset(3 as libc::c_int as isize) as libc::c_int,
                *args.offset(4 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(5 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(6 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(7 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(8 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(9 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(10 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(11 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(12 as libc::c_int as isize))
                    as *mut libc::c_char,
            ) as crate::stdlib::intptr_t
        }
        515 => {
            return (*botlib_export)
                .ai
                .BotChatLength
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        516 => {
            (*botlib_export)
                .ai
                .BotEnterChat
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
                *args.offset(3 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        570 => {
            (*botlib_export)
                .ai
                .BotGetChatMessage
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
                *args.offset(3 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        517 => {
            return (*botlib_export)
                .ai
                .StringContains
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
                *args.offset(3 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        518 => {
            return (*botlib_export)
                .ai
                .BotFindMatch
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::botlib::be_ai_chat::bot_match_s,
                *args.offset(3 as libc::c_int as isize) as libc::c_ulong,
            ) as crate::stdlib::intptr_t
        }
        519 => {
            (*botlib_export)
                .ai
                .BotMatchVariable
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::src::botlib::be_ai_chat::bot_match_s,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut libc::c_char,
                *args.offset(4 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        520 => {
            (*botlib_export)
                .ai
                .UnifyWhiteSpaces
                .expect("non-null function pointer")(crate::src::vm::vm::VM_ArgPtr(
                *args.offset(1 as libc::c_int as isize),
            )
                as *mut libc::c_char);
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        521 => {
            (*botlib_export)
                .ai
                .BotReplaceSynonyms
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut libc::c_char,
                *args.offset(2 as libc::c_int as isize) as libc::c_ulong,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        522 => {
            return (*botlib_export)
                .ai
                .BotLoadChatFile
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut libc::c_char,
            ) as crate::stdlib::intptr_t
        }
        523 => {
            (*botlib_export)
                .ai
                .BotSetChatGender
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        524 => {
            (*botlib_export)
                .ai
                .BotSetChatName
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
                *args.offset(3 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        525 => {
            (*botlib_export)
                .ai
                .BotResetGoalState
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        526 => {
            (*botlib_export)
                .ai
                .BotResetAvoidGoals
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        571 => {
            (*botlib_export)
                .ai
                .BotRemoveFromAvoidGoals
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        527 => {
            (*botlib_export)
                .ai
                .BotPushGoal
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::botlib::be_ai_goal::bot_goal_s,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        528 => {
            (*botlib_export)
                .ai
                .BotPopGoal
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        529 => {
            (*botlib_export)
                .ai
                .BotEmptyGoalStack
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        530 => {
            (*botlib_export)
                .ai
                .BotDumpAvoidGoals
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        531 => {
            (*botlib_export)
                .ai
                .BotDumpGoalStack
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        532 => {
            (*botlib_export)
                .ai
                .BotGoalName
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
                *args.offset(3 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        533 => {
            return (*botlib_export)
                .ai
                .BotGetTopGoal
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::botlib::be_ai_goal::bot_goal_s,
            ) as crate::stdlib::intptr_t
        }
        534 => {
            return (*botlib_export)
                .ai
                .BotGetSecondGoal
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::botlib::be_ai_goal::bot_goal_s,
            ) as crate::stdlib::intptr_t
        }
        535 => {
            return (*botlib_export)
                .ai
                .BotChooseLTGItem
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut libc::c_int,
                *args.offset(4 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        536 => {
            return (*botlib_export)
                .ai
                .BotChooseNBGItem
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut libc::c_int,
                *args.offset(4 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(5 as libc::c_int as isize))
                    as *mut crate::src::botlib::be_ai_goal::bot_goal_s,
                _vmf(*args.offset(6 as libc::c_int as isize)),
            ) as crate::stdlib::intptr_t
        }
        537 => {
            return (*botlib_export)
                .ai
                .BotTouchingGoal
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::botlib::be_ai_goal::bot_goal_s,
            ) as crate::stdlib::intptr_t
        }
        538 => {
            return (*botlib_export)
                .ai
                .BotItemGoalInVisButNotVisible
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(4 as libc::c_int as isize))
                    as *mut crate::src::botlib::be_ai_goal::bot_goal_s,
            ) as crate::stdlib::intptr_t
        }
        539 => {
            return (*botlib_export)
                .ai
                .BotGetLevelItemGoal
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut crate::src::botlib::be_ai_goal::bot_goal_s,
            ) as crate::stdlib::intptr_t
        }
        567 => {
            return (*botlib_export)
                .ai
                .BotGetNextCampSpotGoal
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::botlib::be_ai_goal::bot_goal_s,
            ) as crate::stdlib::intptr_t
        }
        568 => {
            return (*botlib_export)
                .ai
                .BotGetMapLocationGoal
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::botlib::be_ai_goal::bot_goal_s,
            ) as crate::stdlib::intptr_t
        }
        540 => {
            return FloatAsInt((*botlib_export)
                .ai
                .BotAvoidGoalTime
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
            )) as crate::stdlib::intptr_t
        }
        573 => {
            (*botlib_export)
                .ai
                .BotSetAvoidGoalTime
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
                _vmf(*args.offset(3 as libc::c_int as isize)),
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        541 => {
            (*botlib_export)
                .ai
                .BotInitLevelItems
                .expect("non-null function pointer")();
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        542 => {
            (*botlib_export)
                .ai
                .BotUpdateEntityItems
                .expect("non-null function pointer")();
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        543 => {
            return (*botlib_export)
                .ai
                .BotLoadItemWeights
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
            ) as crate::stdlib::intptr_t
        }
        544 => {
            (*botlib_export)
                .ai
                .BotFreeItemWeights
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        565 => {
            (*botlib_export)
                .ai
                .BotInterbreedGoalFuzzyLogic
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
                *args.offset(3 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        545 => {
            (*botlib_export)
                .ai
                .BotSaveGoalFuzzyLogic
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        566 => {
            (*botlib_export)
                .ai
                .BotMutateGoalFuzzyLogic
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                _vmf(*args.offset(2 as libc::c_int as isize)),
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        546 => {
            return (*botlib_export)
                .ai
                .BotAllocGoalState
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        547 => {
            (*botlib_export)
                .ai
                .BotFreeGoalState
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        548 => {
            (*botlib_export)
                .ai
                .BotResetMoveState
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        574 => {
            (*botlib_export)
                .ai
                .BotAddAvoidSpot
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                _vmf(*args.offset(3 as libc::c_int as isize)),
                *args.offset(4 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        549 => {
            (*botlib_export)
                .ai
                .BotMoveToGoal
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::src::botlib::be_ai_move::bot_moveresult_s,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut crate::src::botlib::be_ai_goal::bot_goal_s,
                *args.offset(4 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        550 => {
            return (*botlib_export)
                .ai
                .BotMoveInDirection
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                _vmf(*args.offset(3 as libc::c_int as isize)),
                *args.offset(4 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        551 => {
            (*botlib_export)
                .ai
                .BotResetAvoidReach
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        552 => {
            (*botlib_export)
                .ai
                .BotResetLastAvoidReach
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        553 => {
            return (*botlib_export)
                .ai
                .BotReachabilityArea
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        554 => {
            return (*botlib_export)
                .ai
                .BotMovementViewTarget
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::botlib::be_ai_goal::bot_goal_s,
                *args.offset(3 as libc::c_int as isize) as libc::c_int,
                _vmf(*args.offset(4 as libc::c_int as isize)),
                crate::src::vm::vm::VM_ArgPtr(*args.offset(5 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
            ) as crate::stdlib::intptr_t
        }
        572 => {
            return (*botlib_export)
                .ai
                .BotPredictVisiblePosition
                .expect("non-null function pointer")(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut crate::src::botlib::be_ai_goal::bot_goal_s,
                *args.offset(4 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(5 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
            ) as crate::stdlib::intptr_t
        }
        555 => {
            return (*botlib_export)
                .ai
                .BotAllocMoveState
                .expect("non-null function pointer")()
                as crate::stdlib::intptr_t
        }
        556 => {
            (*botlib_export)
                .ai
                .BotFreeMoveState
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        557 => {
            (*botlib_export)
                .ai
                .BotInitMoveState
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::botlib::be_ai_move::bot_initmove_s,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        558 => {
            return (*botlib_export)
                .ai
                .BotChooseBestFightWeapon
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        559 => {
            (*botlib_export)
                .ai
                .BotGetWeaponInfo
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut crate::src::botlib::be_ai_weap::weaponinfo_s,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        560 => {
            return (*botlib_export)
                .ai
                .BotLoadWeaponWeights
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_char,
            ) as crate::stdlib::intptr_t
        }
        561 => {
            return (*botlib_export)
                .ai
                .BotAllocWeaponState
                .expect("non-null function pointer")()
                as crate::stdlib::intptr_t
        }
        562 => {
            (*botlib_export)
                .ai
                .BotFreeWeaponState
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        563 => {
            (*botlib_export)
                .ai
                .BotResetWeaponState
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        564 => {
            return (*botlib_export)
                .ai
                .GeneticParentsAndChildSelection
                .expect("non-null function pointer")(
                *args.offset(1 as libc::c_int as isize) as libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut libc::c_float,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(4 as libc::c_int as isize))
                    as *mut libc::c_int,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(5 as libc::c_int as isize))
                    as *mut libc::c_int,
            ) as crate::stdlib::intptr_t
        }
        100 => {
            crate::stdlib::memset(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize)),
                *args.offset(2 as libc::c_int as isize) as libc::c_int,
                *args.offset(3 as libc::c_int as isize) as libc::c_ulong,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        101 => {
            crate::stdlib::memcpy(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize)),
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize)),
                *args.offset(3 as libc::c_int as isize) as libc::c_ulong,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        102 => {
            crate::stdlib::strncpy(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut libc::c_char,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *const libc::c_char,
                *args.offset(3 as libc::c_int as isize) as libc::c_ulong,
            );
            return *args.offset(1 as libc::c_int as isize);
        }
        103 => {
            return FloatAsInt(crate::stdlib::sin(
                _vmf(*args.offset(1 as libc::c_int as isize)) as libc::c_double
            ) as libc::c_float) as crate::stdlib::intptr_t
        }
        104 => {
            return FloatAsInt(crate::stdlib::cos(
                _vmf(*args.offset(1 as libc::c_int as isize)) as libc::c_double
            ) as libc::c_float) as crate::stdlib::intptr_t
        }
        105 => {
            return FloatAsInt(crate::stdlib::atan2(
                _vmf(*args.offset(1 as libc::c_int as isize)) as libc::c_double,
                _vmf(*args.offset(2 as libc::c_int as isize)) as libc::c_double,
            ) as libc::c_float) as crate::stdlib::intptr_t
        }
        106 => {
            return FloatAsInt(crate::stdlib::sqrt(
                _vmf(*args.offset(1 as libc::c_int as isize)) as libc::c_double
            ) as libc::c_float) as crate::stdlib::intptr_t
        }
        107 => {
            crate::src::qcommon::q_math::MatrixMultiply(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut [libc::c_float; 3],
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut [libc::c_float; 3],
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut [libc::c_float; 3],
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        108 => {
            crate::src::qcommon::q_math::AngleVectors(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *const crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(3 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(4 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        109 => {
            crate::src::qcommon::q_math::PerpendicularVector(
                crate::src::vm::vm::VM_ArgPtr(*args.offset(1 as libc::c_int as isize))
                    as *mut crate::src::qcommon::q_shared::vec_t,
                crate::src::vm::vm::VM_ArgPtr(*args.offset(2 as libc::c_int as isize))
                    as *const crate::src::qcommon::q_shared::vec_t,
            );
            return 0 as libc::c_int as crate::stdlib::intptr_t;
        }
        110 => {
            return FloatAsInt(crate::stdlib::floor(
                _vmf(*args.offset(1 as libc::c_int as isize)) as libc::c_double
            ) as libc::c_float) as crate::stdlib::intptr_t
        }
        111 => {
            return FloatAsInt(crate::stdlib::ceil(
                _vmf(*args.offset(1 as libc::c_int as isize)) as libc::c_double
            ) as libc::c_float) as crate::stdlib::intptr_t
        }
        _ => {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as libc::c_int,
                b"Bad game system trap: %ld\x00" as *const u8 as *const libc::c_char,
                *args.offset(0 as libc::c_int as isize),
            );
        }
    };
}
/*
===============
SV_ShutdownGameProgs

Called every time a map changes
===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_ShutdownGameProgs() {
    if crate::src::server::sv_main::gvm.is_null() {
        return;
    }
    crate::src::vm::vm::VM_Call(
        crate::src::server::sv_main::gvm,
        crate::g_public_h::GAME_SHUTDOWN as libc::c_int,
        crate::src::qcommon::q_shared::qfalse as libc::c_int,
    );
    crate::src::vm::vm::VM_Free(crate::src::server::sv_main::gvm);
    crate::src::server::sv_main::gvm = 0 as *mut crate::qcommon_h::vm_t;
}
/*
==================
SV_InitGameVM

Called for both a full init and a restart
==================
*/

unsafe extern "C" fn SV_InitGameVM(mut restart: crate::src::qcommon::q_shared::qboolean) {
    let mut i: libc::c_int = 0;
    // start the entity parsing at the beginning
    crate::src::server::sv_main::sv.entityParsePoint =
        crate::src::qcommon::cm_load::CM_EntityString();
    // clear all gentity pointers that might still be set from
    // a previous level
    // https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=522
    //   now done before GAME_INIT call
    i = 0 as libc::c_int;
    while i < (*crate::src::server::sv_main::sv_maxclients).integer {
        let ref mut fresh0 = (*crate::src::server::sv_main::svs.clients.offset(i as isize)).gentity;
        *fresh0 = 0 as *mut crate::g_public_h::sharedEntity_t;
        i += 1
    }
    // use the current msec count for a random seed
    // init for this gamestate
    crate::src::vm::vm::VM_Call(
        crate::src::server::sv_main::gvm,
        crate::g_public_h::GAME_INIT as libc::c_int,
        crate::src::server::sv_main::sv.time,
        crate::src::qcommon::common::Com_Milliseconds(),
        restart as libc::c_uint,
    );
}
/*
===================
SV_RestartGameProgs

Called on a map_restart, but not on a normal map change
===================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_RestartGameProgs() {
    if crate::src::server::sv_main::gvm.is_null() {
        return;
    }
    crate::src::vm::vm::VM_Call(
        crate::src::server::sv_main::gvm,
        crate::g_public_h::GAME_SHUTDOWN as libc::c_int,
        crate::src::qcommon::q_shared::qtrue as libc::c_int,
    );
    // do a restart instead of a free
    crate::src::server::sv_main::gvm = crate::src::vm::vm::VM_Restart(
        crate::src::server::sv_main::gvm,
        crate::src::qcommon::q_shared::qtrue,
    );
    if crate::src::server::sv_main::gvm.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"VM_Restart on game failed\x00" as *const u8 as *const libc::c_char,
        );
    }
    SV_InitGameVM(crate::src::qcommon::q_shared::qtrue);
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
//
// sv_game.c
//
/*
===============
SV_InitGameProgs

Called on a normal map change, not on a map_restart
===============
*/
#[no_mangle]

pub unsafe extern "C" fn SV_InitGameProgs() {
    let mut var: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    //FIXME these are temp while I make bots run in vm
    extern "C" {
        #[no_mangle]
        pub static mut bot_enable: libc::c_int;
    }
    var = crate::src::qcommon::cvar::Cvar_Get(
        b"bot_enable\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0x20 as libc::c_int,
    );
    if !var.is_null() {
        bot_enable = (*var).integer
    } else {
        bot_enable = 0 as libc::c_int
    }
    // load the dll or bytecode
    crate::src::server::sv_main::gvm = crate::src::vm::vm::VM_Create(
        b"qagame\x00" as *const u8 as *const libc::c_char,
        Some(
            SV_GameSystemCalls
                as unsafe extern "C" fn(_: *mut crate::stdlib::intptr_t) -> crate::stdlib::intptr_t,
        ),
        crate::src::qcommon::cvar::Cvar_VariableValue(
            b"vm_game\x00" as *const u8 as *const libc::c_char,
        ) as crate::qcommon_h::vmInterpret_t,
    );
    if crate::src::server::sv_main::gvm.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as libc::c_int,
            b"VM_Create on game failed\x00" as *const u8 as *const libc::c_char,
        );
    }
    SV_InitGameVM(crate::src::qcommon::q_shared::qfalse);
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
// qcommon.h -- definitions common between client and server, but not game.or ref modules
//Ignore __attribute__ on non-gcc platforms
//#define	PRE_RELEASE_DEMO
//============================================================================
//
// msg.c
//
// if false, do a Com_Error
// set to true if the buffer size failed (with allowoverflow set)
// set to true if the buffer size failed (with allowoverflow set)
// for bitwise reads and writes
// TTimo
// copy a msg_t in case we need to store it as is for a bit
// (as I needed this to keep an msg_t from a static var for later use)
// sets data buffer as MSG_Init does prior to do the copy
//============================================================================
/*
==============================================================

NET

==============================================================
*/
// if this flag is set, always attempt ipv6 connections instead of ipv4 if a v6 address is found.
// disables ipv6 multicast support if set.
// number of old messages that must be kept on client and
// server for delta comrpession and ping estimation
// max number of usercmd_t in a packet
// max string commands buffered for restransmit
// an address lookup failed
// maximum length of an IPv6 address string including trailing '\0'
// Needed for IPv6 link-local addresses
// max length of a message, which may
// be fragmented into multiple packets
// ACK window of 48 download chunks. Cannot set this higher, or clients
// will overflow the reliable commands buffer
// 896 byte block chunks
/*
Netchan handles packet fragmentation and out of order / duplicate suppression
*/
// between last packet and previous
// qport value to write when transmitting
// sequencing variables
// incoming fragment assembly buffer
// outgoing fragment buffer
// we need to space out the sending of large fragmented messages
/*
==============================================================

PROTOCOL

==============================================================
*/
// 1.31 - 67
// maintain a list of compatible protocols for demo playing
// NOTE: that stuff only works with two digits protocols
// override on command line, config files etc.
// broadcast scan this many ports after
// PORT_SERVER so a single machine can
// run multiple servers
// the svc_strings[] array in cl_parse.c should mirror this
//
// server to client
//
// [short] [string] only in gamestate messages
// only in gamestate messages
// [string] to be executed by client game module
// [short] size [size bytes]
// new commands, supported only by ioquake3 protocol but not legacy
// not wrapped in USE_VOIP, so this value is reserved.
//
//
// client to server
//
// [[usercmd_t]
// [[usercmd_t]
// [string] message
// new commands, supported only by ioquake3 protocol but not legacy
// not wrapped in USE_VOIP, so this value is reserved.
//
/*
==============================================================

VIRTUAL MACHINE

==============================================================
*/
// module should be bare: "cgame", not "cgame.dll" or "vm/cgame.qvm"
/*
==============================================================

CMD

Command text buffering and command execution

==============================================================
*/
/*

Any number of commands can be added in a frame, from several different sources.
Most commands come from either keybindings or console line input, but entire text
files can be execed.

*/
// allocates an initial text buffer that will grow as needed
// Adds command text at the end of the buffer, does NOT add a final \n
// this can be used in place of either Cbuf_AddText or Cbuf_InsertText
// Pulls off \n terminated lines of text from the command buffer and sends
// them through Cmd_ExecuteString.  Stops when the buffer is empty.
// Normally called once per frame, but may be explicitly invoked.
// Do not call inside a command function, or current args will be destroyed.
//===========================================================================
/*

Command execution takes a null terminated string, breaks it into tokens,
then searches for a command or variable that matches the first token.

*/
// called by the init functions of other parts of the program to
// register commands and functions to call for them.
// The cmd_name is referenced later, so it should not be in temp memory
// if function is NULL, the command will be forwarded to the server
// as a clc_clientCommand instead of executed locally
// don't allow VMs to remove system commands
// callback with each valid string
// The functions that execute commands get their parameters with these
// functions. Cmd_Argv () will return an empty string, not a NULL
// if arg > argc, so string operations are allways safe.
// Takes a null terminated string.  Does not need to be /n terminated.
// breaks the string up into arg tokens.
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
// creates the variable if it doesn't exist, or returns the existing one
// if it exists, the value will not be changed, but flags will be ORed in
// that allows variables to be unarchived without needing bitflags
// if value is "", the value will not override a previously set value.
// basically a slightly modified Cvar_Get for the interpreted modules
// updates an interpreted modules' version of a cvar
// will create the variable with no flags if it doesn't exist
// same as Cvar_Set, but allows more control over setting of cvar
// sometimes we set variables from an untrusted source: fail if flags & CVAR_PROTECTED
// don't set the cvar immediately
// expands value to a string and calls Cvar_Set/Cvar_SetSafe
// returns 0 if not defined or non numeric
// returns an empty string if not defined
// returns CVAR_NONEXISTENT if cvar doesn't exist or the flags of that particular CVAR.
// callback with each valid string
// reset all testing vars to a safe value
// called by Cmd_ExecuteString when Cmd_Argv(0) doesn't match a known
// command.  Returns true if the command was a variable reference that
// was handled. (print or change)
// writes lines containing "set variable value" for all variables
// with the archive flag set to true.
// returns an info string containing all the cvars that have the given bit set
// in their flags ( CVAR_USERINFO, CVAR_SERVERINFO, CVAR_SYSTEMINFO, etc )
// whenever a cvar is modifed, its flags will be OR'd into this, so
// a single check can determine if any CVAR_USERINFO, CVAR_SERVERINFO,
// etc, variables have been modified since the last check.  The bit
// can then be cleared to allow another change detection.
/*
==============================================================

FILESYSTEM

No stdio calls should be used by any part of the game, because
we need to deal with all sorts of directory and seperator char
issues.
==============================================================
*/
// referenced flags
// these are in loop specific order so don't change the order
// number of id paks that will never be autodownloaded from baseq3/missionpack
// shutdown and restart the filesystem so changes to fs_gamedir can take effect
// directory should not have either a leading or trailing /
// if extension is "/", only subdirectories will be returned
// the returned files will not include any directories or /
// will properly create any needed paths and deal with seperater character issues
// if uniqueFILE is true, then a new FILE will be fopened even if the file
// is found in an already open pak file.  If uniqueFILE is false, you must call
// FS_FCloseFile instead of fclose, otherwise the pak FILE would be improperly closed
// It is generally safe to always set uniqueFILE to true, because the majority of
// file IO goes through FS_ReadFile, which Does The Right Thing already.
// returns 1 if a file is in the PAK file, otherwise -1
// properly handles partial reads and reads from other dlls
// note: you can't just fclose from another DLL, due to MS libc issues
// returns the length of the file
// a null buffer will just return the file length without loading
// as a quick check for existence. -1 length == not present
// A 0 byte will always be appended at the end, so string ops are safe.
// the buffer should be considered read-only, because it may be cached
// for other uses.
// forces flush on files we're writing to.
// frees the memory returned by FS_ReadFile
// writes a complete file, creating any subdirectories needed
// doesn't work for files that are opened from a pack file
// where are we?
// like fprintf
// opens a file for reading, writing, or appending depending on the value of mode
// seek on a file
// Returns a space separated string containing the checksums of all loaded pk3 files.
// Servers with sv_pure set will get this string and pass it to clients.
// Returns a space separated string containing the checksums of all loaded
// AND referenced pk3 files. Servers with sv_pure set will get this string
// back from clients for pure validation
// clears referenced booleans on loaded pk3s
// If the string is empty, all data sources will be allowed.
// If not empty, only pk3 files that match one of the space
// separated checksums will be checked for files, with the
// sole exception of .cfg files.
/*
==============================================================

Edit fields and command line history/completion

==============================================================
*/
/*
==============================================================

MISC

==============================================================
*/
// centralizing the declarations for cl_cdkey
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=470
// returned by Sys_GetProcessorFeatures
// centralized and cleaned, that's the max string you can send to a Com_Printf / Com_DPrintf (above gets truncated)
// SE_NONE must be zero
// evTime is still valid
// evValue is a key code, evValue2 is the down flag
// evValue is an ascii char
// evValue and evValue2 are relative signed x / y moves
// evValue is an axis number and evValue2 is the current state (-127 to 127)
// evPtr is a char*
// bytes of data pointed to by evPtr, for journaling
// this must be manually freed if not NULL
// will be journaled properly
// checks for and removes command line "+set var arg" constructs
// if match is NULL, all set commands will be executed, otherwise
// only a set with the exact name.  Only used during startup.
// for building release pak files
// both client and server must agree to pause
// com_speeds times
// renderer backend time
/*

--- low memory ----
server vm
server clipmap
---mark---
renderer initialization (shaders, etc)
UI vm
cgame vm
renderer map
renderer models

---free---

temp file loading
--- high memory ---

*/
// NOT 0 filled memory
// returns 0 filled memory
// returns 0 filled memory
// commandLine should not include the executable name (argv[0])
/*
==============================================================

CLIENT / SERVER SYSTEMS

==============================================================
*/
//
// client interface
//
// the keyboard binding interface must be setup before execing
// config files, but the rest of client startup will happen later
// char events are for field typing, not game control
// do a screen update before starting to load a map
// when the server is going to load a new map, the entire hunk
// will be cleared, so the client must shutdown cgame, ui, and
// the renderer
// adds the current command line as a clc_clientCommand to the client message.
// things like godmode, noclip, etc, are commands directed to the server,
// so when they are typed in at the console, they will need to be forwarded.
// bring up the "need a cd to play" dialog
// dump all memory on an error
// shutdown client
// initialize renderer interface
// start all the client stuff using the hunk
// Restart sound subsystem
// for keyname autocompletion
// for writing the config files
// call before filesystem access
// FIXME: move logging to common?
// AVI files have the start of pixel lines 4 byte-aligned
//
// server interface
//
/*
====================
SV_GameCommand

See if the current console command is claimed by the game
====================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_GameCommand() -> crate::src::qcommon::q_shared::qboolean {
    if crate::src::server::sv_main::sv.state as libc::c_uint
        != crate::server_h::SS_GAME as libc::c_int as libc::c_uint
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::vm::vm::VM_Call(
        crate::src::server::sv_main::gvm,
        crate::g_public_h::GAME_CONSOLE_COMMAND as libc::c_int,
    ) as crate::src::qcommon::q_shared::qboolean;
}
