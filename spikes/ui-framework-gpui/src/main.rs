use std::{collections::HashSet, ops::Range, time::Duration};

use gpui::{
    App, Application, Bounds, Context, FocusHandle, FontFallbacks, KeyBinding, Timer, Window,
    WindowBounds, WindowOptions, actions, div, font, prelude::*, px, rgb, size, uniform_list,
};
use review_sweeper_gpui_spike::{ROW_COUNT, ReviewUiState, UiIntent, row_label};

actions!(
    review_sweeper_spike,
    [SelectNext, SelectPrevious, Refresh, SwitchFonts]
);

const JAPANESE_CORPUS: &str = "ひらがな・カタカナ・漢字（一般）・𠮷野家（非BMP） / Rustと日本語ABC123\n全角：ＡＢＣ１２３　半角: ABC123 ｶﾀｶﾅ / 濁点: が・か\u{3099}・ハ\u{309a}\nemoji: ⚙️ ☕️ 👩‍💻 🧑🏽‍🚀 / path: C:\\レビュー\\差分\\日本語ファイル.rs";
const DIFF_LEFT: &str = "  9 | fn review(path: &str) {\n-10 |   println!(\"旧: 日本語ＡBC / か\u{3099} / 👩‍💻\");\n 11 |   // 長い行を折返す: C:\\レビュー\\差分\\日本語ファイル.rs / 𠮷野家 / ｶﾀｶﾅ\n 12 | }";
const DIFF_RIGHT: &str = "  9 | fn review(path: &str) {\n+10 |   println!(\"新: 日本語ＡBC / が / 👩‍💻\");\n 11 |   // 長い行を折返す: C:\\レビュー\\差分\\日本語ファイル.rs / 𠮷野家 / ｶﾀｶﾅ\n 12 | }";
const TERMINAL_GRID: &str =
    "terminal cells (VT/grid engine owns width): ASCII|日本語|ＡＢ|ｶﾀｶﾅ|か\u{3099}|⚙️|👩‍💻|";

struct SpikeView {
    state: ReviewUiState,
    focus: FocusHandle,
    last_visible: Range<usize>,
    font_preset: usize,
    installed_fonts: HashSet<String>,
}

impl SpikeView {
    fn new(window: &mut Window, cx: &mut Context<Self>, installed_fonts: HashSet<String>) -> Self {
        let focus = cx.focus_handle();
        window.focus(&focus);
        Self {
            state: ReviewUiState::default(),
            focus,
            last_visible: 0..0,
            font_preset: 0,
            installed_fonts,
        }
    }

    fn apply(&mut self, intent: UiIntent, cx: &mut Context<Self>) {
        self.state.apply(intent);
        println!(
            "EVENT intent={intent:?} selected={} generation={} running={}",
            self.state.selected_row(),
            self.state.refresh_generation(),
            self.state.refresh_running()
        );
        cx.notify();
    }

    fn select_next(&mut self, _: &SelectNext, _: &mut Window, cx: &mut Context<Self>) {
        self.apply(UiIntent::SelectNext, cx);
    }

    fn select_previous(&mut self, _: &SelectPrevious, _: &mut Window, cx: &mut Context<Self>) {
        self.apply(UiIntent::SelectPrevious, cx);
    }

    fn refresh(&mut self, _: &Refresh, _: &mut Window, cx: &mut Context<Self>) {
        self.start_refresh(cx);
    }

    fn switch_fonts(&mut self, _: &SwitchFonts, _: &mut Window, cx: &mut Context<Self>) {
        self.font_preset = (self.font_preset + 1) % 2;
        let (ui, diff, terminal) = font_preset(self.font_preset);
        println!(
            "FONT_SWITCH preset={} ui={:?} ui_valid={} diff={:?} diff_valid={} terminal={:?} terminal_valid={}",
            self.font_preset,
            ui.family,
            self.installed_fonts.contains(ui.family.as_ref()),
            diff.family,
            self.installed_fonts.contains(diff.family.as_ref()),
            terminal.family,
            self.installed_fonts.contains(terminal.family.as_ref())
        );
        cx.notify();
    }

    fn start_refresh(&mut self, cx: &mut Context<Self>) {
        if self.state.refresh_running() {
            return;
        }

        self.apply(UiIntent::StartBackgroundRefresh, cx);
        let next_generation = self.state.refresh_generation() + 1;
        let work = cx.background_executor().spawn(async move {
            Timer::after(Duration::from_millis(500)).await;
            next_generation
        });

        cx.spawn(async move |view, cx| {
            let generation = work.await;
            _ = view.update(cx, |view, cx| {
                view.apply(UiIntent::FinishBackgroundRefresh(generation), cx);
            });
        })
        .detach();
    }
}

fn configured_font(primary: &str, fallbacks: &[&str]) -> gpui::Font {
    let mut value = font(primary.to_owned());
    value.fallbacks = Some(FontFallbacks::from_fonts(
        fallbacks
            .iter()
            .map(|family| (*family).to_owned())
            .collect(),
    ));
    value
}

fn font_preset(index: usize) -> (gpui::Font, gpui::Font, gpui::Font) {
    if index == 0 {
        (
            configured_font("Yu Gothic UI", &["Meiryo UI", "Segoe UI Emoji"]),
            configured_font("Cascadia Mono", &["BIZ UDゴシック", "Segoe UI Emoji"]),
            configured_font("Consolas", &["BIZ UDゴシック", "Segoe UI Emoji"]),
        )
    } else {
        (
            configured_font("Meiryo UI", &["Yu Gothic UI", "Segoe UI Emoji"]),
            configured_font("Consolas", &["BIZ UDゴシック", "Segoe UI Emoji"]),
            configured_font("Cascadia Mono", &["BIZ UDゴシック", "Segoe UI Emoji"]),
        )
    }
}

