#![no_std]
#![no_main]

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(naked)]
#[no_mangle]
extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        r#"
            mov rdi, [rsp]
            lea rsi, [rsp + 8]
            jmp entry
        "#
    )
}

#[no_mangle]
extern "C" fn entry(argc: usize, argv: *const *const u8) -> ! {
    let mut argument_counter = 1usize;

    unsafe {
        while argument_counter < argc {
            let mut offset = 0usize;
            while *(*argv.add(argument_counter)).add(offset) != 0 {
                offset += 1;
            }
            let fd = sys_open(*argv.add(argument_counter));
            if fd < 0 {
                error_string("Error: Cant read the file ");
                sys_write(2, *argv.add(argument_counter), offset);
                error_string("\n");
                argument_counter += 1;
                continue;
            }

            let mut inner_buffer = [0u8; 4096 * 10];
            let mut bytes_read =
                sys_read(fd as usize, inner_buffer.as_mut_ptr(), inner_buffer.len());

            while bytes_read > 0 {
                sys_write(1, inner_buffer.as_ptr(), bytes_read);
                bytes_read = sys_read(fd as usize, inner_buffer.as_mut_ptr(), inner_buffer.len());
            }
            sys_close(fd as usize);
            argument_counter += 1;
        }
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

#[no_mangle]
fn sys_read(fd: usize, buffer: *mut u8, size: usize) -> usize {
    let mut byte_read: usize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 0,
            in("rdi") fd,
            in("rsi") buffer,
            in("rdx") size,
            out("rcx") _,
            out("r11") _,
            lateout("rax") byte_read,
        );
    }
    byte_read
}

#[no_mangle]
fn sys_open(filename: *const u8) -> isize {
    let mut fd: isize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 2,
            in("rdi") filename,
            in("rsi") 0,
            in("rdx") 0,
            out("rcx") _,
            out("r11") _,
            lateout("rax") fd,
        );
    }
    fd
}

#[no_mangle]
fn sys_close(fd: usize) {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 3,
            in("rdi") fd,
            out("rcx") _,
            out("r11") _,
        );
    }
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

fn error_string(value: &str) -> usize {
    sys_write(2, value.as_ptr(), value.len())
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
