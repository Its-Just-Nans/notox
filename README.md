# notox

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
