## Comparison to LLVM

Dorian has 2 levels of abstraction, an LLVM-analogous yet library-compatible abstraction, and then a high-level
intuitive abstraction that is appropriate for most applications.

Dorian does not acknowledge any deprecated LLVM functions. Moreover, Dorian does not acknowledge functions
that don't require an LLVM context, if a similar function exists that does require a LLVM context.

Listed below are all the modules that LLVM features, and the Dorian equivalent, if such exists.

This document is useful for migrating 

<details>
  <summary>analysis</summary>

✅ `enum LLVMVerifierFailureAction` as `dorian::llvm::VerifierFailureAction`

✅ `fn LLVMVerifyFunction` as `dorian::llvm::fun::Fun::verify`

✅ `fn LLVMVerifyModule` as `dorian::llvm::module::Module::verify`

✅ `fn LLVMViewFunctionCFG` as `dorian::llvm::fun::Fun::view_cfg`

✅ `fn LLVMViewFunctionCFGOnly` as `dorian::llvm::fun::Fun::view_cfg_only`
</details>
<details>
  <summary>bit_reader</summary>

❌ `fn LLVMGetBitcodeModule2` will not be supported.

✅ `fn LLVMGetBitcodeModuleInContext2`

❌ `fn LLVMParseBitcode2` will not be supported.

❌ `fn LLVMParseBitcodeInContext2`
</details>
<details>
  <summary>bit_writer</summary>

✅ `fn LLVMWriteBitcodeToFD` as `dorian::llvm::module::Module::write_bitcode_to_file_descriptor`

✅ `fn LLVMWriteBitcodeToFile` as `dorian::llvm::module::Module::write_bitcode_to_file`

✅ `fn LLVMWriteBitcodeToMemoryBuffer` as `dorian::llvm::module::Module::write_bitcode_to_memory_buffer`
</details>
<details>
  <summary>comdat</summary>

✅ `enum LLVMComdatSelectionKind`

✅ `fn LLVMGetComdat`

✅ `fn LLVMGetComdatSelectionKind`

✅ `fn LLVMGetOrInsertComdat`

✅ `fn LLVMSetComdat`

✅ `fn LLVMSetComdatSelectionKind`
</details>

<details>
  <summary>core</summary>

❌ `fn LLVMAddAlias2`

❌ `fn LLVMAddAttributeAtIndex`

❌ `fn LLVMAddCallSiteAttribute`

❌ `fn LLVMAddCase`

❌ `fn LLVMAddClause`

❌ `fn LLVMAddDestination`

✅ `fn LLVMAddFunction` as `dorian::module::Module::add_fun`

❌ `fn LLVMAddGlobal`

❌ `fn LLVMAddGlobalIFunc`

❌ `fn LLVMAddGlobalInAddressSpace`

❌ `fn LLVMAddHandler`

❌ `fn LLVMAddIncoming`

❌ `fn LLVMAddMetadataToInst`

❌ `fn LLVMAddModuleFlag`


❌ `fn LLVMAddNamedMetadataOperand`

❌ `fn LLVMAddTargetDependentFunctionAttr`

❌ `fn LLVMAliasGetAliasee	`

❌ `fn LLVMAliasSetAliasee`

❌ `fn LLVMAlignOf`

❌ `fn LLVMAppendBasicBlock`

❌ `fn LLVMAppendBasicBlockInContext`

❌ `fn LLVMAppendExistingBasicBlock`

❌ `fn LLVMAppendModuleInlineAsm`

❌ `fn LLVMArrayType`

❌ `fn LLVMBFloatType`

❌ `fn LLVMBFloatTypeInContext`

❌ `fn LLVMBasicBlockAsValue`

❌ `fn LLVMBlockAddress`

❌ `fn LLVMBuildAShr`

❌ `fn LLVMBuildAdd`

❌ `fn LLVMBuildAddrSpaceCast`

❌ `fn LLVMBuildAggregateRet`

❌ `fn LLVMBuildAlloca`

❌ `fn LLVMBuildAnd`

❌ `fn LLVMBuildArrayAlloca`

❌ `fn LLVMBuildArrayMalloc`

❌ `fn LLVMBuildAtomicCmpXchg`

❌ `fn LLVMBuildAtomicRMW`

❌ `fn LLVMBuildBinOp`

❌ `fn LLVMBuildBitCast`

❌ `fn LLVMBuildBr`

❌ `fn LLVMBuildCall2`

❌ `fn LLVMBuildCast`

❌ `fn LLVMBuildCatchPad`

❌ `fn LLVMBuildCatchRet`

❌ `fn LLVMBuildCatchSwitch`

❌ `fn LLVMBuildCleanupPad`

❌ `fn LLVMBuildCleanupRet`

❌ `fn LLVMBuildCondBr`

❌ `fn LLVMBuildExactSDiv`

❌ `fn LLVMBuildExactUDiv`

❌ `fn LLVMBuildExtractElement`

❌ `fn LLVMBuildExtractValue`

❌ `fn LLVMBuildFAdd`

❌ `fn LLVMBuildFCmp`

❌ `fn LLVMBuildFDiv`

❌ `fn LLVMBuildFMul`

❌ `fn LLVMBuildFNeg`

❌ `fn LLVMBuildFPCast`

❌ `fn LLVMBuildFPExt`

❌ `fn LLVMBuildFPToSI`

❌ `fn LLVMBuildFPToUI`

❌ `fn LLVMBuildFPTrunc`

❌ `fn LLVMBuildFRem`

❌ `fn LLVMBuildFSub`

❌ `fn LLVMBuildFence`

❌ `fn LLVMBuildFree`

❌ `fn LLVMBuildFreeze`

❌ `fn LLVMBuildGEP2`

❌ `fn LLVMBuildGlobalString`

❌ `fn LLVMBuildGlobalStringPtr`

❌ `fn LLVMBuildICmp`

❌ `fn LLVMBuildInBoundsGEP2`

❌ `fn LLVMBuildIndirectBr`

❌ `fn LLVMBuildInsertElement`

❌ `fn LLVMBuildInsertValue`

❌ `fn LLVMBuildIntCast`

❌ `fn LLVMBuildIntCast2`

❌ `fn LLVMBuildIntToPtr`

❌ `fn LLVMBuildInvoke2`

❌ `fn LLVMBuildIsNotNull`

❌ `fn LLVMBuildIsNull`

❌ `fn LLVMBuildLShr`

❌ `fn LLVMBuildLandingPad`

❌ `fn LLVMBuildLoad2`

❌ `fn LLVMBuildMalloc`

❌ `fn LLVMBuildMemCpy`

❌ `fn LLVMBuildMemMove`

❌ `fn LLVMBuildMemSet`

❌ `fn LLVMBuildMul`

❌ `fn LLVMBuildNSWAdd`

❌ `fn LLVMBuildNSWMul`

❌ `fn LLVMBuildNSWNeg`

❌ `fn LLVMBuildNSWSub`

❌ `fn LLVMBuildNUWAdd`

❌ `fn LLVMBuildNUWMul`

❌ `fn LLVMBuildNUWNeg`

❌ `fn LLVMBuildNUWSub`

❌ `fn LLVMBuildNeg`

❌ `fn LLVMBuildNot`

❌ `fn LLVMBuildOr`

❌ `fn LLVMBuildPhi`

❌ `fn LLVMBuildPointerCast`

❌ `fn LLVMBuildPtrDiff2`

❌ `fn LLVMBuildPtrToInt`

❌ `fn LLVMBuildResume`

❌ `fn LLVMBuildRet`

❌ `fn LLVMBuildRetVoid`

❌ `fn LLVMBuildSDiv`

❌ `fn LLVMBuildSExt`

❌ `fn LLVMBuildSExtOrBitCast`

❌ `fn LLVMBuildSIToFP`

❌ `fn LLVMBuildSRem`

❌ `fn LLVMBuildSelect`

❌ `fn LLVMBuildShl`

❌ `fn LLVMBuildShuffleVector`

❌ `fn LLVMBuildStore`

❌ `fn LLVMBuildStructGEP2`

❌ `fn LLVMBuildSub`

❌ `fn LLVMBuildSwitch`

❌ `fn LLVMBuildTrunc`

❌ `fn LLVMBuildTruncOrBitCast`

❌ `fn LLVMBuildUDiv`

❌ `fn LLVMBuildUIToFP`

❌ `fn LLVMBuildURem`

❌ `fn LLVMBuildUnreachable`

❌ `fn LLVMBuildVAArg`

❌ `fn LLVMBuildXor`

❌ `fn LLVMBuildZExt`

❌ `fn LLVMBuildZExtOrBitCast`

❌ `fn LLVMBuilderGetDefaultFPMathTag`

❌ `fn LLVMBuilderSetDefaultFPMathTag`

❌ `fn LLVMClearInsertionPosition`

❌ `fn LLVMCloneModule`

❌ `fn LLVMConstAShr`

❌ `fn LLVMConstAdd`

❌ `fn LLVMConstAddrSpaceCast`

❌ `fn LLVMConstAllOnes`

❌ `fn LLVMConstAnd`

❌ `fn LLVMConstArray`

❌ `fn LLVMConstBitCast`

❌ `fn LLVMConstExactSDiv`

❌ `fn LLVMConstExactUDiv`

❌ `fn LLVMConstExtractElement`

❌ `fn LLVMConstExtractValue`

❌ `fn LLVMConstFAdd`

❌ `fn LLVMConstFCmp`

❌ `fn LLVMConstFDiv`

❌ `fn LLVMConstFMul`

❌ `fn LLVMConstFNeg`

❌ `fn LLVMConstFPCast`

❌ `fn LLVMConstFPExt`

❌ `fn LLVMConstFPToSI`

❌ `fn LLVMConstFPToUI`

❌ `fn LLVMConstFPTrunc`

❌ `fn LLVMConstFRem`

❌ `fn LLVMConstFSub`

❌ `fn LLVMConstGEP2`

❌ `fn LLVMConstICmp`

❌ `fn LLVMConstInBoundsGEP2`

❌ `fn LLVMConstInsertElement`

❌ `fn LLVMConstInsertValue`

❌ `fn LLVMConstInt`

❌ `fn LLVMConstIntCast`

❌ `fn LLVMConstIntGetSExtValue`

❌ `fn LLVMConstIntGetZExtValue`

❌ `fn LLVMConstIntOfArbitraryPrecision`

❌ `fn LLVMConstIntOfString`

❌ `fn LLVMConstIntOfStringAndSize`

❌ `fn LLVMConstIntToPtr`

❌ `fn LLVMConstLShr`

❌ `fn LLVMConstMul`

❌ `fn LLVMConstNSWAdd`

❌ `fn LLVMConstNSWMul`

❌ `fn LLVMConstNSWNeg`

❌ `fn LLVMConstNSWSub`

❌ `fn LLVMConstNUWAdd`

❌ `fn LLVMConstNUWMul`

❌ `fn LLVMConstNUWNeg`

❌ `fn LLVMConstNUWSub`

❌ `fn LLVMConstNamedStruct`

❌ `fn LLVMConstNeg`

❌ `fn LLVMConstNot`

❌ `fn LLVMConstNull`

❌ `fn LLVMConstOr`

❌ `fn LLVMConstPointerCast`

❌ `fn LLVMConstPointerNull`

❌ `fn LLVMConstPtrToInt`

❌ `fn LLVMConstReal`

❌ `fn LLVMConstRealGetDouble`

❌ `fn LLVMConstRealOfString`

