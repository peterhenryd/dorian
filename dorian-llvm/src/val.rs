use inkwell::values::BasicValue;
use ast::val::{Arg, Bin, BinOp, Call, ContextValue, Expr, Float, Int, Lit, Num, SignedInt, Una, UnaOp, UnsignedInt, Value};
use crate::{llvm, Llvm};
use crate::scope::{LocalScope, Scope};

impl Llvm {
    pub(crate) fn compile_value<'ctx>(
        &'ctx self,
        value: &Value,
        scope: Scope<'ctx, '_, '_>,
    ) -> Option<llvm::Value<'ctx>> {
        match &value {
            Value::Context(x) => Some(self.compile_context_value(x, scope.to_local()?)),
            Value::Expr(x) => self.compile_expr(x, scope),
            Value::Lit(x) => Some(self.compile_lit(x)),
            Value::Call(x) => self.compile_call(x, scope),
        }
    }

    fn compile_context_value<'ctx>(
        &'ctx self,
        value: &ContextValue,
        scope: &LocalScope<'ctx, '_>,
    ) -> llvm::Value<'ctx> {
        match value {
            ContextValue::Arg(arg) => self.compile_arg_value(arg, scope),
            ContextValue::Var(var) => scope
                .get_var(var, true)
                .expect("Failed to get variable from scope"),
        }
    }

    fn compile_arg_value<'ctx>(
        &'ctx self,
        arg: &Arg,
        scope: &LocalScope<'ctx, '_>,
    ) -> llvm::Value<'ctx> {
        let value = scope
            .function
            .get_nth_param(arg.param_index)
            .expect("Failed to get function parameter");
        
        let attribute_loc = llvm::AttributeLoc::Param(arg.param_index);
        let signage = scope.function
            .get_string_attribute(attribute_loc, "signage")
            .map(|x| x.get_string_value().to_bytes() == b"signed");

        llvm::Value::new(value, signage)
    }

    fn compile_expr<'ctx>(
        &'ctx self,
        value: &Expr,
        scope: Scope<'ctx, '_, '_>
    ) -> Option<llvm::Value<'ctx>> {
        match value {
            Expr::Bin(x) => self.compile_bin(x, scope),
            Expr::Una(x) => self.compile_una(x, scope),
        }
    }

    fn compile_bin<'ctx>(&'ctx self, value: &Bin, scope: Scope<'ctx, '_, '_>) -> Option<llvm::Value<'ctx>> {
        let lhs = self.compile_value(&value.lhs, scope)?;
        let rhs = self.compile_value(&value.rhs, scope)?;

        let value = if lhs.raw.is_int_value() {
            let signed = lhs.signage.unwrap_or(false);
            let lhs = lhs.raw.into_int_value();

            self.compile_int_bin(value.op, value.no_wrap, signed, lhs, rhs.raw, scope.to_local()?)
                .as_basic_value_enum()
        } else if lhs.raw.is_float_value() {
            let lhs = lhs.raw.into_float_value();

            self.compile_float_bin(value.op, lhs, rhs.raw, scope.to_local()?)
        } else {
            panic!("Value does not support binary operations: {:?}", lhs.raw);
        };

        Some(llvm::Value::new(value, lhs.signage))
    }

    fn compile_int_bin<'ctx>(
        &'ctx self,
        op: BinOp,
        no_wrap: bool,
        signed: bool,
        lhs: llvm::Int<'ctx>,
        rhs: llvm::RawValue<'ctx>,
        scope: &LocalScope<'ctx, '_>,
    ) -> llvm::Int<'ctx> {
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
                .build_int_compare(llvm::IntCmpOp::EQ, lhs, rhs, "")
                .unwrap(),
            BinOp::Ne => scope
                .builder
                .build_int_compare(llvm::IntCmpOp::NE, lhs, rhs, "")
                .unwrap(),
            BinOp::Lt if signed => scope
                .builder
                .build_int_compare(llvm::IntCmpOp::SLT, lhs, rhs, "")
                .unwrap(),
            BinOp::Lt => scope
                .builder
                .build_int_compare(llvm::IntCmpOp::ULT, lhs, rhs, "")
                .unwrap(),
            BinOp::Gt if signed => scope
                .builder
                .build_int_compare(llvm::IntCmpOp::SGT, lhs, rhs, "")
                .unwrap(),
            BinOp::Gt => scope
                .builder
                .build_int_compare(llvm::IntCmpOp::UGT, lhs, rhs, "")
                .unwrap(),
            BinOp::Le if signed => scope
                .builder
                .build_int_compare(llvm::IntCmpOp::SLE, lhs, rhs, "")
                .unwrap(),
            BinOp::Le => scope
                .builder
                .build_int_compare(llvm::IntCmpOp::ULE, lhs, rhs, "")
                .unwrap(),
            BinOp::Ge if signed => scope
                .builder
                .build_int_compare(llvm::IntCmpOp::SGE, lhs, rhs, "")
                .unwrap(),
            BinOp::Ge => scope
                .builder
                .build_int_compare(llvm::IntCmpOp::UGE, lhs, rhs, "")
                .unwrap(),
        }
    }

    fn compile_float_bin<'ctx>(
        &'ctx self,
        op: BinOp,
        lhs: llvm::Float<'ctx>,
        rhs: llvm::RawValue<'ctx>,
        scope: &LocalScope<'ctx, '_>,
    ) -> llvm::RawValue<'ctx> {
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
                .build_float_compare(llvm::FloatCmpOp::OEQ, lhs, rhs, "")
                .unwrap()
                .as_basic_value_enum(),
            BinOp::Ne => scope
                .builder
                .build_float_compare(llvm::FloatCmpOp::ONE, lhs, rhs, "")
                .unwrap()
                .as_basic_value_enum(),
            BinOp::Lt => scope
                .builder
                .build_float_compare(llvm::FloatCmpOp::OLT, lhs, rhs, "")
                .unwrap()
                .as_basic_value_enum(),
            BinOp::Gt => scope
                .builder
                .build_float_compare(llvm::FloatCmpOp::OGT, lhs, rhs, "")
                .unwrap()
                .as_basic_value_enum(),
            BinOp::Le => scope
                .builder
                .build_float_compare(llvm::FloatCmpOp::OLE, lhs, rhs, "")
                .unwrap()
                .as_basic_value_enum(),
            BinOp::Ge => scope
                .builder
                .build_float_compare(llvm::FloatCmpOp::OGE, lhs, rhs, "")
                .unwrap()
                .as_basic_value_enum(),
            _ => panic!("Unsupported float operation: {:?}", op),
        }
    }

    fn compile_una<'ctx>(&'ctx self, value: &Una, scope: Scope<'ctx, '_, '_>) -> Option<llvm::Value<'ctx>> {
        let operand = self.compile_value(&value.operand, scope)?;
        let signed = operand.signage.unwrap_or(false);

        let value = if operand.raw.is_int_value() {
            let operand = operand.raw.into_int_value();
            self.compile_int_una(value.op, value.no_wrap, signed, operand, scope).as_basic_value_enum()
        } else if operand.raw.is_float_value() {
            let operand = operand.raw.into_float_value();
            self.compile_float_una(value.op, operand, scope.to_local()?).as_basic_value_enum()
        } else {
            panic!("Value does not support unary operations: {:?}", operand.raw);
        };

        Some(llvm::Value::new(value, operand.signage))
    }

    fn compile_int_una<'ctx>(
        &'ctx self,
        op: UnaOp,
        no_wrap: bool,
        signed: bool,
        operand: llvm::Int<'ctx>,
        scope: Scope<'ctx, '_, '_>
    ) -> llvm::Int<'ctx> {
        match scope {
            Scope::Global { .. } => match op {
                UnaOp::Neg if no_wrap && signed => operand.const_nsw_neg(),
                UnaOp::Neg if no_wrap && !signed => operand.const_nuw_neg(),
                UnaOp::Neg => operand.const_neg(),
                UnaOp::Not => operand.const_not(),
            }
            Scope::Local(LocalScope { builder: b, .. }) => match op {
                UnaOp::Neg if no_wrap && signed => b.build_int_nsw_neg(operand, ""),
                UnaOp::Neg if no_wrap && !signed => b.build_int_nuw_neg(operand, ""),
                UnaOp::Neg => b.build_int_neg(operand, ""),
                UnaOp::Not => b.build_not(operand, ""),
            }.unwrap(),
        }
    }

    fn compile_float_una<'ctx>(
        &'ctx self,
        op: UnaOp,
        operand: llvm::Float<'ctx>,
        scope: &LocalScope<'ctx, '_>,
    ) -> llvm::Float<'ctx> {
        match op {
            UnaOp::Neg => scope.builder.build_float_neg(operand, "").unwrap(),
            UnaOp::Not => panic!("Not operation is not supported for float values"),
        }
    }

    fn compile_lit(&self, value: &Lit) -> llvm::Value {
        match value {
            Lit::Num(x) => self.compile_num(x),
            Lit::Bool(x) => {
                let raw_value = self.context.bool_type()
                    .const_int(*x as u64, false)
                    .as_basic_value_enum();
                llvm::Value::new(raw_value, None)
            },
        }
    }

    fn compile_num(&self, value: &Num) -> llvm::Value {
        match value {
            Num::Int(x) => match x {
                &Int::Signed(x) => {
                    let (raw_value, neg) = match x {
                        SignedInt::B8(i) => (
                            self.context.i8_type().const_int(i.abs() as u64, true),
                            i < 0,
                        ),
                        SignedInt::B16(i) => (
                            self.context.i16_type().const_int(i.abs() as u64, true),
                            i < 0,
                        ),
                        SignedInt::B32(i) => (
                            self.context.i32_type().const_int(i.abs() as u64, true),
                            i < 0,
                        ),
                        SignedInt::B64(i) => (
                            self.context.i64_type().const_int(i.abs() as u64, true),
                            i < 0,
                        ),
                        SignedInt::B128(i) => (
                            self.context.i128_type().const_int_from_string(
                                &i.abs().to_string(),
                                llvm::StringRadix::Decimal,
                            ).unwrap(),
                            i < 0,
                        ),
                    };
                    
                    let raw_value = if neg {
                        raw_value.const_neg().as_basic_value_enum()
                    } else {
                        raw_value.as_basic_value_enum()
                    };
                    
                    llvm::Value::new(raw_value, Some(true))
                },
                &Int::Unsigned(x) => {
                    let raw_value = match x {
                        UnsignedInt::U8(i) => self.context.i8_type().const_int(i as u64, false),
                        UnsignedInt::U16(i) => self.context.i16_type().const_int(i as u64, false),
                        UnsignedInt::U32(i) => self.context.i32_type().const_int(i as u64, false),
                        UnsignedInt::U64(i) => self.context.i64_type().const_int(i, false),
                        UnsignedInt::U128(i) => self.context.i128_type()
                            .const_int_from_string(&i.to_string(), llvm::StringRadix::Decimal)
                            .unwrap(),
                    }.as_basic_value_enum();
                    
                    llvm::Value::new(raw_value, Some(false))
                },
            },
            Num::Float(x) => {
                let raw_value = match x {
                    Float::F32(x) => self.context.f32_type()
                        .const_float(*x as f64)
                        .as_basic_value_enum(),
                    Float::F64(x) => self.context.f64_type()
                        .const_float(*x)
                        .as_basic_value_enum(),
                };
                llvm::Value::new(raw_value, None)
            },
        }
    }

    fn compile_call<'ctx>(
        &'ctx self,
        value: &Call,
        scope: Scope<'ctx, '_, '_>,
    ) -> Option<llvm::Value<'ctx>> {
        let function_value = scope.to_local()?.module
            .get_function(&value.function_name)
            .expect("Failed to get function");
        let signage = function_value
            .get_string_attribute(llvm::AttributeLoc::Return, "signage")
            .map(|x| x.get_string_value().to_bytes() == b"signed");

        if function_value.get_type().get_return_type().is_none() {
            panic!("Function {} has no return type", value.function_name);
        }

        let mut args = Vec::with_capacity(value.args.len());
        for arg in &value.args {
            let compiled_arg = self.compile_value(arg, scope)?;
            args.push(compiled_arg.raw.into());
        }
        let value = scope.to_local()?.builder
            .build_call(function_value, &args, "")
            .unwrap()
            .try_as_basic_value()
            .unwrap_left();
        
        Some(llvm::Value::new(value, signage))
    }
}
