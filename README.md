# fum

Fum is a command-line tool for fuzzy matching, searching recursively through
files in the current directory and skipping those excluded by gitignore rules.

It can be used like this

```
$ fum pattrn
./Cargo.toml:6: pattern
./Cargo.toml:13: pattern
./README.md:9: pattern
./README.md:10: pattern
./README.md:11: pattern
./README.md:12: pattern
./README.md:13: pattern
./README.md:14: pattern
./README.md:15: pattern
./README.md:16: pattern
./README.md:17: pattern
./README.md:18: pattern
./src/main.rs:10: pattern
./src/main.rs:11: pattern
./src/main.rs:16: pattern
./src/main.rs:23: pattern
./src/main.rs:31: pattern
./src/main.rs:39: pattern
```

