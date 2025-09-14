# Pengin

> He just wanted a good object ID generator

The `bson` gem is great, if you're using BSON. However, it's a big dependency
just for generating object IDs generation. Penguin provides _just_ object ID
generation.

## Usage

Penguin provides the same interface as `BSON::ObjectId` so you should be able to
drop it in as a replacement. If not, that's a bug.

Here's a shortlist of what you likely want to do this with gem:

- Generate IDs: `Penguin::ObjectId.new`
- Convert IDs to strings: `Penguin::ObjectId.new.to_s`
- Construct an ID from a given time: `Penguin::ObjectId.from_time(Time.now, unique: true)`

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake test` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine:

1. Run `bundle install`
2. Compile and test using `bundle exec rake build test`

You can run and test the Rust code by itself using cargo.

### Benchmarking

There are benchmarking tests in the Ruby test suite that ensure `penguin` is at
least as performant as `bson`. There are also criterion benchmarks in the Rust
code so you can delve more deeply into the library specific performance.
Currently, it is heavily dominated by Ruby-Rust interop, rather than the ID
generation itself, parsing, etc.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
