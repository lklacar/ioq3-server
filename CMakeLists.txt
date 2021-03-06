cmake_minimum_required(VERSION 3.15)
project(q3_server C)

add_compile_definitions(ARCH_STRING=x86_64)
add_compile_definitions(QDECL=)
#add_compile_definitions(SCREWUP)
add_compile_definitions(BOTLIB)
add_compile_definitions(DEDICATED)
#add_compile_definitions(Q3_VM)

set(CMAKE_C_STANDARD 99)

add_executable(q3_server
        src/sys/sys_autoupdater.c
        src/null/null_snddma.c
        src/botlib/be_ea.c
        src/botlib/be_interface.h
        src/botlib/be_aas_main.h
        src/sys/sys_loadlib.h
        src/botlib/l_precomp.h
        src/botlib/be_ai_chat.c
        src/server/sv_client.c
        src/botlib/be_aas_bspq3.c
        src/qcommon/cm_polylib.c
        src/botlib/l_log.c
        src/qcommon/cmd.c
        src/botlib/be_aas_reach.c
        src/qcommon/huffman.c
        src/qcommon/cm_patch.c
        src/qcommon/cm_trace.c
        src/vm/vm_x86.c
        src/botlib/be_aas_entity.h
        src/sys/sys_main.c
        src/sys/sys_unix.c
        src/qcommon/q_platform.h
        src/botlib/be_aas_routealt.c
        src/botlib/l_script.c
        src/botlib/be_aas_debug.h
        src/qcommon/common.c
        src/server/sv_init.c
        src/botlib/be_ai_goal.c
        src/botlib/botlib.h
        src/botlib/be_ai_move.h
        src/qcommon/cm_load.c
        src/botlib/be_aas_move.h
        src/botlib/be_aas_move.c
        src/botlib/be_aas_cluster.h
        src/qcommon/surfaceflags.h
        src/botlib/be_ai_chat.h
        src/botlib/l_crc.c
        src/botlib/be_ai_goal.h
        src/botlib/l_crc.h
        src/qcommon/qfiles.h
        src/botlib/be_aas_cluster.c
        src/qcommon/files.c
        src/server/server.h
        src/sys/sys_local.h
        src/botlib/be_aas_entity.c
        src/vm/vm_local.h
        src/sys/con_log.c
        src/botlib/l_utils.h
        src/qcommon/unzip.c
        src/vm/vm_interpreted.c
        src/qcommon/cm_patch.h
        src/botlib/be_aas_optimize.c
        src/asm/qasm-inline.h
        src/qcommon/qcommon.h
        src/botlib/l_precomp.c
        src/qcommon/unzip.h
        src/botlib/be_aas_sample.c
        src/botlib/be_ai_weight.h
        src/botlib/l_memory.h
        src/botlib/be_ai_gen.c
        src/qcommon/cm_local.h
        src/qcommon/cm_test.c
        src/qcommon/cvar.c
        src/qcommon/ioapi.h
        src/null/null_client.c
        src/vm/vm.c
        src/qcommon/msg.c
        src/botlib/be_aas_sample.h
        src/botlib/be_aas.h
        src/qcommon/ioapi.c
        src/botlib/be_aas_file.h
        src/botlib/l_libvar.c
        src/botlib/be_aas_debug.c
        src/botlib/be_aas_def.h
        src/botlib/be_aas_main.c
        src/qcommon/net_ip.c
        src/qcommon/md4.c
        src/botlib/be_ai_weap.h
        src/game/g_public.h
        src/botlib/be_ai_char.c
        src/server/sv_ccmds.c
        src/botlib/l_struct.h
        src/qcommon/q_shared.c
        src/botlib/l_memory.c
        src/botlib/be_interface.c
        src/server/sv_game.c
        src/sys/con_tty.c
        src/server/sv_main.c
        src/qcommon/q_shared.h
        src/server/sv_world.c
        src/botlib/be_aas_file.c
        src/botlib/l_script.h
        src/botlib/be_aas_route.h
        src/botlib/be_ea.h
        src/botlib/l_libvar.h
        src/server/sv_net_chan.c
        src/botlib/be_ai_weight.c
        src/botlib/be_ai_weap.c
        src/asm/snapvector.c
        src/qcommon/cm_public.h
        src/botlib/be_aas_route.c
        src/botlib/be_ai_move.c
        src/botlib/be_aas_routealt.h
        src/botlib/l_log.h
        src/server/sv_snapshot.c
        src/botlib/bg_public.h
        src/botlib/l_struct.c
        src/botlib/be_ai_gen.h
        src/botlib/be_ai_char.h
        src/botlib/aasfile.h
        src/botlib/be_aas_reach.h
        src/botlib/be_aas_bsp.h
        src/botlib/be_aas_optimize.h
        src/qcommon/q_common.h
        src/asm/ftola.c
        src/botlib/be_aas_funcs.h
        src/qcommon/net_chan.c
        src/server/sv_bot.c
        src/null/null_input.c
        src/qcommon/cm_polylib.h
        src/botlib/bg_lib.c
        src/botlib/bg_lib.h
        src/qcommon/q_math.c
        )

target_link_libraries(q3_server m z ${CMAKE_DL_LIBS})
