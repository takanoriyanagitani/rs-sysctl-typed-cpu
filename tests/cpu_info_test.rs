use rs_sysctl_typed_cpu::CPUInfo;

#[test]
fn test_cpu_info_creation() {
    // This test ensures that the CPUInfo::new() function can be called without panicking.
    // It's a basic smoke test to ensure that the sysctl calls are not failing in a way
    // that causes a crash.
    let _cpu_info = CPUInfo::new();
}

#[test]
fn test_cpu_info_population() {
    // This test checks that the CPUInfo struct is populated with some data.
    // We don't check for specific values, as they will vary between machines.
    // Instead, we check that numbers are non-zero where it's reasonable to assume they should be.
    let cpu_info = CPUInfo::new();

    // Check CPUCoreCounts
    assert!(
        cpu_info.core_counts.physical > 0,
        "Physical cores should be greater than 0"
    );
    assert!(
        cpu_info.core_counts.logical > 0,
        "Logical cores should be greater than 0"
    );
    assert!(
        cpu_info.core_counts.max_physical > 0,
        "Max physical cores should be greater than 0"
    );
    assert!(
        cpu_info.core_counts.max_logical > 0,
        "Max logical cores should be greater than 0"
    );

    // Check CPUFrequency - this might be 0 on some systems, so we don't assert > 0
    // We just ensure it's there.
    assert!(
        cpu_info.frequency.hz >= 0,
        "Frequency should be a non-negative number"
    );

    // If there are performance levels, check their contents
    for level in &cpu_info.performance_levels {
        assert!(level.id >= 0, "Performance level ID should be non-negative");

        // Check CacheInfo
        assert!(
            level.cache.l1_instruction_bytes > 0,
            "L1i cache size should be greater than 0"
        );
        assert!(
            level.cache.l1_data_bytes > 0,
            "L1d cache size should be greater than 0"
        );
        assert!(
            level.cache.l2_bytes > 0,
            "L2 cache size should be greater than 0"
        );
        // L3 cache can be 0, so we don't check for > 0

        // Check CacheSharing
        assert!(
            level.cache_sharing.cores_per_l2 > 0,
            "Cores per L2 should be greater than 0"
        );
        // Cores per L3 can be 0
    }
}
