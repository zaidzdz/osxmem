use mach::vm::mach_vm_read_overwrite;
use mach::kern_return::{kern_return_t, KERN_SUCCESS};
use crate::process::{Process, ProcessErr};
/// Reads the specified type in the given processes memory at the given address
pub fn read_mem<T: Copy>(process: &Process, address: usize) -> Result<T, ProcessErr> {
    let mut buf: Vec<u8>= vec![0u8; std::mem::size_of::<T>()];
    let mut outsize: u64 = 0;
    let kern_ret:kern_return_t = unsafe {
        mach_vm_read_overwrite(
            process.port,
            address as u64,
            buf.len() as u64,
            buf.as_mut_ptr() as u64,
            &mut outsize,
        )
    };
    if kern_ret != KERN_SUCCESS {
        return Err(ProcessErr::from_kern(kern_ret));
    }
    Ok(unsafe { std::ptr::read(buf.as_ptr() as *const T) })
}
