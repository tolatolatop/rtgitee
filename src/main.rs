use clap::{App, Arg};

fn main() {
    // Create a new instance of the command-line application
    let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        // Define the command-line arguments
        .arg(
            Arg::with_name("input")
                .help("Input file")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .help("Output file")
                .index(2)
                .required(true),
        );

    // Parse the command-line arguments
    let matches = app.get_matches();

    // Access the values of the arguments
    let input_file = matches.value_of("input").unwrap();
    let output_file = matches.value_of("output").unwrap();

    // Your application logic goes here

    println!("Input file: {}", input_file);
    println!("Output file: {}", output_file);
}