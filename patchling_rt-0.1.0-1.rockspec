package = "patchling_rt"
version = "0.1.0-1"
source = {
   url = "https://github.com/Lymia/patchling"
}
description = {
   homepage = "https://github.com/Lymia/patchling",
   license = "MIT/Apache-2.0"
}
build = {
    type="command",
    build_command = "lua5.1 patchling_rt/patchling_private/build.lua",
    install = {
        lua = {
            ["patchling_private.mlua.ast_to_src_precompiled"] =
                "target/lua_src/ast_to_src_precompiled.lua",
            ["patchling_private.mlua.metalua_compiler"] =
                "patchling_rt/patchling_private/mlua/metalua_compiler.lua",
            ["patchling_private.mlua.metalua_globals"] =
                "patchling_rt/patchling_private/mlua/metalua_globals.lua",

            ["patchling_private.privileged.checks"] =
                "patchling_rt/patchling_private/privileged/checks.lua",
            ["patchling_private.privileged.metalua_loader"] =
                "patchling_rt/patchling_private/privileged/metalua_loader.lua",
            ["patchling_private.privileged.traceback"] =
                "patchling_rt/patchling_private/privileged/traceback.lua",

            ["patchling_private.check_error"] = "patchling_rt/patchling_private/check_error.lua",
            ["patchling_private.compile_and_minify"] = "patchling_rt/patchling_private/compile_and_minify.lua",
        }
    }
}
dependencies = {
    "lua ~> 5.1",
    "luasrcdiet >= 1.0",
    "metalua-compiler >= 0.7.2",
    "metalua-parser >= 0.7.2",
}

