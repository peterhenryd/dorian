use std::mem::{MaybeUninit, transmute};
use crate::llvm::builder::Builder;
use crate::llvm::module::Module;
use crate::llvm::sys::core::*;
use crate::llvm::sys::LLVMContext;
use crate::llvm::{Error, from_c_string, to_c_string};
use crate::llvm::types::Type;
use std::ptr::NonNull;
use std::slice;
use llvm_sys::ir_reader::LLVMParseIRInContext;
use llvm_sys::{LLVMDiagnosticHandler, LLVMOpaqueAttributeRef};
use llvm_sys::target::{LLVMIntPtrTypeForASInContext, LLVMIntPtrTypeInContext};
use crate::llvm::basic_block::BasicBlock;
use crate::llvm::debug::Metadata;
use crate::llvm::fun::Fun;
use crate::llvm::memory_buffer::MemoryBuffer;
use crate::llvm::target::TargetData;
use crate::llvm::value::Value;

pub struct Context(NonNull<LLVMContext>);

impl Context {
    pub fn from_raw(raw: NonNull<LLVMContext>) -> Context {
        Context(raw)
    }

    pub fn as_raw(&self) -> NonNull<LLVMContext> {
        self.0
    }

    pub fn new() -> Context {
        unsafe { Context(NonNull::new_unchecked(LLVMContextCreate())) }
    }

    pub fn global() -> Context {
        Context(unsafe {
            NonNull::new_unchecked(
                LLVMGetGlobalContext()
            )
        })
    }

    pub fn create_builder(&self) -> Builder {
        unsafe {
            Builder::from_raw(NonNull::new_unchecked(LLVMCreateBuilderInContext(
                self.0.as_ptr(),
            )))
        }
    }

    pub fn create_module(&self, name: &str) -> Module {
        unsafe {
            Module::from_raw(
                self,
                NonNull::new_unchecked(LLVMModuleCreateWithNameInContext(
                    to_c_string(Some(name)).as_ptr(),
                    self.0.as_ptr(),
                )),
            )
        }
    }

    pub fn get_fun_type(&self, parameters: Vec<Type>, return_type: Type, is_var_arg: bool) -> Type {
        let len = parameters.len();
        unsafe {
            Type::from_raw(NonNull::new_unchecked(LLVMFunctionType(
                return_type.as_raw().as_ptr(),
                parameters
                    .into_iter()
                    .map(|t| t.as_raw().as_ptr())
                    .collect::<Vec<_>>()
                    .as_mut_ptr(),
                len as u32,
                is_var_arg as i32,
            )))
        }
    }

    pub fn create_named_struct_type(&self, name: &str, parameters: Vec<Type>, is_packed: bool) -> Type {
        unsafe {
            let s = LLVMStructCreateNamed(self.0.as_ptr(), to_c_string(Some(name)).as_ptr());

            let len = parameters.len();
            LLVMStructSetBody(s, parameters.into_iter()
                .map(|t| t.as_raw().as_ptr())
                .collect::<Vec<_>>()
                .as_mut_ptr(), len as u32, is_packed as i32);

            Type::from_raw(NonNull::new_unchecked(s))
        }
    }

    pub fn get_struct_type(&self, parameters: Vec<Type>, is_packed: bool) -> Type {
        let len = parameters.len();
        unsafe {
            Type::from_raw(NonNull::new_unchecked(
                LLVMStructTypeInContext(
                    self.0.as_ptr(),
                    parameters.into_iter()
                        .map(|t| t.as_raw().as_ptr())
                        .collect::<Vec<_>>()
                        .as_mut_ptr(),
                    len as u32,
                    is_packed as i32
                )
            ))
        }
    }

    pub fn get_integer_type(&self, bits: u32) -> Type {
        unsafe {
            Type::from_raw(NonNull::new_unchecked(LLVMIntTypeInContext(
                self.0.as_ptr(),
                bits,
            )))
        }
    }

    pub fn get_f16_type(&self) -> Type {
        unsafe {
            Type::from_raw(NonNull::new_unchecked(LLVMHalfTypeInContext(
                self.0.as_ptr(),
            )))
        }
    }

