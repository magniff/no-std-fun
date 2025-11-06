#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
fn sys_write(fd: usize, buffer: *const u8, size: usize) -> usize {
    let mut bytes_written: usize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 1,
            in("rdi") fd,
            in("rsi") buffer,
            in("rdx") size,
            out("rcx") _,
            out("r11") _,
            lateout("rax") bytes_written,
        );
    }
    bytes_written
}

fn print_string(value: &str) -> usize {
    sys_write(1, value.as_ptr(), value.len())
}

fn error_string(value: &str) -> usize {
    sys_write(2, value.as_ptr(), value.len())
}

#[unsafe(no_mangle)]
fn render_usize(mut value: usize, buffer: &mut [u8]) -> &[u8] {
    let mut offset = buffer.len() - 1;
    let digits = b"0123456789abcdef";

    if value == 0 {
        buffer[offset] = '0' as u8;
        offset -= 1;
    }

    while value > 0 {
        buffer[offset] = digits[value & 0xf] as u8;
        value >>= 4;
        offset -= 1;
    }

    buffer[offset] = 'x' as u8;
    offset -= 1;
    buffer[offset] = '0' as u8;
    offset -= 1;

    &buffer[offset + 1..]
}

#[unsafe(no_mangle)]
fn print_usize(value: usize) -> usize {
    let mut buffer = [0u8; 20];
    let rendered_portion = render_usize(value, &mut buffer);
    sys_write(1, rendered_portion.as_ptr(), rendered_portion.len())
}

#[unsafe(no_mangle)]
extern "C" fn rust_eh_personality() {}

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
