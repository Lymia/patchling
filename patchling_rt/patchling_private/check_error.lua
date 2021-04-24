local function check_error(func, ...)
    local flag, result = xpcall(func, debug.traceback, ...)
    if not flag then
        return nil, result
    else
        return result, nil
    end
end

return check_error