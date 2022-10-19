use tracing_subscriber::{prelude::*, EnvFilter};
use zero_knowledge::ZeroKnowledge;

fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_forest::ForestLayer::default())
        .init();

    risc::fib(1000).prove_and_verify();
    println!("Hello, world!");
}
