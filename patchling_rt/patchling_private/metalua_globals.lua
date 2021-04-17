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
--     AuroraAmissa - Patches to better support LuaJIT functionality
--
--------------------------------------------------------------------------------

--*-lua-*-----------------------------------------------------------------------
-- Override Lua's default compilation functions, so that they support Metalua
-- rather than only plain Lua
--------------------------------------------------------------------------------

local mlc = require 'metalua.compiler'

local M = { }

-- Original versions
local original_lua_versions = {
    load = load,
    loadstring = loadstring,
}

local lua_load = load
local function load(chunk, name, mode, env)
    local text_mode = (not mode or mode:find("t"))
    if text_mode and type(chunk) == "string" then
        if chunk:sub(1, 3) == "\27LJ" or chunk:sub(1, 4) == "\27Lua" then
            return lua_load(chunk, name, mode, env)
        else
            local n = chunk:match '^#![^\n]*\n()'
            if n then
                chunk = chunk:sub(n, -1)
            end
            return mlc.new():src_to_function(chunk, name)
        end
    elseif text_mode then
        local acc = { }
        while true do
            local x = f()
            if not x then
                break
            end
            assert(type(x) == 'string', "function passed to load() must return strings")
            table.insert(acc, x)
        end
        return load(table.concat(acc), name, mode, env)
    else
        return lua_load(chunk, name, mode, env)
    end
end

function M.dostring(src)
    local f, msg = M.loadstring(src)
    if not f then
        error(msg)
    end
    return f()
end

M.load = load
M.loadstring = load

-- Export replacement functions as globals
for name, f in pairs(M) do
    _G[name] = f
end

-- To be done *after* exportation
M.lua = original_lua_versions

return M