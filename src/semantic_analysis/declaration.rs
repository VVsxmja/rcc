use inkwell::{types::BasicType, values::BasicValueEnum};

use crate::{
    semantic_analysis::remove_dead_code,
    syntax_analysis::{
        declaration::Declaration, expression::Expression,
        parameter_definition::ParameterDefinition, types::Type,
    },
};

use super::IR;

impl<'ctx> IR<'ctx> {
    pub(super) fn analysis_declaration(&mut self, decl: Declaration) -> anyhow::Result<()> {
        match decl {
            Declaration::Function(ret_type, name, params, body) => {
                let param_types: Vec<_> = params
                    .iter()
                    .map(|ParameterDefinition(param_type, _)| match param_type {
                        Type::Int => self.context.i32_type().as_basic_type_enum(),
                        Type::Void => unreachable!(),
                    })
                    .collect();
                let fn_type = {
                    let param_types: Box<[_]> =
                        param_types.iter().map(|t| t.to_owned().into()).collect();
                    match ret_type {
                        Type::Int => self.context.i32_type().fn_type(&param_types, false),
                        Type::Void => self.context.void_type().fn_type(&param_types, false),
                    }
                };
                if self.builder.get_insert_block().is_some() {
                    anyhow::bail!("Local function in not implemented");
                } else {
                    let func = match self.module.get_function(&name) {
                        None => self.module.add_function(&name, fn_type, None),
                        Some(prev_def) => {
                            anyhow::ensure!(
                                prev_def.get_type() == fn_type,
                                "Function {name} redefined."
                            );
                            anyhow::ensure!(
                                prev_def.get_basic_blocks().is_empty(),
                                "Function {name} redefined."
                            );
                            prev_def
                        }
                    };
                    if let Some(body) = body {
                        let entry_block = self.context.append_basic_block(func, "entry");
                        let return_block = self.context.append_basic_block(func, "return");
                        match fn_type.get_return_type() {
                            None => {
                                self.return_value = None;
                                self.builder.position_at_end(return_block);
                                self.builder.build_return(None)?;
                            }
                            Some(ret_type) => {
                                self.builder.position_at_end(entry_block);
                                let alloca = self
                                    .builder
                                    .build_alloca(self.context.i32_type(), "return_value")?;
                                self.return_value = Some(alloca);
                                self.builder.position_at_end(return_block);
                                let return_value = self.builder.build_load(
                                    ret_type,
                                    alloca,
                                    "load_return_value",
                                )?;
                                self.builder.build_return(Some(&return_value))?;
                            }
                        }
                        self.builder.position_at_end(entry_block);
                        let args = func.get_params();
                        debug_assert!(
                            args.len() == params.len(),
                            "length of parameter list changed"
                        );
                        self.local_variables.clear();
                        for (i, arg) in args.into_iter().enumerate() {
                            let ParameterDefinition(_, param_name) = &params[i];
                            let alloca = self.builder.build_alloca(arg.get_type(), param_name)?;
                            let None = self
                                .local_variables
                                .insert(param_name.to_owned(), alloca)
                            else {
                                anyhow::bail!("Duplicate parameter name: {param_name}");
                            };
                            self.builder.build_store(alloca, arg)?;
                        }
                        self.analysis_block(body)?;
                        for bb in func.get_basic_block_iter() {
                            remove_dead_code(&bb);
                        }
                        anyhow::ensure!(func.verify(cfg!(debug_assertions)), "Illegal function");
                        self.builder.clear_insertion_position();
                    }
                }
            }
            Declaration::Variable(var_type, name, value) => {
                let var_type = match var_type {
                    Type::Int => self.context.i32_type().as_basic_type_enum(),
                    Type::Void => anyhow::bail!("Variable cannot be of void type"),
                };
                if self.local_variables.contains_key(&name) {
                    anyhow::bail!("Redifined {name}");
                }
                if let Some(bb) = self.builder.get_insert_block() {
                    let this_func = bb.get_parent().unwrap();
                    let entry = this_func.get_first_basic_block().unwrap();
                    let alloca = {
                        let builder = self.context.create_builder();
                        builder.position_at_end(entry);
                        builder.build_alloca(var_type, &name)?
                    };
                    self.local_variables.insert(name, alloca);
                    if let Some(value) = value {
                        let Some(value) = self.analysis_expression(value)? else {
                            anyhow::bail!("Operand cannot be void")
                        };
                        match value {
                            BasicValueEnum::IntValue(value) => {
                                self.builder.build_store(alloca, value)?;
                            }
                            _ => anyhow::bail!("Unimplemented type of operand"),
                        }
                    }
                } else {
                    self.module.add_global(var_type, None, &name);
                    if let Some(value) = value {
                        let Expression::Constant(_value) = value else {
                            anyhow::bail!("Global variable can only be initialized with constant");
                        };
                        anyhow::bail!("Global variable initialization unimplemented");
                    }
                }
            }
        }
        Ok(())
    }
}
