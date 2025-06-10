use crate::Llvm;
use crate::scope::Scope;
use dorian_ast::val::{
    Arg, Bin, BinOp, Call, ContextValue, Expr, Float, Int, Lit, Num, SignedInt, Una, UnaOp,
    UnsignedInt, Value,
};
use inkwell::attributes::AttributeLoc;
use inkwell::types::StringRadix;
use inkwell::values::{BasicValue, BasicValueEnum, FloatValue, IntValue};
use inkwell::{FloatPredicate, IntPredicate};

#[derive(Debug, Copy, Clone)]
pub struct LlvmValue<'ctx> {
    pub(crate) value: BasicValueEnum<'ctx>,
    signage: Option<bool>,
}

impl Llvm {
    pub(crate) fn compile_value<'ctx>(
        &'ctx self,
        value: &Value,
        scope: &mut Scope<'ctx, '_>,
    ) -> LlvmValue<'ctx> {
        let value = match &value {
            Value::Context(x) => self.compile_context_value(x, scope),
            // TODO: remove hardcoded type
            Value::Expr(x) => self.compile_expr(x, scope),
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
    ) -> LlvmValue<'ctx> {
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
    ) -> LlvmValue<'ctx> {
        let signage = scope
            .llvm_function
            .get_string_attribute(AttributeLoc::Param(value.param_index), "signage")
            .map(|x| x.get_string_value().to_bytes() == b"signed");
        let value = scope
            .llvm_function
            .get_nth_param(value.param_index)
            .expect("Failed to get function parameter");

        LlvmValue { value, signage }
    }

    fn compile_expr<'ctx>(
        &'ctx self,
        value: &Expr,
        scope: &mut Scope<'ctx, '_>,
    ) -> LlvmValue<'ctx> {
        match value {
            Expr::Bin(x) => self.compile_bin(x, scope),
            Expr::Una(x) => self.compile_una(x, scope),
        }
    }

    fn compile_bin<'ctx>(&'ctx self, value: &Bin, scope: &mut Scope<'ctx, '_>) -> LlvmValue<'ctx> {
        let lhs = self.compile_value(&value.lhs, scope);
        let rhs = self.compile_value(&value.rhs, scope);

        let value = if lhs.value.is_int_value() {
            let signed = lhs.signage.unwrap_or(false);
            let lhs = lhs.value.into_int_value();

            self.compile_int_bin(value.op, value.no_wrap, signed, lhs, rhs.value, scope)
                .as_basic_value_enum()
        } else if lhs.value.is_float_value() {
            let lhs = lhs.value.into_float_value();

            self.compile_float_bin(value.op, lhs, rhs.value, scope)
        } else {
            panic!("Value does not support binary operations: {:?}", lhs.value);
        };

        LlvmValue {
            value,
            signage: lhs.signage,
        }
    }

    fn compile_int_bin<'ctx>(
        &'ctx self,
        op: BinOp,
        no_wrap: bool,
        signed: bool,
        lhs: IntValue<'ctx>,
        rhs: BasicValueEnum<'ctx>,
        scope: &Scope<'ctx, '_>,
    ) -> IntValue<'ctx> {
        if !rhs.is_int_value() {
            panic!(
                "Right-hand value type of binary operation does not match left-hand type which is an integer"
            );
        }
        let rhs = rhs.into_int_value();

        match op {
            BinOp::Add if no_wrap && signed => {
                scope.builder.build_int_nsw_add(lhs, rhs, "").unwrap()
            }
            BinOp::Add if no_wrap && !signed => {
                scope.builder.build_int_nuw_add(lhs, rhs, "").unwrap()
            }
            BinOp::Add => scope.builder.build_int_add(lhs, rhs, "").unwrap(),
            BinOp::Sub if no_wrap && signed => {
                scope.builder.build_int_nsw_sub(lhs, rhs, "").unwrap()
            }
            BinOp::Sub if no_wrap && !signed => {
                scope.builder.build_int_nuw_sub(lhs, rhs, "").unwrap()
            }
            BinOp::Sub => scope.builder.build_int_sub(lhs, rhs, "").unwrap(),
            BinOp::Mul if no_wrap && signed => {
                scope.builder.build_int_nsw_mul(lhs, rhs, "").unwrap()
            }
            BinOp::Mul if no_wrap && !signed => {
                scope.builder.build_int_nuw_mul(lhs, rhs, "").unwrap()
            }
            BinOp::Mul => scope.builder.build_int_mul(lhs, rhs, "").unwrap(),
            BinOp::Div if signed => scope.builder.build_int_signed_div(lhs, rhs, "").unwrap(),
            BinOp::Div => scope.builder.build_int_unsigned_div(lhs, rhs, "").unwrap(),
            BinOp::Rem if signed => scope.builder.build_int_signed_rem(lhs, rhs, "").unwrap(),
            BinOp::Rem => scope.builder.build_int_unsigned_rem(lhs, rhs, "").unwrap(),
            BinOp::And | BinOp::BitAnd => scope.builder.build_and(lhs, rhs, "").unwrap(),
            BinOp::Or | BinOp::BitOr => scope.builder.build_or(lhs, rhs, "").unwrap(),
            BinOp::BitXor => scope.builder.build_xor(lhs, rhs, "").unwrap(),
            BinOp::Shl => scope.builder.build_left_shift(lhs, rhs, "").unwrap(),
            BinOp::Shr => scope.builder.build_right_shift(lhs, rhs, true, "").unwrap(),
            BinOp::Eq => scope
                .builder
                .build_int_compare(IntPredicate::EQ, lhs, rhs, "")
                .unwrap(),
            BinOp::Ne => scope
                .builder
                .build_int_compare(IntPredicate::NE, lhs, rhs, "")
                .unwrap(),
            BinOp::Lt if signed => scope
                .builder
                .build_int_compare(IntPredicate::SLT, lhs, rhs, "")
                .unwrap(),
            BinOp::Lt => scope
                .builder
                .build_int_compare(IntPredicate::ULT, lhs, rhs, "")
                .unwrap(),
            BinOp::Gt if signed => scope
                .builder
                .build_int_compare(IntPredicate::SGT, lhs, rhs, "")
                .unwrap(),
            BinOp::Gt => scope
                .builder
                .build_int_compare(IntPredicate::UGT, lhs, rhs, "")
                .unwrap(),
            BinOp::Le if signed => scope
                .builder
                .build_int_compare(IntPredicate::SLE, lhs, rhs, "")
                .unwrap(),
            BinOp::Le => scope
                .builder
                .build_int_compare(IntPredicate::ULE, lhs, rhs, "")
                .unwrap(),
            BinOp::Ge if signed => scope
                .builder
                .build_int_compare(IntPredicate::SGE, lhs, rhs, "")
                .unwrap(),
            BinOp::Ge => scope
                .builder
                .build_int_compare(IntPredicate::UGE, lhs, rhs, "")
                .unwrap(),
        }
    }

    fn compile_float_bin<'ctx>(
        &'ctx self,
        op: BinOp,
        lhs: FloatValue<'ctx>,
        rhs: BasicValueEnum<'ctx>,
        scope: &Scope<'ctx, '_>,
    ) -> BasicValueEnum<'ctx> {
        if !rhs.is_float_value() {
            panic!(
                "Right-hand value type of binary operation does not match left-hand type which is a float"
            );
        }
        let rhs = rhs.into_float_value();

        match op {
            BinOp::Add => scope
                .builder
                .build_float_add(lhs, rhs, "")
                .unwrap()
                .as_basic_value_enum(),
            BinOp::Sub => scope
                .builder
                .build_float_sub(lhs, rhs, "")
                .unwrap()
                .as_basic_value_enum(),
            BinOp::Mul => scope
                .builder
                .build_float_mul(lhs, rhs, "")
                .unwrap()
                .as_basic_value_enum(),
            BinOp::Div => scope
                .builder
                .build_float_div(lhs, rhs, "")
                .unwrap()
                .as_basic_value_enum(),
            BinOp::Rem => scope
                .builder
                .build_float_rem(lhs, rhs, "")
                .unwrap()
                .as_basic_value_enum(),
            BinOp::Eq => scope
                .builder
                .build_float_compare(FloatPredicate::OEQ, lhs, rhs, "")
                .unwrap()
                .as_basic_value_enum(),
            BinOp::Ne => scope
                .builder
                .build_float_compare(FloatPredicate::ONE, lhs, rhs, "")
                .unwrap()
                .as_basic_value_enum(),
            BinOp::Lt => scope
                .builder
                .build_float_compare(FloatPredicate::OLT, lhs, rhs, "")
                .unwrap()
                .as_basic_value_enum(),
            BinOp::Gt => scope
                .builder
                .build_float_compare(FloatPredicate::OGT, lhs, rhs, "")
                .unwrap()
                .as_basic_value_enum(),
            BinOp::Le => scope
                .builder
                .build_float_compare(FloatPredicate::OLE, lhs, rhs, "")
                .unwrap()
                .as_basic_value_enum(),
            BinOp::Ge => scope
                .builder
                .build_float_compare(FloatPredicate::OGE, lhs, rhs, "")
                .unwrap()
                .as_basic_value_enum(),
            _ => panic!("Unsupported float operation: {:?}", op),
        }
    }

    fn compile_una<'ctx>(&'ctx self, value: &Una, scope: &mut Scope<'ctx, '_>) -> LlvmValue<'ctx> {
        let operand = self.compile_value(&value.operand, scope);
        let signed = operand.signage.unwrap_or(false);

        let value = if operand.value.is_int_value() {
            let operand = operand.value.into_int_value();
            self.compile_int_una(value.op, value.no_wrap, signed, operand, scope)
                .as_basic_value_enum()
        } else if operand.value.is_float_value() {
            let operand = operand.value.into_float_value();
            self.compile_float_una(value.op, operand, scope)
                .as_basic_value_enum()
        } else {
            panic!(
                "Value does not support unary operations: {:?}",
                operand.value
            );
        };

        LlvmValue {
            value,
            signage: operand.signage,
        }
    }

    fn compile_int_una<'ctx>(
        &'ctx self,
        op: UnaOp,
        no_wrap: bool,
        signed: bool,
        operand: IntValue<'ctx>,
        scope: &Scope<'ctx, '_>,
    ) -> IntValue<'ctx> {
        match op {
            UnaOp::Neg if no_wrap && signed => {
                scope.builder.build_int_nsw_neg(operand, "").unwrap()
            }
            UnaOp::Neg if no_wrap && !signed => {
                scope.builder.build_int_nuw_neg(operand, "").unwrap()
            }
            UnaOp::Neg => scope.builder.build_int_neg(operand, "").unwrap(),
            UnaOp::Not => scope.builder.build_not(operand, "").unwrap(),
        }
    }

    fn compile_float_una<'ctx>(
        &'ctx self,
        op: UnaOp,
        operand: FloatValue<'ctx>,
        scope: &Scope<'ctx, '_>,
    ) -> FloatValue<'ctx> {
        match op {
            UnaOp::Neg => scope.builder.build_float_neg(operand, "").unwrap(),
            UnaOp::Not => panic!("Not operation is not supported for float values"),
        }
    }

    fn compile_lit<'ctx>(&'ctx self, value: &Lit, scope: &Scope<'ctx, '_>) -> LlvmValue<'ctx> {
        match value {
            Lit::Num(x) => self.compile_num(x, scope),
            Lit::Bool(x) => LlvmValue {
                value: scope
                    .llvm
                    .context
                    .bool_type()
                    .const_int(*x as u64, false)
                    .as_basic_value_enum(),
                signage: None,
            },
        }
    }

    fn compile_num<'ctx>(&'ctx self, value: &Num, scope: &Scope<'ctx, '_>) -> LlvmValue<'ctx> {
        match value {
            Num::Int(x) => match x {
                &Int::Signed(x) => LlvmValue {
                    value: {
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
                                    .const_int_from_string(
                                        &i.abs().to_string(),
                                        StringRadix::Decimal,
                                    )
                                    .unwrap(),
                                i < 0,
                            ),
                        };

                        if neg {
                            value.const_neg().as_basic_value_enum()
                        } else {
                            value.as_basic_value_enum()
                        }
                    },
                    signage: Some(true),
                },
                &Int::Unsigned(x) => LlvmValue {
                    value: match x {
                        UnsignedInt::U8(i) => {
                            scope.llvm.context.i8_type().const_int(i as u64, false)
                        }
                        UnsignedInt::U16(i) => {
                            scope.llvm.context.i16_type().const_int(i as u64, false)
                        }
                        UnsignedInt::U32(i) => {
                            scope.llvm.context.i32_type().const_int(i as u64, false)
                        }
                        UnsignedInt::U64(i) => scope.llvm.context.i64_type().const_int(i, false),
                        UnsignedInt::U128(i) => scope
                            .llvm
                            .context
                            .i128_type()
                            .const_int_from_string(&i.to_string(), StringRadix::Decimal)
                            .unwrap(),
                    }
                    .as_basic_value_enum(),
                    signage: Some(false),
                },
            },
            Num::Float(x) => LlvmValue {
                value: match x {
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
                signage: None,
            },
        }
    }

    fn compile_call<'ctx>(
        &'ctx self,
        value: &Call,
        scope: &mut Scope<'ctx, '_>,
    ) -> LlvmValue<'ctx> {
        let function_value = scope
            .llvm_module
            .get_function(&value.function_name)
            .expect("Failed to get function");
        let signage = function_value
            .get_string_attribute(AttributeLoc::Return, "signage")
            .map(|x| x.get_string_value().to_bytes() == b"signed");

        if function_value.get_type().get_return_type().is_none() {
            panic!("Function {} has no return type", value.function_name);
        }

        let args = value
            .args
            .iter()
            .map(|arg| self.compile_value(arg, scope).value.into())
            .collect::<Vec<_>>();
        let value = scope
            .builder
            .build_call(function_value, &args, "")
            .unwrap()
            .try_as_basic_value()
            .unwrap_left();

        LlvmValue { value, signage }
    }
}