    pub fn get_bf16_type(&self) -> Type {
        unsafe {
            Type::from_raw(NonNull::new_unchecked(LLVMBFloatTypeInContext(
                self.0.as_ptr(),
            )))
        }
    }

    pub fn get_f32_type(&self) -> Type {
        unsafe {
            Type::from_raw(NonNull::new_unchecked(LLVMFloatTypeInContext(
                self.0.as_ptr(),
            )))
        }
    }

    pub fn get_f64_type(&self) -> Type {
        unsafe {
            Type::from_raw(NonNull::new_unchecked(LLVMDoubleTypeInContext(
                self.0.as_ptr(),
            )))
        }
    }

    pub fn get_x86_f80_type(&self) -> Type {
        unsafe {
            Type::from_raw(NonNull::new_unchecked(LLVMX86FP80TypeInContext(
                self.0.as_ptr(),
            )))
        }
    }

    pub fn get_f128_type(&self) -> Type {
        unsafe {
            Type::from_raw(NonNull::new_unchecked(LLVMFP128TypeInContext(
                self.0.as_ptr(),
            )))
        }
    }

    pub fn get_ppc_f128_type(&self) -> Type {
        unsafe {
            Type::from_raw(NonNull::new_unchecked(LLVMPPCFP128TypeInContext(
                self.0.as_ptr(),
            )))
        }
    }

    pub fn get_void_type(&self) -> Type {
        unsafe {
            Type::from_raw(NonNull::new_unchecked(
                LLVMVoidTypeInContext(self.0.as_ptr())
            ))
        }
    }

    pub fn get_vector_type(ty: Type, count: u32) -> Type {
        unsafe {
            Type::from_raw(NonNull::new_unchecked(
                LLVMVectorType(ty.as_raw().as_ptr(), count)
            ))
        }
    }

