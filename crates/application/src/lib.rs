#![forbid(unsafe_code)]
//! UI と外部 adapter から独立した application 境界。

/// Composition root から UI へ渡す、機能を持たない静的メタデータ。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AppMetadata {
    name: &'static str,
}

impl AppMetadata {
    pub const fn name(self) -> &'static str {
        self.name
    }
}

impl Default for AppMetadata {
    fn default() -> Self {
        Self {
            name: "Review Sweeper",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_metadata_has_the_product_name() {
        assert_eq!(AppMetadata::default().name(), "Review Sweeper");
    }
}
