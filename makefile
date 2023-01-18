# Building
TARGET := riscv64gc-unknown-none-elf
USER_MODE := release
MODE := release
KERNEL_NAME = y_core
KERNEL_ELF = ./target/$(TARGET)/$(MODE)/$(KERNEL_NAME)
KERNEL_BIN = $(KERNEL_ELF).bin
FEATURES = 
# debug_test
QEMU_ARGS := 
CARGO_ARGS :=
USER := ../user
FS_IMG := $(USER)/target/$(TARGET)/$(USER_MODE)/fs.img
APPS := $(USER)/src/bin/*
# Log level: error | warn | info
export LOG ?= info

test_build: 
	@cargo build --tests

build:
	@cargo build --features "$(FEATURES)" --$(MODE)

$(APPS):

fs-img: $(APPS)
	@cd ../user && make build
	@rm -f $(FS_IMG)
	@cd ../easy-fs-fuse && cargo run --$(USER_MODE) -- -s $(USER)/src/bin/ -t $(USER)/target/$(TARGET)/release/


$(KERNEL_BIN): build
	@rust-objcopy --strip-all $(KERNEL_ELF) -O binary $@


KERNEL_ENTRY_ADDR := 0x80200000
BOARD ?= qemu
SBI ?= rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin
# GDB_PORT := 9433

run_only:
	@qemu-system-riscv64 \
	  -M 128m \
      -machine virt \
      -nographic \
      -bios $(BOOTLOADER) \
	  -kernel $(KERNEL_ELF) \
	  -drive file=$(FS_IMG),if=none,format=raw,id=x0 \
      -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
	  -smp 2,cores=2,threads=1,sockets=1 \
	  $(QEMU_ARGS)


run: build run_only

# gdb connect the qemu
cn:
	@riscv64-unknown-elf-gdb \
    -ex "file $(KERNEL_ELF)" \
    -ex "set arch riscv:rv64" \
    -ex "target remote localhost:$(GDB_PORT)"

clean:
	@cargo clean