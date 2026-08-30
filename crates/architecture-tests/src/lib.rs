#![forbid(unsafe_code)]
//! Cargo metadata を使う workspace 境界テスト専用 crate。

#[cfg(test)]
mod tests {
    use std::{collections::BTreeSet, process::Command};

    use serde_json::Value;

    const INTERNAL_PREFIX: &str = "review-sweeper";

    #[test]
    fn internal_dependencies_follow_the_approved_direction() {
        let output = Command::new(env!("CARGO"))
            .args(["metadata", "--format-version", "1", "--no-deps"])
            .current_dir(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .parent()
                    .and_then(std::path::Path::parent)
                    .expect("workspace rootが存在する"),
            )
            .output()
            .expect("cargo metadataを実行できる");
        assert!(output.status.success(), "cargo metadataが失敗しました");

        let metadata: Value = serde_json::from_slice(&output.stdout).expect("metadataはJSONである");
        let packages = metadata["packages"].as_array().expect("packagesが存在する");

        for (package, allowed) in [
            ("review-sweeper-domain", &[][..]),
            ("review-sweeper-application", &["review-sweeper-domain"][..]),
            (
                "review-sweeper-integrations",
                &["review-sweeper-application"][..],
            ),
            (
                "review-sweeper-execution",
                &["review-sweeper-application"][..],
            ),
            (
                "review-sweeper-ui-gpui",
                &["review-sweeper-application"][..],
            ),
        ] {
            let package = packages
                .iter()
                .find(|candidate| candidate["name"] == package)
                .expect("検証対象packageが存在する");
            let actual = package["dependencies"]
                .as_array()
                .expect("dependenciesが存在する")
                .iter()
                .filter_map(|dependency| dependency["name"].as_str())
                .filter(|name| name.starts_with(INTERNAL_PREFIX))
                .collect::<BTreeSet<_>>();
            let expected = allowed.iter().copied().collect::<BTreeSet<_>>();
            assert_eq!(actual, expected, "{package:?} の依存方向が変わっています");
        }
    }
}
