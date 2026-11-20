#![no_std]
#![no_main]

#[path = "arch/rv64/plic/plic.rs"]
mod plic;
#[path = "arch/rv64/plic/trap_handler.rs"]
mod trap_handler;
#[path = "uart/uart.rs"]
mod uart;

use crate::uart::*;
use crate::plic::plic_init;
use core::arch::global_asm;

global_asm!(include_str!("arch/rv64/asm/crt0.s"));

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    plic_init();
    uart_init();
    
    print("Hello, world!\n");

    loop {}
}
