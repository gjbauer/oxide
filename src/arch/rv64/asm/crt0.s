.section .init

.option norvc

.type start, @function
.global start
start:
    .cfi_startproc
    
.option push
.option norelax
    la gp, global_pointer
.option pop
    
    /* Reset satp */
    csrw satp, zero
    
    /* Setup stack */
    la sp, stack_top
    
    /* Set the trap handler address */
    la t0, trap_vector
    csrw mtvec, t0

    /* Enable machine external interrupts (MEIE, bit 11) */
    li t0, (1 << 11)
    csrs mie, t0

    /* Enable machine interrupts globally (MIE, bit 3) */
    li t0, (1 << 3)
    csrs mstatus, t0
    
    /* Clear the BSS section */
    la t5, bss_start
    la t6, bss_end
bss_clear:
    sd zero, (t5)
    addi t5, t5, 8
    bltu t5, t6, bss_clear
    
    la t0, kmain
    csrw mepc, t0
    
    /* Jump to kernel! */
    tail kmain
    
    .cfi_endproc

.section .text
.balign 4
.globl trap_vector

trap_vector:
    # Allocate space for 32 registers (RV64)
    addi sp, sp, -256

    # Save caller-saved registers
    sd ra,   0(sp)
    sd t0,   8(sp)
    sd t1,  16(sp)
    sd t2,  24(sp)
    sd t3,  32(sp)
    sd t4,  40(sp)
    sd t5,  48(sp)
    sd t6,  56(sp)

    sd a0,  64(sp)
    sd a1,  72(sp)
    sd a2,  80(sp)
    sd a3,  88(sp)
    sd a4,  96(sp)
    sd a5, 104(sp)
    sd a6, 112(sp)
    sd a7, 120(sp)

    # Pass mcause to Rust
    csrr a0, mcause

    # Call Rust handler
    call trap_handler

    # Restore registers
    ld ra,   0(sp)
    ld t0,   8(sp)
    ld t1,  16(sp)
    ld t2,  24(sp)
    ld t3,  32(sp)
    ld t4,  40(sp)
    ld t5,  48(sp)
    ld t6,  56(sp)

    ld a0,  64(sp)
    ld a1,  72(sp)
    ld a2,  80(sp)
    ld a3,  88(sp)
    ld a4,  96(sp)
    ld a5, 104(sp)
    ld a6, 112(sp)
    ld a7, 120(sp)

    addi sp, sp, 256

    # Return to interrupted execution
    mret

.end
