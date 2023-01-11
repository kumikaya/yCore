use std::fs::{self, read_dir, File};
use std::io::{Result, Write};
use std::{env, path::PathBuf};
const TARGET_PATH: &str = "../user/target/riscv64gc-unknown-none-elf/release/";

fn main() {
    let ld = &PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("linker.ld");
    fs::write(ld, LINKER).unwrap();
    insert_app_data().unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../user/src/");
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
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


fn insert_app_data() -> Result<()> {
    let mut file = File::create("src/link_app.S").unwrap();
    let mut apps: Vec<String> = read_dir("../user/src/bin").unwrap()
        .filter_map(|entry| {
            let mut name_ext = entry.unwrap().file_name().into_string().unwrap();
            if let Some(i) = name_ext.find(".") {
                let ext: String = name_ext.drain(i..name_ext.len()).collect();
                if ext.as_str() == ".rs" {
                    Some(name_ext)
                } else {
                    None
                }
            } else {
                None
            }
        }).collect();
    apps.sort();
    writeln!(file, "
.align 3
.section .data
.global _app_num
.global _app_names"
        )?;

    apps.iter().enumerate().for_each(|(i, _name)| {
        writeln!(file, ".global app_{}_start", i).unwrap();
        writeln!(file, ".global app_{}_end", i).unwrap();
    });

    writeln!(file, "
_app_num:
    .quad {}",
        apps.len())?;

    apps.iter().enumerate().for_each(|(i, _name)| {
        writeln!(file, "    .quad app_{}_start", i).unwrap();
    });
    writeln!(file, "    .quad app_{}_end", apps.len() - 1).unwrap();
    
    writeln!(file, "
_app_names:")?;
    apps.iter().for_each(|name| {
        writeln!(file, r#"    .string "{}""#, name).unwrap();
    });
    
    apps.iter().enumerate().for_each(|(i, name)| {
        writeln!(file, "
.align 3
app_{0}_start:
    .incbin \"{1}{2}\"
app_{0}_end:",
        i, TARGET_PATH, name).unwrap();
    });

    Ok(())
}
