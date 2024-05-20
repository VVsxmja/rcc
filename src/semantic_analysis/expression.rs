use either::Either;
use inkwell::values::{BasicValue, BasicValueEnum};

use crate::{
    lexical_analysis::Constant,
    syntax_analysis::expression::{BinaryOperator, Expression, PrefixUnaryOperator, RefOrCall},
};

use super::IR;

impl<'ctx> IR<'ctx> {
    pub(super) fn analysis_expression(
        &self,
        expr: Expression,
    ) -> anyhow::Result<Option<BasicValueEnum<'ctx>>> {
        tracing::trace!("Emit {expr:?}");
        match expr {
            Expression::Evaluate(expr) => match self.analysis_expression(*expr)? {
                None => Ok(None),
                Some(value) => match value {
                    BasicValueEnum::PointerValue(ptr) => {
                        let value =
                            self.builder
                                .build_load(self.context.i32_type(), ptr, "load")?;
                        Ok(Some(value))
                    }
                    _ => Ok(Some(value)),
                },
            },
            Expression::Binary(lhs, bin_op, rhs) => {
                if let BinaryOperator::Assign = bin_op {
                    let Expression::RefOrCall(RefOrCall::Variable(lhs)) = *lhs else {
                        anyhow::bail!("Only variable can be assigned");
                    };
                    let Some(rhs) = self.analysis_expression(*rhs)? else {
                        anyhow::bail!("Cannot assign void to variable");
                    };
                    let Some(lhs) = self.local_variables.get(&lhs) else {
                        anyhow::bail!("Undefined variable: {lhs}");
                    };
                    tracing::trace!("assigning {rhs} to {lhs}");
                    // anyhow::ensure!(
                    //     lhs.as_basic_value_enum().get_type() == rhs.get_type(),
                    //     "Cannot assign a value of different type"
                    // );
                    match bin_op {
                        BinaryOperator::Assign => {
                            self.builder.build_store(lhs.to_owned(), rhs)?;
                            Ok(Some(lhs.as_basic_value_enum()))
                        }
                        _ => unreachable!(),
                    }
                } else {
                    let Some(lhs) = self.analysis_expression(*lhs)? else {
                        anyhow::bail!("Operand cannot be void");
                    };
                    let Some(rhs) = self.analysis_expression(*rhs)? else {
                        anyhow::bail!("Operand cannot be void");
                    };
                    tracing::trace!("doing {} {:?} {}", lhs, bin_op, rhs);
                    use inkwell::IntPredicate;
                    let result = match bin_op {
                        BinaryOperator::Multiply => match lhs {
                            BasicValueEnum::IntValue(lhs) => match rhs {
                                BasicValueEnum::IntValue(rhs) => self
                                    .builder
                                    .build_int_mul(lhs, rhs, "mul")?
                                    .as_basic_value_enum(),
                                _ => anyhow::bail!("Operand of unimplemented type"),
                            },
                            _ => anyhow::bail!("Operand of unimplemented type"),
                        },
                        BinaryOperator::Divide => match lhs {
                            BasicValueEnum::IntValue(lhs) => match rhs {
                                BasicValueEnum::IntValue(rhs) => self
                                    .builder
                                    .build_int_signed_div(lhs, rhs, "signed_div")?
                                    .as_basic_value_enum(),
                                _ => anyhow::bail!("Operand of unimplemented type"),
                            },
                            _ => anyhow::bail!("Operand of unimplemented type"),
                        },
                        BinaryOperator::Plus => match lhs {
                            BasicValueEnum::IntValue(lhs) => match rhs {
                                BasicValueEnum::IntValue(rhs) => self
                                    .builder
                                    .build_int_add(lhs, rhs, "add")?
                                    .as_basic_value_enum(),
                                _ => anyhow::bail!("Operand of unimplemented type"),
                            },
                            _ => anyhow::bail!("Operand of unimplemented type"),
                        },
                        BinaryOperator::Minus => match lhs {
                            BasicValueEnum::IntValue(lhs) => match rhs {
                                BasicValueEnum::IntValue(rhs) => self
                                    .builder
                                    .build_int_sub(lhs, rhs, "sub")?
                                    .as_basic_value_enum(),
                                _ => anyhow::bail!("Operand of unimplemented type"),
                            },
                            _ => anyhow::bail!("Operand of unimplemented type"),
                        },
                        BinaryOperator::Less => match lhs {
                            BasicValueEnum::IntValue(lhs) => match rhs {
                                BasicValueEnum::IntValue(rhs) => self
                                    .builder
                                    .build_int_compare(IntPredicate::SLT, lhs, rhs, "slt")?
                                    .as_basic_value_enum(),
                                _ => anyhow::bail!("Operand of unimplemented type"),
                            },
                            _ => anyhow::bail!("Operand of unimplemented type"),
                        },
                        BinaryOperator::Greater => match lhs {
                            BasicValueEnum::IntValue(lhs) => match rhs {
                                BasicValueEnum::IntValue(rhs) => self
                                    .builder
                                    .build_int_compare(IntPredicate::SGT, lhs, rhs, "sgt")?
                                    .as_basic_value_enum(),
                                _ => anyhow::bail!("Operand of unimplemented type"),
                            },
                            _ => anyhow::bail!("Operand of unimplemented type"),
                        },
                        BinaryOperator::LessEqual => match lhs {
                            BasicValueEnum::IntValue(lhs) => match rhs {
                                BasicValueEnum::IntValue(rhs) => self
                                    .builder
                                    .build_int_compare(IntPredicate::SLE, lhs, rhs, "sle")?
                                    .as_basic_value_enum(),
                                _ => anyhow::bail!("Operand of unimplemented type"),
                            },
                            _ => anyhow::bail!("Operand of unimplemented type"),
                        },
                        BinaryOperator::GreaterEqual => match lhs {
                            BasicValueEnum::IntValue(lhs) => match rhs {
                                BasicValueEnum::IntValue(rhs) => self
                                    .builder
                                    .build_int_compare(IntPredicate::SGE, lhs, rhs, "sge")?
                                    .as_basic_value_enum(),
                                _ => anyhow::bail!("Operand of unimplemented type"),
                            },
                            _ => anyhow::bail!("Operand of unimplemented type"),
                        },
                        BinaryOperator::Equal => match lhs {
                            BasicValueEnum::IntValue(lhs) => match rhs {
                                BasicValueEnum::IntValue(rhs) => self
                                    .builder
                                    .build_int_compare(IntPredicate::EQ, lhs, rhs, "eq")?
                                    .as_basic_value_enum(),
                                _ => anyhow::bail!("Operand of unimplemented type"),
                            },
                            _ => anyhow::bail!("Operand of unimplemented type"),
                        },
                        BinaryOperator::NotEqual => match lhs {
                            BasicValueEnum::IntValue(lhs) => match rhs {
                                BasicValueEnum::IntValue(rhs) => self
                                    .builder
                                    .build_int_compare(IntPredicate::NE, lhs, rhs, "ne")?
                                    .as_basic_value_enum(),
                                _ => anyhow::bail!("Operand of unimplemented type"),
                            },
                            _ => anyhow::bail!("Operand of unimplemented type"),
                        },
                        BinaryOperator::Comma => {
                            anyhow::bail!("Comma expression not implemented")
                        }
                        BinaryOperator::Assign => unreachable!(),
                    };
                    Ok(Some(result))
                }
            }
            Expression::PrefixUnary(op, operand) => {
                let Some(operand) = self.analysis_expression(*operand)? else {
                    anyhow::bail!("Operand cannot be void");
                };
                use inkwell::IntPredicate;
                let result = match op {
                    PrefixUnaryOperator::Minus => match operand {
                        BasicValueEnum::IntValue(operand) => Some(
                            self.builder
                                .build_int_neg(operand, "neg")?
                                .as_basic_value_enum(),
                        ),
                        _ => anyhow::bail!("Uninplemented operand type"),
                    },
                    PrefixUnaryOperator::Plus => None,
                    PrefixUnaryOperator::Not => match operand {
                        BasicValueEnum::IntValue(operand) => Some(
                            self.builder
                                .build_int_compare(
                                    IntPredicate::EQ,
                                    operand,
                                    self.context.i32_type().const_zero(),
                                    "not",
                                )?
                                .as_basic_value_enum(),
                        ),
                        _ => anyhow::bail!("Uninplemented operand type"),
                    },
                };
                Ok(result)
            }
            Expression::Constant(value) => match value {
                Constant::Int(value) => Ok(Some(
                    self.context
                        .i32_type()
                        .const_int(value as u64, true)
                        .as_basic_value_enum(),
                )),
            },
            Expression::Paren(expr) => self.analysis_expression(*expr),
            Expression::RefOrCall(r) => match r {
                RefOrCall::Variable(var) => {
                    if let Some(local) = self.local_variables.get(&var) {
                        Ok(Some(local.as_basic_value_enum()))
                    } else if let Some(global) = self.module.get_global(&var) {
                        let global = global.as_pointer_value();
                        Ok(Some(global.as_basic_value_enum()))
                    } else {
                        anyhow::bail!("Undefined variable: {var}");
                    }
                }
                RefOrCall::FunctionCall(func, args) => {
                    let Some(func) = self.module.get_function(&func) else {
                        anyhow::bail!("Undefined function: {func}");
                    };
                    let params_len = func.get_type().get_param_types().len();
                    if args.len() != params_len {
                        anyhow::bail!("Expected {} arguments, found {}", params_len, args.len());
                    }
                    let mut parsed_args = Vec::new();
                    for arg in args {
                        let Some(arg) = self.analysis_expression(arg)? else {
                            anyhow::bail!("Argument cannot be void");
                        };
                        match arg {
                            BasicValueEnum::IntValue(arg) => parsed_args.push(arg.into()),
                            _ => anyhow::bail!("Invalid argument type"),
                        }
                    }
                    let callsite = self.builder.build_call(func, &parsed_args, "call")?;
                    match callsite.try_as_basic_value() {
                        Either::Left(value) => Ok(Some(value)),
                        Either::Right(_) => Ok(None),
                    }
                }
            },
        }
    }
}
