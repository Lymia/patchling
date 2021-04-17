#!/usr/bin/env bash

cd metalua || exit 1
../luarocks make metalua-parser-0.7.2-1.rockspec || exit 1
../luarocks make metalua-compiler-0.7.2-1.rockspec || exit 1
cd .. || exit 1

./luarocks make patchling_rt-0.1.0-1.rockspec || exit 1
