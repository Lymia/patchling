#!/usr/bin/env bash

rm -rf lua_modules/*
scripts/make_lua_root.sh

# Remove native dependencies
./luarocks remove --force luafilesystem
./luarocks remove --force luaposix
./luarocks remove --force readline
./luarocks remove --force checks

# Remove documentation
rm -rfv lua_modules/lib/luarocks/rocks-*/*/*/doc

# Remove luajit's bytecode module
rm -rfv lua_modules/share/lua/5.1/metalua/compiler/bytecode
