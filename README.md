# Spew Id

It’s a command-line utility to generate unique identifiers (UUID, ULID, ObjectId)
written in Rust.

The executable name is `spwd`, which is a vowelless continuous writing of “spew id” —
easy to remember and quick to type.

## Motivation

Sometimes I just need a random identifier, sometimes not so random. Sometimes it’s an
ObjectId, yet sometimes it’s a UUID v7. Sometimes I need ten of those. And what is the
simplest and fastest way to get one? Is it REPL, web search, or a terminal? Probably
the latter. Especially if I want it in my shell scripts. But what options do I have?

For UUID, it’s the standard `uuidgen`. How standard? Well, the macOS version is quite
limited. And `brew search uuid` gives me one other option from 2008. Can I build the
Linux version of `uuidgen`? Sure. Do I want to do it? Not really.

For ULID, there’re no standard options, and `brew search ulid` gives nothing useful.
There are a few options in the wilds, like [`timclicks/ulid-lite`][ulid-lite] and
[`technosophos/ulid`][ulid-go], but nothing I can simply `brew install`.

For ObjectId, which is my favorite, there’s nothing. Nope, I’m not counting anything
JavaScript. With `mongosh`, it’s quite easy to write a shell function that’ll do the
job, but doesn’t it require Node.js? Well, thank you, but no, thank you.

As for an option that covers all three, I never found one. So, it turns out, the
simplest way right now is to use web search or the online tools I found earlier. And
that’s quite sad.

**UPD**: Later I found various implementations among [crates.io][crates-io] packages
but none of those ticked all my boxes and were available through Homebrew.

So, here’s my take on a single utility — a one-stop shop if you like — to cover all
those cases.

[ulid-lite]: https://github.com/timclicks/ulid-lite
[ulid-go]: https://github.com/technosophos/ulid
[crates-io]: https://crates.io

## Usage

The `spwd` is available as stand-alone binaries, Cargo and Homebrew packages, and a
container image.

Binaries can be found on the repo’s [releases page][releases]. If there’s no platform
you’re looking for, you can compile an appropriate binary yourself. Or feel free to
create [a PR][pulls] or [an issue][issues].

The Cargo package can be installed as usually:

```sh
cargo install spwd
```

The Homebrew package can be obtained through the tap:

```sh
brew install aeron/tap/swpd
```

The container image is available as [`docker.io/aeron/spwd`][docker] and
[`ghcr.io/Aeron/spwd`][github]. You can use them both interchangeably.

```sh
docker pull docker.io/aeron/spwd
# …or…
docker pull ghcr.io/aeron/spwd
```

[releases]: https://github.com/Aeron/spwd/releases
[pulls]: https://github.com/Aeron/spwd/pulls
[issues]: https://github.com/Aeron/spwd/issues
[docker]: https://hub.docker.com/r/aeron/spwd
[github]: https://github.com/Aeron/spwd/pkgs/container/spwd

### Arguments

Running the app with `-h` or `--help` option will give you the following:

```text
Usage: spwd [OPTIONS] <COMMAND>

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
$ hyperfine --warmup 10 -N 'target/release/spwd uuid' 'uuidgen'

Benchmark 1: target/release/spwd uuid
  Time (mean ± σ):       1.7 ms ±   0.1 ms    [User: 0.8 ms, System: 0.5 ms]
  Range (min … max):     1.5 ms …   3.1 ms    1528 runs

Benchmark 2: uuidgen
  Time (mean ± σ):       2.5 ms ±   0.2 ms    [User: 1.2 ms, System: 0.6 ms]
  Range (min … max):     2.3 ms …   3.2 ms    1218 runs

Summary
  target/release/spwd uuid ran
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
