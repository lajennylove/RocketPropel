# Propel docs

The documentation site for [Propel](https://github.com/lajennylove/Propel), at [propel.rocketboost.rs](https://propel.rocketboost.rs). It is a Rocket app that uses Propel, laid out the way your own project would be.

## Setup

You need Rust (stable, with rustup), and Node.js 22 or later.

```sh
rustup default stable               # the toolchain in rust-toolchain.toml is installed on first build
npm install
npm run build                       # builds the stylesheet and scripts into public/build
```

## Run it

```sh
npm run dev                         # Vite: hot reload for CSS and JavaScript, in one terminal
cargo run                           # the site on http://127.0.0.1:8000, in another
```

Run the commands from the repository root. The views in `resources/views/` are compiled into the binary when it builds (`build.rs`), so a change to a view needs a rebuild. `bacon` rebuilds and restarts the site on each save.

## Layout

```
src/                  routes, and the typed context of each view (views.rs)
resources/
  views/              the Propel views
    layouts/app.propel
    docs/             the documentation pages
  css/ js/ images/    the front-end sources, built by Vite
build.rs              compiles the views listed in it
package.json          Vite and Tailwind
vite.config.js
```

## Propel as a dependency

The site lists Propel in `Cargo.toml` by git revision:

```toml
propel_rocket = { git = "https://github.com/lajennylove/Propel", tag = "v0.2.0" }
```

To move to a newer Propel, change the tag and run `cargo update -p propel_rocket`.

## Assets

Templates reference assets with `@vite([...])` and `asset('...')`. The Vite dev server is used while `npm run dev` runs, and the built files otherwise. The full setup is in the [Propel documentation](https://propel.rocketboost.rs/docs/assets).

## Test pages

| Page | What it checks |
| --- | --- |
| `/` | the home page |
| `/posts`, `/posts/1` to `/posts/3` | `@forelse` and `@empty`, `@if`, route parameters, layouts |
| `/features/escaping` | `{{ }}` escaping, `{!! !!}` raw output, URL checks, `@@` |
| `/features/control` | `@if`, `@foreach`, `@forelse`, null values |
| `/features/dump`, `/features/dd` | `@dump` (keeps rendering) and `@dd` (stops the page) |
| `/features/cache` | the template cache in development |
| `/errors/missing` | a missing variable, a 500 in debug builds |
| `/errors/nested-section` | a `@section` inside `@if`, a 500 in debug builds |

Bare directive words in text must be written as `@@`, for example `@@dump`, or they fail to parse.

## Deploy

Pushing to `main` deploys the site: the GitHub Action runs the deploy script on the server, which pulls the repository, builds the assets and the binary, and restarts the service.

## Security

`npm run dev` listens on the network so another device on the same network can load the styles. Use it on networks you trust, and stop it when you are done.
