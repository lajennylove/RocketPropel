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
propel_rocket = { git = "https://github.com/lajennylove/propel", rev = "2e4336c25b043c543546bb19cbbb6bddb0bf40d3" }
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

## Test pages

The home page at `/` links to each page. Every page exercises one part of Propel.

| Page | What it checks |
| --- | --- |
| `/` | the home page, `@foreach` over a list of links |
| `/posts`, `/posts/1` to `/posts/3` | `@forelse` / `@empty`, `@if` / `@else`, route parameters, layouts |
| `/posts/99`, `/posts/abc` | the 404 page, for an unknown or non-numeric id |
| `/features/escaping` | `{{ }}` escaping, `{!! !!}` raw output, URL checks in `href`, `@@`, `@{{`, plain `@words` |
| `/features/control` | `@if` / `@elseif` / `@else`, `@foreach`, `@forelse` with `@empty`, null values |
| `/features/dump` | `@dump`: prints the value and keeps rendering |
| `/features/dd` | `@dd`: prints the value and stops the page |
| `/features/cache` | the template cache: edit `views/features/cache.propel` and refresh |
| `/errors/missing` | a missing variable: a 500 with the message in debug, generic in release |
| `/errors/nested-section` | a `@section` inside `@if`: a 500 with the message |

Notes:

- Bare directive words in text must be written as `@@`, for example `@@dump`, or they fail to parse.
- `/posts/abc` returns the same 404 page as an unknown post.
- The dev server needs `ROCKET_ADDRESS=0.0.0.0` to be reachable from a phone. Run it on a network you trust.

## Assets: CSS, JS and images

The full setup guide, including the config files and troubleshooting, is in the Propel repository at `docs/assets.md`.

The app uses Vite 8 with Tailwind 4 and the latest `laravel-vite-plugin`, the same setup Laravel uses. Install once, then run the tools you need:

```sh
npm install
npm run dev     # Vite dev server: hot reload for CSS, JS and assets
npm run build   # production build into public/build
```

### In templates

```blade
@vite(['resources/css/app.css', 'resources/js/app.js'])
<img src="{{ asset('resources/images/propel-mark.svg') }}" alt="">
```

- `@vite([...])` writes the `<link>` and `<script>` tags for each entry. Entries listed in the layout load on every page.
- `asset('path')` returns the URL of a file. The path is relative to the app folder.
- Inside CSS, `url(@images/…)`, `url(@fonts/…)`, `url(@styles/…)` and `url(@scripts/…)` resolve to the matching `resources/` folder.

### How the URLs are chosen

- **While `npm run dev` runs**, Vite writes its URL to `public/hot`. Propel then points every asset at the dev server, and `@vite` adds the HMR client. The URL uses this machine's LAN address, so a phone on the same Wi-Fi can load the styles. The address is read when `npm run dev` starts. If your address changes, restart the dev server.
- **Otherwise** (after `npm run build`), Propel reads `public/build/manifest.json` for hashed files. Images and fonts are copied into `public/build/resources`, so `asset('resources/images/x.svg')` resolves to `/build/resources/images/x.svg`.
- **If the file is in neither place**, the page fails with a 500 that says what to run. This is on purpose: a missing asset is a bug to fix, not a broken image to ship.

### Security

- `npm run dev` listens on the network (`server.host: true`) so a phone can reach it. Use it on networks you trust, and stop it when you are done. The dev server serves your source files.
- `public/hot` is removed when the dev server stops.
