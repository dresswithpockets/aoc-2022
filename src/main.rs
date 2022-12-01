mod day1;

use std::env;
use std::io;

fn print_usage(cmd: &String) {
    println!("{} (day) (variant)", cmd);
}

fn run_all() -> io::Result<()> {
    day1::run()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        print_usage(&args[0]);
        println!("Defaulting to run_all\n");
        return run_all();
    }

    let day = &args[1];
    let variant = &args[2];

    let result = match &day[..] {
        "day1" => match &variant[..] {
            "functional" => day1_functional(),
            _ => day1_progscan(),
        },
        _ => Err(io::Error::new(io::ErrorKind::Other, "oh no!")),
    };

    if let Err(err) = result {
        println!("{}", err);
        print_usage(&args[0]);
    }

    Ok(())
}
