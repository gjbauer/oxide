mod plic;
use crate::uart;

fn is_interrupt(signal: usize) -> bool {
    ((signal) >> 63) != 0
}

#[unsafe(no_mangle)]
pub extern "C" fn trap_handler(mcause: usize)
{
    let cause = mcause & 0x7FFF_FFFF_FFFF_FFFF;
    if is_interrupt(mcause) && cause == 11 {
        let irq;
        unsafe { irq = core::ptr::read_volatile(plic::PLIC_CLAIM); }
        
        match irq {
            10 => {
                uart::putchar(uart::getchar());
            }
            _ => {
                ()
            }
        }
        
        if irq != 0 {
            unsafe {
                core::ptr::write_volatile(
                    plic::PLIC_CLAIM,
                    irq,
                    );
            }
        }
    }
}
