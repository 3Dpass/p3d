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

#[test]
fn car_test() {
    let expected = [
        "bf17730aba48f4009fc70f3a260ad5593206b2ce59a6815ad35ec9261d237e33",
        "d8f09c88e67e6eaea86cc40159833dfee48916de21a6772360741f9133803f75",
        "038a353e262b9ab8879d2c5870a1509072368e7f330627128f8d9e62bfe98546",
        "36652d64ec11ffc7d959d310ca046ca05c4f3a6ed94e57e639cdc59ada5b0d72",
        "64567758553ab4f2a579a3652e61855dcacfdad222f6abbb0e3d8537cf76924e",
        "c619c03dad6ab35419fe5e57c98d03185b2fa5927ceb492c890d15d4ee1830fe",
        "114deb64f18032740f40884f6762fd7ce6edb11823c8556ee272423745964f9b",
        "38bb90eb76d23e4060bfd24d55334bdec99635402b056111d10986e0348b9300",
        "72bc930116b1c3e8b3dd1b8c0b04f6eb492ada56f52c94e3844e1d14735abdca",
        "396af033ba7d19900f14ca3b0de3e829565d3c45980fd90ced2649791ec54154"              
    ];
    run_test(String::from("./tests/objs/Car.obj"), &expected);
}


#[test]
fn characterlowpoly2_test() {
    let expected = [
        "a399c094c487186ddcab2b67216a768a2b1246c9227efd8406170ad6b79943b3",
        "2502a11979bae733a9786f2d18c2f826e52e36fd4840983b1c12055a73c22b4d",
        "1f2041a171db2e34989050ef8f240eb85bb65473768b7b751b5d5d9fb004ac18",
        "66faffc26cf178a0184413aa54ca7de3a5bc5b20ffcc59135633b316e5b34e40"            
    ];
    run_test(String::from("./tests/objs/characterlowpoly2.obj"), &expected);
}

#[test]
fn t_3dpass_576329_test() {
    let expected = [
        "0b4f312022d98d8102214833874b5b195822ec0c7b42edd991c6399ce19fc728",
        "64b525dc3df5ec5ce258230670a8dd48a5e899577712d21e14f49c04c8818391",
        "265da7b2f0f36850eb3308521fa6c01eb8de1feff4b503a0af55f66cfd9f0a01",
        "ecc0d12220d9c9d2f017ff90a5d5af6c96b0cf1dbd56bd676387f5dd5db4804c",
        "ed3d39d01319a63c868d5d1f5974ccf72699b5a20e55f61d492ba607666c831d",
        "41736d92cd93cd132da92977f7a44ae459638363d9ad7eb3ed550633b4a79bc9",
        "1394773edbbd926cc3ad995f9f1b84a36ca79eaa33914be83e0c08d982b4b3a3",
        "747fd248453fc3aa93450052c428fd3f1ea275c1ca32ec1108c66eaa17d11f53",
        "db6a9934d4f9329e7517f9ddbc66c58dc0e66fbdf8e863f1d459217fbeb9b306",
        "86b03cb4941fdddb87fa8aab7325c6c81fbda58181e6e739c07d40846ac38c42"           
    ];
    run_test(String::from("./tests/objs/t_3dpass-576329.obj"), &expected);
}


#[test]
fn deer_test() {
    let expected = [
        "68c08f90057196cda3b172d74e2de5b1bc434b2dd7ed050a5db799c80a3c38b4",
        "413df07c3b478c7e66104e24ebe46e840fad63f4abae34a67584f33c185b9f61",
        "85f77f4883021f5450d59bb7d8dd45f6722cded8321fb6de96e0f0a2568f9115",
        "2100bdaed8ec8f67abd7c7e7037138f7011eb3be37d81248f591d6ee5dcf3f20"         
    ];
    run_test(String::from("./tests/objs/deer.obj"), &expected);
}


#[test]
fn fish_test() {
    let expected = [
        "51fe1abc4920f10b3a79237911762978defcc7e736b37bc8e74f22dd15913a03",
        "ef1b79e77981944bdbc899e5e5e239ae41a7e91b325e85268ca7247cc07ca052"        
    ];
    run_test(String::from("./tests/objs/fish.obj"), &expected);
}

#[test]
fn fish2_test() {
    let expected = [
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"     
    ];
    run_test(String::from("./tests/objs/fish2.obj"), &expected);
}


#[test]
fn garbage_truck_test() {
    let expected = [
        "6d95ad4630f9964b90745083619a3fc35a706d566f62acd1358aad2379b81b63",
        "fd19ab8f9100c728421187553eb26a1a922a134fc53e62f8ae011b55901c7655",
        "300b96b0d1e8a081f41a598e99451eef8740e22d4e72a44f9d16a40e0bd6a531",
        "c69a6eafb043f7d9c1c60c0f4dbe890fbf518fadb4acb530f9cc1be194de3555",
        "a6e99bcb49a32b178e3edeeb0c8ee3105582e9ffaf91c0db26f2303c6faabe64",
        "322d542b56d05b9549cadb665448f0cbab3de94982a2dd3f6724b62b81be823a",
        "48b49d24f970f33b2fbfdb638ee96eb74b676a2b10b11ca4f97f2d5763993045",
        "23df97fe02a0bd0f234a1da5fff04ece024ad0f7f978751ccf77a15d8380253e",
        "916013d032df282c715c551fdc2fc9d3d532ab831871d959d7503655ffc19fca",
        "1bf5d3182588633cb349ac00f0c78d0485efb33abe38c0b5126f782f572b7860"            
    ];
    run_test(String::from("./tests/objs/garbageTruck.obj"), &expected);
}

