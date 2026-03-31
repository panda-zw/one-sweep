use serde::{Deserialize, Serialize};

use super::safety::SafetyLevel;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ScanCategory {
    NodeDependencies,
    NodeCaches,
    DockerImages,
    DockerBuildCache,
    XcodeSimulators,
    XcodeDerivedData,
    GradleCache,
    SystemCaches,
    HomebrewCache,
}

impl ScanCategory {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::NodeDependencies => "JavaScript Project Dependencies",
            Self::NodeCaches => "JavaScript Package Manager Caches",
            Self::DockerImages => "Docker Unused Images",
            Self::DockerBuildCache => "Docker Build Cache",
            Self::XcodeSimulators => "Xcode Simulators & Runtimes",
            Self::XcodeDerivedData => "Xcode Build Data",
            Self::GradleCache => "Android/Gradle Build Cache",
            Self::SystemCaches => "System Caches",
            Self::HomebrewCache => "Homebrew Package Cache",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::NodeDependencies => "Downloaded libraries for JavaScript projects. Automatically re-downloaded when you open the project.",
            Self::NodeCaches => "Package manager download caches. Clearing these may slow your next install slightly.",
            Self::DockerImages => "Container images not used by any running container. Re-pulled when needed.",
            Self::DockerBuildCache => "Cached layers from building containers. Rebuilt automatically on next build.",
            Self::XcodeSimulators => "iOS/watchOS/tvOS simulator devices and runtimes. Re-downloaded from Xcode when needed.",
            Self::XcodeDerivedData => "Build artifacts from Xcode projects. Rebuilt automatically when you open the project.",
            Self::GradleCache => "Cached dependencies for Android/Java projects. Re-downloaded on next build.",
            Self::SystemCaches => "Application caches in your Library folder. Apps regenerate these as needed.",
            Self::HomebrewCache => "Downloaded package files from Homebrew. No longer needed after installation.",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NodeDependencies => "node_dependencies",
            Self::NodeCaches => "node_caches",
            Self::DockerImages => "docker_images",
            Self::DockerBuildCache => "docker_build_cache",
            Self::XcodeSimulators => "xcode_simulators",
            Self::XcodeDerivedData => "xcode_derived_data",
            Self::GradleCache => "gradle_cache",
            Self::SystemCaches => "system_caches",
            Self::HomebrewCache => "homebrew_cache",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "node_dependencies" => Self::NodeDependencies,
            "node_caches" => Self::NodeCaches,
            "docker_images" => Self::DockerImages,
            "docker_build_cache" => Self::DockerBuildCache,
            "xcode_simulators" => Self::XcodeSimulators,
            "xcode_derived_data" => Self::XcodeDerivedData,
            "gradle_cache" => Self::GradleCache,
            "system_caches" => Self::SystemCaches,
            "homebrew_cache" => Self::HomebrewCache,
            _ => Self::SystemCaches,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanItem {
    pub id: String,
    pub path: String,
    pub display_name: String,
    pub description: String,
    pub size_bytes: u64,
    pub safety: SafetyLevel,
    pub category: ScanCategory,
    pub last_modified: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryResult {
    pub category: ScanCategory,
    pub display_name: String,
    pub description: String,
    pub total_bytes: u64,
    pub items: Vec<ScanItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub started_at: i64,
    pub completed_at: Option<i64>,
    pub total_bytes: u64,
    pub categories: Vec<CategoryResult>,
}
