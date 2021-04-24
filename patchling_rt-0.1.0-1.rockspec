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
    build_command = "lua5.1 patchling_rt/patchling_private/build_package.lua",
    install = {
        lua = {
            ["patchling_private.ast_to_src_precompiled"] = "target/lua_src/ast_to_src_precompiled.lua",
            ["patchling_private.check_error"] = "patchling_rt/patchling_private/check_error.lua",
            ["patchling_private.checks"] = "patchling_rt/patchling_private/checks.lua",
            ["patchling_private.compile_and_minify"] = "patchling_rt/patchling_private/compile_and_minify.lua",
            ["patchling_private.metalua_compiler"] = "patchling_rt/patchling_private/metalua_compiler.lua",
            ["patchling_private.metalua_globals"] = "patchling_rt/patchling_private/metalua_globals.lua",
            ["patchling_private.metalua_loader"] = "patchling_rt/patchling_private/metalua_loader.lua",
            ["patchling_private.traceback"] = "patchling_rt/patchling_private/traceback.lua"
        }
    }
}
