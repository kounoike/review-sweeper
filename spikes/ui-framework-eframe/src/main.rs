use std::{
    collections::BTreeSet,
    ops::Range,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

const ROW_COUNT: usize = 100_000;
const JAPANESE_CORPUS: &str = "ひらがな・カタカナ・漢字（一般）・𠮷野家（非BMP） / Rustと日本語ABC123\n全角：ＡＢＣ１２３　半角: ABC123 ｶﾀｶﾅ / 濁点: が・か\u{3099}・ハ\u{309a}\nemoji: ⚙️ ☕️ 👩‍💻 🧑🏽‍🚀 / path: C:\\レビュー\\差分\\日本語ファイル.rs";
const DIFF_LEFT: &str = "  9 | fn review(path: &str) {\n-10 |     println!(\"旧: 日本語ＡBC / か\u{3099} / 👩‍💻\");\n 11 |     // 長い行を折り返してselectionとcopyを確認する: C:\\レビュー\\差分\\日本語ファイル.rs / 𠮷野家 / ｶﾀｶﾅ\n 12 | }";
const DIFF_RIGHT: &str = "  9 | fn review(path: &str) {\n+10 |     println!(\"新: 日本語ＡBC / が / 👩‍💻\");\n 11 |     // 長い行を折り返してselectionとcopyを確認する: C:\\レビュー\\差分\\日本語ファイル.rs / 𠮷野家 / ｶﾀｶﾅ\n 12 | }";
const TERMINAL_GRID: &str = "terminal cells (VT/grid engine owns width):\nASCII|日本語|ＡＢ|ｶﾀｶﾅ|か\u{3099}|⚙️|👩‍💻|\ncol  1 2 3 4 5 6 7 8 9 10  ※ frameworkはglyph描画のみ";

struct SliceApp {
    selected: usize,
    visible: Range<usize>,
    generation: Arc<AtomicU64>,
    refresh_running: Arc<AtomicBool>,
    text: String,
    diff_left: String,
    diff_right: String,
    started: Instant,
    first_paint_logged: bool,
    font_database: fontdb::Database,
    font_preset: usize,
    font_status: String,
}

const UI_FAMILY: &str = "review-ui";
const DIFF_FAMILY: &str = "review-diff";
const TERMINAL_FAMILY: &str = "review-terminal";

fn font_preset(index: usize) -> [&'static [&'static str]; 3] {
    if index == 0 {
        [
            &["Yu Gothic UI", "Meiryo", "Segoe UI Emoji"],
            &["BIZ UDGothic", "MS Gothic", "Segoe UI Emoji"],
            &["MS Gothic", "BIZ UDGothic", "Segoe UI Emoji"],
        ]
    } else {
        [
            &["Meiryo", "Yu Gothic UI", "Segoe UI Emoji"],
            &["MS Gothic", "BIZ UDGothic", "Segoe UI Emoji"],
            &["BIZ UDGothic", "MS Gothic", "Segoe UI Emoji"],
        ]
    }
}

fn find_face<'a>(database: &'a fontdb::Database, family: &str) -> Option<&'a fontdb::FaceInfo> {
    database
        .faces()
        .filter(|face| {
            face.families
                .iter()
                .any(|(candidate, _)| candidate.eq_ignore_ascii_case(family))
                && face.style == fontdb::Style::Normal
        })
        .min_by_key(|face| face.weight.0.abs_diff(fontdb::Weight::NORMAL.0))
}

