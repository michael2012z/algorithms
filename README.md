# algorithms

## Usage

Run the test of all algorithm packages:

```shell
cargo test --workspace -- --show-output
```

Run the test of single package:

```shell
cargo test --package "sort" -- --show-output
```

Run the test of a package and generate `.dot` file for `Graphviz`:

```shell
cargo test --package "sort" --features graph -- --show-output
```

Run a specified test case:

```shell
cargo test --package "sort" test_bubble_sort_simple --features graph -- --show-output
```
