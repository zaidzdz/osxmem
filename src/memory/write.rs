use mach::vm::{mach_vm_write};
use mach::kern_return::{kern_return_t, KERN_SUCCESS};
use crate::process::{Process, ProcessErr};
/// Writes the given data to the memory in the given process at the given address.
pub fn write_mem<T: Copy>(process: &Process, address: usize, data: T) -> Result<(), ProcessErr> {

    let kern_ret: kern_return_t = unsafe {
        mach_vm_write(
            process.port,
            address as u64,
            &data as *const T as usize,
            std::mem::size_of::<T>() as u32,
        )
    };
    if kern_ret != KERN_SUCCESS {
        return Err(ProcessErr::from_kern(kern_ret));
    }

    Ok(())
}
