local mlc = require "metalua.compiler"
local luasrcdiet = require "luasrcdiet.init"

local function compile_and_minify(source, name)
    local lua_source = mlc.new():src_to_lua(source, name);
    return luasrcdiet.optimize(luasrcdiet.MAXIMUM_OPTS, lua_source)
end

return compile_and_minify