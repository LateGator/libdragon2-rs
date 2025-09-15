use core::alloc::{AllocError, Allocator};
use core::{alloc::GlobalAlloc, ptr::NonNull};

use crate::sys::malloc::*;

pub struct System;

#[global_allocator]
static ALLOCATOR: System = System;

const MIN_ALIGN: usize = size_of::<*const ()>() * 2;

unsafe impl Allocator for System {
    #[inline]
    fn allocate(&self, layout: core::alloc::Layout) -> Result<NonNull<[u8]>, AllocError> {
        NonNull::new(unsafe { self.alloc(layout) })
            .map(|p| NonNull::slice_from_raw_parts(p, layout.size()))
            .ok_or(AllocError)
    }

    #[inline]
    unsafe fn deallocate(&self, ptr: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        unsafe { self.dealloc(ptr.as_ptr(), layout) }
    }

    #[inline]
    fn allocate_zeroed(&self, layout: core::alloc::Layout) -> Result<NonNull<[u8]>, AllocError> {
        NonNull::new(unsafe { self.alloc_zeroed(layout) })
            .map(|p| NonNull::slice_from_raw_parts(p, layout.size()))
            .ok_or(AllocError)
    }

    #[inline]
    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: core::alloc::Layout,
        new_layout: core::alloc::Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        debug_assert!(
            new_layout.size() >= old_layout.size(),
            "`new_layout.size()` must be greater than or equal to `old_layout.size()`"
        );
        NonNull::new(unsafe { self.realloc(ptr.as_ptr(), new_layout, old_layout.size()) })
            .map(|p| NonNull::slice_from_raw_parts(p, new_layout.size()))
            .ok_or(AllocError)
    }

    #[inline]
    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        old_layout: core::alloc::Layout,
        new_layout: core::alloc::Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            self.grow(ptr, old_layout, new_layout).map(|mut p| {
                p.as_mut().get_unchecked_mut(old_layout.size()..).fill(0);
                p
            })
        }
    }

    #[inline]
    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: core::alloc::Layout,
        new_layout: core::alloc::Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        debug_assert!(
            new_layout.size() <= old_layout.size(),
            "`new_layout.size()` must be smaller than or equal to `old_layout.size()`"
        );
        NonNull::new(unsafe { self.realloc(ptr.as_ptr(), new_layout, old_layout.size()) })
            .map(|p| NonNull::slice_from_raw_parts(p, new_layout.size()))
            .ok_or(AllocError)
    }
}

unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let (Ok(align), Ok(size)) = (<_>::try_from(layout.align()), <_>::try_from(layout.size()))
        else {
            return core::ptr::null_mut();
        };
        unsafe { aligned_alloc(align, size) as *mut u8 }
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: core::alloc::Layout) -> *mut u8 {
        let (Ok(align), Ok(size)) = (<_>::try_from(layout.align()), <_>::try_from(layout.size()))
        else {
            return core::ptr::null_mut();
        };
        if align <= MIN_ALIGN as _ && align <= size {
            unsafe { calloc(size, 1) as *mut u8 }
        } else {
            let ptr = unsafe { aligned_alloc(align, size) } as *mut u8;
            if !ptr.is_null() {
                unsafe { core::ptr::write_bytes(ptr, 0, size as usize) };
            }
            ptr
        }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        unsafe { free(ptr as _) }
    }

    #[inline]
    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: core::alloc::Layout,
        new_size: usize,
    ) -> *mut u8 {
        let (Ok(align), Ok(new_size)) = (<_>::try_from(layout.align()), <_>::try_from(new_size))
        else {
            return core::ptr::null_mut();
        };
        if align <= MIN_ALIGN as _ && align <= new_size {
            unsafe { realloc(ptr as _, new_size) as *mut u8 }
        } else {
            let new_ptr = unsafe { aligned_alloc(align, new_size) } as *mut u8;
            if !new_ptr.is_null() {
                unsafe {
                    core::ptr::copy_nonoverlapping(ptr, new_ptr, layout.size().min(new_size as _));
                    self.dealloc(ptr, layout);
                }
            }
            new_ptr
        }
    }
}
