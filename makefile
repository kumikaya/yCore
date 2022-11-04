# Building
TARGET := riscv64gc-unknown-none-elf
MODE := debug
KERNEL_NAME = yife-os
KERNEL_ELF = target/$(TARGET)/$(MODE)/$(KERNEL_NAME)
KERNEL_BIN = $(KERNEL_ELF).bin
FEATURES = 
# debug_test
QEMU_ARGS := 
CARGO_ARGS :=
gdb = true
# Log level: error | warn | info
export LOG ?= info

test_build: 
	@cargo build --tests

build:
	@cargo build --features "$(FEATURES)"

$(KERNEL_BIN): build
	@rust-objcopy --strip-all $(KERNEL_ELF) -O binary $@


KERNEL_ENTRY_ADDR := 0x80200000
BOARD ?= qemu
SBI ?= rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin
# GDB_PORT := 9433

run_only:
	@qemu-system-riscv64 \
      -machine virt \
      -nographic \
      -bios $(BOOTLOADER) \
      -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_ADDR) \
	  $(QEMU_ARGS) \

# -smp 2,cores=2,threads=1,sockets=1 \

run: $(KERNEL_BIN) run_only

# gdb connect the qemu
cn:
	@riscv64-unknown-elf-gdb \
    -ex "file $(KERNEL_ELF)" \
    -ex "set arch riscv:rv64" \
    -ex "target remote localhost:$(GDB_PORT)"

clean:
	@cargo clean