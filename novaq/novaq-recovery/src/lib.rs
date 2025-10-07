//! Failure classification and recovery strategy selection for NOVAQ quantization runs.

/// Failure categories observed during quantization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FailureClass {
    NumericalInstability,
    Divergence,
    InsufficientCoverage,
    HardwareLimit,
}

/// Snapshot of telemetry signals emitted by the quantization core.
#[derive(Debug, Clone, Copy)]
pub struct TelemetrySnapshot {
    pub has_nan: bool,
    pub divergence_ratio: f32,
    pub codebook_utilization: f32,
    pub backend_memory_ok: bool,
}

impl TelemetrySnapshot {
    pub fn classify(&self) -> FailureClass {
        if self.has_nan || !self.backend_memory_ok {
            FailureClass::NumericalInstability
        } else if self.divergence_ratio > 0.25 {
            FailureClass::Divergence
        } else if self.codebook_utilization < 0.35 {
            FailureClass::InsufficientCoverage
        } else {
            FailureClass::HardwareLimit
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classification_prefers_numerical_instability_on_nan() {
        let snapshot = TelemetrySnapshot {
            has_nan: true,
            divergence_ratio: 0.1,
            codebook_utilization: 0.8,
            backend_memory_ok: true,
        };
        assert_eq!(snapshot.classify(), FailureClass::NumericalInstability);
    }
}
