#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

mod symbols;
mod syscalls;

pub use syscalls::*;

pub fn print_string(value: &str) -> usize {
    sys_write(1, value.as_ptr(), value.len())
}

pub fn error_string(value: &str) -> usize {
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
pub fn print_usize(value: usize) -> usize {
    let mut buffer = [0u8; 20];
    let rendered_portion = render_usize(value, &mut buffer);
    sys_write(1, rendered_portion.as_ptr(), rendered_portion.len())
}
