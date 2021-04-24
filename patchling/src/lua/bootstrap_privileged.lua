-- NOTE: This is privileged code and has access to functions that should not be available to user code.
--       Take extra care when editing this file.

local modules_path, mod_paths = ...

-- Remove unsafe functions.
function dofile(...)
    return nil, "File operations are not supported."
end
function loadfile(...)
    return nil, "File operations are not supported."
end

-- Whitelist `package.preload` entries.
local preload_whitelist = {
    ["table.isempty"] = true,
    ["table.isarray"] = true,
    ["table.nkeys"] = true,
    ["table.clone"] = true,
}
for k, v in pairs(package.preload) do
    if not preload_whitelist[k] then
        package.preload[v] = nil
    end
end
table.isempty = require "table.isempty"
table.isarray = require "table.isarray"
table.nkeys = require "table.nkeys"
table.clone = require "table.clone"

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

-- Setup bootstrap loader paths
local pkg_root = modules_path.."/share/lua/5.1/"
package.cpath = ""
package.path = pkg_root.."?.lua;"..pkg_root.."?/init.lua"

-- Pass package paths to our loader
package.modules_path = modules_path
package.mod_paths = mod_paths

-- Loads privileged modules.
package.loaded["checks"] = require "patchling_private.privileged.checks"
require "patchling_private.privileged.traceback"
package.loaded["metalua.loader"] = require "patchling_private.privileged.metalua_loader"

-- Remove unsafe functions that are used by privileged modules.
debug = nil
package.loaded.debug = nil
io = nil
package.loaded.io = nil

-- Recreate debug.traceback
debug = {}
debug.traceback = (require "patchling_private.privileged.traceback").traceback
package.loaded.debug = debug

-- Create shims for some missing functions
os = {}
os.setlocale = function(...) end
package.loaded.os = os

-- Seal the package table
package.loaded.package = nil
package = nil