.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

    .section .text
    .global __alltraps
    .global __restore
    .align 2

__alltraps:
    # 1. 保存用户态的 sp 到 sscratch，并切换到内核栈
    csrrw sp, sscratch, sp
    # 2. 在内核栈上分配空间
    addi sp, sp, -32*8
    # 3. 保存 32 个通用寄存器
    .set n, 0
    .rept 32
        SAVE_GP %n
        .set n, n+1
    .endr
    # 4. 保存 sstatus 和 sepc 到栈上
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # 5. 调用 trap_handler，参数是 Context 的地址
    mv a0, sp
    call trap_handler

__restore:
    # 1. 恢复 sstatus 和 sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    # 2. 恢复通用寄存器
    .set n, 0
    .rept 32
        LOAD_GP %n
        .set n, n+1
    .endr
    # 3. 恢复 sp
    ld sp, 2*8(sp)
    # 4. 切换回用户栈并返回用户态
    csrrw sp, sscratch, sp
    sret
