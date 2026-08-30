//! TASK-8 の失敗・競合 oracle を検証する、外部サービス非依存の最小 harness。

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestKind {
    Query,
    Mutation,
}

/// 通常 query の bounded retry。mutation は remote state の照合なしに retry しない。
#[must_use]
pub fn retry_delays(kind: RequestKind) -> &'static [u64] {
    match kind {
        RequestKind::Query => &[1, 2, 4],
        RequestKind::Mutation => &[],
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenerationGate {
    active: u64,
    cancelled: bool,
}

impl GenerationGate {
    #[must_use]
    pub fn new(active: u64) -> Self {
        Self {
            active,
            cancelled: false,
        }
    }

    pub fn begin(&mut self, generation: u64) {
        self.active = generation;
        self.cancelled = false;
    }

    pub fn cancel(&mut self) {
        self.cancelled = true;
    }

    #[must_use]
    pub fn accepts(&self, generation: u64) -> bool {
        !self.cancelled && generation == self.active
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendBoundValue {
    pub worktree_id: String,
    pub backend_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FixtureError {
    BackendMismatch { expected: String, actual: String },
    Corrupt,
}

pub fn ensure_same_backend(
    worktree: &BackendBoundValue,
    request: &BackendBoundValue,
) -> Result<(), FixtureError> {
    if worktree.worktree_id == request.worktree_id && worktree.backend_id == request.backend_id {
        Ok(())
    } else {
        Err(FixtureError::BackendMismatch {
            expected: worktree.backend_id.clone(),
            actual: request.backend_id.clone(),
        })
    }
}

#[must_use]
pub fn fixture_digest(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

pub fn verify_blob(expected_digest: u64, bytes: &[u8]) -> Result<(), FixtureError> {
    (expected_digest == fixture_digest(bytes))
        .then_some(())
        .ok_or(FixtureError::Corrupt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stale_async_response_cannot_overwrite_new_generation() {
        let mut gate = GenerationGate::new(10);
        gate.begin(11);
        assert!(!gate.accepts(10));
        assert!(gate.accepts(11));
    }

    #[test]
    fn completion_after_cancel_is_ignored() {
        let mut gate = GenerationGate::new(7);
        gate.cancel();
        assert!(!gate.accepts(7));
    }

    #[test]
    fn later_generation_can_start_after_cancel() {
        let mut gate = GenerationGate::new(7);
        gate.cancel();
        gate.begin(8);
        assert!(gate.accepts(8));
    }

    #[test]
    fn retry_is_bounded_and_mutation_requires_reconciliation() {
        assert_eq!(retry_delays(RequestKind::Query), &[1, 2, 4]);
        assert!(retry_delays(RequestKind::Mutation).is_empty());
    }

    #[test]
    fn corrupt_blob_is_detected() {
        let expected = fixture_digest(b"complete");
        assert_eq!(
            verify_blob(expected, b"partial"),
            Err(FixtureError::Corrupt)
        );
    }

    #[test]
    fn backend_identity_cannot_be_mixed() {
        let worktree = BackendBoundValue {
            worktree_id: "wt-42".into(),
            backend_id: "windows-native:v1".into(),
        };
        let request = BackendBoundValue {
            worktree_id: "wt-42".into(),
            backend_id: "wsl:ubuntu-24.04:v1".into(),
        };
        assert_eq!(
            ensure_same_backend(&worktree, &request),
            Err(FixtureError::BackendMismatch {
                expected: "windows-native:v1".into(),
                actual: "wsl:ubuntu-24.04:v1".into(),
            })
        );
    }
}
