#[macro_export]
macro_rules! screen_registry {
    {
        screens: [
            $(
                $id:ident => $screen:ident $(($screen_arg:expr))?
            ),+ $(,)?
        ],
        $(
            args: [
                $(
                    $arg:ident: $arg_ty:ty
                ),+ $(,)?
            ],
        )?
    } => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum ScreenId {
            $(
                $id
            ),+
        }

        impl ScreenId {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(
                        ScreenId::$id => stringify!($id)
                    ),+
                }
            }
        }

        pub enum Screen {
            $(
                $id(relm::Component<$screen>),
            )+
        }

        impl Screen {
            pub fn widget(&self) -> &impl gtk::prelude::IsA<gtk::Widget> {
                match self {
                    $(
                        Screen::$id(widget) => widget.widget(),
                    )+
                }
            }
        }

        pub struct Screens {
            map: ::std::collections::HashMap<ScreenId, Screen>,
        }

        impl Screens {
            pub fn new($($( $arg: $arg_ty, )+)?) -> Self {
                let mut map = ::std::collections::HashMap::new();

                $(
                    map.insert(
                        ScreenId::$id, 
                        Screen::$id(relm::init(($($screen_arg)?)).unwrap())
                    );
                )+

                Screens { map }
            }

            pub fn get(&self, id: ScreenId) -> &Screen {
                &self.map[&id]
            }
        }

        impl std::ops::Index<ScreenId> for Screens {
            type Output = Screen;

            fn index(&self, id: ScreenId) -> &Self::Output {
                self.get(id)
            }
        }
    };
}

// TODO: make Msg have 0 and 2+ arguments
#[macro_export]
macro_rules! relm_thread {
    {
        relm: $relm:expr,
        name: $name:expr,
        $(msg: $msg:ident,)?
        result: $result:block $(,)?
    } => {
        {
            let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
            
            ::std::thread::Builder::new()
                .name($name.into())
                .spawn(move || {
                    let data = $result;

                    sender.send(data).expect("Failed to send data to main thread");
                })
                .expect("msg");

            let stream = $relm.stream().clone();
            receiver.attach(None, move |data| {
                $(stream.emit($msg(data));)?

                glib::Continue(false)
            });
        }
    };
}

#[macro_export]
macro_rules! option_error {
    ($msg:expr) => {
        || {
            log::error!($msg);

            std::process::exit(1);
        }
    };
}

#[macro_export]
macro_rules! result_error {
    ($msg:expr) => {
        |e| {
            log::error!("{}: {e}", $msg);

            std::process::exit(1);
        }
    };
}

#[macro_export]
macro_rules! include_gresource {
    ($res:expr) => {
        gio::resources_register(
            &gio::Resource::from_data(
                &glib::Bytes::from_static(include_bytes!($res))
            ).unwrap()
        );
    };
}

#[macro_export]
macro_rules! apply_css {
    (resource: $res:expr) => {
        {
            use gtk::prelude::CssProviderExt as _;

            let provider = gtk::CssProvider::new();
            provider.load_from_resource($res);

            gtk::StyleContext::add_provider_for_screen(&gdk::Screen::default().unwrap(), &provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
        }
    };

    (data: $data:expr) => {
        {
            use gtk::prelude::CssProviderExt as _;

            let provider = gtk::CssProvider::new();
            provider.load_from_data($data).unwrap_or_else(
                $crate::result_error!("Failed to load CSS data")
            );

            gtk::StyleContext::add_provider_for_screen(&gdk::Screen::default().unwrap(), &provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
        }
    };
}