#[test]
fn goat_test() {
    let expected = [
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"      
    ];
    run_test(String::from("./tests/objs/goat.obj"), &expected);
}

#[test]
fn hummer_hx_test() {
    let expected = [
        "6a4f70e95bebdc8baa5440798bee04e4f37babb74ce6289020358bef91d10db3",
        "7a42b21c56faf0670a8514b08852476d69e2541c84b922decc9a00bafe889de5",
        "93b7e0ec8a4e9061f5f7a70dbf2d461265de72df027047f5c491d3d2ece7a2db",
        "5e3d34fbef70b163ae23f316628622dbfcbc90620d597ac0721585d44352c7fa",
        "ad68b79844451b4aa80d6cf9533b1007f7c806112d24693caea9cc56393ef7b1",
        "60f28a178fc9684ab54223c1af6c9c72ffc4bddfc1a3ab477085b674fa55dd4c",
        "a9bdfe907611016387740ead0079ad81160c4e07fad4d2b225abb9b276b68bd2",
        "577652bbb629afa389c6bc19c52394d880cc37c9b61cc8c0ec52034854471d59",
        "911407692264f5bae94ec0217e5549c82e388f9e01de408c5057ae6ad2d436ac",
        "465ae3a6963786287c4c517f16937cef0b6b19830503aef369680f07bc5d3f42"
    ];
    run_test(String::from("./tests/objs/hummerHX.obj"), &expected);
}

#[test]
fn low_poly_mill_test() {
    let expected = [
        "49809e4682cb0185b2e837207a111560ed87db6330b7418d43b82c17894db7f3",
        "cba97ec2b07109e971a2f0b15ab8cae83620734722c470a9109329e8c65113b9",
        "958153df096f5079cfaf7f7ba41a649129320e9ed3227279c3222174511a2a91",
        "e9a7b019b14011d3c47acc2516007a3314efc40092bc53363a68123c4a8cf2f8",
        "91bfe139bcf20f946148bc12b8fd0bc9bdaebe00c3ea5b107db73668a816e523",
        "8c749803f921e63069ab620804cca631c83a7534c662d03409d85397f5136177",
        "71dff1aed63788e26370b8f4004cf9225a83328b912c911b241753cff7100715",
        "adda913cb741b5eb5ab25981fddd5cd9673dd88f4cb5127fcce760116516c538",
        "53bc3ec849b8a8876de93800deaaa17b080c3a53942dc5b56527e30ea498ea3b",
        "95f9935970d8446fc089dd44c8d87b94d4965b0f57acb920ba34a1c7ff5c4f60"
    ];
    run_test(String::from("./tests/objs/low-poly-mill.obj"), &expected);
}

#[test]
fn low_poly_tree_test() {
    let expected = [
        "424ca3d5dc62ab13588f034d3e27334865e0434e269e55bff9c892efd576d6fc",
        "90d664918aea7992ef5908b164b052f748ab858570ded736a335f983474ba3b3",
        "59aaaa84c72717d11d339e71d4ff89bbe070cafff7fe8ec6dca17443e8ba0e71",
        "2138189f5f6c2c4202c274ec13aeccbee8133e350d294112bd0647bc6fd18150",
        "71249f38f40e4e2dc296d0a9c164776b4e2e00d837877a9e1ea5a250ad01087f",
        "233def335ec59c7a03261187c45f7cad2a1e36ef8bc9f3991c1f3f561fe814bb",
        "5dbd2d79213828c55b6470b8802d982001da9b380ed9aa746a611ced2798e2e6",
        "8cad66fba5041e5a8fa5e11371dc1836ed2f9ff8066b628bddc67b2f6cd6d7c8",
        "6cc37f682204e24a52963c407b1e0fb73f9e4bdd4e5f1a2eb5e086f8d292ee22",
        "086c978146913e9f5da78a9a2472518749a9df1b586db6defddcce7a83ba5777"        
    ];
    run_test(String::from("./tests/objs/low-poly-tree.obj"), &expected);
}

#[test]
fn traffic_light_test() {
    let expected = [
        "490181da4bdc032adea3286fe507bf92d5c09793e7b0db66a11a3604b7abc17f",
        "14c523a91ef70d892f2d94a702b816529b045287f042598651c1197018ecf7a5",
        "fee749b8baeb2a9936a47eef8d6bf9b43a8aab3d6faa4b05c97c2cd55f1a9161",
        "fc715a77d94dea668541e289b62fe7bf6d962a10b196c944dab55207435b75ff"  
    ];
    run_test(String::from("./tests/objs/trafficLight.obj"), &expected);
}

