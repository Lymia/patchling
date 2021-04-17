local l_loadstring = loadstring

-- Bootstraps our modified metalua compile process for LuaJIT
package.loaded["metalua.compiler.ast_to_src"] = require "patchling_private.ast_to_src_precompiled"
package.loaded["metalua.compiler"] = require "patchling_private.metalua_compiler"
package.loaded["metalua.compiler.globals"] = require "patchling_private.metalua_globals"

-- TODO Test
local encountered = {}
local function dump(head, table)
    if not encountered[table] then
        encountered[table] = true

        local names = {}
        for k, _ in pairs(table) do
            _G.table.insert(names, k)
        end
        _G.table.sort(names)

        for _, k in ipairs(names) do
            local v = table[k]
            print(head.."."..tostring(k), v)
            if type(k) == "string" and type(v) == "table" then
                dump(head.."."..k, v)
            end
        end
    end
end
dump("_G", _G)
