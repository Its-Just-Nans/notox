# notox [![crates.io version](https://img.shields.io/crates/v/notox)](https://crates.io/crates/notox) ![crates.io downloads](https://img.shields.io/crates/d/notox) [![docs.rs](https://img.shields.io/docsrs/notox)](https://docs.rs/notox)

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
| `-d`, `--do`          | Do the actions (rename)      |
| `-q`, `--quiet`       | No output                    |
| `-j`, `--json`        | Output as JSON               |
| `-p`, `--json-pretty` | Output as JSON (prettified)  |
| `-e`, `--json-error`  | Output as JSON (only errors) |

## Usage as lib

```rust
use std::collections::HashSet;
use std::path::PathBuf;
use notox::{Notox, NotoxArgs, NotoxOutput};
let paths: HashSet<PathBuf> = HashSet::from(["README.md".into(), "Cargo.toml".into()]);
let notox_args = NotoxArgs {
    dry_run: true, // change here
    // if using serde
    // output: NotoxOutput::JsonOutput {
    //    json: JsonOutput::JsonDefault,
    //    pretty: false,
    // },
    output: NotoxOutput::Quiet
};
let notox_inst = Notox::new(notox_args);
let res = notox_inst.run(&paths);
// to print them
notox_inst.print_output(res);
```

## Infos

- [Changelog](CHANGELOG.md)
- [Coverage](https://its-just-nans.github.io/notox/coverage/)

## License

- [MIT](LICENSE)
