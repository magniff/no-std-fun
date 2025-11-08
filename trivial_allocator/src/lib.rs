#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

use common::{print_string, print_usize};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
    print_string("-----> allocating ");
    print_usize(size);
    let mut ptr: *mut u8;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 9,
            in("rdi") 0,
            in("rsi") size + core::mem::size_of::<usize>(),
            in("rdx") 0x1 | 0x2,
            in("r10") 0x2 | 0x20,
            in("r8") -1,
            in("r9") 0,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ptr,
        );
    }
    print_string(" address ");
    print_usize(ptr as usize);
    print_string("\n");
    *(ptr as *mut usize) = size;
    ptr.add(8)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free(ptr: *const u8) {
    if ptr.is_null() {
        return;
    }

    print_string("-----> freeing ");
    print_usize(ptr as usize);
    print_string("\n");

    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 11,
            in("rdi") ptr.sub(8),
            in("rsi") *(ptr as *const usize).sub(1) + 8,
            out("rcx") _,
            out("r11") _,
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn realloc(ptr: *const u8, new_size: usize) -> *mut u8 {
    print_usize(ptr as usize);
    print_string(" ");
    print_usize(new_size as usize);
    print_string("\n");
    if !ptr.is_null() && new_size == 0 {
        free(ptr);
        return ptr as *mut _;
    }
    if ptr.is_null() && new_size > 0 {
        return malloc(new_size);
    }
    let new_allocation = malloc(new_size);
    let old_size = *(ptr as *const usize).sub(1);
    core::ptr::copy_nonoverlapping(ptr, new_allocation, new_size.min(old_size));
    free(ptr);
    new_allocation
}
