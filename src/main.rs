use clap::{App, Arg};
mod lib;

fn main() {
    let matches = App::new("Boiler-CLI")
        .version("1.0")
        .author("Niemand. <chill.out.x19@gmail.com>")
        .about("Boilerplates Manager")
        .subcommand(
            App::new("new")
                .about("Generates a Boilerplate")
                .arg(
                    Arg::new("type")
                        .short('t')
                        .long("type")
                        .about("The Boilerplate Type(Express/React/Next)")
                        .possible_values(["express", "react", "react-ts", "next"])
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .about("The project name")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("where")
                        .short('w')
                        .long("where")
                        .about("Where the boilerplate is going to be located (defaults to .")
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("add")
                .about("Adds Features to the current boilerplate")
                .arg(
                    Arg::new("feature")
                        .short('w')
                        .long("what")
                        .multiple_values(true)
                        .about("What Feature to add")
                        .takes_value(true),
                ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("new") => println!("Generating a New Boilerplate"),
        Some("add") => println!("add boilerplates boilerplate"),
        None => println!("Please Enter a command"),
        _ => (),
    }

    if let Some(matches) = matches.subcommand_matches("new") {
        let boilerplate_type = matches.value_of("type").unwrap();
        let project_name = matches.value_of("name").unwrap();
        let boilerplate_location = matches.value_of("where").unwrap_or(project_name);

        lib::create_boilerplate(boilerplate_type, boilerplate_location, project_name);
    }
}
