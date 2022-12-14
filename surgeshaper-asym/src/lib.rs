#![feature(stdarch)]

#[macro_use] mod imports; use imports::*;

x![asym];

#[cfg(not(target_arch = "x86_64"))] 
x![sse1];

#[cfg(target_arch = "x86_64")] 
x![sse2];

#[test] fn smoke() {

    ix!();
    use surge_math::simd_m128;

    let srunit = SampleRateHandle::default();
    let tables = TablesHandle::new(&srunit);

    let shaper = AsymShaper::new(&tables);

    let result = shaper.shape(simd_m128::one(), simd_m128::half());
    println!("result: {:?}", result);

    let result = shaper.shape(result, simd_m128::half());
    println!("result: {:?}", result);
}