❌ `fn LLVMConstRealOfStringAndSize`

❌ `fn LLVMConstSDiv`

❌ `fn LLVMConstSExt`

❌ `fn LLVMConstSExtOrBitCast`

❌ `fn LLVMConstSIToFP`

❌ `fn LLVMConstSRem`

❌ `fn LLVMConstSelect`

❌ `fn LLVMConstShl`

❌ `fn LLVMConstShuffleVector`

❌ `fn LLVMConstString`

❌ `fn LLVMConstStringInContext`

❌ `fn LLVMConstStruct`

❌ `fn LLVMConstStructInContext`

❌ `fn LLVMConstSub`

❌ `fn LLVMConstTrunc`

❌ `fn LLVMConstTruncOrBitCast`

❌ `fn LLVMConstUDiv`

❌ `fn LLVMConstUIToFP`

❌ `fn LLVMConstURem`

❌ `fn LLVMConstVector`

❌ `fn LLVMConstXor`

❌ `fn LLVMConstZExt`

❌ `fn LLVMConstZExtOrBitCast`

❌ `fn LLVMContextCreate`

❌ `fn LLVMContextDispose`

❌ `fn LLVMContextGetDiagnosticContext`

❌ `fn LLVMContextGetDiagnosticHandler`

❌ `fn LLVMContextSetDiagnosticHandler`

❌ `fn LLVMContextSetDiscardValueNames`

❌ `fn LLVMContextSetYieldCallback`

❌ `fn LLVMContextShouldDiscardValueNames`

❌ `fn LLVMCopyModuleFlagsMetadata`

❌ `fn LLVMCountBasicBlocks`

❌ `fn LLVMCountIncoming`

❌ `fn LLVMCountParamTypes`

❌ `fn LLVMCountParams`

❌ `fn LLVMCountStructElementTypes`

❌ `fn LLVMCreateBasicBlockInContext`

❌ `fn LLVMCreateBuilder`

❌ `fn LLVMCreateBuilderInContext`

❌ `fn LLVMCreateEnumAttribute`

❌ `fn LLVMCreateFunctionPassManager`

❌ `fn LLVMCreateFunctionPassManagerForModule`

❌ `fn LLVMCreateMemoryBufferWithContentsOfFile`

❌ `fn LLVMCreateMemoryBufferWithMemoryRange`

❌ `fn LLVMCreateMemoryBufferWithMemoryRangeCopy`

❌ `fn LLVMCreateMemoryBufferWithSTDIN`

❌ `fn LLVMCreateMessage`

❌ `fn LLVMCreateModuleProviderForExistingModule`

❌ `fn LLVMCreatePassManager`

❌ `fn LLVMCreateStringAttribute`

❌ `fn LLVMCreateTypeAttribute`

❌ `fn LLVMDeleteBasicBlock`

❌ `fn LLVMDeleteFunction`

❌ `fn LLVMDeleteGlobal`

❌ `fn LLVMDisposeBuilder`

❌ `fn LLVMDisposeMemoryBuffer`

❌ `fn LLVMDisposeMessage`

❌ `fn LLVMDisposeModule`

❌ `fn LLVMDisposeModuleFlagsMetadata`

❌ `fn LLVMDisposeModuleProvider`

❌ `fn LLVMDisposePassManager`

❌ `fn LLVMDisposeValueMetadataEntries`

❌ `fn LLVMDoubleType`

❌ `fn LLVMDoubleTypeInContext`

❌ `fn LLVMDumpModule`

❌ `fn LLVMDumpType`

❌ `fn LLVMDumpValue`

❌ `fn LLVMEraseGlobalIFunc`

❌ `fn LLVMFP128Type`

❌ `fn LLVMFP128TypeInContext`

❌ `fn LLVMFinalizeFunctionPassManager`

❌ `fn LLVMFloatType`

❌ `fn LLVMFloatTypeInContext`

❌ `fn LLVMFunctionType`

❌ `fn LLVMGetAlignment`

❌ `fn LLVMGetAllocatedType`

❌ `fn LLVMGetArgOperand`

❌ `fn LLVMGetArrayLength`

❌ `fn LLVMGetAsString`

❌ `fn LLVMGetAtomicRMWBinOp`

❌ `fn LLVMGetAttributeCountAtIndex`

❌ `fn LLVMGetAttributesAtIndex`

❌ `fn LLVMGetBasicBlockName`

❌ `fn LLVMGetBasicBlockParent`

❌ `fn LLVMGetBasicBlockTerminator`

❌ `fn LLVMGetBasicBlocks`

❌ `fn LLVMGetBufferSize`

❌ `fn LLVMGetBufferStart`

❌ `fn LLVMGetCallSiteAttributeCount`

❌ `fn LLVMGetCallSiteAttributes`

❌ `fn LLVMGetCallSiteEnumAttribute`

❌ `fn LLVMGetCallSiteStringAttribute`

❌ `fn LLVMGetCalledFunctionType`

❌ `fn LLVMGetCalledValue`

❌ `fn LLVMGetClause`

❌ `fn LLVMGetCmpXchgFailureOrdering`

❌ `fn LLVMGetCmpXchgSuccessOrdering`

❌ `fn LLVMGetCondition`

❌ `fn LLVMGetConstOpcode`

❌ `fn LLVMGetCurrentDebugLocation`

❌ `fn LLVMGetCurrentDebugLocation2`

❌ `fn LLVMGetDLLStorageClass`

❌ `fn LLVMGetDataLayoutStr`

❌ `fn LLVMGetDebugLocColumn`

❌ `fn LLVMGetDebugLocDirectory`

❌ `fn LLVMGetDebugLocFilename`

❌ `fn LLVMGetDebugLocLine`

❌ `fn LLVMGetDiagInfoDescription`

❌ `fn LLVMGetDiagInfoSeverity`

❌ `fn LLVMGetElementAsConstant`

❌ `fn LLVMGetElementType`

❌ `fn LLVMGetEntryBasicBlock`

❌ `fn LLVMGetEnumAttributeAtIndex`

❌ `fn LLVMGetEnumAttributeKind`

❌ `fn LLVMGetEnumAttributeKindForName`

❌ `fn LLVMGetEnumAttributeValue`

❌ `fn LLVMGetFCmpPredicate`

❌ `fn LLVMGetFirstBasicBlock`

❌ `fn LLVMGetFirstFunction`

❌ `fn LLVMGetFirstGlobal`

❌ `fn LLVMGetFirstGlobalAlias`

❌ `fn LLVMGetFirstGlobalIFunc`

❌ `fn LLVMGetFirstInstruction`

❌ `fn LLVMGetFirstNamedMetadata`

❌ `fn LLVMGetFirstParam`

❌ `fn LLVMGetFirstUse`

❌ `fn LLVMGetFunctionCallConv`

❌ `fn LLVMGetGC`

❌ `fn LLVMGetGEPSourceElementType`

❌ `fn LLVMGetGlobalContext`

❌ `fn LLVMGetGlobalIFuncResolver`

❌ `fn LLVMGetGlobalParent`

❌ `fn LLVMGetGlobalPassRegistry`

❌ `fn LLVMGetHandlers`

❌ `fn LLVMGetICmpPredicate`

❌ `fn LLVMGetIncomingBlock`

❌ `fn LLVMGetIncomingValue`

❌ `fn LLVMGetIndices`

❌ `fn LLVMGetInitializer`

❌ `fn LLVMGetInlineAsm`

❌ `fn LLVMGetInsertBlock`

❌ `fn LLVMGetInstructionCallConv`

❌ `fn LLVMGetInstructionOpcode`

❌ `fn LLVMGetInstructionParent`

❌ `fn LLVMGetIntTypeWidth`

❌ `fn LLVMGetIntrinsicDeclaration`

❌ `fn LLVMGetIntrinsicID`

❌ `fn LLVMGetLastBasicBlock`

❌ `fn LLVMGetLastEnumAttributeKind`

❌ `fn LLVMGetLastFunction`

❌ `fn LLVMGetLastGlobal`

❌ `fn LLVMGetLastGlobalAlias`

❌ `fn LLVMGetLastGlobalIFunc`

❌ `fn LLVMGetLastInstruction`

❌ `fn LLVMGetLastNamedMetadata`

❌ `fn LLVMGetLastParam`

❌ `fn LLVMGetLinkage`

❌ `fn LLVMGetMDKindID`

❌ `fn LLVMGetMDKindIDInContext`

❌ `fn LLVMGetMDNodeNumOperands`

❌ `fn LLVMGetMDNodeOperands`

❌ `fn LLVMGetMDString`

❌ `fn LLVMGetMaskValue`

❌ `fn LLVMGetMetadata`

❌ `fn LLVMGetModuleContext`

❌ `fn LLVMGetModuleFlag`

❌ `fn LLVMGetModuleIdentifier`

❌ `fn LLVMGetModuleInlineAsm`

❌ `fn LLVMGetNamedFunction`

❌ `fn LLVMGetNamedGlobal`

❌ `fn LLVMGetNamedGlobalAlias`

❌ `fn LLVMGetNamedGlobalIFunc`

❌ `fn LLVMGetNamedMetadata`

❌ `fn LLVMGetNamedMetadataName`

❌ `fn LLVMGetNamedMetadataNumOperands`

❌ `fn LLVMGetNamedMetadataOperands`

❌ `fn LLVMGetNextBasicBlock`

❌ `fn LLVMGetNextFunction`

❌ `fn LLVMGetNextGlobal`

❌ `fn LLVMGetNextGlobalAlias`

❌ `fn LLVMGetNextGlobalIFunc`

❌ `fn LLVMGetNextInstruction`

❌ `fn LLVMGetNextNamedMetadata`

❌ `fn LLVMGetNextParam`

❌ `fn LLVMGetNextUse`

❌ `fn LLVMGetNormalDest`

❌ `fn LLVMGetNumArgOperands`

❌ `fn LLVMGetNumClauses`

❌ `fn LLVMGetNumContainedTypes`

❌ `fn LLVMGetNumHandlers`

❌ `fn LLVMGetNumIndices`

❌ `fn LLVMGetNumMaskElements`

❌ `fn LLVMGetNumOperands`

❌ `fn LLVMGetNumSuccessors`

❌ `fn LLVMGetOperand`

❌ `fn LLVMGetOperandUse`

❌ `fn LLVMGetOrInsertNamedMetadata`

❌ `fn LLVMGetOrdering`

❌ `fn LLVMGetParam`

❌ `fn LLVMGetParamParent`

❌ `fn LLVMGetParamTypes`

❌ `fn LLVMGetParams`

❌ `fn LLVMGetParentCatchSwitch`

❌ `fn LLVMGetPersonalityFn`

❌ `fn LLVMGetPointerAddressSpace`

❌ `fn LLVMGetPoison`

❌ `fn LLVMGetPreviousBasicBlock`

❌ `fn LLVMGetPreviousFunction`

❌ `fn LLVMGetPreviousGlobal`

❌ `fn LLVMGetPreviousGlobalAlias`

❌ `fn LLVMGetPreviousGlobalIFunc`

❌ `fn LLVMGetPreviousInstruction`

❌ `fn LLVMGetPreviousNamedMetadata`

❌ `fn LLVMGetPreviousParam`

❌ `fn LLVMGetReturnType`

❌ `fn LLVMGetSection`

❌ `fn LLVMGetSourceFileName`

❌ `fn LLVMGetStringAttributeAtIndex`

❌ `fn LLVMGetStringAttributeKind`

❌ `fn LLVMGetStringAttributeValue`

