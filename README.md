A library for reading logs created by [RDS Spy](https://rdsspy.com/).

There is a large collection of logs, from around the world, at https://github.com/walczakp/rds-spy-logs.
This project references rds-spy-logs as a Git submodule and uses them in tests.

# Getting Source

```sh
git clone --recurse-submodules https://github.com/cmumford/rdspy.git
```

# Running tests

```sh
cargo test
```

# Example Program

To process and dump a single file:
```sh
cargo run --example sample third_party/rds-spy-logs/Austria/A540_-_2021-07-26_19-08-06.spy
```

or to read from stdin:
```sh
cargo run --example sample third_party/rds-spy-logs/Austria/A540_-_2021-07-26_19-08-06.spy
```

or to process all `*.spy` and `*.rds` files in a directory (recursively)
```sh
cargo run --example sample third_party/rds-spy-logs
```