#[test]
fn tree_birch_test() {
    let expected = [
        "f92c7b4c15b81fa7f2015471834de515315f2d03d74ade8b1d7dc288defe7d0f",
        "4e53d906b122ca3e3e33f786b7ed51cffa045283a0d3432de038d723fbe23911",
        "2c60e2df649fb62ba54d5d2f40a740c2ad25aded2b662149c84da440a603d34e",
        "41122b00c3c385ff083c79fcb2369272cea944329124ef37d05699e9177b2013",
        "2dc757734c92214efae863d04fdf09517c89582ac242e0c0c41ce643ddec6eaf",
        "c605212686ca9b1e03632ded3706f9a0de2d676dcc9931dae7ccc59196164e72",
        "8d6fc1a718d24b4b536236db4017e9849a2e719bdfa9410a144707d7d08183c6",
        "42f38b68e77c8a0648f3cb1dd5ab53e31a8bff9a7f7264807144dcc9d31ec40f",
        "65ed1451eda30178899af6bfcf29dc9bc57c3a2a484caf13c6e26be0eec71f1c",
        "2a48c226d71a827fac959f992d8c37c91df735b73104fcfc5a37ef97aabe8c9e"        
    ];
    run_test(String::from("./tests/objs/treeBirch.obj"), &expected);
}

#[test]
fn warehouse_test() {
    let expected = [
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    ];
    run_test(String::from("./tests/objs/warehouse.obj"), &expected);
}

#[test]
fn water_tank_test() {
    let expected = [
        "279647fa563293fa945b379d23c1de0523bf12d62815b6379c51b16334c08f07",
        "90f5da729d0f3ef653909f8059dcb7ced2d56b1d3ad3f9c2154217ec46f4cb86",
        "7449eec05712301846a5fb34152b089b9f68966108ce8f09c8a61313575fabf5",
        "e71fbaca345ca5aa6117024e6030822362a342ceafd47242c467c8ff1d944835",
        "6a4e22359f59087cae84fb9e76d17f24ccb2c55eabf29110018e5f31c20130f2",
        "ea8da99ef8bf3dfe07a2919b19d7e699dc534cd79055937e68d2655b3b799d99",
        "544e0276d47f04f0ef59988299ec60f6fb03c6f01567507636318151d86f95dc",
        "abc76e1fde6b21743497d65ef46c6497498d171d9da58c335440d048146f4568",
        "209281531b59a9fed3f7fba379b00802e238e0f3c981ddda64aeb7258c59f916",
        "dab8cd8697bd44176d533607ccf579e86d8e5d7c0c2928ad74bd771266b81a5d"        
    ];
    run_test(String::from("./tests/objs/waterTank.obj"), &expected);
}


#[test]
fn wolf2_test() {
    let expected = [
        "8c8558f3a2f117f9bb1b8a680f436cf2709a23055d609c3605aa9766b8c7d2db",
        "c55550faf3bda79a8fa475260d3157c078ec2d9f6220f41d218d7c408fbe5eba",
        "bbf75d5b2f3269cccc32b5f67133bc3d18025207e64f703d9cf700f4b1504784",
        "a84951e98673573fa444ae6558f4303545165a5710738e16736783a606532312",
        "7a9c750a1f2ca40fa43c6467a95fadcebf9ab5dfcef172e63d76c77d2dbf3a63",
        "f9bb7d193ffe559d1535e924f7da1cd7158bcc118a5e7ed43609b86d9163b917",
        "91df2139a784b2e6a863c28662279a0f82d5b447d72a334cecb4b6f119525379",
        "47a67b0f40b31fffa8b293bcc7a2cd276fd0dd4eabbb821a667468e39b1f3cd5"      
    ];
    run_test(String::from("./tests/objs/wolf2.obj"), &expected);
}

#[test]
fn wolf_test() {
    let expected = [
        "db4178981f512f69dca6785100e04da00139ad7d2e9160db39a77f2145364f46",
        "0a18b8305e930dc99eb082f8febb32958d1a402b606540f1955c5f2bbe3097e1",
        "b05c3100f9322d186f2359b4f8817dbf805e4df3d11970533c51c9509f624a2d",
        "1de8602f5d4680cc1ef4cde77a64fff652a7671f3d12f9b5c5416adc4423c23b",
        "a7b65f0d8cb7a594f257a3c578d57fbf3a84749adfb89f5b1273b244c5e9e34f",
        "26bb206fb8fb8bec9de722db36dc00bb60a52f2522dbc7abe45310e80815f320",
        "3963bd8bd85ca2bd200f8ca9a2918a190164c329666ed2c4bc5096293f4a6fd0",
        "c5dbc69ecec9361e53967e4c62ecba04e6a61a38110bb9ac8321d9afc9dd8650",
        "1f86bdcb5e27e58ffd74692b4ad0e6ce999a821b8801d20433eb2d3fb907633d",
        "cb729c119e95e27977eb99e9773b7d928c8a0f98fc46d2ff2b24ee1c6fb46212"            
    ];
    run_test(String::from("./tests/objs/Wolf.obj"), &expected);
}