❌ `fn LLVMGetStructElementTypes`

❌ `fn LLVMGetStructName`

❌ `fn LLVMGetSubtypes`

❌ `fn LLVMGetSuccessor`

❌ `fn LLVMGetSwitchDefaultDest`

❌ `fn LLVMGetTarget`

❌ `fn LLVMGetThreadLocalMode`

❌ `fn LLVMGetTypeAttributeValue`

❌ `fn LLVMGetTypeByName2`

❌ `fn LLVMGetTypeContext`

❌ `fn LLVMGetTypeKind`

❌ `fn LLVMGetUndef`

❌ `fn LLVMGetUndefMaskElem`

❌ `fn LLVMGetUnnamedAddress`

❌ `fn LLVMGetUnwindDest`

❌ `fn LLVMGetUsedValue`

❌ `fn LLVMGetUser`

❌ `fn LLVMGetValueKind`

❌ `fn LLVMGetValueName2`

❌ `fn LLVMGetVectorSize`

❌ `fn LLVMGetVisibility`

❌ `fn LLVMGetVolatile`

❌ `fn LLVMGetWeak`

❌ `fn LLVMGlobalClearMetadata`

❌ `fn LLVMGlobalCopyAllMetadata`

❌ `fn LLVMGlobalEraseMetadata`

❌ `fn LLVMGlobalGetValueType`

❌ `fn LLVMGlobalSetMetadata`

❌ `fn LLVMHalfType`

❌ `fn LLVMHalfTypeInContext`

❌ `fn LLVMHasMetadata`

❌ `fn LLVMHasPersonalityFn`

❌ `fn LLVMInitializeFunctionPassManager`

❌ `fn LLVMInsertBasicBlock`

❌ `fn LLVMInsertBasicBlockInContext`

❌ `fn LLVMInsertExistingBasicBlockAfterInsertBlock`

❌ `fn LLVMInsertIntoBuilder`

❌ `fn LLVMInsertIntoBuilderWithName`

❌ `fn LLVMInstructionClone`

❌ `fn LLVMInstructionEraseFromParent`

❌ `fn LLVMInstructionGetAllMetadataOtherThanDebugLoc`

❌ `fn LLVMInstructionRemoveFromParent`

❌ `fn LLVMInt1Type`

❌ `fn LLVMInt1TypeInContext`

❌ `fn LLVMInt8Type`

❌ `fn LLVMInt8TypeInContext`

❌ `fn LLVMInt16Type`

❌ `fn LLVMInt16TypeInContext`

❌ `fn LLVMInt32Type`

❌ `fn LLVMInt32TypeInContext`

❌ `fn LLVMInt64Type`

❌ `fn LLVMInt64TypeInContext`

❌ `fn LLVMInt128Type`

❌ `fn LLVMInt128TypeInContext`

❌ `fn LLVMIntType`

❌ `fn LLVMIntTypeInContext`

❌ `fn LLVMIntrinsicCopyOverloadedName2`

❌ `fn LLVMIntrinsicGetName`

❌ `fn LLVMIntrinsicGetType`

❌ `fn LLVMIntrinsicIsOverloaded`

❌ `fn LLVMIsAAddrSpaceCastInst`

❌ `fn LLVMIsAAllocaInst`

❌ `fn LLVMIsAArgument`

❌ `fn LLVMIsAAtomicCmpXchgInst`

❌ `fn LLVMIsAAtomicRMWInst`

❌ `fn LLVMIsABasicBlock`

❌ `fn LLVMIsABinaryOperator`

❌ `fn LLVMIsABitCastInst`

❌ `fn LLVMIsABlockAddress`

❌ `fn LLVMIsABranchInst`

❌ `fn LLVMIsACallBrInst`

❌ `fn LLVMIsACallInst`

❌ `fn LLVMIsACastInst`

❌ `fn LLVMIsACatchPadInst`

❌ `fn LLVMIsACatchReturnInst`

❌ `fn LLVMIsACatchSwitchInst`

❌ `fn LLVMIsACleanupPadInst`

❌ `fn LLVMIsACleanupReturnInst`

❌ `fn LLVMIsACmpInst`

❌ `fn LLVMIsAConstant`

❌ `fn LLVMIsAConstantAggregateZero`

❌ `fn LLVMIsAConstantArray`

❌ `fn LLVMIsAConstantDataArray`

❌ `fn LLVMIsAConstantDataSequential`

❌ `fn LLVMIsAConstantDataVector`

❌ `fn LLVMIsAConstantExpr`

❌ `fn LLVMIsAConstantFP`

❌ `fn LLVMIsAConstantInt`

❌ `fn LLVMIsAConstantPointerNull`

❌ `fn LLVMIsAConstantStruct`

❌ `fn LLVMIsAConstantTokenNone`

❌ `fn LLVMIsAConstantVector`

❌ `fn LLVMIsADbgDeclareInst`

❌ `fn LLVMIsADbgInfoIntrinsic`

❌ `fn LLVMIsADbgLabelInst`

❌ `fn LLVMIsADbgVariableIntrinsic`

❌ `fn LLVMIsAExtractElementInst`

❌ `fn LLVMIsAExtractValueInst`

❌ `fn LLVMIsAFCmpInst`

❌ `fn LLVMIsAFPExtInst`

❌ `fn LLVMIsAFPToSIInst`

❌ `fn LLVMIsAFPToUIInst`

❌ `fn LLVMIsAFPTruncInst`

❌ `fn LLVMIsAFenceInst`

❌ `fn LLVMIsAFreezeInst`

❌ `fn LLVMIsAFuncletPadInst`

❌ `fn LLVMIsAFunction`

❌ `fn LLVMIsAGetElementPtrInst`

❌ `fn LLVMIsAGlobalAlias`

❌ `fn LLVMIsAGlobalIFunc`

❌ `fn LLVMIsAGlobalObject`

❌ `fn LLVMIsAGlobalValue`

❌ `fn LLVMIsAGlobalVariable`

❌ `fn LLVMIsAICmpInst`

❌ `fn LLVMIsAIndirectBrInst`

❌ `fn LLVMIsAInlineAsm`

❌ `fn LLVMIsAInsertElementInst`

❌ `fn LLVMIsAInsertValueInst`

❌ `fn LLVMIsAInstruction`

❌ `fn LLVMIsAIntToPtrInst`

❌ `fn LLVMIsAIntrinsicInst`

❌ `fn LLVMIsAInvokeInst`

❌ `fn LLVMIsALandingPadInst`

❌ `fn LLVMIsALoadInst`

❌ `fn LLVMIsAMDNode`

❌ `fn LLVMIsAMDString`

❌ `fn LLVMIsAMemCpyInst`

❌ `fn LLVMIsAMemIntrinsic`

❌ `fn LLVMIsAMemMoveInst`

❌ `fn LLVMIsAMemSetInst`

❌ `fn LLVMIsAPHINode`

❌ `fn LLVMIsAPoisonValue`

❌ `fn LLVMIsAPtrToIntInst`

❌ `fn LLVMIsAResumeInst`

❌ `fn LLVMIsAReturnInst`

❌ `fn LLVMIsASExtInst`

❌ `fn LLVMIsASIToFPInst`

❌ `fn LLVMIsASelectInst`

❌ `fn LLVMIsAShuffleVectorInst`

❌ `fn LLVMIsAStoreInst`

❌ `fn LLVMIsASwitchInst`

❌ `fn LLVMIsATerminatorInst`

❌ `fn LLVMIsATruncInst`

❌ `fn LLVMIsAUIToFPInst`

❌ `fn LLVMIsAUnaryInstruction`

❌ `fn LLVMIsAUnaryOperator`

❌ `fn LLVMIsAUndefValue`

❌ `fn LLVMIsAUnreachableInst`

❌ `fn LLVMIsAUser`

❌ `fn LLVMIsAVAArgInst`

❌ `fn LLVMIsAZExtInst`

❌ `fn LLVMIsAtomicSingleThread`

❌ `fn LLVMIsCleanup	`

❌ `fn LLVMIsConditional`

❌ `fn LLVMIsConstant`

❌ `fn LLVMIsConstantString`

❌ `fn LLVMIsDeclaration`

❌ `fn LLVMIsEnumAttribute`

❌ `fn LLVMIsExternallyInitialized`

❌ `fn LLVMIsFunctionVarArg`

❌ `fn LLVMIsGlobalConstant`

❌ `fn LLVMIsInBounds`

❌ `fn LLVMIsLiteralStruct`

❌ `fn LLVMIsMultithreaded`

❌ `fn LLVMIsNull`

❌ `fn LLVMIsOpaqueStruct`

❌ `fn LLVMIsPackedStruct`

❌ `fn LLVMIsPoison`

❌ `fn LLVMIsStringAttribute`

❌ `fn LLVMIsTailCall`

❌ `fn LLVMIsThreadLocal`

❌ `fn LLVMIsTypeAttribute`

❌ `fn LLVMIsUndef`

❌ `fn LLVMLabelType`

❌ `fn LLVMLabelTypeInContext`

❌ `fn LLVMLookupIntrinsicID`

❌ `fn LLVMMDNodeInContext2`

❌ `fn LLVMMDStringInContext2`

❌ `fn LLVMMetadataAsValue`

❌ `fn LLVMMetadataTypeInContext`

❌ `fn LLVMModuleCreateWithName`

❌ `fn LLVMModuleCreateWithNameInContext`

❌ `fn LLVMModuleFlagEntriesGetFlagBehavior`

❌ `fn LLVMModuleFlagEntriesGetKey`

❌ `fn LLVMModuleFlagEntriesGetMetadata`

❌ `fn LLVMMoveBasicBlockAfter`

❌ `fn LLVMMoveBasicBlockBefore`

❌ `fn LLVMPPCFP128Type`

❌ `fn LLVMPPCFP128TypeInContext`

❌ `fn LLVMPointerType`

❌ `fn LLVMPositionBuilder`

❌ `fn LLVMPositionBuilderAtEnd`

❌ `fn LLVMPositionBuilderBefore`

❌ `fn LLVMPrintModuleToFile`

❌ `fn LLVMPrintModuleToString`

❌ `fn LLVMPrintTypeToString`

❌ `fn LLVMPrintValueToString`

❌ `fn LLVMRemoveBasicBlockFromParent`

❌ `fn LLVMRemoveCallSiteEnumAttribute`

❌ `fn LLVMRemoveCallSiteStringAttribute`

❌ `fn LLVMRemoveEnumAttributeAtIndex`

❌ `fn LLVMRemoveGlobalIFunc	`

❌ `fn LLVMRemoveStringAttributeAtIndex`

❌ `fn LLVMReplaceAllUsesWith`

❌ `fn LLVMRunFunctionPassManager`

❌ `fn LLVMRunPassManager`

❌ `fn LLVMScalableVectorType	`

❌ `fn LLVMSetAlignment`

❌ `fn LLVMSetArgOperand	`

❌ `fn LLVMSetAtomicRMWBinOp`

❌ `fn LLVMSetAtomicSingleThread`

❌ `fn LLVMSetCleanup`

❌ `fn LLVMSetCmpXchgFailureOrdering`

❌ `fn LLVMSetCmpXchgSuccessOrdering`

❌ `fn LLVMSetCondition`

❌ `fn LLVMSetCurrentDebugLocation2`

❌ `fn LLVMSetDLLStorageClass`

❌ `fn LLVMSetDataLayout`

❌ `fn LLVMSetExternallyInitialized`

