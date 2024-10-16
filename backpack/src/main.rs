use color_eyre::eyre::Result;
use console::{Emoji, Style};
use dirs;
use eyre::{Report, WrapErr};
use std::{fs, io, path::PathBuf};
use template_eyre::Hook;
use test2::*;
use tracing::{debug, error, info, instrument, span, warn, Level};
use tracing_subscriber::{layer::SubscriberExt, registry::Registry};
use tracing_tree::HierarchicalLayer;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use config::{Config, File};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::RwLock;
use std::time::Duration;

fn main() {
    Hook::new(include_str!("errors.hbs"))
        .unwrap()
        .install()
        .unwrap();

    let cyan = Style::new().cyan();
    let result = multiply(3, 5);

    println!("3 time 5 is {} {}", cyan.apply_to(, Emoji("✨", ":-)")));

    // Logger
    pretty_env_logger::init();
    info!("such information");
    warn!("o_O");
    error!("much error");

    // Tracing
    let subscriber = Registry::default().with(HierarchicalLayer::new(2));
    tracing::subscriber::set_global_default(subscriber).unwrap();

    // Error handling
    color_eyre::install()?;
    if std::env::var("RUST_SPANTRACE").is_err() {
        std::env::set_var("RUST_SPANTRACE", "0");
    }

    // CLI arguments
    let cli = Command::new("example")
        .about("A fictional versioning CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("clone")
                .about("Clones repos")
                .arg(arg!(<REMOTE> "The remote to clone"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("diff")
                .about("Compare two commits")
                .arg(arg!(base: [COMMIT]))
                .arg(arg!(head: [COMMIT]))
                .arg(arg!(path: [PATH]).last(true))
                .arg(
                    arg!(--color <WHEN>)
                        .value_parser(["always", "auto", "never"])
                        .num_args(0..=1)
                        .require_equals(true)
                        .default_value("auto")
                        .default_missing_value("always"),
                ),
        )
        .subcommand(
            Command::new("push")
                .about("pushes things")
                .arg(arg!(<REMOTE> "The remote to target"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("add")
                .about("adds things")
                .arg_required_else_help(true)
                .arg(arg!(<PATH> ... "Stuff to add").value_parser(clap::value_parser!(PathBuf))),
        )
        .subcommand(
            Command::new("stash")
                .args_conflicts_with_subcommands(true)
                .args(push_args())
                .subcommand(Command::new("push").args(push_args()))
                .subcommand(Command::new("pop").arg(arg!([STASH])))
                .subcommand(Command::new("apply").arg(arg!([STASH]))),
        );

    // Generate help information
    generate_completions();
    generate_manpages();

    // Watch config file
    config_show();
    config_watch();
}
