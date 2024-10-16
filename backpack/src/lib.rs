use clap_allgen::{render_manpages, render_shell_completions};
use snafu::{prelude::*, Whatever};
use tracing;

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
    render_shell_completions::<Commands>(Path::new(fmt!(
        "{}/{}",
        dirs::home_dir().unwrap(),
        "/.config/fish/completions"
    )))
    .expect("shell completions generation failed");
}

pub fn generate_manpages() {
    render_manpages::<Commands>(Path::new(fmt!(
        "{}/{}",
        dirs::home_dir().unwrap(),
        "/.local/share/man/man1"
    )))
    .expect("man page generation failed");
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

lazy_static::lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();
        settings.merge(File::with_name("examples/watch/Settings.toml")).unwrap();

        settings
    });
}

#[tracing::instrument(level = "trace", skip_all, err)]
fn config_show() {
    println!(
        " * Settings :: \n\x1b[31m{:?}\x1b[0m",
        SETTINGS
            .read()
            .unwrap()
            .clone()
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    );
}

#[tracing::instrument(level = "trace", skip_all, err)]
fn config_watch() {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(
        tx,
        notify::Config::default().with_poll_interval(Duration::from_secs(2)),
    )
    .unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(
            Path::new(fmt!(
                "{}/{}",
                dirs::home_dir().unwrap(),
                "/examples/watch/Settings.toml"
            )),
            RecursiveMode::NonRecursive,
        )
        .unwrap();

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(Ok(Event {
                kind: notify::event::EventKind::Modify(_),
                ..
            })) => {
                println!(" * Settings.toml written; refreshing configuration ...");
                SETTINGS.write().unwrap().refresh().unwrap();
                show();
            }

            Err(e) => println!("watch error: {:?}", e),

            _ => {
                // Ignore event
            }
        }
    }
}
