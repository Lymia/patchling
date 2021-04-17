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

-- Alternative implementation of checks() in Lua. Slower than
-- the C counterpart, but no compilation/porting concerns.

checkers = { }

local checkers = checkers
local debug_getlocal = debug.getlocal
local debug_getinfo = debug.getinfo
local error = error
local getmetatable = getmetatable
local ipairs = ipairs
local string_format = string.format
local string_gmatch = string.gmatch
local string_sub = string.sub
local type = type

local function check_one(expected, val)
    if type(val)==expected then return true end
    local mt = getmetatable(val)
    if mt and mt.__type==expected then return true end
    local f = checkers[expected]
    if f and f(val) then return true end
    return false
end

local function check_many(expected, val)
    if expected=='?' then return true
    elseif expected=='!' then return (val~=nil)
    elseif type(expected) ~= 'string' then
        error 'strings expected by checks()'
    elseif val==nil and string_sub(expected, 1, 1) == '?' then return true end
    for one in string_gmatch(expected, "[^|?]+") do
        if check_one(one, val) then return true end
    end
    return false
end

function checks(...)
    for i, arg in ipairs{...} do
        local name, val = debug_getlocal(2, i)
        local success = check_many(arg, val)
        if not success then
            local fname = debug_getinfo(2, 'n').name
            local fmt = "bad argument #%d to '%s' (%s expected, got %s)"
            local msg = string_format(fmt, i, fname or "?", arg, type(val))
            error(msg, 3)
        end
    end
end

return checks
