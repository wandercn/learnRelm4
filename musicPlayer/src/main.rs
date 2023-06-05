use gtk::{ffi::gtk_file_chooser_get_file, prelude::*, FileChooserDialog};
use rand::prelude::*;
use relm4::prelude::*;
use relm4_components::{
    open_button::{OpenButton, OpenButtonSettings},
    open_dialog::OpenDialogSettings,
};
use std::path::PathBuf;

#[derive(Debug)]
enum Msg {
    Play,
    Suspend,
    Stop,
    Next,
    Prev,
    ShowSearchBar,
    CloseWindows,
    OpenFolder,
    Open(PathBuf),
}

struct App {
    music_dir: &'static str,
    volume: f64,
    play_mode: Modes,
    music_library: Option<Vec<Song>>,
    search_mode_enable: bool,
    showSelectFile: bool,
    open_button: Controller<OpenButton>,
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
        set_modal:true,
        #[wrap(Some)]
        set_titlebar =&gtk::HeaderBar{
            pack_start: model.open_button.widget(),
        },
        // relm4::Component::SaveDialogSettings,
        // gtk::FileChooserDialog{
        // // set_select_multiple:true,
        // // set_create_folders:true,
        // // set_mnemonics_visible:true,
        // // // set_handle_menubar_accel:true,
        // set_modal:true,
        // #[watch]
        // set_visible:model.showSelectFile,
        // set_deletable:true,
        // set_destroy_with_parent:true,
        // add_buttons:&[("打开",gtk::ResponseType::Ok),("取消",gtk::ResponseType::Cancel)],
        // set_can_focus:true,
        // set_focus_on_click:true,
        // #[wrap(Some)]
        // set_current_folder=&gio::File::for_path("~/"),

        // // set_overflow:gtk::Overflow::Visible,
        // // set_title:Some("Open file"),
        // set_action:gtk::FileChooserAction::SelectFolder,
        // // set_accept_label:Some("确定"),
        // // set_cancel_label:Some("取消"),
        // set_default_height: 450,
        // set_default_width: 300,
        // // set_accessible_role:gtk::AccessibleRole::,
        // // connect_response[sender] => move|dialog, res|{
        // //     match res {
        // //         match res{
        // //             gtk::ResponseType::Ok{
        // //             sender.input(Msg::)
        // //             }
        // //         }

        // //     }

        // // }


        // },
        set_width_request: 600,
        set_height_request:400,
        //  connect_destroy[sender]=> move |_| {sender.input(Msg::CloseWindows)},
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
                            connect_clicked=> Msg::OpenFolder,
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
        }


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
            Msg::OpenFolder => self.showSelectFile = true,
            Msg::Open(path) => {
                println!("* Opened file {path:?} *");
            }
        }
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let open_button = OpenButton::builder()
            .launch(OpenButtonSettings {
                dialog_settings: OpenDialogSettings::default(),
                text: "打开文件夹",
                recently_opened_files: Some(".recent_files"),
                max_recent_files: 10,
            })
            .forward(sender.input_sender(), Msg::Open);
        let model = App {
            music_dir: "",
            volume: 0.4,
            play_mode: Modes::Order,
            music_library: None,
            search_mode_enable: false,
            showSelectFile: true,
            open_button: open_button,
        };

        // let openFile = FileChooserDialog::new()

        // let filedialog: FileChooserDialog = openFile
        //     .select_multiple(true)
        //     .title(glib::gstr!("Open file"))
        //     .visible(true)
        //     .action(gtk::FileChooserAction::Open)
        //     .build();
        // root.container_add(&filedialog);
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

fn main() {
    let app = RelmApp::new("player.ffactory.org");

    app.run::<App>(());
}
