use clap_allgen::{render_shell_completions, render_manpages};
use snafu::{prelude::*, Whatever};

#[derive(Debug, clap::Parser)]
enum Commands {
    First,
    Second,
    Third,
}

/// Multiplies two integers
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn generate_completions() {
    render_shell_completions::<Commands>("~/.config/fish/completions").expect("shell completions generation failed");
}

pub fn generate_manpages() {
    render_manpages::<Commands>("~/.local/share/man/man1").expect("man page generation failed");
}

#[cfg(test)]
mod test {
    use super::*;

    fn is_valid_result(outcome: u32, expected: u32) -> Result<(), Whatever> {
        if outcome != expected {
            whatever!(fmt!("Result is not valid: {} != {}", outcome, expected));
        }
        Ok(())
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 2), 4);

        // Snapshots tests (also sometimes called approval tests) are tests that
        // assert values against a reference value (the snapshot).
        insta::assert_snapshot!(multiply(3, 5));

        // Snafu error messages and backtracing
        if let Err(e) = is_valid_number(multiply(3, 5), 15) {
            eprintln!("An error occurred: {e}");
            if let Some(bt) = ErrorCompat::backtrace(&e) {
                eprintln!("{bt}");
            }
        }
    }

    #[test]
    fn test_completions() {
        assert_eq!("TODO", "TODO");
    }

    fn test_manpages() {
        assert_eq!("TODO", "TODO");
    }
}
