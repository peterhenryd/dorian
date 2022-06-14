use std::mem::MaybeUninit;
use std::path::Path;
use std::ptr::NonNull;
use llvm_sys::core::{LLVMCreateMemoryBufferWithContentsOfFile, LLVMCreateMemoryBufferWithMemoryRange, LLVMCreateMemoryBufferWithMemoryRangeCopy, LLVMCreateMemoryBufferWithSTDIN, LLVMDisposeMemoryBuffer, LLVMGetBufferSize, LLVMGetBufferStart};
use llvm_sys::LLVMMemoryBuffer;
use crate::llvm::{Error, from_c_string, to_c_string};

#[derive(Debug, Clone)]
pub struct MemoryBuffer(NonNull<LLVMMemoryBuffer>);

impl MemoryBuffer {
    pub fn from_raw(raw: NonNull<LLVMMemoryBuffer>) -> MemoryBuffer {
        MemoryBuffer(raw)
    }

    pub fn as_raw(&self) -> NonNull<LLVMMemoryBuffer> {
        self.0
    }

    pub fn from_path(path: &Path) -> Result<Self, Error> {
        unsafe {
            let mut memory_buffer = MaybeUninit::uninit();
            let mut message = MaybeUninit::uninit();
            let result = LLVMCreateMemoryBufferWithContentsOfFile(
                to_c_string(Some(path.to_str().unwrap())).as_ptr(),
                memory_buffer.as_mut_ptr(),
                message.as_mut_ptr()
            );

            if result == 0 {
                Ok(MemoryBuffer::from_raw(NonNull::new_unchecked(memory_buffer.assume_init())))
            } else {
                Err(Error::create(from_c_string(message.assume_init())))
            }
        }
    }

    pub fn from_stdin() -> Result<Self, Error> {
        unsafe {
            let mut memory_buffer = MaybeUninit::uninit();
            let mut message = MaybeUninit::uninit();
            let result = LLVMCreateMemoryBufferWithSTDIN(
                memory_buffer.as_mut_ptr(),
                message.as_mut_ptr()
            );

            if result == 0 {
                Ok(MemoryBuffer::from_raw(NonNull::new_unchecked(memory_buffer.assume_init())))
            } else {
                Err(Error::create(from_c_string(message.assume_init())))
            }
        }
    }

    pub fn from_memory_range(input: &str, buffer_name: &str, requires_null_terminator: bool) -> Self {
        MemoryBuffer::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMCreateMemoryBufferWithMemoryRange(
                    to_c_string(Some(input)).as_ptr(),
                    input.len(),
                    to_c_string(Some(buffer_name)).as_ptr(),
                    requires_null_terminator as i32,
                )
            )
        })
    }

    pub fn from_memory_range_copy(input: &str, buffer_name: &str) -> Self {
        MemoryBuffer::from_raw(unsafe {
            NonNull::new_unchecked(
                LLVMCreateMemoryBufferWithMemoryRangeCopy(
                    to_c_string(Some(input)).as_ptr(),
                    input.len(),
                    to_c_string(Some(buffer_name)).as_ptr(),
                )
            )
        })
    }

    pub fn get_buffer_start(&self) -> &str {
        unsafe { from_c_string(LLVMGetBufferStart(self.0.as_ptr())) }
    }

    pub fn get_buffer_size(&self) -> usize {
        unsafe { LLVMGetBufferSize(self.0.as_ptr()) }
    }
}

impl Drop for MemoryBuffer {
    fn drop(&mut self) {
        unsafe { LLVMDisposeMemoryBuffer(self.0.as_ptr()); }
    }
}