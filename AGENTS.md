# Repository Guidelines

## Project Structure & Module Organization

- Ruby entrypoints live in `lib/`; `lib/penguin.rb` wires the extension and
  exposes `Penguin::ObjectId`.
- The Rust crate is in `crates/object_id`, this is usable on its own without any
  Ruby dependency.
- The native extension is generated from `ext/penguin_object_id` which is a
  light wrapper around `crates/object_id`. No logic should live here. It is
  compiled into into `lib/penguin_object_id.bundle` via rb-sys.
- Ruby level tests and benchmarks sit in `test/`, following `test_*.rb` and
  `bench_*.rb` patterns.
- The BSON Object ID spec can be found in `specifications/object_id.md`

## Build, Test, and Development Commands

- `bundle exec rake test`: compile the extension (if needed) and execute the
  full Minitest suite, including benchmarks.
- `cargo nextest run -p object_id`: run Rust unit tests in isolation
- `cargo bench -p object_id`: execute Criterion benchmarks for the Rust core
  implementation.

## Testing Guidelines

- Aim to keep parity with BSON’s expectations; consult
  `specifications/object_id.md` when drafting scenarios.
