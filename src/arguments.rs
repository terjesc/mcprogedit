extern crate clap;

pub fn matches() -> clap::ArgMatches<'static> {
    clap::App::new("mcprogedit - Programmatically Edit MineCraft savefiles.")
        .version(clap::crate_version!())
        .author("Terje Schjelderup <terjesc@pvv.org>")
        .arg(
            clap::Arg::with_name("input_save")
                .short("-i")
                .long("input-directory")
                .value_name("DIRECTORY")
                .help("Input savefile directory")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("output_save")
                .short("-o")
                .long("output-directory")
                .value_name("DIRECTORY")
                .help("Output savefile directory")
                .takes_value(true),
        )
        .get_matches()
}
