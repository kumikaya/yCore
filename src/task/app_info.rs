use alloc::collections::BTreeMap;
use xmas_elf::ElfFile;
use core::slice;
use lazy_static::lazy_static;

use crate::{println, tools::from_cstr};

lazy_static! {
    static ref APP_DATAS: BTreeMap<&'static str, &'static [u8]> = get_app_infos();
}

fn get_app_infos() -> BTreeMap<&'static str, &'static [u8]> {
    unsafe {
        extern "C" {
            fn _app_num();
            fn _app_names();
        }
        let entry_ptr = _app_num as *const usize;
        let num = entry_ptr.read_volatile();
        let start = slice::from_raw_parts(entry_ptr.add(1), num + 1);
        let mut name_start = _app_names as *const u8;
        let mut result = BTreeMap::new();
        for (&app_start, &app_end) in start[..num].iter().zip(start[1..].iter()) {
            let name = from_cstr(name_start);
            let data = slice::from_raw_parts(app_start as *const u8, app_end - app_start);
            name_start = name_start.add(name.len() + 1);
            result.insert(name, data);
        }
        result
    }
}

pub fn get_app_data(name: &str) -> Option<ElfFile> {
    APP_DATAS.get(name).map(|data| ElfFile::new(*data).unwrap())
}

pub fn list_apps() {
    println!("-APPS------------------");
    println!("{:02}  {:14} size", "id", "name");
    for (i, (&name, &data)) in (0..).zip(APP_DATAS.iter()) {
        println!("{:02}: {:14} {}kb", i, name, data.len() / 1024);
    }
    println!("-----------------------");
}
