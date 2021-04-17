-- NOTE: This is privileged code and has access to functions that should not be available to user code.
--       Take extra care when editing this file.

local debug_getinfo = debug.getinfo

-- TODO: Finish traceback, implement lineinfo for ast_to_src

local function traceback(...)
    local frame = 1
    while true do
        local info = debug_getinfo(frame, "Snl")
        if not info then
            break
        end

        print(frame, require("metalua.pprint").tostring(info))

        frame = frame + 1
    end

    return ""
end

return traceback