    pub fn const_string(&self, str: &str, dont_null_terminate: bool) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMConstStringInContext(self.0.as_ptr(), to_c_string(Some(str)).as_ptr(), str.len() as u32, dont_null_terminate as i32)
            )
        })
    }

    pub fn const_struct(&self, values: Vec<Value>, packed: bool, ) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMConstStructInContext(
                    self.0.as_ptr(),
                    values.iter()
                        .map(|value| value.as_raw().as_ptr())
                        .collect::<Vec<_>>()
                        .as_mut_ptr(),
                    values.len() as u32,
                    packed as i32
                )
            )
        })
    }


    /*
    pub fn set_diagnostic_handler(
        &self,
        handler: DiagnosticHandler,
        diagnostic_context: *mut c_void,
    ) {
        unsafe {
            LLVMContextSetDiagnosticHandler(
                self.0.as_ptr(),
                handler.0,
                diagnostic_context
            )
        }
    }

    pub fn get_diagnostic_handler(&self) -> DiagnosticHandler {
        DiagnosticHandler(unsafe {
            LLVMContextGetDiagnosticHandler(self.0.as_ptr())
        })
    }

    // TODO: find a way to remove c_void
    pub fn get_diagnostic_context(&self) -> *mut c_void {
        unsafe {
            LLVMContextGetDiagnosticContext(self.0.as_ptr())
        }
    }

    // TODO: LLVMYieldCallback needs to be abstracted
    pub fn set_yield_callback(&self, callback: LLVMYieldCallback, opaque_handle: *mut c_void) {
        unsafe {
            LLVMContextSetYieldCallback(self.0.as_ptr(), callback, opaque_handle)
        }
    }
     */

    pub fn should_discard_value_names(&self) -> bool {
        unsafe { LLVMContextShouldDiscardValueNames(self.0.as_ptr()) != 0 }
    }

    pub fn set_discard_value_names(&self, discard_value_names: bool) {
        unsafe {
            LLVMContextSetDiscardValueNames(self.0.as_ptr(), discard_value_names as i32)
        }
    }

    pub fn get_md_kind_id(&self, name: &str) -> u32 {
        unsafe {
            LLVMGetMDKindIDInContext(self.0.as_ptr(), to_c_string(Some(name)).as_ptr(), name.len() as u32)
        }
    }

    pub fn create_enum_attribute(&self, kind_id: u32, val: u64) -> Attribute {
        Attribute::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMCreateEnumAttribute(self.0.as_ptr(), kind_id, val)
            )
        })
    }

    pub fn create_type_attribute(&self, kind_id: u32, ty: Type) -> Attribute {
        Attribute::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMCreateTypeAttribute(self.0.as_ptr(), kind_id, ty.as_raw().as_ptr())
            )
        })
    }

    pub fn create_string_attribute(&self, k: &str, v: &str) -> Attribute {
        Attribute::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMCreateStringAttribute(
                    self.0.as_ptr(),
                    to_c_string(Some(k)).as_ptr(), k.len() as u32,
                    to_c_string(Some(v)).as_ptr(), k.len() as u32
                )
            )
        })
    }

    pub fn get_type_by_name(&self, name: &str) -> Type {
        Type::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetTypeByName2(
                    self.0.as_ptr(),
                    to_c_string(Some(name)).as_ptr()
                )
            )
        })
    }

    pub fn struct_type(
        &self,
        elements: Vec<Type>,
        packed: bool,
    ) -> Type {
        Type::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMStructType(
                    elements.iter()
                        .map(|element| element.as_raw().as_ptr())
                        .collect::<Vec<_>>()
                        .as_mut_ptr(),
                    elements.len() as u32,
                    packed as i32
                )
            )
        })
    }

    pub fn struct_create_named(&self, name: &str) -> Type {
        Type::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMStructCreateNamed(
                    self.0.as_ptr(),
                    to_c_string(Some(name)).as_ptr()
                )
            )
        })
    }

    pub fn parse_ir<'a>(
        &'a self,
        context: &'a Context,
        memory_buffer: MemoryBuffer,
    ) -> Result<Module, Error> {
        let mut module = MaybeUninit::uninit();
        let mut msg = MaybeUninit::uninit();
        let code = unsafe {
            LLVMParseIRInContext(
                self.0.as_ptr(),
                memory_buffer.as_raw().as_ptr(),
                module.as_mut_ptr(),
                msg.as_mut_ptr(),
            )
        };

        if code == 0 {
            unsafe { Ok(Module::from_raw(context, NonNull::new_unchecked(module.assume_init()))) }
        } else {
            unsafe { Err(Error::create(from_c_string(msg.assume_init()))) }
        }
    }

    pub fn int_ptr_type(&self, td: TargetData) -> Type {
        Type::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMIntPtrTypeInContext(
                    self.0.as_ptr(),
                    td.as_raw().as_ptr()
                )
            )
        })
    }

    pub fn int_ptr_type_for_as(&self, td: TargetData, r#as: u32) -> Type {
        Type::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMIntPtrTypeForASInContext(
                    self.0.as_ptr(),
                    td.as_raw().as_ptr(),
                    r#as
                )
            )
        })
    }

    pub fn create_metadata_string(&self, str: &str) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMMDStringInContext2(self.0.as_ptr(), to_c_string(Some(str)).as_ptr(), str.len())
            )
        })
    }

    pub fn create_metadata_node(&self, metadata: Vec<Metadata>) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMMDNodeInContext2(
                    self.0.as_ptr(),
                    metadata.iter()
                        .map(|m| m.as_raw().as_ptr())
                        .collect::<Vec<_>>()
                        .as_mut_ptr(),
                    metadata.len()
                )
            )
        })
    }

    pub fn as_value(&self, metadata: Metadata) -> Value {
        Value::from_raw(unsafe {
            NonNull::new_unchecked(LLVMMetadataAsValue(self.0.as_ptr(), metadata.as_raw().as_ptr()))
        })
    }

    pub fn create_basic_block(&self, name: &str) -> BasicBlock {
        BasicBlock::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMCreateBasicBlockInContext(self.0.as_ptr(), to_c_string(Some(name)).as_ptr())
            )
        })
    }

    pub fn append_basic_block(&self, fun: Fun, name: &str) -> BasicBlock {
        BasicBlock::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMAppendBasicBlockInContext(self.0.as_ptr(), fun.as_raw().as_ptr(), to_c_string(Some(name)).as_ptr())
            )
        })
    }

    pub fn insert_basic_block(
        &self, basic_block: BasicBlock, name: &str
    ) -> BasicBlock {
        BasicBlock::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMInsertBasicBlockInContext(self.0.as_ptr(), basic_block.as_raw().as_ptr(), to_c_string(Some(name)).as_ptr())
            )
        })
    }

}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.0.as_ptr());
        }
    }
}

