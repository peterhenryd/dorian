use std::ptr::NonNull;
use llvm_sys::LLVMMemoryBuffer;

#[derive(Debug, Copy, Clone)]
pub struct MemoryBuffer(NonNull<LLVMMemoryBuffer>);

impl MemoryBuffer {
    pub fn from_raw(raw: NonNull<LLVMMemoryBuffer>) -> MemoryBuffer {
        MemoryBuffer(raw)
    }

    pub fn as_raw(&self) -> NonNull<LLVMMemoryBuffer> {
        self.0
    }

    pub fn LLVMCreateMemoryBufferWithContentsOfFile(
        Path: *const ::libc::c_char,
        OutMemBuf: *mut LLVMMemoryBufferRef,
        OutMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
    pub fn LLVMCreateMemoryBufferWithSTDIN(
        OutMemBuf: *mut LLVMMemoryBufferRef,
        OutMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
    pub fn LLVMCreateMemoryBufferWithMemoryRange(
        InputData: *const ::libc::c_char,
        InputDataLength: ::libc::size_t,
        BufferName: *const ::libc::c_char,
        RequiresNullTerminator: LLVMBool,
    ) -> LLVMMemoryBufferRef;
    pub fn LLVMCreateMemoryBufferWithMemoryRangeCopy(
        InputData: *const ::libc::c_char,
        InputDataLength: ::libc::size_t,
        BufferName: *const ::libc::c_char,
    ) -> LLVMMemoryBufferRef;
    pub fn LLVMGetBufferStart(MemBuf: LLVMMemoryBufferRef) -> *const ::libc::c_char;
    pub fn LLVMGetBufferSize(MemBuf: LLVMMemoryBufferRef) -> ::libc::size_t;
    pub fn LLVMDisposeMemoryBuffer(MemBuf: LLVMMemoryBufferRef);

}
