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

local M = require "package" -- extend Lua's basic "package" module

-- Copy important functions to upvalues
local io_open = io.open
local require = require
local string_format = string.format
local string_gmatch = string.gmatch
local string_gsub = string.gsub
local table_concat = table.concat
local table_insert = table.insert

-- Create the extension prefix
M.metalua_extension_prefix = 'metalua.extension.'

-- Initialize package.mpath from package.path
M.mpath = (M.path .. ";"):gsub("%.(lua[:;])", ".m%1"):sub(1, -2)

----------------------------------------------------------------------
-- resc(k) returns "%"..k if it's a special regular expression char,
-- or just k if it's normal.
----------------------------------------------------------------------
local regexp_magic = { }
for k in ("^$()%.[]*+-?"):gmatch "." do
    regexp_magic[k] = "%" .. k
end

local function resc(k)
    return regexp_magic[k] or k
end

----------------------------------------------------------------------
-- Take a Lua module name, return the open file and its name,
-- or <false> and an error message.
----------------------------------------------------------------------
do
    local config_regexp = ("([^\n])\n"):rep(5):sub(1, -2)
    local dir_sep, path_sep, path_mark, execdir, igmark = M.config:match(config_regexp)

    function M.findfile(name, path_string)
        name = string_gsub(name, '%.', dir_sep)
        local errors = { }
        local path_pattern = string_format('[^%s]+', resc(path_sep))
        for path in string_gmatch(path_string, path_pattern) do
            local filename = string_gsub(path, resc(path_mark), name)
            local file = io_open(filename, 'r')
            if file then
                return file, filename
            end
            table_insert(errors, string_format("\tno metalua file %q", filename))
        end
        return false, '\n' .. table_concat(errors, "\n") .. '\n'
    end
end

----------------------------------------------------------------------
-- Load a metalua source file.
----------------------------------------------------------------------
function M.metalua_loader (name)
    local file, filename_or_msg = M.findfile(name, M.mpath)
    if not file then
        return filename_or_msg
    end

    local luastring = file:read '*a'
    file:close()

    local compiler = M.loaded["metalua.compiler"]
    if compiler then
        fn, err = compiler.new():src_to_function(luastring, name)
        if not fn then
            error(err)
        else
            return fn
        end
    else
        return false, "Metalua compiler not loaded."
    end
end

----------------------------------------------------------------------
-- Placed after lua/luac loader, so precompiled files have
-- higher precedence.
----------------------------------------------------------------------
table.insert(M.loaders, M.metalua_loader)

----------------------------------------------------------------------
-- Load an extension.
----------------------------------------------------------------------
function extension (name, mlp)
    local complete_name = M.metalua_extension_prefix .. name
    local extend_func = require(complete_name)
    if not mlp.extensions[complete_name] then
        local ast = extend_func(mlp)
        mlp.extensions[complete_name] = extend_func
        return ast
    end
end

return true
