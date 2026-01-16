# Corn-Sprite Makefile

TARGET := riscv64gc-unknown-none-elf
KERNEL_ELF := target/$(TARGET)/release/kernel
KERNEL_BIN := $(KERNEL_ELF).bin

QEMU := qemu-system-riscv64
QEMU_OPTS := -machine virt \
             -nographic \
             -bios none \
             -kernel $(KERNEL_ELF)

.PHONY: all build run debug clean fmt check

all: build

build:
	cargo build --release -p kernel

$(KERNEL_BIN): build
	rust-objcopy --strip-all -O binary $(KERNEL_ELF) $(KERNEL_BIN)

run: build
	$(QEMU) $(QEMU_OPTS)

# Run with GDB server on port 1234
debug: build
	$(QEMU) $(QEMU_OPTS) -s -S

# Run with limited output (for testing)
test-boot: build
	timeout 5 $(QEMU) $(QEMU_OPTS) || true

clean:
	cargo clean

fmt:
	cargo fmt --all

check:
	cargo check --all
