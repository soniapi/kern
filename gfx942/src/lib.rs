#![no_std]
#![feature(abi_gpu_kernel)]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "gpu-kernel" fn add_vector_float(
    a: *const f32,
    b: *const f32,
    c: *mut f32,
    n: usize,
) {
    let gid = unsafe {
        get_global_id()
    };

    if gid < n as u32 {
        unsafe {
            *c.add(gid as usize) = *a.add(gid as usize) + *b.add(gid as usize);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

unsafe extern "C" {
    fn get_global_id() -> u32;
}