fn install_role(
    definitions: &mut eframe::egui::FontDefinitions,
    database: &fontdb::Database,
    role_name: &str,
    families: &[&str],
) -> Result<Vec<String>, String> {
    use eframe::egui::{FontData, FontFamily};

    let mut registered = Vec::new();
    let mut details = Vec::new();
    for (position, family) in families.iter().enumerate() {
        let face = find_face(database, family)
            .ok_or_else(|| format!("family {family:?} is not installed"))?;
        let (bytes, face_index) = database
            .with_face_data(face.id, |data, index| (data.to_vec(), index))
            .ok_or_else(|| format!("font data for {family:?} cannot be read"))?;
        let source = match &face.source {
            fontdb::Source::File(path) => path.display().to_string(),
            fontdb::Source::SharedFile(path, _) => path.display().to_string(),
            fontdb::Source::Binary(_) => "<memory>".to_owned(),
        };
        let key = format!("{role_name}-{position}-{family}");
        let mut data = FontData::from_owned(bytes);
        data.index = face_index;
        definitions
            .font_data
            .insert(key.clone(), std::sync::Arc::new(data));
        registered.push(key);
        details.push(format!(
            "{family}@{source}#{face_index}:mono={}",
            face.monospaced
        ));
    }
    definitions
        .families
        .insert(FontFamily::Name(role_name.into()), registered);
    Ok(details)
}

fn install_system_preset(
    ctx: &eframe::egui::Context,
    database: &fontdb::Database,
    preset_index: usize,
) -> Result<String, String> {
    let mut definitions = eframe::egui::FontDefinitions::default();
    let preset = font_preset(preset_index);
    let ui = install_role(&mut definitions, database, UI_FAMILY, preset[0])?;
    let diff = install_role(&mut definitions, database, DIFF_FAMILY, preset[1])?;
    let terminal = install_role(&mut definitions, database, TERMINAL_FAMILY, preset[2])?;
    ctx.set_fonts(definitions);
    let status = format!(
        "preset={preset_index} UI={:?} diff={:?} terminal={:?}",
        preset[0], preset[1], preset[2]
    );
    println!("FONT_SWITCH {status} ui={ui:?} diff={diff:?} terminal={terminal:?}");
    Ok(status)
}

impl SliceApp {
    fn start_refresh(&self, repaint: eframe::egui::Context) {
        if self.refresh_running.swap(true, Ordering::SeqCst) {
            return;
        }

        let generation = Arc::clone(&self.generation);
        let refresh_running = Arc::clone(&self.refresh_running);
        println!("EVENT background=start");
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(500));
            let next = generation.fetch_add(1, Ordering::SeqCst) + 1;
            refresh_running.store(false, Ordering::SeqCst);
            println!("EVENT background=finish generation={next}");
            repaint.request_repaint();
        });
    }
}

