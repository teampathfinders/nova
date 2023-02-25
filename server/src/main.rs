#![allow(dead_code)]
#![allow(unused)]
#![allow(clippy::module_inception)]
#![warn(clippy::nursery)]

extern crate core;

use std::sync::atomic::{AtomicU16, Ordering};

use clap::{crate_authors, crate_description, Command};
use tokio::runtime;

use crate::instance_manager::InstanceManager;
use common::VResult;

mod command;
mod config;
mod crypto;
mod entity;
mod instance_manager;
mod level_manager;
mod network;

#[cfg(test)]
mod test;

fn main() -> VResult<()> {
    let matches = Command::new("nova")
        .version(concat!(
            env!("VERGEN_GIT_SHA_SHORT"),
            " ",
            env!("VERGEN_BUILD_TIMESTAMP")
        ))
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .get_matches();

    init_logging();
    init_runtime()
}

fn init_runtime() -> VResult<()> {
    let runtime = runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .thread_name_fn(|| {
            static ATOMIC_THREAD_COUNTER: AtomicU16 = AtomicU16::new(0);
            format!(
                "async-thread-{}",
                ATOMIC_THREAD_COUNTER.fetch_add(1, Ordering::Relaxed)
            )
        })
        .build()
        .expect("Failed to build runtime");

    tracing::info!("Starting server...");
    runtime.block_on(InstanceManager::run())
}

/// Initialises logging with tokio-console.
#[cfg(feature = "tokio-console")]
fn init_logging() {
    use std::time::Duration;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    let console_layer = console_subscriber::Builder::default()
        .retention(Duration::from_secs(1))
        .recording_path("console_trace.log")
        .spawn();

    let fmt = tracing_subscriber::fmt::layer().with_target(false);

    tracing_subscriber::registry()
        .with(console_layer)
        .with(fmt)
        .init();

    tracing::info!("Tokio console enabled");
}

/// Initialises logging without tokio-console.
#[cfg(not(feature = "tokio-console"))]
fn init_logging() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(tracing::Level::DEBUG)
        .with_file(true)
        .with_line_number(true)
        .init();
}
