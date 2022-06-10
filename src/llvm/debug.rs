use std::mem::{MaybeUninit, transmute};
use std::ptr::NonNull;
use std::slice;
use llvm_sys::debuginfo::{LLVMDebugMetadataVersion, LLVMDIBuilderCreateArrayType, LLVMDIBuilderCreateArtificialType, LLVMDIBuilderCreateAutoVariable, LLVMDIBuilderCreateBasicType, LLVMDIBuilderCreateBitFieldMemberType, LLVMDIBuilderCreateClassType, LLVMDIBuilderCreateCompileUnit, LLVMDIBuilderCreateConstantValueExpression, LLVMDIBuilderCreateEnumerationType, LLVMDIBuilderCreateEnumerator, LLVMDIBuilderCreateExpression, LLVMDIBuilderCreateFile, LLVMDIBuilderCreateForwardDecl, LLVMDIBuilderCreateFunction, LLVMDIBuilderCreateGlobalVariableExpression, LLVMDIBuilderCreateImportedDeclaration, LLVMDIBuilderCreateImportedModuleFromAlias, LLVMDIBuilderCreateImportedModuleFromModule, LLVMDIBuilderCreateImportedModuleFromNamespace, LLVMDIBuilderCreateInheritance, LLVMDIBuilderCreateLexicalBlock, LLVMDIBuilderCreateLexicalBlockFile, LLVMDIBuilderCreateMacro, LLVMDIBuilderCreateMemberPointerType, LLVMDIBuilderCreateMemberType, LLVMDIBuilderCreateModule, LLVMDIBuilderCreateNameSpace, LLVMDIBuilderCreateNullPtrType, LLVMDIBuilderCreateObjCIVar, LLVMDIBuilderCreateObjCProperty, LLVMDIBuilderCreateObjectPointerType, LLVMDIBuilderCreateParameterVariable, LLVMDIBuilderCreatePointerType, LLVMDIBuilderCreateQualifiedType, LLVMDIBuilderCreateReferenceType, LLVMDIBuilderCreateReplaceableCompositeType, LLVMDIBuilderCreateStaticMemberType, LLVMDIBuilderCreateStructType, LLVMDIBuilderCreateSubroutineType, LLVMDIBuilderCreateTempGlobalVariableFwdDecl, LLVMDIBuilderCreateTempMacroFile, LLVMDIBuilderCreateTypedef, LLVMDIBuilderCreateUnionType, LLVMDIBuilderCreateUnspecifiedType, LLVMDIBuilderCreateVectorType, LLVMDIBuilderFinalize, LLVMDIBuilderFinalizeSubprogram, LLVMDIBuilderGetOrCreateArray, LLVMDIBuilderGetOrCreateSubrange, LLVMDIBuilderGetOrCreateTypeArray, LLVMDIBuilderInsertDbgValueAtEnd, LLVMDIBuilderInsertDbgValueBefore, LLVMDIBuilderInsertDeclareAtEnd, LLVMDIBuilderInsertDeclareBefore, LLVMDIFileGetDirectory, LLVMDIFileGetFilename, LLVMDIGlobalVariableExpressionGetExpression, LLVMDIGlobalVariableExpressionGetVariable, LLVMDILocationGetColumn, LLVMDILocationGetInlinedAt, LLVMDILocationGetLine, LLVMDILocationGetScope, LLVMDIScopeGetFile, LLVMDisposeDIBuilder, LLVMDisposeTemporaryMDNode, LLVMDISubprogramGetLine, LLVMDITypeGetAlignInBits, LLVMDITypeGetFlags, LLVMDITypeGetLine, LLVMDITypeGetName, LLVMDITypeGetOffsetInBits, LLVMDITypeGetSizeInBits, LLVMDIVariableGetFile, LLVMDIVariableGetLine, LLVMDIVariableGetScope, LLVMGetMetadataKind, LLVMMetadataReplaceAllUsesWith, LLVMTemporaryMDNode};
use llvm_sys::{LLVMOpaqueDIBuilder, LLVMOpaqueMetadata};
use llvm_sys::prelude::LLVMMetadataRef;
use crate::llvm::basic_block::BasicBlock;
use crate::llvm::context::Context;
use crate::llvm::to_c_string;
use crate::llvm::value::Value;
use crate::value::LlvmValue;

