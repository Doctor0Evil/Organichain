// Rust 1.78+, no_std-capable core guard for neuromodulation safety envelopes[web:9][web:12]

#![forbid(unsafe_code)]

#[derive(Clone, Copy, Debug)]
pub struct StimulusEnvelope {
    pub amplitude_mt: f32,      // magnetic or equivalent field
    pub freq_hz: f32,           // stimulation frequency
    pub pulse_width_ms: f32,    // single pulse width
    pub duty_cycle: f32,        // 0.0–1.0
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuardDecision {
    Allow,
    SafeOff,
}

pub struct EnvelopeBounds {
    pub amp_max_mt: f32,
    pub freq_min_hz: f32,
    pub freq_max_hz: f32,
    pub pw_min_ms:  f32,
    pub pw_max_ms:  f32,
    pub duty_max:   f32,
}

impl EnvelopeBounds {
    pub const fn clinical_default() -> Self {
        Self {
            amp_max_mt: 10.0,    // computational neuromod limits for MENPs[web:9][web:12]
            freq_min_hz: 1.0,
            freq_max_hz: 150.0,
            pw_min_ms:  0.5,
            pw_max_ms:  10.0,
            duty_max:   0.25,
        }
    }

    pub fn evaluate(&self, s: StimulusEnvelope) -> GuardDecision {
        if s.amplitude_mt <= 0.0
            || s.amplitude_mt > self.amp_max_mt
            || s.freq_hz < self.freq_min_hz
            || s.freq_hz > self.freq_max_hz
            || s.pulse_width_ms < self.pw_min_ms
            || s.pulse_width_ms > self.pw_max_ms
            || s.duty_cycle < 0.0
            || s.duty_cycle > self.duty_max
        {
            GuardDecision::SafeOff
        } else {
            GuardDecision::Allow
        }
    }
}
