//! NextGuide Editor v1 — Relm4 / GTK4 two-pane shell.

#[cfg(all(feature = "ui", target_os = "macos"))]
mod macos_focus;

#[cfg(feature = "ui")]
mod ui {
    use std::cell::Cell;
    use std::path::PathBuf;
    use std::rc::Rc;

    use nextguide_gde::NodePath;
    use nextguide_rtf::{
        load as load_rtf, Block, Document, FontFamily, FontSize, LoadBody, Run, SpanStyle,
    };
    use nextguide::{
        default_recent_store_path, needs_dirty_prompt, present, window_title_for_path,
        ActionOutcome, DirtyPromptChoice, EditorDoc, EditorSnapshot, FocusedPane, FormatToggle,
        FormattingControls, FormattingSnapshot, ListLineKind, RecentFiles, StructureRefusal,
        TreeRow, APP_ID, UNDO_DEPTH_LIMIT,
    };
    use relm4::gtk;
    use relm4::gtk::glib;
    use relm4::gtk::prelude::*;
    use relm4::prelude::*;

    const COL_TITLE: i32 = 0;
    const COL_PATH: i32 = 1;

    /// Action deferred until a dirty Save/Discard/Cancel prompt resolves.
    #[derive(Clone, Debug)]
    enum PendingAction {
        New,
        OpenDialog,
        OpenPath(PathBuf),
        Quit,
    }

