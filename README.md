# p3d
3D object shape recognition tools for WASM, which not only has the same functionality to [pass3d](https://github.com/3Dpass/pass3d), but also it's compatible to Substrate. Now, it's used as a part of 3Dpass network Node.

# p3d_opt
My fun with p3d algo optimization focused on processing larger objects (but maybe also useful for mining)

* main branch is baseline
* testing branch is optimized (currently ~35% faster)

# Running tests

```
git checkout main
cargo test --release --test '*'  -- --test-threads 1
warning: unused manifest key: build
    Finished release [optimized] target(s) in 0.02s
     Running tests/hashes_test.rs (target/release/deps/hashes_test-aa8c1695328682ec)

running 20 tests
test car_test ... ok
test cat_test ... ok
test characterlowpoly2_test ... ok
test cow_test ... ok
test deer_test ... ok
test fish2_test ... ok
test fish_test ... ok
test garbage_truck_test ... ok
test goat_test ... ok
test hummer_hx_test ... ok
test low_poly_mill_test ... ok
test low_poly_tree_test ... ok
test rat_test ... ok
test t_3dpass_576329_test ... ok
test traffic_light_test ... ok
test tree_birch_test ... ok
test warehouse_test ... ok
test water_tank_test ... ok
test wolf2_test ... ok
test wolf_test ... ok

test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 311.14s

git checkout testing
cargo test --release --test '*'  -- --test-threads 1
warning: unused manifest key: build
   Compiling p3d v0.7.0 (/home/crypto/3dpass/p3d_opt)
    Finished release [optimized] target(s) in 1.71s
     Running tests/hashes_test.rs (target/release/deps/hashes_test-128e20a6c175eb26)

running 20 tests
test car_test ... ok
test cat_test ... ok
test characterlowpoly2_test ... ok
test cow_test ... ok
test deer_test ... ok
test fish2_test ... ok
test fish_test ... ok
test garbage_truck_test ... ok
test goat_test ... ok
test hummer_hx_test ... ok
test low_poly_mill_test ... ok
test low_poly_tree_test ... ok
test rat_test ... ok
test t_3dpass_576329_test ... ok
test traffic_light_test ... ok
test tree_birch_test ... ok
test warehouse_test ... ok
test water_tank_test ... ok
test wolf2_test ... ok
test wolf_test ... ok

test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 184.81s
```
