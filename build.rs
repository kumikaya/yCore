use std::{env, path::PathBuf, fs};

fn main() {
    let ld = &PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("linker.ld");
    fs::write(ld, LINKER).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-arg=-T{}", ld.display());
}


const LINKER: &[u8] = b"
OUTPUT_ARCH(riscv)
ENTRY(_start)
MEMORY {
    /* qemu-system-risc64 virt machine */
    DRAM : ORIGIN = 0x80200000, LENGTH = 8M
}
SECTIONS {
    skernel = .;

    .text : ALIGN(4k) {
        stext = .;
        *(.text.entry)

        . = ALIGN(4K);
        strampoline = .;
        *(.text.trampoline.entry);
        *(.text.trampoline);
        . = ALIGN(4K);

        *(.text .text.*)
        etext = .;
    } > DRAM

    .rodata : ALIGN(4k) {
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        erodata = .;
    } > DRAM
    
    .data : ALIGN(4k) {
        sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        edata = .;
    } > DRAM
    
    .bss (NOLOAD) : ALIGN(4k) {
        sbss = .;
        stack_top = .;
        *(.bss.stack)
        stack_bottom = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    } > DRAM
    ebss = .;

    . = ALIGN(4K);
    ekernel = .;
    /DISCARD/ : {
        *(.eh_frame)
    }
}";
