pub fn sys_exit(code: usize) -> ! {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 60,
            in("rdi") code,
        )
    }
    unreachable!()
}

#[unsafe(no_mangle)]
pub fn sys_read(fd: usize, buffer: *mut u8, size: usize) -> usize {
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

#[unsafe(no_mangle)]
pub fn sys_write(fd: usize, buffer: *const u8, size: usize) -> usize {
    let mut bytes_written: usize;
    unsafe {
        core::arch::asm!(
            "syscall", in("rax") 1, in("rdi") fd,
            in("rsi") buffer,
            in("rdx") size,
            out("rcx") _,
            out("r11") _,
            lateout("rax") bytes_written,
        );
    }
    bytes_written
}

#[unsafe(no_mangle)]
pub fn sys_open(filename: *const u8) -> isize {
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

#[unsafe(no_mangle)]
pub fn sys_close(fd: usize) {
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
