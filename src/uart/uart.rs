const UART_BASE: *mut u8 = 0x1000_0000 as *mut u8;

pub fn uart_init() {
    unsafe {
        // Disable all UART interrupts
        core::ptr::write_volatile(
            (UART_BASE.add(1)) as *mut u8,
            0,
        );

        // Enable 8-bit words, DLAB = 0
        core::ptr::write_volatile(
            (UART_BASE.add(3)) as *mut u8,
            0x03,
        );

        // Enable received-data interrupts
        core::ptr::write_volatile(
            (UART_BASE.add(1)) as *mut u8,
            0x01,
        );
    }
}

pub fn getchar() -> u8
{
    unsafe {
        core::ptr::read_volatile(UART_BASE)
    }
}

pub fn putchar(c: u8) {
    unsafe {
        core::ptr::write_volatile(UART_BASE, c);
    }
}

pub fn print(s: &str) {
    for byte in s.bytes() {
        putchar(byte);
    }
}
