//! # memedit
//!
//! A macOS library for reading and writing process memory using the Mach kernel APIs.
//!
//! # Example
//!
//! ```no_run
//! use memedit::process::Process;
//! use memedit::memory::{read_mem, write_mem};
//!
//! let health_address: usize = 0x7060E5800;
//! // Attach to process with name "Game"
//! let process: Process = Process::open_by_name("Game").unwrap();
//! // Read a f32 value (float) at the "health_address"
//! let mut value: f32 = read_mem::<f32>(&process, health_address).unwrap();
//! println!("{}", value);
//! // Write 200 to the "health_address"
//! write_mem::<f32>(&process, health_address, 200.0).unwrap();
//!  // Read the same address again
//! value = read_mem::<f32>(&process, health_address).unwrap();
//! println!("{}", value);
//! ```

pub mod memory;
pub mod process;
