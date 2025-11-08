#![no_std]
#![no_main]

use common::{error_string, sys_close, sys_exit, sys_open, sys_read, sys_write};

#[unsafe(naked)]
#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        r#"
            mov rdi, [rsp]
            lea rsi, [rsp + 8]
            jmp entry
        "#
    )
}

#[unsafe(no_mangle)]
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