❌ `fn LLVMSetFunctionCallConv`

❌ `fn LLVMSetGC`

❌ `fn LLVMSetGlobalConstant`

❌ `fn LLVMSetGlobalIFuncResolver	`

❌ `fn LLVMSetInitializer`

❌ `fn LLVMSetInstrParamAlignment`

❌ `fn LLVMSetInstructionCallConv`

❌ `fn LLVMSetIsInBounds	`

❌ `fn LLVMSetLinkage`

❌ `fn LLVMSetMetadata`

❌ `fn LLVMSetModuleIdentifier	`

❌ `fn LLVMSetModuleInlineAsm2`

❌ `fn LLVMSetNormalDest`

❌ `fn LLVMSetOperand`

❌ `fn LLVMSetOrdering`

❌ `fn LLVMSetParamAlignment`

❌ `fn LLVMSetParentCatchSwitch`

❌ `fn LLVMSetPersonalityFn	`

❌ `fn LLVMSetSection`

❌ `fn LLVMSetSourceFileName	`

❌ `fn LLVMSetSuccessor`

❌ `fn LLVMSetTailCall`

❌ `fn LLVMSetTarget`

❌ `fn LLVMSetThreadLocal`

❌ `fn LLVMSetThreadLocalMode`

❌ `fn LLVMSetUnnamedAddress`

❌ `fn LLVMSetUnwindDest`

❌ `fn LLVMSetValueName2`

❌ `fn LLVMSetVisibility`

❌ `fn LLVMSetVolatile`

❌ `fn LLVMSetWeak`

❌ `fn LLVMShutdown`

❌ `fn LLVMSizeOf`

❌ `fn LLVMStructCreateNamed`

❌ `fn LLVMStructGetTypeAtIndex`

❌ `fn LLVMStructSetBody`

❌ `fn LLVMStructType`

❌ `fn LLVMStructTypeInContext`

❌ `fn LLVMTokenTypeInContext`

❌ `fn LLVMTypeIsSized`

❌ `fn LLVMTypeOf`

❌ `fn LLVMValueAsBasicBlock`

❌ `fn LLVMValueAsMetadata`

❌ `fn LLVMValueIsBasicBlock`

❌ `fn LLVMValueMetadataEntriesGetKind`

❌ `fn LLVMValueMetadataEntriesGetMetadata`

❌ `fn LLVMVectorType`

❌ `fn LLVMVoidType`

❌ `fn LLVMVoidTypeInContext`

❌ `fn LLVMX86AMXType`

❌ `fn LLVMX86AMXTypeInContext`

❌ `fn LLVMX86FP80Type`

❌ `fn LLVMX86FP80TypeInContext`

❌ `fn LLVMX86MMXType`

❌ `fn LLVMX86MMXTypeInContext`
</details>

<details>
  <summary>debuginfo</summary>

✅ `enum LLVMDWARFEmissionKind`

✅ `enum LLVMDWARFMacinfoRecordType`

✅ `enum LLVMDWARFSourceLanguage`

✅ `enum LLVMMetadataKind`

✅ `const LLVMDIFlagAccessibility`

✅ `const LLVMDIFlagAppleBlock`

✅ `const LLVMDIFlagArtificial`

✅ `const LLVMDIFlagBigendian`

✅ `const LLVMDIFlagBitField`

✅ `const LLVMDIFlagEnumClass`

✅ `const LLVMDIFlagExplicit`

✅ `const LLVMDIFlagFwdDecl`

✅ `const LLVMDIFlagIndirectVirtualBase`

✅ `const LLVMDIFlagIntroducedVirtual`

✅ `const LLVMDIFlagLValueReference`

✅ `const LLVMDIFlagLittleEndian`

✅ `const LLVMDIFlagMultipleInheritance`

✅ `const LLVMDIFlagNoReturn`

✅ `const LLVMDIFlagNonTrivial`

✅ `const LLVMDIFlagObjcClassComplete`

✅ `const LLVMDIFlagObjectPointer`

✅ `const LLVMDIFlagPrivate`

✅ `const LLVMDIFlagProtected`

✅ `const LLVMDIFlagPrototyped`

✅ `const LLVMDIFlagPtrToMemberRep`

✅ `const LLVMDIFlagPublic`

✅ `const LLVMDIFlagRValueReference`

✅ `const LLVMDIFlagReserved`

✅ `const LLVMDIFlagReservedBit4`

✅ `const LLVMDIFlagSingleInheritance`

✅ `const LLVMDIFlagStaticMember`

✅ `const LLVMDIFlagThunk`

✅ `const LLVMDIFlagTypePassByReference`

✅ `const LLVMDIFlagTypePassByValue`

✅ `const LLVMDIFlagVector`

✅ `const LLVMDIFlagVirtual`

✅ `const LLVMDIFlagVirtualInheritance`

✅ `const LLVMDIFlagZero`

✅ `fn LLVMCreateDIBuilder`

✅ `fn LLVMCreateDIBuilderDisallowUnresolved`

✅ `fn LLVMDIBuilderCreateArrayType`

✅ `fn LLVMDIBuilderCreateArtificialType`

✅ `fn LLVMDIBuilderCreateAutoVariable`

✅ `fn LLVMDIBuilderCreateBasicType`

✅ `fn LLVMDIBuilderCreateBitFieldMemberType`

✅ `fn LLVMDIBuilderCreateClassType`

✅ `fn LLVMDIBuilderCreateCompileUnit`

✅ `fn LLVMDIBuilderCreateConstantValueExpression`

✅ `fn LLVMDIBuilderCreateDebugLocation`

✅ `fn LLVMDIBuilderCreateEnumerationType`

✅ `fn LLVMDIBuilderCreateEnumerator`

✅ `fn LLVMDIBuilderCreateExpression`

✅ `fn LLVMDIBuilderCreateFile`

✅ `fn LLVMDIBuilderCreateForwardDecl`

✅ `fn LLVMDIBuilderCreateFunction`

✅ `fn LLVMDIBuilderCreateGlobalVariableExpression`

✅ `fn LLVMDIBuilderCreateImportedDeclaration`

✅ `fn LLVMDIBuilderCreateImportedModuleFromAlias`

✅ `fn LLVMDIBuilderCreateImportedModuleFromModule`

✅ `fn LLVMDIBuilderCreateImportedModuleFromNamespace`

✅ `fn LLVMDIBuilderCreateInheritance`

✅ `fn LLVMDIBuilderCreateLexicalBlock`

✅ `fn LLVMDIBuilderCreateLexicalBlockFile`

✅ `fn LLVMDIBuilderCreateMacro`

✅ `fn LLVMDIBuilderCreateMemberPointerType`

✅ `fn LLVMDIBuilderCreateMemberType`

✅ `fn LLVMDIBuilderCreateModule`

✅ `fn LLVMDIBuilderCreateNameSpace`

✅ `fn LLVMDIBuilderCreateNullPtrType`

✅ `fn LLVMDIBuilderCreateObjCIVar`

✅ `fn LLVMDIBuilderCreateObjCProperty`

✅ `fn LLVMDIBuilderCreateObjectPointerType`

✅ `fn LLVMDIBuilderCreateParameterVariable`

✅ `fn LLVMDIBuilderCreatePointerType`

✅ `fn LLVMDIBuilderCreateQualifiedType`

✅ `fn LLVMDIBuilderCreateReferenceType`

✅ `fn LLVMDIBuilderCreateReplaceableCompositeType`

✅ `fn LLVMDIBuilderCreateStaticMemberType`

✅ `fn LLVMDIBuilderCreateStructType`

✅ `fn LLVMDIBuilderCreateSubroutineType`

✅ `fn LLVMDIBuilderCreateTempGlobalVariableFwdDecl`

✅ `fn LLVMDIBuilderCreateTempMacroFile`

✅ `fn LLVMDIBuilderCreateTypedef`

✅ `fn LLVMDIBuilderCreateUnionType`

✅ `fn LLVMDIBuilderCreateUnspecifiedType`

✅ `fn LLVMDIBuilderCreateVectorType`

✅ `fn LLVMDIBuilderFinalize`

✅ `fn LLVMDIBuilderFinalizeSubprogram`

✅ `fn LLVMDIBuilderGetOrCreateArray`

✅ `fn LLVMDIBuilderGetOrCreateSubrange`

✅ `fn LLVMDIBuilderGetOrCreateTypeArray`

✅ `fn LLVMDIBuilderInsertDbgValueAtEnd`

✅ `fn LLVMDIBuilderInsertDbgValueBefore`

✅ `fn LLVMDIBuilderInsertDeclareAtEnd`

✅ `fn LLVMDIBuilderInsertDeclareBefore`

✅ `fn LLVMDIFileGetDirectory`

✅ `fn LLVMDIFileGetFilename`

✅ `fn LLVMDIFileGetSource`

✅ `fn LLVMDIGlobalVariableExpressionGetExpression`

✅ `fn LLVMDIGlobalVariableExpressionGetVariable`

✅ `fn LLVMDILocationGetColumn`

✅ `fn LLVMDILocationGetInlinedAt`

✅ `fn LLVMDILocationGetLine`

✅ `fn LLVMDILocationGetScope`

✅ `fn LLVMDIScopeGetFile`

✅ `fn LLVMDISubprogramGetLine`

✅ `fn LLVMDITypeGetAlignInBits`

✅ `fn LLVMDITypeGetFlags`

✅ `fn LLVMDITypeGetLine`

✅ `fn LLVMDITypeGetName`

✅ `fn LLVMDITypeGetOffsetInBits`

✅ `fn LLVMDITypeGetSizeInBits`

✅ `fn LLVMDIVariableGetFile`

✅ `fn LLVMDIVariableGetLine`

✅ `fn LLVMDIVariableGetScope`

✅ `fn LLVMDebugMetadataVersion`

✅ `fn LLVMDisposeDIBuilder`

✅ `fn LLVMDisposeTemporaryMDNode`

✅ `fn LLVMGetMetadataKind`

✅ `fn LLVMGetModuleDebugMetadataVersion`

✅ `fn LLVMGetSubprogram`

✅ `fn LLVMInstructionGetDebugLoc`

✅ `fn LLVMInstructionSetDebugLoc`

✅ `fn LLVMMetadataReplaceAllUsesWith`

✅ `fn LLVMSetSubprogram`

✅ `fn LLVMStripModuleDebugInfo`

✅ `fn LLVMTemporaryMDNode`

✅ `type LLVMDIFlags`

✅ `type LLVMDWARFTypeEncoding`
</details>
<details>
  <summary>disassembler</summary>

❌ `struct LLVMOpInfoSymbol1`

❌ `struct Struct_LLVMOpInfo1`

❌ `enum LLVMOpaqueDisasmContext`

❌ `const LLVMDisassembler_Option_AsmPrinterVariant`

❌ `const LLVMDisassembler_Option_PrintImmHex`

❌ `const LLVMDisassembler_Option_PrintLatency`

❌ `const LLVMDisassembler_Option_SetInstrComments`

❌ `const LLVMDisassembler_Option_UseMarkup`

❌ `const LLVMDisassembler_ReferenceType_DeMangled_Name`

❌ `const LLVMDisassembler_ReferenceType_InOut_None`

❌ `const LLVMDisassembler_ReferenceType_In_ARM64_ADDXri`

❌ `const LLVMDisassembler_ReferenceType_In_ARM64_ADR`

❌ `const LLVMDisassembler_ReferenceType_In_ARM64_ADRP`

❌ `const LLVMDisassembler_ReferenceType_In_ARM64_LDRXl`

