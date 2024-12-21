#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(rustdoc_missing_doc_code_examples))]
#![cfg_attr(docsrs, warn(rustdoc::missing_doc_code_examples))]
#![cfg_attr(docsrs, warn(rustdoc::invalid_codeblock_attributes))]

//! lambda_sqs
//!
//! Specialised lambda_runtime to accept and process events from SQS.
//!
//! # SQS Events
//!
//! SQS dispatches events to the a lambda function in batches (often, it seems
//! to my surprise). This crate provides a lambda_runtime implementation which
//! expects to receive a batch of messages in the [SqsEvent] type and provides
//! a method to transform the batch of events to a vector of your Struct.
//!
//! # Example
//!
//! In Cargo.toml add lambda_sqs as a dependency.
//!
//! ```toml
//! [dependencies]
//! lambda_sqs = " {0.2.17"
//! ```
//!
//! ```no_run
//! # type YourStruct = String;
//! use lambda_sqs::{handler_fn, Context, Error};
//! use lambda_sqs::SqsEvent;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     lambda_sqs::run(handler_fn(my_handler)).await?;
//!
//!     Ok(())
//! }
//!
//! async fn my_handler(e: SqsEvent, c: Context) -> Result<(), Error> {
//!     let events: Vec<YourStruct> = e.into_t();
//!#     // Process events
//!#     Ok(())
//!# }
//! ```
//!

mod domain;

pub use domain::SqsEvent;
pub use lambda_runtime::*;
