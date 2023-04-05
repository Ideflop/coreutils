// * This file is part of the uutils coreutils package.
// *
// * (c) 2014 T. Jameson Little <t.jameson.little@gmail.com>
// * (c) 2020 nicoo <nicoo@debian.org>
// *
// * For the full copyright and license information, please view the LICENSE file
// * that was distributed with this source code.

use std::error::Error;
use std::fmt::Write as FmtWrite;
use std::io::BufRead;
use std::io::{self, stdin, stdout, Write};

mod factor;
use clap::{crate_version, Arg, ArgAction, Command};
pub use factor::*;
use uucore::display::Quotable;
use uucore::error::UResult;
use uucore::{format_usage, help_about, help_usage, show_error, show_warning};

mod miller_rabin;
pub mod numeric;
mod rho;
pub mod table;

const ABOUT: &str = help_about!("factor.md");
const USAGE: &str = help_usage!("factor.md");

mod options {
    pub static EXPONENTS: &str = "exponents";
    pub static HELP: &str = "help";
    pub static NUMBER: &str = "NUMBER";
}

mod exponent_options {

    pub static mut PRINT_EXPONENTS: bool = false;

    // Safety
    //
    // This function contains unsafe code that modifies the static mutable variable
    // PRINT_EXPONENTS.
    // This function is called ones inside uumain if the -h or --exponents flag
    // is found.
    // If print_exponents() is called and PRINT_EXPONENTS is true then the p^e
    // output format will be used.
    pub fn print_exponents() {
        unsafe {
            PRINT_EXPONENTS = true;
        }
    }
}

fn print_factors_str(
    num_str: &str,
    w: &mut io::BufWriter<impl io::Write>,
    factors_buffer: &mut String,
) -> Result<(), Box<dyn Error>> {
    num_str
        .trim()
        .parse::<u64>()
        .map_err(|e| e.into())
        .and_then(|x| {
            factors_buffer.clear();
            writeln!(factors_buffer, "{}:{}", x, factor(x))?;
            w.write_all(factors_buffer.as_bytes())?;
            Ok(())
        })
}

#[uucore::main]
pub fn uumain(args: impl uucore::Args) -> UResult<()> {
    let matches = uu_app().try_get_matches_from(args)?;

    if matches.get_flag(options::EXPONENTS) {
        exponent_options::print_exponents();
    };

    let stdout = stdout();
    // We use a smaller buffer here to pass a gnu test. 4KiB appears to be the default pipe size for bash.
    let mut w = io::BufWriter::with_capacity(4 * 1024, stdout.lock());
    let mut factors_buffer = String::new();

    if let Some(values) = matches.get_many::<String>(options::NUMBER) {
        for number in values {
            if let Err(e) = print_factors_str(number, &mut w, &mut factors_buffer) {
                show_warning!("{}: {}", number.maybe_quote(), e);
            }
        }
    } else {
        let stdin = stdin();
        let lines = stdin.lock().lines();
        for line in lines {
            for number in line.unwrap().split_whitespace() {
                if let Err(e) = print_factors_str(number, &mut w, &mut factors_buffer) {
                    show_warning!("{}: {}", number.maybe_quote(), e);
                }
            }
        }
    }

    if let Err(e) = w.flush() {
        show_error!("{}", e);
    }

    Ok(())
}

pub fn uu_app() -> Command {
    Command::new(uucore::util_name())
        .version(crate_version!())
        .about(ABOUT)
        .override_usage(format_usage(USAGE))
        .infer_long_args(true)
        .disable_help_flag(true)
        .arg(Arg::new(options::NUMBER).action(ArgAction::Append))
        .arg(
            Arg::new(options::EXPONENTS)
                .short('h')
                .long(options::EXPONENTS)
                .help("Print factors in the form p^e")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::HELP)
                .long(options::HELP)
                .help("Print help information.")
                .action(ArgAction::Help),
        )
}
