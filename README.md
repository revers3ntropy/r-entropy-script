# Simple Compiler in Rust

Tested on Ubuntu.

## Requirements

- nasm
- ld

## Examples
See `spec/*` for more examples

```shell
$ bin/exec "1+1"
2
```
```shell
$ bin/exec 'print("Hello, World!")'
Hello, world!
```

## Usage

```shell
$ res [input_file?] [options]
```

Quote the input to escape shell expansion, 
e.g. `res -e "(1+1)*2"` instead of `./res -e (1+1)*2`

### Options

| Command          | Type | Description                            | Default     |
|------------------|------|----------------------------------------|-------------|
| `-o`, `--output` | Str  | Output assembly file path              | `'out.asm'` |
| `-e`, `--eval`   | Str  | Evaluate and print a single expression |             |
| `-x`, `--exec`   | Int  | Exec mode                              | `0`         |

#### Exec Mode
`0`: Compile to application

`1`: Print last expression

`2`: Compile as library

## Commands

```shell
# Build
$ bin/build

# Compile, assemble, link and run file
$ bin/run <file>

# Compile, assemble, link and print expression
$ bin/exec "1+1"

# Run test suite
$ bin/test

# Assemble a file
$ bin/asm <file>
```