use relm4::gtk::prelude::*;
use relm4::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

pub struct ComponentModel {}

#[derive(Debug)]
pub enum ComponentInput {}

#[derive(Debug)]
pub enum ComponentOutput {}

pub struct ComponentInit {}

#[relm4::component(pub)]
impl SimpleComponent for ComponentModel {
    type Input = ComponentInput;
    type Output = ComponentOutput;
    type Init = ();
    view! {
                #[root]
                gtk::Window{
                    set_title:Some("MusicPlayer"),
                    gtk::HeaderBar{

                        #[wrap(Some)]
        set_title_widget = &gtk::Label{
            #[watch]
    set_label:"musicPlayer",

        }
                    },
                    gtk::Box{

                    }


                }
            }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ComponentModel {};
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {}
    }
}
fn main() {
    let app = RelmApp::new("musicplayer.ffactory.org");
    app.run::<ComponentModel>(());
}
