use core::alloc::{GlobalAlloc, Layout};
pub struct NDSAllocator;
use crate::{print, println};
unsafe impl GlobalAlloc for NDSAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        //TODO: Check if it's a DSi or a DS, DSi has 16MB of RAM compared to DS's 4
        if layout.size() > 1024 * 1024 * 4 {
            panic!(
                "Attempted to allocate more than 4 MB ( {} bytes )",
                layout.size()
            )
        } else {
            crate::malloc(layout.size() as u32) as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        crate::free(ptr as *mut _);
    }
}

//Taken from https://github.com/rust-wii/ogc-rs
#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    unsafe {
        println!("#######################################");
        println!("# <[ ALLOC ]> Allocation Error!");
        println!(
            "# <[ ALLOC ]> Size: {} - Alignment: {}",
            layout.size(),
            layout.align()
        );
        println!("#######################################");
    }
    loop {}
}
