use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, Button, CompositeTemplate, Image};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/xyz/neupokoev/QuRobot/ui/window.ui")]
pub struct Window {
    #[template_child]
    pub button: TemplateChild<Button>,
    #[template_child]
    pub image: TemplateChild<Image>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
  fn constructed(&self) {
      // Call "constructed" on parent
      self.parent_constructed();

      // Connect to "clicked" signal of `button`
      self.button.connect_clicked(move |button| {
          // Set the label to "Hello World!" after the button has been clicked on
          button.set_label("Hello World!");
      });

      let filename = "reward_vs_steps.png";
      self.image.set_from_file(Some(filename));
  }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}