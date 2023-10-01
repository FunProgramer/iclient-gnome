use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/funprogramer/iclient/gnome/window.ui")]
    pub struct IClientGnomeWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<gtk::HeaderBar>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for IClientGnomeWindow {
        const NAME: &'static str = "IClientGnomeWindow";
        type Type = super::IClientGnomeWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for IClientGnomeWindow {}
    impl WidgetImpl for IClientGnomeWindow {}
    impl WindowImpl for IClientGnomeWindow {}
    impl ApplicationWindowImpl for IClientGnomeWindow {}
    impl AdwApplicationWindowImpl for IClientGnomeWindow {}
}

glib::wrapper! {
    pub struct IClientGnomeWindow(ObjectSubclass<imp::IClientGnomeWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl IClientGnomeWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
