use gtk::prelude::*;
use adw::subclass::prelude::*;
use webkit6::{prelude::*, WebView};
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
        pub web_view_container: TemplateChild<gtk::ScrolledWindow>,
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

    impl ObjectImpl for IClientGnomeWindow {
        fn constructed(&self) {
            self.parent_constructed();

            let web_view = WebView::new();
            web_view.load_uri("https://community.iserv.eu");
            web_view.set_vexpand(true);
            self.web_view_container.set_child(Some(&web_view));
        }
    }
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
