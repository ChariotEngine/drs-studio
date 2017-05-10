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
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("file-name")
                .long("file-name")
                .help("Name of the file to extract (example: 50500.bin)")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .help("Output filepath")
                .takes_value(true)))
        .get_matches();

        match matches.subcommand() {
            ("extract", Some(args)) => extract_files(args),
            _ => unreachable!(),
        }
}

fn extract_files(args: &clap::ArgMatches) {
    let drs_name = args.value_of("drs").unwrap();
    let file_name = args.value_of("file-name").unwrap();

    let (file_id, table_type) = {
        let mut split = file_name.split('.');
        let stem = split.next().expect(&format!("Failed to extract file stem from '{}'", file_name));
        let ext = split.next().expect(&format!("Failed to extract file extension from '{}'", file_name));

        let table_type = match &*ext[..].to_lowercase() {
            "slp" => drs::DrsFileType::Slp,
            "shp" => drs::DrsFileType::Shp,
            "bin" => drs::DrsFileType::Binary,
            "wav" => drs::DrsFileType::Wav,
            _ => panic!("Invalid file-name! Extension expected to be one of: 'slp', 'shp', 'bin', or 'wav'"),
        };

        let file_id: u32 = stem.parse().expect(&format!("Failed to parse '{}' into a u32", stem));

        (file_id, table_type)
    };

    let archive = drs::DrsFile::read_from_file(&drs_name)
        .expect(&format!("Failed to load {}", &drs_name));

    let table = archive.find_table(table_type)
        .expect(&format!("Failed to find {:?} table in {}", &table_type, &drs_name));

    let contents = table.find_file_contents(file_id)
        .expect(&format!("Failed to find {:?} with id {} in {}", &table_type, &file_id, &drs_name));

    let output_path = args.value_of("output").unwrap_or(file_name);
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(output_path)
        .expect(&format!("Failed to setup file {}", &output_path));

    f.write(&contents).expect(&format!("Failed to write to {}", &output_path));
    println!("Wrote {} bytes to {}", contents.len(), output_path);
}

