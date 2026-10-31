//! NextGuide Editor v1 — Relm4 / GTK4 two-pane shell.

#[cfg(feature = "ui")]
mod ui {
    use std::path::PathBuf;

    use nextguide_gde::NodePath;
    use nextguide_rtf::{load as load_rtf, Block, Document, LoadBody, Run};
    use nextguide::{window_title_for_path, EditorDoc, TreeRow, APP_ID};
    use relm4::gtk;
    use relm4::gtk::glib;
    use relm4::gtk::prelude::*;
    use relm4::prelude::*;

    const COL_TITLE: i32 = 0;
    const COL_PATH: i32 = 1;

    struct AppModel {
        doc: Option<EditorDoc>,
        title: String,
        status: String,
        tree_store: gtk::TreeStore,
        tree_view: gtk::TreeView,
        body_buffer: gtk::TextBuffer,
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
                set_title: Some(model.title.as_str()),
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
                            set_child = &gtk::TextView {
                                set_buffer: Some(&model.body_buffer),
                                set_editable: false,
                                set_cursor_visible: false,
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
            let column = gtk::TreeViewColumn::new();
            column.set_title("Title");
            column.pack_start(&title_cell, true);
            column.add_attribute(&title_cell, "text", COL_TITLE);
            tree_view.append_column(&column);

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
            tag_table.add(&bold);
            tag_table.add(&italic);
            tag_table.add(&underline);
            let body_buffer = gtk::TextBuffer::builder()
                .tag_table(&tag_table)
                .build();

            let model = AppModel {
                doc: None,
                title: window_title_for_path(None),
                status: "Open a Guide (.gde) to begin.".into(),
                tree_store,
                tree_view,
                body_buffer,
            };

            let widgets = view_output!();
            widgets.tree_scroll.set_child(Some(&model.tree_view));

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
                        self.title = window_title_for_path(Some(&path));
                        widgets.main_window.set_title(Some(&self.title));
                        fill_tree(&self.tree_store, &doc.visible_tree());
                        self.tree_view.expand_all();
                        self.body_buffer.set_text("");
                        self.status = format!("Opened {}", path.display());
                        self.doc = Some(doc);
                    }
                    Err(err) => {
                        self.status = format!("Open failed: {err}");
                    }
                },
                Msg::OpenFailed(msg) => {
                    self.status = format!("Open failed: {msg}");
                }
                Msg::Save => {
                    let Some(doc) = &self.doc else {
                        self.status = "Nothing to save.".into();
                        return;
                    };
                    match doc.save_unchanged() {
                        Ok(()) => sender.input(Msg::Saved),
                        Err(err) => sender.input(Msg::SaveFailed(err.to_string())),
                    }
                }
                Msg::Saved => {
                    self.status = "Saved (unchanged bodies preserved).".into();
                }
                Msg::SaveFailed(msg) => {
                    self.status = format!("Save failed: {msg}");
                }
                Msg::SelectPath(path) => {
                    let Some(doc) = &self.doc else {
                        return;
                    };
                    match doc.body_view(&path) {
                        Ok(view) => {
                            apply_body_to_buffer(&self.body_buffer, doc, &path);
                            self.status = if view.soft_fail {
                                format!(
                                    "Node {path} (unparseable RTF — showing fallback)"
                                )
                            } else {
                                format!("Node {path}")
                            };
                        }
                        Err(err) => {
                            self.body_buffer.set_text("");
                            self.status = format!("Select failed: {err}");
                        }
                    }
                }
                Msg::ClearBody => {
                    self.body_buffer.set_text("");
                }
            }
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

    fn apply_body_to_buffer(buffer: &gtk::TextBuffer, doc: &EditorDoc, path: &NodePath) {
        buffer.set_text("");
        let Ok(node) = doc.guide().node(path) else {
            return;
        };
        match load_rtf(node.rtf_body()) {
            LoadBody::Editable(document) => apply_document(buffer, &document),
            LoadBody::Unparseable { plain_fallback } => {
                buffer.set_text(&plain_fallback);
            }
        }
    }

    fn apply_document(buffer: &gtk::TextBuffer, document: &Document) {
        let mut number = 1u32;
        for (i, block) in document.blocks.iter().enumerate() {
            if i > 0 {
                buffer.insert(&mut buffer.end_iter(), "\n");
            }
            match block {
                Block::Paragraph(runs) => insert_runs(buffer, runs),
                Block::BulletItem(runs) => {
                    buffer.insert(&mut buffer.end_iter(), "• ");
                    insert_runs(buffer, runs);
                }
                Block::NumberedItem(runs) => {
                    buffer.insert(&mut buffer.end_iter(), &format!("{number}. "));
                    insert_runs(buffer, runs);
                    number += 1;
                }
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
