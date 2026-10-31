//! NextGuide Editor v1 — Relm4 / GTK4 two-pane shell.

#[cfg(feature = "ui")]
mod ui {
    use std::cell::Cell;
    use std::path::PathBuf;
    use std::rc::Rc;

    use nextguide_gde::NodePath;
    use nextguide_rtf::{load as load_rtf, Block, Document, LoadBody, Run, SpanStyle};
    use nextguide::{window_title_for_path, EditorDoc, TreeRow, APP_ID};
    use relm4::gtk;
    use relm4::gtk::glib;
    use relm4::gtk::prelude::*;
    use relm4::prelude::*;

    const COL_TITLE: i32 = 0;
    const COL_PATH: i32 = 1;

    struct AppModel {
        doc: Option<EditorDoc>,
        selected: Option<NodePath>,
        status: String,
        tree_store: gtk::TreeStore,
        tree_view: gtk::TreeView,
        body_buffer: gtk::TextBuffer,
        body_view: gtk::TextView,
        /// When true, buffer change signals are ignored (programmatic fills).
        suppress_body_signal: Rc<Cell<bool>>,
        /// True when the body pane text differs from the last committed Node body.
        body_dirty: bool,
    }

    #[derive(Debug)]
    enum Msg {
        OpenDialog,
        Opened(PathBuf),
        OpenFailed(String),
        Save,
        Saved,
        SaveFailed(String),
        SelectPath(NodePath),
        ClearBody,
        TitleEdited { path: NodePath, title: String },
        BodyChanged,
    }

    #[relm4::component]
    impl Component for AppModel {
        type Init = Option<PathBuf>;
        type Input = Msg;
        type Output = ();
        type CommandOutput = ();

        view! {
            #[root]
            main_window = gtk::ApplicationWindow {
                set_title: Some("Untitled"),
                set_default_size: (960, 640),

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    gtk::HeaderBar {
                        pack_start = &gtk::Button {
                            set_label: "Open",
                            connect_clicked => Msg::OpenDialog,
                        },
                        pack_start = &gtk::Button {
                            set_label: "Save",
                            connect_clicked => Msg::Save,
                        },
                    },

                    gtk::Paned {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_wide_handle: true,
                        set_position: 320,
                        set_vexpand: true,
                        set_resize_start_child: true,
                        set_shrink_start_child: false,
                        set_resize_end_child: true,

                        #[wrap(Some)]
                        set_start_child: tree_scroll = &gtk::ScrolledWindow {
                            set_width_request: 180,
                            set_hexpand: true,
                            set_vexpand: true,
                        },

                        #[wrap(Some)]
                        set_end_child = &gtk::ScrolledWindow {
                            set_hexpand: true,
                            set_vexpand: true,

                            #[wrap(Some)]
                            set_child = body_view = &gtk::TextView {
                                set_buffer: Some(&model.body_buffer),
                                set_editable: true,
                                set_cursor_visible: true,
                                set_wrap_mode: gtk::WrapMode::Word,
                                set_left_margin: 8,
                                set_right_margin: 8,
                                set_top_margin: 8,
                                set_bottom_margin: 8,
                            },
                        },
                    },

                    gtk::Label {
                        #[watch]
                        set_label: &model.status,
                        set_xalign: 0.0,
                        set_margin_all: 6,
                    },
                },
            }
        }

