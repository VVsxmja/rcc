use std::path::PathBuf;

use anyhow::anyhow;
use inkwell::{
    context::Context,
    memory_buffer::MemoryBuffer,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
    OptimizationLevel,
};

pub(crate) fn generate_object_file(bitcode: MemoryBuffer, output_path: PathBuf, file_type: FileType) -> anyhow::Result<()> {
    let triple = TargetMachine::get_default_triple();
    Target::initialize_all(&InitializationConfig::default());
    let target =
        Target::from_triple(&triple).map_err(|llvm_string| anyhow!(llvm_string.to_string()))?;
    let Some(target_machine) = target.create_target_machine(
        &triple,
        "generic",
        "",
        OptimizationLevel::None,
        RelocMode::Static,
        CodeModel::Default,
    ) else {
        anyhow::bail!("Failed to create target machine");
    };
    let context = Context::create();
    let module = context
        .create_module_from_ir(bitcode)
        .map_err(|llvm_string| anyhow!(llvm_string.to_string()))?;
    target_machine
        .write_to_file(&module, file_type, &output_path)
        .map_err(|llvm_string| anyhow!(llvm_string.to_string()))?;
    Ok(())
}
