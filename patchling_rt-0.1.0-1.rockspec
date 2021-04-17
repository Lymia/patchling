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
            ["patchling_private.load_metalua"] = "patchling_rt/patchling_private/load_metalua.lua"
        }
    }
}
