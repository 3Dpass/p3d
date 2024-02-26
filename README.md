# p3d
3D object shape recognition tools for WASM, which not only has the same functionality to [pass3d](https://github.com/3Dpass/pass3d), but also it's compatible to Substrate. Now, it's used as a part of 3Dpass network Node.

# p3d_opt
My fun with p3d algo optimization focused on processing larger objects (but maybe also useful for mining)

* main branch is baseline
* testing branch is optimized (currently ~35% faster)

# Running tests

```
git checkout main
cargo test --release --test '*'
warning: unused manifest key: build
   Compiling p3d v0.7.0 (/home/crypto/3dpass/p3d_opt)
    Finished release [optimized] target(s) in 1.70s
     Running tests/hashes_test.rs (target/release/deps/hashes_test-aa8c1695328682ec)

running 3 tests
test rat_test ... ok
test cow_test ... ok
test cat_test ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 49.28s

git checkout testing
cargo test --release --test '*'
warning: unused manifest key: build
   Compiling p3d v0.7.0 (/home/crypto/3dpass/p3d_opt)
    Finished release [optimized] target(s) in 1.69s
     Running tests/hashes_test.rs (target/release/deps/hashes_test-aa8c1695328682ec)

running 3 tests
test rat_test ... ok
test cow_test ... ok
test cat_test ... ok


test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 31.97s
```
