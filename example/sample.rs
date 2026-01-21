use rdspy::RdsGroupIterator;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let reader: Box<dyn BufRead> = if args.len() == 1 {
        println!("Reading RDS groups from stdin...");
        Box::new(BufReader::new(io::stdin()))
    } else if args.len() == 2 {
        let path = &args[1];
        println!("Reading RDS groups from: {}", path);
        Box::new(BufReader::new(File::open(path)?))
    } else {
        eprintln!("Usage: {} [path_to_rds_file]", args[0]);
        eprintln!("(omit path to read from stdin)");
        std::process::exit(1);
    };

    for group_result in RdsGroupIterator::new(reader) {
        match group_result {
            Ok(group) => {
                println!(
                    "A:0x{:04X} B:0x{:04X} C:0x{:04X} D:0x{:04X}",
                    group.a, group.b, group.c, group.d
                );
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}
