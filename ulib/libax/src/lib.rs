#![cfg_attr(not(test), no_std)]

pub mod time;

cfg_if::cfg_if! {
    if #[cfg(feature = "user")] {
        mod syscall;
        pub use syscall::*;
    } else {
        pub use axlog::{debug, error, info, print, println, trace, warn};
        
        #[cfg(feature = "alloc")]
        extern crate alloc;
        
        #[macro_use]
        extern crate axlog;
        
        #[cfg(not(test))]
        extern crate axruntime;
        
        pub mod io;
        pub mod rand;
        pub mod sync;
        
        
        #[cfg(feature = "multitask")]
        pub mod task;
        
        #[cfg(feature = "net")]
        pub mod net;
        
        #[cfg(feature = "display")]
        pub mod display;
    }
}
