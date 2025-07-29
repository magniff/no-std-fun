#![no_std]
#![no_main]

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn _start() -> ! {
    for value in usize::MAX - 10..=usize::MAX {
        print_string("-- ");
        print_usize(value);
        print_string("\n");
    }
    sys_exit(0);
}

#[no_mangle]
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

fn sys_exit(code: usize) -> ! {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 60,
            in("rdi") code,
        )
    }
    unreachable!()
}

fn print_string(value: &str) -> usize {
    sys_write(1, value.as_ptr(), value.len())
}

#[no_mangle]
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

#[no_mangle]
fn print_usize(value: usize) -> usize {
    let mut buffer = [0u8; 20];
    let rendered_portion = render_usize(value, &mut buffer);
    sys_write(1, rendered_portion.as_ptr(), rendered_portion.len())
}

#[no_mangle]
extern "C" fn memset(buffer: *mut u8, value: u8, size: usize) -> *const u8 {
    let mut offset = 0usize;
    unsafe {
        while offset < size {
            *buffer.add(offset) = value;
            offset += 1;
        }
    }
    buffer
}

#[no_mangle]
extern "C" fn rust_eh_personality() {}
