use human_panic::{metadata, setup_panic};
use test2::*;
use update_informer::{registry, Check};

fn main() {
    setup_panic!(metadata!()
        .authors("Acme Inc. <support@example.com")
        .homepage("www.example.com")
        .support("- Open a support request by email to support@example.com"));

    //let name = env!("CARGO_PKG_NAME");
    //let version = env!("CARGO_PKG_VERSION");
    let name = "update-informer";
    let version = "v1.0.0";
    let informer = update_informer::new(registry::Crates, name, version);

    if let Some(version) = informer.check_version().ok().flatten() {
        println!("New version is available: {}", version);
    }

    println!("3 time 5 is {}", multiply(3, 5));
}
