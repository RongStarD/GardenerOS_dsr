TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin

BOOTLOADER := ../bootloader/opensbi.bin

OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

build: $(KERNEL_BIN)

user:
	@$(MAKE) -C ../user build

kernel: user
	@cargo build --release

$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

run: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-kernel $(KERNEL_ELF)

clean:
	@cargo clean
	@$(MAKE) -C ../user clean

disasm: kernel
	@$(OBJDUMP) -d $(KERNEL_ELF) | less

.PHONY: build user kernel run clean disasm