❌ `const LLVMDisassembler_ReferenceType_In_ARM64_LDRXui`

❌ `const LLVMDisassembler_ReferenceType_In_Branch`

❌ `const LLVMDisassembler_ReferenceType_In_PCrel_Load`

❌ `const LLVMDisassembler_ReferenceType_Out_LitPool_CstrAddr`

❌ `const LLVMDisassembler_ReferenceType_Out_LitPool_SymAddr`

❌ `const LLVMDisassembler_ReferenceType_Out_Objc_CFString_Ref`

❌ `const LLVMDisassembler_ReferenceType_Out_Objc_Class_Ref`

❌ `const LLVMDisassembler_ReferenceType_Out_Objc_Message`

❌ `const LLVMDisassembler_ReferenceType_Out_Objc_Message_Ref`

❌ `const LLVMDisassembler_ReferenceType_Out_Objc_Selector_Ref`

❌ `const LLVMDisassembler_ReferenceType_Out_SymbolStub`

❌ `const LLVMDisassembler_VariantKind_ARM64_GOTPAGE`

❌ `const LLVMDisassembler_VariantKind_ARM64_GOTPAGEOFF`

❌ `const LLVMDisassembler_VariantKind_ARM64_PAGE`

❌ `const LLVMDisassembler_VariantKind_ARM64_PAGEOFF`

❌ `const LLVMDisassembler_VariantKind_ARM64_TLVOFF`

❌ `const LLVMDisassembler_VariantKind_ARM64_TLVP`

❌ `const LLVMDisassembler_VariantKind_ARM_HI16`

❌ `const LLVMDisassembler_VariantKind_ARM_LO16`

❌ `const LLVMDisassembler_VariantKind_None`

❌ `fn LLVMCreateDisasm`

❌ `fn LLVMCreateDisasmCPU`

❌ `fn LLVMCreateDisasmCPUFeatures`

❌ `fn LLVMDisasmDispose`

❌ `fn LLVMDisasmInstruction`

❌ `fn LLVMSetDisasmOptions`

❌ `type LLVMDisasmContextRef`

❌ `type LLVMOpInfoCallback`

❌ `type LLVMSymbolLookupCallback`
</details>
<details>
  <summary>error</summary>

❌ `enum LLVMOpaqueError`

❌ `const LLVMErrorSuccess`

❌ `fn LLVMConsumeError`

❌ `fn LLVMCreateStringError`

❌ `fn LLVMDisposeErrorMessage`

❌ `fn LLVMGetErrorMessage`

❌ `fn LLVMGetErrorTypeId`

❌ `fn LLVMGetStringErrorTypeId`

❌ `type LLVMErrorRef`

❌ `type LLVMErrorTypeId`
</details>
<details>
  <summary>error_handling</summary>

❌ `fn LLVMEnablePrettyStackTrace`

❌ `fn LLVMInstallFatalErrorHandler`

❌ `fn LLVMResetFatalErrorHandler`

❌ `type LLVMFatalErrorHandler`
</details>
<details>
  <summary>execution_engine</summary>

❌ `struct LLVMMCJITCompilerOptions`

❌ `enum LLVMOpaqueExecutionEngine`

❌ `enum LLVMOpaqueGenericValue`

❌ `enum LLVMOpaqueMCJITMemoryManager`

❌ `fn LLVMAddGlobalMapping`

❌ `fn LLVMAddModule`

❌ `fn LLVMCreateExecutionEngineForModule`

❌ `fn LLVMCreateGDBRegistrationListener`

❌ `fn LLVMCreateGenericValueOfFloat`

❌ `fn LLVMCreateGenericValueOfInt`

❌ `fn LLVMCreateGenericValueOfPointer`

❌ `fn LLVMCreateIntelJITEventListener`

❌ `fn LLVMCreateInterpreterForModule`

❌ `fn LLVMCreateJITCompilerForModule`

❌ `fn LLVMCreateMCJITCompilerForModule`

❌ `fn LLVMCreateOProfileJITEventListener`

❌ `fn LLVMCreatePerfJITEventListener`

❌ `fn LLVMCreateSimpleMCJITMemoryManager`

❌ `fn LLVMDisposeExecutionEngine`

❌ `fn LLVMDisposeGenericValue`

❌ `fn LLVMDisposeMCJITMemoryManager`

❌ `fn LLVMExecutionEngineGetErrMsg`

❌ `fn LLVMFindFunction`

❌ `fn LLVMFreeMachineCodeForFunction`

❌ `fn LLVMGenericValueIntWidth`

❌ `fn LLVMGenericValueToFloat`

❌ `fn LLVMGenericValueToInt`

❌ `fn LLVMGenericValueToPointer`

❌ `fn LLVMGetExecutionEngineTargetData`

❌ `fn LLVMGetExecutionEngineTargetMachine`

❌ `fn LLVMGetFunctionAddress`

❌ `fn LLVMGetGlobalValueAddress`

❌ `fn LLVMGetPointerToGlobal`

❌ `fn LLVMInitializeMCJITCompilerOptions`

❌ `fn LLVMLinkInInterpreter`

❌ `fn LLVMLinkInMCJIT`

❌ `fn LLVMRecompileAndRelinkFunction`

❌ `fn LLVMRemoveModule`

❌ `fn LLVMRunFunction`

❌ `fn LLVMRunFunctionAsMain`

❌ `fn LLVMRunStaticConstructors`

❌ `fn LLVMRunStaticDestructors`

❌ `type LLVMExecutionEngineRef`

❌ `type LLVMGenericValueRef`

❌ `type LLVMMCJITMemoryManagerRef`

❌ `type LLVMMemoryManagerAllocateCodeSectionCallback`

❌ `type LLVMMemoryManagerAllocateDataSectionCallback`

❌ `type LLVMMemoryManagerDestroyCallback`

❌ `type LLVMMemoryManagerFinalizeMemoryCallback`
</details>
<details>
  <summary>initialization</summary>

❌ `fn LLVMInitializeAggressiveInstCombiner`

❌ `fn LLVMInitializeAnalysis`

❌ `fn LLVMInitializeCodeGen`

❌ `fn LLVMInitializeCore`

❌ `fn LLVMInitializeIPA`

❌ `fn LLVMInitializeIPO`

❌ `fn LLVMInitializeInstCombine`

❌ `fn LLVMInitializeInstrumentation`

❌ `fn LLVMInitializeObjCARCOpts`

❌ `fn LLVMInitializeScalarOpts`

❌ `fn LLVMInitializeTarget`

❌ `fn LLVMInitializeTransformUtils`

❌ `fn LLVMInitializeVectorization`
</details>
<details>
  <summary>ir_reader</summary>

❌ `fn LLVMParseIRInContext`
</details>
<details>
  <summary>linker</summary>

❌ `enum LLVMLinkerMode`

❌ `fn LLVMLinkModules2`
</details>
<details>
  <summary>lto</summary>

❌ `struct LTOObjectBuffer`

❌ `enum LLVMOpaqueLTOCodeGenerator`

❌ `enum LLVMOpaqueLTOInput`

❌ `enum LLVMOpaqueLTOModule`

❌ `enum LLVMOpaqueThinLTOCodeGenerator`

❌ `enum lto_codegen_diagnostic_severity_t`

❌ `enum lto_codegen_model`

❌ `enum lto_debug_model`

❌ `enum lto_symbol_attributes`

❌ `fn lto_api_version`

❌ `fn lto_codegen_add_module`

❌ `fn lto_codegen_add_must_preserve_symbol`

❌ `fn lto_codegen_compile`

❌ `fn lto_codegen_compile_optimized`

❌ `fn lto_codegen_compile_to_file`

❌ `fn lto_codegen_create`

❌ `fn lto_codegen_create_in_local_context`

❌ `fn lto_codegen_debug_options`

❌ `fn lto_codegen_debug_options_array`

❌ `fn lto_codegen_dispose`

❌ `fn lto_codegen_optimize`

❌ `fn lto_codegen_set_assembler_args`

❌ `fn lto_codegen_set_assembler_path`

❌ `fn lto_codegen_set_cpu`

❌ `fn lto_codegen_set_debug_model`

❌ `fn lto_codegen_set_diagnostic_handler`

❌ `fn lto_codegen_set_module`

❌ `fn lto_codegen_set_pic_model`

❌ `fn lto_codegen_set_should_embed_uselists`

❌ `fn lto_codegen_set_should_internalize`

❌ `fn lto_codegen_write_merged_modules`

❌ `fn lto_get_error_message`

❌ `fn lto_get_version`

❌ `fn lto_initialize_disassembler`

❌ `fn lto_input_create`

❌ `fn lto_input_dispose`

❌ `fn lto_input_get_dependent_library`

❌ `fn lto_input_get_num_dependent_libraries`

❌ `fn lto_module_create`

❌ `fn lto_module_create_from_fd`

❌ `fn lto_module_create_from_fd_at_offset`

❌ `fn lto_module_create_from_memory`

❌ `fn lto_module_create_from_memory_with_path`

❌ `fn lto_module_create_in_codegen_context`

❌ `fn lto_module_create_in_local_context`

❌ `fn lto_module_dispose`

❌ `fn lto_module_get_linkeropts`

❌ `fn lto_module_get_macho_cputype`

❌ `fn lto_module_get_num_symbols`

❌ `fn lto_module_get_symbol_attribute`

❌ `fn lto_module_get_symbol_name`

❌ `fn lto_module_get_target_triple`

❌ `fn lto_module_has_ctor_dtor`

❌ `fn lto_module_has_objc_category`

❌ `fn lto_module_is_object_file`

❌ `fn lto_module_is_object_file_for_target`

❌ `fn lto_module_is_object_file_in_memory`

❌ `fn lto_module_is_object_file_in_memory_for_target`

❌ `fn lto_module_is_thinlto`

❌ `fn lto_module_set_target_triple`

❌ `fn lto_runtime_lib_symbols_list`

❌ `fn lto_set_debug_options`

❌ `fn thinlto_codegen_add_cross_referenced_symbol`

❌ `fn thinlto_codegen_add_module`

❌ `fn thinlto_codegen_add_must_preserve_symbol`

❌ `fn thinlto_codegen_disable_codegen`

❌ `fn thinlto_codegen_dispose`

❌ `fn thinlto_codegen_process`

❌ `fn thinlto_codegen_set_cache_dir`

❌ `fn thinlto_codegen_set_cache_entry_expiration`

❌ `fn thinlto_codegen_set_cache_pruning_interval`

❌ `fn thinlto_codegen_set_cache_size_bytes`

❌ `fn thinlto_codegen_set_cache_size_files`

❌ `fn thinlto_codegen_set_cache_size_megabytes`

❌ `fn thinlto_codegen_set_codegen_only`

❌ `fn thinlto_codegen_set_cpu`

❌ `fn thinlto_codegen_set_final_cache_size_relative_to_available_space`

❌ `fn thinlto_codegen_set_pic_model`

❌ `fn thinlto_codegen_set_savetemps_dir`

❌ `fn thinlto_create_codegen`

❌ `fn thinlto_debug_options`

❌ `fn thinlto_module_get_num_object_files`

❌ `fn thinlto_module_get_num_objects`

❌ `fn thinlto_module_get_object`

❌ `fn thinlto_module_get_object_file`

❌ `fn thinlto_set_generated_objects_dir`

❌ `type lto_bool_t`

❌ `type lto_code_gen_t`

❌ `type lto_diagnostic_handler_t`

