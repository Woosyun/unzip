unzip objects

## How to install
1. clone repository
2. build
```
cargo build --release
```
3. install
```
cargo install --path .
```

## How to use
```
cat .git/objects/37/56aa57fa56d2ddec7c54412ccd3c7619d5ab53 | unzip | hexdump -C
```
