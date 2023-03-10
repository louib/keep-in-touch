# keep-in-touch
[![dependency status](https://deps.rs/repo/github/louib/keep-in-touch/status.svg)](https://deps.rs/repo/github/louib/keep-in-touch)
[![License file](https://img.shields.io/github/license/louib/keep-in-touch)](https://github.com/louib/keep-in-touch/blob/master/LICENSE)

Contact manager based on the KDBX4 encrypted database format.

`keep-in-touch` is still in active development. There currently is only a repl-like CLI, but
my plan is to add a TUI in the future. See [this issue](https://github.com/louib/keep-in-touch/issues/1#issuecomment-1418337633) for the reasoning behind the project.

## Installing

### With Nix
Assuming that you have enabled both the `flakes` and `nix-command` experimental features:
```
nix profile install github:louib/keep-in-touch
```

### With Cargo
```
cargo install --path .
```

## References
* https://datatracker.ietf.org/doc/html/rfc6350
* https://en.wikipedia.org/wiki/VCard