❌ `type lto_input_t`

❌ `type lto_module_t`

❌ `type thinlto_code_gen_t`
</details>
<details>
  <summary>object</summary>

❌ `enum LLVMBinaryType`

❌ `enum LLVMOpaqueBinary`

❌ `enum LLVMOpaqueRelocationIterator`

❌ `enum LLVMOpaqueSectionIterator`

❌ `enum LLVMOpaqueSymbolIterator`

❌ `fn LLVMBinaryCopyMemoryBuffer`

❌ `fn LLVMBinaryGetType`

❌ `fn LLVMCreateBinary`

❌ `fn LLVMCreateObjectFile`

❌ `fn LLVMDisposeBinary`

❌ `fn LLVMDisposeRelocationIterator`

❌ `fn LLVMDisposeSectionIterator`

❌ `fn LLVMDisposeSymbolIterator`

❌ `fn LLVMGetRelocationOffset`

❌ `fn LLVMGetRelocationSymbol`

❌ `fn LLVMGetRelocationType`

❌ `fn LLVMGetRelocationTypeName`

❌ `fn LLVMGetRelocationValueString`

❌ `fn LLVMGetRelocations`

❌ `fn LLVMGetSectionAddress`

❌ `fn LLVMGetSectionContainsSymbol`

❌ `fn LLVMGetSectionContents`

❌ `fn LLVMGetSectionName`

❌ `fn LLVMGetSectionSize`

❌ `fn LLVMGetSymbolAddress`

❌ `fn LLVMGetSymbolName`

❌ `fn LLVMGetSymbolSize`

❌ `fn LLVMIsRelocationIteratorAtEnd`

❌ `fn LLVMMachOUniversalBinaryCopyObjectForArch`

❌ `fn LLVMMoveToContainingSection`

❌ `fn LLVMMoveToNextRelocation`

❌ `fn LLVMMoveToNextSection`

❌ `fn LLVMMoveToNextSymbol`

❌ `fn LLVMObjectFileCopySectionIterator`

❌ `fn LLVMObjectFileCopySymbolIterator`

❌ `fn LLVMObjectFileIsSectionIteratorAtEnd`

❌ `fn LLVMObjectFileIsSymbolIteratorAtEnd`

❌ `type LLVMBinaryRef`

❌ `type LLVMRelocationIteratorRef`

❌ `type LLVMSectionIteratorRef`

❌ `type LLVMSymbolIteratorRef`
</details>
<details>
  <summary>orc2</summary>

❌ `fn ee::LLVMOrcCreateRTDyldObjectLinkingLayerWithSectionMemoryManager`

❌ `fn ee::LLVMOrcRTDyldObjectLinkingLayerRegisterJITEventListener`

❌ `enum lljit::LLVMOrcOpaqueLLJIT`

❌ `enum lljit::LLVMOrcOpaqueLLJITBuilder`

❌ `fn lljit::LLVMOrcCreateLLJIT`

❌ `fn lljit::LLVMOrcCreateLLJITBuilder`

❌ `fn lljit::LLVMOrcDisposeLLJIT`

❌ `fn lljit::LLVMOrcDisposeLLJITBuilder`

❌ `fn lljit::LLVMOrcLLJITAddLLVMIRModule`

❌ `fn lljit::LLVMOrcLLJITAddLLVMIRModuleWithRT`

❌ `fn lljit::LLVMOrcLLJITAddObjectFile`

❌ `fn lljit::LLVMOrcLLJITAddObjectFileWithRT`

❌ `fn lljit::LLVMOrcLLJITBuilderSetJITTargetMachineBuilder`

❌ `fn lljit::LLVMOrcLLJITBuilderSetObjectLinkingLayerCreator`

❌ `fn lljit::LLVMOrcLLJITGetDataLayoutStr`

❌ `fn lljit::LLVMOrcLLJITGetExecutionSession`

❌ `fn lljit::LLVMOrcLLJITGetGlobalPrefix`

❌ `fn lljit::LLVMOrcLLJITGetIRTransformLayer`

❌ `fn lljit::LLVMOrcLLJITGetMainJITDylib`

❌ `fn lljit::LLVMOrcLLJITGetObjLinkingLayer`

❌ `fn lljit::LLVMOrcLLJITGetObjTransformLayer`

❌ `fn lljit::LLVMOrcLLJITGetTripleString`

❌ `fn lljit::LLVMOrcLLJITLookup`

❌ `fn lljit::LLVMOrcLLJITMangleAndIntern`

❌ `type lljit::LLVMOrcLLJITBuilderObjectLinkingLayerCreatorFunction`

❌ `type lljit::LLVMOrcLLJITBuilderRef`

❌ `type lljit::LLVMOrcLLJITRef`

❌ `struct LLVMJITCSymbolMapPair`

❌ `struct LLVMJITEvaluatedSymbol`

❌ `struct LLVMJITSymbolFlags`

❌ `struct LLVMOrcCDependenceMapPair`

❌ `struct LLVMOrcCLookupSetElement`

❌ `struct LLVMOrcCSymbolAliasMapEntry`

❌ `struct LLVMOrcCSymbolAliasMapPair`

❌ `struct LLVMOrcCSymbolFlagsMapPair`

❌ `struct LLVMOrcCSymbolsList`

❌ `enum LLVMJITSymbolGenericFlags`

❌ `enum LLVMOrcJITDylibLookupFlags`

❌ `enum LLVMOrcLookupKind`

❌ `enum LLVMOrcOpaqueDefinitionGenerator`

❌ `enum LLVMOrcOpaqueDumpObjects`

❌ `enum LLVMOrcOpaqueExecutionSession`

❌ `enum LLVMOrcOpaqueIRTransformLayer`

❌ `enum LLVMOrcOpaqueIndirectStubsManager`

❌ `enum LLVMOrcOpaqueJITDylib`

❌ `enum LLVMOrcOpaqueJITTargetMachineBuilder`

❌ `enum LLVMOrcOpaqueLazyCallThroughManager`

❌ `enum LLVMOrcOpaqueLookupState`

❌ `enum LLVMOrcOpaqueMaterializationResponsibility`

❌ `enum LLVMOrcOpaqueMaterializationUnit`

❌ `enum LLVMOrcOpaqueObjectLayer`

❌ `enum LLVMOrcOpaqueObjectLinkingLayer`

❌ `enum LLVMOrcOpaqueObjectTransformLayer`

❌ `enum LLVMOrcOpaqueResourceTracker`

❌ `enum LLVMOrcOpaqueSymbolStringPool`

❌ `enum LLVMOrcOpaqueSymbolStringPoolEntry`

❌ `enum LLVMOrcOpaqueThreadSafeContext`

❌ `enum LLVMOrcOpaqueThreadSafeModule`

❌ `enum LLVMOrcSymbolLookupFlags`

❌ `fn LLVMOrcAbsoluteSymbols`

❌ `fn LLVMOrcCreateCustomCAPIDefinitionGenerator`

❌ `fn LLVMOrcCreateCustomMaterializationUnit`

❌ `fn LLVMOrcCreateDumpObjects`

❌ `fn LLVMOrcCreateDynamicLibrarySearchGeneratorForPath`

❌ `fn LLVMOrcCreateDynamicLibrarySearchGeneratorForProcess`

❌ `fn LLVMOrcCreateLocalIndirectStubsManager`

❌ `fn LLVMOrcCreateLocalLazyCallThroughManager`

❌ `fn LLVMOrcCreateNewThreadSafeContext`

❌ `fn LLVMOrcCreateNewThreadSafeModule`

❌ `fn LLVMOrcCreateStaticLibrarySearchGeneratorForPath`

❌ `fn LLVMOrcDisposeCSymbolFlagsMap`

❌ `fn LLVMOrcDisposeDefinitionGenerator`

❌ `fn LLVMOrcDisposeDumpObjects`

❌ `fn LLVMOrcDisposeIndirectStubsManager`

❌ `fn LLVMOrcDisposeJITTargetMachineBuilder`

❌ `fn LLVMOrcDisposeLazyCallThroughManager`

❌ `fn LLVMOrcDisposeMaterializationResponsibility`

❌ `fn LLVMOrcDisposeMaterializationUnit`

❌ `fn LLVMOrcDisposeObjectLayer`

❌ `fn LLVMOrcDisposeSymbols`

❌ `fn LLVMOrcDisposeThreadSafeContext`

❌ `fn LLVMOrcDisposeThreadSafeModule`

❌ `fn LLVMOrcDumpObjects_CallOperator`

❌ `fn LLVMOrcExecutionSessionCreateBareJITDylib`

❌ `fn LLVMOrcExecutionSessionCreateJITDylib`

❌ `fn LLVMOrcExecutionSessionGetJITDylibByName`

❌ `fn LLVMOrcExecutionSessionGetSymbolStringPool`

❌ `fn LLVMOrcExecutionSessionIntern`

❌ `fn LLVMOrcExecutionSessionSetErrorReporter`

❌ `fn LLVMOrcIRTransformLayerEmit`

❌ `fn LLVMOrcIRTransformLayerSetTransform`

❌ `fn LLVMOrcJITDylibAddGenerator`

❌ `fn LLVMOrcJITDylibClear`

❌ `fn LLVMOrcJITDylibCreateResourceTracker`

❌ `fn LLVMOrcJITDylibDefine`

❌ `fn LLVMOrcJITDylibGetDefaultResourceTracker`

❌ `fn LLVMOrcJITTargetMachineBuilderCreateFromTargetMachine`

❌ `fn LLVMOrcJITTargetMachineBuilderDetectHost`

❌ `fn LLVMOrcJITTargetMachineBuilderGetTargetTriple`

❌ `fn LLVMOrcJITTargetMachineBuilderSetTargetTriple`

❌ `fn LLVMOrcLazyReexports`

❌ `fn LLVMOrcMaterializationResponsibilityAddDependencies`

❌ `fn LLVMOrcMaterializationResponsibilityAddDependenciesForAll`

❌ `fn LLVMOrcMaterializationResponsibilityDefineMaterializing`

❌ `fn LLVMOrcMaterializationResponsibilityDelegate`

❌ `fn LLVMOrcMaterializationResponsibilityFailMaterialization`

❌ `fn LLVMOrcMaterializationResponsibilityGetExecutionSession`

❌ `fn LLVMOrcMaterializationResponsibilityGetInitializerSymbol`

❌ `fn LLVMOrcMaterializationResponsibilityGetRequestedSymbols`

❌ `fn LLVMOrcMaterializationResponsibilityGetSymbols`

❌ `fn LLVMOrcMaterializationResponsibilityGetTargetDylib`

❌ `fn LLVMOrcMaterializationResponsibilityNotifyEmitted`

❌ `fn LLVMOrcMaterializationResponsibilityNotifyResolved`

❌ `fn LLVMOrcMaterializationResponsibilityReplace`

❌ `fn LLVMOrcObjectLayerAddObjectFile`

❌ `fn LLVMOrcObjectLayerAddObjectFileWithRT`

❌ `fn LLVMOrcObjectLayerEmit`

❌ `fn LLVMOrcObjectTransformLayerSetTransform`

❌ `fn LLVMOrcReleaseResourceTracker`

❌ `fn LLVMOrcReleaseSymbolStringPoolEntry`

❌ `fn LLVMOrcResourceTrackerRemove`

❌ `fn LLVMOrcResourceTrackerTransferTo`

❌ `fn LLVMOrcRetainSymbolStringPoolEntry`

❌ `fn LLVMOrcSymbolStringPoolClearDeadEntries`

