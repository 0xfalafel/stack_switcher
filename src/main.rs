use gtk::prelude::*;
use relm4::{
    component, gtk, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent,
};

#[derive(Default)]
struct StackApp {
    current_page: u8,
}

#[derive(Debug)]
enum StackAppMsg {
    SwitchPage(u8),
}

#[component]
impl SimpleComponent for StackApp {
    type Init = ();
    type Input = StackAppMsg;
    type Output = ();
    type Widgets = StackAppWidgets;

    view! {
        gtk::ApplicationWindow {
            set_default_size: (300, 200),

            #[wrap(Some)]
            set_titlebar = &gtk::HeaderBar {
                
                #[wrap(Some)]
                set_title_widget = &gtk::StackSwitcher {
                    set_stack: Some(&stack),
                    set_halign: gtk::Align::Center,
                },
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 6,
                set_margin_all: 12,

                #[name = "stack"]
                gtk::Stack {
                    set_transition_type: gtk::StackTransitionType::SlideLeftRight,
                    set_transition_duration: 200,

                    add_titled: (
                        &gtk::Label::new(Some("PAGE 1")),
                        Some("Hi mom!"),
                        "Page 1"
                    ),

                    add_titled: (
                        &gtk::Label::new(Some("PAGE 2")),
                        Some("Page 2"),
                        "Page 2"
                    ),
                }
            }
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = StackApp::default();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            StackAppMsg::SwitchPage(page) => {
                println!("clicked page {}", page);
                self.current_page = page;
            }
        }
    }

    // fn post_update(&mut self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
    //     // Update the visible page based on current_page
    //     match self.current_page {
    //         0 => widgets.stack.set_visible_child_name("page1"),
    //         1 => widgets.stack.set_visible_child_name("page2"),
    //         _ => unreachable!(),
    //     }
    // }
}

fn main() {
    let app = relm4::RelmApp::new("org.example.MinimalStackExample");
    app.run::<StackApp>(());
}