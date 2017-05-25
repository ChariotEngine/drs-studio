extern crate chariot_drs as drs;

use std::io::Write;

extern crate clap;
use clap::{App, Arg, AppSettings, SubCommand};

fn main() {
    let matches = App::new("drs-studio")
        .version("0.1.0")
        .author("Taryn Hill <taryn@phrohdoh.com>")
        .about("A command-line utility for managing DRS archives (currently only extracts files)")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("extract")
            .arg(Arg::with_name("drs")
                .long("drs-path")
                .value_name("drs")
                .help("The path to the DRS you will extract from")
                .required(true))
            .arg(Arg::with_name("file-names")
                .long("file-names")
                .help("Set of filenames to extract (example: `--file-names 50500.bin 00412.slp`)")
                .multiple(true)
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("output-dir")
                .short("o")
                .long("output-dir")
                .help("Directory to write extracted files to")
                .default_value(".")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("list")
            .arg(Arg::with_name("drs")
                .long("drs-path")
                .value_name("drs")
                .help("The path to the DRS to list the contents of")
                .required(true)))
        .get_matches();

        match matches.subcommand() {
            ("extract", Some(args)) => cmd_extract(args),
            ("list", Some(args)) => cmd_list(args),
            _ => unreachable!(),
        }
}

fn cmd_extract(args: &clap::ArgMatches) {
    let drs_name = args.value_of("drs").unwrap();
    let archive = drs::DrsFile::read_from_file(&drs_name)
        .expect(&format!("Failed to load {}", &drs_name));

    for file_name in args.values_of("file-names").unwrap() {
        let (file_id, file_type) = {
            let mut split = file_name.split('.');
            let stem = split.next().expect(&format!("Failed to extract file stem from '{}'", file_name));
            let ext = split.next().expect(&format!("Failed to extract file extension from '{}'", file_name));

            let file_type = match &*ext[..].to_lowercase() {
                "slp" => drs::DrsFileType::Slp,
                "shp" => drs::DrsFileType::Shp,
                "bin" => drs::DrsFileType::Binary,
                "wav" => drs::DrsFileType::Wav,
                _ => panic!("Invalid file-name! Extension expected to be one of: 'slp', 'shp', 'bin', or 'wav'"),
            };

            let file_id: u32 = stem.parse().expect(&format!("Failed to parse '{}' into a u32", stem));

            (file_id, file_type)
        };

        let table = archive.find_table(file_type)
            .expect(&format!("Failed to find {:?} table in {}", &file_type, &drs_name));

        let contents = table.find_file_contents(file_id)
            .expect(&format!("Failed to find {:?} with id {} in {}", &file_type, &file_id, &drs_name));

        let output_dir = args.value_of("output-dir").unwrap();
        let output_path = std::path::Path::new(&output_dir).join(file_name);

        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&output_path)
            .expect(&format!("Failed to setup file {}", &output_path.display()));

        f.write(&contents).expect(&format!("Failed to write to {}", &output_path.display()));
        println!("Wrote {} bytes to {}", contents.len(), output_path.display());
    }
}

fn cmd_list(args: &clap::ArgMatches) {
    let drs_name = args.value_of("drs").unwrap();
    let archive = drs::DrsFile::read_from_file(&drs_name)
        .expect(&format!("Failed to load {}", &drs_name));

    for table in &archive.tables {
        let ext = table.header.file_extension();
        for entry in &table.entries {
            println!("{}.{}", entry.file_id, ext);
        }
    }
}