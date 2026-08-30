use rusqlite::{Connection, OptionalExtension, Transaction, backup::Backup, params};
use sha2::{Digest, Sha256};
use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};
use thiserror::Error;

const CURRENT_SCHEMA_VERSION: i64 = 2;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("filesystem error: {0}")]
    Io(#[from] std::io::Error),
    #[error("unsupported schema version {found}; maximum supported version is {supported}")]
    FutureSchema { found: i64, supported: i64 },
    #[error("structured state is corrupt and no valid backup is available")]
    UnrecoverableCorruption,
    #[error("fixture migration failure")]
    FixtureMigrationFailure,
    #[error("generated backup failed integrity validation")]
    InvalidGeneratedBackup,
    #[error("fixture backup failure before publish")]
    FixtureBackupFailure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PersistentState {
    pub account_id: String,
    pub repository_id: i64,
    pub pr_number: i64,
    pub head_sha: String,
    pub draft: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorktreeBinding {
    pub worktree_id: String,
    pub backend_id: String,
    pub native_path: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CacheGcReport {
    pub removed_orphans: usize,
    pub removed_corrupt: usize,
    pub removed_missing_indexes: usize,
}

pub struct Storage {
    root: PathBuf,
    connection: Connection,
}

impl Storage {
    pub fn open(root: impl AsRef<Path>) -> Result<Self, StorageError> {
        Self::open_inner(root.as_ref(), false)
    }

    fn open_inner(root: &Path, fail_migration: bool) -> Result<Self, StorageError> {
        fs::create_dir_all(root.join("blobs"))?;
        let database_path = root.join("state.sqlite3");
        let backup_path = root.join("state.backup.sqlite3");

        let mut connection = match Connection::open(&database_path) {
            Ok(connection) if database_is_valid(&connection) => connection,
            Ok(connection) => {
                drop(connection);
                restore_backup(root, &database_path, &backup_path)?;
                Connection::open(&database_path)?
            }
            Err(error) if backup_path.exists() => {
                let _ = error;
                restore_backup(root, &database_path, &backup_path)?;
                Connection::open(&database_path)?
            }
            Err(error) => return Err(error.into()),
        };

        configure(&connection)?;
        migrate(&mut connection, fail_migration)?;
        Ok(Self {
            root: root.to_path_buf(),
            connection,
        })
    }

    pub fn schema_version(&self) -> Result<i64, StorageError> {
        Ok(self
            .connection
            .pragma_query_value(None, "user_version", |row| row.get(0))?)
    }

    pub fn save_review(&mut self, state: &PersistentState) -> Result<(), StorageError> {
        let transaction = self.connection.transaction()?;
        transaction.execute(
            "INSERT INTO review_sessions
                (account_id, repository_id, pr_number, head_sha, draft)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(account_id, repository_id, pr_number, head_sha)
             DO UPDATE SET draft = excluded.draft",
            params![
                state.account_id,
                state.repository_id,
                state.pr_number,
                state.head_sha,
                state.draft
            ],
        )?;
        transaction.commit()?;
        Ok(())
    }

    pub fn load_review(
        &self,
        account_id: &str,
        repository_id: i64,
        pr_number: i64,
        head_sha: &str,
    ) -> Result<Option<PersistentState>, StorageError> {
        Ok(self
            .connection
            .query_row(
                "SELECT account_id, repository_id, pr_number, head_sha, draft
                 FROM review_sessions
                 WHERE account_id = ?1 AND repository_id = ?2
                   AND pr_number = ?3 AND head_sha = ?4",
                params![account_id, repository_id, pr_number, head_sha],
                |row| {
                    Ok(PersistentState {
                        account_id: row.get(0)?,
                        repository_id: row.get(1)?,
                        pr_number: row.get(2)?,
                        head_sha: row.get(3)?,
                        draft: row.get(4)?,
                    })
                },
            )
            .optional()?)
    }

    pub fn save_worktree_binding(&mut self, binding: &WorktreeBinding) -> Result<(), StorageError> {
        let transaction = self.connection.transaction()?;
        transaction.execute(
            "INSERT INTO worktree_bindings (worktree_id, backend_id, native_path)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(worktree_id) DO UPDATE SET
               backend_id = excluded.backend_id,
               native_path = excluded.native_path",
            params![binding.worktree_id, binding.backend_id, binding.native_path],
        )?;
        transaction.commit()?;
        Ok(())
    }

    pub fn load_worktree_binding(
        &self,
        worktree_id: &str,
    ) -> Result<Option<WorktreeBinding>, StorageError> {
        Ok(self
            .connection
            .query_row(
                "SELECT worktree_id, backend_id, native_path
                 FROM worktree_bindings WHERE worktree_id = ?1",
                [worktree_id],
                |row| {
                    Ok(WorktreeBinding {
                        worktree_id: row.get(0)?,
                        backend_id: row.get(1)?,
                        native_path: row.get(2)?,
                    })
                },
            )
            .optional()?)
    }

    pub fn publish_blob(&mut self, bytes: &[u8]) -> Result<String, StorageError> {
        let digest = content_digest(bytes);
        let final_path = self.blob_path(&digest);
        if !final_path.exists() {
            let temporary_path = self.root.join("blobs").join(format!(".{digest}.tmp"));
            let mut file = OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&temporary_path)?;
            file.write_all(bytes)?;
            file.sync_all()?;
            match fs::rename(&temporary_path, &final_path) {
                Ok(()) => {}
                Err(_error) if final_path.exists() => fs::remove_file(&temporary_path)?,
                Err(error) => return Err(error.into()),
            }
        }
        self.connection.execute(
            "INSERT OR IGNORE INTO cache_objects (digest) VALUES (?1)",
            [&digest],
        )?;
        Ok(digest)
    }

    pub fn link_blob_to_review(
        &mut self,
        state: &PersistentState,
        digest: &str,
    ) -> Result<(), StorageError> {
        let transaction = self.connection.transaction()?;
        transaction.execute(
            "INSERT OR REPLACE INTO cache_references
               (account_id, repository_id, pr_number, head_sha, digest)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                state.account_id,
                state.repository_id,
                state.pr_number,
                state.head_sha,
                digest
            ],
        )?;
        transaction.commit()?;
        Ok(())
    }

    pub fn blob_path(&self, digest: &str) -> PathBuf {
        self.root.join("blobs").join(digest)
    }

    pub fn collect_cache_garbage(&mut self) -> Result<CacheGcReport, StorageError> {
        let referenced = self
            .connection
            .prepare("SELECT DISTINCT digest FROM cache_references")?
            .query_map([], |row| row.get::<_, String>(0))?
            .collect::<Result<HashSet<_>, _>>()?;
        let mut indexed = self
            .connection
            .prepare("SELECT digest FROM cache_objects")?
            .query_map([], |row| row.get::<_, String>(0))?
            .collect::<Result<HashSet<_>, _>>()?;
        let mut report = CacheGcReport::default();

        for entry in fs::read_dir(self.root.join("blobs"))? {
            let entry = entry?;
            if !entry.file_type()?.is_file() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with('.') || !referenced.contains(&name) {
                fs::remove_file(entry.path())?;
                if indexed.remove(&name) {
                    self.connection
                        .execute("DELETE FROM cache_objects WHERE digest = ?1", [&name])?;
                }
                report.removed_orphans += 1;
                continue;
            }
            if !file_matches_digest(&entry.path(), &name)? {
                fs::remove_file(entry.path())?;
                self.connection
                    .execute("DELETE FROM cache_objects WHERE digest = ?1", [&name])?;
                indexed.remove(&name);
                report.removed_corrupt += 1;
            }
        }

        for digest in indexed {
            if !self.blob_path(&digest).exists() {
                self.connection
                    .execute("DELETE FROM cache_objects WHERE digest = ?1", [&digest])?;
                report.removed_missing_indexes += 1;
            }
        }
        Ok(report)
    }

    pub fn create_backup(&self) -> Result<(), StorageError> {
        self.create_backup_inner(false)
    }

    fn create_backup_inner(&self, fail_before_publish: bool) -> Result<(), StorageError> {
        let backup_path = self.root.join("state.backup.sqlite3");
        let temporary = tempfile::Builder::new()
            .prefix(".state.backup.")
            .suffix(".sqlite3.tmp")
            .tempfile_in(&self.root)?;
        let temporary_path = temporary.into_temp_path();
        let mut destination = Connection::open(&temporary_path)?;
        let backup = Backup::new(&self.connection, &mut destination)?;
        backup.run_to_completion(32, std::time::Duration::from_millis(1), None)?;
        drop(backup);
        drop(destination);

        let generated = Connection::open(&temporary_path)?;
        if !database_is_valid(&generated) {
            return Err(StorageError::InvalidGeneratedBackup);
        }
        drop(generated);
        File::open(&temporary_path)?.sync_all()?;

        if fail_before_publish {
            return Err(StorageError::FixtureBackupFailure);
        }

        // TempPath::persistは同一directory内でrenameする。Windowsでは
        // MoveFileExW(MOVEFILE_REPLACE_EXISTING)を使うため、既存backupを先に
        // 削除せず置換でき、失敗時はtemporaryのdrop cleanupに任せられる。
        temporary_path
            .persist(&backup_path)
            .map_err(|error| StorageError::Io(error.error))?;
        sync_parent_directory(&self.root)?;
        Ok(())
    }
}

#[cfg(unix)]
fn sync_parent_directory(directory: &Path) -> Result<(), StorageError> {
    File::open(directory)?.sync_all()?;
    Ok(())
}

#[cfg(not(unix))]
fn sync_parent_directory(_directory: &Path) -> Result<(), StorageError> {
    Ok(())
}

fn configure(connection: &Connection) -> Result<(), StorageError> {
    connection.pragma_update(None, "foreign_keys", "ON")?;
    connection.pragma_update(None, "journal_mode", "WAL")?;
    connection.pragma_update(None, "synchronous", "FULL")?;
    Ok(())
}

fn migrate(connection: &mut Connection, fail_migration: bool) -> Result<(), StorageError> {
    let version: i64 = connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
    if version > CURRENT_SCHEMA_VERSION {
        return Err(StorageError::FutureSchema {
            found: version,
            supported: CURRENT_SCHEMA_VERSION,
        });
    }
    if version == 0 {
        let transaction = connection.transaction()?;
        create_schema_v1(&transaction)?;
        transaction.pragma_update(None, "user_version", 1)?;
        transaction.commit()?;
    }
    let version: i64 = connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
    if version == 1 {
        let transaction = connection.transaction()?;
        transaction.execute_batch(
            "ALTER TABLE review_sessions ADD COLUMN updated_at INTEGER NOT NULL DEFAULT 0;
             CREATE INDEX review_sessions_updated_at ON review_sessions(updated_at);",
        )?;
        if fail_migration {
            return Err(StorageError::FixtureMigrationFailure);
        }
        transaction.pragma_update(None, "user_version", 2)?;
        transaction.commit()?;
    }
    Ok(())
}

fn create_schema_v1(transaction: &Transaction<'_>) -> Result<(), StorageError> {
    transaction.execute_batch(
        "CREATE TABLE review_sessions (
           account_id TEXT NOT NULL,
           repository_id INTEGER NOT NULL,
           pr_number INTEGER NOT NULL,
           head_sha TEXT NOT NULL,
           draft TEXT NOT NULL,
           PRIMARY KEY (account_id, repository_id, pr_number, head_sha)
         );
         CREATE TABLE worktree_bindings (
           worktree_id TEXT PRIMARY KEY,
           backend_id TEXT NOT NULL,
           native_path BLOB NOT NULL
         );
         CREATE TABLE cache_objects (digest TEXT PRIMARY KEY);
         CREATE TABLE cache_references (
           account_id TEXT NOT NULL,
           repository_id INTEGER NOT NULL,
           pr_number INTEGER NOT NULL,
           head_sha TEXT NOT NULL,
           digest TEXT NOT NULL,
           PRIMARY KEY (account_id, repository_id, pr_number, head_sha, digest),
           FOREIGN KEY (account_id, repository_id, pr_number, head_sha)
             REFERENCES review_sessions(account_id, repository_id, pr_number, head_sha)
             ON DELETE CASCADE,
           FOREIGN KEY (digest) REFERENCES cache_objects(digest) ON DELETE CASCADE
         );",
    )?;
    Ok(())
}

