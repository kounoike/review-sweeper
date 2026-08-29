use std::fmt;

mod storage;

pub use storage::{CacheGcReport, PersistentState, Storage, StorageError, WorktreeBinding};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BackendId {
    WindowsNative,
    Wsl {
        binding_id: String,
        distribution: String,
    },
}

impl fmt::Display for BackendId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WindowsNative => formatter.write_str("windows-native"),
            Self::Wsl {
                binding_id,
                distribution,
            } => write!(formatter, "wsl:{binding_id}:{distribution}"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackendPath {
    pub backend: BackendId,
    pub native_path: NativePath,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NativePath {
    WindowsWtf8(Vec<u8>),
    UnixBytes(Vec<u8>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PullRequestKey {
    pub account_id: String,
    pub repository_id: u64,
    pub number: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevisionKey {
    pub pull_request: PullRequestKey,
    pub head_sha: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CacheState {
    Ready,
    Stale,
    Missing,
    Corrupt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendState {
    Available,
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CacheBlob {
    pub expected_digest: u64,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewSession {
    pub revision: RevisionKey,
    pub viewed_files: Vec<String>,
    pub draft_comments: Vec<String>,
}

/// force-push後のheadへ、以前のレビュー進捗を暗黙に移さない。
pub fn start_revision_after_force_push(
    previous: &ReviewSession,
    next_head_sha: impl Into<String>,
) -> (CacheState, ReviewSession) {
    let next = ReviewSession {
        revision: RevisionKey {
            pull_request: previous.revision.pull_request.clone(),
            head_sha: next_head_sha.into(),
        },
        viewed_files: Vec::new(),
        draft_comments: Vec::new(),
    };
    (CacheState::Stale, next)
}

/// backend利用不能時も別backendへfallbackせず、bindingを維持する。
pub fn effective_backend(
    bound: &BackendId,
    availability: BackendState,
) -> Result<BackendId, &'static str> {
    match availability {
        BackendState::Available => Ok(bound.clone()),
        BackendState::Unavailable => Err("bound backend unavailable; explicit rebind required"),
    }
}

/// Fixture用の小さなdeterministic digest。製品では暗号学的hashを使う。
pub fn fixture_digest(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

pub fn verify_cache(blob: &CacheBlob) -> CacheState {
    if fixture_digest(&blob.bytes) == blob.expected_digest {
        CacheState::Ready
    } else {
        CacheState::Corrupt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pr(account: &str, repository_id: u64, number: u64) -> PullRequestKey {
        PullRequestKey {
            account_id: account.into(),
            repository_id,
            number,
        }
    }

    #[test]
    fn multiple_prs_and_accounts_do_not_collide() {
        assert_ne!(pr("account-a", 10, 42), pr("account-a", 10, 43));
        assert_ne!(pr("account-a", 10, 42), pr("account-b", 10, 42));
        assert_ne!(pr("account-a", 10, 42), pr("account-a", 11, 42));
    }

    #[test]
    fn force_push_invalidates_old_cache_without_carrying_review_progress() {
        let old = ReviewSession {
            revision: RevisionKey {
                pull_request: pr("account-a", 10, 42),
                head_sha: "1111111".into(),
            },
            viewed_files: vec!["src/lib.rs".into()],
            draft_comments: vec!["確認が必要".into()],
        };

        let (old_cache, new_session) = start_revision_after_force_push(&old, "2222222");
        assert_eq!(old_cache, CacheState::Stale);
        assert_eq!(new_session.revision.head_sha, "2222222");
        assert!(new_session.viewed_files.is_empty());
        assert!(new_session.draft_comments.is_empty());
        assert_eq!(old.draft_comments, ["確認が必要"]);
    }

    #[test]
    fn windows_and_wsl_paths_are_never_the_same_identity() {
        let windows = BackendPath {
            backend: BackendId::WindowsNative,
            native_path: NativePath::WindowsWtf8(br"C:\src\review-sweeper".to_vec()),
        };
        let wsl = BackendPath {
            backend: BackendId::Wsl {
                binding_id: "binding-1".into(),
                distribution: "Ubuntu-24.04".into(),
            },
            native_path: NativePath::UnixBytes(b"/mnt/c/src/review-sweeper".to_vec()),
        };
        assert_ne!(windows, wsl);
    }

    #[test]
    fn different_wsl_distributions_are_distinct_namespaces() {
        let ubuntu = BackendPath {
            backend: BackendId::Wsl {
                binding_id: "binding-1".into(),
                distribution: "Ubuntu-24.04".into(),
            },
            native_path: NativePath::UnixBytes(b"/home/user/repo".to_vec()),
        };
        let debian = BackendPath {
            backend: BackendId::Wsl {
                binding_id: "binding-2".into(),
                distribution: "Debian".into(),
            },
            native_path: NativePath::UnixBytes(b"/home/user/repo".to_vec()),
        };
        assert_ne!(ubuntu, debian);
    }

    #[test]
    fn unavailable_backend_does_not_fallback() {
        let backend = BackendId::Wsl {
            binding_id: "binding-1".into(),
            distribution: "Ubuntu-24.04".into(),
        };
        assert_eq!(
            effective_backend(&backend, BackendState::Unavailable),
            Err("bound backend unavailable; explicit rebind required")
        );
        assert_eq!(
            effective_backend(&backend, BackendState::Available),
            Ok(backend)
        );
    }

    #[test]
    fn wsl_native_path_does_not_require_utf8() {
        let path = BackendPath {
            backend: BackendId::Wsl {
                binding_id: "binding-1".into(),
                distribution: "Ubuntu-24.04".into(),
            },
            native_path: NativePath::UnixBytes(b"/home/user/\xffrepo".to_vec()),
        };
        assert_eq!(
            path.native_path,
            NativePath::UnixBytes(b"/home/user/\xffrepo".to_vec())
        );
    }

    #[test]
    fn corrupt_cache_is_detected_without_mutating_review_session() {
        let session = ReviewSession {
            revision: RevisionKey {
                pull_request: pr("account-a", 10, 42),
                head_sha: "1111111".into(),
            },
            viewed_files: vec!["src/lib.rs".into()],
            draft_comments: vec!["保持する下書き".into()],
        };
        let original = b"cached diff";
        let corrupt = CacheBlob {
            expected_digest: fixture_digest(original),
            bytes: b"tampered diff".to_vec(),
        };

        assert_eq!(verify_cache(&corrupt), CacheState::Corrupt);
        assert_eq!(session.viewed_files, ["src/lib.rs"]);
        assert_eq!(session.draft_comments, ["保持する下書き"]);
    }
}
