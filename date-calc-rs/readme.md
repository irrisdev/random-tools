# Date Calculation Tool

A simple command-line application written in Rust that calculates a future date based on the number of days added to the current date.

## Features

- Calculate the date a specified number of days from today.
- Option to display only the date without any additional text.

## Installation

To build and run this application, you need to have Rust and Cargo installed on your machine. If you haven't installed Rust yet, you can do so by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can clone this repository and build the project:

```bash
git clone <repository-url>
cd <repository-name>
cargo build --release
```

## Usage

You can run the application using the following command:

```bash
./target/release/date-calc-rs <DAYS>
```

## Options

-   ```-d``` or ```--date-only```: (Optional) Display only the date

## Example

To calculate the date 10 days from now:
```bash 
./target/release/date-calc-rs 10
```

Output:
```
In 10 days the date will be 11-11-2024
```

Or with the date-only flag:
```bash 
./target/release/date-calc-rs --date-only 10
```

Output:
```
11-11-2024
```

## Dependencies

This project uses the following dependencies:
-   [chrono](https://docs.rs/chrono/latest/chrono/) - For date and time manipulation
  - [clap](https://docs.rs/clap/latest/clap/) - For parsing command-line arguments

## License

This project is licensed under the MIT license