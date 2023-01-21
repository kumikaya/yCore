# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
USER_MODE := $(MODE)
BUILD_DIR := $(HOME)/build
TARGET_DIR := $(BUILD_DIR)/$(TARGET)/$(MODE)
# TARGET_DIR := ./target/$(TARGET)/$(MODE)
KERNEL_NAME = ycore
KERNEL_ELF = $(TARGET_DIR)/$(KERNEL_NAME)
KERNEL_BIN = $(KERNEL_ELF).bin
FEATURES = 
# debug
QEMU_ARGS :=
CARGO_ARGS :=
USER := ../user
FS_IMG := $(TARGET_DIR)/fs.img
APPS := $(USER)/src/bin/*
# Log level: error | warn | info
export LOG ?= info
GDB_PATH := riscv64-unknown-elf-gdb
RUST_GDB := RUST_GDB=$(GDB_PATH) rust-gdb

test_build: 
	@cargo build --tests

build:
	@cargo build --features "$(FEATURES)" --$(MODE)

$(APPS):

fs-img: $(APPS)
	@cd ../user && make build
	@rm -f $(FS_IMG)
	@cd ../easy-fs-fuse && cargo run --$(USER_MODE) -- -s $(USER)/src/bin -t $(TARGET_DIR)


$(KERNEL_BIN): build
	@rust-objcopy --strip-all $(KERNEL_ELF) -O binary $@


KERNEL_ENTRY_ADDR := 0x80200000
BOARD ?= qemu
SBI ?= rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin
GDB_PORT := 1234

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

test: test_build run_only

# gdb connect the qemu
cn:
	@$(RUST_GDB) \
    -ex "file $(KERNEL_ELF)" \
    -ex "set arch riscv:rv64" \
    -ex "target remote localhost:$(GDB_PORT)"

clean:
	@cargo clean