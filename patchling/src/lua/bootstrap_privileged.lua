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
package.preload["thread.exdata"] = nil
package.preload["thread.exdata2"] = nil

-- Creates some non-public (but safe) extension functions for bootstrap_privileged
do
    local package = package
    function require_alias(target, what)
        if not package.loaded[target] then
            package.loaded[target] = require(what)
        end
    end
end

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
        if not source then
            source = "<string>"
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
require "patchling_private.traceback"

-- Remove unsafe functions that are used by privileged modules.
debug = nil
package.loaded.debug = nil
io = nil
package.loaded.io = nil

-- Recreate debug.traceback
debug = {}
debug.traceback = require "patchling_private.traceback"
package.loaded.debug = debug

-- Create shims for some missing functions
os = {}
os.setlocale = function(...) end
package.loaded.os = os

-- Seal the package table
package.loaded.package = nil
package = nil