❌ `fn LLVMOrcSymbolStringPoolEntryStr`

❌ `fn LLVMOrcThreadSafeContextGetContext`

❌ `fn LLVMOrcThreadSafeModuleWithModuleDo`

❌ `type LLVMJITSymbolTargetFlags`

❌ `type LLVMOrcCAPIDefinitionGeneratorTryToGenerateFunction`

❌ `type LLVMOrcCDependenceMapPairs`

❌ `type LLVMOrcCLookupSet`

❌ `type LLVMOrcCSymbolAliasMapPairs`

❌ `type LLVMOrcCSymbolFlagsMapPairs`

❌ `type LLVMOrcCSymbolMapPairs`

❌ `type LLVMOrcDefinitionGeneratorRef`

❌ `type LLVMOrcDumpObjectsRef`

❌ `type LLVMOrcErrorReporterFunction`

❌ `type LLVMOrcExecutionSessionRef`

❌ `type LLVMOrcExecutorAddress`

❌ `type LLVMOrcGenericIRModuleOperationFunction`

❌ `type LLVMOrcIRTransformLayerRef`

❌ `type LLVMOrcIRTransformLayerTransformFunction`

❌ `type LLVMOrcIndirectStubsManagerRef`

❌ `type LLVMOrcJITDylibRef`

❌ `type LLVMOrcJITTargetAddress`

❌ `type LLVMOrcJITTargetMachineBuilderRef`

❌ `type LLVMOrcLazyCallThroughManagerRef`

❌ `type LLVMOrcLookupStateRef`

❌ `type LLVMOrcMaterializationResponsibilityRef`

❌ `type LLVMOrcMaterializationUnitDestroyFunction`

❌ `type LLVMOrcMaterializationUnitDiscardFunction`

❌ `type LLVMOrcMaterializationUnitMaterializeFunction`

❌ `type LLVMOrcMaterializationUnitRef`

❌ `type LLVMOrcObjectLayerRef`

❌ `type LLVMOrcObjectLinkingLayerRef`

❌ `type LLVMOrcObjectTransformLayerRef`

❌ `type LLVMOrcObjectTransformLayerTransformFunction`

❌ `type LLVMOrcResourceTrackerRef`

❌ `type LLVMOrcSymbolPredicate`

❌ `type LLVMOrcSymbolStringPoolEntryRef`

❌ `type LLVMOrcSymbolStringPoolRef`

❌ `type LLVMOrcThreadSafeContextRef`

❌ `type LLVMOrcThreadSafeModuleRef`
</details>
<details>
  <summary>prelude</summary>

❌ `type LLVMAttributeRef`

❌ `type LLVMBasicBlockRef`

❌ `type LLVMBool`

❌ `type LLVMBuilderRef`

❌ `type LLVMComdatRef`

❌ `type LLVMContextRef`

❌ `type LLVMDIBuilderRef`

❌ `type LLVMDiagnosticInfoRef`

❌ `type LLVMJITEventListenerRef`

❌ `type LLVMMemoryBufferRef`

❌ `type LLVMMetadataRef`

❌ `type LLVMModuleFlagEntry`

❌ `type LLVMModuleProviderRef`

❌ `type LLVMModuleRef`

❌ `type LLVMNamedMDNodeRef`

❌ `type LLVMPassManagerRef`

❌ `type LLVMPassRegistryRef`

❌ `type LLVMTypeRef`

❌ `type LLVMUseRef`

❌ `type LLVMValueMetadataEntry`

❌ `type LLVMValueRef`
</details>
<details>
  <summary>remarks</summary>

❌ `enum LLVMRemarkOpaqueArg`

❌ `enum LLVMRemarkOpaqueDebugLoc`

❌ `enum LLVMRemarkOpaqueEntry`

❌ `enum LLVMRemarkOpaqueParser`

❌ `enum LLVMRemarkOpaqueString`

❌ `enum LLVMRemarkType`

❌ `const REMARKS_API_VERSION`

❌ `fn LLVMRemarkArgGetDebugLoc`

❌ `fn LLVMRemarkArgGetKey`

❌ `fn LLVMRemarkArgGetValue`

❌ `fn LLVMRemarkDebugLocGetSourceColumn`

❌ `fn LLVMRemarkDebugLocGetSourceFilePath`

❌ `fn LLVMRemarkDebugLocGetSourceLine`

❌ `fn LLVMRemarkEntryDispose`

❌ `fn LLVMRemarkEntryGetDebugLoc`

❌ `fn LLVMRemarkEntryGetFirstArg`

❌ `fn LLVMRemarkEntryGetFunctionName`

❌ `fn LLVMRemarkEntryGetHotness`

❌ `fn LLVMRemarkEntryGetNextArg`

❌ `fn LLVMRemarkEntryGetNumArgs`

❌ `fn LLVMRemarkEntryGetPassName`

❌ `fn LLVMRemarkEntryGetRemarkName`

❌ `fn LLVMRemarkEntryGetType`

❌ `fn LLVMRemarkParserCreateBitstream`

❌ `fn LLVMRemarkParserCreateYAML`

❌ `fn LLVMRemarkParserDispose`

❌ `fn LLVMRemarkParserGetErrorMessage`

❌ `fn LLVMRemarkParserGetNext`

❌ `fn LLVMRemarkParserHasError`

❌ `fn LLVMRemarkStringGetData`

❌ `fn LLVMRemarkStringGetLen`

❌ `fn LLVMRemarkVersion`

❌ `type LLVMRemarkArgRef`

❌ `type LLVMRemarkDebugLocRef`

❌ `type LLVMRemarkEntryRef`

❌ `type LLVMRemarkParserRef`

❌ `type LLVMRemarkStringRef`
</details>
<details>
  <summary>support</summary>

❌ `fn LLVMAddSymbol`

❌ `fn LLVMLoadLibraryPermanently`

❌ `fn LLVMParseCommandLineOptions`

❌ `fn LLVMSearchForAddressOfSymbol`
</details>
<details>
  <summary>target</summary>

❌ `enum LLVMByteOrdering`

❌ `enum LLVMOpaqueTargetData`

❌ `enum LLVMOpaqueTargetLibraryInfotData`

❌ `fn LLVMABIAlignmentOfType`

❌ `fn LLVMABISizeOfType`

❌ `fn LLVMAddTargetLibraryInfo`

❌ `fn LLVMByteOrder`

❌ `fn LLVMCallFrameAlignmentOfType`

❌ `fn LLVMCopyStringRepOfTargetData`

❌ `fn LLVMCreateTargetData`

❌ `fn LLVMDisposeTargetData`

❌ `fn LLVMElementAtOffset`

❌ `fn LLVMGetModuleDataLayout`

❌ `fn LLVMInitializeAArch64AsmParser`

❌ `fn LLVMInitializeAArch64AsmPrinter`

❌ `fn LLVMInitializeAArch64Disassembler`

❌ `fn LLVMInitializeAArch64Target`

❌ `fn LLVMInitializeAArch64TargetInfo`

❌ `fn LLVMInitializeAArch64TargetMC`

❌ `fn LLVMInitializeAMDGPUAsmParser`

❌ `fn LLVMInitializeAMDGPUAsmPrinter`

❌ `fn LLVMInitializeAMDGPUTarget`

❌ `fn LLVMInitializeAMDGPUTargetInfo`

❌ `fn LLVMInitializeAMDGPUTargetMC`

❌ `fn LLVMInitializeARMAsmParser`

❌ `fn LLVMInitializeARMAsmPrinter`

❌ `fn LLVMInitializeARMDisassembler`

❌ `fn LLVMInitializeARMTarget`

❌ `fn LLVMInitializeARMTargetInfo`

❌ `fn LLVMInitializeARMTargetMC`

❌ `fn LLVMInitializeBPFAsmPrinter`

❌ `fn LLVMInitializeBPFDisassembler`

❌ `fn LLVMInitializeBPFTarget`

❌ `fn LLVMInitializeBPFTargetInfo`

❌ `fn LLVMInitializeBPFTargetMC`

❌ `fn LLVMInitializeHexagonAsmPrinter`

❌ `fn LLVMInitializeHexagonDisassembler`

❌ `fn LLVMInitializeHexagonTarget`

❌ `fn LLVMInitializeHexagonTargetInfo`

❌ `fn LLVMInitializeHexagonTargetMC`

❌ `fn LLVMInitializeLanaiAsmParser`

❌ `fn LLVMInitializeLanaiAsmPrinter`

❌ `fn LLVMInitializeLanaiDisassembler`

❌ `fn LLVMInitializeLanaiTarget`

❌ `fn LLVMInitializeLanaiTargetInfo`

❌ `fn LLVMInitializeLanaiTargetMC`

❌ `fn LLVMInitializeMSP430AsmPrinter`

❌ `fn LLVMInitializeMSP430Target`

❌ `fn LLVMInitializeMSP430TargetInfo`

❌ `fn LLVMInitializeMSP430TargetMC`

❌ `fn LLVMInitializeMipsAsmParser`

❌ `fn LLVMInitializeMipsAsmPrinter`

❌ `fn LLVMInitializeMipsDisassembler`

❌ `fn LLVMInitializeMipsTarget`

❌ `fn LLVMInitializeMipsTargetInfo`

❌ `fn LLVMInitializeMipsTargetMC`

❌ `fn LLVMInitializeNVPTXAsmPrinter`

❌ `fn LLVMInitializeNVPTXTarget`

❌ `fn LLVMInitializeNVPTXTargetInfo`

❌ `fn LLVMInitializeNVPTXTargetMC`

❌ `fn LLVMInitializePowerPCAsmParser`

❌ `fn LLVMInitializePowerPCAsmPrinter`

❌ `fn LLVMInitializePowerPCDisassembler`

❌ `fn LLVMInitializePowerPCTarget`

❌ `fn LLVMInitializePowerPCTargetInfo`

❌ `fn LLVMInitializePowerPCTargetMC`

❌ `fn LLVMInitializeRISCVAsmParser`

❌ `fn LLVMInitializeRISCVAsmPrinter`

❌ `fn LLVMInitializeRISCVDisassembler`

❌ `fn LLVMInitializeRISCVTarget`

❌ `fn LLVMInitializeRISCVTargetInfo`

❌ `fn LLVMInitializeRISCVTargetMC`

❌ `fn LLVMInitializeSparcAsmParser`

❌ `fn LLVMInitializeSparcAsmPrinter`

❌ `fn LLVMInitializeSparcDisassembler`

❌ `fn LLVMInitializeSparcTarget`

❌ `fn LLVMInitializeSparcTargetInfo`

❌ `fn LLVMInitializeSparcTargetMC`

❌ `fn LLVMInitializeSystemZAsmParser`

❌ `fn LLVMInitializeSystemZAsmPrinter`

❌ `fn LLVMInitializeSystemZDisassembler`

❌ `fn LLVMInitializeSystemZTarget`

❌ `fn LLVMInitializeSystemZTargetInfo`

❌ `fn LLVMInitializeSystemZTargetMC`

❌ `fn LLVMInitializeWebAssemblyAsmParser`

❌ `fn LLVMInitializeWebAssemblyAsmPrinter`

❌ `fn LLVMInitializeWebAssemblyDisassembler`

❌ `fn LLVMInitializeWebAssemblyTarget`

❌ `fn LLVMInitializeWebAssemblyTargetInfo`

❌ `fn LLVMInitializeWebAssemblyTargetMC`

❌ `fn LLVMInitializeX86AsmParser`

