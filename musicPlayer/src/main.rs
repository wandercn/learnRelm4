use gtk::prelude::*;
use rand::prelude::*;
use relm4::prelude::*;

#[derive(Debug)]
enum Msg {
    Play,
    Suspend,
    Stop,
    Next,
    Prev,
}

struct App {
    music_dir: &'static str,
    volume: f64,
    play_mode: Modes,
    music_library: Option<Vec<Song>>,
}

enum Modes {
    Order,
    Random,
    Repet,
}

struct Song {
    title: String,
    artist: String,
    album: String,
    duration: f64,
    playing: bool,
    date: String,
    file: String,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = Msg;
    type Output = ();

    view! {
        #[root]
        gtk::Window {
            gtk::Box{
                set_orientation:gtk::Orientation::Vertical,
                gtk::SearchEntry{
                    set_placeholder_text:Some("搜索"),
                        },
                        gtk::Box{
                            set_orientation:gtk::Orientation::Vertical,


                            gtk::Button{
                                set_height_request:48,
                                set_width_request:48,
                                set_icon_name:"media-optical-symbolic",

                            },
                            gtk::Button{
                                set_height_request:48,
                                set_width_request:48,
                            set_icon_name:"media-playlist-consecutive-symbolic"
                            }
                        },
            },

        }
    }

    fn update(&mut self, msg: Msg, _sender: ComponentSender<Self>) {
        match msg {
            Msg::Play => println!("Play"),
            Msg::Suspend => println!("Suspend"),
            Msg::Stop => println!("Stop"),
            Msg::Next => println!("Next"),
            Msg::Prev => println!("Prev"),
        }
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App {
            music_dir: "",
            volume: 0.4,
            play_mode: Modes::Order,
            music_library: None,
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

fn main() {
    let app = RelmApp::new("player.ffactory.org");
    app.run::<App>(());
}
