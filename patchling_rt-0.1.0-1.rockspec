package = "patchling_rt"
version = "0.1.0-1"
source = {
   url = "*** please add URL for source tarball, zip or repository here ***"
}
description = {
   homepage = "*** please enter a project homepage ***",
   license = "*** please specify a license ***"
}
build = {
   type = "builtin",
   modules = {
      ["metalua.checks"] = "metalua/checks.lua",
      ["metalua.doc.manual.mkhtml"] = "metalua/doc/manual/mkhtml.lua",
      ["metalua.metalua"] = "metalua/metalua.lua",
      ["metalua.metalua.bytecode"] = "metalua/metalua/bytecode.lua",
      ["metalua.metalua.compiler"] = "metalua/metalua/compiler.lua",
      ["metalua.metalua.compiler.bytecode"] = "metalua/metalua/compiler/bytecode.lua",
      ["metalua.metalua.compiler.bytecode.compile"] = "metalua/metalua/compiler/bytecode/compile.lua",
      ["metalua.metalua.compiler.bytecode.lcode"] = "metalua/metalua/compiler/bytecode/lcode.lua",
      ["metalua.metalua.compiler.bytecode.ldump"] = "metalua/metalua/compiler/bytecode/ldump.lua",
      ["metalua.metalua.compiler.bytecode.lopcodes"] = "metalua/metalua/compiler/bytecode/lopcodes.lua",
      ["metalua.metalua.compiler.globals"] = "metalua/metalua/compiler/globals.lua",
      ["metalua.metalua.compiler.parser"] = "metalua/metalua/compiler/parser.lua",
      ["metalua.metalua.compiler.parser.annot.generator"] = "metalua/metalua/compiler/parser/annot/generator.lua",
      ["metalua.metalua.compiler.parser.annot.grammar"] = "metalua/metalua/compiler/parser/annot/grammar.lua",
      ["metalua.metalua.compiler.parser.common"] = "metalua/metalua/compiler/parser/common.lua",
      ["metalua.metalua.compiler.parser.expr"] = "metalua/metalua/compiler/parser/expr.lua",
      ["metalua.metalua.compiler.parser.ext"] = "metalua/metalua/compiler/parser/ext.lua",
      ["metalua.metalua.compiler.parser.lexer"] = "metalua/metalua/compiler/parser/lexer.lua",
      ["metalua.metalua.compiler.parser.meta"] = "metalua/metalua/compiler/parser/meta.lua",
      ["metalua.metalua.compiler.parser.misc"] = "metalua/metalua/compiler/parser/misc.lua",
      ["metalua.metalua.compiler.parser.stat"] = "metalua/metalua/compiler/parser/stat.lua",
      ["metalua.metalua.compiler.parser.table"] = "metalua/metalua/compiler/parser/table.lua",
      ["metalua.metalua.grammar.generator"] = "metalua/metalua/grammar/generator.lua",
      ["metalua.metalua.grammar.lexer"] = "metalua/metalua/grammar/lexer.lua",
      ["metalua.metalua.loader"] = "metalua/metalua/loader.lua",
      ["metalua.metalua.pprint"] = "metalua/metalua/pprint.lua"
   }
}
