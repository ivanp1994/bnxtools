use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
//use std::path::Path;

#[derive(Debug)]
struct Molecule {
    backbone: String,
    labels_basepair: String,
    labels_snr: String,
    labels_int: String,
}

fn filter_molecule(molecule: &Molecule, min_length: f64, min_labels: i32) -> bool {
    let parts: Vec<&str> = molecule.backbone.split('\t').collect();
    let length: f64 = parts[2].parse().unwrap_or(0.0);
    let num_labels: i32 = parts[5].parse().unwrap_or(0);
    length >= min_length && num_labels >= min_labels
}

fn write_molecule<W: Write>(writer: &mut W, molecule: &Molecule) -> io::Result<()> {
    writeln!(writer, "{}", molecule.backbone)?;
    writeln!(writer, "{}", molecule.labels_basepair)?;
    writeln!(writer, "{}", molecule.labels_snr)?;
    writeln!(writer, "{}", molecule.labels_int)?;
    Ok(())
}

fn process_bnx_file(
    input_path: &str,
    output_path: Option<&String>,
    min_length: f64,
    min_labels: i32,
) -> io::Result<()> {
    // Open the input file
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);

    // Create or use stdout for the output file
    let output: Box<dyn Write> = if let Some(output_path) = output_path {
        Box::new(File::create(output_path)?)
    } else {
        Box::new(io::stdout())
    };

    let mut writer = io::BufWriter::new(output);

    let mut current_molecule = Molecule {
        backbone: String::new(),
        labels_basepair: String::new(),
        labels_snr: String::new(),
        labels_int: String::new(),
    };

    for line_result in reader.lines() {
        let line = line_result?;
        if line.starts_with('#') {
            writeln!(writer, "{}", line)?;
        } else if line.starts_with('0') {
            // If there is a current molecule, check if it passes the filter and write it
            if !current_molecule.backbone.is_empty() {
                if filter_molecule(&current_molecule, min_length, min_labels) {
                    write_molecule(&mut writer, &current_molecule)?;
                }
            }
            // Start a new molecule
            current_molecule = Molecule {
                backbone: line,
                labels_basepair: String::new(),
                labels_snr: String::new(),
                labels_int: String::new(),
            };
        } else if line.starts_with('1') {
            current_molecule.labels_basepair = line;
        } else if line.starts_with("QX11") {
            current_molecule.labels_snr = line;
        } else if line.starts_with("QX12") {
            current_molecule.labels_int = line;
        }
    }

    // Process the last molecule
    if !current_molecule.backbone.is_empty() {
        if filter_molecule(&current_molecule, min_length, min_labels) {
            write_molecule(&mut writer, &current_molecule)?;
        }
    }

    Ok(())
}


fn mainr() -> io::Result<()> {

    
    let matches = Command::new("BNX Filter")
        .version("1.0")
        .about("Filters BNX files based on molecule length and number of labels")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .num_args(1)
            .required(true)
            .help("Input BNX file"))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .num_args(1)
            .help("Output BNX file, if not provided, output will be written to stdout"))
        .arg(Arg::new("length")
            .long("length")
            .num_args(1)
            .required(true)
            .help("Minimum molecule length"))
        .arg(Arg::new("labels")
            .long("labels")
            .num_args(1)
            .required(true)
            .help("Minimum number of labels"))
        .get_matches();

    let input_path = matches.get_one::<String>("input").unwrap();
    let output_path = matches.get_one::<String>("output");
    let min_length: f64 = matches.get_one::<String>("length").unwrap().parse().expect("Please provide a valid number for length");
    let min_labels: i32 = matches.get_one::<String>("labels").unwrap().parse().expect("Please provide a valid number for labels");
    


    process_bnx_file(input_path, output_path, min_length, min_labels)
}

fn process_bnx_file_with_constants() -> io::Result<()> {
    let input_path = r"C:\Users\Ivan\Desktop\RUST\RustProject\newfile_big.bnx";
    let min_length: f64 = 1000000.0;  // Example minimum length
    let min_labels: i32 = 15;      // Example minimum number of labels
    //let output_path: Option<&String> = None;

    let binding = String::from(r"C:\Users\Ivan\Desktop\RUST\RustProject\newfile_big_filtered.bnx");
    let output_path: Option<&String> = Some(&binding);
    

    process_bnx_file(input_path, output_path, min_length, min_labels)
}

fn main()-> io::Result<()>{
    process_bnx_file_with_constants()
}