        fn init(
            init: Self::Init,
            root: Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
            let tree_store =
                gtk::TreeStore::new(&[glib::Type::STRING, glib::Type::STRING]);
            let tree_view = gtk::TreeView::new();
            tree_view.set_model(Some(&tree_store));
            tree_view.set_headers_visible(false);
            tree_view.set_enable_search(true);
            tree_view.set_activate_on_single_click(true);

            let title_cell = gtk::CellRendererText::new();
            title_cell.set_editable(true);
            let column = gtk::TreeViewColumn::new();
            column.set_title("Title");
            column.pack_start(&title_cell, true);
            column.add_attribute(&title_cell, "text", COL_TITLE);
            tree_view.append_column(&column);

            let store_for_edit = tree_store.clone();
            let edit_sender = sender.clone();
            title_cell.connect_edited(move |_cell, tree_path, new_text| {
                if let Some(iter) = store_for_edit.iter(&tree_path) {
                    let path_str: String = store_for_edit.get(&iter, COL_PATH);
                    edit_sender.input(Msg::TitleEdited {
                        path: path_from_string(&path_str),
                        title: new_text.to_string(),
                    });
                }
            });

            let sel_sender = sender.clone();
            tree_view.selection().connect_changed(move |selection| {
                let Some((model, iter)) = selection.selected() else {
                    sel_sender.input(Msg::ClearBody);
                    return;
                };
                let store = model.downcast::<gtk::TreeStore>().expect("treestore");
                let path_str: String = store.get(&iter, COL_PATH);
                sel_sender.input(Msg::SelectPath(path_from_string(&path_str)));
            });

            let tag_table = gtk::TextTagTable::new();
            let bold = gtk::TextTag::new(Some("bold"));
            bold.set_weight(700);
            let italic = gtk::TextTag::new(Some("italic"));
            italic.set_style(gtk::pango::Style::Italic);
            let underline = gtk::TextTag::new(Some("underline"));
            underline.set_underline(gtk::pango::Underline::Single);
            // Structural markers for list lines (not visible styles). Prefix text is
            // display-only; commit uses these tags so "1. Introduction" paragraphs stay paragraphs.
            let list_bullet = gtk::TextTag::new(Some("list-bullet"));
            let list_numbered = gtk::TextTag::new(Some("list-numbered"));
            tag_table.add(&bold);
            tag_table.add(&italic);
            tag_table.add(&underline);
            tag_table.add(&list_bullet);
            tag_table.add(&list_numbered);
            let body_buffer = gtk::TextBuffer::builder()
                .tag_table(&tag_table)
                .build();

            let suppress_body_signal = Rc::new(Cell::new(false));
            let body_sender = sender.clone();
            let suppress = suppress_body_signal.clone();
            body_buffer.connect_changed(move |_| {
                if suppress.get() {
                    return;
                }
                body_sender.input(Msg::BodyChanged);
            });

            let model = AppModel {
                doc: None,
                selected: None,
                status: "Open a Guide (.gde) to begin.".into(),
                tree_store,
                tree_view,
                body_buffer: body_buffer.clone(),
                body_view: gtk::TextView::new(),
                suppress_body_signal,
                body_dirty: false,
            };

            let widgets = view_output!();
            widgets.tree_scroll.set_child(Some(&model.tree_view));
            let body_view = widgets.body_view.clone();

            // Ctrl/Cmd+B/I/U toggle pragmatic subset styles on the selection.
            let key = gtk::EventControllerKey::new();
            let buf_for_keys = body_buffer;
            let key_sender = sender.clone();
            key.connect_key_pressed(move |_, keyval, _keycode, mods| {
                let shortcut = mods.contains(gtk::gdk::ModifierType::CONTROL_MASK)
                    || mods.contains(gtk::gdk::ModifierType::META_MASK);
                if !shortcut {
                    return glib::Propagation::Proceed;
                }
                let tag = match keyval {
                    gtk::gdk::Key::b | gtk::gdk::Key::B => "bold",
                    gtk::gdk::Key::i | gtk::gdk::Key::I => "italic",
                    gtk::gdk::Key::u | gtk::gdk::Key::U => "underline",
                    _ => return glib::Propagation::Proceed,
                };
                if toggle_style_tag(&buf_for_keys, tag) {
                    key_sender.input(Msg::BodyChanged);
                }
                glib::Propagation::Stop
            });
            body_view.add_controller(key);

            let model = AppModel { body_view, ..model };
            model.refresh_title(&widgets);

            if let Some(path) = init {
                sender.input(Msg::Opened(path));
            }

            let _ = root;
            ComponentParts { model, widgets }
        }