impl eframe::App for SliceApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        if !self.first_paint_logged {
            println!("METRIC first_ui_ms={}", self.started.elapsed().as_millis());
            self.first_paint_logged = true;
        }
        if !ui.ctx().egui_wants_keyboard_input()
            && ui.input(|input| input.key_pressed(eframe::egui::Key::ArrowDown))
        {
            self.selected = (self.selected + 1).min(ROW_COUNT - 1);
            println!("EVENT keyboard=down selected={}", self.selected);
        }
        if !ui.ctx().egui_wants_keyboard_input()
            && ui.input(|input| input.key_pressed(eframe::egui::Key::ArrowUp))
        {
            self.selected = self.selected.saturating_sub(1);
            println!("EVENT keyboard=up selected={}", self.selected);
        }

        let generation = self.generation.load(Ordering::SeqCst);
        let running = self.refresh_running.load(Ordering::SeqCst);
        ui.heading("Review Sweeper / eframe 日本語blocking spike");
        ui.label(format!(
            "selected={} / rows={} / visible={}..{} / generation={} / running={}",
            self.selected, ROW_COUNT, self.visible.start, self.visible.end, generation, running
        ));

        ui.horizontal(|ui| {
            if ui.button("Background update").clicked() {
                self.start_refresh(ui.ctx().clone());
            }
            ui.label("IME / Unicode input:");
            let response = ui.text_edit_singleline(&mut self.text);
            if response.changed() {
                println!("EVENT text={:?}", self.text);
            }
            if ui.button("Installed font切替").clicked() {
                let next = (self.font_preset + 1) % 2;
                match install_system_preset(ui.ctx(), &self.font_database, next) {
                    Ok(status) => {
                        self.font_preset = next;
                        self.font_status = status;
                    }
                    Err(error) => {
                        self.font_status = format!("invalid: {error}; last-known-goodを維持");
                        println!("FONT_ERROR {error}");
                    }
                }
            }
        });
        ui.label(&self.font_status);
        ui.separator();

        ui.label("UI用 proportional font（同一corpus）");
        ui.add(
            eframe::egui::Label::new(eframe::egui::RichText::new(JAPANESE_CORPUS).font(
                eframe::egui::FontId::new(16.0, eframe::egui::FontFamily::Name(UI_FAMILY.into())),
            ))
            .selectable(true),
        );
        ui.label("省略: 日本語タイトルと非常に長いPR名を表示する必要があるため末尾を省略…");
        ui.separator();

        ui.label("side-by-side diff（monospace CJK / selectable / copy / wrap / gutter同期）");
        ui.columns(2, |columns| {
            columns[0].add(
                eframe::egui::TextEdit::multiline(&mut self.diff_left)
                    .code_editor()
                    .font(eframe::egui::FontId::new(
                        14.0,
                        eframe::egui::FontFamily::Name(DIFF_FAMILY.into()),
                    ))
                    .desired_rows(4),
            );
            columns[1].add(
                eframe::egui::TextEdit::multiline(&mut self.diff_right)
                    .code_editor()
                    .font(eframe::egui::FontId::new(
                        14.0,
                        eframe::egui::FontFamily::Name(DIFF_FAMILY.into()),
                    ))
                    .desired_rows(4),
            );
        });
        ui.colored_label(
            eframe::egui::Color32::LIGHT_GREEN,
            "syntax span: + const 日本語: &str = \"ok\"; // 選択可能",
        );
        ui.label(
            eframe::egui::RichText::new(TERMINAL_GRID).font(eframe::egui::FontId::new(
                14.0,
                eframe::egui::FontFamily::Name(TERMINAL_FAMILY.into()),
            )),
        );
        ui.separator();

        let previous_visible = self.visible.clone();
        let selected = &mut self.selected;
        let visible = &mut self.visible;
        eframe::egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show_rows(ui, 20.0, ROW_COUNT, |ui, rows| {
                *visible = rows.clone();
                for row in rows {
                    if ui
                        .selectable_label(
                            *selected == row,
                            format!("{row:06}  sample/file_{:03}.rs", row % 251),
                        )
                        .clicked()
                    {
                        *selected = row;
                        println!("EVENT pointer=click selected={row}");
                    }
                }
            });
        if self.visible != previous_visible {
            println!("EVENT visible={}..{}", self.visible.start, self.visible.end);
        }
    }
}

fn main() -> eframe::Result {
    let process_started = Instant::now();
    eframe::run_native(
        "Review Sweeper eframe slice",
        eframe::NativeOptions::default(),
        Box::new(move |creation_context| {
            let font_started = Instant::now();
            let mut font_database = fontdb::Database::new();
            font_database.load_system_fonts();
            let family_names = font_database
                .faces()
                .flat_map(|face| face.families.iter().map(|(family, _)| family.clone()))
                .collect::<BTreeSet<_>>();
            let candidates = family_names
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
                "FONT_ENUM backend=fontdb count={} candidates={candidates:?}",
                family_names.len()
            );
            let font_status = install_system_preset(&creation_context.egui_ctx, &font_database, 0)
                .expect("初期installed font presetを解決できません");
            println!(
                "METRIC font_registration_ms={}",
                font_started.elapsed().as_millis()
            );
            Ok(Box::new(SliceApp {
                selected: 0,
                visible: 0..0,
                generation: Arc::new(AtomicU64::new(0)),
                refresh_running: Arc::new(AtomicBool::new(false)),
                text: String::new(),
                diff_left: DIFF_LEFT.into(),
                diff_right: DIFF_RIGHT.into(),
                started: process_started,
                first_paint_logged: false,
                font_database,
                font_preset: 0,
                font_status,
            }))
        }),
    )
}