    /// Which undo stack toolbar Undo/Redo and shortcuts target.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum FocusPane {
        Tree,
        Body,
    }

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
        /// Active undo stack follows focus (body GTK buffer vs tree/title app stack).
        focus_pane: FocusPane,
        recent: RecentFiles,
        recent_store: PathBuf,
        pending: Option<PendingAction>,
        recent_menu: gtk::Box,
        file_popover: gtk::Popover,
        recent_popover: gtk::Popover,
        save_button: gtk::Button,
        insert_child_button: gtk::Button,
        insert_sibling_button: gtk::Button,
        delete_button: gtk::Button,
        move_up_button: gtk::Button,
        move_down_button: gtk::Button,
        indent_button: gtk::Button,
        outdent_button: gtk::Button,
        undo_button: gtk::Button,
        redo_button: gtk::Button,
        format_bar: FormatBar,
    }

    #[derive(Clone, Debug)]
    enum Msg {
        New,
        OpenDialog,
        Opened(PathBuf),
        OpenFailed(String),
        Save,
        SaveAsDialog,
        SaveAs(PathBuf),
        Saved,
        SaveFailed(String),
        DirtyChoice(DirtyPromptChoice),
        RequestQuit,
        SelectPath(NodePath),
        ClearBody,
        TitleEdited { path: NodePath, title: String },
        BodyChanged,
        /// Caret or selection moved in the body. Refresh formatting controls.
        BodySelectionChanged,
        FocusPane(FocusPane),
        Undo,
        Redo,
        /// Toggle bold on the body selection.
        ToggleBold,
        /// Toggle italic on the body selection.
        ToggleItalic,
        /// Toggle underline on the body selection.
        ToggleUnderline,
        /// Set the font family on the body selection.
        SetFontFamily(FontFamily),
        /// Set the font size on the body selection.
        SetFontSize(FontSize),
        /// Toggle bullet list on the selected body lines.
        ToggleBulletList,
        /// Toggle numbered list on the selected body lines.
        ToggleNumberedList,
        /// Insert a child under the selection (or Dummy root when none).
        InsertChild,
        /// Insert a sibling after the selection (requires a selection).
        InsertSibling,
        /// Delete the selected Node (never the Dummy root — it is not selectable).
        DeleteNode,
        /// Reorder selected Node one slot earlier among siblings.
        MoveUp,
        /// Reorder selected Node one slot later among siblings.
        MoveDown,
        /// Reparent selected Node under its previous sibling (demote / indent).
        Indent,
        /// Reparent selected Node under its grandparent after its parent (promote).
        Outdent,
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

                    // Toolbar under the operating-system title bar. File starts the
                    // row; Save, Undo, and Redo are icons beside it.
                    // Formatting controls follow. Structure commands follow those.
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 6,
                        set_margin_top: 4,
                        set_margin_bottom: 4,
                        set_margin_start: 6,
                        set_margin_end: 6,

                        gtk::MenuButton {
                            set_label: "File",
                            set_always_show_arrow: false,
                            #[wrap(Some)]
                            #[name = "file_popover"]
                            set_popover = &gtk::Popover {
                                gtk::Box {
                                    set_orientation: gtk::Orientation::Vertical,
                                    set_spacing: 2,
                                    set_margin_all: 6,

                                    gtk::Button {
                                        set_label: "New",
                                        set_has_frame: false,
                                        connect_clicked => Msg::New,
                                    },
                                    gtk::Button {
                                        set_label: "Open…",
                                        set_has_frame: false,
                                        connect_clicked => Msg::OpenDialog,
                                    },
                                    gtk::Button {
                                        set_label: "Save As…",
                                        set_has_frame: false,
                                        connect_clicked => Msg::SaveAsDialog,
                                    },
                                    gtk::Separator {
                                        set_orientation: gtk::Orientation::Horizontal,
                                    },
                                    gtk::MenuButton {
                                        set_label: "Recent",
                                        set_has_frame: false,
                                        set_halign: gtk::Align::Start,
                                        #[wrap(Some)]
                                        #[name = "recent_popover"]
                                        set_popover = &gtk::Popover {
                                            #[name = "recent_menu"]
                                            gtk::Box {
                                                set_orientation: gtk::Orientation::Vertical,
                                                set_spacing: 2,
                                                set_margin_all: 6,
                                            },
                                        },
                                    },
                                    gtk::Separator {
                                        set_orientation: gtk::Orientation::Horizontal,
                                    },
                                    gtk::Button {
                                        set_label: "Quit",
                                        set_has_frame: false,
                                        connect_clicked => Msg::RequestQuit,
                                    },
                                },
                            },
                        },
                        gtk::Box {
                            set_tooltip_text: Some("Save (Ctrl+S)"),
                            #[name = "save_button"]
                            gtk::Button {
                                set_icon_name: "document-save-symbolic",
                                set_tooltip_text: Some("Save (Ctrl+S)"),
                                set_focus_on_click: false,
                                connect_clicked => Msg::Save,
                            },
                        },
                        // Parent tooltips stay visible when an icon is grey.
                        gtk::Box {
                            set_tooltip_text: Some("Undo (Ctrl+Z)"),
                            #[name = "undo_button"]
                            gtk::Button {
                                set_icon_name: "edit-undo-symbolic",
                                set_tooltip_text: Some("Undo (Ctrl+Z)"),
                                set_focus_on_click: false,
                                connect_clicked => Msg::Undo,
                            },
                        },
                        gtk::Box {
                            set_tooltip_text: Some("Redo (Ctrl+Shift+Z, Ctrl+Y)"),
                            #[name = "redo_button"]
                            gtk::Button {
                                set_icon_name: "edit-redo-symbolic",
                                set_tooltip_text: Some("Redo (Ctrl+Shift+Z, Ctrl+Y)"),
                                set_focus_on_click: false,
                                connect_clicked => Msg::Redo,
                            },
                        },
                        #[name = "formatting_box"]
                        gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 0,
                            set_margin_start: 12,
                        },
                        gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 6,
                            set_margin_start: 12,

                            gtk::Box {
                                set_tooltip_text: Some("Insert Child"),
                                #[name = "insert_child_button"]
                                gtk::Button {
                                    set_tooltip_text: Some("Insert Child"),
                                    set_focus_on_click: false,
                                    connect_clicked => Msg::InsertChild,
                                    // GTK4 keeps only the last of icon-name and label, so both
                                    // live in the child: the standard add symbol plus the words.
                                    #[wrap(Some)]
                                    set_child = &gtk::Box {
                                        set_spacing: 6,
                                        gtk::Image {
                                            set_icon_name: Some("list-add-symbolic"),
                                        },
                                        gtk::Label {
                                            set_label: "Insert Child",
                                        },
                                    },
                                },
                            },
                            gtk::Box {
                                set_tooltip_text: Some("Insert Sibling"),
                                #[name = "insert_sibling_button"]
                                gtk::Button {
                                    set_tooltip_text: Some("Insert Sibling"),
                                    set_focus_on_click: false,
                                    connect_clicked => Msg::InsertSibling,
                                    #[wrap(Some)]
                                    set_child = &gtk::Box {
                                        set_spacing: 6,
                                        gtk::Image {
                                            set_icon_name: Some("list-add-symbolic"),
                                        },
                                        gtk::Label {
                                            set_label: "Insert Sibling",
                                        },
                                    },
                                },
                            },
                            gtk::Box {
                                set_tooltip_text: Some("Delete"),
                                #[name = "delete_button"]
                                gtk::Button {
                                    set_icon_name: "edit-delete-symbolic",
                                    set_tooltip_text: Some("Delete"),
                                    set_focus_on_click: false,
                                    connect_clicked => Msg::DeleteNode,
                                },
                            },
                        },
                        gtk::Box {
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 6,
                            set_margin_start: 12,

                            gtk::Box {
                                set_tooltip_text: Some("Move Up"),
                                #[name = "move_up_button"]
                                gtk::Button {
                                    set_icon_name: "go-up-symbolic",
                                    set_tooltip_text: Some("Move Up"),
                                    set_focus_on_click: false,
                                    connect_clicked => Msg::MoveUp,
                                },
                            },
                            gtk::Box {
                                set_tooltip_text: Some("Move Down"),
                                #[name = "move_down_button"]
                                gtk::Button {
                                    set_icon_name: "go-down-symbolic",
                                    set_tooltip_text: Some("Move Down"),
                                    set_focus_on_click: false,
                                    connect_clicked => Msg::MoveDown,
                                },
                            },
                            gtk::Box {
                                set_tooltip_text: Some("Indent"),
                                #[name = "indent_button"]
                                gtk::Button {
                                    set_icon_name: "format-indent-more-symbolic",
                                    set_tooltip_text: Some("Indent"),
                                    set_focus_on_click: false,
                                    connect_clicked => Msg::Indent,
                                },
                            },
                            gtk::Box {
                                set_tooltip_text: Some("Outdent"),
                                #[name = "outdent_button"]
                                gtk::Button {
                                    set_icon_name: "format-indent-less-symbolic",
                                    set_tooltip_text: Some("Outdent"),
                                    set_focus_on_click: false,
                                    connect_clicked => Msg::Outdent,
                                },
                            },
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
                                add_css_class: "ng-body",
                                // Empty / no-selection sessions keep the body disabled.
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
            for family in FontFamily::ALL {
                let tag = gtk::TextTag::new(Some(family_tag_name(family)));
                tag.set_family(Some(family_pango_name(family)));
                tag_table.add(&tag);
            }
            for size in FontSize::ALL {
                let tag = gtk::TextTag::new(Some(&size_tag_name(size)));
                tag.set_size_points(f64::from(size.pt()));
                tag_table.add(&tag);
            }
            tag_table.add(&list_bullet);
            tag_table.add(&list_numbered);
            let body_buffer = gtk::TextBuffer::builder()
                .tag_table(&tag_table)
                .enable_undo(true)
                .build();
            body_buffer.set_max_undo_levels(UNDO_DEPTH_LIMIT as u32);

            let suppress_body_signal = Rc::new(Cell::new(false));
            let body_sender = sender.clone();
            let suppress = suppress_body_signal.clone();
            body_buffer.connect_changed(move |_| {
                if suppress.get() {
                    return;
                }
                body_sender.input(Msg::BodyChanged);
            });
            let mark_sender = sender.clone();
            let mark_suppress = suppress_body_signal.clone();
            body_buffer.connect_mark_set(move |_, _, mark| {
                if mark_suppress.get() {
                    return;
                }
                let Some(name) = mark.name() else {
                    return;
                };
                if name == "insert" || name == "selection_bound" {
                    mark_sender.input(Msg::BodySelectionChanged);
                }
            });

            let format_bar = FormatBar::new(&sender);
            let recent_store = default_recent_store_path();
            let recent = RecentFiles::load_from(&recent_store).unwrap_or_default();

            let tree_focus = gtk::EventControllerFocus::new();
            let focus_sender = sender.clone();
            tree_focus.connect_enter(move |_| {
                focus_sender.input(Msg::FocusPane(FocusPane::Tree));
            });
            tree_view.add_controller(tree_focus);
            tree_view.set_can_focus(true);

            let model = AppModel {
                doc: Some(EditorDoc::new()),
                selected: None,
                status: String::new(),
                tree_store,
                tree_view,
                body_buffer: body_buffer.clone(),
                body_view: gtk::TextView::new(),
                suppress_body_signal,
                body_dirty: false,
                focus_pane: FocusPane::Tree,
                recent,
                recent_store,
                pending: None,
                recent_menu: gtk::Box::new(gtk::Orientation::Vertical, 2),
                file_popover: gtk::Popover::new(),
                recent_popover: gtk::Popover::new(),
                save_button: gtk::Button::new(),
                insert_child_button: gtk::Button::new(),
                insert_sibling_button: gtk::Button::new(),
                delete_button: gtk::Button::new(),
                move_up_button: gtk::Button::new(),
                move_down_button: gtk::Button::new(),
                indent_button: gtk::Button::new(),
                outdent_button: gtk::Button::new(),
                undo_button: gtk::Button::new(),
                redo_button: gtk::Button::new(),
                format_bar,
            };

            let widgets = view_output!();
            widgets.tree_scroll.set_child(Some(&model.tree_view));
            // Untagged body text reads as the default face and size (Arial 9 pt).
            relm4::set_global_css(&format!(
                ".ng-body {{ font-family: {}; font-size: {}pt; }}",
                family_pango_name(FontFamily::default()),
                FontSize::DEFAULT.pt()
            ));
            model.format_bar.attach(&widgets.formatting_box);
            let body_view = widgets.body_view.clone();
            let recent_menu = widgets.recent_menu.clone();
            let file_popover = widgets.file_popover.clone();
            let recent_popover = widgets.recent_popover.clone();
            let save_button = widgets.save_button.clone();
            let insert_child_button = widgets.insert_child_button.clone();
            let insert_sibling_button = widgets.insert_sibling_button.clone();
            let delete_button = widgets.delete_button.clone();
            let move_up_button = widgets.move_up_button.clone();
            let move_down_button = widgets.move_down_button.clone();
            let indent_button = widgets.indent_button.clone();
            let outdent_button = widgets.outdent_button.clone();
            let undo_button = widgets.undo_button.clone();
            let redo_button = widgets.redo_button.clone();

            // Body-local format shortcuts: B/I/U and list toggles.
            let key = gtk::EventControllerKey::new();
            let key_sender = sender.clone();
            key.connect_key_pressed(move |_, keyval, _keycode, mods| {
                let shortcut = mods.contains(gtk::gdk::ModifierType::CONTROL_MASK)
                    || mods.contains(gtk::gdk::ModifierType::META_MASK);
                if !shortcut {
                    return glib::Propagation::Proceed;
                }
                let shift = mods.contains(gtk::gdk::ModifierType::SHIFT_MASK);
                match keyval {
                    gtk::gdk::Key::b | gtk::gdk::Key::B if !shift => {
                        key_sender.input(Msg::ToggleBold);
                        glib::Propagation::Stop
                    }
                    gtk::gdk::Key::i | gtk::gdk::Key::I if !shift => {
                        key_sender.input(Msg::ToggleItalic);
                        glib::Propagation::Stop
                    }
                    gtk::gdk::Key::u | gtk::gdk::Key::U if !shift => {
                        key_sender.input(Msg::ToggleUnderline);
                        glib::Propagation::Stop
                    }
                    // Ctrl/Cmd+Shift+8 → bullet; Ctrl/Cmd+Shift+7 → numbered (common desktop habit).
                    gtk::gdk::Key::_8 | gtk::gdk::Key::KP_8 if shift => {
                        key_sender.input(Msg::ToggleBulletList);
                        glib::Propagation::Stop
                    }
                    gtk::gdk::Key::_7 | gtk::gdk::Key::KP_7 if shift => {
                        key_sender.input(Msg::ToggleNumberedList);
                        glib::Propagation::Stop
                    }
                    _ => glib::Propagation::Proceed,
                }
            });
            body_view.add_controller(key);

            let body_focus = gtk::EventControllerFocus::new();
            let body_focus_sender = sender.clone();
            body_focus.connect_enter(move |_| {
                body_focus_sender.input(Msg::FocusPane(FocusPane::Body));
            });
            body_view.add_controller(body_focus);

            // Window-level File + Undo/Redo shortcuts (stack follows focus_pane).
            let app_keys = gtk::EventControllerKey::new();
            let app_sender = sender.clone();
            app_keys.connect_key_pressed(move |_, keyval, _keycode, mods| {
                let primary = mods.contains(gtk::gdk::ModifierType::CONTROL_MASK)
                    || mods.contains(gtk::gdk::ModifierType::META_MASK);
                if !primary {
                    return glib::Propagation::Proceed;
                }
                let shift = mods.contains(gtk::gdk::ModifierType::SHIFT_MASK);
                match keyval {
                    gtk::gdk::Key::n | gtk::gdk::Key::N if !shift => {
                        app_sender.input(Msg::New);
                        glib::Propagation::Stop
                    }
                    gtk::gdk::Key::o | gtk::gdk::Key::O if !shift => {
                        app_sender.input(Msg::OpenDialog);
                        glib::Propagation::Stop
                    }
                    gtk::gdk::Key::s | gtk::gdk::Key::S if shift => {
                        app_sender.input(Msg::SaveAsDialog);
                        glib::Propagation::Stop
                    }
                    gtk::gdk::Key::s | gtk::gdk::Key::S => {
                        app_sender.input(Msg::Save);
                        glib::Propagation::Stop
                    }
                    gtk::gdk::Key::z | gtk::gdk::Key::Z if shift => {
                        app_sender.input(Msg::Redo);
                        glib::Propagation::Stop
                    }
                    gtk::gdk::Key::z | gtk::gdk::Key::Z => {
                        app_sender.input(Msg::Undo);
                        glib::Propagation::Stop
                    }
                    gtk::gdk::Key::y | gtk::gdk::Key::Y if !shift => {
                        app_sender.input(Msg::Redo);
                        glib::Propagation::Stop
                    }
                    _ => glib::Propagation::Proceed,
                }
            });
            root.add_controller(app_keys);

            let quit_sender = sender.clone();
            root.connect_close_request(move |_| {
                quit_sender.input(Msg::RequestQuit);
                glib::Propagation::Stop
            });

            let model = AppModel {
                body_view,
                recent_menu,
                file_popover,
                recent_popover,
                save_button,
                insert_child_button,
                insert_sibling_button,
                delete_button,
                move_up_button,
                move_down_button,
                indent_button,
                outdent_button,
                undo_button,
                redo_button,
                ..model
            };
            model.rebuild_recent_menu(&sender);
            model.show_unselected_body();
            model.refresh_title(&widgets);

            if let Some(path) = init {
                sender.input(Msg::Opened(path));
            }

            // Terminal launches leave the process backgrounded on macOS (and
            // occasionally elsewhere). Present + platform activate once the
            // event loop is running and the window can become key.
            let launch_window = root.clone();
            glib::idle_add_local_once(move || {
                bring_main_window_forward(&launch_window);
            });

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
                Msg::New => {
                    self.dismiss_file_menus();
                    self.request_with_dirty_prompt(
                        widgets,
                        &sender,
                        root,
                        PendingAction::New,
                    );
                }
                Msg::OpenDialog => {
                    self.dismiss_file_menus();
                    self.request_with_dirty_prompt(
                        widgets,
                        &sender,
                        root,
                        PendingAction::OpenDialog,
                    );
                }
                Msg::Opened(path) => {
                    self.dismiss_file_menus();
                    // CLI / Recent may open while dirty — prompt first.
                    if self.pending.is_none()
                        && needs_dirty_prompt(self.session_dirty())
                    {
                        self.request_with_dirty_prompt(
                            widgets,
                            &sender,
                            root,
                            PendingAction::OpenPath(path),
                        );
                        return;
                    }
                    self.load_opened(widgets, &sender, path);
                }
                Msg::OpenFailed(msg) => {
                    self.status = format!("Open failed: {msg}");
                }
                Msg::Save => {
                    self.dismiss_file_menus();
                    if let Err(err) = self.commit_body_if_needed() {
                        sender.input(Msg::SaveFailed(err));
                        return;
                    }
                    let needs_as = self
                        .doc
                        .as_ref()
                        .map(|d| d.path().is_none())
                        .unwrap_or(true);
                    if needs_as {
                        self.show_save_as_dialog(root, &sender);
                        return;
                    }
                    let Some(doc) = self.doc.as_mut() else {
                        self.publish_outcome(ActionOutcome::NothingToSave);
                        return;
                    };
                    match doc.save() {
                        Ok(()) => sender.input(Msg::Saved),
                        Err(err) => sender.input(Msg::SaveFailed(err.to_string())),
                    }
                }
                Msg::SaveAsDialog => {
                    self.dismiss_file_menus();
                    if let Err(err) = self.commit_body_if_needed() {
                        sender.input(Msg::SaveFailed(err));
                        return;
                    }
                    self.show_save_as_dialog(root, &sender);
                }
                Msg::SaveAs(path) => {
                    if let Err(err) = self.commit_body_if_needed() {
                        sender.input(Msg::SaveFailed(err));
                        return;
                    }
                    let Some(doc) = self.doc.as_mut() else {
                        self.publish_outcome(ActionOutcome::NothingToSave);
                        return;
                    };
                    match doc.save_as(&path) {
                        Ok(()) => {
                            self.remember_recent(&path, &sender);
                            sender.input(Msg::Saved);
                            if let Some(pending) = self.pending.take() {
                                self.run_pending(widgets, &sender, root, pending);
                            }
                        }
                        Err(err) => sender.input(Msg::SaveFailed(err.to_string())),
                    }
                }
                Msg::Saved => {
                    self.publish_outcome(ActionOutcome::Success);
                    self.refresh_title(widgets);
                }
                Msg::SaveFailed(msg) => {
                    self.publish_outcome(ActionOutcome::SaveFailed { detail: msg });
                }
                Msg::DirtyChoice(choice) => match choice {
                    DirtyPromptChoice::Cancel => {
                        self.pending = None;
                        self.status = "Cancelled.".into();
                    }
                    DirtyPromptChoice::Discard => {
                        if let Some(pending) = self.pending.take() {
                            // Revert unsaved edits so a follow-up Open chooser
                            // does not re-prompt for the abandoned dirty state.
                            self.discard_edits(widgets);
                            self.run_pending(widgets, &sender, root, pending);
                        }
                    }
                    DirtyPromptChoice::Save => {
                        if let Err(err) = self.commit_body_if_needed() {
                            self.pending = None;
                            sender.input(Msg::SaveFailed(err));
                            return;
                        }
                        let needs_as = self
                            .doc
                            .as_ref()
                            .map(|d| d.path().is_none())
                            .unwrap_or(true);
                        if needs_as {
                            self.show_save_as_dialog(root, &sender);
                            return;
                        }
                        let Some(doc) = self.doc.as_mut() else {
                            self.pending = None;
                            self.publish_outcome(ActionOutcome::NothingToSave);
                            return;
                        };
                        match doc.save() {
                            Ok(()) => {
                                self.refresh_title(widgets);
                                if let Some(pending) = self.pending.take() {
                                    self.run_pending(widgets, &sender, root, pending);
                                }
                            }
                            Err(err) => {
                                self.pending = None;
                                sender.input(Msg::SaveFailed(err.to_string()));
                            }
                        }
                    }
                },
                Msg::RequestQuit => {
                    self.dismiss_file_menus();
                    self.request_with_dirty_prompt(
                        widgets,
                        &sender,
                        root,
                        PendingAction::Quit,
                    );
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
                            self.set_body_editable(true);
                            self.status = if view.soft_fail {
                                format!(
                                    "Node {path} (unparseable RTF — edit replaces original bytes)"
                                )
                            } else {
                                format!("Node {path}")
                            };
                        }
                        Err(err) => {
                            self.selected = None;
                            self.show_unselected_body();
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
                    self.show_unselected_body();
                    self.refresh_title(widgets);
                }
                Msg::TitleEdited { path, title } => {
                    self.focus_pane = FocusPane::Tree;
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
                Msg::BodySelectionChanged => {
                    self.refresh_formatting();
                }
                Msg::FocusPane(pane) => {
                    self.focus_pane = pane;
                    // Undo and Redo follow the pane that was focused before the click.
                    self.refresh_title(widgets);
                }
                Msg::Undo => {
                    self.run_undo(widgets);
                }
                Msg::Redo => {
                    self.run_redo(widgets);
                }
                Msg::ToggleBold => {
                    self.toggle_body_style(widgets, "bold");
                }
                Msg::ToggleItalic => {
                    self.toggle_body_style(widgets, "italic");
                }
                Msg::ToggleUnderline => {
                    self.toggle_body_style(widgets, "underline");
                }
                Msg::SetFontFamily(family) => {
                    let names: Vec<String> = FontFamily::ALL
                        .iter()
                        .map(|f| family_tag_name(*f).to_string())
                        .collect();
                    self.set_body_font(widgets, &names, family_tag_name(family), family.label());
                }
                Msg::SetFontSize(size) => {
                    let names: Vec<String> =
                        FontSize::ALL.iter().copied().map(size_tag_name).collect();
                    self.set_body_font(
                        widgets,
                        &names,
                        &size_tag_name(size),
                        &format!("{} pt", size.pt()),
                    );
                }
                Msg::ToggleBulletList => {
                    self.toggle_body_list(widgets, LineKind::Bullet);
                }
                Msg::ToggleNumberedList => {
                    self.toggle_body_list(widgets, LineKind::Numbered);
                }
                Msg::InsertChild => {
                    if let Err(err) = self.commit_body_if_needed() {
                        self.status = format!("Could not apply body edit: {err}");
                        return;
                    }
                    // Nothing selected: the child hangs under the hidden Dummy root.
                    let parent = self.selected.clone().unwrap_or_else(NodePath::root);
                    let Some(doc) = self.doc.as_mut() else {
                        self.publish_outcome(ActionOutcome::StructureRefused(
                            StructureRefusal::NoGuide,
                        ));
                        return;
                    };
                    let index = match doc.guide().node(&parent) {
                        Ok(node) => node.children().len(),
                        Err(err) => {
                            self.publish_outcome(ActionOutcome::InsertChildFailed {
                                detail: err.to_string(),
                            });
                            return;
                        }
                    };
                    match doc.insert_child(&parent, index) {
                        Ok(path) => {
                            self.after_tree_op(widgets, Some(path));
                        }
                        Err(err) => {
                            self.publish_outcome(ActionOutcome::InsertChildFailed {
                                detail: err.to_string(),
                            });
                        }
                    }
                }
                Msg::InsertSibling => {
                    if let Err(err) = self.commit_body_if_needed() {
                        self.status = format!("Could not apply body edit: {err}");
                        return;
                    }
                    let Some(selected) = self.selected.clone() else {
                        self.publish_outcome(ActionOutcome::StructureRefused(
                            StructureRefusal::SelectNodeToInsertSibling,
                        ));
                        return;
                    };
                    if selected.is_root() {
                        self.publish_outcome(ActionOutcome::StructureRefused(
                            StructureRefusal::CannotInsertSiblingOfDummyRoot,
                        ));
                        return;
                    }
                    let Some(parent) = selected.parent() else {
                        self.publish_outcome(ActionOutcome::StructureRefused(
                            StructureRefusal::InsertSiblingNoParent,
                        ));
                        return;
                    };
                    let Some(index) = selected.last_index() else {
                        self.publish_outcome(ActionOutcome::StructureRefused(
                            StructureRefusal::InsertSiblingNoIndex,
                        ));
                        return;
                    };
                    let Some(doc) = self.doc.as_mut() else {
                        self.publish_outcome(ActionOutcome::StructureRefused(
                            StructureRefusal::NoGuide,
                        ));
                        return;
                    };
                    match doc.insert_child(&parent, index + 1) {
                        Ok(path) => {
                            self.after_tree_op(widgets, Some(path));
                        }
                        Err(err) => {
                            self.publish_outcome(ActionOutcome::InsertSiblingFailed {
                                detail: err.to_string(),
                            });
                        }
                    }
                }
                Msg::DeleteNode => {
                    if let Err(err) = self.commit_body_if_needed() {
                        self.status = format!("Could not apply body edit: {err}");
                        return;
                    }
                    let Some(selected) = self.selected.clone() else {
                        self.publish_outcome(ActionOutcome::StructureRefused(
                            StructureRefusal::SelectNodeToDelete,
                        ));
                        return;
                    };
                    if selected.is_root() {
                        self.publish_outcome(ActionOutcome::StructureRefused(
                            StructureRefusal::CannotDeleteDummyRoot,
                        ));
                        return;
                    }
                    let Some(doc) = self.doc.as_mut() else {
                        self.publish_outcome(ActionOutcome::StructureRefused(
                            StructureRefusal::NoGuide,
                        ));
                        return;
                    };
                    match doc.remove_node(&selected) {
                        Ok(()) => {
                            self.selected = None;
                            self.body_dirty = false;
                            self.show_unselected_body();
                            self.after_tree_op(widgets, None);
                        }
                        Err(err) => {
                            self.publish_outcome(ActionOutcome::DeleteFailed {
                                detail: err.to_string(),
                            });
                        }
                    }
                }
                Msg::MoveUp => {
                    self.reorder_selected(widgets, -1);
                }
                Msg::MoveDown => {
                    self.reorder_selected(widgets, 1);
                }
                Msg::Indent => {
                    if let Err(err) = self.commit_body_if_needed() {
                        self.status = format!("Could not apply body edit: {err}");
                        return;
                    }
                    let Some(selected) = self.selected.clone() else {
                        self.publish_outcome(ActionOutcome::StructureRefused(
                            StructureRefusal::SelectNodeToIndent,
                        ));
                        return;
                    };
                    let Some(parent) = selected.parent() else {
                        self.publish_outcome(ActionOutcome::StructureRefused(
                            StructureRefusal::CannotIndentDummyRoot,
                        ));
                        return;
                    };
                    let Some(index) = selected.last_index() else {
                        self.publish_outcome(ActionOutcome::StructureRefused(
                            StructureRefusal::IndentNoIndex,
                        ));
                        return;
                    };
                    if index == 0 {
                        self.publish_outcome(ActionOutcome::StructureRefused(
                            StructureRefusal::IndentNeedsPreviousSibling,
                        ));
                        return;
                    }
                    let new_parent = parent.child(index - 1);
                    let Some(doc) = self.doc.as_mut() else {
                        self.publish_outcome(ActionOutcome::StructureRefused(
                            StructureRefusal::NoGuide,
                        ));
                        return;
                    };
                    let dest_index = match doc.guide().node(&new_parent) {
                        Ok(node) => node.children().len(),
                        Err(err) => {
                            self.publish_outcome(ActionOutcome::IndentFailed {
                                detail: err.to_string(),
                            });
                            return;
                        }
                    };
                    match doc.move_node(&selected, &new_parent, dest_index) {
                        Ok(path) => {
                            self.after_tree_op(widgets, Some(path));
                        }
                        Err(err) => {
                            self.publish_outcome(ActionOutcome::IndentFailed {
                                detail: err.to_string(),
                            });
                        }
                    }
                }
                Msg::Outdent => {
                    if let Err(err) = self.commit_body_if_needed() {
                        self.status = format!("Could not apply body edit: {err}");
                        return;
                    }
                    let Some(selected) = self.selected.clone() else {
                        self.publish_outcome(ActionOutcome::StructureRefused(
                            StructureRefusal::SelectNodeToOutdent,
                        ));
                        return;
                    };
                    let Some(parent) = selected.parent() else {
                        self.publish_outcome(ActionOutcome::StructureRefused(
                            StructureRefusal::CannotOutdentDummyRoot,
                        ));
                        return;
                    };
                    // Top-level under Dummy root cannot outdent further.
                    let Some(grandparent) = parent.parent() else {
                        self.publish_outcome(ActionOutcome::StructureRefused(
                            StructureRefusal::AlreadyAtTopLevel,
                        ));
                        return;
                    };
                    let Some(parent_index) = parent.last_index() else {
                        self.publish_outcome(ActionOutcome::StructureRefused(
                            StructureRefusal::OutdentNoParentIndex,
                        ));
                        return;
                    };
                    let Some(doc) = self.doc.as_mut() else {
                        self.publish_outcome(ActionOutcome::StructureRefused(
                            StructureRefusal::NoGuide,
                        ));
                        return;
                    };
                    match doc.move_node(&selected, &grandparent, parent_index + 1) {
                        Ok(path) => {
                            self.after_tree_op(widgets, Some(path));
                        }
                        Err(err) => {
                            self.publish_outcome(ActionOutcome::OutdentFailed {
                                detail: err.to_string(),
                            });
                        }
                    }
                }
            }
        }
    }

    impl AppModel {
        fn set_body_text(&self, text: &str) {
            self.suppress_body_signal.set(true);
            // Irreversible fill seals/clears the body undo stack (selection change).
            self.body_buffer.begin_irreversible_action();
            self.body_buffer.set_text(text);
            self.body_buffer.end_irreversible_action();
            self.suppress_body_signal.set(false);
        }

        fn run_undo(&mut self, widgets: &AppModelWidgets) {
            match self.focus_pane {
                FocusPane::Body => {
                    if self.body_buffer.can_undo() {
                        self.body_buffer.undo();
                        self.publish_outcome(ActionOutcome::Success);
                        self.refresh_title(widgets);
                    } else {
                        self.status = "Nothing to undo in the body".into();
                    }
                }
                FocusPane::Tree => {
                    // Discard uncommitted body buffer edits so refresh cannot
                    // re-apply them onto the restored Guide checkpoint.
                    self.body_dirty = false;
                    let Some(doc) = self.doc.as_mut() else {
                        self.status = "Nothing to undo".into();
                        return;
                    };
                    if !doc.undo_tree() {
                        self.status = "Nothing to undo in the tree".into();
                        return;
                    }
                    self.after_tree_history(widgets);
                }
            }
        }

        fn run_redo(&mut self, widgets: &AppModelWidgets) {
            match self.focus_pane {
                FocusPane::Body => {
                    if self.body_buffer.can_redo() {
                        self.body_buffer.redo();
                        self.publish_outcome(ActionOutcome::Success);
                        self.refresh_title(widgets);
                    } else {
                        self.status = "Nothing to redo in the body".into();
                    }
                }
                FocusPane::Tree => {
                    self.body_dirty = false;
                    let Some(doc) = self.doc.as_mut() else {
                        self.status = "Nothing to redo".into();
                        return;
                    };
                    if !doc.redo_tree() {
                        self.status = "Nothing to redo in the tree".into();
                        return;
                    }
                    self.after_tree_history(widgets);
                }
            }
        }

        /// Refresh UI after an app-level tree/title undo or redo.
        ///
        /// A successful undo or redo publishes no bottom-line sentence.
        fn after_tree_history(&mut self, widgets: &AppModelWidgets) {
            let selected = self.selected.clone();
            let Some(doc) = self.doc.as_ref() else {
                return;
            };
            fill_tree(&self.tree_store, &doc.visible_tree());
            self.tree_view.expand_all();
            if let Some(path) = selected {
                if doc.guide().node(&path).is_ok() {
                    // Avoid selection-changed commit: body_dirty already cleared.
                    self.selected = None;
                    select_path_in_tree(&self.tree_view, &self.tree_store, &path);
                } else {
                    self.selected = None;
                    self.body_dirty = false;
                    self.show_unselected_body();
                    self.tree_view.selection().unselect_all();
                }
            }
            self.publish_outcome(ActionOutcome::Success);
            self.refresh_title(widgets);
        }

        fn set_body_editable(&self, editable: bool) {
            self.body_view.set_editable(editable);
            self.body_view.set_cursor_visible(editable);
        }

        fn toggle_body_style(&mut self, widgets: &AppModelWidgets, tag: &str) {
            if !self.body_view.is_editable() {
                self.status = "Select a Node to format its body.".into();
                return;
            }
            if toggle_style_tag(&self.body_buffer, tag) {
                self.body_dirty = true;
                self.publish_outcome(ActionOutcome::Formatted);
                self.refresh_title(widgets);
            }
        }

        /// Apply one tag from a mutually exclusive group (family or size) to the selection.
        fn set_body_font(
            &mut self,
            widgets: &AppModelWidgets,
            group: &[String],
            tag: &str,
            label: &str,
        ) {
            if !self.body_view.is_editable() {
                self.status = "Select a Node to format its body.".into();
                return;
            }
            if apply_exclusive_tag(&self.body_buffer, group, tag) {
                self.body_dirty = true;
                self.publish_outcome(ActionOutcome::Formatted);
                self.refresh_title(widgets);
            } else {
                // Selection-only (no typing-style stack): tell the user why nothing changed.
                self.status = format!("Select text to set {label}.");
                self.refresh_formatting();
            }
        }

        fn toggle_body_list(&mut self, widgets: &AppModelWidgets, kind: LineKind) {
            if !self.body_view.is_editable() {
                self.status = "Select a Node to format its body.".into();
                return;
            }
            if toggle_list_kind(&self.body_buffer, kind) {
                self.body_dirty = true;
                self.publish_outcome(ActionOutcome::Formatted);
                self.refresh_title(widgets);
            }
        }

        fn session_dirty(&self) -> bool {
            self.doc
                .as_ref()
                .map(|d| d.is_dirty() || self.body_dirty)
                .unwrap_or(self.body_dirty)
        }

        fn editor_snapshot(&self) -> EditorSnapshot {
            EditorSnapshot {
                guide_open: self.doc.is_some(),
                dirty: self.session_dirty(),
                has_nodes: self
                    .doc
                    .as_ref()
                    .map(|doc| !doc.visible_tree().is_empty())
                    .unwrap_or(false),
                node_selected: self.selected.is_some(),
                focused_pane: match self.focus_pane {
                    FocusPane::Tree => FocusedPane::Tree,
                    FocusPane::Body => FocusedPane::Body,
                },
                body_undo_empty: !self.body_buffer.can_undo(),
                tree_undo_empty: self
                    .doc
                    .as_ref()
                    .map(|doc| !doc.can_undo_tree())
                    .unwrap_or(true),
                body_redo_empty: !self.body_buffer.can_redo(),
                tree_redo_empty: self
                    .doc
                    .as_ref()
                    .map(|doc| !doc.can_redo_tree())
                    .unwrap_or(true),
                formatting: self.formatting_snapshot(),
            }
        }

        /// Facts for bold, italic, underline, lists, and the font dropdowns.
        fn formatting_snapshot(&self) -> FormattingSnapshot {
            if !self.body_view.is_editable() {
                return FormattingSnapshot::default();
            }
            let lines = list_lines_for_selection(&self.body_buffer);
            let Some((start, end)) = self.body_buffer.selection_bounds() else {
                return FormattingSnapshot {
                    body_editing: true,
                    lines,
                    ..FormattingSnapshot::default()
                };
            };
            if start.offset() == end.offset() {
                return FormattingSnapshot {
                    body_editing: true,
                    lines,
                    ..FormattingSnapshot::default()
                };
            }
            let tags = StyleTags::lookup(&self.body_buffer);
            let mut iter = start;
            let end_off = end.offset();
            let mut saw = false;
            let mut all_bold = true;
            let mut all_italic = true;
            let mut all_underline = true;
            let mut family: Option<FontFamily> = None;
            let mut size: Option<FontSize> = None;
            let mut mixed_family = false;
            let mut mixed_size = false;
            let mut cached_line = -1;
            let mut prefix_len = 0;
            while iter.offset() < end_off {
                if iter.line() != cached_line {
                    cached_line = iter.line();
                    prefix_len = line_prefix_len(&self.body_buffer, cached_line);
                }
                let ch = iter.char();
                if ch != '\n' && ch != '\r' && iter.line_offset() >= prefix_len {
                    saw = true;
                    let style = tags.style_at(&iter);
                    all_bold &= style.bold;
                    all_italic &= style.italic;
                    all_underline &= style.underline;
                    match family {
                        None => family = Some(style.family),
                        Some(current) if current != style.family => mixed_family = true,
                        _ => {}
                    }
                    match size {
                        None => size = Some(style.size),
                        Some(current) if current != style.size => mixed_size = true,
                        _ => {}
                    }
                }
                if !iter.forward_char() {
                    break;
                }
            }
            FormattingSnapshot {
                text_selected: saw,
                all_bold: saw && all_bold,
                all_italic: saw && all_italic,
                all_underline: saw && all_underline,
                shared_family: if saw && !mixed_family { family } else { None },
                shared_size: if saw && !mixed_size { size } else { None },
                body_editing: true,
                lines,
            }
        }

        fn refresh_title(&self, widgets: &AppModelWidgets) {
            let dirty = self.session_dirty();
            let path = self.doc.as_ref().and_then(|d| d.path());
            let title = window_title_for_path(path, dirty);
            widgets.main_window.set_title(Some(&title));
            let presented = present(&self.editor_snapshot(), None);
            self.save_button.set_sensitive(presented.save_enabled);
            self.insert_child_button
                .set_sensitive(presented.insert_child_enabled);
            self.insert_sibling_button
                .set_sensitive(presented.insert_sibling_enabled);
            self.delete_button.set_sensitive(presented.delete_enabled);
            self.move_up_button.set_sensitive(presented.move_up_enabled);
            self.move_down_button
                .set_sensitive(presented.move_down_enabled);
            self.indent_button.set_sensitive(presented.indent_enabled);
            self.outdent_button.set_sensitive(presented.outdent_enabled);
            self.undo_button.set_sensitive(presented.undo_enabled);
            self.redo_button.set_sensitive(presented.redo_enabled);
            self.format_bar.sync(&presented.formatting);
        }

        fn refresh_formatting(&self) {
            let presented = present(&self.editor_snapshot(), None);
            self.format_bar.sync(&presented.formatting);
        }

        /// Publish a bottom-line sentence only when presentation says the action failed.
        fn publish_outcome(&mut self, outcome: ActionOutcome) {
            if let Some(sentence) = present(&self.editor_snapshot(), Some(outcome)).status_sentence
            {
                self.status = sentence;
            }
        }

        /// Hint or blank body when no Node is selected. The hint is not editable.
        fn show_unselected_body(&self) {
            let presented = present(&self.editor_snapshot(), None);
            match presented.empty_hint {
                Some(hint) => self.set_body_text(hint),
                None => self.set_body_text(""),
            }
            self.set_body_editable(false);
            self.refresh_formatting();
        }

        fn request_with_dirty_prompt(
            &mut self,
            widgets: &AppModelWidgets,
            sender: &ComponentSender<Self>,
            root: &gtk::ApplicationWindow,
            pending: PendingAction,
        ) {
            if needs_dirty_prompt(self.session_dirty()) {
                self.pending = Some(pending);
                self.show_dirty_prompt(root, sender);
            } else {
                self.run_pending(widgets, sender, root, pending);
            }
        }

        fn show_dirty_prompt(
            &self,
            root: &gtk::ApplicationWindow,
            sender: &ComponentSender<Self>,
        ) {
            let dialog = gtk::MessageDialog::builder()
                .transient_for(root)
                .modal(true)
                .message_type(gtk::MessageType::Warning)
                .text("Save changes before continuing?")
                .secondary_text("Your Guide has unsaved changes.")
                .build();
            dialog.add_buttons(&[
                (DirtyPromptChoice::Save.label(), gtk::ResponseType::Accept),
                (DirtyPromptChoice::Discard.label(), gtk::ResponseType::Reject),
                (DirtyPromptChoice::Cancel.label(), gtk::ResponseType::Cancel),
            ]);
            let dialog_sender = sender.clone();
            dialog.connect_response(move |dialog, response| {
                dialog.destroy();
                let choice = match response {
                    gtk::ResponseType::Accept => DirtyPromptChoice::Save,
                    gtk::ResponseType::Reject => DirtyPromptChoice::Discard,
                    _ => DirtyPromptChoice::Cancel,
                };
                dialog_sender.input(Msg::DirtyChoice(choice));
            });
            dialog.show();
        }

        fn show_open_dialog(&self, root: &gtk::ApplicationWindow, sender: &ComponentSender<Self>) {
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
                    dialog_sender.input(Msg::OpenFailed("No file selected".into()));
                }
            });
            dialog.show();
        }

        fn show_save_as_dialog(
            &self,
            root: &gtk::ApplicationWindow,
            sender: &ComponentSender<Self>,
        ) {
            let dialog = gtk::FileChooserNative::builder()
                .title("Save Guide As")
                .action(gtk::FileChooserAction::Save)
                .accept_label("Save")
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
                        if let Some(mut path) = file.path() {
                            if path.extension().is_none() {
                                path.set_extension("gde");
                            }
                            dialog_sender.input(Msg::SaveAs(path));
                            return;
                        }
                    }
                    dialog_sender.input(Msg::SaveFailed("No file selected".into()));
                } else {
                    // Cancelled Save As while a dirty-prompt action was pending.
                    dialog_sender.input(Msg::DirtyChoice(DirtyPromptChoice::Cancel));
                }
            });
            dialog.show();
        }

        fn run_pending(
            &mut self,
            widgets: &AppModelWidgets,
            sender: &ComponentSender<Self>,
            root: &gtk::ApplicationWindow,
            pending: PendingAction,
        ) {
            match pending {
                PendingAction::New => self.replace_with_new(widgets),
                PendingAction::OpenDialog => self.show_open_dialog(root, sender),
                PendingAction::OpenPath(path) => self.load_opened(widgets, sender, path),
                PendingAction::Quit => {
                    widgets.main_window.destroy();
                }
            }
        }

        fn discard_edits(&mut self, widgets: &AppModelWidgets) {
            self.body_dirty = false;
            let restored = match self.doc.as_ref().and_then(|d| d.path().map(|p| p.to_path_buf()))
            {
                Some(path) => EditorDoc::open(path).unwrap_or_else(|_| EditorDoc::new()),
                None => EditorDoc::new(),
            };
            self.doc = Some(restored);
            self.selected = None;
            fill_tree(
                &self.tree_store,
                &self.doc.as_ref().map(|d| d.visible_tree()).unwrap_or_default(),
            );
            self.tree_view.expand_all();
            self.show_unselected_body();
            self.refresh_title(widgets);
        }

        fn replace_with_new(&mut self, widgets: &AppModelWidgets) {
            self.doc = Some(EditorDoc::new());
            self.selected = None;
            self.body_dirty = false;
            self.pending = None;
            fill_tree(&self.tree_store, &[]);
            self.show_unselected_body();
            // The empty-Guide sentence lives in the body now, not on the bottom line.
            self.status.clear();
            self.refresh_title(widgets);
        }

        fn load_opened(
            &mut self,
            widgets: &AppModelWidgets,
            sender: &ComponentSender<Self>,
            path: PathBuf,
        ) {
            match EditorDoc::open(&path) {
                Ok(doc) => {
                    self.selected = None;
                    self.body_dirty = false;
                    self.pending = None;
                    self.doc = Some(doc);
                    fill_tree(
                        &self.tree_store,
                        &self.doc.as_ref().unwrap().visible_tree(),
                    );
                    self.tree_view.expand_all();
                    self.show_unselected_body();
                    self.status = format!("Opened {}", path.display());
                    self.remember_recent(&path, sender);
                    self.refresh_title(widgets);
                }
                Err(err) => {
                    self.status = format!("Open failed: {err}");
                }
            }
        }

        fn remember_recent(&mut self, path: &std::path::Path, sender: &ComponentSender<Self>) {
            self.recent.record(path.to_path_buf());
            let _ = self.recent.save_to(&self.recent_store);
            self.rebuild_recent_menu(sender);
        }

        /// Pop down File and nested Recent popovers (MenuButton does not auto-dismiss
        /// nested menus when a recent row is activated).
        fn dismiss_file_menus(&self) {
            self.recent_popover.popdown();
            self.file_popover.popdown();
        }

        fn rebuild_recent_menu(&self, sender: &ComponentSender<Self>) {
            while let Some(child) = self.recent_menu.first_child() {
                self.recent_menu.remove(&child);
            }
            if self.recent.paths().is_empty() {
                let empty = gtk::Label::new(Some("No recent Guides"));
                empty.set_margin_all(4);
                self.recent_menu.append(&empty);
                return;
            }
            for path in self.recent.paths() {
                let label = path.display().to_string();
                let button = gtk::Button::with_label(&label);
                button.set_has_frame(false);
                button.set_halign(gtk::Align::Start);
                button.set_tooltip_text(Some(&label));
                let path = path.clone();
                let open_sender = sender.clone();
                // `Msg::Opened` dismisses the File/Recent popovers (single dismiss path).
                button.connect_clicked(move |_| {
                    open_sender.input(Msg::Opened(path.clone()));
                });
                self.recent_menu.append(&button);
            }
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

        /// Refresh the tree from the Core model and optionally select `path`.
        ///
        /// A successful structure edit publishes no sentence and does not take focus,
        /// so the pane that was active before the click keeps the undo stack.
        fn after_tree_op(&mut self, widgets: &AppModelWidgets, select: Option<NodePath>) {
            let Some(doc) = self.doc.as_ref() else {
                return;
            };
            fill_tree(&self.tree_store, &doc.visible_tree());
            self.tree_view.expand_all();
            if let Some(path) = select {
                // Clear so the selection-changed handler loads the body for `path`.
                self.selected = None;
                select_path_in_tree(&self.tree_view, &self.tree_store, &path);
            }
            self.refresh_title(widgets);
        }

        fn reorder_selected(&mut self, widgets: &AppModelWidgets, delta: isize) {
            if let Err(err) = self.commit_body_if_needed() {
                self.status = format!("Could not apply body edit: {err}");
                return;
            }
            let Some(selected) = self.selected.clone() else {
                self.publish_outcome(ActionOutcome::StructureRefused(
                    StructureRefusal::SelectNodeToReorder,
                ));
                return;
            };
            let Some(parent) = selected.parent() else {
                self.publish_outcome(ActionOutcome::StructureRefused(
                    StructureRefusal::CannotReorderDummyRoot,
                ));
                return;
            };
            let Some(index) = selected.last_index() else {
                self.publish_outcome(ActionOutcome::StructureRefused(
                    StructureRefusal::ReorderNoIndex,
                ));
                return;
            };
            let Some(doc) = self.doc.as_mut() else {
                self.publish_outcome(ActionOutcome::StructureRefused(StructureRefusal::NoGuide));
                return;
            };
            let sibling_count = match doc.guide().node(&parent) {
                Ok(node) => node.children().len(),
                Err(err) => {
                    self.publish_outcome(ActionOutcome::ReorderFailed {
                        detail: err.to_string(),
                    });
                    return;
                }
            };
            let new_index = index as isize + delta;
            if new_index < 0 || new_index as usize >= sibling_count {
                self.publish_outcome(ActionOutcome::StructureRefused(
                    StructureRefusal::AlreadyAtEdgeOfSiblings,
                ));
                return;
            }
            // move_node index is the insertion slot before removal adjustment.
            let insert_at = if delta > 0 {
                new_index as usize + 1
            } else {
                new_index as usize
            };
            match doc.move_node(&selected, &parent, insert_at) {
                Ok(path) => {
                    self.after_tree_op(widgets, Some(path));
                }
                Err(err) => {
                    self.publish_outcome(ActionOutcome::ReorderFailed {
                        detail: err.to_string(),
                    });
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

    /// Formatting icon buttons and font dropdowns. The shell renders `FormattingControls`.
    struct FormatBar {
        bold: gtk::ToggleButton,
        italic: gtk::ToggleButton,
        underline: gtk::ToggleButton,
        bullet: gtk::ToggleButton,
        numbered: gtk::ToggleButton,
        family: gtk::DropDown,
        size: gtk::DropDown,
        /// Set while syncing widgets so a dropdown notify does not apply a font.
        suppress: Rc<Cell<bool>>,
    }

    impl FormatBar {
        fn new(sender: &ComponentSender<AppModel>) -> Self {
            let seed = present(
                &EditorSnapshot {
                    guide_open: false,
                    dirty: false,
                    has_nodes: false,
                    node_selected: false,
                    focused_pane: FocusedPane::Tree,
                    body_undo_empty: true,
                    tree_undo_empty: true,
                    body_redo_empty: true,
                    tree_redo_empty: true,
                    formatting: FormattingSnapshot::default(),
                },
                None,
            )
            .formatting;
            let suppress = Rc::new(Cell::new(false));
            let bold = format_toggle("format-text-bold-symbolic", sender, Msg::ToggleBold);
            let italic = format_toggle("format-text-italic-symbolic", sender, Msg::ToggleItalic);
            let underline = format_toggle(
                "format-text-underline-symbolic",
                sender,
                Msg::ToggleUnderline,
            );
            let bullet = format_toggle(
                "format-list-unordered-symbolic",
                sender,
                Msg::ToggleBulletList,
            );
            let numbered = format_toggle(
                "format-list-ordered-symbolic",
                sender,
                Msg::ToggleNumberedList,
            );
            let family = font_dropdown(seed.family.choices.iter().map(|family| family.label()));
            let size = font_dropdown(seed.size.choices.iter().map(|size| size.pt().to_string()));
            connect_family_dropdown(&family, &suppress, sender, seed.family.choices);
            connect_size_dropdown(&size, &suppress, sender, seed.size.choices);
            Self {
                bold,
                italic,
                underline,
                bullet,
                numbered,
                family,
                size,
                suppress,
            }
        }

        fn attach(&self, bar: &gtk::Box) {
            let characters = gtk::Box::new(gtk::Orientation::Horizontal, 2);
            characters.append(&self.bold);
            characters.append(&self.italic);
            characters.append(&self.underline);
            let lists = gtk::Box::new(gtk::Orientation::Horizontal, 2);
            lists.set_margin_start(12);
            lists.append(&self.bullet);
            lists.append(&self.numbered);
            let fonts = gtk::Box::new(gtk::Orientation::Horizontal, 6);
            fonts.set_margin_start(12);
            fonts.append(&self.family);
            fonts.append(&self.size);
            bar.append(&characters);
            bar.append(&lists);
            bar.append(&fonts);
        }

        fn sync(&self, controls: &FormattingControls) {
            self.suppress.set(true);
            sync_toggle(&self.bold, controls.bold);
            sync_toggle(&self.italic, controls.italic);
            sync_toggle(&self.underline, controls.underline);
            sync_toggle(&self.bullet, controls.bullet_list);
            sync_toggle(&self.numbered, controls.numbered_list);
            sync_dropdown(
                &self.family,
                controls.family.enabled,
                choice_index(controls.family.choices, controls.family.displayed),
            );
            sync_dropdown(
                &self.size,
                controls.size.enabled,
                choice_index(controls.size.choices, controls.size.displayed),
            );
            self.suppress.set(false);
        }
    }

    fn format_toggle(
        icon: &str,
        sender: &ComponentSender<AppModel>,
        message: Msg,
    ) -> gtk::ToggleButton {
        let button = gtk::ToggleButton::new();
        button.set_icon_name(icon);
        button.set_focus_on_click(false);
        let tx = sender.input_sender().clone();
        button.connect_clicked(move |_| tx.emit(message.clone()));
        button
    }

    fn font_dropdown(labels: impl IntoIterator<Item = impl AsRef<str>>) -> gtk::DropDown {
        let owned: Vec<String> = labels
            .into_iter()
            .map(|label| label.as_ref().to_string())
            .collect();
        let refs: Vec<&str> = owned.iter().map(String::as_str).collect();
        let dropdown = gtk::DropDown::from_strings(&refs);
        dropdown.set_focus_on_click(false);
        dropdown.set_hexpand(false);
        dropdown.set_selected(gtk::INVALID_LIST_POSITION);
        dropdown
    }

    fn connect_family_dropdown(
        dropdown: &gtk::DropDown,
        suppress: &Rc<Cell<bool>>,
        sender: &ComponentSender<AppModel>,
        choices: &'static [FontFamily],
    ) {
        let suppress = suppress.clone();
        let tx = sender.input_sender().clone();
        dropdown.connect_selected_notify(move |dropdown| {
            if suppress.get() {
                return;
            }
            let Some(family) = choices.get(dropdown.selected() as usize).copied() else {
                return;
            };
            tx.emit(Msg::SetFontFamily(family));
        });
    }

    fn connect_size_dropdown(
        dropdown: &gtk::DropDown,
        suppress: &Rc<Cell<bool>>,
        sender: &ComponentSender<AppModel>,
        choices: &'static [FontSize],
    ) {
        let suppress = suppress.clone();
        let tx = sender.input_sender().clone();
        dropdown.connect_selected_notify(move |dropdown| {
            if suppress.get() {
                return;
            }
            let Some(size) = choices.get(dropdown.selected() as usize).copied() else {
                return;
            };
            tx.emit(Msg::SetFontSize(size));
        });
    }

    fn sync_toggle(button: &gtk::ToggleButton, control: FormatToggle) {
        button.set_sensitive(control.enabled);
        button.set_active(control.pressed);
        button.set_tooltip_text(Some(control.tooltip));
    }

    fn sync_dropdown(dropdown: &gtk::DropDown, enabled: bool, selected: Option<u32>) {
        dropdown.set_sensitive(enabled);
        dropdown.set_selected(selected.unwrap_or(gtk::INVALID_LIST_POSITION));
    }

    fn choice_index<T: PartialEq>(choices: &[T], displayed: Option<T>) -> Option<u32> {
        displayed.and_then(|value| {
            choices
                .iter()
                .position(|item| *item == value)
                .map(|index| index as u32)
        })
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
        // Changing Node selection seals the previous body's GTK undo stack.
        buffer.begin_irreversible_action();
        buffer.set_text("");
        let Ok(node) = doc.guide().node(path) else {
            buffer.end_irreversible_action();
            suppress.set(false);
            return;
        };
        match load_rtf(node.rtf_body()) {
            LoadBody::Editable(document) => apply_document(buffer, &document),
            LoadBody::Unparseable { plain_fallback } => {
                buffer.set_text(&plain_fallback);
            }
        }
        buffer.end_irreversible_action();
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
            if run.style.family != FontFamily::default() {
                buffer.apply_tag_by_name(family_tag_name(run.style.family), &start, &end);
            }
            if run.style.size != FontSize::default() {
                buffer.apply_tag_by_name(&size_tag_name(run.style.size), &start, &end);
            }
        }
    }

    fn family_tag_name(family: FontFamily) -> &'static str {
        match family {
            FontFamily::Arial => "family-arial",
            FontFamily::Sans => "family-sans",
        }
    }

    /// Pango family name for a [`FontFamily`] (`Sans` is the system default sans alias).
    fn family_pango_name(family: FontFamily) -> &'static str {
        match family {
            FontFamily::Arial => "Arial",
            FontFamily::Sans => "Sans",
        }
    }

    fn size_tag_name(size: FontSize) -> String {
        format!("size-{}", size.pt())
    }

    /// Replace any tag in `group` on the selection with `tag`. Needs a non-empty selection.
    fn apply_exclusive_tag(buffer: &gtk::TextBuffer, group: &[String], tag: &str) -> bool {
        let Some((start, end)) = buffer.selection_bounds() else {
            return false;
        };
        if start == end {
            return false;
        }
        for name in group {
            buffer.remove_tag_by_name(name, &start, &end);
        }
        buffer.apply_tag_by_name(tag, &start, &end);
        true
    }

    /// Looked-up style tags for reading [`SpanStyle`] back out of the buffer.
    struct StyleTags {
        bold: Option<gtk::TextTag>,
        italic: Option<gtk::TextTag>,
        underline: Option<gtk::TextTag>,
        families: Vec<(FontFamily, gtk::TextTag)>,
        sizes: Vec<(FontSize, gtk::TextTag)>,
    }

    impl StyleTags {
        fn lookup(buffer: &gtk::TextBuffer) -> Self {
            let table = buffer.tag_table();
            Self {
                bold: table.lookup("bold"),
                italic: table.lookup("italic"),
                underline: table.lookup("underline"),
                families: FontFamily::ALL
                    .iter()
                    .filter_map(|f| table.lookup(family_tag_name(*f)).map(|t| (*f, t)))
                    .collect(),
                sizes: FontSize::ALL
                    .iter()
                    .filter_map(|s| table.lookup(&size_tag_name(*s)).map(|t| (*s, t)))
                    .collect(),
            }
        }

        fn style_at(&self, iter: &gtk::TextIter) -> SpanStyle {
            SpanStyle {
                bold: self.bold.as_ref().is_some_and(|t| iter.has_tag(t)),
                italic: self.italic.as_ref().is_some_and(|t| iter.has_tag(t)),
                underline: self.underline.as_ref().is_some_and(|t| iter.has_tag(t)),
                family: self
                    .families
                    .iter()
                    .find(|(_, t)| iter.has_tag(t))
                    .map(|(f, _)| *f)
                    .unwrap_or_default(),
                size: self
                    .sizes
                    .iter()
                    .find(|(_, t)| iter.has_tag(t))
                    .map(|(s, _)| *s)
                    .unwrap_or_default(),
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

    /// Toggle bullet/numbered list tags (+ display prefixes) on selected lines.
    ///
    /// List structure is owned by `list-bullet` / `list-numbered` tags so
    /// [`document_from_buffer`] round-trips through the `nextguide-rtf` model.
    /// Lines a list toggle would touch. A caret is its one line.
    fn selection_line_span(buffer: &gtk::TextBuffer) -> (i32, i32) {
        let (sel_start, sel_end) = match buffer.selection_bounds() {
            Some(bounds) => bounds,
            None => {
                let insert = buffer.iter_at_mark(&buffer.get_insert());
                (insert.clone(), insert)
            }
        };
        let start_line = sel_start.line();
        let mut end_line = sel_end.line();
        if sel_end.line_offset() == 0
            && sel_end.offset() > sel_start.offset()
            && end_line > start_line
        {
            end_line -= 1;
        }
        (start_line, end_line)
    }

    fn list_lines_for_selection(buffer: &gtk::TextBuffer) -> Vec<ListLineKind> {
        let (start_line, end_line) = selection_line_span(buffer);
        let list_bullet = buffer.tag_table().lookup("list-bullet");
        let list_numbered = buffer.tag_table().lookup("list-numbered");
        let mut lines = Vec::new();
        for line in start_line..=end_line {
            let Some(iter) = buffer.iter_at_line(line) else {
                continue;
            };
            let kind = line_kind_from_tags(&iter, list_bullet.as_ref(), list_numbered.as_ref());
            lines.push(match kind {
                LineKind::Paragraph => ListLineKind::Paragraph,
                LineKind::Bullet => ListLineKind::Bullet,
                LineKind::Numbered => ListLineKind::Numbered,
            });
        }
        lines
    }

    fn line_prefix_len(buffer: &gtk::TextBuffer, line: i32) -> i32 {
        let Some(line_start) = buffer.iter_at_line(line) else {
            return 0;
        };
        let list_bullet = buffer.tag_table().lookup("list-bullet");
        let list_numbered = buffer.tag_table().lookup("list-numbered");
        let kind = line_kind_from_tags(&line_start, list_bullet.as_ref(), list_numbered.as_ref());
        let mut line_end = line_start.clone();
        if !line_end.ends_line() {
            line_end.forward_to_line_end();
        }
        let text = buffer.text(&line_start, &line_end, false);
        list_prefix_chars(&text, kind) as i32
    }

    fn toggle_list_kind(buffer: &gtk::TextBuffer, want: LineKind) -> bool {
        let (start_line, end_line) = selection_line_span(buffer);

        let list_bullet = buffer.tag_table().lookup("list-bullet");
        let list_numbered = buffer.tag_table().lookup("list-numbered");

        let mut all_want = true;
        for line in start_line..=end_line {
            let Some(iter) = buffer.iter_at_line(line) else {
                continue;
            };
            let kind = line_kind_from_tags(&iter, list_bullet.as_ref(), list_numbered.as_ref());
            if kind != want {
                all_want = false;
                break;
            }
        }

        let target = if all_want {
            LineKind::Paragraph
        } else {
            want
        };

        buffer.begin_user_action();
        // Bottom-up so prefix insert/delete does not shift later line indexes mid-pass.
        for line in (start_line..=end_line).rev() {
            set_line_list_kind(
                buffer,
                line,
                target,
                list_bullet.as_ref(),
                list_numbered.as_ref(),
            );
        }
        if matches!(target, LineKind::Numbered) {
            renumber_numbered_lines(buffer, list_numbered.as_ref());
        }
        buffer.end_user_action();
        true
    }

    fn line_kind_from_tags(
        line_iter: &gtk::TextIter,
        list_bullet: Option<&gtk::TextTag>,
        list_numbered: Option<&gtk::TextTag>,
    ) -> LineKind {
        if list_bullet.is_some_and(|t| line_iter.has_tag(t)) {
            LineKind::Bullet
        } else if list_numbered.is_some_and(|t| line_iter.has_tag(t)) {
            LineKind::Numbered
        } else {
            LineKind::Paragraph
        }
    }

    fn set_line_list_kind(
        buffer: &gtk::TextBuffer,
        line: i32,
        target: LineKind,
        list_bullet: Option<&gtk::TextTag>,
        list_numbered: Option<&gtk::TextTag>,
    ) {
        let Some(mut line_start) = buffer.iter_at_line(line) else {
            return;
        };
        let mut line_end = line_start.clone();
        if !line_end.ends_line() {
            line_end.forward_to_line_end();
        }

        let current = line_kind_from_tags(&line_start, list_bullet, list_numbered);
        let line_text = buffer.text(&line_start, &line_end, true);
        let prefix_chars = list_prefix_chars(&line_text, current);

        if let Some(tag) = list_bullet {
            buffer.remove_tag(tag, &line_start, &line_end);
        }
        if let Some(tag) = list_numbered {
            buffer.remove_tag(tag, &line_start, &line_end);
        }

        if prefix_chars > 0 {
            let mut del_end = line_start.clone();
            del_end.forward_chars(prefix_chars as i32);
            buffer.delete(&mut line_start, &mut del_end);
            // `line_start` / `line_end` are invalidated; re-resolve.
            let Some(fresh_start) = buffer.iter_at_line(line) else {
                return;
            };
            line_start = fresh_start;
            line_end = line_start.clone();
            if !line_end.ends_line() {
                line_end.forward_to_line_end();
            }
        }

        let insert_prefix = match target {
            LineKind::Paragraph => None,
            LineKind::Bullet => Some("• ".to_string()),
            LineKind::Numbered => Some("1. ".to_string()),
        };
        if let Some(prefix) = insert_prefix {
            buffer.insert(&mut line_start, &prefix);
            let Some(fresh_start) = buffer.iter_at_line(line) else {
                return;
            };
            line_start = fresh_start;
            line_end = line_start.clone();
            if !line_end.ends_line() {
                line_end.forward_to_line_end();
            }
        }

        let tag_name = match target {
            LineKind::Paragraph => None,
            LineKind::Bullet => Some("list-bullet"),
            LineKind::Numbered => Some("list-numbered"),
        };
        if let Some(name) = tag_name {
            buffer.apply_tag_by_name(name, &line_start, &line_end);
        }
    }

    fn list_prefix_chars(line: &str, kind: LineKind) -> usize {
        match kind {
            LineKind::Paragraph => 0,
            LineKind::Bullet => {
                if line.starts_with("• ") {
                    2
                } else {
                    0
                }
            }
            LineKind::Numbered => {
                let chars: Vec<char> = line.chars().collect();
                let mut i = 0;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                if i > 0 && i + 1 < chars.len() && chars[i] == '.' && chars[i + 1] == ' ' {
                    i + 2
                } else {
                    0
                }
            }
        }
    }

    /// Refresh display prefixes for consecutive numbered-tagged lines.
    fn renumber_numbered_lines(buffer: &gtk::TextBuffer, list_numbered: Option<&gtk::TextTag>) {
        let Some(list_numbered) = list_numbered else {
            return;
        };
        let line_count = buffer.line_count();
        let mut number = 1u32;
        for line in 0..line_count {
            let Some(mut line_start) = buffer.iter_at_line(line) else {
                continue;
            };
            if !line_start.has_tag(list_numbered) {
                number = 1;
                continue;
            }
            let mut line_end = line_start.clone();
            if !line_end.ends_line() {
                line_end.forward_to_line_end();
            }
            let line_text = buffer.text(&line_start, &line_end, true);
            let old_prefix = list_prefix_chars(&line_text, LineKind::Numbered);
            let new_prefix = format!("{number}. ");
            number += 1;
            if old_prefix > 0 {
                let mut del_end = line_start.clone();
                del_end.forward_chars(old_prefix as i32);
                buffer.delete(&mut line_start, &mut del_end);
            }
            let Some(mut fresh_start) = buffer.iter_at_line(line) else {
                continue;
            };
            buffer.insert(&mut fresh_start, &new_prefix);
            let Some(tag_start) = buffer.iter_at_line(line) else {
                continue;
            };
            let mut tag_end = tag_start.clone();
            if !tag_end.ends_line() {
                tag_end.forward_to_line_end();
            }
            buffer.apply_tag(list_numbered, &tag_start, &tag_end);
        }
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

        let style_tags = StyleTags::lookup(buffer);
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
                &style_tags,
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
        tags: &StyleTags,
    ) -> Vec<Run> {
        if content.is_empty() {
            return Vec::new();
        }
        let chars: Vec<char> = content.chars().collect();
        let style_at = |i: usize| -> SpanStyle {
            let iter = buffer.iter_at_offset((start_off + i) as i32);
            tags.style_at(&iter)
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

    /// Raise the main window and activate the process for terminal launches.
    fn bring_main_window_forward(window: &impl IsA<gtk::Window>) {
        window.present();
        #[cfg(target_os = "macos")]
        crate::macos_focus::activate_running_app();
    }
}

fn main() {
    let initial = nextguide::launch_path_from_args(std::env::args_os());

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
