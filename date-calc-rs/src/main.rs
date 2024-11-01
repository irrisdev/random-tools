use chrono::{Duration, Utc};
use clap::{Arg, Command};

fn main() {
    let date_only_arg = Arg::new("date_only")
        .long("date-only")
        .short('d')
        .help("Display only the date")
        .num_args(0);

    let days_arg = Arg::new("days")
        .help("The number of days to add to the current date")
        .value_name("DAYS")
        .required(true)
        .num_args(1);

    let app = Command::new("addate")
        .version("0.1")
        .about("Given a number of days, calculates the date from today")
        .arg(date_only_arg)
        .arg(days_arg)
        .get_matches();

    let days: i64 = app.get_one::<String>("days")
        .expect("Days is a required argument")
        .parse()
        .expect("Please provide a valid number.");

    let current_date = Utc::now();
    let future_date = current_date + Duration::days(days);

    let date_only = app.get_flag("date_only");

    if date_only {
        println!("{}", future_date.format("%d-%m-%Y"));
    } else {
        println!("In {} day{} the date will be {}",
                 days,
                 {if days == 1 { "" } else { "s" }},
                 future_date.format("%d-%m-%Y"));
    }
}
