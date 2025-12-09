mod days;

use std::{
    fs::{self, read_to_string},
    path::Path,
};

use anyhow::{bail, Context, Result};
use chrono::{Datelike, Local};
use clap::{Parser, Subcommand};
use days::*;
use reqwest::{blocking::Client, header::COOKIE};

const YEAR: i32 = 2025;
const LAST_DAY: i64 = 12;

#[derive(Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run your solution (specify the day or leave blank to run today,)
    Run {
        #[arg(
            value_parser = clap::value_parser!(u32).range(1..={LAST_DAY}),
            help = "The day you want to run (or leave blank to run today)")
        ]
        day: Option<u32>,
    },
    /// Download the input from adventofcode.org (specify the day or leave blank to fetch today)
    Fetch {
        #[arg(
            value_parser = clap::value_parser!(u32).range(1..=LAST_DAY),
            help = "The the day you want to fetch the input for (or leave blank to fetch today)")
        ]
        day: Option<u32>,
    },
}

fn today() -> Result<u32> {
    match Local::now() {
        date if date.year() == YEAR
            && date.month() == 12
            && date.day() >= 1
            && i64::from(date.day()) <= LAST_DAY =>
        {
            Ok(date.day())
        }
        date => bail!(
            "{}-{}-{} is not a valid AoC {} day!",
            date.year(),
            date.month(),
            date.day(),
            YEAR
        ),
    }
}

fn run_day(day: &Option<u32>) -> Result<()> {
    let day = match day {
        Some(day) => *day,
        None => today().context(
            "Failed to get/parse today's date, please call the program again and specify a day",
        )?,
    };

    if !Path::new(&format!("input/day{day:02}.txt")).exists() {
        println!("No input found, attempting to fetch from AoC website.");
        fetch_day(&Some(day))
            .context("Failed to fetch input. Did you put your session token in a .token file?")?;
        println!();
    }

    match day {
        1 => day01::Day01::run(),
        2 => day02::Day02::run(),
        3 => day03::Day03::run(),
        4 => day04::Day04::run(),
        5 => day05::Day05::run(),
        6 => day06::Day06::run(),
        7 => day07::Day07::run(),
        8 => day08::Day08::run(),
        9 => day09::Day09::run(),
        10 => day10::Day10::run(),
        11 => day11::Day11::run(),
        12 => day12::Day12::run(),
        day => bail!("The Advent of Code {{YEAR}} doesn't have a day {day}"),
    }

    Ok(())
}

fn fetch_day(day: &Option<u32>) -> Result<()> {
    let day = match day {
        Some(day) => *day,
        None => match today() {
            Ok(day) => day,
            Err(e) => bail!("Can't get current day: {e}"),
        },
    };

    let token = read_to_string(".token").context("Failed to read .token file")?;

    let client = Client::new();
    let response = client
        .get(format!("https://adventofcode.com/{YEAR}/day/{day}/input"))
        .header(COOKIE, format!("session={token};"))
        .send()?
        .error_for_status()
        .context("Couldn't retrieve input from AoC website. Did you put your session cookie in the .session file?")?;

    let input = response
        .text()
        .context("Failed to parse the website's response")?;
    let path = format!("input/day{day:02}.txt");
    fs::write(&path, input).context("Couldn't write input to file")?;

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run { day }) => match run_day(day) {
            Ok(()) => (),
            Err(e) => println!("Couldn't run day: {e}"),
        },
        Some(Commands::Fetch { day }) => match fetch_day(day) {
            Ok(()) => (),
            Err(e) => println!("Couldn't fetch day: {e}"),
        },
        None => match run_day(&None) {
            Ok(()) => (),
            Err(e) => println!("Couldn't run today: {e}"),
        },
    }
}
