#[macro_use] extern crate log;

use std::{env, str};
use std::fs::File;
use std::io::{self, BufRead};
use trust_dns_resolver::Resolver;
use psl::{List, Psl};

fn main() -> io::Result<()> {
    env_logger::init();
    // Get the input file from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        return Ok(());
    }

    // Open the domains file and create a buffered reader
    let input_file = &args[1];
    let file = File::open(input_file)?;
    let reader = io::BufReader::new(file);

    // Set up DNS resolver
    let resolver = Resolver::from_system_conf().expect("Failed to create DNS resolver");

    let mut count = 0;

    // Process each line in the file
    for line in reader.lines() {
        if let Some(domain) = List.domain(line?.as_bytes()) {
            let root_domain = str::from_utf8(domain.suffix().as_bytes()).unwrap().to_string();
            info!("{}", root_domain);

            match resolver.ns_lookup(root_domain.clone()) {
                Ok(response) => {
                    let mut found = false;
                    for record in response.iter() {
                        let ns = record.to_string();
                        if ns.contains("cyberdns") {
                            println!("{}", root_domain);
                            count += 1;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        eprintln!("No 'cyberdns' found in NS for {}", root_domain);
                    }
                }
                Err(e) => eprintln!("Failed to lookup NS for {}: {}", root_domain, e),
            }
        }
        else { continue; }
        // let suffix = str::from_utf8(domain.suffix().as_bytes()).unwrap().to_string();

    }

    println!("Total domains with 'cyberdns' in NS: {}", count);

    Ok(())
}

