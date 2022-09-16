#![allow(unused)]

use core::fmt;


#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum SGR {
    Reset = 0,
    Bold = 1,
    Underline = 4,
    DefaultColor = 39,
    Red = 31,
    RedB = 91,
    Blue = 34,
    BlueB = 94,
    Green = 32,
    GreenB = 92,
    White = 37,
    WhiteB = 97,
    Cyan = 36,
    CyanB = 96,
    Yellow = 33,
    YellowB = 93,
}

#[macro_export]
macro_rules! sgr {
    ($sgr: ident) => {
        format_args!("\x1b[{}m", $crate::stdlib::ansi::SGR::$sgr as u8)
    }
}
