unzip object

## How to install
1. clone this repository
2. build
```
cargo build --release
```
3. install
```
cargo install --path .
```

## How to use
1. go to directory containing .git directory
2. run command with hexdump, optionally.
```
unzip <object-id> | hexdump -C
```
