use test2::multiply;

#[cfg(not(target_arch = "wasm32"))]
use human_panic::{metadata, setup_panic};

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    setup_panic!(
        metadata!()
            .authors("Acme Inc. <support@example.com")
            .homepage("www.example.com")
            .support("- Open a support request by email to support@example.com")
    );

    println!("3 time 5 is {}", multiply(3, 5));
}
