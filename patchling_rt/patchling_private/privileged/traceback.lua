-- NOTE: This is privileged code and has access to functions that should not be available to user code.
--       Take extra care when editing this file.

local checks = checks
local coroutine_running = coroutine.running
local debug_getinfo = debug.getinfo
local string_gmatch = string.gmatch
local string_gsub = string.gsub
local string_sub = string.sub
local table_concat = table.concat
local table_insert = table.insert
local type = type

-- TODO: Finish traceback, implement lineinfo for ast_to_src

local module_names = {}

local function traceback(thread, message, level)
    if type(thread) ~= "thread" then
        message, level = thread, message
        thread = coroutine_running()
    end
    checks('?thread', '?', '?number')

    local frame = (level or 1) + 1
    local accum = { }
    if message then
        table_insert(accum, message)
        table_insert(accum, "\n\n")
    end
    table_insert(accum, "stack traceback:")
    while true do
        local info = thread and debug_getinfo(thread, frame, "Snl") or debug_getinfo(frame, "Snl")
        if not info then
            break
        end

        table_insert(accum, string.format("\n%4d: ", frame - 1))

        if string_sub(info.source, 1, 1) == "@" then
            local source = string_sub(info.source, 2)
            if module_names[source] then
                local names = module_names[source]
                table_insert(accum, names.mod_name)
                table_insert(accum, ":")
                table_insert(accum, names.module_name)
            elseif string_gmatch(source, "lua_modules/share/5.1") then
                table_insert(accum, "patchling:<bootstrap>/")
                local short_source = string_gsub(source, ".*/", "")
                table_insert(accum, short_source)
            else
                table_insert(accum, info.short_src)
            end
        else
            table_insert(accum, info.short_src)
        end
        if info.linedefined ~= -1 then
            table_insert(accum, ":")
            table_insert(accum, info.linedefined) -- TODO: lineinfo
        end
        table_insert(accum, ": ")

        if not info.namewhat or not info.name then
            table_insert(accum, "in unknown function")
        else
            table_insert(accum, "in function '")
            table_insert(accum, info.name)
            table_insert(accum, "'")
        end

        frame = frame + 1
    end
    return table_concat(accum)
end

local function register_file(file_name, module_name, mod_name)
    module_names[file_name] = {
        module_name = module_name,
        mod_name = mod_name,
    }
end

return {
    traceback = traceback,
    register_file = register_file,
}