-- NOTE: This is privileged code and has access to functions that should not be available to user code.
--       Take extra care when editing this file.

local modules_path, sources_path = ...

-- Remove unsafe functions.
function dofile(...)
    return nil, "File operations are not supported."
end
function loadfile(...)
    return nil, "File operations are not supported."
end
package.preload.thread = nil

-- Create shims for some missing functions
os = {}
os.setlocale = function(...) end
package.loaded.os = os

-- Disable bytecode loading
do
    local error = error
    local l_load = load
    local string_find = string.find
    local string_sub = string.sub

    function load(chunk, source, mode, env)
        if mode ~= nil and string_find(mode, "b") then
            return nil, "Bytecode loading is not allowed."
        end
        if type(chunk) == "string" then
            if string_sub(chunk, 1, 3) == "\27LJ" then
                return nil, "Bytecode loading is not allowed."
            end
            if string_sub(chunk, 1, 4) == "\27Lua" then
                return nil, "Bytecode loading is not allowed."
            end
        end
        return l_load(chunk, source, "t", env)
    end
    loadstring = load
end

-- Remove .so loading capabilities
function package.loadlib(...)
    error("Shared library loading is disabled for safety reasons.")
end
package.loaders[3] = nil
package.loaders[4] = nil

-- Setup loader paths
local pkg_root = modules_path.."/share/lua/5.1/"
local pkg_source = sources_path.."/"
local path_base = pkg_root.."?.lua;"..pkg_root.."?/init.lua"
local path_sources = pkg_source.."?.lua;"..pkg_source.."?/init.lua"

package.cpath = ""
package.path = path_base..";"..path_sources

-- Loads privileged modules.
package.loaded["checks"] = require "patchling_private.checks"
package.loaded["metalua.loader"] = require "patchling_private.metalua_loader"

-- Remove unsafe functions that are used by privileged modules.
debug.getfenv = nil
debug.setfenv = nil
debug.gethook = nil
debug.debug = nil
debug.getregistry = nil
debug.getinfo = nil
debug.getlocal = nil
debug.setlocal = nil
debug.getupvalue = nil
debug.setupvalue = nil
debug.upvalueid = nil
debug.upvaluejoin = nil
debug.sethook = nil
debug.getmetatable = nil
debug.setmetatable = nil

io = nil
package.loaded.io = nil