fn database_is_valid(connection: &Connection) -> bool {
    connection
        .pragma_query_value(None, "quick_check", |row| row.get::<_, String>(0))
        .is_ok_and(|result| result == "ok")
}

fn restore_backup(root: &Path, database: &Path, backup: &Path) -> Result<(), StorageError> {
    if !backup.exists() {
        return Err(StorageError::UnrecoverableCorruption);
    }
    let backup_connection = Connection::open(backup)?;
    if !database_is_valid(&backup_connection) {
        return Err(StorageError::UnrecoverableCorruption);
    }
    drop(backup_connection);
    if database.exists() {
        let quarantine = root.join("state.corrupt.sqlite3");
        if quarantine.exists() {
            fs::remove_file(&quarantine)?;
        }
        fs::rename(database, quarantine)?;
    }
    fs::copy(backup, database)?;
    Ok(())
}

fn content_digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn file_matches_digest(path: &Path, expected: &str) -> Result<bool, StorageError> {
    let mut bytes = Vec::new();
    File::open(path)?.read_to_end(&mut bytes)?;
    Ok(content_digest(&bytes) == expected)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn review(account: &str, pr_number: i64, head_sha: &str, draft: &str) -> PersistentState {
        PersistentState {
            account_id: account.into(),
            repository_id: 100,
            pr_number,
            head_sha: head_sha.into(),
            draft: draft.into(),
        }
    }

    #[test]
    fn restart_preserves_multiple_prs_and_force_push_revisions() {
        let directory = tempdir().unwrap();
        {
            let mut storage = Storage::open(directory.path()).unwrap();
            storage
                .save_review(&review("a", 1, "old", "旧draft"))
                .unwrap();
            storage.save_review(&review("a", 1, "new", "")).unwrap();
            storage
                .save_review(&review("b", 1, "head", "別account"))
                .unwrap();
            storage
                .save_review(&review("a", 2, "head", "別PR"))
                .unwrap();
        }

        let storage = Storage::open(directory.path()).unwrap();
        assert_eq!(
            storage
                .load_review("a", 100, 1, "old")
                .unwrap()
                .unwrap()
                .draft,
            "旧draft"
        );
        assert_eq!(
            storage
                .load_review("a", 100, 1, "new")
                .unwrap()
                .unwrap()
                .draft,
            ""
        );
        assert_eq!(
            storage
                .load_review("b", 100, 1, "head")
                .unwrap()
                .unwrap()
                .draft,
            "別account"
        );
        assert_eq!(
            storage
                .load_review("a", 100, 2, "head")
                .unwrap()
                .unwrap()
                .draft,
            "別PR"
        );
    }

    #[test]
    fn failed_migration_rolls_back_schema_and_version() {
        let directory = tempdir().unwrap();
        let database_path = directory.path().join("state.sqlite3");
        fs::create_dir_all(directory.path().join("blobs")).unwrap();
        let mut connection = Connection::open(&database_path).unwrap();
        let transaction = connection.transaction().unwrap();
        create_schema_v1(&transaction).unwrap();
        transaction.pragma_update(None, "user_version", 1).unwrap();
        transaction.commit().unwrap();
        drop(connection);

        assert!(matches!(
            Storage::open_inner(directory.path(), true),
            Err(StorageError::FixtureMigrationFailure)
        ));
        let connection = Connection::open(&database_path).unwrap();
        let version: i64 = connection
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .unwrap();
        let has_column: i64 = connection
            .query_row(
                "SELECT count(*) FROM pragma_table_info('review_sessions') WHERE name = 'updated_at'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(version, 1);
        assert_eq!(has_column, 0);
        drop(connection);

        assert_eq!(
            Storage::open(directory.path())
                .unwrap()
                .schema_version()
                .unwrap(),
            2
        );
    }

    #[test]
    fn corrupt_database_is_quarantined_and_restored_from_backup() {
        let directory = tempdir().unwrap();
        {
            let mut storage = Storage::open(directory.path()).unwrap();
            storage
                .save_review(&review("a", 1, "head", "保持するdraft"))
                .unwrap();
            storage.create_backup().unwrap();
        }
        fs::write(directory.path().join("state.sqlite3"), b"not sqlite").unwrap();

        let storage = Storage::open(directory.path()).unwrap();
        assert_eq!(
            storage
                .load_review("a", 100, 1, "head")
                .unwrap()
                .unwrap()
                .draft,
            "保持するdraft"
        );
        assert!(directory.path().join("state.corrupt.sqlite3").exists());
    }

    #[test]
    fn failed_backup_keeps_last_valid_backup_and_cleans_temporary_file() {
        let directory = tempdir().unwrap();
        let backup_path = directory.path().join("state.backup.sqlite3");
        let mut storage = Storage::open(directory.path()).unwrap();
        storage
            .save_review(&review("a", 1, "head", "最後の正常backup"))
            .unwrap();
        storage.create_backup().unwrap();
        let previous_backup = fs::read(&backup_path).unwrap();

        storage
            .save_review(&review("a", 1, "head", "未publishの新状態"))
            .unwrap();
        assert!(matches!(
            storage.create_backup_inner(true),
            Err(StorageError::FixtureBackupFailure)
        ));
        assert_eq!(fs::read(&backup_path).unwrap(), previous_backup);
        assert!(fs::read_dir(directory.path()).unwrap().all(|entry| {
            !entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with(".state.backup.")
        }));

        drop(storage);
        fs::write(directory.path().join("state.sqlite3"), b"not sqlite").unwrap();
        let restored = Storage::open(directory.path()).unwrap();
        assert_eq!(
            restored
                .load_review("a", 100, 1, "head")
                .unwrap()
                .unwrap()
                .draft,
            "最後の正常backup"
        );
    }

    #[test]
    fn backend_binding_survives_restart_without_implicit_fallback() {
        let directory = tempdir().unwrap();
        let binding = WorktreeBinding {
            worktree_id: "wt-1".into(),
            backend_id: "wsl:binding-1:Ubuntu-24.04".into(),
            native_path: b"/home/user/repo".to_vec(),
        };
        {
            let mut storage = Storage::open(directory.path()).unwrap();
            storage.save_worktree_binding(&binding).unwrap();
        }
        let storage = Storage::open(directory.path()).unwrap();
        assert_eq!(
            storage.load_worktree_binding("wt-1").unwrap(),
            Some(binding)
        );
    }

    #[test]
    fn orphan_gc_repairs_files_and_indexes_without_deleting_review_state() {
        let directory = tempdir().unwrap();
        let mut storage = Storage::open(directory.path()).unwrap();
        let state = review("a", 1, "head", "消してはいけないdraft");
        storage.save_review(&state).unwrap();
        let referenced = storage.publish_blob(b"referenced").unwrap();
        storage.link_blob_to_review(&state, &referenced).unwrap();

        let corrupt = storage.publish_blob(b"will be corrupt").unwrap();
        storage.link_blob_to_review(&state, &corrupt).unwrap();
        fs::write(storage.blob_path(&corrupt), b"tampered").unwrap();
        let missing = storage.publish_blob(b"will be missing").unwrap();
        fs::remove_file(storage.blob_path(&missing)).unwrap();
        let orphan = storage.publish_blob(b"orphan").unwrap();

        let report = storage.collect_cache_garbage().unwrap();
        assert_eq!(
            report,
            CacheGcReport {
                removed_orphans: 1,
                removed_corrupt: 1,
                removed_missing_indexes: 1,
            }
        );
        assert!(storage.blob_path(&referenced).exists());
        assert!(!storage.blob_path(&orphan).exists());
        assert_eq!(
            storage
                .load_review("a", 100, 1, "head")
                .unwrap()
                .unwrap()
                .draft,
            "消してはいけないdraft"
        );
    }
}
