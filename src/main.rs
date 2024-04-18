use clap::CommandFactory;
use clap::Parser;
use num_bigint::BigUint;
use num_bigint::ToBigUint;
use num_traits::Zero;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Perform Kaprekar's routine on a number.
    #[arg(short, long, default_value = "0")]
    number: BigUint,
    
    /// Perform Kaprekar's routine starting from a number.
    #[arg(short, long, default_value = "0")]
    start: BigUint,
    
    /// Perform Kaprekar's routine up to a number.
    #[arg(short, long, default_value = "0")]
    end: BigUint,
    
    /// Perform Kaprekar's routine on all numbers.
    #[arg(short, long)]
    all: bool,
    
    /// Number of iterations to perform.
    #[arg(short, long, default_value = "20")]
    iterations: u16,
    
    /// Empty out non-series and non-constant vectors.
    /// https://kaprekar.sourceforge.net/output/sample.php
    #[arg(short, long, default_value = "false")]
    truncate: bool,
    
    /// Output the results to a csv file.
    #[arg(short, long)]
    output: Option<PathBuf>,
    
    /// Continue adding results to an existing csv file.
    #[arg(short, long)]
    cont: Option<PathBuf>,

    /// Print the Kaprekar routine.
    #[arg(short, long)]
    verbose: bool,
    
    /// Create a symlink in /usr/local/bin.
    #[arg(long)]
    symlink: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Clap
    let args = Args::parse();
    let mut cmd = Args::command();
    // Buffer Writer
    let mut wtr: Option<BufWriter::<File>> = None;
    // Kaprekar on a single number
    if args.number != Zero::zero() {
        let result = kaprekar(args.number, args.verbose, args.iterations, args.truncate);
        println!("{:?}", result);
    // Kaprekar on all numbers
    } else if args.all {
        let mut number: BigUint = Zero::zero();
        if let Some(output_path) = args.output {
            match File::create(&output_path) {
                Ok(file) => {
                    wtr = Some(BufWriter::new(file));
                    // Now you can use wtr to write your CSV data
                },
                Err(e) => eprintln!("Failed to create file: {}", e),
            }
        } else if let Some(cont_path) = args.cont {
            match File::open(&cont_path) {
                Ok(file) => {
                    let rdr = BufReader::new(file);
                    for line in rdr.lines() {
                        let line = line?;
                        let mut parts = line.split(",");
                        let n = parts.next().unwrap().parse::<BigUint>().unwrap();
                        number = n;
                    }
                },
                Err(e) => eprintln!("Failed to open file: {}", e),
            }
            match OpenOptions::new().append(true).create(true).open(&cont_path) {
                Ok(file) => {
                    wtr = Some(BufWriter::new(file));
                    // Now you can use wtr to write your CSV data
                },
                Err(e) => eprintln!("Failed to open file: {}", e),
            }
        }
        loop {
            let result = kaprekar(number.clone(), args.verbose, args.iterations, args.truncate);
            println!("{}\t{:?}", number, result);
            if let Some(ref mut writer) = wtr {
                let mut series = result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
                let current_columns = series.len();
                if current_columns < args.iterations as usize {
                    series.push_str(&",".repeat(args.iterations as usize - current_columns));
                }
                writer.write_all(format!("{},{}\n", number, series).as_bytes())?;
                writer.flush()?;
            }
            number += 1.to_biguint().unwrap();
        }
    } else if args.start != Zero::zero() && args.end != Zero::zero() {
        let mut number = args.start.clone();
        if let Some(output_path) = args.output {
            match File::create(&output_path) {
                Ok(file) => {
                    wtr = Some(BufWriter::new(file));
                    // Now you can use wtr to write your CSV data
                },
                Err(e) => eprintln!("Failed to create file: {}", e),
            }
        } else if let Some(cont_path) = args.cont {
            match File::open(&cont_path) {
                Ok(file) => {
                    let rdr = BufReader::new(file);
                    for line in rdr.lines() {
                        let line = line?;
                        let mut parts = line.split(",");
                        let n = parts.next().unwrap().parse::<BigUint>().unwrap();
                        number = n;
                    }
                },
                Err(e) => eprintln!("Failed to open file: {}", e),
            }
            match OpenOptions::new().append(true).create(true).open(&cont_path) {
                Ok(file) => {
                    wtr = Some(BufWriter::new(file));
                    // Now you can use wtr to write your CSV data
                },
                Err(e) => eprintln!("Failed to open file: {}", e),
            }
        }
        while number <= args.end {
            let result = kaprekar(number.clone(), args.verbose, args.iterations, args.truncate);
            println!("{}\t{:?}", number, result);
            if let Some(ref mut writer) = wtr {
                let mut series = result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
                let current_columns = series.len();
                if current_columns < args.iterations as usize {
                    series.push_str(&",".repeat(args.iterations as usize - current_columns));
                }
                writer.write_all(format!("{},{}\n", number, series).as_bytes())?;
                writer.flush()?;
            }
            number += 1.to_biguint().unwrap();
        }
    } else if args.start != Zero::zero() {
        let mut number = args.start.clone();
        if let Some(output_path) = args.output {
            match File::create(&output_path) {
                Ok(file) => {
                    wtr = Some(BufWriter::new(file));
                    // Now you can use wtr to write your CSV data
                },
                Err(e) => eprintln!("Failed to create file: {}", e),
            }
        } else if let Some(cont_path) = args.cont {
            match File::open(&cont_path) {
                Ok(file) => {
                    let rdr = BufReader::new(file);
                    for line in rdr.lines() {
                        let line = line?;
                        let mut parts = line.split(",");
                        let n = parts.next().unwrap().parse::<BigUint>().unwrap();
                        number = n;
                    }
                },
                Err(e) => eprintln!("Failed to open file: {}", e),
            }
            match OpenOptions::new().append(true).create(true).open(&cont_path) {
                Ok(file) => {
                    wtr = Some(BufWriter::new(file));
                    // Now you can use wtr to write your CSV data
                },
                Err(e) => eprintln!("Failed to open file: {}", e),
            }
        }
        loop {
            let result = kaprekar(number.clone(), args.verbose, args.iterations, args.truncate);
            println!("{}\t{:?}", number, result);
            if let Some(ref mut writer) = wtr {
                let mut series = result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
                let current_columns = series.len();
                if current_columns < args.iterations as usize {
                    series.push_str(&",".repeat(args.iterations as usize - current_columns));
                }
                writer.write_all(format!("{},{}\n", number, series).as_bytes())?;
                writer.flush()?;
            }
            number += 1.to_biguint().unwrap();
        }
    } else if args.end != Zero::zero() {
        let mut number: BigUint = Zero::zero();
        if let Some(output_path) = args.output {
            match File::create(&output_path) {
                Ok(file) => {
                    wtr = Some(BufWriter::new(file));
                    // Now you can use wtr to write your CSV data
                },
                Err(e) => eprintln!("Failed to create file: {}", e),
            }
        } else if let Some(cont_path) = args.cont {
            match File::open(&cont_path) {
                Ok(file) => {
                    let rdr = BufReader::new(file);
                    for line in rdr.lines() {
                        let line = line?;
                        let mut parts = line.split(",");
                        let n = parts.next().unwrap().parse::<BigUint>().unwrap();
                        number = n;
                    }
                },
                Err(e) => eprintln!("Failed to open file: {}", e),
            }
            match OpenOptions::new().append(true).create(true).open(&cont_path) {
                Ok(file) => {
                    wtr = Some(BufWriter::new(file));
                    // Now you can use wtr to write your CSV data
                },
                Err(e) => eprintln!("Failed to open file: {}", e),
            }
        }
        while number <= args.end {
            let result = kaprekar(number.clone(), args.verbose, args.iterations, args.truncate);
            println!("{}\t{:?}", number, result);
            if let Some(ref mut writer) = wtr {
                let mut series = result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
                let current_columns = series.len();
                if current_columns < args.iterations as usize {
                    series.push_str(&",".repeat(args.iterations as usize - current_columns));
                }
                writer.write_all(format!("{},{}\n", number, series).as_bytes())?;
                writer.flush()?;
            }
            number += 1.to_biguint().unwrap();
        }
    } else if args.symlink {
        let path = std::env::current_exe().unwrap();
        let symlink = std::path::Path::new("/usr/local/bin/kaprekar");
        std::fs::create_dir_all(symlink.parent().unwrap()).unwrap();
        std::os::unix::fs::symlink(path, symlink).unwrap();
    } else {
        cmd.print_help().unwrap();
    }
    return Ok(());
}

fn kaprekar(mut num: BigUint, verbose: bool, iterations: u16, truncate: bool) -> Vec<BigUint> {
    let mut results = Vec::new();
    for _ in 0..iterations { // Loop up to 19 times (the first number is already added)
        let mut digits = num.to_string().chars().collect::<Vec<char>>();
        digits.sort();
        let asc = digits.iter().collect::<String>().parse::<BigUint>().unwrap();
        digits.sort_by(|a, b| b.cmp(a));
        let desc = digits.iter().collect::<String>().parse::<BigUint>().unwrap();
        num = &desc - &asc;
        if verbose {
            println!("{} - {} = {}", desc, asc, num);
        }
        // Check for a cycle
        if results.len() > 0 && results[0] == num {
            return results;
        } 
        results.push(num.clone());
        if num == 6174.to_biguint().unwrap() || num == 495.to_biguint().unwrap() || num == Zero::zero() {
            return last_element(&results);
        }
    }
    if truncate {
        return vec![];
    }
    results // Return the vector after 20 iterations or when a cycle is detected
}

fn last_element<T: Clone>(vec: &[T]) -> Vec<T> {
    vec.last().cloned().map_or_else(Vec::new, |item| vec![item])
}
