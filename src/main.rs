mod day1;
mod day2;
mod day3;
mod day4;

use std::env;
use std::io;

fn print_usage(cmd: &String) {
    println!("{} (day) (variant)", cmd);
}

fn run_all() -> io::Result<()> {
    day1::run()
        .and(day2::run())
        .and(day3::run())
        .and(day4::run())
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
            "functional" => day1::functional(),
            _ => day1::progscan(),
        },
        "day2" => match &variant[..] {
            _ => day2::functional(),
        },
        "day3" => match &variant[..] {
            "ugly" => day3::ugly(),
            _ => day3::realistic(),
        },
        _ => Err(io::Error::new(io::ErrorKind::Other, "oh no!")),
    };

    if let Err(err) = result {
        println!("{}", err);
        print_usage(&args[0]);
    }

    Ok(())
}
