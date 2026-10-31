//! NextGuide Editor v1 — Relm4 / GTK4 stub shell.

#[cfg(feature = "ui")]
mod ui {
    use relm4::gtk;
    use relm4::gtk::prelude::*;
    use relm4::prelude::*;

    use nextguide::{window_title_for_path, APP_ID};

    struct AppModel {
        title: String,
    }

    #[relm4::component]
    impl SimpleComponent for AppModel {
        type Init = ();
        type Input = ();
        type Output = ();

        view! {
            gtk::Window {
                set_title: Some(model.title.as_str()),
                set_default_size: (800, 600),

                gtk::Label {
                    set_label: "NextGuide Editor v1 (stub)",
                    set_margin_all: 12,
                }
            }
        }

        fn init(
            _init: Self::Init,
            _root: Self::Root,
            _sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
            let model = AppModel {
                title: window_title_for_path(None),
            };
            let widgets = view_output!();
            ComponentParts { model, widgets }
        }
    }

    pub fn run() {
        let app = RelmApp::new(APP_ID);
        app.run::<AppModel>(());
    }
}

fn main() {
    #[cfg(feature = "ui")]
    ui::run();

    #[cfg(not(feature = "ui"))]
    {
        eprintln!(
            "nextguide was built without the `ui` feature (GTK4 + Relm4).\n\
             Rebuild with: cargo run -p nextguide --features ui"
        );
        std::process::exit(1);
    }
}
