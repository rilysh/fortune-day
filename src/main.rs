//! Enable unstable from_mins() from standard library
#![feature(duration_constructors)]

use clap::Parser;
use cli::{Cli, Commands};
use fortune::{get_rand_fortune, get_rand_fortune_from_all, search_fortune, search_fortune_regex};
use std::{process, thread, time};
use termsize::get_termsize;

mod cli;
mod colors;
mod fortune;
mod randnum;
mod termsize;

fn main() {
    let args = Cli::parse();

    // Match with command line arguments
    match args.command {
        Some(Commands::Search(search)) => {
            if search.regex {
                if search.colors {
                    eprintln!("whoops - coloring the regex matches isn't supported!");
                } else {
                    search_fortune_regex(&search.query, search.insensitive, search.single);
                }
            } else {
                search_fortune(
                    &search.query,
                    search.insensitive,
                    search.colors,
                    search.single,
                );
            }
        }

        Some(Commands::Day(mut day)) => {
            let multiple_quotes = day.howmany > 1;
            let col_size = get_termsize().1 - 10;

            if multiple_quotes {
                println!("{}", ".".repeat(col_size as usize));
            }

            while day.howmany > 0 {
                if let Ok(quote) = get_rand_fortune_from_all() {
                    println!("{}", quote);

                    // Only print the separator if day needed to be invoked
                    // multiple times.
                    if day.howmany > 1 {
                        println!("{}", ".".repeat(col_size as usize));
                    }
                } else {
                    process::exit(1);
                }
                day.howmany -= 1;
            }

            if multiple_quotes {
                println!("{}", ".".repeat(col_size as usize));
            }
        }

        Some(Commands::Fortune(mut fortune)) => {
            let multiple_quotes = fortune.howmany > 1;
            // Adjust the size
            let col_size = get_termsize().1 - 10;

            if multiple_quotes {
                println!("{}", ".".repeat(col_size as usize));
            }
            while fortune.howmany > 0 {
                println!("{}", get_rand_fortune());
                if fortune.howmany > 1 {
                    println!("{}", ".".repeat(col_size as usize));
                }
                fortune.howmany -= 1;
            }

            if multiple_quotes {
                println!("{}", ".".repeat(col_size as usize));
            }
        }

        Some(Commands::Wait(wait)) => {
            print!("\x1b[H\x1b[2J\x1b[3J");

            let mut dur = time::Duration::from_secs(5);
            if wait.days > 0 {
                dur = time::Duration::from_days(wait.days);
            } else if wait.hours > 0 {
                dur = time::Duration::from_hours(wait.hours);
            } else if wait.mins > 0 {
                dur = time::Duration::from_mins(wait.mins);
            } else if wait.secs > 0 {
                dur = time::Duration::from_secs(wait.secs);
            }

            loop {
                let fortune = get_rand_fortune();
                println!("{}", fortune);
                thread::sleep(dur);
                print!("\x1b[H\x1b[2J\x1b[3J");
            }
        }

        // If no argument is specified then print a random fortune and exit.
        None => println!("{}", get_rand_fortune()),
    }
}