pub const DI_FLAG_ZERO: i32 = 0;
pub const DI_FLAG_PRIVATE: i32 = 1;
pub const DI_FLAG_PROTECTED: i32 = 2;
pub const DI_FLAG_PUBLIC: i32 = 3;
pub const DI_FLAG_FWD_DECL: i32 = 1 << 2;
pub const DI_FLAG_APPLE_BLOCK: i32 = 1 << 3;
pub const DI_FLAG_RESERVED_BIT4: i32 = 1 << 4;
pub const DI_FLAG_VIRTUAL: i32 = 1 << 5;
pub const DI_FLAG_ARTIFICIAL: i32 = 1 << 6;
pub const DI_FLAG_EXPLICIT: i32 = 1 << 7;
pub const DI_FLAG_PROTOTYPED: i32 = 1 << 8;
pub const DI_FLAG_OBJC_CLASS_COMPLETE: i32 = 1 << 9;
pub const DI_FLAG_OBJECT_POINTER: i32 = 1 << 10;
pub const DI_FLAG_VECTOR: i32 = 1 << 11;
pub const DI_FLAG_STATIC_MEMBER: i32 = 1 << 12;
pub const DI_FLAG_LVALUE_REFERENCE: i32 = 1 << 13;
pub const DI_FLAG_RVALUE_REFERENCE: i32 = 1 << 14;
pub const DI_FLAG_RESERVED: i32 = 1 << 15;
pub const DI_FLAG_SINGLE_INHERITANCE: i32 = 1 << 16;
pub const DI_FLAG_MULTIPLE_INHERITANCE: i32 = 2 << 16;
pub const DI_FLAG_VIRTUAL_INHERITANCE: i32 = 3 << 16;
pub const DI_FLAG_INTRODUCED_VIRTUAL: i32 = 1 << 18;
pub const DI_FLAG_BIT_FIELD: i32 = 1 << 19;
pub const DI_FLAG_NO_RETURN: i32 = 1 << 20;
pub const DI_FLAG_TYPE_PASS_BY_VALUE: i32 = 1 << 22;
pub const DI_FLAG_TYPE_PASS_BY_REFERENCE: i32 = 1 << 23;
pub const DI_FLAG_ENUM_CLASS: i32 = 1 << 24;
pub const DI_FLAG_THUNK: i32 = 1 << 25;
pub const DI_FLAG_NON_TRIVIAL: i32 = 1 << 26;
pub const DI_FLAG_BIG_ENDIAN: i32 = 1 << 27;
pub const DI_FLAG_LITTLE_ENDIAN: i32 = 1 << 28;
pub const DI_FLAG_INDIRECT_VIRTUAL_BASE: i32 = (1 << 2) | (1 << 5);
pub const DI_FLAG_ACCESSIBILITY: i32 =
    DI_FLAG_PROTECTED | DI_FLAG_PRIVATE | DI_FLAG_PUBLIC;
pub const DI_FLAG_PTR_TO_MEMBER_REP: i32 =
    DI_FLAG_SINGLE_INHERITANCE | DI_FLAG_MULTIPLE_INHERITANCE | DI_FLAG_VIRTUAL_INHERITANCE;

#[repr(C)]
#[derive(Debug)]
pub enum DWARFSourceLanguage {
    C89,
    C,
    Ada83,
    CPP,
    Cobol74,
    Cobol85,
    Fortran77,
    Fortran90,
    Pascal83,
    Modula2,
    Java,
    C99,
    Ada95,
    Fortran95,
    PLI,
    ObjC,
    ObjCPP,
    UPC,
    D,
    Python,
    OpenCL,
    Go,
    Modula3,
    Haskell,
    CPP03,
    CPP11,
    OCaml,
    Rust,
    C11,
    Swift,
    Julia,
    Dylan,
    CPP14,
    Fortran03,
    Fortran08,
    RenderScript,
    BLISS,
    MipsAssembler,
    GoogleRenderScript,
    BorlandDelphi,
}

#[repr(C)]
#[derive(Debug)]
pub enum DWARFEmissionKind {
    None = 0,
    Full,
    LineTablesOnly,
}

#[repr(C)]
#[derive(Debug)]
pub enum MetadataKind {
    MDString,
    ConstantAsMetadata,
    LocalAsMetadata,
    DistinctMDOperandPlaceholder,
    MDTuple,
    DILocation,
    DIExpression,
    DIGlobalVariableExpression,
    GenericDINode,
    DISubrange,
    DIEnumerator,
    DIBasicType,
    DIDerivedType,
    DICompositeType,
    DISubroutineType,
    DIFile,
    DICompileUnit,
    DISubprogram,
    DILexicalBlock,
    DILexicalBlockFile,
    DINamespace,
    DIModule,
    DITemplateTypeParameter,
    DITemplateValueParameter,
    DIGlobalVariable,
    DILocalVariable,
    DILabel,
    DIObjCProperty,
    DIImportedEntity,
    DIMacro,
    DIMacroFile,
    DICommonBlock,
    DIStringType,
    DIGenericSubrange,
    DIArgList,
}

pub type DWARFTypeEncoding = u32;

#[repr(C)]
#[derive(Debug)]
pub enum DWARFMacinfoRecordType {
    Define = 0x01,
    Macro = 0x02,
    StartFile = 0x03,
    EndFile = 0x04,
    VendorExt = 0xff,
}

/// The current debug metadata version number.
pub fn debug_metadata_version() -> u32 {
    unsafe { LLVMDebugMetadataVersion() }
}

// TODO: i'm not sure if this is correctly implemented whatsoever (impl)
pub struct MetadataNode(Metadata);

impl MetadataNode {
    pub fn temporary(context: &Context, data: Vec<Metadata>) -> MetadataNode {
        MetadataNode(Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMTemporaryMDNode(
                    context.as_raw().as_ptr(),
                    data.iter()
                        .map(|metadata| metadata.0.as_ptr())
                        .collect::<Vec<LLVMMetadataRef>>()
                        .as_mut_ptr(),
                    data.len(),
                )
            )
        }))
    }
}

impl Drop for MetadataNode {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeTemporaryMDNode(self.0.as_raw().as_ptr());
        }
    }
}

/// TODO: i have absolutely ZERO idea what the purpose of this is so i just copied LLVM directly (feel free to fix/edit)
pub struct Metadata(NonNull<LLVMOpaqueMetadata>);

impl Metadata {
    pub fn from_raw(raw: NonNull<LLVMOpaqueMetadata>) -> Metadata {
        Metadata(raw)
    }

    pub fn as_raw(&self) -> NonNull<LLVMOpaqueMetadata> {
        self.0
    }

