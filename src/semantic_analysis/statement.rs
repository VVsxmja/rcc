use inkwell::values::{BasicValue, BasicValueEnum};

use crate::syntax_analysis::statement::{JumpStatement, Statement};

use super::IR;

impl<'ctx> IR<'ctx> {
    pub(super) fn analysis_statement(&mut self, stmt: Statement) -> anyhow::Result<()> {
        match stmt {
            Statement::Block(block) => {
                self.analysis_block(block)?;
            }
            Statement::Empty => (),
            Statement::Expression(expr) => {
                self.analysis_expression(expr)?;
            }
            Statement::If(condition, true_stmt, false_stmt) => {
                tracing::trace!("If {condition:?}");
                let Some(condition) = self.analysis_expression(condition)? else {
                    anyhow::bail!("Condition cannot be void");
                };
                let condition = match condition {
                    BasicValueEnum::IntValue(condition) => {
                        use inkwell::IntPredicate;
                        let condition = self.builder.build_int_cast(
                            condition,
                            self.context.bool_type(),
                            "to_bool",
                        )?;
                        self.builder.build_int_compare(
                            IntPredicate::NE,
                            condition,
                            self.context.bool_type().const_zero(),
                            "cond",
                        )?
                    }
                    _ => anyhow::bail!("Unimplemented condition type"),
                };
                let this_block = self.builder.get_insert_block().unwrap();
                let _this_func = this_block.get_parent().unwrap();
                let end_block = self.context.insert_basic_block_after(this_block, "if_end");
                let then_block = self.context.insert_basic_block_after(this_block, "if_then");
                if let Some(false_stmt) = false_stmt {
                    let else_block = self.context.insert_basic_block_after(this_block, "if_else");
                    self.builder
                        .build_conditional_branch(condition, then_block, else_block)?;
                    self.builder.position_at_end(else_block);
                    self.analysis_statement(*false_stmt)?;
                    self.builder.build_unconditional_branch(end_block)?;
                } else {
                    self.builder
                        .build_conditional_branch(condition, then_block, end_block)?;
                }
                self.builder.position_at_end(then_block);
                self.analysis_statement(*true_stmt)?;
                self.builder.build_unconditional_branch(end_block)?;
                self.builder.position_at_end(end_block);
            }
            Statement::While(condition, body) => {
                tracing::trace!("While {condition:?}");
                let this_block = self.builder.get_insert_block().unwrap();
                let _this_func = this_block.get_parent().unwrap();
                let body_block = self
                    .context
                    .insert_basic_block_after(this_block, "while_body");
                let end_block = self
                    .context
                    .insert_basic_block_after(this_block, "while_end");
                let cond_block = self
                    .context
                    .insert_basic_block_after(this_block, "while_cond");
                self.builder.build_unconditional_branch(cond_block)?;
                self.builder.position_at_end(cond_block);
                let Some(condition) = self.analysis_expression(condition)? else {
                    anyhow::bail!("Condition cannot be void");
                };
                let condition = match condition {
                    BasicValueEnum::IntValue(condition) => {
                        use inkwell::IntPredicate;
                        let condition = self.builder.build_int_cast(
                            condition,
                            self.context.bool_type(),
                            "to_bool",
                        )?;
                        self.builder.build_int_compare(
                            IntPredicate::NE,
                            condition,
                            self.context.bool_type().const_zero(),
                            "cond",
                        )?
                    }
                    _ => anyhow::bail!("Condition of unimplemented type"),
                };
                self.builder
                    .build_conditional_branch(condition, body_block, end_block)?;
                self.builder.position_at_end(body_block);
                self.analysis_statement(*body)?;
                self.builder.build_unconditional_branch(cond_block)?;
                self.builder.position_at_end(end_block);
            }
            Statement::Jump(jump) => match jump {
                JumpStatement::Return(ret_value) => {
                    let ret_value = match ret_value {
                        Some(ret_value) => self
                            .analysis_expression(ret_value)?
                            .map(|value| value.as_basic_value_enum()),
                        None => None,
                    };
                    let this_func = self
                        .builder
                        .get_insert_block()
                        .unwrap()
                        .get_parent()
                        .unwrap();
                    let return_type = this_func.get_type().get_return_type();
                    match (ret_value, return_type) {
                        (Some(ret_value), Some(return_type)) => {
                            anyhow::ensure!(
                                ret_value.get_type() == return_type,
                                "Unexpected return type"
                            );
                            self.builder
                                .build_store(self.return_value.clone().unwrap(), ret_value)?;
                        }
                        (None, None) => {}
                        (None, Some(_)) => {
                            anyhow::bail!("This function must return non-void value.")
                        }
                        (Some(_), None) => anyhow::bail!("This function must return nothing."),
                    }
                    let return_block = this_func.get_last_basic_block().unwrap();
                    self.builder.build_unconditional_branch(return_block)?;
                }
            },
        }
        Ok(())
    }
}