        fn update_with_view(
            &mut self,
            widgets: &mut Self::Widgets,
            message: Self::Input,
            sender: ComponentSender<Self>,
            root: &Self::Root,
        ) {
            match message {
                Msg::OpenDialog => {
                    let dialog = gtk::FileChooserNative::builder()
                        .title("Open Guide")
                        .action(gtk::FileChooserAction::Open)
                        .accept_label("Open")
                        .cancel_label("Cancel")
                        .modal(true)
                        .transient_for(root)
                        .build();
                    let filter = gtk::FileFilter::new();
                    filter.set_name(Some("The Guide (.gde)"));
                    filter.add_pattern("*.gde");
                    dialog.add_filter(&filter);

                    let dialog_sender = sender.clone();
                    dialog.connect_response(move |dialog, response| {
                        if response == gtk::ResponseType::Accept {
                            if let Some(file) = dialog.file() {
                                if let Some(path) = file.path() {
                                    dialog_sender.input(Msg::Opened(path));
                                    return;
                                }
                            }
                            dialog_sender
                                .input(Msg::OpenFailed("No file selected".into()));
                        }
                    });
                    dialog.show();
                }
                Msg::Opened(path) => match EditorDoc::open(&path) {
                    Ok(doc) => {
                        self.selected = None;
                        self.body_dirty = false;
                        self.doc = Some(doc);
                        fill_tree(
                            &self.tree_store,
                            &self.doc.as_ref().unwrap().visible_tree(),
                        );
                        self.tree_view.expand_all();
                        self.set_body_text("");
                        self.status = format!("Opened {}", path.display());
                        self.refresh_title(widgets);
                    }
                    Err(err) => {
                        self.status = format!("Open failed: {err}");
                    }
                },
                Msg::OpenFailed(msg) => {
                    self.status = format!("Open failed: {msg}");
                }
                Msg::Save => {
                    if let Err(err) = self.commit_body_if_needed() {
                        sender.input(Msg::SaveFailed(err));
                        return;
                    }
                    let Some(doc) = self.doc.as_mut() else {
                        self.status = "Nothing to save.".into();
                        return;
                    };
                    match doc.save() {
                        Ok(()) => sender.input(Msg::Saved),
                        Err(err) => sender.input(Msg::SaveFailed(err.to_string())),
                    }
                }
                Msg::Saved => {
                    self.status = "Saved.".into();
                    self.refresh_title(widgets);
                }
                Msg::SaveFailed(msg) => {
                    self.status = format!("Save failed: {msg}");
                }
                Msg::SelectPath(path) => {
                    if self.selected.as_ref() == Some(&path) {
                        return;
                    }
                    if let Err(err) = self.commit_body_if_needed() {
                        self.status = format!("Could not apply body edit: {err}");
                        return;
                    }
                    let Some(doc) = &self.doc else {
                        return;
                    };
                    match doc.body_view(&path) {
                        Ok(view) => {
                            apply_body_to_buffer(
                                &self.body_buffer,
                                &self.suppress_body_signal,
                                doc,
                                &path,
                            );
                            self.body_dirty = false;
                            self.selected = Some(path.clone());
                            self.body_view.set_editable(true);
                            self.status = if view.soft_fail {
                                format!(
                                    "Node {path} (unparseable RTF — edit replaces original bytes)"
                                )
                            } else {
                                format!("Node {path}")
                            };
                        }
                        Err(err) => {
                            self.set_body_text("");
                            self.selected = None;
                            self.status = format!("Select failed: {err}");
                        }
                    }
                    self.refresh_title(widgets);
                }
                Msg::ClearBody => {
                    if let Err(err) = self.commit_body_if_needed() {
                        self.status = format!("Could not apply body edit: {err}");
                        return;
                    }
                    self.selected = None;
                    self.body_dirty = false;
                    self.set_body_text("");
                }
                Msg::TitleEdited { path, title } => {
                    let Some(doc) = self.doc.as_mut() else {
                        return;
                    };
                    match doc.set_title(&path, title) {
                        Ok(()) => {
                            fill_tree(&self.tree_store, &doc.visible_tree());
                            self.tree_view.expand_all();
                            if let Some(sel) = &self.selected {
                                select_path_in_tree(&self.tree_view, &self.tree_store, sel);
                            }
                            self.status = format!("Renamed {path}");
                            self.refresh_title(widgets);
                        }
                        Err(err) => {
                            self.status = format!("Rename failed: {err}");
                        }
                    }
                }
                Msg::BodyChanged => {
                    self.body_dirty = true;
                    self.refresh_title(widgets);
                }
            }
        }
    }

    impl AppModel {
        fn set_body_text(&self, text: &str) {
            self.suppress_body_signal.set(true);
            self.body_buffer.set_text(text);
            self.suppress_body_signal.set(false);
        }

        fn refresh_title(&self, widgets: &AppModelWidgets) {
            let dirty = self
                .doc
                .as_ref()
                .map(|d| d.is_dirty() || self.body_dirty)
                .unwrap_or(false);
            let path = self.doc.as_ref().and_then(|d| d.path());
            let title = window_title_for_path(path, dirty);
            widgets.main_window.set_title(Some(&title));
        }

        fn commit_body_if_needed(&mut self) -> std::result::Result<(), String> {
            if !self.body_dirty {
                return Ok(());
            }
            let Some(path) = self.selected.clone() else {
                self.body_dirty = false;
                return Ok(());
            };
            let Some(doc) = self.doc.as_mut() else {
                self.body_dirty = false;
                return Ok(());
            };
            let document = document_from_buffer(&self.body_buffer);
            doc.set_body(&path, &document)
                .map_err(|e| e.to_string())?;
            self.body_dirty = false;
            Ok(())
        }
    }

    fn fill_tree(store: &gtk::TreeStore, rows: &[TreeRow]) {
        store.clear();
        fn append(store: &gtk::TreeStore, parent: Option<&gtk::TreeIter>, rows: &[TreeRow]) {
            for row in rows {
                let iter = store.append(parent);
                let path = path_to_string(&row.path);
                store.set(
                    &iter,
                    &[(COL_TITLE as u32, &row.title), (COL_PATH as u32, &path)],
                );
                append(store, Some(&iter), &row.children);
            }
        }
        append(store, None, rows);
    }

    fn select_path_in_tree(
        tree_view: &gtk::TreeView,
        store: &gtk::TreeStore,
        target: &NodePath,
    ) {
        let target_s = path_to_string(target);
        if let Some(iter) = find_iter(store, None, &target_s) {
            let path = store.path(&iter);
            tree_view.selection().select_path(&path);
        }
    }

    fn find_iter(
        store: &gtk::TreeStore,
        parent: Option<&gtk::TreeIter>,
        target: &str,
    ) -> Option<gtk::TreeIter> {
        let n = store.iter_n_children(parent);
        for i in 0..n {
            if let Some(iter) = store.iter_nth_child(parent, i) {
                let path_str: String = store.get(&iter, COL_PATH);
                if path_str == target {
                    return Some(iter);
                }
                if let Some(found) = find_iter(store, Some(&iter), target) {
                    return Some(found);
                }
            }
        }
        None
    }

    fn path_to_string(path: &NodePath) -> String {
        path.indices()
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn path_from_string(s: &str) -> NodePath {
        if s.is_empty() {
            return NodePath::root();
        }
        let indices: Vec<usize> = s.split(',').filter_map(|p| p.parse().ok()).collect();
        NodePath::from_indices(indices)
    }

    fn apply_body_to_buffer(
        buffer: &gtk::TextBuffer,
        suppress: &Rc<Cell<bool>>,
        doc: &EditorDoc,
        path: &NodePath,
    ) {
        suppress.set(true);
        buffer.set_text("");
        let Ok(node) = doc.guide().node(path) else {
            suppress.set(false);
            return;
        };
        match load_rtf(node.rtf_body()) {
            LoadBody::Editable(document) => apply_document(buffer, &document),
            LoadBody::Unparseable { plain_fallback } => {
                buffer.set_text(&plain_fallback);
            }
        }
        suppress.set(false);
    }

    fn apply_document(buffer: &gtk::TextBuffer, document: &Document) {
        let mut number = 1u32;
        for (i, block) in document.blocks.iter().enumerate() {
            if i > 0 {
                buffer.insert(&mut buffer.end_iter(), "\n");
            }
            let line_start = buffer.end_iter().offset();
            let list_tag = match block {
                Block::Paragraph(runs) => {
                    insert_runs(buffer, runs);
                    None
                }
                Block::BulletItem(runs) => {
                    buffer.insert(&mut buffer.end_iter(), "• ");
                    insert_runs(buffer, runs);
                    Some("list-bullet")
                }
                Block::NumberedItem(runs) => {
                    buffer.insert(&mut buffer.end_iter(), &format!("{number}. "));
                    insert_runs(buffer, runs);
                    number += 1;
                    Some("list-numbered")
                }
            };
            if let Some(name) = list_tag {
                let start = buffer.iter_at_offset(line_start);
                let end = buffer.end_iter();
                buffer.apply_tag_by_name(name, &start, &end);
            }
        }
    }

    fn insert_runs(buffer: &gtk::TextBuffer, runs: &[Run]) {
        for run in runs {
            let start_off = buffer.end_iter().offset();
            buffer.insert(&mut buffer.end_iter(), &run.text);
            let start = buffer.iter_at_offset(start_off);
            let end = buffer.end_iter();
            if run.style.bold {
                buffer.apply_tag_by_name("bold", &start, &end);
            }
            if run.style.italic {
                buffer.apply_tag_by_name("italic", &start, &end);
            }
            if run.style.underline {
                buffer.apply_tag_by_name("underline", &start, &end);
            }
        }
    }

    fn toggle_style_tag(buffer: &gtk::TextBuffer, name: &str) -> bool {
        let Some((start, end)) = buffer.selection_bounds() else {
            return false;
        };
        if start == end {
            return false;
        }
        let Some(tag) = buffer.tag_table().lookup(name) else {
            return false;
        };
        if start.has_tag(&tag) {
            buffer.remove_tag(&tag, &start, &end);
        } else {
            buffer.apply_tag(&tag, &start, &end);
        }
        true
    }

    /// Convert the body TextBuffer back to a subset Document.
    ///
    /// List structure comes from `list-bullet` / `list-numbered` tags applied when
    /// loading — not from guessing on `"1. "` / `"• "` text prefixes.
    fn document_from_buffer(buffer: &gtk::TextBuffer) -> Document {
        let start = buffer.start_iter();
        let end = buffer.end_iter();
        let text = buffer.text(&start, &end, true);
        if text.is_empty() {
            return Document::empty();
        }

        let bold = buffer.tag_table().lookup("bold");
        let italic = buffer.tag_table().lookup("italic");
        let underline = buffer.tag_table().lookup("underline");
        let list_bullet = buffer.tag_table().lookup("list-bullet");
        let list_numbered = buffer.tag_table().lookup("list-numbered");

        let chars: Vec<char> = text.chars().collect();
        let mut blocks = Vec::new();
        let mut line_start = 0usize;
        loop {
            let line_end = chars[line_start..]
                .iter()
                .position(|&c| c == '\n')
                .map(|p| line_start + p)
                .unwrap_or(chars.len());
            let line: String = chars[line_start..line_end].iter().collect();
            let line_iter = buffer.iter_at_offset(line_start as i32);
            let (kind, content_start) = classify_line(
                &line,
                &line_iter,
                list_bullet.as_ref(),
                list_numbered.as_ref(),
            );
            let content: String = line.chars().skip(content_start).collect();
            let abs_content_start = line_start + content_start;
            let runs = runs_for_range(
                buffer,
                abs_content_start,
                &content,
                bold.as_ref(),
                italic.as_ref(),
                underline.as_ref(),
            );
            match kind {
                LineKind::Paragraph => blocks.push(Block::Paragraph(runs)),
                LineKind::Bullet => blocks.push(Block::BulletItem(runs)),
                LineKind::Numbered => blocks.push(Block::NumberedItem(runs)),
            }
            if line_end >= chars.len() {
                break;
            }
            line_start = line_end + 1;
        }
        Document { blocks }
    }

    enum LineKind {
        Paragraph,
        Bullet,
        Numbered,
    }

    /// Block kind from list tags; strip display prefixes only when tagged as a list.
    fn classify_line(
        line: &str,
        line_iter: &gtk::TextIter,
        list_bullet: Option<&gtk::TextTag>,
        list_numbered: Option<&gtk::TextTag>,
    ) -> (LineKind, usize) {
        if list_bullet.is_some_and(|t| line_iter.has_tag(t)) {
            let start = if line.starts_with("• ") { 2 } else { 0 };
            return (LineKind::Bullet, start);
        }
        if list_numbered.is_some_and(|t| line_iter.has_tag(t)) {
            let chars: Vec<char> = line.chars().collect();
            let mut i = 0;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            let start = if i > 0 && i + 1 < chars.len() && chars[i] == '.' && chars[i + 1] == ' '
            {
                i + 2
            } else {
                0
            };
            return (LineKind::Numbered, start);
        }
        (LineKind::Paragraph, 0)
    }

    fn runs_for_range(
        buffer: &gtk::TextBuffer,
        start_off: usize,
        content: &str,
        bold: Option<&gtk::TextTag>,
        italic: Option<&gtk::TextTag>,
        underline: Option<&gtk::TextTag>,
    ) -> Vec<Run> {
        if content.is_empty() {
            return Vec::new();
        }
        let chars: Vec<char> = content.chars().collect();
        let style_at = |i: usize| -> SpanStyle {
            let iter = buffer.iter_at_offset((start_off + i) as i32);
            SpanStyle {
                bold: bold.map(|t| iter.has_tag(t)).unwrap_or(false),
                italic: italic.map(|t| iter.has_tag(t)).unwrap_or(false),
                underline: underline.map(|t| iter.has_tag(t)).unwrap_or(false),
            }
        };
        let mut runs = Vec::new();
        let mut run_start = 0usize;
        let mut cur_style = style_at(0);
        for i in 1..chars.len() {
            let st = style_at(i);
            if st != cur_style {
                runs.push(Run {
                    text: chars[run_start..i].iter().collect(),
                    style: cur_style,
                });
                run_start = i;
                cur_style = st;
            }
        }
        runs.push(Run {
            text: chars[run_start..].iter().collect(),
            style: cur_style,
        });
        runs
    }

    pub fn run(initial_path: Option<PathBuf>) {
        let app = RelmApp::new(APP_ID);
        app.run::<AppModel>(initial_path);
    }
}

fn main() {
    let initial = std::env::args_os().nth(1).map(std::path::PathBuf::from);

    #[cfg(feature = "ui")]
    ui::run(initial);

    #[cfg(not(feature = "ui"))]
    {
        let _ = initial;
        eprintln!(
            "nextguide was built without the `ui` feature (GTK4 + Relm4).\n\
             Rebuild with: cargo run -p nextguide --features ui"
        );
        std::process::exit(1);
    }
}
