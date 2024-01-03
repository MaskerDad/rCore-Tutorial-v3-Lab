//! The main module and entrypoint
//!
//! The operating system and app also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality [`clear_bss()`]. (See its source code for
//! details.)
//!
//! We then call [`println!`] to display `Hello, world!`.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;
use riscv::register::mhartid;
use log::*;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

global_asm!(include_str!("entry.asm"));

/// clear BSS segment
pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

/// STARTED is used to distinguish major/minor hart
pub static mut STARTED: usize = 0;

/// the rust entry-point of os
#[no_mangle]
pub fn rust_main() -> ! {
    let hart_id: usize = mhartid::read();
    if hart_id == 0 {
        logging::init();
        println!("[kernel] major hart {} booting...", hart_id);
        println!("[kernel] Hello, world!");
        unsafe {
            STARTED = 1;
        }
    } else {
        unsafe {
            while STARTED == 0 {};
        }
        println!("[kernel] minor hart {} booting...", hart_id);
        extern "C" {
            fn stext(); // begin addr of text segment
            fn etext(); // end addr of text segment
            fn srodata(); // start addr of Read-Only data segment
            fn erodata(); // end addr of Read-Only data ssegment
            fn sdata(); // start addr of data segment
            fn edata(); // end addr of data segment
            fn sbss(); // start addr of BSS segment
            fn ebss(); // end addr of BSS segment
            fn boot_stack_lower_bound(); // stack lower bound
            fn boot_stack_top(); // stack top
        }
        clear_bss();
        trace!(
            "[kernel] .text [{:#x}, {:#x})",
            stext as usize,
            etext as usize
        );
        debug!(
            "[kernel] .rodata [{:#x}, {:#x})",
            srodata as usize, erodata as usize
        );
        info!(
            "[kernel] .data [{:#x}, {:#x})",
            sdata as usize, edata as usize
        );
        warn!(
            "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
            boot_stack_top as usize, boot_stack_lower_bound as usize
        );
        error!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    }
    // CI autotest failed : sbi::shutdown(true)
    // CI autotest success: sbi::shutdown(false)
    sbi::shutdown(false)
}

