const PLIC_BASE: *mut u8 = 0x0C00_0000 as *mut u8;
pub const PLIC_CLAIM: *mut u32 = 0x0C20_0004 as *mut u32;

macro_rules! plic_priority {
    ($irq:expr) => {
        PLIC_BASE + 4 * $irq
    };
}

macro_rules! plic_pending {
    () => {
        PLIC_BASE + 0x1000
    };
}

macro_rules! plic_menable {
    ($hart:expr) => {
        PLIC_BASE + 0x2000 + (hart) * 0x100
    };
}

macro_rules! plic_mthreshold {
    ($hart:expr) => {
        PLIC_BASE + 0x200000 + (hart) * 0x2000
    };
}

macro_rules! plic_mclaim {
    ($hart:expr) => {
        PLIC_BASE + 0x200004 + (hart) * 0x2000
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn plic_init() {
    unsafe {
        // Set UART IRQ 10 priority to 1
        core::ptr::write_volatile(
            (PLIC_BASE.add(4 * 10)) as *mut u32,
            5,
        );

        // Enable UART IRQ 10 for hart 0 M-mode
        core::ptr::write_volatile(
            (PLIC_BASE.add(0x2000)) as *mut u32,
            1 << 10,
        );

        // Set priority threshold to 0
        core::ptr::write_volatile(
            (PLIC_BASE.add(0x200000)) as *mut u32,
            0,
        );
    }
}
