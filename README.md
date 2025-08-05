# Rust URL Shortener

A simple URL shortener built with Rust and Axum.

## Usage

- Start the server:

```bash
cargo clean
cargo build
cargo run -- serve
```
- Shorten a URL:
```bash
curl -X POST [http://127.0.0.1:8000/shorten](http://127.0.0.1:8000/shorten) \
  -H "Content-Type: application/json" \
  -d '{"url":"[https://example.com](https://example.com)"}'
```
- Expand a short code:
```bash
curl -i [http://127.0.0.1:8000/u/](http://127.0.0.1:8000/u/)<short_code>
```
**Development**
- Run tests
The core logic is tested in `lib.rs`. To run tests:
```bash
cargo test
```
**Formatting and linting**
- Ensure code style and quality by running:
```bash
cargo fmt
#cargo clippy -- -D warnings still to implement this 
```
License: [MIT](./LICENSE)
