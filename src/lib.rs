use serde::Serialize;
use std::str::FromStr;

use sysctl::Ctl;
use sysctl::Sysctl; // Import Ctl directly

// Scalar for representing large integer values (64-bit).
// Maps to standard `i64` in Rust.
type Long = i64;

/// Generic helper to get a sysctl value.
/// Returns a default value if the key is not found, cannot be read, or fails to parse.
fn get_sysctl_value<T>(key: &str) -> T
where
    T: FromStr + Default,
{
    Ctl::new(key)
        .ok()
        .and_then(|ctl| ctl.value_string().ok())
        .and_then(|s| s.parse::<T>().ok())
        .unwrap_or_default()
}

/// Represents the overall CPU information, acting as a container.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "gql", derive(async_graphql::SimpleObject))]
pub struct CPUInfo {
    pub identification: CPUIdentification,
    pub core_counts: CPUCoreCounts,
    pub frequency: CPUFrequency,
    pub performance_levels: Vec<PerformanceLevel>,
}

impl Default for CPUInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl CPUInfo {
    /// Creates a new CPUInfo instance by fetching data from sysctl.
    pub fn new() -> Self {
        // No Result
        CPUInfo {
            identification: CPUIdentification::from_sysctl(),
            core_counts: CPUCoreCounts::from_sysctl(),
            frequency: CPUFrequency::from_sysctl(),
            performance_levels: PerformanceLevel::all_from_sysctl(),
        }
    }
}

/// Detailed identification and branding of the CPU.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "gql", derive(async_graphql::SimpleObject))]
pub struct CPUIdentification {
    /// sysctl: machdep.cpu.brand_string
    pub brand_string: String,
    /// sysctl: machdep.cpu.vendor
    pub vendor: String,
    /// sysctl: machdep.cpu.feature_bits
    pub feature_bits: String,
}

impl CPUIdentification {
    /// Creates a new CPUIdentification instance by fetching data from sysctl.
    pub fn from_sysctl() -> Self {
        CPUIdentification {
            brand_string: get_sysctl_value("machdep.cpu.brand_string"),
            vendor: get_sysctl_value("machdep.cpu.vendor"),
            feature_bits: get_sysctl_value("machdep.cpu.feature_bits"),
        }
    }
}

/// Information about the number of physical and logical cores.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "gql", derive(async_graphql::SimpleObject))]
pub struct CPUCoreCounts {
    /// sysctl: hw.physicalcpu
    pub physical: i32,
    /// sysctl: hw.logicalcpu
    pub logical: i32,
    /// sysctl: hw.physicalcpu_max
    pub max_physical: i32,
    /// sysctl: hw.logicalcpu_max
    pub max_logical: i32,
}

impl CPUCoreCounts {
    /// Creates a new CPUCoreCounts instance by fetching data from sysctl.
    pub fn from_sysctl() -> Self {
        CPUCoreCounts {
            physical: get_sysctl_value("hw.physicalcpu"),
            logical: get_sysctl_value("hw.logicalcpu"),
            max_physical: get_sysctl_value("hw.physicalcpu_max"),
            max_logical: get_sysctl_value("hw.logicalcpu_max"),
        }
    }
}

/// CPU frequency information.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "gql", derive(async_graphql::SimpleObject))]
pub struct CPUFrequency {
    /// sysctl: hw.cpufrequency (Note: May not be accurate or available on modern Macs)
    pub hz: Long,
}

impl CPUFrequency {
    /// Creates a new CPUFrequency instance by fetching data from sysctl.
    pub fn from_sysctl() -> Self {
        CPUFrequency {
            hz: get_sysctl_value("hw.cpufrequency"),
        }
    }
}

/// Represents a performance level of the CPU (e.g., Performance Cores, Efficiency Cores)
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "gql", derive(async_graphql::SimpleObject))]
pub struct PerformanceLevel {
    /// sysctl: Internal identifier, typically derived from perflevel0, perflevel1, etc.
    pub id: i32,
    pub cache: CacheInfo,
    pub cache_sharing: CacheSharing,
}

impl PerformanceLevel {
    /// Attempts to create a PerformanceLevel from a given level ID.
    /// Returns None if the specific perflevelX sysctl keys are not found.
    pub fn from_sysctl_id(id: i32) -> Self {
        // No Option
        PerformanceLevel {
            id,
            cache: CacheInfo::from_sysctl_id(id), // No `?` needed
            cache_sharing: CacheSharing::from_sysctl_id(id), // No `?` needed
        }
    }

    /// Fetches all available PerformanceLevel instances by iterating through perflevelX.
    pub fn all_from_sysctl() -> Vec<Self> {
        (0..)
            .map(|id| {
                let test_key = format!("hw.perflevel{id}.l1icachesize");
                if Ctl::new(&test_key).is_ok() {
                    Some(PerformanceLevel::from_sysctl_id(id))
                } else {
                    None
                }
            })
            .take_while(|level| level.is_some())
            .flatten()
            .collect()
    }
}

/// Detailed cache information for a specific performance level.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "gql", derive(async_graphql::SimpleObject))]
pub struct CacheInfo {
    /// sysctl: perflevelX.l1icachesize
    pub l1_instruction_bytes: Long,
    /// sysctl: perflevelX.l1dcachesize
    pub l1_data_bytes: Long,
    /// sysctl: perflevelX.l2cachesize
    pub l2_bytes: Long,
    /// sysctl: perflevelX.l3cachesize
    pub l3_bytes: Long,
}

impl CacheInfo {
    /// Creates a new CacheInfo instance for a specific performance level ID.
    pub fn from_sysctl_id(id: i32) -> Self {
        // No Option
        let prefix = format!("hw.perflevel{id}");
        CacheInfo {
            l1_instruction_bytes: get_sysctl_value(&format!("{prefix}.l1icachesize")),
            l1_data_bytes: get_sysctl_value(&format!("{prefix}.l1dcachesize")),
            l2_bytes: get_sysctl_value(&format!("{prefix}.l2cachesize")),
            l3_bytes: get_sysctl_value(&format!("{prefix}.l3cachesize")),
        }
    }
}

/// Information about how cores share caches.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "gql", derive(async_graphql::SimpleObject))]
pub struct CacheSharing {
    /// sysctl: perflevelX.cpusperl2
    pub cores_per_l2: i32,
    /// sysctl: perflevelX.cpusperl3
    pub cores_per_l3: i32,
}

impl CacheSharing {
    /// Creates a new CacheSharing instance for a specific performance level ID.
    pub fn from_sysctl_id(id: i32) -> Self {
        // No Option
        let prefix = format!("hw.perflevel{id}");

        CacheSharing {
            cores_per_l2: get_sysctl_value(&format!("{prefix}.cpusperl2")),
            cores_per_l3: get_sysctl_value(&format!("{prefix}.cpusperl3")), // L3 might be 0 if not present
        }
    }
}
