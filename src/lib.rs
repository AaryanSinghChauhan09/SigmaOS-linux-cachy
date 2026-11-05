#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PerformanceProfile {
    LowLatencyGaming,
    ThroughputServer,
    BatterySaving,
}

#[derive(Debug, Clone)]
pub struct CachyKernelTuning {
    pub sched_bore_enabled: bool,
    pub bcachefs_optimized: bool,
    pub transparent_hugepages: bool,
}

impl CachyKernelTuning {
    pub fn for_profile(profile: PerformanceProfile) -> Self {
        match profile {
            PerformanceProfile::LowLatencyGaming => Self {
                sched_bore_enabled: true,
                bcachefs_optimized: true,
                transparent_hugepages: true,
            },
            PerformanceProfile::ThroughputServer => Self {
                sched_bore_enabled: true,
                bcachefs_optimized: true,
                transparent_hugepages: false,
            },
            PerformanceProfile::BatterySaving => Self {
                sched_bore_enabled: false,
                bcachefs_optimized: false,
                transparent_hugepages: false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cachy_tuning() {
        // Arrange & Act
        let tuning = CachyKernelTuning::for_profile(PerformanceProfile::LowLatencyGaming);
        // Assert
        assert!(tuning.sched_bore_enabled);
        assert!(tuning.transparent_hugepages);
    }
}
