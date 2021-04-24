---------------------------------------------------------------------------
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
--     AuroraAmissa - Patches to compile via source rather than bytecode
--
--------------------------------------------------------------------------------

--------------------------------------------------------------------------------
--
-- Convert between various code representation formats. Atomic
-- converters are written in extenso, others are composed automatically
-- by chaining the atomic ones together in a closure.
--
-- Supported formats are:
--
-- * srcfile:    the name of a file containing sources.
-- * src:        these sources as a single string.
-- * lexstream:  a stream of lexemes.
-- * ast:        an abstract syntax tree.
-- * lua:        a normal Lua string (requires Luajit)
-- * function:   an executable lua function in RAM.
--
--------------------------------------------------------------------------------

require 'checks'

local M = { }

--------------------------------------------------------------------------------
-- Order of the transformations. if 'a' is on the left of 'b', then a 'a' can
-- be transformed into a 'b' (but not the other way around).
-- M.sequence goes for numbers to format names, M.order goes from format
-- names to numbers.
--------------------------------------------------------------------------------
M.sequence = { 'srcfile', 'src', 'lexstream', 'ast', 'lua', 'function' }

local arg_types = {
    srcfile = { 'string', '?string' },
    src = { 'string', '?string' },
    lexstream = { 'lexer.stream', '?string' },
    ast = { 'table', '?string' },
    lua = { 'string', '?string' },
}

if false then
    -- if defined, runs on every newly-generated AST
    function M.check_ast(ast)
        local function rec(x, n, parent)
            if not x.lineinfo and parent.lineinfo then
                local pp = require 'metalua.pprint'
                pp.printf("WARNING: Missing lineinfo in child #%s `%s{...} of node at %s",
                        n, x.tag or '', tostring(parent.lineinfo))
            end
            for i, child in ipairs(x) do
                if type(child) == 'table' then
                    rec(child, i, x)
                end
            end
        end
        rec(ast, -1, { })
    end
end

M.order = { };
for a, b in pairs(M.sequence) do
    M.order[b] = a
end

local CONV = { } -- conversion metatable __index

function CONV:srcfile_to_src(x, name)
    checks('metalua.compiler', 'string', '?string')
    name = name or '@' .. x
    local f, msg = io.open(x, 'rb')
    if not f then
        error(msg)
    end
    local r, msg = f:read '*a'
    if not r then
        error("Cannot read file '" .. x .. "': " .. msg)
    end
    f:close()
    return r, name
end

function CONV:src_to_lexstream(src, name)
    checks('metalua.compiler', 'string', '?string')
    local r = self.parser.lexer:newstream(src, name)
    return r, name
end

function CONV:lexstream_to_ast(lx, name)
    checks('metalua.compiler', 'lexer.stream', '?string')
    local r = self.parser.chunk(lx)
    r.source = name
    if M.check_ast then
        M.check_ast(r)
    end
    return r, name
end

local ast_to_lua_impl = require 'metalua.compiler.ast_to_src'
function CONV:ast_to_lua(ast, name)
    return ast_to_lua_impl(ast), name
end

local loadstring = loadstring -- save this to avoid later loops
function CONV:lua_to_function(lua, name)
    checks('metalua.compiler', 'string', '?string')

    local load, err = loadstring(lua, name);
    if not load then
        return nil, nil, err
    else
        return load, name
    end
end

-- Create all sensible combinations
for i = 1, #M.sequence do
    local src = M.sequence[i]
    for j = i + 2, #M.sequence do
        local dst = M.sequence[j]
        local dst_name = src .. "_to_" .. dst
        local my_arg_types = arg_types[src]
        local functions = { }
        for k = i, j - 1 do
            local name = M.sequence[k] .. "_to_" .. M.sequence[k + 1]
            local f = assert(CONV[name], name .. " does not exist!")
            table.insert(functions, f)
        end
        CONV[dst_name] = function(self, a, b)
            checks('metalua.compiler', unpack(my_arg_types))
            local e
            for _, f in ipairs(functions) do
                a, b, e = f(self, a, b)
                if not a then
                    return nil, e
                end
            end
            return a, b
        end
    end
end


--------------------------------------------------------------------------------
-- This one goes in the "wrong" direction, cannot be composed.
--------------------------------------------------------------------------------
function CONV:function_to_bytecode(...)
    return string.dump(...)
end

CONV.ast_to_src = CONV.ast_to_lua

local MT = { __index = CONV, __type = 'metalua.compiler' }

function M.new()
    local parser = require 'metalua.compiler.parser' .new()
    local self = { parser = parser }
    setmetatable(self, MT)
    return self
end

return M