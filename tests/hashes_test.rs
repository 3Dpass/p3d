use p3d;
use std::fs::File;
use std::io::prelude::*;

fn run_test(file_name : String, expected: &[&str]) {
    let mut f = File::open(file_name).unwrap();
    let mut buffer = Vec::new();
    let _ = f.read_to_end(&mut buffer);

    let input = buffer.as_slice();
    let algo = p3d::AlgoType::Grid2dV3a;
    let grid_size = 8;
    let n_sections = 12;
    let depth = 10;

    let res = p3d::p3d_process_n(input, algo, depth, grid_size, n_sections, None).unwrap();

    assert_eq!(res.len(), expected.len());

    for i in 0..res.len() {
        assert_eq!(res[i], expected[i]);
    }
}

#[test]
fn cow_test() {
    let expected  = [
        "0a473742b521840065c0f77370b775331bf6c764289646d63916da0913064f71",
        "2184b2068ca98ea2f0a8911b1cacd1ef462207e90bde6bcd8e065af308b2fd15",
        "a7046762d381b0ee5c4217a92f999b8df625d8aaa6c2d328ea9b090066bce00a",
        "39d789a044ae5fca9ee7a7063f33a7becd8678b62e6633f57a7c9fb97cea892a"
    ];    
    run_test(String::from("./tests/objs/cow.obj"), &expected);
}

#[test]
fn cat_test() {
    let expected = [
        "25de28689529b5c90f1a436d1deba2ea922f010d664ed0a3aace52fbc78d6ff3",
        "9681b028f805d56aed1072dcda88e0bf4ddcded29e0e779684417c36fa7ec199",
        "494bbe45ec0de352c7112da3e4ed33c0c12de02108313e5759735902cd29f255",
        "4367449c8fa11d23f5eb6a16fc1522cd349f2aee20d472e4061fb180c6600898",
        "2899e8b044aa3172b7392afac4814cbbf0be69f8da12dfd7b14f8c239d917b30",
        "185985bacf95e4bf0cae0b474a47d0e6c29b554cbdac44def39fb678a378130f",
        "13581906c48e487de0c6f3c45efc69c626ad61bd6d9a5b7aa78aebb677061b38",
        "93878a8477bd37fad8b89d0bda94e3521900518d0961d742a07964adb7fa6476",
        "77265544e0062ec903c4b34554b7a4ad5fdc34133f2226e7a0c322e364b74499",
        "1ebe6b533401c264cfc1a197f3a3ed146ced43f9145af79e4d9b363a63f79f31"        
    ];
    run_test(String::from("./tests/objs/cat.obj"), &expected);
}

#[test]
fn rat_test() {
    let expected = [
        "b9192c327cbd1a6c489e03ed097abafce2582fa619f32eb94582b92b5c8ba926",
        "0a7f7a6aae5d71619174bf86f5296b1d1fbabd66215d50ee740fab6e93e8816f",
        "f8ab1cc26829b5c9cbe1af61cd17b14ec1ee868a4526383b6dc64a9cd4ccf6ab",
        "388a1e86a920f3238b0892717a4be7ef6f0c9dbb5ed4c86e73bb801cecc1a148",
        "572273c92bc9221f757c13791f3b1e1f5b36e3ac026f052455d685409b30eeef",
        "d28adbcff8afe16869de43fcec211a8982262c76cbecf271e22ce3fed5e0419d",
        "c028a542993753082a1829ce77641cc2c6d2fbd9d32dc3bf14ed554280008184",
        "174aa286cfc3e459e5a3bf1fa120afe596c1068900d5d24066bdbe62ac925e9f"        
    ];
    run_test(String::from("./tests/objs/rat.obj"), &expected);
}