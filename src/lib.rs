//! http://atlas.lidar.io

#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]

extern crate iron;
extern crate heartbeat;
extern crate notify;
extern crate router;
extern crate rustc_serialize;
extern crate toml;

mod config;
mod error;
mod handler;
mod server;
mod watch;

pub use error::Error;
pub use server::Server;

/// Crate-specific result type.
pub type Result<T> = std::result::Result<T, Error>;
