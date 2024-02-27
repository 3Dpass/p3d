# p3d
3D object shape recognition tools for WASM, which not only has the same functionality to [pass3d](https://github.com/3Dpass/pass3d), but also it's compatible to Substrate. Now, it's used as a part of 3Dpass network Node.

# p3d_opt
My fun with p3d algo optimization focused on processing larger objects (but maybe also useful for mining)

* main branch is baseline
* testing branch is optimized (currently ~45% faster)
* only tested params: grid_size=8, n_sections=12, depth=10, algo=Grid2dV3a (but other values should also work)
* objects for tests are from https://github.com/xchainw/pass3d_bench (thanks!)

# Running tests

```
git checkout main
cargo test --release --test '*'  -- --test-threads 1 -Zunstable-options --report-time
warning: unused manifest key: build
   Compiling p3d v0.7.0 (/home/crypto/3dpass/p3d_opt)
    Finished release [optimized] target(s) in 1.74s
     Running tests/hashes_test.rs (target/release/deps/hashes_test-aa8c1695328682ec)

running 20 tests
test car_test ... ok <0.009s>
test cat_test ... ok <48.992s>
test characterlowpoly2_test ... ok <0.010s>
test cow_test ... ok <8.212s>
test deer_test ... ok <0.034s>
test fish2_test ... ok <0.002s>
test fish_test ... ok <0.002s>
test garbage_truck_test ... ok <8.056s>
test goat_test ... ok <0.387s>
test hummer_hx_test ... ok <3.408s>
test low_poly_mill_test ... ok <12.370s>
test low_poly_tree_test ... ok <25.179s>
test rat_test ... ok <1.867s>
test t_3dpass_576329_test ... ok <0.003s>
test traffic_light_test ... ok <0.028s>
test tree_birch_test ... ok <194.617s>
test warehouse_test ... ok <0.190s>
test water_tank_test ... ok <0.384s>
test wolf2_test ... ok <2.121s>
test wolf_test ... ok <4.859s>

test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 310.74s

git checkout testing
cargo test --release --test '*'  -- --test-threads 1 -Zunstable-options --report-time
warning: unused manifest key: build
    Finished release [optimized] target(s) in 0.02s
     Running tests/hashes_test.rs (target/release/deps/hashes_test-128e20a6c175eb26)

running 20 tests
test car_test ... ok <0.008s>
test cat_test ... ok <24.764s>
test characterlowpoly2_test ... ok <0.007s>
test cow_test ... ok <4.103s>
test deer_test ... ok <0.020s>
test fish2_test ... ok <0.002s>
test fish_test ... ok <0.001s>
test garbage_truck_test ... ok <3.949s>
test goat_test ... ok <0.196s>
test hummer_hx_test ... ok <1.655s>
test low_poly_mill_test ... ok <6.684s>
test low_poly_tree_test ... ok <13.431s>
test rat_test ... ok <0.888s>
test t_3dpass_576329_test ... ok <0.002s>
test traffic_light_test ... ok <0.015s>
test tree_birch_test ... ok <110.553s>
test warehouse_test ... ok <0.083s>
test water_tank_test ... ok <0.165s>
test wolf2_test ... ok <1.044s>
test wolf_test ... ok <2.273s>

test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 169.85s
```
