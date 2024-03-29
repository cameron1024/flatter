# Flatter

A command line utility for rendering SVGs to PNGs for Flutter applications.

Flutter does not natively support SVGs, and 3rd-party packages to provide SVG support have historically had issues with certain types of SVGs.

Often, however, SVGs are available ahead of time (e.g. provided by a designer) and can be "flattened" into PNGs, which Flutter natively supports. Flatter aims to provide a fast, accurate, and repeatable renders of static SVGs, optimized for this use case.


## Installation

Cargo users can run:
```
cargo install flatter
```

Homebrew users can run:
```
brew tap cameron1024/flatter
brew install flatter
```

More platforms coming soon...

## Usage

```
flatter --input path/to/svgs --output flutter_app/assets/images --scales 1 2 3 4
```
Generate 1x, 2x, 3x, 4x versions of each SVG in `path/to/svgs`, and write them to `flutter_app/assets/images`

## Contributing

Got feedback? Suggestions? Improvements? Issues? Feel free to file an issue or submit a PR and I'll be happy to take a look!
