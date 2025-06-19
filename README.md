# IdGen

It’s a command-line utility to generate unique identifiers (UUID, ULID, ObjectId)
written in Rust.

## Motivation

Sometimes I just need a random identifier, sometimes not so random. Sometimes it’s an
ObjectId, yet sometimes it’s a UUID v7. Sometimes I need ten of those. And what is the
simplest and fastest way to get one? Is it REPL, web search, or a terminal? Probably
the latter. Especially, if I want it in my shell scipts. But what options do I have?

For UUID, it’s the standard `uuidgen`. How standard? Well, the macOS version is quite
limited. And `brew search uuid` gives me one other option from 2008. Can I build the
Linux version of `uuidgen`? Sure. Do I want to do it? Not really.

For ULID, there’re no standard options, and `brew search ulid` gives nothing useful.
There a few options in the wilds, like [`timclicks/ulid-lite`][ulid-lite] and
[`technosophos/ulid`][ulid-go], but nothing I can simply `brew install`.

For ObjectId, which is my favorite, there’s nothing. Nope, I’m not counting anything
JavaScript. With `mongosh`, it’s quite easy to write a shell function that’ll do the
job, but doesn’t it require Node.js? Well, thank you, but no, thank you.

As for an option that covers all three, I never found one. So, it turns out, the
simplest way right now is to use web search or the online tools I found earlier. And
that’s quite sad.

So, here’s my take on a single utility to cover all those cases.

[ulid-lite]: https://github.com/timclicks/ulid-lite
[ulid-go]: https://github.com/technosophos/ulid

## Installation

### From Source

Clone the repository and build with Cargo:

```sh
git clone https://github.com/yourusername/idgen.git
cd idgen
cargo build --release
```

The binary will be available at `target/release/idgen`. You can copy it to a directory
in your `PATH`:

```sh
cp target/release/idgen /usr/local/bin/
```

### Using Cargo

If the crate is published to crates.io:

```sh
cargo install idgen
```

## Usage

### Arguments

Running the app with `-h` or `--help` option will give you the following:

```text
Usage: idgen [OPTIONS] <COMMAND>

Commands:
  uuid  Generate a new UUID
  ulid  Generate a new ULID
  oid   Generate a new ObjectId

Options:
  -n, --num <NUMBER>  Number of results [default: 1]
  -h, --help          Print help
  -V, --version       Print version
```

Simply run `--help` for a certain command to see command-specific options.

For usage examples, see [USAGE.md](USAGE.md).

## Performance

In case the performance is a consideration, here are the benchmarks against the standard
`uuidgen` utility (macOS 26.3 @ Apple M1 Max):

```sh
$ hyperfine --warmup 10 -N 'target/release/idgen uuid' 'uuidgen'

Benchmark 1: target/release/idgen uuid
  Time (mean ± σ):       1.7 ms ±   0.1 ms    [User: 0.8 ms, System: 0.5 ms]
  Range (min … max):     1.5 ms …   3.1 ms    1528 runs

Benchmark 2: uuidgen
  Time (mean ± σ):       2.5 ms ±   0.2 ms    [User: 1.2 ms, System: 0.6 ms]
  Range (min … max):     2.3 ms …   3.2 ms    1218 runs

Summary
  target/release/idgen uuid ran
    1.52 ± 0.15 times faster than uuidgen
```

## Acknowledgments

This project relies on the following excellent libraries for identifier generation:

- **[uuid]** by [uuid-rs], available under Apache-2.0 OR MIT
- **[bson]** by [MongoDB Inc.], available under MIT
- **[ulid]** by [Dylan Hart], available under MIT

[uuid]: https://github.com/uuid-rs/uuid
[uuid-rs]: https://github.com/uuid-rs
[bson]: https://github.com/mongodb/bson-rust
[MongoDB Inc.]: https://github.com/mongodb
[ulid]: https://github.com/dylanhart/ulid-rs
[Dylan Hart]: https://github.com/dylanhart
