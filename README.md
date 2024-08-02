
#  Cloud sync-point.

Small web service with one endpoint: `/wait-for-second-party/:unique-id`

This endpoint allows two parties to sync. When one party makes a POST request, the response will be delayed until the second party requests the same URL. 

In other words, the first party is blocked until the second party arrives or a timeout occurs ( 10 seconds).

We use `tokio` to handle the async operations and `warp` to create the web service.

## Rust

This implementation uses the latest stable version of Rust (rustc 1.80.0).

To install Rust, follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

## Usage

You can run a simple execution of the code under the `main.rs` file using 

```bash
cargo run
```