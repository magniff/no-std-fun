#[cfg(not(test))]
#[panic_handler]
#[allow(unused_variables)]
pub fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
extern "C" fn rust_eh_personality() {}
