use crate::Llvm;
use crate::scope::Scope;
use dorian_ast::ty::OperativeType;
use dorian_ast::val::{Arg, Bin, BinOp, Call, ContextValue, Expr, Float, Int, Lit, Num, SignedInt, Una, UnaOp, UnsignedInt, Value};
use inkwell::types::StringRadix;
use inkwell::values::{BasicValue, BasicValueEnum};
use inkwell::{FloatPredicate, IntPredicate};

impl Llvm {
    pub(crate) fn compile_value<'ctx>(
        &'ctx self,
        value: &Value,
        scope: &mut Scope<'ctx, '_>,
    ) -> BasicValueEnum<'ctx> {
        let value = match &value {
            Value::Context(x) => self.compile_context_value(x, scope),
            // TODO: remove hardcoded type
            Value::Expr(x) => self.compile_expr(x, OperativeType::UnsignedInt, scope),
            Value::Lit(x) => self.compile_lit(x, scope),
            Value::Call(x) => self.compile_call(x, scope),
        };

        scope.insert_value(value);

        value
    }

    fn compile_context_value<'ctx>(
        &'ctx self,
        value: &ContextValue,
        scope: &Scope<'ctx, '_>,
    ) -> BasicValueEnum<'ctx> {
        match value {
            ContextValue::Arg(arg) => self.compile_arg_value(arg, scope),
            ContextValue::Var(var) => scope
                .get_var(var)
                .expect("Failed to get variable from scope"),
        }
    }

    fn compile_arg_value<'ctx>(
        &'ctx self,
        value: &Arg,
        scope: &Scope<'ctx, '_>,
    ) -> BasicValueEnum<'ctx> {
        scope
            .llvm_function
            .get_nth_param(value.param_index)
            .expect("Failed to get function parameter")
    }

    fn compile_expr<'ctx>(
        &'ctx self,
        value: &Expr,
        ty: OperativeType,
        scope: &mut Scope<'ctx, '_>,
    ) -> BasicValueEnum<'ctx> {
        match value {
            Expr::Bin(x) => self.compile_bin(x, ty, scope),
            Expr::Una(x) => self.compile_una(x, ty, scope),
        }
    }

    fn compile_bin<'ctx>(
        &'ctx self,
        value: &Bin,
        ty: OperativeType,
        scope: &mut Scope<'ctx, '_>,
    ) -> BasicValueEnum<'ctx> {
        let lhs = self.compile_value(&value.lhs, scope);
        let rhs = self.compile_value(&value.rhs, scope);

        macro_rules! build {
            ($scope:ident.$func:ident($(=> $ex0:expr, )? $lhs:expr, $rhs:expr $(, $extra:expr)?)) => {
                $scope.builder.$func($($ex0, )? $lhs, $rhs $(, $extra)?, "").unwrap().as_basic_value_enum()
            };
        }

        macro_rules! build_int {
            ($scope:ident.$func:ident($(=> $ex0:expr, )? $lhs:expr, $rhs:expr $(, $extra:expr)?)) => {
                build!($scope.$func($($ex0, )? $lhs.into_int_value(), $rhs.into_int_value() $(, $extra)?))
            };
        }

        macro_rules! build_float {
            ($scope:ident.$func:ident($(=> $ex0:expr, )? $lhs:expr, $rhs:expr $(, $extra:expr)?)) => {
                build!($scope.$func($($ex0, )? $lhs.into_float_value(), $rhs.into_float_value() $(, $extra)?))
            };
        }

        match value.op {
            BinOp::Add => match ty {
                OperativeType::SignedInt | OperativeType::UnsignedInt => {
                    build_int! { scope.build_int_add(lhs, rhs) }
                }
                OperativeType::Float => build_float! { scope.build_float_add(lhs, rhs) },
                _ => panic!("Unsupported type for Add operation: {:?}", ty),
            },
            BinOp::Sub => match ty {
                OperativeType::SignedInt | OperativeType::UnsignedInt => {
                    build_int! { scope.build_int_sub(lhs, rhs) }
                }
                OperativeType::Float => build_float! { scope.build_float_sub(lhs, rhs) },
                _ => panic!("Unsupported type for Sub operation: {:?}", ty),
            },
            BinOp::Mul => match ty {
                OperativeType::SignedInt | OperativeType::UnsignedInt => {
                    build_int! { scope.build_int_mul(lhs, rhs) }
                }
                OperativeType::Float => build_float! { scope.build_float_mul(lhs, rhs) },
                _ => panic!("Unsupported type for Mul operation: {:?}", ty),
            },
            BinOp::Div => match ty {
                OperativeType::SignedInt => build_int! { scope.build_int_signed_div(lhs, rhs) },
                OperativeType::UnsignedInt => build_int! { scope.build_int_unsigned_div(lhs, rhs) },
                OperativeType::Float => build_float! { scope.build_float_div(lhs, rhs) },
                _ => panic!("Unsupported type for Div operation: {:?}", ty),
            },
            BinOp::Rem => match ty {
                OperativeType::SignedInt => build_int! { scope.build_int_signed_rem(lhs, rhs) },
                OperativeType::UnsignedInt => build_int! { scope.build_int_unsigned_rem(lhs, rhs) },
                OperativeType::Float => build_float! { scope.build_float_rem(lhs, rhs) },
                _ => panic!("Unsupported type for Rem operation: {:?}", ty),
            },
            // TODO: support short-circuiting for logical operations
            BinOp::BitAnd | BinOp::And => match ty {
                OperativeType::SignedInt | OperativeType::UnsignedInt | OperativeType::Bool => {
                    build_int! { scope.build_and(lhs, rhs) }
                }
                _ => panic!("Unsupported type for And operation: {:?}", ty),
            },
            BinOp::BitOr | BinOp::Or => match ty {
                OperativeType::SignedInt | OperativeType::UnsignedInt | OperativeType::Bool => {
                    build_int! { scope.build_or(lhs, rhs) }
                }
                _ => panic!("Unsupported type for Or operation: {:?}", ty),
            },
            BinOp::BitXor => match ty {
                OperativeType::SignedInt | OperativeType::UnsignedInt => {
                    build_int! { scope.build_xor(lhs, rhs) }
                }
                _ => panic!("Unsupported type for BitXor operation: {:?}", ty),
            },
            BinOp::Shl => match ty {
                OperativeType::SignedInt | OperativeType::UnsignedInt => {
                    build_int! { scope.build_left_shift(lhs, rhs) }
                }
                _ => panic!("Unsupported type for Shl operation: {:?}", ty),
            },
            BinOp::Shr => match ty {
                OperativeType::SignedInt => build_int! { scope.build_right_shift(lhs, rhs, true) },
                OperativeType::UnsignedInt => {
                    build_int! { scope.build_right_shift(lhs, rhs, false) }
                }
                _ => panic!("Unsupported type for Shr operation: {:?}", ty),
            },
            BinOp::Eq => match ty {
                OperativeType::SignedInt | OperativeType::UnsignedInt | OperativeType::Bool => {
                    build_int! { scope.build_int_compare(=> IntPredicate::EQ, lhs, rhs) }
                }
                OperativeType::Float => {
                    build_float! { scope.build_float_compare(=> FloatPredicate::OEQ, lhs, rhs) }
                }
                _ => panic!("Unsupported type for Eq operation: {:?}", ty),
            },
            BinOp::Ne => match ty {
                OperativeType::SignedInt | OperativeType::UnsignedInt | OperativeType::Bool => {
                    build_int! { scope.build_int_compare(=> IntPredicate::NE, lhs, rhs) }
                }
                OperativeType::Float => {
                    build_float! { scope.build_float_compare(=> FloatPredicate::ONE, lhs, rhs) }
                }
                _ => panic!("Unsupported type for Ne operation: {:?}", ty),
            },
            BinOp::Lt => match ty {
                OperativeType::SignedInt => {
                    build_int! { scope.build_int_compare(=> IntPredicate::SLT, lhs, rhs) }
                }
                OperativeType::UnsignedInt => {
                    build_int! { scope.build_int_compare(=> IntPredicate::ULT, lhs, rhs) }
                }
                OperativeType::Float => {
                    build_float! { scope.build_float_compare(=> FloatPredicate::OLT, lhs, rhs) }
                }
                _ => panic!("Unsupported type for Lt operation: {:?}", ty),
            },
            BinOp::Gt => match ty {
                OperativeType::SignedInt => {
                    build_int! { scope.build_int_compare(=> IntPredicate::SGT, lhs, rhs) }
                }
                OperativeType::UnsignedInt => {
                    build_int! { scope.build_int_compare(=> IntPredicate::UGT, lhs, rhs) }
                }
                OperativeType::Float => {
                    build_float! { scope.build_float_compare(=> FloatPredicate::OGT, lhs, rhs) }
                }
                _ => panic!("Unsupported type for Gt operation: {:?}", ty),
            },
            BinOp::Le => match ty {
                OperativeType::SignedInt => {
                    build_int! { scope.build_int_compare(=> IntPredicate::SLE, lhs, rhs) }
                }
                OperativeType::UnsignedInt => {
                    build_int! { scope.build_int_compare(=> IntPredicate::ULE, lhs, rhs) }
                }
                OperativeType::Float => {
                    build_float! { scope.build_float_compare(=> FloatPredicate::OLE, lhs, rhs) }
                }
                _ => panic!("Unsupported type for Le operation: {:?}", ty),
            },
            BinOp::Ge => match ty {
                OperativeType::SignedInt => {
                    build_int! { scope.build_int_compare(=> IntPredicate::SGE, lhs, rhs) }
                }
                OperativeType::UnsignedInt => {
                    build_int! { scope.build_int_compare(=> IntPredicate::UGE, lhs, rhs) }
                }
                OperativeType::Float => {
                    build_float! { scope.build_float_compare(=> FloatPredicate::OGE, lhs, rhs) }
                }
                _ => panic!("Unsupported type for Ge operation: {:?}", ty),
            },
        }
    }

    fn compile_una<'ctx>(
        &'ctx self,
        value: &Una,
        ty: OperativeType,
        scope: &mut Scope<'ctx, '_>,
    ) -> BasicValueEnum<'ctx> {
        let operand = self.compile_value(&value.operand, scope);
        
        match value.op {
            UnaOp::Neg => match ty {
                OperativeType::SignedInt | OperativeType::UnsignedInt => {
                    scope.builder.build_int_neg(operand.into_int_value(), "").unwrap().as_basic_value_enum()
                }
                OperativeType::Float => {
                    scope.builder.build_float_neg(operand.into_float_value(), "").unwrap().as_basic_value_enum()
                }
                _ => panic!("Unsupported type for Neg operation: {:?}", ty),
            },
            UnaOp::Not => match ty {
                OperativeType::SignedInt | OperativeType::UnsignedInt | OperativeType::Bool => {
                    scope.builder.build_not(operand.into_int_value(), "").unwrap().as_basic_value_enum()
                }
                _ => panic!("Unsupported type for Not operation: {:?}", ty),
            },
        }
    }

    fn compile_lit<'ctx>(&'ctx self, value: &Lit, scope: &Scope<'ctx, '_>) -> BasicValueEnum<'ctx> {
        match value {
            Lit::Num(x) => self.compile_num(x, scope),
            Lit::Bool(x) => scope
                .llvm
                .context
                .bool_type()
                .const_int(*x as u64, false)
                .as_basic_value_enum(),
        }
    }

    fn compile_num<'ctx>(&'ctx self, value: &Num, scope: &Scope<'ctx, '_>) -> BasicValueEnum<'ctx> {
        match value {
            Num::Int(x) => match x {
                &Int::Signed(x) => {
                    let (value, neg) = match x {
                        SignedInt::B8(i) => (
                            scope.llvm.context.i8_type().const_int(i.abs() as u64, true),
                            i < 0,
                        ),
                        SignedInt::B16(i) => (
                            scope
                                .llvm
                                .context
                                .i16_type()
                                .const_int(i.abs() as u64, true),
                            i < 0,
                        ),
                        SignedInt::B32(i) => (
                            scope
                                .llvm
                                .context
                                .i32_type()
                                .const_int(i.abs() as u64, true),
                            i < 0,
                        ),
                        SignedInt::B64(i) => (
                            scope
                                .llvm
                                .context
                                .i64_type()
                                .const_int(i.abs() as u64, true),
                            i < 0,
                        ),
                        SignedInt::B128(i) => (
                            scope
                                .llvm
                                .context
                                .i128_type()
                                .const_int_from_string(&i.abs().to_string(), StringRadix::Decimal)
                                .unwrap(),
                            i < 0,
                        ),
                    };

                    if neg {
                        value.const_neg().as_basic_value_enum()
                    } else {
                        value.as_basic_value_enum()
                    }
                }
                &Int::Unsigned(x) => match x {
                    UnsignedInt::U8(i) => scope.llvm.context.i8_type().const_int(i as u64, false),
                    UnsignedInt::U16(i) => scope.llvm.context.i16_type().const_int(i as u64, false),
                    UnsignedInt::U32(i) => scope.llvm.context.i32_type().const_int(i as u64, false),
                    UnsignedInt::U64(i) => scope.llvm.context.i64_type().const_int(i, false),
                    UnsignedInt::U128(i) => scope
                        .llvm
                        .context
                        .i128_type()
                        .const_int_from_string(&i.to_string(), StringRadix::Decimal)
                        .unwrap(),
                }
                .as_basic_value_enum(),
            },
            Num::Float(x) => match x {
                Float::F32(x) => scope
                    .llvm
                    .context
                    .f32_type()
                    .const_float(*x as f64)
                    .as_basic_value_enum(),
                Float::F64(x) => scope
                    .llvm
                    .context
                    .f64_type()
                    .const_float(*x)
                    .as_basic_value_enum(),
            },
        }
    }

    fn compile_call<'ctx>(
        &'ctx self,
        value: &Call,
        scope: &mut Scope<'ctx, '_>,
    ) -> BasicValueEnum<'ctx> {
        let function_value = scope
            .llvm_module
            .get_function(&value.function_name)
            .expect("Failed to get function");

        if function_value.get_type().get_return_type().is_none() {
            panic!("Function {} has no return type", value.function_name);
        }

        let args = value
            .args
            .iter()
            .map(|arg| self.compile_value(arg, scope).into())
            .collect::<Vec<_>>();
        let value = scope.builder.build_call(function_value, &args, "").unwrap();

        value.try_as_basic_value().unwrap_left()
    }
}
