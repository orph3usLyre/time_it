# time_it

A Rust procedural macro that adds execution timing to your functions. Requires the `tracing` library.

## Features

- **Simple**: Just add `#[time_it]` to any function
- **Async/sync compatible**: Works with both regular and `async` functions  
- **Configurable log levels**: Choose from trace, debug, info, warn, or error

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
time_it = "0.1.0"
tracing = "0.1"
```

### Basic Usage

```rust
use time_it::time_it;

#[time_it]  // Logs at DEBUG level by default
fn slow_computation() -> u64 {
    std::thread::sleep(std::time::Duration::from_millis(100));
    42
}

#[time_it]
async fn async_work() {
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
}
```

### Custom Log Levels

```rust
#[time_it("trace")]
fn traced_function() {
    // Logs execution time at TRACE level
}

#[time_it("info")]  
async fn important_async_work() {
    // Logs execution time at INFO level
}

#[time_it("error")]
fn critical_path() {
    // Logs execution time at ERROR level
}
```

### Complete Example

```rust
use time_it::time_it;
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    regular_work();
    async_work().await;
}

#[time_it]
fn regular_work() {
    println!("Doing some work...");
    std::thread::sleep(std::time::Duration::from_millis(100));
}

#[time_it("info")]
async fn async_work() {
    println!("Doing async work...");
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
}
```

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

