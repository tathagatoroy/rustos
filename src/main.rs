// no standard library, os binary hence you can't use prebuilt features 
// also no main as we don't have access to the c runtime which calls the main function
// so we need to define our own entry point
#![no_std]
#![no_main]

// no panic handler without standard library so we must define our own 
use core::panic::PanicInfo;
// panic handler
// this function is called when a panic occurs
// it must never return so ! is used to indicate that it never returns . This is a diverging function
// & PanicInfo is a reference to a struct that contains information about the panic like the location of the panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello, world!\n";
// u8 is an unsigned 8-bit integer
// b"Hello, world!\n" is a byte string literal
// this is a static variable that contains the string "Hello, world!\n" as a byte array

// no name mangling so we need to use the #[no_mangle] attribute to prevent the compiler from renaming the function
// this is the entry point of the program
// should use the C calling convention. Not sure why 
// this is a diverging function so it never returns. This is because it is called by the operating system or bootloader
// instead of returning it should call exit system call or 
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8; // VGA buffer address
    // 0xb8000 is the address of the VGA buffer in memory
    // this can be use to write to the screen
    // *mut u8 is a mutable pointer to an 8-bit unsigned integer

    for(i, &byte) in HELLO.iter().enumerate(){
        unsafe {
            // unsafe block is used to indicate that this code is unsafe and can cause undefined behavior
            // this is because we are dereferencing a raw pointer
            // we need to use unsafe block to indicate that we are aware of the risks
            *vga_buffer.offset(i as isize * 2) = byte; // write the byte to the VGA buffer
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // set the attribute byte (light cyan)
            // 0x07 is the attribute byte for cyan on black
            //i as isize converts to isize to usize
            // 2 bytes for each character (one for the character and one for the attribute)
        }
    }
    loop {}
}