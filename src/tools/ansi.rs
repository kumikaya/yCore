#![allow(unused)]

use core::fmt::{self, Debug, Display, Arguments};

use alloc::{format, string::String};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Reset =     0,
    Bold =      1,
    Underline = 4,
    Black =     30,
    Red =       31,
    Green =     32,
    Yellow =    33,
    Blue =      34,
    Magenta =   35,
    Cyan =      36,
    White =     37,
    Default =   39,
    BlackB =    90,
    RedB =      91,
    GreenB =    92,
    YellowB =   93,
    BlueB =     94,
    MagentaB =  95,
    WhiteB =    97,
    CyanB =     96,
}

pub trait Colour {
    type T;
    fn dye(&self, color: Color) -> Self::T;
}

impl<T> Colour for T
where
    T: Display,
{
    type T = String;

    fn dye(&self, color: Color) -> Self::T {
        format!(
            "\x1b[{}m{}\x1b[{}m",
            color as u8,
            self,
            Color::Default as u8
        )
    }
}
