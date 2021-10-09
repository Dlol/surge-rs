#[cfg(target_arch = "x86_64")]
pub use core::arch::x86_64::*;
pub use surge_filter::Waveshaper;
pub use surge_tables::TablesHandle;
pub use surge_samplerate::SampleRateHandle;
pub use surge_math::{
    m128_half,
    m128_one,
    WetBlock1t,
};
