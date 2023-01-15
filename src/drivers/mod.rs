pub mod block;
use dtb_walker::{self, Dtb, DtbObj, utils::indent, WalkOperation};


const INDENT_WIDTH: usize = 4;
pub fn print_dtb(dtb_pa: usize) {
    let dtb = unsafe { Dtb::from_raw_parts(dtb_pa as *const u8).unwrap() };
    dtb.walk(|path, obj| match obj {
        DtbObj::SubNode { name } => {
            println!("{}{}/{}", indent(path.level(), INDENT_WIDTH), path, name);
            WalkOperation::StepInto
        }
        DtbObj::Property(prop) => {
            let indent = indent(path.level(), INDENT_WIDTH);
            println!("{}{:?}", indent, prop);
            WalkOperation::StepOver
        }
    });
}