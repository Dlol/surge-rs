#![feature(in_band_lifetimes)]

macro_rules! x  { ($x:ident) => { mod $x; pub use $x::*; } }
macro_rules! ix { 
    () => { 
        #[allow(unused_imports)]
        use crate::{ 
            imports::* , 
        };
    }
}

#[macro_use] mod imports;
#[cfg(test)] mod tests;

x![emphasize];
x![init];
x![param];
x![process];
