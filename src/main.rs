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

// no name mangling so we need to use the #[no_mangle] attribute to prevent the compiler from renaming the function
// this is the entry point of the program
// should use the C calling convention. Not sure why 
// this is a diverging function so it never returns. This is because it is called by the operating system or bootloader
// instead of returning it should call exit system call or 
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}