use core::panic::PanicInfo;
use nds_sys::allocator::NDSAllocator;
use nds_sys::{print, println};

#[global_allocator]
static GLOBAL_ALLOCATOR: NDSAllocator = NDSAllocator;

//Taken from https://github.com/rust-wii/ogc-rs
#[panic_handler]
fn panic_handler(panic_info: &PanicInfo) -> ! {
    unsafe {
        println!("#######################################");
        println!("# <[ PANIC ]> {} ", panic_info);
        println!("#######################################");
    }
    loop {}
}