❌ `fn LLVMInitializeX86AsmPrinter`

❌ `fn LLVMInitializeX86Disassembler`

❌ `fn LLVMInitializeX86Target`

❌ `fn LLVMInitializeX86TargetInfo`

❌ `fn LLVMInitializeX86TargetMC`

❌ `fn LLVMInitializeXCoreAsmPrinter`

❌ `fn LLVMInitializeXCoreDisassembler`

❌ `fn LLVMInitializeXCoreTarget`

❌ `fn LLVMInitializeXCoreTargetInfo`

❌ `fn LLVMInitializeXCoreTargetMC`

❌ `fn LLVMIntPtrType`

❌ `fn LLVMIntPtrTypeForAS`

❌ `fn LLVMIntPtrTypeForASInContext`

❌ `fn LLVMIntPtrTypeInContext`

❌ `fn LLVMOffsetOfElement`

❌ `fn LLVMPointerSize`

❌ `fn LLVMPointerSizeForAS`

❌ `fn LLVMPreferredAlignmentOfGlobal`

❌ `fn LLVMPreferredAlignmentOfType`

❌ `fn LLVMSetModuleDataLayout`

❌ `fn LLVMSizeOfTypeInBits`

❌ `fn LLVMStoreSizeOfType`

❌ `fn LLVM_InitializeAllAsmParsers`

❌ `fn LLVM_InitializeAllAsmPrinters`

❌ `fn LLVM_InitializeAllDisassemblers`

❌ `fn LLVM_InitializeAllTargetInfos`

❌ `fn LLVM_InitializeAllTargetMCs`

❌ `fn LLVM_InitializeAllTargets`

❌ `fn LLVM_InitializeNativeAsmParser`

❌ `fn LLVM_InitializeNativeAsmPrinter`

❌ `fn LLVM_InitializeNativeDisassembler`

❌ `fn LLVM_InitializeNativeTarget`

❌ `type LLVMTargetDataRef`

❌ `type LLVMTargetLibraryInfoRef`
</details>
<details>
  <summary>target_machine</summary>

❌ `enum LLVMCodeGenFileType`

❌ `enum LLVMCodeGenOptLevel`

❌ `enum LLVMCodeModel`

❌ `enum LLVMOpaqueTargetMachine`

❌ `enum LLVMRelocMode`

❌ `enum LLVMTarget`

❌ `fn LLVMAddAnalysisPasses`

❌ `fn LLVMCreateTargetDataLayout`

❌ `fn LLVMCreateTargetMachine`

❌ `fn LLVMDisposeTargetMachine`

❌ `fn LLVMGetDefaultTargetTriple`

❌ `fn LLVMGetFirstTarget`

❌ `fn LLVMGetHostCPUFeatures`

❌ `fn LLVMGetHostCPUName`

❌ `fn LLVMGetNextTarget`

❌ `fn LLVMGetTargetDescription`

❌ `fn LLVMGetTargetFromName`

❌ `fn LLVMGetTargetFromTriple`

❌ `fn LLVMGetTargetMachineCPU`

❌ `fn LLVMGetTargetMachineFeatureString`

❌ `fn LLVMGetTargetMachineTarget`

❌ `fn LLVMGetTargetMachineTriple`

❌ `fn LLVMGetTargetName`

❌ `fn LLVMNormalizeTargetTriple`

❌ `fn LLVMSetTargetMachineAsmVerbosity`

❌ `fn LLVMTargetHasAsmBackend`

❌ `fn LLVMTargetHasJIT`

❌ `fn LLVMTargetHasTargetMachine`

❌ `fn LLVMTargetMachineEmitToFile`

❌ `fn LLVMTargetMachineEmitToMemoryBuffer`

❌ `type LLVMTargetMachineRef`

❌ `type LLVMTargetRef`
</details>
<details>
  <summary>transforms</summary>

❌ `fn aggressive_instcombine::LLVMAddAggressiveInstCombinerPass`

❌ `fn coroutines::LLVMAddCoroCleanupPass`

❌ `fn coroutines::LLVMAddCoroEarlyPass`

❌ `fn coroutines::LLVMAddCoroElidePass`

❌ `fn coroutines::LLVMAddCoroSplitPass`

❌ `fn coroutines::LLVMPassManagerBuilderAddCoroutinePassesToExtensionPoints`

❌ `fn instcombine::LLVMAddInstructionCombiningPass`

❌ `fn ipo::LLVMAddAlwaysInlinerPass`

❌ `fn ipo::LLVMAddArgumentPromotionPass`

❌ `fn ipo::LLVMAddCalledValuePropagationPass`

❌ `fn ipo::LLVMAddConstantMergePass`

❌ `fn ipo::LLVMAddDeadArgEliminationPass`

❌ `fn ipo::LLVMAddFunctionAttrsPass`

❌ `fn ipo::LLVMAddFunctionInliningPass`

❌ `fn ipo::LLVMAddGlobalDCEPass`

❌ `fn ipo::LLVMAddGlobalOptimizerPass`

❌ `fn ipo::LLVMAddIPSCCPPass`

❌ `fn ipo::LLVMAddInternalizePass`

❌ `fn ipo::LLVMAddInternalizePassWithMustPreservePredicate`

❌ `fn ipo::LLVMAddMergeFunctionsPass`

❌ `fn ipo::LLVMAddPruneEHPass`

❌ `fn ipo::LLVMAddStripDeadPrototypesPass`

❌ `fn ipo::LLVMAddStripSymbolsPass`

❌ `enum pass_builder::LLVMOpaquePassBuilderOptions`

❌ `fn pass_builder::LLVMCreatePassBuilderOptions`

❌ `fn pass_builder::LLVMDisposePassBuilderOptions`

❌ `fn pass_builder::LLVMPassBuilderOptionsSetCallGraphProfile`

❌ `fn pass_builder::LLVMPassBuilderOptionsSetDebugLogging`

❌ `fn pass_builder::LLVMPassBuilderOptionsSetForgetAllSCEVInLoopUnroll`

❌ `fn pass_builder::LLVMPassBuilderOptionsSetLicmMssaNoAccForPromotionCap`

❌ `fn pass_builder::LLVMPassBuilderOptionsSetLicmMssaOptCap`

❌ `fn pass_builder::LLVMPassBuilderOptionsSetLoopInterleaving`

❌ `fn pass_builder::LLVMPassBuilderOptionsSetLoopUnrolling`

❌ `fn pass_builder::LLVMPassBuilderOptionsSetLoopVectorization`

❌ `fn pass_builder::LLVMPassBuilderOptionsSetMergeFunctions`

❌ `fn pass_builder::LLVMPassBuilderOptionsSetSLPVectorization`

❌ `fn pass_builder::LLVMPassBuilderOptionsSetVerifyEach`

❌ `fn pass_builder::LLVMRunPasses`

❌ `type pass_builder::LLVMPassBuilderOptionsRef`

❌ `enum pass_manager_builder::LLVMOpaquePassManagerBuilder`

❌ `fn pass_manager_builder::LLVMPassManagerBuilderCreate`

❌ `fn pass_manager_builder::LLVMPassManagerBuilderDispose`

❌ `fn pass_manager_builder::LLVMPassManagerBuilderPopulateFunctionPassManager`

❌ `fn pass_manager_builder::LLVMPassManagerBuilderPopulateLTOPassManager`

❌ `fn pass_manager_builder::LLVMPassManagerBuilderPopulateModulePassManager`

❌ `fn pass_manager_builder::LLVMPassManagerBuilderSetDisableSimplifyLibCalls`

❌ `fn pass_manager_builder::LLVMPassManagerBuilderSetDisableUnitAtATime`

❌ `fn pass_manager_builder::LLVMPassManagerBuilderSetDisableUnrollLoops`

❌ `fn pass_manager_builder::LLVMPassManagerBuilderSetOptLevel`

❌ `fn pass_manager_builder::LLVMPassManagerBuilderSetSizeLevel`

❌ `fn pass_manager_builder::LLVMPassManagerBuilderUseInlinerWithThreshold`

❌ `type pass_manager_builder::LLVMPassManagerBuilderRef`

❌ `fn scalar::LLVMAddAggressiveDCEPass`

❌ `fn scalar::LLVMAddAlignmentFromAssumptionsPass`

❌ `fn scalar::LLVMAddBasicAliasAnalysisPass`

❌ `fn scalar::LLVMAddBitTrackingDCEPass`

❌ `fn scalar::LLVMAddCFGSimplificationPass`

❌ `fn scalar::LLVMAddCorrelatedValuePropagationPass`

❌ `fn scalar::LLVMAddDCEPass`

❌ `fn scalar::LLVMAddDeadStoreEliminationPass`

❌ `fn scalar::LLVMAddDemoteMemoryToRegisterPass`

❌ `fn scalar::LLVMAddEarlyCSEMemSSAPass`

❌ `fn scalar::LLVMAddEarlyCSEPass`

❌ `fn scalar::LLVMAddGVNPass`

❌ `fn scalar::LLVMAddIndVarSimplifyPass`

❌ `fn scalar::LLVMAddInstructionCombiningPass`

❌ `fn scalar::LLVMAddInstructionSimplifyPass`

❌ `fn scalar::LLVMAddJumpThreadingPass`

❌ `fn scalar::LLVMAddLICMPass`

❌ `fn scalar::LLVMAddLoopDeletionPass`

❌ `fn scalar::LLVMAddLoopIdiomPass`

❌ `fn scalar::LLVMAddLoopRerollPass`

❌ `fn scalar::LLVMAddLoopRotatePass`

❌ `fn scalar::LLVMAddLoopUnrollAndJamPass`

❌ `fn scalar::LLVMAddLoopUnrollPass`

❌ `fn scalar::LLVMAddLoopUnswitchPass`

❌ `fn scalar::LLVMAddLowerAtomicPass`

❌ `fn scalar::LLVMAddLowerConstantIntrinsicsPass`

❌ `fn scalar::LLVMAddLowerExpectIntrinsicPass`

❌ `fn scalar::LLVMAddMemCpyOptPass`

❌ `fn scalar::LLVMAddMergedLoadStoreMotionPass`

❌ `fn scalar::LLVMAddNewGVNPass`

❌ `fn scalar::LLVMAddPartiallyInlineLibCallsPass`

❌ `fn scalar::LLVMAddReassociatePass`

❌ `fn scalar::LLVMAddSCCPPass`

❌ `fn scalar::LLVMAddScalarReplAggregatesPass`

❌ `fn scalar::LLVMAddScalarReplAggregatesPassSSA`

❌ `fn scalar::LLVMAddScalarReplAggregatesPassWithThreshold`

❌ `fn scalar::LLVMAddScalarizerPass`

❌ `fn scalar::LLVMAddScopedNoAliasAAPass`

❌ `fn scalar::LLVMAddSimplifyLibCallsPass`

❌ `fn scalar::LLVMAddTailCallEliminationPass`

❌ `fn scalar::LLVMAddTypeBasedAliasAnalysisPass`

❌ `fn scalar::LLVMAddUnifyFunctionExitNodesPass`

❌ `fn scalar::LLVMAddVerifierPass`

❌ `fn util::LLVMAddAddDiscriminatorsPass`

❌ `fn util::LLVMAddLowerSwitchPass`

❌ `fn util::LLVMAddPromoteMemoryToRegisterPass`

❌ `fn vectorize::LLVMAddLoopVectorizePass`

❌ `fn vectorize::LLVMAddSLPVectorizePass`
</details>