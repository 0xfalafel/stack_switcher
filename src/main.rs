use gtk::prelude::*;
// use relm4::gtk::{gdk, Grid};
use relm4::prelude::*;

struct App {

}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        gtk::ApplicationWindow {
            set_default_size: (477, 400),
            set_resizable: false,

            // #[wrap(Some)]
            // set_titlebar = &gtk::StackSwitcher {
            //     gtk::Box {

            //         gtk::Button {
            //             set_label: "Page 1"
            //         }
            //     }
            // },

            gtk::Stack {

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 5,
                    set_spacing: 5,

                    gtk::Label {
                        set_label: "Hi mom!"
                    }
                }
            },

            gtk::Stack {

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 5,
                    set_spacing: 5,

                    gtk::Label {
                        set_label: "Hi dad!"
                    }
                }
            }

        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
            
        let model = App {
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

fn main() {
    let app = RelmApp::new("falafel.stackswitcher");
    app.run::<App>(());
}
