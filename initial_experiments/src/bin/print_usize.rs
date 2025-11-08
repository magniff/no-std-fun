#![no_std]
#![no_main]

use common::{print_string, print_usize, sys_exit};

#[unsafe(no_mangle)]
fn _start() -> ! {
    for value in usize::MAX - 10..=usize::MAX {
        print_string("-- ");
        print_usize(value);
        print_string("\n");
    }
    sys_exit(0);
}
