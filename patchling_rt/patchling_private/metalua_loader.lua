--------------------------------------------------------------------------------
-- Copyright (c) 2006-2013 Fabien Fleutot and others.
--
-- All rights reserved.
--
-- This program and the accompanying materials are made available
-- under the terms of the Eclipse Public License v1.0 which
-- accompanies this distribution, and is available at
-- http://www.eclipse.org/legal/epl-v10.html
--
-- This program and the accompanying materials are also made available
-- under the terms of the MIT public license which accompanies this
-- distribution, and is available at http://www.lua.org/license.html
--
-- Contributors:
--     Fabien Fleutot - API and implementation
--     AuroraAmissa - Modify this to account for being privileged code.
--
--------------------------------------------------------------------------------

-- NOTE: This is privileged code and has access to functions that should not be available to user code.
--       Take extra care when editing this file.

-- TODO: Annotate tracebacks with the mod a module is from.

local M = require "package" -- extend Lua's basic "package" module

-- Copy important functions to upvalues
local checks = checks
local io_open = io.open
local loadstring = loadstring
local require = require
local string_format = string.format
local string_gsub = string.gsub
local table_concat = table.concat
local table_insert = table.insert

-- Various constants
local metalua_extension_prefix = 'metalua.extension.'

-- Create the path cache.
local path_cache = {}
table.insert(path_cache, {
    mod_name = "patchling",
    root = M.modules_path .. "/share/lua/5.1/",
})
for _, mod in ipairs(M.mod_paths) do
    for _, lib in ipairs(mod.lib_paths) do
        table.insert(path_cache, {
            mod_name = mod.info.id,
            root = lib .. "/",
        })
    end
end
M.mod_paths = nil -- this is a large table, so we remove it now

-- Load references to other modules
local register_file = (require "patchling_private.traceback").register_file

----------------------------------------------------------------------
-- Take a Lua module name, return the open file and its name,
-- or <false> and an error message.
----------------------------------------------------------------------
function M.findfile(name, path_string, no, extension)
    name = string_gsub(name, '%.', "/")
    local errors = { }
    for _, path in ipairs(path_cache) do
        local filename = path.root .. name .. extension
        local file = io_open(filename, 'r')
        if file then
            return file, filename, name .. extension, path.mod_name
        end
        table_insert(errors, string_format("\tno "..no.." file %q", filename))
    end
    return false, '\n' .. table_concat(errors, "\n") .. '\n'
end

----------------------------------------------------------------------
-- Load a Lua source file.
----------------------------------------------------------------------
function M.lua_loader (name)
    checks('string')

    local file, filename_or_msg, short_name, mod_name = M.findfile(name, M.path, "lua", ".lua")
    if not file then
        return filename_or_msg
    end

    local luastring = file:read '*a'
    file:close()

    local fn, err = loadstring(luastring, "@"..filename_or_msg)
    if not fn then
        error(err)
    else
        register_file(filename_or_msg, short_name, mod_name)
        return fn
    end
end

----------------------------------------------------------------------
-- Load a metalua source file.
----------------------------------------------------------------------
function M.metalua_loader (name)
    checks('string')

    local compiler = M.loaded["metalua.compiler"]
    if compiler then
        local file, filename_or_msg, short_name, mod_name = M.findfile(name, M.mpath, "metalua", ".mlua")
        if not file then
            return filename_or_msg
        end

        local luastring = file:read '*a'
        file:close()

        local fn, err = compiler.new():src_to_function(luastring, "@"..filename_or_msg)
        if not fn then
            error(err)
        else
            register_file(filename_or_msg, short_name, mod_name)
            return fn
        end
    else
        return "tno metalua compiler\n"
    end
end

----------------------------------------------------------------------
-- Placed after lua/luac loader, so precompiled files have
-- higher precedence.
----------------------------------------------------------------------
M.loaders[2] = M.lua_loader
table.insert(M.loaders, M.metalua_loader)

----------------------------------------------------------------------
-- Load an extension.
----------------------------------------------------------------------
function extension (name, mlp)
    checks('string', 'string')

    local complete_name = metalua_extension_prefix .. name
    local extend_func = require(complete_name)
    if not mlp.extensions[complete_name] then
        local ast = extend_func(mlp)
        mlp.extensions[complete_name] = extend_func
        return ast
    end
end
