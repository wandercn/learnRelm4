use gtk::{prelude::*, FileChooserDialog};
use rand::prelude::*;
use relm4::prelude::*;

#[derive(Debug)]
enum Msg {
    Play,
    Suspend,
    Stop,
    Next,
    Prev,
    ShowSearchBar,
    CloseWindows,
    // OpenFolder,
}

struct App {
    music_dir: &'static str,
    volume: f64,
    play_mode: Modes,
    music_library: Option<Vec<Song>>,
    search_mode_enable: bool,
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
            set_width_request: 600,
            set_height_request:400,
             connect_destroy[sender]=> move |_| {sender.input(Msg::CloseWindows)},
            gtk::Paned{
                set_position:100,
                set_size_request:(600,-1),
                #[wrap(Some)]
                set_start_child = &gtk::Frame{

                set_size_request:(100,-1),
                gtk::Box{
                    set_margin_all:5,
                    set_spacing:10,
                    set_orientation:gtk::Orientation::Vertical,

                    gtk::Button{
                        set_icon_name:"edit-find-symbolic",
                        set_tooltip_markup:Some("搜索"),
                        set_height_request:64,
                        set_width_request:64,

                        connect_clicked =>Msg::ShowSearchBar,
                            },
                                set_orientation:gtk::Orientation::Vertical,


                                gtk::Button{
                                    set_height_request:64,
                                    set_width_request:64,
                                    set_icon_name:"media-optical-symbolic",
                                    set_tooltip_markup:Some("音乐库"),
                                    // connect_activate => println!("{}",css_name()),

                                },
                                gtk::Button{
                                    set_height_request:64,
                                    set_width_request:64,
                                set_icon_name:"media-playlist-consecutive-symbolic",
                                set_tooltip_markup:Some("播放列表"),
                                }

                },},
                #[wrap(Some)]
                set_end_child=&gtk::Frame{
                    gtk::Box{
                        set_orientation: gtk::Orientation::Vertical,
                        gtk::Box{
                            set_orientation: gtk::Orientation::Horizontal,
                            // append = &filedialog,
                            // set_height_request:20,
                            gtk::Button{
                                set_icon_name:"folder-open-symbolic",
                                // connect_clicked=>filedialog.clone()
                            }
                        },
                    gtk::SearchBar{
                        set_visible:true,
                        #[watch]
                        set_search_mode:model.search_mode_enable,
                        // set_show_close_button:true,
                    #[wrap(Some)]
                        set_child = &gtk::SearchEntry{
                            set_editable:true,
                            set_enable_undo:true,
                            set_placeholder_text: Some("搜索"),
                            set_max_width_chars: 25,
                       },
                    },gtk::Label{
                    set_label:"right",

                },
            },
                set_size_request:(200,-1),
            },
            },


        }
    }

    fn update(&mut self, msg: Msg, sender: ComponentSender<Self>) {
        match msg {
            Msg::Play => println!("Play"),
            Msg::Suspend => println!("Suspend"),
            Msg::Stop => println!("Stop"),
            Msg::Next => println!("Next"),
            Msg::Prev => println!("Prev"),
            Msg::ShowSearchBar => {
                println!("{}", self.search_mode_enable);
                self.search_mode_enable = !self.search_mode_enable;
            }
            Msg::CloseWindows => relm4::main_application().quit(),
            // Msg::OpenFolder => ,
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
            search_mode_enable: false,
        };
        let openFile = FileChooserDialog::builder();

        let filedialog = openFile
            .select_multiple(true)
            .title(glib::gstr!("Open file"))
            .visible(true)
            .action(gtk::FileChooserAction::Open)
            .build();

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

fn main() {
    let app = RelmApp::new("player.ffactory.org");

    app.run::<App>(());
}
