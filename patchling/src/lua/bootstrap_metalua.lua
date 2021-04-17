-- Bootstraps our modified metalua compile process for LuaJIT
require_alias("metalua.compiler.ast_to_src", "patchling_private.ast_to_src_precompiled")
require_alias("metalua.compiler", "patchling_private.metalua_compiler")
require_alias("metalua.compiler.globals", "patchling_private.metalua_globals")

-- Removes nonpublic (but safe) functions
require_alias = nil
