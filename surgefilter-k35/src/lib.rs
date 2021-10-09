#![feature(in_band_lifetimes)]

macro_rules! x  { ($x:ident) => { mod $x; pub use $x::*; } }
macro_rules! ix { 
    () => { 
        use crate::{ 
            imports::* , 
        };
    }
}

#[macro_use] mod imports;
#[cfg(test)] mod tests;

x![k35];
x![coeff];
x![process];