    pub fn get_kind(&self) -> MetadataKind {
        unsafe {
            transmute(LLVMGetMetadataKind(self.0.as_ptr()))
        }
    }

    pub fn subprogram_get_line(&self) -> u32 {
        unsafe {
            LLVMDISubprogramGetLine(self.0.as_ptr())
        }
    }

    pub fn replace_all_uses_with(&self, replacement: Metadata) {
        unsafe {
            LLVMMetadataReplaceAllUsesWith(self.0.as_ptr(), replacement.0.as_ptr());
        }
    }

    pub fn type_get_size_in_bits(&self) -> u64 {
        unsafe {
            LLVMDITypeGetSizeInBits(self.0.as_ptr())
        }
    }

    pub fn type_get_offset_in_bits(&self) -> u64 {
        unsafe {
            LLVMDITypeGetOffsetInBits(self.0.as_ptr())
        }
    }

    pub fn type_get_align_in_bits(&self) -> u32 {
        unsafe {
            LLVMDITypeGetAlignInBits(self.0.as_ptr())
        }
    }

    pub fn type_get_line(&self) -> u32 {
        unsafe {
            LLVMDITypeGetLine(self.0.as_ptr())
        }
    }

    pub fn type_get_flags(&self) -> i32 {
        unsafe {
            LLVMDITypeGetFlags(self.0.as_ptr())
        }
    }

    pub fn global_variable_expression_get_variable(&self) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIGlobalVariableExpressionGetVariable(self.0.as_ptr())
            )
        })
    }

    pub fn global_variable_expression_get_expression(&self) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIGlobalVariableExpressionGetExpression(self.0.as_ptr())
            )
        })
    }

    pub fn variable_get_file(&self) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIVariableGetFile(self.0.as_ptr())
            )
        })
    }

    pub fn variable_get_scope(&self) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIVariableGetScope(self.0.as_ptr())
            )
        })
    }

    pub fn variable_get_line(&self) -> u32 {
        unsafe {
            LLVMDIVariableGetLine(self.0.as_ptr())
        }
    }

    pub fn type_get_name(&self) -> String {
        let mut len = MaybeUninit::<usize>::uninit();
        let chars = unsafe { LLVMDITypeGetName(self.0.as_ptr(), len.as_mut_ptr()) };

        String::from_utf8(unsafe {
            let char_slice = slice::from_raw_parts(chars, len.assume_init());

            transmute::<&[i8], &[u8]>(char_slice)
        }.to_vec()).unwrap()
    }

    pub fn location_get_line(&self) -> u32 {
        unsafe {
            LLVMDILocationGetLine(self.0.as_ptr())
        }
    }

    pub fn location_get_column(&self) -> u32 {
        unsafe {
            LLVMDILocationGetColumn(self.0.as_ptr())
        }
    }

    pub fn location_get_scope(&self) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDILocationGetScope(self.0.as_ptr())
            )
        })
    }

    pub fn location_get_inlined_at(&self) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDILocationGetInlinedAt(self.0.as_ptr())
            )
        })
    }

    pub fn scope_get_file(&self) -> Metadata {
         Metadata::from_raw(unsafe {
             NonNull::new_unchecked(
                 LLVMDIScopeGetFile(self.0.as_ptr())
             )
         })
    }

    pub fn file_get_directory(&self) -> String {
        let mut len = MaybeUninit::<u32>::uninit();
        let chars = unsafe { LLVMDIFileGetDirectory(self.0.as_ptr(), len.as_mut_ptr()) };

        String::from_utf8(unsafe {
            let char_slice = slice::from_raw_parts(chars, len.assume_init() as usize);

            transmute::<&[i8], &[u8]>(char_slice)
        }.to_vec()).unwrap()
    }

    pub fn file_get_filename(&self) -> String {
        let mut len = MaybeUninit::<u32>::uninit();
        let chars = unsafe { LLVMDIFileGetFilename(self.0.as_ptr(), len.as_mut_ptr()) };

        String::from_utf8(unsafe {
            let char_slice = slice::from_raw_parts(chars, len.assume_init() as usize);

            transmute::<&[i8], &[u8]>(char_slice)
        }.to_vec()).unwrap()
    }

    pub fn file_get_source(&self, ) -> String {
        let mut len = MaybeUninit::<u32>::uninit();
        let chars = unsafe { LLVMDIFileGetDirectory(self.0.as_ptr(), len.as_mut_ptr()) };

        String::from_utf8(unsafe {
            let char_slice = slice::from_raw_parts(chars, len.assume_init() as usize);

            transmute::<&[i8], &[u8]>(char_slice)
        }.to_vec()).unwrap()
    }
}

pub struct DIBuilder(NonNull<LLVMOpaqueDIBuilder>);

impl DIBuilder {
    pub fn from_raw(raw: NonNull<LLVMOpaqueDIBuilder>) -> DIBuilder {
        DIBuilder(raw)
    }

    pub fn as_raw(&self) -> NonNull<LLVMOpaqueDIBuilder> {
        self.0
    }

    pub fn finalize_subprogram(&self, subprogram: Metadata) {
        unsafe {
            LLVMDIBuilderFinalizeSubprogram(self.0.as_ptr(), subprogram.0.as_ptr());
        }
    }

