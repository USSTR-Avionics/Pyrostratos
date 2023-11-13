use core::mem::MaybeUninit;

use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

pub fn init_allocator() {
    const HEAP_SIZE: usize = 1024;
    static mut HEAP_SPACE: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe {
        HEAP.init(HEAP_SPACE.as_ptr() as usize, HEAP_SIZE);
    }
}
