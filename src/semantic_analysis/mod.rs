mod declaration;
mod expression;
mod statement;

use std::collections::HashMap;

use anyhow::anyhow;
use inkwell::{
    builder::Builder, context::Context, memory_buffer::MemoryBuffer, module::Module,
    values::PointerValue,
};

use crate::syntax_analysis::{
    block_statement::{Block, BlockInner},
    translation_unit::TranslationUnit,
};

struct IR<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    local_variables: HashMap<String, PointerValue<'ctx>>,
    return_value: Option<PointerValue<'ctx>>,
}

impl<'ctx> IR<'ctx> {
    fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("main");
        let builder = context.create_builder();
        IR {
            builder,
            context,
            module,
            local_variables: HashMap::new(),
            return_value: None,
        }
    }

    fn analysis_block(&mut self, Block(block_inner): Block) -> anyhow::Result<()> {
        for inner in block_inner {
            match inner {
                BlockInner::Declaration(decl) => {
                    self.analysis_declaration(decl)?;
                }
                BlockInner::Statement(stmt) => {
                    self.analysis_statement(stmt)?;
                }
            }
        }
        Ok(())
    }

    fn analysis_translation_unit(&mut self, unit: TranslationUnit) -> anyhow::Result<()> {
        let TranslationUnit(decls) = unit;
        for decl in decls {
            self.analysis_declaration(decl)?;
        }
        Ok(())
    }
}

pub(crate) fn analysis(unit: TranslationUnit) -> anyhow::Result<MemoryBuffer> {
    let context = Context::create();
    let mut ir = IR::new(&context);
    ir.analysis_translation_unit(unit)?;
    ir.module
        .verify()
        .map_err(|llvm_string| anyhow!(llvm_string.to_string()))?;
    Ok(ir.module.write_bitcode_to_memory())
}

pub(crate) fn bitcode_to_string(bitcode: MemoryBuffer) -> anyhow::Result<String> {
    let context = Context::create();
    let module = context
        .create_module_from_ir(bitcode)
        .map_err(|llvm_string| anyhow!(llvm_string.to_string()))?;
    Ok(module.print_to_string().to_string())
}
