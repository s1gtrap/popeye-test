# Popeye

Silly web application of graph coloring for optimizing training programs/workout routines.

Written in Rust, made with [Yew](https://github.com/yewstack/yew), built with [Trunk](https://github.com/thedodd/trunk) and running on WebAssembly.

Credit to all the wonderful people behind the aforementioned projects not to mention the Rust web ecosystem in general. Also shout out to https://favicon.io/, https://www.noisetexturegenerator.com/ and the Quicksand font family for letting me shine up the place and especially https://exrx.net/ for their detailed and accessible muscle and exercise database.

## Usage

### Running

```bash
trunk serve
```

Rebuilds the app whenever a change is detected and runs a local server to host it.

There's also the `trunk watch` command which does the same thing but without hosting it.

### Release

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`.
You can also pass the `--release` flag to `trunk serve` if you need to get every last drop of performance.

Unless overwritten, the output will be located in the `dist` directory.
