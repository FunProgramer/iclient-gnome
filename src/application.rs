use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::config::VERSION;
use crate::IClientGnomeWindow;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct IClientGnomeApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for IClientGnomeApplication {
        const NAME: &'static str = "IClientGnomeApplication";
        type Type = super::IClientGnomeApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for IClientGnomeApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
        }
    }

    impl ApplicationImpl for IClientGnomeApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            let application = self.obj();
            // Get the current window or create one if necessary
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = IClientGnomeWindow::new(&*application);
                window.upcast()
            };

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for IClientGnomeApplication {}
    impl AdwApplicationImpl for IClientGnomeApplication {}
}

glib::wrapper! {
    pub struct IClientGnomeApplication(ObjectSubclass<imp::IClientGnomeApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl IClientGnomeApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        self.add_action_entries([quit_action, about_action]);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = adw::AboutWindow::builder()
            .transient_for(&window)
            .application_name("iClient")
            .application_icon("com.github.funprogramer.iclient.gnome")
            .developer_name("FunProgramer")
            .version(VERSION)
            .developers(vec!["FunProgramer"])
            .copyright("© 2023 FunProgramer")
            .build();

        about.present();
    }
}