impl Render for SpikeView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_row = self.state.selected_row();
        let generation = self.state.refresh_generation();
        let refresh_label = if self.state.refresh_running() {
            "更新中…".to_owned()
        } else {
            format!("バックグラウンド更新 (R) / generation={generation}")
        };
        let status = format!("selected={selected_row} / rows={ROW_COUNT}");

        let (explicit_ui_font, explicit_diff_font, explicit_terminal_font) =
            font_preset(self.font_preset);
        let preset_status = format!(
            "font preset={} / UI={} / diff={} / terminal={} / Fでruntime切替",
            self.font_preset,
            explicit_ui_font.family,
            explicit_diff_font.family,
            explicit_terminal_font.family
        );

        div()
            .id("review-sweeper-spike")
            .track_focus(&self.focus)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::refresh))
            .on_action(cx.listener(Self::switch_fonts))
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0x161b22))
            .text_color(rgb(0xe6edf3))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .h(px(52.0))
                    .border_b_1()
                    .border_color(rgb(0x30363d))
                    .child("Review Sweeper / GPUI Windows spike")
                    .child(status)
                    .child(
                        div()
                            .id("refresh")
                            .px_3()
                            .py_1()
                            .rounded_md()
                            .bg(rgb(0x238636))
                            .cursor_pointer()
                            .child(refresh_label)
                            .on_click(cx.listener(|view, _, _, cx| view.start_refresh(cx))),
                    ),
            )
            .child(
                div()
                    .px_4()
                    .py_2()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .border_b_1()
                    .border_color(rgb(0x30363d))
                    .child(preset_status)
                    .child("DirectWrite system collection + family fallback（同一corpus）")
                    .child(div().whitespace_normal().child(JAPANESE_CORPUS))
                    .child("用途別設定: UI proportional / diff monospace / terminal monospace")
                    .child(
                        div()
                            .font(explicit_ui_font)
                            .whitespace_normal()
                            .child(JAPANESE_CORPUS),
                    )
                    .child(
                        div()
                            .max_w(px(700.0))
                            .truncate()
                            .child("省略: 日本語タイトルと非常に長いPR名を表示する必要があるため末尾を省略するテストです"),
                    )
                    .child("side-by-side diff（明示monospace CJK fallback / gutter同期）")
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .font(explicit_diff_font.clone())
                            .child(
                                div()
                                    .w_1_2()
                                    .p_2()
                                    .bg(rgb(0x2d1117))
                                    .whitespace_normal()
                                    .child(DIFF_LEFT),
                            )
                            .child(
                                div()
                                    .w_1_2()
                                    .p_2()
                                    .bg(rgb(0x0f2d1c))
                                    .whitespace_normal()
                                    .child(DIFF_RIGHT),
                            ),
                    )
                    .child(
                        div()
                            .font(explicit_diff_font)
                            .text_color(rgb(0x7ee787))
                            .child("syntax span: + const 日本語: &str = \"ok\";"),
                    )
                    .child(div().font(explicit_terminal_font).child(TERMINAL_GRID)),
            )
            .child(
                uniform_list(
                    "review-rows",
                    ROW_COUNT,
                    cx.processor(move |view, range: Range<usize>, _window, cx| {
                        if view.last_visible != range {
                            println!("EVENT visible={}..{}", range.start, range.end);
                            view.last_visible = range.clone();
                        }
                        range
                            .map(|index| {
                                let selected = index == selected_row;
                                div()
                                    .id(("row", index))
                                    .h(px(26.0))
                                    .px_3()
                                    .flex()
                                    .items_center()
                                    .cursor_pointer()
                                    .when(selected, |row| row.bg(rgb(0x1f6feb)))
                                    .when(!selected && index % 2 == 1, |row| row.bg(rgb(0x0d1117)))
                                    .child(row_label(index, generation))
                                    .on_click(cx.listener(move |view, _, _, cx| {
                                        view.apply(UiIntent::SelectRow(index), cx);
                                    }))
                            })
                            .collect::<Vec<_>>()
                    }),
                )
                .flex_1(),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.bind_keys([
            KeyBinding::new("down", SelectNext, None),
            KeyBinding::new("up", SelectPrevious, None),
            KeyBinding::new("r", Refresh, None),
            KeyBinding::new("f", SwitchFonts, None),
        ]);

        let font_names = cx.text_system().all_font_names();
        let installed_fonts = font_names.iter().cloned().collect::<HashSet<_>>();
        let japanese_fonts = font_names
            .iter()
            .filter(|name| {
                [
                    "Yu", "Meiryo", "BIZ", "Gothic", "Mincho", "Cascadia", "Consolas",
                ]
                .iter()
                .any(|needle| name.contains(needle))
            })
            .cloned()
            .collect::<Vec<_>>();
        println!(
            "FONT_ENUM backend=DirectWrite count={} candidates={japanese_fonts:?}",
            font_names.len()
        );
        for preset in 0..2 {
            let (ui, diff, terminal) = font_preset(preset);
            println!(
                "FONT_VALIDATE preset={preset} ui={:?}:{} diff={:?}:{} terminal={:?}:{}",
                ui.family,
                installed_fonts.contains(ui.family.as_ref()),
                diff.family,
                installed_fonts.contains(diff.family.as_ref()),
                terminal.family,
                installed_fonts.contains(terminal.family.as_ref())
            );
        }

        let bounds = Bounds::centered(None, size(px(960.0), px(720.0)), cx);
        cx.open_window(
            WindowOptions {
                focus: true,
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("Review Sweeper GPUI slice".into()),
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| SpikeView::new(window, cx, installed_fonts)),
        )
        .expect("GPUIウィンドウを作成できませんでした");
        cx.activate(true);
    });
}
