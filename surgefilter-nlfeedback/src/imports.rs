#[cfg(target_arch = "x86_64")]
pub use core::arch::x86_64::*;
pub use std::convert::TryFrom;
pub use surge_samplerate::SampleRateHandle;
pub use surge_filter::{CoeffMake,FilterProcessQuad};
pub use surge_qfunit::QuadFilterUnitState;
pub use enhanced_enum::enhanced_enum;

pub use surge_tuning::{
    TunerHandle,
};

pub use surge_math::{
    fastsin,
    fastcos,
    fasttanh_sse_clamped,
    softclip8_ps,
    softclip_ps,
    limit_range,
};

pub use surge_types::{
    coeffidx,
    FilterSubType
};

pub use surge_constants::{
    CONCERT_A_HZ,
    MIDI_0_FREQ,
    N_COEFFMAKER_COEFFS,
};

pub use core::f64::consts::PI;
pub use core::f32::consts::PI as PI_32;
pub use float_ord::FloatOrd;
