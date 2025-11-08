#![no_std]
#![no_main]

use common::{print_string, print_usize, sys_exit, sys_write};

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
    let mut argument_counter = 0usize;
    print_string("-- argc: ");
    print_usize(argc);
    print_string("\n");
    unsafe {
        while argument_counter < argc {
            let mut offset = 0usize;
            while *(*argv.add(argument_counter)).add(offset) != 0 {
                offset += 1;
            }
            print_usize(argument_counter);
            print_string(": ");
            sys_write(1, *argv.add(argument_counter), offset);
            print_string("\n");
            argument_counter += 1;
        }
    }
    sys_exit(0);
}
