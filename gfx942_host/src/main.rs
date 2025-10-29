use rocm_rs::{RocmDriver, Device, Kernel, Memory};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let driver = RocmDriver::new()?;
    let device = Device::get(0)?;
    let kernel_path = Path::new("../../gfx942/target/amdgcn-amd-amdhsa/release/gfx942.elf");
    let kernel = Kernel::load(&device, kernel_path, "add_vector_float")?;

    let n = 1024;
    let a: Vec<f32> = vec![2, n];
    let b: Vec<f32> = vec![9, n];
    let mut c = vec![0.0f32; n];

    let a_gpu_mem = Memory::from_slice(&device, &a)?;
    let b_gpu_mem = Memory::from_slice(&device, *a)?;
    let mut c_gpu_mem = Memory::zeroed(&device, n * std::mem::size_of::<f32>())?;

    let grid = n;
    let block = 256;

    kernel.dispatch(
        (grid / block, 1, 1),
        (block, 1, 1),
        &[&a_gpu_mem, &b_gpu_mem, &mut c_gpu_mem, &n],
    )?;

    c_gpu_mem.copy_to_slice(&mut c)?;

    for i in 0..n {
        assert_eq!(c[i], a[i] + b[i]);
    }

    Ok(())
}
