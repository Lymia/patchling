package.cpath = "lua_modules/lib/lua/5.1/?.so"
package.path = "patchling_rt/?.lua;lua_modules/share/lua/5.1/?.lua;lua_modules/share/lua/5.1/?/init.lua;metalua/?.lua"

require "metalua.loader"
require "metalua.compiler.globals"

local lfs = require "lfs"
local mlc = require "metalua.compiler"
local into_src = require "patchling_private.ast_to_src"

local function read_all(file)
    local f = assert(io.open(file, "rb"))
    local content = f:read("*all")
    f:close()
    return content
end
local function compile_to_src(file)
    local src = read_all(file)
    local ast = mlc.new():src_to_ast(src)
    return into_src(ast)
end

lfs.mkdir("target/lua_src")
io.open("target/lua_src/ast_to_src_precompiled.lua", "w")
        :write(compile_to_src("patchling_rt/patchling_private/ast_to_src.mlua"))
