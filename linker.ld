OUTPUT_ARCH(riscv)
ENTRY(start)

SECTIONS {
    . = 0x80000000;

    .text : ALIGN(4K) {
        KEEP(*(.init))
        *(.text .text.*)
    }

    .rodata : ALIGN(4K) {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(4K) {
        *(.data .data.*)
    }

    .bss (NOLOAD) : ALIGN(4K) {
        PROVIDE(bss_start = .);
        *(.sbss .sbss.*)
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(8);
        PROVIDE(bss_end = .);
    }

    . = ALIGN(16);
    . += 16K;
    PROVIDE(stack_top = .);

    PROVIDE(global_pointer = . + 0x800);
}
