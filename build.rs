use std::fs::{read_dir, File};
use std::io::{Result, Write};

static TARGET_PATH: &str = "../user/target/riscv64gc-unknown-none-elf/release/";

fn main() {
    println!("cargo:rerun-if-changed=../user/src/");
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
    insert_app_data().unwrap();
}

fn insert_app_data() -> Result<()> {
    let mut file = File::create("src/link_app.S").unwrap();
    let mut apps: Vec<String> = read_dir("../user/src/bin").unwrap()
        .filter_map(|entry| {
            let mut name_ext = entry.unwrap().file_name().into_string().unwrap();
            // println!("{}", name_ext);
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
.global _num_app"
        )?;

    apps.iter().enumerate().for_each(|(i, _name)| {
        writeln!(file, ".global app_{}_start", i).unwrap();
        writeln!(file, ".global app_{}_end", i).unwrap();
    });

    writeln!(file, "
_num_app:
    .quad {}",
        apps.len())?;

    apps.iter().enumerate().for_each(|(i, _name)| {
        writeln!(file, "    .quad app_{}_start", i).unwrap();
    });
    writeln!(file, "    .quad app_{}_end", apps.len() - 1).unwrap();
    apps.iter().enumerate().for_each(|(i, name)| {
        writeln!(file, "
app_{0}_start:
    .incbin \"{1}{2}.bin\"
app_{0}_end:",
        i, TARGET_PATH, name).unwrap();
    });

    Ok(())
}
