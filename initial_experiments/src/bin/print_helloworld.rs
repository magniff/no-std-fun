#![no_std]
#![no_main]

use common::{print_string, sys_exit};

#[unsafe(no_mangle)]
fn _start() -> ! {
    print_string("Hello, world!\n");
    sys_exit(0);
}