pub struct Attribute(NonNull<LLVMOpaqueAttributeRef>);

impl Attribute {
    pub fn from_raw(raw: NonNull<LLVMOpaqueAttributeRef>) -> Attribute {
        Attribute(raw)
    }

    pub fn as_raw(&self) -> NonNull<LLVMOpaqueAttributeRef> {
        self.0
    }

    pub fn get_enum_kind_for_name(name: &str) -> u32 {
        unsafe {
            LLVMGetEnumAttributeKindForName(
                to_c_string(Some(name)).as_ptr(),
                name.len()
            )
        }
    }

    pub fn get_last_enum_kind() -> u32 {
        unsafe {
            LLVMGetLastEnumAttributeKind()
        }
    }

    pub fn get_enum_attribute_kind(&self) -> u32 {
        unsafe {
            LLVMGetEnumAttributeKind(self.0.as_ptr())
        }
    }

    pub fn get_enum_attribute_value(&self) -> u64 {
        unsafe {
            LLVMGetEnumAttributeValue(self.0.as_ptr())
        }
    }

    pub fn get_type_attribute_value(&self) -> Type {
        Type::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMGetTypeAttributeValue(self.0.as_ptr())
            )
        })
    }

    pub fn get_string_attribute_kind(&self) -> String {
        let mut len = MaybeUninit::<u32>::uninit();
        let chars = unsafe { LLVMGetStringAttributeKind(self.0.as_ptr(), len.as_mut_ptr()) };

        String::from_utf8(unsafe {
            let char_slice = slice::from_raw_parts(chars, len.assume_init() as usize);

            transmute::<&[i8], &[u8]>(char_slice)
        }.to_vec()).unwrap()
    }

    pub fn get_string_attribute_value(&self) -> String {
        let mut len = MaybeUninit::<u32>::uninit();
        let chars = unsafe { LLVMGetStringAttributeValue(self.0.as_ptr(), len.as_mut_ptr()) };

        String::from_utf8(unsafe {
            let char_slice = slice::from_raw_parts(chars, len.assume_init() as usize);

            transmute::<&[i8], &[u8]>(char_slice)
        }.to_vec()).unwrap()
    }

    pub fn is_enum_attribute(&self) -> bool {
        unsafe { LLVMIsEnumAttribute(self.0.as_ptr()) != 0 }
    }

    pub fn is_string_attribute(&self) -> bool {
        unsafe { LLVMIsStringAttribute(self.0.as_ptr()) != 0 }
    }

    pub fn is_type_attribute(&self) -> bool {
        unsafe { LLVMIsTypeAttribute(self.0.as_ptr()) != 0 }
    }
}

pub struct DiagnosticHandler(LLVMDiagnosticHandler);

impl DiagnosticHandler {
    pub fn from_raw(raw: LLVMDiagnosticHandler) -> DiagnosticHandler {
        DiagnosticHandler(raw)
    }

    pub fn as_raw(&self) -> LLVMDiagnosticHandler {
        self.0
    }
}

// TODO: pub fn LLVMGetDiagInfoDescription(DI: LLVMDiagnosticInfoRef) -> *mut ::libc::c_char;
// TODO: pub fn LLVMGetDiagInfoSeverity(DI: LLVMDiagnosticInfoRef) -> LLVMDiagnosticSeverity;
// TODO: pub fn LLVMGetMDKindID(Name: *const ::libc::c_char, SLen: ::libc::c_uint) -> ::libc::c_uint;