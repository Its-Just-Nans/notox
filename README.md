# notox [![crates.io version](https://img.shields.io/crates/v/notox)](https://crates.io/crates/notox) ![crates.io downloads](https://img.shields.io/crates/d/notox) [![docs.rs](https://img.shields.io/docsrs/notox)](https://crates.io/crates/notox)

no toxic names anymore. May looks like [detox](https://github.com/dharple/detox).

## Usage

```sh
# installation
cargo install notox

notox --do my_path
```

> By default, notox will only print the names that would be renamed. Use the `--do` or `-d` option to actually rename the files.

## Options

| Option                | Description                  |
| --------------------- | ---------------------------- |
| `-v`, `--version`     | Prints version information   |
| `-d`, `--do`          | Do the actions               |
| `-q`, `--quiet`       | No output                    |
| `-j`, `--json`        | Output as JSON               |
| `-p`, `--json-pretty` | Output as JSON (prettified)  |
| `-e`, `--json-error`  | Output as JSON (only errors) |

## Infos

- [Changelog](CHANGELOG.md)
- [Coverage](https://its-just-nans.github.io/notox/coverage/)

## License

- [MIT](LICENSE)
