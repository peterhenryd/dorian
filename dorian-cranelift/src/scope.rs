use crate::cl::{FuncOrDataId, InstBuilder};
use crate::{cl, Cranelift};
use ast::block::stmt::Stmt;
use ast::block::Block;
use ast::val::{Bin, BinOp, Call, ContextValue, Expr, Float, Int, Lit, Num, SignedInt, Una, UnaOp, Value};
use std::collections::HashMap;

pub(crate) struct Scope<'ctx> {
    namespace: u32,
    module: &'ctx cl::Module,
    builder: cl::FunctionBuilder<'ctx>,
    imported_functions: HashMap<cl::FuncId, cl::FuncRef>,
}

impl Scope<'_> {
    pub(crate) fn compile_body(&mut self, body: &Block) {
        let block = self.builder.create_block();
        self.builder.switch_to_block(block);
        self.builder.append_block_params_for_function_params(block);

        for stmt in &body.stmts {
            self.compile_stmt(stmt);
        }
    }

    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::If(_) => todo!("If statements are not yet implemented"),
            Stmt::While(_) => todo!("While statements are not yet implemented"),
            Stmt::Return(x) => {
                let values = x.values.iter()
                    .map(|x| self.compile_value(x).raw.to_scalar().unwrap())
                    .collect::<Vec<_>>();
                self.builder.ins().return_(&values);
            }
            Stmt::Bind(_) => todo!("Bind statements are not yet implemented"),
            Stmt::Assign(_) => todo!("Assign statements are not yet implemented"),
        }
    }

    fn compile_value(&mut self, value: &Value) -> cl::Value { 
        match value {
            Value::Context(x) => self.compile_context_value(x),
            Value::Expr(x) => self.compile_expr(x),
            Value::Lit(x) => self.compile_lit(x),
            Value::Call(x) => self.compile_call(x),
        }
    }
    
    fn compile_context_value(&mut self, value: &ContextValue) -> cl::Value {
        todo!("Context values are not yet implemented: {:?}", value);
    }

    fn compile_expr(&mut self, value: &Expr) -> cl::Value {
        match value {
            Expr::Bin(x) => self.compile_bin(x),
            Expr::Una(x) => self.compile_una(x),
        }
    }

    fn compile_bin(&mut self, value: &Bin) -> cl::Value {
        let (lhs, rhs) = (self.compile_value(&value.lhs), self.compile_value(&value.rhs));
        let signed = lhs.signage.unwrap_or(false);
        let (lhs, rhs) = (lhs.raw.to_scalar().unwrap(), rhs.raw.to_scalar().unwrap());

        let ty = self.builder.func.dfg.value_type(lhs);
        if ty.is_int() {
            self.compile_int_bin(value.op, signed, lhs, rhs)
        } else if ty.is_float() {
            self.compile_float_bin(value.op, lhs, rhs)
        } else {
            panic!("Type does not support binary operations: {:?}", ty);
        }
    }

    fn compile_int_bin(
        &mut self,
        op: BinOp,
        signed: bool,
        lhs: cl::Scalar,
        rhs: cl::Scalar,
    ) -> cl::Value {
        let ty = self.builder.func.dfg.value_type(rhs);
        if !ty.is_int() {
            panic!(
                "Right-hand value type of binary operation does not match left-hand type which is an integer"
            );
        }

        let scalar = match op {
            BinOp::Add => self.builder.ins().iadd(lhs, rhs),
            BinOp::Sub => self.builder.ins().isub(lhs, rhs),
            BinOp::Mul => self.builder.ins().imul(lhs, rhs),
            BinOp::Div if signed => self.builder.ins().sdiv(lhs, rhs),
            BinOp::Div => self.builder.ins().udiv(lhs, rhs),
            BinOp::Rem if signed => self.builder.ins().srem(lhs, rhs),
            BinOp::Rem => self.builder.ins().urem(lhs, rhs),
            BinOp::And | BinOp::BitAnd => self.builder.ins().band(lhs, rhs),
            BinOp::Or | BinOp::BitOr => self.builder.ins().bor(lhs, rhs),
            BinOp::BitXor => self.builder.ins().bxor(lhs, rhs),
            BinOp::Shl => self.builder.ins().ishl(lhs, rhs),
            BinOp::Shr if signed => self.builder.ins().sshr(lhs, rhs),
            BinOp::Shr => self.builder.ins().ushr(lhs, rhs),
            BinOp::Eq => self.builder.ins().icmp(cl::IntCmpOp::Equal, lhs, rhs),
            BinOp::Ne => self.builder.ins().icmp(cl::IntCmpOp::NotEqual, lhs, rhs),
            BinOp::Lt if signed => self.builder.ins().icmp(cl::IntCmpOp::SignedLessThan, lhs, rhs),
            BinOp::Lt => self.builder.ins().icmp(cl::IntCmpOp::UnsignedLessThan, lhs, rhs),
            BinOp::Gt if signed => self.builder.ins().icmp(cl::IntCmpOp::SignedGreaterThan, lhs, rhs),
            BinOp::Gt => self.builder.ins().icmp(cl::IntCmpOp::UnsignedGreaterThan, lhs, rhs),
            BinOp::Le if signed => self.builder.ins().icmp(cl::IntCmpOp::SignedLessThanOrEqual, lhs, rhs),
            BinOp::Le => self.builder.ins().icmp(cl::IntCmpOp::UnsignedLessThanOrEqual, lhs, rhs),
            BinOp::Ge if signed => self.builder.ins().icmp(cl::IntCmpOp::SignedGreaterThanOrEqual, lhs, rhs),
            BinOp::Ge => self.builder.ins().icmp(cl::IntCmpOp::UnsignedGreaterThanOrEqual, lhs, rhs),
        };
        
        cl::Value {
            raw: cl::ValueItem::Scalar(scalar),
            signage: Some(signed),
        }
    }

    fn compile_float_bin(
        &mut self,
        op: BinOp,
        lhs: cl::Scalar,
        rhs: cl::Scalar,
    ) -> cl::Value {
        let ty = self.builder.func.dfg.value_type(lhs);
        if !ty.is_float() {
            panic!(
                "Right-hand value type of binary operation does not match left-hand type which is a float"
            );
        }

        let scalar = match op {
            BinOp::Add => self.builder.ins().fadd(lhs, rhs),
            BinOp::Sub => self.builder.ins().fsub(lhs, rhs),
            BinOp::Mul => self.builder.ins().fmul(lhs, rhs),
            BinOp::Div => self.builder.ins().fdiv(lhs, rhs),
            BinOp::Eq => self.builder.ins().fcmp(cl::FloatCmpOp::Equal, lhs, rhs),
            BinOp::Ne => self.builder.ins().fcmp(cl::FloatCmpOp::NotEqual, lhs, rhs),
            BinOp::Lt => self.builder.ins().fcmp(cl::FloatCmpOp::LessThan, lhs, rhs),
            BinOp::Gt => self.builder.ins().fcmp(cl::FloatCmpOp::GreaterThan, lhs, rhs),
            BinOp::Le => self.builder.ins().fcmp(cl::FloatCmpOp::LessThanOrEqual, lhs, rhs),
            BinOp::Ge => self.builder.ins().fcmp(cl::FloatCmpOp::GreaterThanOrEqual, lhs, rhs),
            _ => panic!("Unsupported float operation: {:?}", op),
        };
        
        cl::Value {
            raw: cl::ValueItem::Scalar(scalar),
            signage: None,
        }
    }

    fn compile_una(&mut self, value: &Una) -> cl::Value {
        let operand = self.compile_value(&value.operand);
        let signed = operand.signage.unwrap_or(false);
        
        let ty = self.builder.func.dfg.value_type(operand.raw.to_scalar().unwrap());
        if ty.is_int() {
            self.compile_int_una(value.op, signed, operand.raw.to_scalar().unwrap())
        } else if ty.is_float() {
            self.compile_float_una(value.op, operand.raw.to_scalar().unwrap())
        } else {
            panic!("Value does not support unary operations: {:?}", ty);
        }
    }
        
    fn compile_int_una(
        &mut self,
        op: UnaOp,
        signed: bool,
        operand: cl::Scalar,
    ) -> cl::Value { 
        let raw = match op {
            UnaOp::Neg => self.builder.ins().ineg(operand),
            UnaOp::Not => self.builder.ins().bnot(operand),
        };
        
        cl::Value {
            raw: cl::ValueItem::Scalar(raw),
            signage: Some(signed),
        }
    }

    fn compile_float_una<'ctx>(
        &mut self,
        op: UnaOp,
        operand: cl::Scalar,
    ) -> cl::Value {
        match op {
            UnaOp::Neg => cl::Value {
                raw: cl::ValueItem::Scalar(self.builder.ins().fneg(operand)),
                signage: None,
            },
            UnaOp::Not => panic!("Not operation is not supported for float values"),
        }
    }

    fn compile_lit(&mut self, value: &Lit) -> cl::Value { 
        match value {
            Lit::Num(x) => match x {
                Num::Int(x) => {
                    let (integer, signed) = match *x {
                        Int::Signed(x) => (x, true),
                        Int::Unsigned(x) => (x.cast_signed(), false),
                    };

                    let raw = match integer {
                        SignedInt::B8(x) => self.builder.ins().iconst(cl::I8, x as i64),
                        SignedInt::B16(x) => self.builder.ins().iconst(cl::I16, x as i64),
                        SignedInt::B32(x) => self.builder.ins().iconst(cl::I32, x as i64),
                        SignedInt::B64(x) => self.builder.ins().iconst(cl::I64, x),
                        SignedInt::B128(x) => {
                            let (lo, hi) = (x as i64, (x >> 64) as i64);
                            let lo = self.builder.ins().iconst(cl::I64, lo);
                            let hi = self.builder.ins().iconst(cl::I64, hi);
                            self.builder.ins().iconcat(lo, hi)
                        },
                    };
                    
                    cl::Value {
                        raw: cl::ValueItem::Scalar(raw),
                        signage: Some(signed),
                    }
                }
                Num::Float(x) => cl::Value {
                    raw: cl::ValueItem::Scalar(match x { 
                        &Float::F32(x) => self.builder.ins().f32const(x), 
                        &Float::F64(x) => self.builder.ins().f64const(x), 
                    }),
                    signage: None,
                }
            }
            &Lit::Bool(x) => cl::Value {
                raw: cl::ValueItem::Scalar(self.builder.ins().iconst(cl::I8, x as i64)),
                signage: Some(false),             
            },
        }
    }
    
    fn compile_call(&mut self, value: &Call) -> cl::Value {
        let func_ref = self.get_func_ref(&value.function_name).unwrap();
        
        let mut values = vec![];
        for arg in &value.args {
            let value = self.compile_value(arg);
            self.flatten_value(value, &mut values);
        }
        
        cl::Value {
            raw: cl::ValueItem::Variable(self.builder.ins().call(func_ref, &values)),
            // TODO: look up function signature to determine signage
            signage: None,
        }
    }

    fn get_func_ref(&mut self, name: &str) -> Option<cl::FuncRef> {
        let func_id = self.get_func_id(name)?;
        if let Some(func_ref) = self.imported_functions.get(&func_id) {
            return Some(*func_ref);
        }

        let name = cl::UserExternalName::new(self.namespace, func_id.as_u32());
        let name_ref = self.builder.func.declare_imported_user_function(name);

        let signature = &self.module.get_function_decl(func_id).signature;
        let sig_ref = self.builder.func.import_signature(signature.clone());

        let data = cl::ExtFuncData {
            name: cl::ExternalName::User(name_ref),
            signature: sig_ref,
            colocated: false,
        };

        let func_ref = self.builder.func.import_function(data);
        self.imported_functions.insert(func_id, func_ref);

        Some(func_ref)
    }

    fn get_func_id(&self, name: &str) -> Option<cl::FuncId> {
        match self.module.get_name(name)? {
            FuncOrDataId::Func(x) => Some(x),
            FuncOrDataId::Data(_) => None,
        }
    }

    pub fn finish(self) {
        self.builder.finalize();
    }

    fn flatten_value(&self, value: cl::Value, values: &mut Vec<cl::Scalar>) {
        match value.raw {
            cl::ValueItem::Scalar(x) => {
                values.push(x);
            }
            cl::ValueItem::Variable(x) => {
                for value in self.builder.inst_results(x) {
                    values.push(*value);
                }
            }
        }
    }
}
        
impl Cranelift {
    pub(crate) fn create_scope(&mut self, func_id: cl::FuncId, signature: cl::Signature) -> Scope {
        let name = cl::UserFuncName::user(self.namespace, func_id.as_u32());
        self.function = Some(cl::Function::with_name_signature(name, signature));

        Scope {
            module: &self.module,
            builder: cl::FunctionBuilder::new(self.function.as_mut().unwrap(), &mut self.context),
            namespace: self.namespace,
            imported_functions: HashMap::new(),
        }
    }
}