    pub fn create_compile_unit(
        &self,
        lang: DWARFSourceLanguage,
        file_ref: Metadata,
        producer: &str,
        is_optimized: bool,
        flags: &str,
        runtime_ver: u32,
        split_name: &str,
        kind: DWARFEmissionKind,
        dwo_id: u32,
        split_debug_inlining: bool,
        debug_info_for_profiling: bool,
        sys_root: &str,
        sdk: &str,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateCompileUnit(
                    self.0.as_ptr(),
                    transmute(lang),
                    file_ref.0.as_ptr(),
                    to_c_string(Some(producer)).as_ptr(),
                    producer.len(),
                    is_optimized as i32,
                    to_c_string(Some(flags)).as_ptr(),
                    flags.len(),
                    runtime_ver,
                    to_c_string(Some(split_name)).as_ptr(),
                    split_name.len(),
                    transmute(kind),
                    dwo_id,
                    split_debug_inlining as i32,
                    debug_info_for_profiling as i32,
                    to_c_string(Some(sys_root)).as_ptr(),
                    sys_root.len(),
                    to_c_string(Some(sdk)).as_ptr(),
                    sdk.len()
                )
            )
        })
    }

    pub fn create_temp_global_variable_fwd_decl(
        &self,
        scope: Metadata,
        name: &str,
        linkage: &str,
        file: Metadata,
        line_no: u32,
        ty: Metadata,
        local_to_unit: bool,
        decl: Metadata,
        align_in_bits: u32,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(LLVMDIBuilderCreateTempGlobalVariableFwdDecl(
                self.0.as_ptr(),
                scope.0.as_ptr(),
                to_c_string(Some(name)).as_ptr(),
                name.len(),
                to_c_string(Some(linkage)).as_ptr(),
                linkage.len(),
                file.0.as_ptr(),
                line_no,
                ty.0.as_ptr(),
                local_to_unit as i32,
                decl.0.as_ptr(),
                align_in_bits
            ))
        })
    }

    pub fn insert_declare_before(
        &self,
        storage: LlvmValue,
        var_info: Metadata,
        expr: Metadata,
        debug_loc: Metadata,
        instr: LlvmValue,
    ) -> LlvmValue {
        LlvmValue::from_raw(unsafe {
            NonNull::new_unchecked(LLVMDIBuilderInsertDeclareBefore(
                self.0.as_ptr(),
                storage.as_raw().as_ptr(),
                var_info.0.as_ptr(),
                expr.0.as_ptr(),
                debug_loc.0.as_ptr(),
                instr.as_raw().as_ptr()
            ))
        })
    }

    pub fn insert_declare_at_end(
        &self,
        storage: LlvmValue,
        var_info: Metadata,
        expr: Metadata,
        debug_loc: Metadata,
        block: BasicBlock,
    ) -> LlvmValue {
        LlvmValue::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderInsertDeclareAtEnd(
                    self.0.as_ptr(),
                    storage.as_raw().as_ptr(),
                    var_info.0.as_ptr(),
                    expr.0.as_ptr(),
                    debug_loc.0.as_ptr(),
                    block.as_raw().as_ptr(),
                )
            )
        })
    }

    pub fn insert_dbg_value_before(
        &self,
        val: LlvmValue,
        var_info: Metadata,
        expr: Metadata,
        debug_loc: Metadata,
        instr: LlvmValue,
    ) -> LlvmValue {
        LlvmValue::from_raw(unsafe {
            NonNull::new_unchecked(LLVMDIBuilderInsertDbgValueBefore(
                self.0.as_ptr(),
                val.as_raw().as_ptr(),
                var_info.0.as_ptr(),
                expr.0.as_ptr(),
                debug_loc.0.as_ptr(),
                instr.as_raw().as_ptr(),
            ))
        })
    }

    pub fn insert_dbg_value_at_end(
        &self,
        val: LlvmValue,
        var_info: Metadata,
        expr: Metadata,
        debug_loc: Metadata,
        block: BasicBlock,
    ) -> LlvmValue {
        LlvmValue::from_raw(unsafe {
            NonNull::new_unchecked(LLVMDIBuilderInsertDbgValueAtEnd(
                self.0.as_ptr(),
                val.as_raw().as_ptr(),
                var_info.0.as_ptr(),
                expr.0.as_ptr(),
                debug_loc.0.as_ptr(),
                block.as_raw().as_ptr()
            ))
        })
    }
    pub fn create_auto_variable(
        &self,
        scope: Metadata,
        name: &str,
        file: Metadata,
        line_no: u32,
        ty: Metadata,
        always_preserve: bool,
        flags: i32,
        align_in_bits: u32,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateAutoVariable(
                    self.0.as_ptr(),
                    scope.0.as_ptr(),
                    to_c_string(Some(name)).as_ptr(),
                    name.len(),
                    file.0.as_ptr(),
                    line_no,
                    ty.0.as_ptr(),
                    always_preserve as i32,
                    flags,
                    align_in_bits,
                )
            )
        })
    }

    pub fn create_parameter_variable(
        &self,
        scope: Metadata,
        name: &str,
        arg_no: u32,
        file: Metadata,
        line_no: u32,
        ty: Metadata,
        always_preserve: bool,
        flags: i32,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(LLVMDIBuilderCreateParameterVariable(
                self.0.as_ptr(),
                scope.0.as_ptr(),
                to_c_string(Some(name)).as_ptr(),
                name.len(),
                arg_no,
                file.0.as_ptr(),
                line_no,
                ty.0.as_ptr(),
                always_preserve as i32,
                flags
            ))
        })
    }

    pub fn get_or_create_type_array(
        &self,
        data: Vec<Metadata>,
        num_elements: usize,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(LLVMDIBuilderGetOrCreateTypeArray(
                self.0.as_ptr(),
                data.iter()
                    .map(|metadta| metadta.0.as_ptr())
                    .collect::<Vec<_>>()
                    .as_mut_ptr(),
                num_elements
            ))
        })
    }

    pub fn create_subroutine_type(
        &self,
        file: Metadata,
        parameter_types: Vec<Metadata>,
        flags: i32,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(LLVMDIBuilderCreateSubroutineType(
                self.0.as_ptr(),
                file.0.as_ptr(),
                parameter_types.iter()
                    .map(|metadata| metadata.0.as_ptr())
                    .collect::<Vec<_>>()
                    .as_mut_ptr(),
                parameter_types.len() as u32,
                flags
            ))
        })
    }

    pub fn create_macro(
        &self,
        parent_macro_file: Metadata,
        line: u32,
        record_type: DWARFMacinfoRecordType,
        name: &str,
        value: &str,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateMacro(
                    self.0.as_ptr(),
                    parent_macro_file.0.as_ptr(),
                    line,
                    transmute(record_type),
                    to_c_string(Some(name)).as_ptr(),
                    name.len(),
                    to_c_string(Some(value)).as_ptr(),
                    value.len()
                )
            )
        })
    }

    pub fn create_temp_macro_file(
        &self,
        parent_macro_file: Metadata,
        line: u32,
        file: Metadata,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateTempMacroFile(
                    self.0.as_ptr(),
                    parent_macro_file.0.as_ptr(),
                    line,
                    file.0.as_ptr()
                )
            )
        })
    }

    pub fn create_enumerator(
        &self,
        name: &str,
        value: i64,
        is_unsigned: bool,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateEnumerator(
                    self.0.as_ptr(),
                    to_c_string(Some(name)).as_ptr(),
                    name.len(),
                    value,
                    is_unsigned as i32
                )
            )
        })
    }

    pub fn create_enumeration_type(
        &self,
        scope: Metadata,
        name: &str,
        file: Metadata,
        line_number: u32,
        size_in_bits: u64,
        align_in_bits: u32,
        elements: Vec<Metadata>,
        class_ty: Metadata,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateEnumerationType(
                    self.0.as_ptr(),
                    scope.0.as_ptr(),
                    to_c_string(Some(name)).as_ptr(),
                    name.len(),
                    file.0.as_ptr(),
                    line_number,
                    size_in_bits,
                    align_in_bits,
                    elements.iter()
                        .map(|metadata| metadata.0.as_ptr())
                        .collect::<Vec<_>>()
                        .as_mut_ptr(),
                    elements.len() as u32,
                    class_ty.0.as_ptr()
                )
            )
        })
    }

    pub fn create_union_type(
        &self,
        scope: Metadata,
        name: &str,
        file: Metadata,
        line_number: u32,
        size_in_bits: u64,
        align_in_bits: u32,
        flags: i32,
        elements: Vec<Metadata>,
        run_time_lang: u32,
        unique_id: &str,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateUnionType(
                    self.0.as_ptr(),
                    scope.0.as_ptr(),
                    to_c_string(Some(name)).as_ptr(),
                    name.len(),
                    file.0.as_ptr(),
                    line_number,
                    size_in_bits,
                    align_in_bits,
                    flags,
                    elements.iter()
                        .map(|metadata| metadata.0.as_ptr())
                        .collect::<Vec<_>>()
                        .as_mut_ptr(),
                    elements.len() as u32,
                    run_time_lang,
                    to_c_string(Some(unique_id)).as_ptr(),
                    unique_id.len()
                )
            )
        })
    }

    pub fn create_array_type(
        &self,
        size: u64,
        align_in_bits: u32,
        ty: Metadata,
        subscripts: Vec<Metadata>,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateArrayType(
                    self.0.as_ptr(),
                    size,
                    align_in_bits,
                    ty.0.as_ptr(),
                    subscripts.iter()
                        .map(|metadata| metadata.0.as_ptr())
                        .collect::<Vec<_>>()
                        .as_mut_ptr(),
                    subscripts.len() as u32,
                )
            )
        })
    }

    pub fn create_vector_type(
        &self,
        size: u64,
        align_in_bits: u32,
        ty: Metadata,
        subscripts: Vec<Metadata>,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateVectorType(
                    self.0.as_ptr(),
                    size,
                    align_in_bits,
                    ty.0.as_ptr(),
                    subscripts.iter()
                        .map(|metadata| metadata.0.as_ptr())
                        .collect::<Vec<_>>()
                        .as_mut_ptr(),
                    subscripts.len() as u32,
                )
            )
        })
    }

    pub fn create_unspecified_type(&self, name: &str) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateUnspecifiedType(
                    self.0.as_ptr(),
                    to_c_string(Some(name)).as_ptr(),
                    name.len(),
                )
            )
        })
    }

    pub fn create_basic_type(
        &self,
        name: &str,
        size_in_bits: u64,
        encoding: u32,
        flags: i32,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateBasicType(
                    self.0.as_ptr(),
                    to_c_string(Some(name)).as_ptr(),
                    name.len(),
                    size_in_bits,
                    encoding,
                    flags
                )
            )
        })
    }

    pub fn create_pointer_type(
        &self,
        pointee_ty: Metadata,
        size_in_bits: u64,
        align_in_bits: u32,
        address_space: u32,
        name: &str,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreatePointerType(
                    self.0.as_ptr(),
                    pointee_ty.0.as_ptr(),
                    size_in_bits,
                    align_in_bits,
                    address_space,
                    to_c_string(Some(name)).as_ptr(),
                    name.len()
                )
            )
        })
    }

    pub fn create_struct_type(
        &self,
        scope: Metadata,
        name: &str,
        file: Metadata,
        line_number: u32,
        size_in_bits: u64,
        align_in_bits: u32,
        flags: i32,
        derived_from: Metadata,
        elements: Vec<Metadata>,
        run_time_lang: u32,
        vtable_holder: Metadata,
        unique_id: &str,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateStructType(
                    self.0.as_ptr(),
                    scope.0.as_ptr(),
                    to_c_string(Some(name)).as_ptr(),
                    name.len(),
                    file.0.as_ptr(),
                    line_number,
                    size_in_bits,
                    align_in_bits,
                    flags,
                    derived_from.0.as_ptr(),
                    elements.iter()
                        .map(|metadata| metadata.0.as_ptr())
                        .collect::<Vec<_>>()
                        .as_mut_ptr(),
                    elements.len() as u32,
                    run_time_lang,
                    vtable_holder.0.as_ptr(),
                    to_c_string(Some(unique_id)).as_ptr(),
                    unique_id.len()
                )
            )
        })
    }

    pub fn create_member_type(
        &self,
        scope: Metadata,
        name: &str,
        file: Metadata,
        line_no: u32,
        size_in_bits: u64,
        align_in_bits: u32,
        offset_in_bits: u64,
        flags: i32,
        ty: Metadata,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateMemberType(
                    self.0.as_ptr(),
                    scope.0.as_ptr(),
                    to_c_string(Some(name)).as_ptr(),
                    name.len(),
                    file.0.as_ptr(),
                    line_no,
                    size_in_bits,
                    align_in_bits,
                    offset_in_bits,
                    flags,
                    ty.0.as_ptr()
                )
            )
        })
    }

    pub fn create_static_member_type(
        &self,
        scope: Metadata,
        name: &str,
        file: Metadata,
        line_number: u32,
        ty: Metadata,
        flags: i32,
        constant_val: Value,
        align_in_bits: u32,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateStaticMemberType(
                    self.0.as_ptr(),
                    scope.0.as_ptr(),
                    to_c_string(Some(name)).as_ptr(),
                    name.len(),
                    file.0.as_ptr(),
                    line_number,
                    ty.0.as_ptr(),
                    flags,
                    constant_val.as_raw().as_ptr(),
                    align_in_bits
                )
            )
        })
    }

    pub fn create_member_pointer_type(
        &self,
        pointee_type: Metadata,
        class_type: Metadata,
        size_in_bits: u64,
        align_in_bits: u32,
        flags: i32,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateMemberPointerType(
                    self.0.as_ptr(),
                    pointee_type.0.as_ptr(),
                    class_type.0.as_ptr(),
                    size_in_bits,
                    align_in_bits,
                    flags
                )
            )
        })
    }

    pub fn create_objc_i_var(
        &self,
        name: &str,
        file: Metadata,
        line_no: u32,
        size_in_bits: u64,
        align_in_bits: u32,
        offset_in_bits: u64,
        flags: i32,
        ty: Metadata,
        property_node: Metadata,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateObjCIVar(
                    self.0.as_ptr(),
                    to_c_string(Some(name)).as_ptr(),
                    name.len(),
                    file.0.as_ptr(),
                    line_no,
                    size_in_bits,
                    align_in_bits,
                    offset_in_bits,
                    flags,
                    ty.0.as_ptr(),
                    property_node.0.as_ptr()
                )
            )
        })
    }

    pub fn create_objc_property(
        &self,
        name: &str,
        file: Metadata,
        line_no: u32,
        getter_name: &str,
        setter_name: &str,
        property_attributes: u32,
        ty: Metadata,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateObjCProperty(
                    self.0.as_ptr(),
                    to_c_string(Some(name)).as_ptr(),
                    name.len(),
                    file.0.as_ptr(),
                    line_no,
                    to_c_string(Some(getter_name)).as_ptr(),
                    getter_name.len(),
                    to_c_string(Some(setter_name)).as_ptr(),
                    setter_name.len(),
                    property_attributes,
                    ty.0.as_ptr()
                )
            )
        })
    }

    pub fn create_object_pointer_type(&self, ty: Metadata, ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateObjectPointerType(self.0.as_ptr(), ty.0.as_ptr())
            )
        })
    }

    pub fn create_qualified_type(&self, tag: u32, ty: Metadata) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateQualifiedType(self.0.as_ptr(), tag, ty.0.as_ptr())
            )
        })
    }

    pub fn create_reference_type(&self, tag: u32, ty: Metadata, ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateReferenceType(self.0.as_ptr(), tag, ty.0.as_ptr())
            )
        })
    }


    pub fn create_null_ptr_type(&self) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateNullPtrType(self.0.as_ptr())
            )
        })
    }

    pub fn create_typedef(
        &self,
        ty: Metadata,
        name: &str,
        file: Metadata,
        line_no: u32,
        scope: Metadata,
        align_in_bits: u32,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateTypedef(
                    self.0.as_ptr(),
                    ty.0.as_ptr(),
                    to_c_string(Some(name)).as_ptr(),
                    name.len(),
                    file.0.as_ptr(),
                    line_no,
                    scope.0.as_ptr(),
                    align_in_bits
                )
            )
        })
    }

    pub fn create_inheritance(
        &self,
        ty: Metadata,
        base_ty: Metadata,
        base_offset: u64,
        vbptr_offset: u32,
        flags: i32,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateInheritance(
                    self.0.as_ptr(),
                    ty.0.as_ptr(),
                    base_ty.0.as_ptr(),
                    base_offset,
                    vbptr_offset,
                    flags
                )
            )
        })
    }

    pub fn create_forward_decl(
        &self,
        tag: u32,
        name: &str,
        scope: Metadata,
        file: Metadata,
        line: u32,
        runtime_lang: u32,
        size_in_bits: u64,
        align_in_bits: u32,
        unique_identifier: &str,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateForwardDecl(
                    self.0.as_ptr(),
                    tag,
                    to_c_string(Some(name)).as_ptr(),
                    name.len(),
                    scope.0.as_ptr(),
                    file.0.as_ptr(),
                    line,
                    runtime_lang,
                    size_in_bits,
                    align_in_bits,
                    to_c_string(Some(unique_identifier)).as_ptr(),
                    unique_identifier.len()
                )
            )
        })
    }

    pub fn create_replaceable_composite_type(
        &self,
        tag: u32,
        name: &str,
        scope: Metadata,
        file: Metadata,
        line: u32,
        runtime_lang: u32,
        size_in_bits: u64,
        align_in_bits: u32,
        flags: i32,
        unique_identifier: &str,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateReplaceableCompositeType(
                    self.0.as_ptr(),
                    tag,
                    to_c_string(Some(name)).as_ptr(),
                    name.len(),
                    scope.0.as_ptr(),
                    file.0.as_ptr(),
                    line,
                    runtime_lang,
                    size_in_bits,
                    align_in_bits,
                    flags,
                    to_c_string(Some(unique_identifier)).as_ptr(),
                    unique_identifier.len()
                )
            )
        })
    }

    pub fn create_bit_field_member_type(
        &self,
        scope: Metadata,
        name: &str,
        file: Metadata,
        line_number: u32,
        size_in_bits: u64,
        offset_in_bits: u64,
        storage_offset_in_bits: u64,
        flags: i32,
        ty: Metadata,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateBitFieldMemberType(
                    self.0.as_ptr(),
                    scope.0.as_ptr(),
                    to_c_string(Some(name)).as_ptr(),
                    name.len(),
                    file.0.as_ptr(),
                    line_number,
                    size_in_bits,
                    offset_in_bits,
                    storage_offset_in_bits,
                    flags,
                    ty.0.as_ptr()
                )
            )
        })
    }

    pub fn create_class_type(
        &self,
        scope: Metadata,
        name: &str,
        file: Metadata,
        line_number: u32,
        size_in_bits: u64,
        align_in_bits: u32,
        offset_in_bits: u64,
        flags: i32,
        derived_from: Metadata,
        elements: Vec<Metadata>,
        vtable_holder: Metadata,
        template_params_node: Metadata,
        unique_identifier: &str,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateClassType(
                    self.0.as_ptr(),
                    scope.0.as_ptr(),
                    to_c_string(Some(name)).as_ptr(),
                    name.len(),
                    file.0.as_ptr(),
                    line_number,
                    size_in_bits,
                    align_in_bits,
                    offset_in_bits,
                    flags,
                    derived_from.0.as_ptr(),
                    elements.iter()
                        .map(|metadata| metadata.0.as_ptr())
                        .collect::<Vec<_>>()
                        .as_mut_ptr(),
                    elements.len() as u32,
                    vtable_holder.0.as_ptr(),
                    template_params_node.0.as_ptr(),
                    to_c_string(Some(unique_identifier)).as_ptr(),
                    unique_identifier.len()
                )
            )
        })
    }

    pub fn create_artificial_type(&self, ty: Metadata) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateArtificialType(
                    self.0.as_ptr(), ty.0.as_ptr()
                )
            )
        })
    }

    pub fn get_or_create_subrange(&self, lower_bound: i64, count: i64) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderGetOrCreateSubrange(
                    self.0.as_ptr(), lower_bound, count
                )
            )
        })
    }

    pub fn get_or_create_array(&self, data: Vec<Metadata>) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderGetOrCreateArray(
                    self.0.as_ptr(),
                    data.iter()
                        .map(|metadata| metadata.0.as_ptr())
                        .collect::<Vec<_>>()
                        .as_mut_ptr(),
                    data.len(),
                )
            )
        })
    }

    pub fn create_expression(&self, mut addr: Vec<u64>) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateExpression(
                    self.0.as_ptr(),
                    addr.as_mut_ptr(),
                    addr.len()
                )
            )
        })
    }

    pub fn create_constant_value_expression(&self, value: u64) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateConstantValueExpression(self.0.as_ptr(), value)
            )
        })
    }

    pub fn create_global_variable_expression(
        &self,
        scope: Metadata,
        name: &str,
        linkage: &str,
        file: Metadata,
        line_no: u32,
        ty: Metadata,
        local_to_unit: bool,
        expr: Metadata,
        decl: Metadata,
        align_in_bits: u32,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateGlobalVariableExpression(
                    self.0.as_ptr(),
                    scope.0.as_ptr(),
                    to_c_string(Some(name)).as_ptr(),
                    name.len(),
                    to_c_string(Some(linkage)).as_ptr(),
                    linkage.len(),
                    file.0.as_ptr(),
                    line_no,
                    ty.0.as_ptr(),
                    local_to_unit as i32,
                    expr.0.as_ptr(),
                    decl.0.as_ptr(),
                    align_in_bits
                )
            )
        })
    }

    pub fn create_file(
        &self,
        filename: &str,
        directory: &str,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateFile(
                    self.0.as_ptr(),
                    to_c_string(Some(filename)).as_ptr(),
                    filename.len(),
                    to_c_string(Some(directory)).as_ptr(),
                    directory.len(),
                )
            )
        })
    }

    pub fn create_module(
        &self,
        parent_scope: Metadata,
        name: &str,
        config_macros: &str,
        include_path: &str,
        apinotes_file: &str,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateModule(
                    self.0.as_ptr(),
                    parent_scope.0.as_ptr(),
                    to_c_string(Some(name)).as_ptr(),
                    name.len(),
                    to_c_string(Some(config_macros)).as_ptr(),
                    config_macros.len(),
                    to_c_string(Some(include_path)).as_ptr(),
                    include_path.len(),
                    to_c_string(Some(apinotes_file)).as_ptr(),
                    apinotes_file.len(),
                )
            )
        })
    }

    pub fn create_name_space(
        &self,
        parent_scope: Metadata,
        name: &str,
        export_symbols: bool,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateNameSpace(
                    self.0.as_ptr(),
                    parent_scope.0.as_ptr(),
                    to_c_string(Some(name)).as_ptr(),
                    name.len(),
                    export_symbols as i32
                )
            )
        })
    }

    pub fn create_function(
        &self,
        scope: Metadata,
        name: &str,
        linkage_name: &str,
        file: Metadata,
        line_no: u32,
        ty: Metadata,
        is_local_to_unit: bool,
        is_definition: bool,
        scope_line: u32,
        flags: i32,
        is_optimized: bool,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateFunction(
                    self.0.as_ptr(),
                    scope.0.as_ptr(),
                    to_c_string(Some(name)).as_ptr(),
                    name.len(),
                    to_c_string(Some(linkage_name)).as_ptr(),
                    linkage_name.len(),
                    file.0.as_ptr(),
                    line_no,
                    ty.0.as_ptr(),
                    is_local_to_unit as i32,
                    is_definition as i32,
                    scope_line,
                    flags,
                    is_optimized as i32
                )
            )
        })
    }

    pub fn create_lexical_block(
        &self,
        scope: Metadata,
        file: Metadata,
        line: u32,
        column: u32,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateLexicalBlock(
                    self.0.as_ptr(),
                    scope.0.as_ptr(),
                    file.0.as_ptr(),
                    line,
                    column
                )
            )
        })
    }

    pub fn create_lexical_block_file(
        &self,
        scope: Metadata,
        file: Metadata,
        discriminator: u32,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateLexicalBlockFile(
                    self.0.as_ptr(),
                    scope.0.as_ptr(),
                    file.0.as_ptr(),
                    discriminator
                )
            )
        })
    }

    pub fn create_imported_module_from_namespace(
        &self,
        scope: Metadata,
        ns: Metadata,
        file: Metadata,
        line: u32,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateImportedModuleFromNamespace(
                    self.0.as_ptr(),
                    scope.0.as_ptr(),
                    ns.0.as_ptr(),
                    file.0.as_ptr(),
                    line
                )
            )
        })
    }

    pub fn create_imported_module_from_alias(
        &self,
        scope: Metadata,
        imported_entity: Metadata,
        file: Metadata,
        line: u32,
        elements: Vec<Metadata>,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateImportedModuleFromAlias(
                    self.0.as_ptr(),
                    scope.0.as_ptr(),
                    imported_entity.0.as_ptr(),
                    file.0.as_ptr(),
                    line,
                    elements.iter()
                        .map(|metadata| metadata.0.as_ptr())
                        .collect::<Vec<_>>()
                        .as_mut_ptr(),
                    elements.len() as u32
                )
            )
        })
    }

    pub fn create_imported_module_from_module(
        &self,
        scope: Metadata,
        m: Metadata,
        file: Metadata,
        line: u32,
        elements: Vec<Metadata>,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateImportedModuleFromModule(
                    self.0.as_ptr(),
                    scope.0.as_ptr(),
                    m.0.as_ptr(),
                    file.0.as_ptr(),
                    line,
                    elements.iter()
                        .map(|metadata| metadata.0.as_ptr())
                        .collect::<Vec<_>>()
                        .as_mut_ptr(),
                    elements.len() as u32
                )
            )
        })
    }

    pub fn create_imported_declaration(
        &self,
        scope: Metadata,
        decl: Metadata,
        file: Metadata,
        line: u32,
        name: &str,
        elements: Vec<Metadata>,
    ) -> Metadata {
        Metadata::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMDIBuilderCreateImportedDeclaration(
                    self.0.as_ptr(),
                    scope.0.as_ptr(),
                    decl.0.as_ptr(),
                    file.0.as_ptr(),
                    line,
                    to_c_string(Some(name)).as_ptr(),
                    name.len(),
                    elements.iter()
                        .map(|metadata| metadata.0.as_ptr())
                        .collect::<Vec<_>>()
                        .as_mut_ptr(),
                    elements.len() as u32,
                )
            )
        })
    }
}

impl Drop for DIBuilder {
    fn drop(&mut self) {
        unsafe {
            LLVMDIBuilderFinalize(self.0.as_ptr());
            LLVMDisposeDIBuilder(self.0.as_ptr());
        }
    }
}