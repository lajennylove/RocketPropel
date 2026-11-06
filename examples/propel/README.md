# propel

A Rocket app that uses Propel. It lives in the Rocket repo, next to Rocket's other examples.

## Setup

You need rustup, not only Homebrew's `cargo`. The `rust-toolchain.toml` in this directory asks rustup for the stable toolchain with clippy and rustfmt, and rustup installs them on the first build.

```sh
brew install rustup bacon          # rustup manages the toolchain; bacon rebuilds on change
rustup-init -y --no-modify-path    # or: rustup default stable, if rustup is already set up
export PATH="$HOME/.cargo/bin:$PATH"   # put rustup's cargo ahead of Homebrew's
```

Check that the toolchain files are honored:

```sh
cargo clippy --version             # should print clippy, from rustup
```

Homebrew's `cargo` ignores `rust-toolchain.toml`, so run the commands through rustup's `cargo` (put `~/.cargo/bin` first in PATH, as above).

Run commands from this directory. rustup reads `rust-toolchain.toml` from the current directory upward, so running from `examples/` skips it.

## Propel as a dependency

Propel is listed in `Cargo.toml`, the same way a Composer project lists packages in `composer.json`:

```toml
propel_rocket = { git = "https://github.com/lajennylove/propel", rev = "dc818cd679c0c9b88c6f3c301449804b07ec0b2b" }
```

Propel isn't on crates.io, so it comes from git. The `rev` pins the version, like a Composer lock entry. To move to a newer Propel, change `rev` and run `cargo update -p propel_rocket`.

## Run it

From this directory:

```sh
cargo run          # serve on http://127.0.0.1:8000/posts
bacon              # rebuild and restart whenever Rust code changes
```

Run from this directory, not from `examples/`. Views are read from `views/` relative to the working directory.

## Hot reload

Two kinds of change, two speeds:

- **View edits** (`views/**/*.propel`) show up on the next request, with no rebuild. Dev builds read the template from disk on each render.
- **Rust edits** (`src/`) need a rebuild. `bacon` does this automatically.
- `bacon` watches `src/` only. A view edit does not restart the server, because the dev renderer reads views on each request.

## Layout

```
src/views.rs          typed context structs, one per view
src/main.rs           routes; handlers build the context
views/                Propel views, in the regular Rocket layout
  layouts/app.propel
  posts/index.propel
```

## Status

- The views use the draft syntax from the Propel plan. Some of it is still open: member access, truthiness of `Option`, and the `@forelse` empty branch. The files may change.
- Rendering uses the dev interpreter (`propel_runtime`). The compiled path (`propel_compiler`) is not built yet.
- Release builds are refused on purpose until it exists. Before shipping this app, read [`propel/docs/production.md`](../../propel/docs/production.md).
