local modules_path, sources_path = ...

-- Remove potentially unsafe/unwated functions.
function dofile(...)
    error("File operations are not supported in patchling.")
end
function loadfile(...)
    error("File operations are not supported in patchling.")
end
package.loaders[3] = nil -- this interferes with metalua

-- Setup loader paths
local pkg_root = modules_path.."/share/lua/5.1/"
local pkg_source = sources_path.."/"
local path_base = pkg_root.."?.lua;"..pkg_root.."?/init.lua"
local path_sources = pkg_source.."?.lua;"..pkg_source.."?/init.lua"
package.cpath = ""
package.path = path_base..";"..path_sources

-- Bootstrap Metalua
local loader = require "patchling_private.load_metalua"
loader(modules_path, sources_path)

-- TODO Test
local encountered = {}
local function dump(head, table)
    if not encountered[table] then
        encountered[table] = true
        for k, v in pairs(table) do
            print(head.."."..tostring(k), v)
            if type(k) == "string" and type(v) == "table" then
                dump(head.."."..k, v)
            end
        end
    end
end
dump("_G", _G)

require("ne")