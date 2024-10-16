use clap_allgen::{render_shell_completions, render_manpages};

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

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 2), 4);
    }

    #[test]
    fn test_completions() {
        assert_eq!("TODO", "TODO");
    }

    fn test_manpages() {
        assert_eq!("TODO", "TODO");
    }
}
