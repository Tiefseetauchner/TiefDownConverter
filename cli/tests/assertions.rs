pub fn read_manifest(path: &std::path::Path) -> tiefdownlib::manifest_model::Manifest {
    let content = std::fs::read_to_string(path).expect("Failed to read manifest file");
    toml::from_str(&content).expect("Failed to parse manifest")
}

#[macro_export]
macro_rules! assert_contains {
    ($haystack:expr, $needle:expr) => {
        assert!(
            $haystack.contains($needle),
            "Expected '{}' to contain '{}'",
            $haystack,
            $needle
        );
    };
}

#[macro_export]
macro_rules! assert_not_contains {
    ($haystack:expr, $needle:expr) => {
        assert!(
            !$haystack.contains($needle),
            "Expected '{}' to not contain '{}'",
            $haystack,
            $needle
        )
    };
}
