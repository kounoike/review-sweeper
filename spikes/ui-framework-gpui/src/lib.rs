//! UI フレームワークから独立させる最小の状態・入力境界。

use std::ops::Range;

pub const ROW_COUNT: usize = 100_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntent {
    SelectRow(usize),
    SelectNext,
    SelectPrevious,
    StartBackgroundRefresh,
    FinishBackgroundRefresh(u64),
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReviewUiState {
    selected_row: usize,
    refresh_generation: u64,
    refresh_running: bool,
}

impl ReviewUiState {
    pub fn selected_row(&self) -> usize {
        self.selected_row
    }

    pub fn refresh_generation(&self) -> u64 {
        self.refresh_generation
    }

    pub fn refresh_running(&self) -> bool {
        self.refresh_running
    }

    pub fn apply(&mut self, intent: UiIntent) {
        match intent {
            UiIntent::SelectRow(row) => self.selected_row = row.min(ROW_COUNT - 1),
            UiIntent::SelectNext => {
                self.selected_row = (self.selected_row + 1).min(ROW_COUNT - 1);
            }
            UiIntent::SelectPrevious => {
                self.selected_row = self.selected_row.saturating_sub(1);
            }
            UiIntent::StartBackgroundRefresh => self.refresh_running = true,
            UiIntent::FinishBackgroundRefresh(generation) => {
                self.refresh_generation = generation;
                self.refresh_running = false;
            }
        }
    }
}

pub fn row_label(index: usize, generation: u64) -> String {
    format!(
        "{index:06}  sample/file_{:03}.rs  refresh={generation}",
        index % 251
    )
}

/// diff widget が全行を所有せず、可視範囲だけを問い合わせるための snapshot。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiffViewport {
    total_rows: usize,
    visible_rows: Range<usize>,
    selected_rows: Option<Range<usize>>,
    revision: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiffUpdate {
    pub revision: u64,
    pub changed_rows: Range<usize>,
    pub total_rows: usize,
}

impl DiffViewport {
    pub fn new(total_rows: usize) -> Self {
        Self {
            total_rows,
            visible_rows: 0..0,
            selected_rows: None,
            revision: 0,
        }
    }

    pub fn visible_rows(&self) -> Range<usize> {
        self.visible_rows.clone()
    }

    pub fn selected_rows(&self) -> Option<Range<usize>> {
        self.selected_rows.clone()
    }

    pub fn set_visible_rows(&mut self, rows: Range<usize>) {
        self.visible_rows = clamp_range(rows, self.total_rows);
    }

    pub fn select_rows(&mut self, rows: Range<usize>) {
        let rows = clamp_range(rows, self.total_rows);
        self.selected_rows = (!rows.is_empty()).then_some(rows);
    }

    /// 増分更新後に再描画が必要な可視行だけを返す。
    pub fn apply_update(&mut self, update: DiffUpdate) -> Range<usize> {
        self.total_rows = update.total_rows;
        self.revision = update.revision;
        self.visible_rows = clamp_range(self.visible_rows.clone(), self.total_rows);
        self.selected_rows = self
            .selected_rows
            .take()
            .map(|rows| clamp_range(rows, self.total_rows))
            .filter(|rows| !rows.is_empty());

        intersect_ranges(
            self.visible_rows.clone(),
            clamp_range(update.changed_rows, self.total_rows),
        )
    }
}

fn clamp_range(rows: Range<usize>, total_rows: usize) -> Range<usize> {
    let start = rows.start.min(total_rows);
    let end = rows.end.max(start).min(total_rows);
    start..end
}

fn intersect_ranges(left: Range<usize>, right: Range<usize>) -> Range<usize> {
    let start = left.start.max(right.start);
    let end = left.end.min(right.end).max(start);
    start..end
}

/// Windows native と WSL の process/PTY 所有場所を暗黙に混在させない。
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalBackendTarget {
    WindowsConPty,
    Wsl { distribution: String },
}

/// terminal widget から backend へ渡す framework 非依存 command。
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalCommand {
    WriteInput(Vec<u8>),
    Reply(Vec<u8>),
    Resize { columns: u16, rows: u16 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalEvent {
    Output(Vec<u8>),
    Exited { code: Option<i32> },
}

/// VT engine/PTY/RPC の実装を UI adapter から隠す port。
pub trait TerminalTransport {
    type Error;

    fn target(&self) -> &TerminalBackendTarget;
    fn send(&mut self, command: TerminalCommand) -> Result<(), Self::Error>;
    fn try_receive(&mut self) -> Result<Option<TerminalEvent>, Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_is_bounded() {
        let mut state = ReviewUiState::default();
        state.apply(UiIntent::SelectPrevious);
        assert_eq!(state.selected_row(), 0);

        state.apply(UiIntent::SelectRow(ROW_COUNT + 10));
        assert_eq!(state.selected_row(), ROW_COUNT - 1);
        state.apply(UiIntent::SelectNext);
        assert_eq!(state.selected_row(), ROW_COUNT - 1);
    }

    #[test]
    fn background_refresh_has_an_explicit_state_transition() {
        let mut state = ReviewUiState::default();
        state.apply(UiIntent::StartBackgroundRefresh);
        assert!(state.refresh_running());

        state.apply(UiIntent::FinishBackgroundRefresh(42));
        assert!(!state.refresh_running());
        assert_eq!(state.refresh_generation(), 42);
    }

    #[test]
    fn diff_update_invalidates_only_the_visible_intersection() {
        let mut viewport = DiffViewport::new(100_000);
        viewport.set_visible_rows(10_000..10_080);
        viewport.select_rows(10_020..10_025);

        let repaint = viewport.apply_update(DiffUpdate {
            revision: 2,
            changed_rows: 10_070..10_120,
            total_rows: 100_010,
        });

        assert_eq!(repaint, 10_070..10_080);
        assert_eq!(viewport.selected_rows(), Some(10_020..10_025));
    }

    #[test]
    fn terminal_target_keeps_windows_and_wsl_backends_explicit() {
        assert_ne!(
            TerminalBackendTarget::WindowsConPty,
            TerminalBackendTarget::Wsl {
                distribution: "Ubuntu".to_owned(),
            }
        );
    }
}
