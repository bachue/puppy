use clap::App;
use clap::Arg;
use version::version;

pub struct Flags {
    pub config_file_path: String,
}

pub fn parse_flags() -> Flags {
    let matcher = App::new("puppy")
        .version(version!())
        .author("Rong Zhou")
        .arg(
            Arg::with_name("config")
                .short("f")
                .long("config")
                .value_name("FILE")
                .empty_values(false)
                .required(true)
                .help("Specify the config file")
                .takes_value(true),
        )
        .get_matches();
    return Flags {
        config_file_path: matcher
            .value_of("config")
            .expect("--config is required")
            .to_string(),
    };
}
