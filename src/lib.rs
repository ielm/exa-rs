pub mod client;
pub mod constants;
pub mod types;

pub use client::Exa;

pub trait ExaRequest {
    fn as_json(&self) -> serde_json::Value;
}
