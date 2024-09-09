use starknet_crypto::Felt;
use swiftness_field::SimpleField;

// Evaluates a periodic column at a point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at point**(2**k)
// instead.
pub fn eval_pedersen_x<F: SimpleField>(point: F) -> F {
    let mut res = F::zero();
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x43869b387c2d0eab20661ebdfaca58b4b23feac014e1e1d9413164312e77da",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4cec4cd52fab6da76b4ab7a41ffd844aad8981917d2295273ff6ab2cce622d8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x295046a010dd6757176414b0fd144c1d2517fc463df01a12c0ab58bbbac26ea",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x60105b3cb5aab151ce615173eaecbe94014ff5d72e884addcd4b9d973fed9fd",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x233eff8cfcc744de79d412f724898d13c0e53b1132046ee45db7a101242a73f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6f753527f0dec9b713d52f08e4556a3963a2f7e5e282b2e97ffde3e12569b76",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x549a83d43c90aaf1a28c445c81abc883cb61e4353a84ea0fcb15ccee6d6482f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4259be645aaf0a661e7877276fa5559ed7d04349f577595702efed3050402c5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x71e67bd6a0b1b8518cb06837a78b92ab3dec98c4989f946285042655ffe516e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x19b7924c29a944ecb61165a663d76d84e5ce44b4617fdbca8ff02fbdea6deba",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x70454f9541d96fc1552f984330389ff616cf80eaf699ba2e82b77f43fd163a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x479c09d33c38f1c8f73247aace507da354ae87ca5cd4aa096bd3a6229e3006d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6fcf0e32e3e99f51d8cdac9c19cc25179eb97f2757844fa0c72e7c3bf453e4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4ccee6b6ecd4ea8733198e95935d13474d34cf54d7631fde59720e40378e1eb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x627cb37206e5ee9da20c04a92cc765e3bd3f3d4e42ad4de0d709f366d446d8e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4183c04ef7d778f11e57b44c1a7f354c4497f1e3d420d3fa9f9c27c4bb58759",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1e2c5c3fb2b47ea8cf33099c610f6132a5dd7099d29b02f4a041fe5947ff53b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4d44944716e0e13728fa8b84fde421f0f66a120ed2b7cfcf59f5ff6718b8b6c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x394d0eed011068acc2f55f541c4d113a9c0afe7269cd7d9711aa7e8be661a60",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x52b5bdbcf28603ba60abcbf52bd4f7b4988ce0b4e2346e4875a3f117d4143b4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5975b93cee7a147a93cc98aabbb713f151924c4ede3306bb5e14e5e4d5d5c05",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x787053fc3649b17965b9e6ef5e05e024cdc188e90aef1cbf13ba78542a0407d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7cbef72611c8e1e08e52ca202382a8545bc7fe124ec080058988e45771e3b40",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x32bd55700baf7283995407f470139326a670d60a5d5428904596584629a053d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3485ad12aa365fac51a6296931abdcb54fa848c587cfbfe5bdbad2d6f6d3bd3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7f9850620a3435695ec7a6d9378cfe218ab0e5fa674cdc572fb9c197b0dbd25",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1c7c40b6e4cd3d473e8f84b8fa63610ac6c7e3f4f0017f3ed84eae8f042bf15",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1e3e1d970342085c482175cf60d93e1cc2cf96dec12f1d839b9b829cc957b7d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2d1547488e174e0a8662decd2cf020dd40718f070c84cf36bfa261aa90f814",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x31b8dd40040d22aae383c1e628e427f7aa4a7b0c3a83f815fd7ae2b36864af0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5efc7dfab3ad0b3f01e313c50ced95363d8dbaa9f91f801d6f1f00869467a16",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x724a3072c9f315cba63e5d99034b3218ff29a9bbf04155060ebdd6c848a652",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2a0c1eade4037c10729bdc8a8f38bb5bf359078eedba633047377a09b6cde4d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6aacb14e31ebb5066e78eb597842812d7ad137880a6dd0d065c4acee231b7c3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x468b88ba32ca1eac6c8d3196eea0561e25770818221ed0da3ae749e2a302e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5b52f487f8c3d78fb6ea4be227325a7386c7e95cd5f9b72710cfcc870cbba59",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2bec10dd6a541c12555ff040b5949407713b4227867f53a435e80847b7932f0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x41e1c8870eea4b7f4308e8173f97482d80afd055f07b1a058f182a775aef593",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x126a03f3c5cbe523484111d915d6d7eab5edad02a327a383171be09597336b4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xa8db86341624832893780e36fe1f60490da5768f9aeb2a5803240f29ec5a2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x751a8c7382e8fc0141b4ec5bf37fa457ab8301640b58cfc3b6a0b8d1a12bec2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5e56f3654b68256095b54a7868763aa3ff60a98ea3508039def82d2098d8a6d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x60a9a4ccb72bde44d8a6c5f1d7b9303cc32013ef621bce1b8af413f00e77ef2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4efe82f8cacf9761cac9fefb6c13c1afdaed68ee650c37684bfce323070e480",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x40c43992a86359c71f5b8051d84d1fd6971eb36ab486f321a1fc50a52a02a44",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x538392c6ca2c04b5096aa69392b76ff109aabe165df488f3d1a8e5c4022db64",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2b783063fa1948abfa91d79d225d52ed2ddd11bf20fc388b1ec00fdb5867921",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1ec66f3f326120e659b78867bdfd7dac4dd3f1a92ffeaf46d39725de341afd4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5f4ed6b86202d76686a0b4c1efdfc93c46dce1b843c7181d1db1f8cd4d6dfb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x34b9f6e8d5debbb4aea334310dc8d8075f896e7eb9f1c09788c7ec62ccb6116",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4e57c6677d3bd56b425a3b3a92517344d4875e1710667e3dee1954395269af",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1c5110241881e087e201d211da338d8377dd228afbd84850b76f3e5dfeb9361",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xa64b536ff29309d613af1c27c7229c3f6c583471c6b589b25026db08d3767a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3f6f9a9c3f6e175b59fd8e4268a6ae5734034fb1d7c43f97ef474b75ba80cc8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x775ad15685181f15e34a6b0036c16fc8d1a9860ced1cc5ece39d19a6add939b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7775ec7b0ee9812c8df83957c5b46c316fdac82a2d736d4a6eea6124abc5849",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x41f320f863037e381ff83f2c9f1a8ae2802fc22cfea674d9cfd10171da6dea8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x67649ac75ea692acb3aa4432d48de15aacfa347a37afdf489cc7e954e4ab100",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7fd8c6108133b8109f4058192bd614b5de2c50afe7ac08a7bb0e0b12ef04e4f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x57f8ef270683ea78b167dcbe5bb122a79ba760c95f8103dc4c6e7788fb1ac9c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6dc0d996fc95036c8cfa408fb12793bf8a4773d698f55085c2ccbc906c6d2d0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6713ddde3f2da61b676f5e4c52177bfc8c1576bc97ab3c48f08ff02d26cc03e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x16b450dd2bb4712f6412b35603aaa02e7345124e5fd13e919c269f3874970f9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5f81b087ad750a0ebddd5239bb3682c84d88326b4679a24890f5fec98df45a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x33f4151b710663772765df7f95b3710c3e8e81bacdbe3729b0a43b6d19e428c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5d122cd95f43fb6fc2373ef7e66072140f0f20d552f186faff2622b55a3e063",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x9b478a0767cca2c6f9b4268bffc9e907eb69b32f8ff7b43fc24edd38a88ec2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x597cee65bc7c6f0faa3e0aa1958897acf7fd4e4e69569f5d18254b0b8c09aab",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x218bc11c668ef7ae5f04a16dc9933c5bc41c194a439d0af802568e598c54630",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3394eac0b3787b323686cddaef3af972d7fbbd75940bf7f682b8fe3676cd46b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4aeb3836ccb2a9ebf9f1c5b6ee3c42f66c8059cc55188335a47a3583d986018",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5db68a5c4527fff0ebf61fe064888b0fb6e277cfecca6d206986293256f31f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x63372394d373e7a2f2fa6405509da05fe9cb546ea2742ac0716bccf50ad9227",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x22d4ed1a29943bc16343e01eab25e45adf74b6a7072e4e26aa8d141f2cac5ca",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xf8ea3b2c0b72747301b2778cc071cb9d2e09bbdd7a386b7931582ab412dbd1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xf78d4b72e0f5f55913884d0714674dd6f534b211ec5dcdba419347828c7c35",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5142430fc3f872dd6fefb7e9804e3e63714f71a2f43b155cebc53671f964af",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x72fe5010e70102306b21cc388b7f2ab8b0324b84654cf98032b83a81099e72e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xfd959b09bb704fe63c73e2331f8e76dc1fbf85c2dc9dcaa0e8108664f7f988",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4180556f79a47df725eca2c2f65389e27281443847a7d9e84640e6d589182f7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2dc12726f7f06ef1adfb10747e5d4ef8052e4e57bad9bb10529d7994ef91035",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3c62720cac42a262b58765d7c0588231c5c2c9ce9d48f0fd547575289ede8c8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5744178090cdd56ae12fdd51b74bc097f23f735b7ca16e415a1854597b1caf8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x597cbefb648f47e763b9b1be8c3f815a0e8b65d0101e11b5bedc380c10e9f4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x78e66ae8b3ef57289d92561dbe4ef72f4ee551d5cad363720a78d104a89163",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3cb89319d8172da012c036c40116fd325d65af69f80a1df8f56ec890e920592",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x29d152196b7ea7446182efe778a2db796f5fab17286405953476ac97f94a96a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6348748d43d48acafb8ce688f25a1245df86dd20c3a96c5c85cfc0960ca2fa7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3609fff81e15da2a88036d1c2d28814035ce829430fabcc3986c08acdc2d44",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3241fcabfd99b666b151970558fb59fdfca47ded4caf2af4b15839767edb190",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3f61241934753ca9c4f4210885b87863abdc8637d4dafe5da4bfa5e0206988e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6a16c0b648c72c8d718d53099cb11725ee09fe1b49487d8f55f307a6a265920",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x43ef0fbc56a0a46c7099f5e6d6550a77e1ac023e2201f01bde0a3f5fb0f16a5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1f6bf768424619cc2d34c01cbf4e137b6cc33a4a5a3db0bc704f790f86ad67c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x49eac48d453d5de07fe3f4bdb5aac21e7fe69858afedfbeb0daf175459dd9d7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2a390a6737563e9edc22b0b0cce94a67adc10db18d6f978c826f24b8848c6df",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x23e127bf290465acfb7500962d426be5241f0e8c6f844d25aa8e262df6e70cb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5e51a00b7437caee2acdb81781212bc3d1c397b477ec784d1a7b304c9f8c687",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6e52308f62433fe92ca9064e06aa17d793d3ad7bedb9590c8bb9edd3272fbae",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7aa1d2348e13a031dc4fa20d453fcd59eead9adbccc3ea64997d09a0f58216b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1c02d3ffc30c7172a132ac604ad28e89466845c139dba509b896c997ee4ce8a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3ea018d81f9118cb5cf251d6c795b4ca4aeeb28d6ea5464fb4807d219453728",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x62c2fa993dee607ef195fb6620051b4df127d933de3a417d21de3b0c6dfdc95",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2fd6fab5c4d0e6bd5bf5b950632e2dfc3be19c9a80e3bf8934e878003b0816a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x12f2b0b280b64cb9f6bd77cd5103b7668ae42e5d40ae156607c69043b4da5a9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x51a3ba83e3f68b2df85f3b9e770b5294312fd634fa48ace215a029fdb5593",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x56f19df91009289c7f5304026cf6d2c26541cd4caf867b2d2ea8a954560ed7b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x362dd19b8207511079a352fad991df9582315ca2539ed4da5cbb5b82e414fc5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2dd7ddf328b439b3047a93c6fff6ef901946438cbb55a4c1fa1848f80baf2ce",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2120274511adcc680703d33146477a31c42684b5163a628eb3f84258ae78786",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x11076126b67298371103d89e76ec2fbe30b28c5de422e61d3fade2e190450a4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1868ebbc59cb1c69b32ea2b3a7ce3f87b680731b96a42403878df0a0e4bb3e2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5b7abf66fda1917e0e1d44924cb73d713b5fc16b3a64bd4857d089adfd6a814",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x13df9c113e40f246d806089e437629de52f8a247ece912785004efcafd4ea94",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xbdcc31feec5ca8cfbf7227269d1e120132c51307ec03cc2d59c471e2510a24",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3b2efe16624d8d0a1beeb037b02f0a4f7e11eb3859852cea1f83ab1752a4099",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x596104fc8bded038e39f0de5e80a2f2b65fd39fa4ab7b3453bbe8a40e06a317",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5fec9e8e9ad35ec1091706f4f39c0e8a610f58be6c987c2327ce0794af7cb7c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5fb45888a7861e18a320bff7b0baee50ef9cbe1b06c78a5a16a6fbda3c6b77f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2c15afc87ef81cb58ec29c7dd81b4cfe291e5d33a7b36126289a8ebc1af4eb4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1f90c3eb7ed36bec79f803ab1884e5455581110ab713139cdb5207561a89a34",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7c537f749e37ed15d7e5d5d0f88686c5d02242b6c487ae2c5606d2c7de986b6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4db0795b76ed3b5cf3cbc23bc47d20abe9b9f76a2731f2774e6dd5ecd6eea05",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3a7ada56cb16708c6eee7af3688765728c706a16baf61d0582186a3717ef552",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4c9b9f2c154c6a8cb1fbf50793787d215f2857d042b21c6f5e2740732cca567",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xdf2e87cea7f46ab09a5011d8afca4e7cb962e008fc991ea16d85c472dcf3ef",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4df83db997cffc8598b838a9c8373bfff5e109d71ee3bf2a18dc0e621e93d2d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x17eb7ae4a950bce2abe1e7165594eaa60be7b75cefd8007425a735264a1371a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6c8258350c092e7b5cf658a6bed95d620afe0563482911a1435a93bcb0d5beb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x554563c23e6ec8a4497d670e81940a92ddad53c27e7bbc18de74d2b3734d824",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6c9568c4a9f64874e71c88cc80576e4083f6d0649f66929612a9bb99bd958e1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x14e46f8471beb6479fadac1286dc86683c659bf1c77dc96bcb303d48c115d7e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5eb4774b76a39af609243ea0ca61eaf03255a02d90be9a83901debf64875f0b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3317e8a32e8f82246423237d2a4039eba358a76adb8065751b6d7939fadb85c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6cfe464b2a4d4e77c09e0beceb4e368bd93aae5efaddbb92e003afc508fcb33",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3b69a8579df2cea96435a07c81ae1d9f8a5e0e52433335c3e7ad81b76789788",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3f26981ddcd3549baf47e3f1242b0bb90d6b7f426ba71d2ce628ceb801f3734",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x40d8fb43bbc7e5c35e4b57fef4e8351ffb118c9d92346f97ff7cb48b0170eff",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x19eec9d276c006f19cfa904a4e2ead857e99000d16e897dc8dc955c57615d54",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xcfff274a78e56ec27e29d01f2e900bd226cdb493a83358f9b807235c9aa407",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x13d4454abb9515f00c3daa6034ed3759ea722a953679c4f857511141b87da93",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2ea5039159478e68762063624b0f396cb7f1bbfe8c1a159f65f0f663f219136",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x40508ab9b5b8d885f85750bb659071d6cc04639f43070b94a802d41723bd0f3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x40a9f47d93280a641e7f903b1e608cb443ed5d59f24cde6b92c6631cab1e009",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5a5085cb551c472af264b5de50ebb7b4bb04539c9afac1339f903b943578eea",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xa89bb9df4a46c56f2f40748d826d50285082118f8995f5e7638a05ec117c47",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5950e4370508dbfa764621025e9341994a3ac21848f3e39d02370b193ba6937",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6f20da2f1a25f1fab33e7856067226784ad992f8bb53249ee7bb17e86c82070",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x42271e06f205c1bfc9f9d9411bf835f43941c88aa3dc75f044a0143faa4d5cc",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x411f9def562556de87d47af60354512d9a1261152e7f4636038699d468fc2bb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x66490371a5dfa3fb85bf3f088b89614b5e56cafc263eec39dc4a1bb39e03433",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6d49bd35b4e4aa46b7098d306632014b4fbfd84892d6997b58d9463a0ea2c05",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xa9c2bf87d58d3f72d985b4b1129f0a1664caac1ee26a15675d1a5086de3a79",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x46df5faa750270394a4253e63ba3e437550ee216ebf8ddbbd7304940c85ad02",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6e5bf767f3b0646dc16377f3bb7c17db6069555e100dd2215eb20c4d29fb1c3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5d9acf8582d4ccc017af36a8a9863e4383b63893d3fb5d81f7fabf4ba3d1023",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6933ce3f88628188f7a1b1be5b0506dedadd9559c4766be0e7db1ace3adb592",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x40ba0e2f504aa0e9972018d91be21f56bde16361282915563796c750f8936b7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6fd73eabd21a86dd8094dd0ebb5924b1aab0753a0d251571ea93f83ab4bd519",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2761e32194ddef695d1837c8a3f48a3773ae392b5633bfa0c1451e51e33b69b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5519091c464bab5646294ae41d087ddcef8bd0508a94a07890fa07220bdaad3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3b160cef807b72e95938852093a3a633e72b61e0afad5099201885b54be4098",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7692b996dcfecd35db6aa22de10144724c478f85a328ab893c6fbadf43d7a9e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3512eb8a3bbded6fad1c19190d857629efc56f93fb4aa527e2958dfcce12153",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x591ad3fb7ab83f8d9fcf184ab793baf3db128cda0de1618932851108771cf0d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5f7da39edb0781ca1f96af191cf4c70fe0c121b7b2c92f09b49503bb070dc99",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4a94669a4901cb5527124a2dc7ff6c278d540da41a95e819d0ca10269f7b380",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x20a06257ccfa90a74adf9bb1130a8385b8c91bc61e18acc30843463e5abaa2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7e3c052c620ef7fcf180898d28e39348e96e92ed0634dcae3f5fc64be5094a6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x578dfc700a95a564b41ba8f33b885ad04209bf5169a4046f603a3d84f792d6d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x796c9b073e2c56f55601eb1f6147d028553275e9fb792f0b76007c9710459c7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x8c2c75a2fe00432f77ef57e906f264ea76c439e0c4cb19e87867a6ebb34d0a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x525fee2e2cdb7a293f50f630a840d5cf5f29a158eadd6fa9d0159951712d19a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7991462c103abfc3bc31427227b1fb82f7fdf2be1b39316f46e3baef2fcf588",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x46d1e806178137e82ea97c54d8c15dd45c2a9a0082b18aeb9f849158ffc0ee5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x25a127bbf961fe2b5bd9facbda706223206c40acff003152cbb3b28e9668030",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1e3d7c65a8f40b6f8aab1635e3b78d0f798746532f08771267a9b6149632a5a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x25fc8ba8ab421b6dacf2ce03263e037374e4d61c6ce26422fbcb2e755c0d9c4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x48294f41052135cca94fcf88cf236437b8a55370c3de81fb0d781aa7b0f8eca",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x8c5762a12210a7fdc96a7d3aa966476d3b28650e7c49fc90f95e49a80d4324",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4ffe8275d3344b4ae2f7d9992d68598e50f365c0b8a721d723841485fc25c0d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2959a6947bc4eee0135bbd0a6f2053b62317a1718bcceecbd507417d31e8806",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x77efd8893058f8e00863205582a5e274c344b9af63b9c40ddd92c97c33b52ea",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x73796a0ce0fe851bc22b99faded48a24a21745bb62603e750f78b854d7c32c7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2d2e43c0ad60d4265774479258211274ae32b5e151aacf6f8ac1b7708076f09",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x22b5eab11c9e1e6b8d64d5db4b12502fdf0899497f72ee1a27c8797b617f76d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x703c768145191a10344e5ca400be8fd249e653d564015d46fcd7096cb723a0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4159f8056bde7fd4f72615f7bdd0bb6408256b8b216ca52fee253113d9d007c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5fde2cdf0d23d5649e3aead1b2b90ca0309715a029654e8984e43de7bde7b06",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x50372d2aff2ebc566505a564d971c6491095e009d9887899aee0b5017fcb877",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3333c3d925d8c58b9e4e533531e93046039577cf0e57d011c7ce87c6ef1a835",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x575fb11a4d7e3876ae4c86b80b4b9530e0d3e9db218f4d5644f612348f8f002",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3481879ec47fc8cfabf38ffaa75311c787b7006e7f9def35e96454263bba4aa",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5c1e733995aad208f0697e4d2a6e28bec9fddc3e30bd033f2f50a83927baef1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2edf44b1f59efb0f36c0fce5edbb7576c89cb9f191300fc5e0240def1b88b9d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1d46036b736e06016c817d2b51a0918189881a4f1b7c71d556db583df762d37",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x258ef77b90879282ccc2ffcea5052cad266d77b75db36b7996e5fe7638e9b00",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4219a0a13e09662f3ec712da51b36967947f6d5a09d8044e3005a7f0ab45915",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x18d0f552fd62f81b6076265c7a3a0b81f6bd37152a2f16c71210021ecf68468",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x31abb6310a44d65ac8c308011d4afab938fdacfbaec14c62b808452310b799b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1765a9eca4f4551f177b35089f8befc808613bbcd971a47d485b1c220d0bbb4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7fd44af0ef24f061aa7dd5bbde15098dfc3721790ee9bac2caa71cca714ebf0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x566edfbe3c59cbd43838ee245edfebe292c7163f79b1454b03ef3cf8af23c10",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x14d01a0c81aa61c5a238243e78afe80e5d0d7bf528c3d05a343d0f4470d2b0a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x38445d5f2de7993c48c9da8e77a87dbe289dc0428b1e4ca87e30b2376535543",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x324db878e3842c25a78e94453c98434c54b41955db62234b0ec5ddde6641556",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x184f23c10c726d4a7036c39466db02c4fe7c3d40bade571fe07acaa282f4c07",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7fb1cbd7a48f2d44a148bd4d17ccd47c438f4f1b45a02945cf4312afa0d6f95",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x20d2002cba899acc7d333031e0977d8df94557ca0749bef6c38b72dbcd462f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x75bd5b63008c2e005df64ca189ecce11c060f0df6903011a3d95cf9f7b48878",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x548724b5683cd6427513b4c4f84a6d888b9a03843bc0dbdc501b8752d99ada1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x38441dfe93fd3133faf52208f3263d4ecaca0643bf9c9d4bc952c86cf280f7a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x182e50e36b753ff5f95f2bc47a6aac8c6f2e5c3975476252a7c29250eefb056",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2593b010eb6fe0f64833e4f22f6854c063085e0dd393226e6b5fb20ea7f432d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xcc060a8b007b2dd0efa786afa5edcb512d83ddcba8ed69c27ccef5769deb23",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x672717b74bc3dca9e53494681a5ffa02edbb0290de1c5209843a16964df7a3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x53774852c8f84d21eec107e1da7a2ab3f4b5ceba6479d1f902ad404e7dde329",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x96de7b9a7eac739df4d13902971804aaf40f5559d18593be0daac0ff86c636",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x725601ed4fcfdaa392b91e8ea982fc57f1874378ab8d6b55301b3d4b6efd802",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7bc4fea0ea687295d72735a62a19c1a160a1b9a19342717b527f94770aca77",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3dfab578cfc7a1581212074e0969db9accb619a043dd7194a253af67ef3698",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x44090861421dbb6b4a325a6832e02986be80f7ea475313ae01a3215c3510346",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7384363b4495aacafd81d0a139a66afd3243309395e3444fb3f1496832240a9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4f6ea70b9090971f8de7071f27b0d036b112211403e0547fb7b7903704f295b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xd55dad93e837d31e8f120398e09b83ca68f160c16043e1c65d033a19adbc30",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6cc09dc2faf0903dbf5121b97ef058300b18efcc30c25f55de752d395b568a9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2b2091e41b10140bea196a1cc28d7f6db6ae1b55d1f115d882c321221a32eb4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x11b19b3abd2b297728768027b1370566bd845bfb6f49197a76255c1d8c661f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3236ab2e0e0b1b013c2100283e36fe75521bd50091f1c73deb165e86616d80a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7351448e92ce6914278e73ceeb080e280c146dbcc21cb35af8d2c7e5560aa7d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x100a89bccb889f183c2a6ced12bab8ef86403230ae6b23def0b784f73ff296f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x31e49312e1d59acae36bf3562443259500039a7a77d9a57a44cbbb4a80932e3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x273647256f95d2e5f98bd7830191abd89dc4ab241fc7fa12b27e16a6bd423c3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x424396bedfddf4192963ef0f87b3989a99f277fe2c60756a4a60fae4d6dfa31",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6e3a0355459b8b7c35837f3f19f0d8954907326cb08d7d084f2ed0f4b2af8f5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2d88caa65f47db103fb1ca354bf50c93f24bca5001598f716b6c9e5c51d1d2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2dca037a615e8cf99f8614f437e953c5625b9b57d95f16c174f63346e31c5da",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5800bf15808a39f1acfbb193af1ab0c22f18d9738753bd3cd2aeea81982409e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4ad97a9b0ab95abc1b8fcff31a48e18fb2391ac95baaacc62125bd87fd75e13",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5ba49d41f62b6d6903fc455bf02bca54becb6ee7f39650fcd0b717ac396159c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x959c7bf3885d75ab3ca9480101ff64d62c9f138d35f63c137009c1b3eb39f3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6da06ce868c140c8ff9ec1eb0323fe2c8b35b46c8d4f5a27727450e87ebd906",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1bf01c19527dd1d9094c44e3acee4d1ec8c4192026b6f996776294cc9dbc4a8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x40d1ae7e7bcddc520ed8c0fd736e9b5147d278ed1b720abf76439377023abea",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2f26fb29017b5ab80328de8488db547e47c44c0d56f30e330354d5b980e50ad",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x72bcdfbdd09f13eeb0c01565dc6a79999a9642dbcb0c570e3e7621ca94df215",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x140ed138dfc5b5417b25a4512bb991f3fd04cf750e082fd4fb82cc15b645835",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x40ffb20c2a3dba0a0d8b6aa51ccaa1b690aa08670ceee556d76053cd671d522",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x283c74c8066141911634401af10106c29dd77458d059ff3b2dd7aa796b2a559",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x593df80dd238cbdc6398146502310a5cb459b0e7d79fa9bee5cc389385c95b3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x669bfada09faa64c005321d60752662598d69c517e9ffa462dc1b1af42228d1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x70cc78b821b198e72f8feeb8f31d81e5a4854de3575a62909e0bb51cee921d0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x68539d0ccd1737a8b2e540f9165638f86f6c4e44943455d311999b0b3684b7d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xbb867c323532bde3d5b0e08b1b7531a95a2a1706132dcd8ebb7063cd1b1bbb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x751a2c218f4feffc61e90939c4d2672a263d3b33528c7c6eb40042640f45146",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6473d78fc37e48379ef8a9d57e3e92cf4fdad3a1bcc170dd177dbc51c4dc62c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7218f86344ea46cdcc372a22a14663105eef03bb0de9da9bfcd10818d36ba28",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xaa17b17cdb757833dd4b1670371ef55345debfb2c1b6bdfae64d8759e04349",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x38cb4173a2b057da41d5d30b55f6d11f25effdc69c14843cc43a9ab269630f0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4350a29d7b4b242b20b68f6eabd75b758d8631c192b7da5032181b71740b96b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x42518069a18922e90fa2fa8fe9bf5e2371a40ea88c25d247e6a73a007105dd9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7f43f7128a1b46f8ab168a06df9d0cade82a3193eec2d51e2b83f4f0c7fabd9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6e694d9385207d7cc8a7cdbf90eb4ed3be49cabc0e6b8d0e69172d73f4a5c11",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2782ab60e8e9c6cccd40f438a2d2814ef39f50f02bbeb790bc6df78d75af42b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x259f8eeaf6cbdabd37b9de029661bdcb219245a7599207d3df08c7cc452a13d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x9345d2e4fc86ae78c4879ecc3adf9e6c482044052bc3738618247b60f069ad",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4419f27879dacd62144bde4f904890c6d5b312282335a57cf1b04b403bddbea",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x581755fd25823d2f3b07ac5d8dd1bd5b26eab362cec3f9e03573a2b03f62ab9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x37622de79f6252ff6bb76900db06504434856faf33c59a1b2e39a4fa60ed143",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2c4ffe18ae93ab53ff6d7d01a7b5bdc5b08dc8d144e0b917f47e60e3cf723f3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x42ca1f8224d317275c78ca7762a78e6c51978afe1abcbf535da6d299c799c1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6403910df189d75aca61c604de3b0802a4ec2ffadb0ff60f1a01f363d66ea67",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x44094080f29bf84d3d5849f264713647289e9af1534ec38d1a7c3d2d2f1ab64",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3dd2900899d2219ea16fc41413af028057f0c2a674e1cc65032fe4dcb062d4d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5a612887264b1ff8e5239b3e04143dc30d0a80cef1c880fe52ee2d5009092b1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5ca2e5676dde96127ca85ff6ac82a8fb35b45651b88bcdbfab7ae5298d427c8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4b135ec421e9138d09c709a5d92ba70e6944cd44a7eb7f705ab3612de315ac",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x152b3265b01fa9ce0cdf58c17cd14c2cf3e3fafba140db9e27da4fdde7d3c0d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x55554f904554d2f262d1db49d7c515414870717c829b73d6c439260a8bba3da",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3e2b9e4151827bb0d04858df547978536215dc06143674d0d2e788dcdc9c36",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2c30d5e07853079c9f11624e2431795e2bd8b4bebd8cac92f158306b45b0549",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4e922a3c7df1c668f86b866cf0c07ee4658e7754f6fc0fb62cb297bb6960320",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x35b787fb9889163a9fb5ab831838f19092aa4ef8d8dabb299045740959573d4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2d847968e995dcfcecc6ef98ab27f9f1db36b14ce3ba81b80cc92cf19750f88",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6785d833096c9d9d06034ba4d7f8d71481d4b680b63693d9fa24ea10d3511cf",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5617f72f8d0da5d7cafeef9269395ee34f921f5cc8d1a4f4c0292a83cb0b9bb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6a1373cca7777e3cfacc6502ca9bc645678445d98acf3d6f5ca6c82cab53174",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5b07a69abcc274ea09eb67f2f6036b492db1f9b7e0a3497d8f3920de22b3b4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x74399a1effe3a13a8effe952dd57142c254ebe807a56f13521da38984a0b55d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6a399f5bede4f507c7251a7ccd110e21173729f5f9a57eb16a27203d3c5e731",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x13102ce3fef387b552a6b8967f788cc8f8502ef0f2ec293d2b872328f78b6c9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2233376db0eee71ed0bc6ec0de23782ca9e244a06b8e515b2855b522259eda4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6319176edd9fe726efbcc70108b516e26152cb56329b842a1e14adc2a3e47b2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x12e828f63839dc0dd62bc23385c0bdd5b11e7b6de2cddeccc47f85027c9862f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1b5bfb21e549706eaf5c771448f91d1ce03498029ff4159d8cd11f4b6d523a8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x223b2c9fcd5a1d4b0f7decaab98bdf87e5083865ef9b6562a261fc75009e725",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6856abcc37696eadf09ac823f589a05b034ef8f86e41d2c6222f039707017fb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xabbe74553aa10ee20ec6f0f49f73281124ca34d0b71c2e80160f37d3ae0345",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xf7043785f78a94a68b669cb366c00538eafb8e87b5380c68518d4e23922d6f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x43a4dbc140986d44a7099720e13ec46817f0131dd109a48fbbcf190671f35d6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7fb8438581e1ae31877119b91ef1ea28181ba8c0a89eb356313c8a910295d7c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x125399adcf39aaf7962e3be41c6f9c7691e45c2c31b937e26257d94b5454985",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3a3c97667e93fa5cc0531c8a2f6d9f84c4f683133b8941fe1382ca8f6f2fe0d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x253548b05c44cb4d8f2d97641773cf812f709663fe8f492f5a77bfbc8477d79",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3e828a46091dc07cbbbb0dcbf390e4b5cc44d086b0ba74051fff237f7d6a74a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x74a80f191573d77481059c14f56764dd2c11571b2736d355efa299c400f0377",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x76169700b631b19086b8b1737e23f1c59cf1428075904c80db724383d3c6b5e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7febfa3ce41434e03eccb6be0099dc31d90e36558dfb6f9d21b3e0be41472b4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xaee16ac845b8bdb7d9c1c85ca7b0e749a7c47229ba24ba097b4b6b8151cc4d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x31cac8c51732d8aad5bc41c9a6440d482c2c4967e75a571c31b2d9aaaa64068",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x203db741e5e80c19c2bea387e3091420b918fe1142bcf2bc13ae7e098282fda",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2449d2be3af1fcd8984a9f857309ab5e0e5c010680e33b03a194c6e902a553c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x40246dcd91afc0098ab9568a5c97d54e09065c551bc9d26ba0ab6a00089bec",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x50cdaac85b8d8bbf55a920bf8d213e333eba5f2bd92e92c61f3946617222ade",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x65dbe95ea2b7d1894854b235f2cc66e910fd2791ff09b92366c7685c652a8c7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x18f3259c8451dc5007e94efcc6e90c6951543474925fd28ff35e56890bfb66",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x69021f5cefc75ce473977c2ceae2e7c66a84bb3d734eebf4bf497e56eb69959",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4ded7eedcfca4ee336fa075aef6a017beab322cf7ddf83bccfba05f1c93cad",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xdcc0df28639fd96570d93a6d1df1cb1dcf6db8a259ab092b34cdb411895aa2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2333165fa7f9414f082253b8451638fe1e9da3ba8c1246723dbf9995e49d017",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xc8b65a737b5605606028a064d168ccf32d8d87fcb55c6c853fd95ae0961410",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x71d72b0c23e31d703f0210ecb2b28994ad828417531a15a17a1fd401daca2cf",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x211c3e223a3c9c4a024b490a819254ee133ef9740a4026eb3a036bb9e5c6581",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x621692ad7ad27517f4de4e528e1271719cf5b344d463c86b9cd8424a4fc274f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5ab65084f4ee8261bfd290e2d5608fde744be92da2eadd5f2fb909ac3d14818",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5896811c73c991f479c7af6238b51252178dcf4371c297326bcceeb8ee454e2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x453705ada0d5db6b0afb289b29db6c9acedb01e742cb0d68705d07f8dfcfaae",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x168f97539fdebde7280f4d33f7d5b469cca77495efd4660f31b7d8018f7f89e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7829c898e33552459e8fff13c01f1e0d9f5b098f0de7161cbf97da52914bc38",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5b23dd8ead53bea28246af5a3a63daabf41e7987fe61255d97f2a57bb6d14eb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7b631dfaa76643b5f46a069b8c40038f77f088374320add0ac3c9924a12f153",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2408fff139dae5eb756ea03ef15a2484f582f7ab27ccaa09fa8154f3bf0024b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x28aa32bfd8c8d7ffcb0b5dadfcfd1b6bbd69b02de9ac1bee786da98ce76c8e1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x33eb39eae1db6ea48126be6b300b31f6bbe275845822f9eb293e9f7ac38a777",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3e549a3d3849a09d8f1c50f84f7caa4aa0a5b8ef6f957dafcd13c7c90e4ea11",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2f5e865731de5068f289b616b39c2294284c111540abfdbb33a39780eb0bceb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4e51def182a5bd5672ced3106f19ecd94b760dcfc68e66a3656d0b5db19165f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x42f46c19b87a82522476372ae65817f8d53f263674a040531bb37935b289893",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xf0bd4817ef6ef818a35ca3678f88abb078678a1364539bd7886dad527cb28d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x501f0235f18b49889497cf7c91fe0a1f81d74da8cb1e88bcfca9127392aabfd",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xc11cd155f0a514a5a419d10ffa72405817256ffc8d580b9d3ab002f596b2ff",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x9f712ae0384a87901ad44f53eee9e7c39544893d10b891a92e87e4d78e8374",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2d511bed457c57d7354252189efd19e4f5c3496c1dbe1f1408ff79c8cb97025",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6c8d7abe5c83db80647ff904bdbf25bd0e979607d2310ffbefaa1edb7ae1bb9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x46e747695d9d234e15781125d05b85ce3cb01d676ef8fc45a939d5e6d4e2e56",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x21c86da8be11246b29f17d5f7f3566c20712711e03eba57f0ecace8c4355418",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7b851f4004fd9f20561e3755d7c89528ddefddbbbcbaa9293e416c0dfbb95d1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x31587ae13086228663118a1fbfad6d65bb9741d5682abfb43c7524cc6c240e6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2ef1cb499e790f2de6129225457520b560c1c3120457e742957d1148bb934ca",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x781fe3d95c096c6df1c9ced110914917e26d0860da4bd769e4682a17540768b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7cf92bf7e933187b6ea01019ed1c2d9936e53a9ea89724e00e36672dca1e36",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x290b573a86b30d59fd1301b7985a68fd9bf9dfca5451179bcd13d10eee988aa",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4d3987a0850d8159f9290a8ae8cf99a0ece9961d22135b584d8fc742d42c15f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6247162754e5af6a0efa837daba678811cd749e92d91acf35d732aaf4bfb4f3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1e3df9ca8b80529441770e007a27cda52e54307e4f3370a83705e0f3ffc86fc",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x25579cd0082839ce295d9bdb24140a8f2fe19f7d582a4993a88639a0347a522",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x70181ba88ba8d19c0220225ca0112845e23ed7609ffa4f2aea3cd40a40eef30",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x481c8091e40139c67f7e69737f83a6c868e582526afd50b548bcfa5ec2e83f9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1a758f2faad6702cac573f8ee11d83977ca75744f52d650a6dff79bd6c5caf3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5a87e6f4731da56e8b078bdea4cc3f1fa2059943de95ba404ab38addce3d6db",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x62a78aa9e73bc6da0a8536da8dd43311ccfb52829e89e9e94f3b413efb8ff93",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4cb8044c471e8cdc896ac725744d1a6942bcb26d50b3641e2a95f57b0e7dddf",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x57d63baf011722f5c5a9c4c60899bd918c3287302c97e91fc6f9f8ba089cb97",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x60de66fa4cc5d53fcd9d027cc06945a96de2f9b4f7d0c81c53a7567fde886dc",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x794e6f83556e5ffee6d83daf40a067363b22e157cdd970366757d5d6a02dbc9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1dcd10514fdded828639c9c21d0c8064647947e9ced01014ba8943b1d81bd12",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7cf749a9a9177ecfa46b901ce91a8ebe103f8920d83713df80efb7fc8868346",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1ecd644cdd8b92b3c042932407033c073c7da5f3a8726210a443f10af466ff5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x55b74b3af769611aa4c4fc71b1abed4396b218a9d5884844c937bc38b30bf8e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4fc4e265c8471510fe6f0dc99d7be1108eab6200b0845dab07c5a126c79919b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x37edf969a82e9364a741858bfca74b30e86b1b69b4f33bb4a31666f4b2e7c10",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2a3ca69d295e5e750b4db8367227f9cb347b3693251ba9761a22d411de1c41c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7e9f729b710f0fb173b36a6ee9611a9d309a9dc69a776c08dfe63c64c528a45",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1e2c3057002cdd12b80fb157887fc066b41436bbb71e328bf79ed2799947c49",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x32124f76e477a3c6f5f4346f8abc19cd481b6f43088ccd1c3e8c634bd90cf",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5007d334256950aba31d4bedb5decc0ba6ab62a09c41baa8ab8d0eb4cdc170d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3a06f0e39b3afd46934c41a79a317f220c6321664cbe236ffe1c191ee0b2c85",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x18db208640b40e1acf69b256f0cf86c76f381ee79fa0bbea47fed2c95b5467c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xa0c5ba0b916bdf79b70c0d23013443f65bd087aaca62088b0d1f7009dd2d70",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5dd2afd2e8b09f86360d183e2700f71a4fb5e458c61823ece1a4e60200b82f3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5b3102b46125dd26f3ae75c22cb8be10a3c98f269a2e91ce7d595d25c77e6aa",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2c489389378216a8f4a24999efae5d41af3bf123b10601d2efb419999f329e9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x28bb7956a08b64ed0ed089f0219b05b282eb25c107731d88867f7a78c3e387e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x450e38572d5b45eba95a4368d52056640cc18213b3065bb7b373a05561cd44f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6931414c4f1e51dd287a8273a71ff946d1502d29539815c6652e6b71c95d013",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x48fdfbe3980d1df8db00fd59b4b529abb0569c82a25c6b23186de11aee23a40",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x680f30c7e737040028b548f49d2110d8889aa8dec6afe1de989e3f1f0c1c84b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x8955f2b26c2c91645402ea61e0b3bd091758afa740b4478e3fd2d97b7d5729",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x62796f07255aabe16df1ca5ebe7f7be4eb1e9b688defe3044b1fb8eb56765a3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x29f85ec8df7c753f09bd36309e6d7d65f5d5c327d4c80ca33eca932da5eea0c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x40f21a24062575a80e5a6b6fa209f04178fce24323888c3fc9a083c6cfffe71",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x29eeec3cf3ff9267792e170045fcbc1358ad5b9c28b97db6f4cb5a131dd1e57",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6d82291b429009057a7d89082c7c3ffeade1cbb4598b6bd1322c2e2d3c6819f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x78ffe33137f03476882656c458a984b78bfe509d0ed005657860541fdd16506",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x13f6d5bd19a25ef48bb5a89c64894e9351380c31e98fcb8404c490081665acf",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x36d9f7e5746b465ccd284ac21d5cec14258587d22189b4f85ea87f9b4d7c2ef",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x54730884e1c5c7ff5bff889e8e5846f7e552f07beedb27035c0eaebfe676023",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7344cd22ecc8029fc605bc46e5f2f60c2910130290257210f9db71f26dfbdcf",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6a60ae65aaae41d02d6ad44360c269051a870c66a87e430eabd1c2c5dd8261f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x639a281c19217bb79dde39d86549ffeaa0694283fa876ab39fa6b663869ac9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x48a07a1f3adb4348f65ca07f7e1ad0b70a6024c4934df5724c35f1930befc90",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x527d59fcf4e21663d7e921cf93b705e95fca41d9d2f88720800586e03bdc283",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1b07576ead1fa791e38995e423ee788587adb512c1bda749fc0869ac6b40c6b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7b59c1f0252efd3b471c3047a2060ccb98cb86148c1b1893af4f86384821b04",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x56e48833c5707aaa1e38a0d644765251c038ac3f89ed4d58fc3b24d03a83e77",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x14d1da51db79b82ff5fc48e18ad84a98b1390d8e61e1580ef5c6100d49da80a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6ade2f8ff114a1c0a0f108286f0f0e820073e7fee989a85fe11a97b972f077e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x20268b11ea1f54c737a14073b8bd83a6151aa30b0d51182446adc72aa2bef83",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x937368e9df8289ef2d93e806914cc9ac730750d1ecc6ccf6c4aa6e6788d35c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x297d16ecee6310efbcf8a2946e1f03e23ce1eaf88fa6279dced371db9dbc299",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x74d40da7c08b8fbd488137dcb60906f2004a26faf06e6ee4dbe1feceb94d98a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5669aa7f25c6cefb4a3e1f5491dc50af7c44ca9f8405864906b353c4c3529b6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4a5077d73c41429dcb66a5557cd392c5e8b64f4f93507e5e7b8f1cbe29a309a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5974550418ba46ba346cb87069b6c17f9a6d57ce7554827c8191072b4ff8357",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6fb7a7f6c760b606b4f7cefa186540604099bd229b954096179a12ccd50e323",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4fe2139f7019584c7f395a18bfca2f5ea89a9300bf208b9dc73686c76e724d6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4b45d3cd223171c9e2e8030a3983c2e4b6ed61a560db3a8da8a2bf1da05ae2a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x223dbf6f82e6f2b2dce8397a7a6d00c8fe38fdd8463fe7612c1a90bb76a16c9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x13e7cbf6809b1c282b1716db08a549825b9e1f24479288cf615c6557249f675",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x13b2e8b26fec1c97c5fac659532830270b08cc6861df86b3f3b4894175551d9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x34a45f657061a57b808e337faed21f722e6298262a2df69d6bd34ecf2e29243",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x372a448e249504e459982c7d114b3c79270419467208096cfa6a96f3e5de755",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5d62087a11238dba183191a31e686ffea34bd393310e7a2b11c75d63ec340",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5f25fb2b70ae9e334bd288d6768a7b3b6b2f4672cb671f6b0ebd781134609d3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1025bf3b6ef4dc8e3637f4dd1cda0ea30ebba8c30ce5638b5f9b5291faa0036",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7d2b1bf76ecca560b7409dee16ead5b2b3691ff75ef8fe5a844306a7e29b252",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7cb9fc14dc4bbad427efdb3f5821fc9dd10fe8595577e39645ab9f62e6fa50",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6f61949d4cbc8298879b470d1fa9aec82261a8099c448dfa4379a597ab01d03",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x336d1ccaccbef10084bc4a18f8c86f699642878a2b5d5af3a3fbe7a773e6904",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x354e8d015485a06adadbf43a6bad63e9330c4070fbf2a704c166e1d278c8d4c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x45526e767c14a531fbc10f287b2a4203e18daad8a4883a1900a63dccc1a18f6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x688bfd9b23436077dd139ccf0a7286444429f3a2457ce7e2cc939be2172921e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5dd26663ad2931b249bcf054211723be60b5b46de16a61928c0a9326874f3e0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xaeb2689fd195377c86c55bb52ba2ee27c7c5395d8163355a3c04135b43333a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x73ce15dc2409ac614aba33d14c4ad294a3a8136eec69e8b34b0b14b92eb240f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x73a3905fdf4a2f53d66ca4cb99ca729e776ce66d9a474fd71da35b3fa949d34",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1d60249bd6492637249efa94de232264fa23d62153d7a36e99aaede0be5d842",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x728595451b9c3918b04e7ce1637804c1df21495ad8f188eb46a5f1796e2e3c1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5dc73d8837d2fd0c754ecd371e94f0af344396efdb4337a8c7c2a0755838f46",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7dd917167308b602914680880c9c8c8519f34be930ccafbaca3d126a30c4a45",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1950cc259cc77027d5c86ea77f51a34cd30ad768676d77a0503f36f797eb4af",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4d5df514fa9a8bd7515039e59bea7a1a1381a76f475a7dea23549106a7df8e2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1d74577e412af12fd886706cdce3c238f2761d096043a084c20d2bd087ad4e6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7871f7217ff1c7b739678e28908c4222f492ebf866cbcc410148ad1d143de0f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x59a88072f92b384925c9091497269ba9f8226c24f740e928e410ae0bfb9350e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x19c90daa3645b62f461545c7c38ce5bf8b5cdad399f417e0abbaf2b2df0ca64",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x53b86dc3cb8ef3d5920ea35c40e2d05496e45245eca4e0d058e2a0e2d583dfb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xe00c78bcfc271dcc6556cb1cf6501e16d20b188c7412681c0b2ae0f2cbae05",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x36ac6cfd4f2ef6be5b1e83cf9e36e894b2575a8f4690c14484a17c222ec3c00",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x32cad92232ea7886b829887e6ca4ae084800803277076107b1078feb66e95bf",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x38269de0c80a2d8f4bef1d5e76805d1e412fef7b18886279e98c57a0fe64627",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x217df85b26b6b3bfc67bec919866b6e146621c30685a31e8c93eaa27d5dbaf5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4ee32b2ff29b0918618f173c4e5dc3b606a1ca2e0eb989257e0bf78dd2e9589",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2d51937feb119772693523625e23756d172e996d1cfb82a258580bd51c15e33",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7c2468c14a7ea994c89e2e4ddd6d2d624b67a96e7ccec4f27a5e0122531291c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x602aca232b11ad63241b5f401c368acb1e9cfd4e5fc8ae699491d9c51b4db18",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x39c92f0c55d99aa6b082d21129a9402e6b0fe38a639a8140d76ebee9dc45877",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4fff8b45f7ede0580424c4e2c75213c4c42ec6c68266c8d5d750a2863bd474a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3ba2c93d59d6a361b9ac28d93e54d775b040bd7fca9ac72339ea4388c533dda",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6d567c1dbb663fb2fd92140cf66ea33a19cda580d18c10fe56a62e5bd3f47b1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2781266a2070c9d3f045010a32c98ce3e0765446e3ee20eacd73a0dc0c7c2c2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7dcfd5001e21e030b006d54c7fe0f7ca97a2c18d4e00ba92c005705a4f0563d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2fe4c112d7bfd4ad5f81ecdc4b30cf73aa51df4e4ba6d255a0e3eee283aff46",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2433d40f2b8f9461b5368cb396f7604999a735414d3537ed6f1451f1fe93cb3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x72f959288185bd36ca4e23472ed7a2577f8e5f0ef0c0d5df6f63e60f40ba307",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4c5592288cc342232d76c80e858c08ecbfde64b747637ccc9a2734e90f85264",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x48ed57151b6dc68b039dc327f79bc2c26db62ff957809c5538360facc04d9c3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5ebd9b268cc66bb85a5e67a6b9d5fdbeba8b3672491068ef43b688a3a043a33",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xdf6c1013f3076e6044f0a7032e0bf80833f3c7d9eb0c3eb1f3c2a37314d19a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5f0cb66613216a1339c1cd15239b7f03c1d4b9098a931f65ae50b877f861880",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5d5fbd560f7bf1e97190f888fa43b32db1e8070f046d6016b536b94d1473a57",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x33381003a653f0327cbdd8a11252ffe714e1061ee214329cb99e667c835af97",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7b86849a979796096f7d7b46eebaf00913a082c638c5b2bfdedbcd78c480272",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5f27bcacc10845ad41cb26244112faa8b91d46d97024445f50ced796ac5a93e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x31cdca47c96b32e99077a96aa5cd73ec9c4da04212667805c82dff3e498f4ed",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x66a032c182ae70dd4487897d0c79dd860a25d21c61e3aeef8b9fa45349dee89",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x148523eabde5554538a1114351f3d8730d4a4d003311c7b57ce9e709afeeca5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x62ea67803c421a4bbdc672d556bca219fd24e7145cb3e9113a625eeb4459254",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2c2c70075ac99cfe68a7354ed29842c5207bbdbd09dbbd225ea93d0c07fd9f6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x48b0cc6241c99407bb346db57db9cf82b2e66d1fcc1d756889a4f4b4bb8b396",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x506f990d7037060dea08ed53c5b17483ca8a7c58f94ba5e64fae258be4c78ed",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x777be5852ea7798899d4750e9decd1430bdad6a8b0d1827a7a89ce6f1afd89a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x689fb22cd95d6f1868a4e3cd6ef1bba9f974931f76153e73038ff5ae7d09018",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x560fef0ed77bc94e16b9d9a21bec0ceadf81b26fe683b9c74b31e2d72a4c92e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x28c74f03f409c942d16a773fde01b3f0bec544b42c1d46944db6253561e1ac2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4d62e1ef04cb039a58dd8cb8c37dceb78b10fd84bbec6302c964b899a957d02",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x44094c265809e3d5765071826547999dce8ba7058a7c1b1301294d8291949c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4e0f90743d3df3d3c4aeb80e7f6db457620430ff28475c6194c757f81927dc5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2cde734d2a82619ba69ca4f5ca5035f699a1e34b47560d761780546c9b04d44",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xf09069dc6a2745587b447ae03ebd6524aa9757f1090a92dc5e7ce8db848195",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5522f48902001bf41de34be900783ac957fd867cca0f35666ca491ea89d8fb3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5b4be8af83915fd955ba32de729f6f2aef6c76501d82ee325d72d620bce8b7b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x78e74ffaf944c363f3fc42cedaea8a9a450ebaac98bf1327590a11e064bd76a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6ade7c482d201c23145e3890086b22ab0d43495f5c83b1672316c10ca52af0b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x79c066efe4c22c6e9e097e84401e183d3c45c645d986ed640a8faf8fd4dd096",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x21d59ca7451d83d78ab4d9d17a662367ac84b555866ae92d036d71de22872bf",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6d100d3db14939bb442e5f5ce6a05939f201837007331536440a57c2bf2b609",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7e5bc177982061f124cbe521c713c24438aa021fe6928d82452e44f6cdcd631",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x179e4b1e4817460085d47376a1971fdcb0287408cc7d11fb62cc3785772249c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2a7cd1fb12f896bff4d3db49ee74a51e970e3e386c2c8e7622412a6156a300d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3c5581c15733dcc4d548aa0a6e648e075e9be412680a76a556f91ae5f01e44e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4a4eb8f57c99e931a666de76c20173adcde82ff59fd8ecaf8b8c05e29b63fc9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x58fa31f9a4a7e8b238898eb1296ec55e3e2000a48a2edc8e65d260d31bfd7bf",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2c37f85ac7b1aab52ce3d28bfc65c65b7d4ffd000757c07fc493d183b7bb582",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x461b788a24347588e4f8d4f2d66640f31d6b580223a21919ccef9480987db1f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x47c3222376f8f18dc6e82eebaab03fcf4c425acd901a7bf9841a3aba54b82a6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xbece573771924d045b75bb992a87b26ab067a0f2dba4d1a9efbe5029963533",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x67c2a0e19b59921666716fe2b3f9c7f59c4da17d993956eb87eece7ef542269",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x8940bc9dc45fd06ce4046337963c849324bbe5f82632b94972c0ccb205480d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x38bff358ccfd92418537a9b9858df499d2c44404c1886b109edb14c897e74fa",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x533a5a2ebd098297604e96118f2007ddd12af50edd525e9e5a0b154e620b2e5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x312411292b7fe7eee015fcfaab65b611bc2b9f9498489fc3c1452862902bbf",
        ));
    res
}

// Evaluates a periodic column at a &point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at &point**(2**k)
// instead.
pub fn eval_pedersen_y<F: SimpleField>(point: F) -> F {
    let mut res = F::zero();
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x62b07622f501888a668440d9b856be4b0c3bf12a401fc2bebaeab4a7e1684ad",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x75a127d817aee244517479bab5c4bfc2a0035d43d673badaf64d8adf94353bd",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x14f3359ce0d2891d1bc2b6f4d2d6dd71fe22925b8a09f66147db095a9d4983",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2d6129632b4fc43e4142abf55fe2d1f3e79dfa01c73d8fb56a465dbd07a9682",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5115ade709c058be5dc6f406794062642086e431bab03c9a86d53c79aa83db4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x29f6aa5fc92eab8b8b9871c8449c1f617b808ea9860717f3e5e1678672ec565",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7e08f9d222cc0764fb5ca69e51ad4cdb7f1b612058568a142bc7a4cdd0e39c4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x54c5dff0aed23c07edcd958ee3690e617011b87a5fec541725237d4ebf34382",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2b7d501bedc4e7c604b0e55dd2d8166fa39a541efc24d81d8464fabfef3fa37",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x68e1d50b4d0570e357eac7bc742ec26dac1edc5b179989c7ae8d31791639103",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x707c572424682b685a1ba90dfd7e56f86254862d86e20b5a2d3ca85fe0017ad",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2cd9a093ece61e554b2bdde3ec474900e4412775ad25456e5be6e11df7b9fff",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7492aa940f34a027f8fb3700f29cf628c1d05d1675cb7865509f20617a90b2f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x55e928ba557ed7fe0ecde6d1fbb83d112e6b06a087b4013b9c425fa36eb0415",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3cdb28a913a41d597915de055aecc59f2b13079d3d8b33ab0a075eeddb1bf8e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xfd48fb35400aaaf57d130b6143b241db8af174cada72ede8f2fac4ec6688d2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x133b6505a6afd2e5fada0e53ea51c012e4935ea6d2d02caaa15ffc50a45079b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2d2d27711772cafff2cad828dd78d8b213e317e8939cf79164ae64dea577d61",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x60ac57e328ff6938a17d43e6137a55399b95459be60fe980ed8960edaeee10d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7b056cb6f172b25e3555d6b1422ff769fd4c07258fa16b03609f0e374012ed4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xd21afb1901f1b3ad66587a7fb97ee06662edc3bc8c8d32b48625a135ba23a9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7784c2e140072fd26e95911df38f9a337107750a72b9ce05a21a0f781b92dba",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7c1667b8d44d288c4f5150d01c5206d4d5868497630745b6916466c8a5b1228",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x491c2243a95c44528b86167a4418ff9d93a04bde8dd7a5d2b19ea579d295312",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6aebd7a9279eba43cb1c0b14bb723dde464a86cac92518ca16ae27a8684d9cf",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x709be747b0a69a9523680ff69e6bfea4637bd570ce5c45256b39ff695557da6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x47275cd67ff3b7637ed55ced299a6142a821ab466a897f1eecfc8bca557269",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x58ccfd44df4e339c65e1423eaad47210f2e16aa6530d3d51f38b70e5eb3a623",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x28218a1bc7586b71ec1989082b8f7ab0efba14569c6f6e5d7aeee1964ab6d70",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x660bd8049bd301d093aab9ae530bedc37467d4ff0a12c89d689d61ef9b9546a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4dd38543d129d0a71b1836c6e2eae47fde5d572e32ee9a5791f7ee823eab4db",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x8509234000e130c8828328ae4997d5116117716cca9490e6e63f30b7df699",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x62ad4d764ed0072a7649425270e2b210421c77d3ce98e2587ea5440a591ecc0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x66c5222dc133e37dfa0566c49d107852d978eb7454489d3b2ada8de022125d8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xe4d1f1f1f9b379bea473f76bc8f3c4d530335e2d6bd782b42324872767b628",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6ac2f92bc4c04fd50ebd3e336b53b866e790ace39838aa96a4b791011455b29",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x59d753a23735a336c50466f5ccaab3671230fbdaf55101530e5f562a5efcaf5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x166b26359c51d067955874f5612eb70806d7b8d5de4d8e0a75e0d57b39b1846",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2dd27ce7910e44ee00ec3335bd79429846a70d92d482adf81b36a9ff1aaa30a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x45970c86c25bc9a68f2e2a34969faa2134c95b19230fcfe7436c98f537539eb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x539cd2c1a28df263709cf0eadef73a600f563ab3d82c27692b1424814cc3e15",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x31eb3b57e6844e3efc1e3975ea393476d8aace5f43ca62b09314c90b8ae9688",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x47359c8dd2b86e4f634a9a50950abde25942877bc5db93d62bf43d2886692e4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2d8ae7b28c8c3acc8bef3d4c2a9f5ef1323748de693a9a1ad3ff8601116b165",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4538fc863186b4babe3b424b4111251bb1e20ba5516be54160cd560ec0d5a3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x559548517b1025ad61020be3e252b6ddbf1d5d53043231f8850c0da52b8268a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3132d42e4a928c08a972e17b2c3b500dbcadbe6190b2e7f5b58300a0c8a38c6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5b02adb78afd4e219642a1fc38b2ef9f63926841ccfda072ac17326d3d50f3c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x75971517855ffbc9657dab30657ed1e3797307bbec1ffe136cb0d8a64ed6eea",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x174a4710688db61da7559255caebf641a268b4df53d45de5e8156d36b4b2ab0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x295fb60eec46a40a33b1a9532427b42e224c0ac6c50e3c1c5d17c2c16651a25",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2037a7d08a1c4fa4d5d4f53436a252302840007c09163026637e9cdddc958f0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7e9e1c3d4bd3231686c235a495f737a9ec3d633331a95d85e17e90f99a08af5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x218da336adf8608530fdf8320c4edc00631d36c8726430732038a369548cf56",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x78c7b0512cae47833eb6bf01c1075aafca19eef9b32e37f4f9a9eff315637c7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x628226c46fe0bfa8aa36074ed0785cb16461ee2945ecee9deaa6399bba2742c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2eb6bd70a00ec26418d347df1a444f7ba0972416103f00c771e0f3d50bd8e5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xdf82eebd6cde9b50958606c6ff83c855c43ce9613fec366c7792cb456ea913",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4221572cf29651f508bab9eb82545b17cf6f9efd0416b65262e5491ad408e39",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1d5fc46deed0eb9b56cba1d2bf8075227504aaf6ab1330b346cc3cb84a07cc8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x55ee57d4096ccf0260baa2a1a2639978d965a786e4fc917cb2426f8a99591d2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x21706619a453a544bee0ccaceda9fe69f860c894b36bc9cb7ea4455dd88a9ca",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x76ea16625d0cf0c04f096ac7d6eacafd00809ef1d1a3cf5e37dc2a13a02d303",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6d0d885e8d2530c7a324f7b2ef47db35aa8162289a4420a54f13a82b871d850",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x471bb97187c83c0e7b51ab70022147e8d8ebe25d4081907e7d1bee8d6c6581f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x12b9157240a237f319beefb6019bf0de1897b9e2d8e5536e3a21d8f9fd689e7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2ef7f1dfebad70ef549da1a143c838cea27749807efcb1a0a29cfab16420928",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x519de0df91b17442a8f60b512297d69a1b516f70f67d76eb9c287f06e37c55c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2a45de0b79a4e9c53d47f6126d35b1d050775d5fb17f3c3dc22c7b6476608c0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x27e3cfc87448bc0392a0d6c1b1aa06626636fc703bbcf3717fbe6f0759c4855",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x77a45066995089dbd4072d6817397ce0c7e92b53d19338e9bd7549e954bd961",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3139ae970d95891aa70cbbf6f172223e07eb521a5149b7e0c9202793f6dbdb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x25dd21ff92e6f1075df6b5ddb2b774ff963b1b84a985261b1e94ca9eedaa49d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x13eb9f5362c087af5ee758bf0b589c0e34af337b3c06c788573534e96de30b7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6a39a27be962632e0bfb245f65a4d70912d1572e39003d63def5f45bbcc8f7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2c11fa8c0ba68518942f1c686dafd32aa26545886d28cdedae00071360df674",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7ce49a9b8d374e1174ae6ccea7cae8d743404552253f7ec854317722a5efffe",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6bf518769635f9fa39c1258844d4f62e5fc00b70792944da0a939990492313b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6123efb57144c151864b4f717a44cecc667fb2ebc47bf24bda4f7d0ef8f550f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2c29d0056cfe7325567a9f2d09d917b37a45aa3cefe20b78c4bda2942af59bd",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x76914b565dab13e76053b7a74c24c005b0930011e48ab26db44b6b49a1e7ae5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x648c35904fdb8bbf9c0bc9288319c248e17974fbb6616a70acdac004878bb9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1cc7ec07c4e1e6626b5263044071687d0ad34ad4d08996a213380e9b728585b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x154a5ad294d41caedd8d927eac226dea1e2b78f6ed0a6901a00e45ae0ad78f6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xbd5fd7dcc1ce2bcd7f7415a22115f0c846d16ac7458e6c531e7e44dc664962",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2b2b08bfc4c3d5941538b2eda43b3cd009656cf83b6b23be56b3041df3dbb0b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5e48cfc304417473eb4e587942a76921fb007d8b11ce648d36828e8cbb5d595",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x79c11c262fc2efc9aceafe4a5886713151352e60c4db45826e0e343cc5919a9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xe2acacfba8f832e4e3cffb6ecf4675df678403610fe91363172229444ac0c0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6dbd918c7623bb07b05ca515146ddd7193373250e0836062fd1c430e2b7894a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x61b210c04a0899fe2a3dc53348507d6f53d4cd3831644e4630eb40564ee5b47",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1e3816f2a6a4900b65d140d144225a8a81cb3ea22f56de3cbcfe3944fc0e898",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3c99839cb11fecd878ab9efd1d2ed2a718f6e0df4caac1a070de06ddf1a6091",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x567205f3e5ec69ce7962918c41ed0309c3ddfd85fc92702ce1c207b282f17c2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x10382fdec78a18047041629179e18ec7dd067bed125bf5fe83f13d637a8ff67",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x43aeb91e6f453d372353d9814a85c21617e6c934c694a0b06100e1e9aec4087",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2b6a2e9d453e19e3d766f24cb7c6753f84adca0f75f7a871092688bb5ba0d37",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1f3e3e61713ab64544b28dfcaf4da25b64e625048ca55cc783dff614f5796d0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x364cf25e248a3f2fc2106025945389328c0ef37848a59ff2afdc685c0854822",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xf9762bf5620ec90d711f12cbe600f29906fcdcdea4f17cf51ffad2e07887e2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x78e4cf312ec50466bfea965b655e9514d9d69bf0bae566fc88187fe730f70",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x8981cc99962f20f8814162568d9d7edb7fcc637fc6907a98b1d1eece9811c6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4e28bfd662fc5e09399fc49a49a622a7e845631244b9472df8c91c4a703321a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3085800be446839854dfb7bd9ea67ff139267fb5268faaf153db26b00368630",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x787a6c80d5a23f91cb91d2508633cce6125da76e797ed460256933a4e6a24b7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5ad768a2e70b4018e505bb5f6f44d249d9f5ba5f126106cde9be7726cf5c0a3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x58afefb8e3180356e33794e20db869aba4bd4e5dfc795f8089d6f123025179b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4f62f4d968964e4908d16fb9412f8d10eb82e14e83f3e094a02470f27eae006",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5ffa0d51bff335ad53cfe99165aa64f5ac1b01c360bd0101856537fb03da5ed",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x43fa3aa05db6331941265719fc1ee057d9f3dc81704f81c2ce7faece0fe86c6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1e836012f5509ea2f3dfdd474e9e4a96f6924e6619924ee4c6870a5294e26a9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x41052d90f803f015bee5bd1a5f6e2f78f30439ecbe39861cdaebaa8f7c56371",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x16f6ec82023f48ea80196121afab584b9bce7f01e9515d0a3b489d68df3e2a9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4b7236fb7f8b72b2d369effbee5b4bebe7d2205ed72f9831b41c711680cbbf2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6620ec871e8a2c03933d0621b13e7f357b7349ea16bb549e7e15e2652692252",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5b553a6606a3f01d862af22a3309a6df0aadec753fd1e0321b0eb08504c1f12",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x17a1bf17777a3b56a76df412810d05c9e222027aca604791694d3b020ea40cc",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x64fecb621f4dc18fa1b66152f28bdd15b7b12d495c496e77016bf3b979e4b1b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7e31ce22d2a3d776ad90e008ce82c594dab9ff2c42708f4f0676000cd86891a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x43530eaa364a9df353dcfc154bae168e0fa9b51a3362c6cb351d47bb7f6b829",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x35fea15e2101714f172da73da6ddc2077ebd42ada067e7879bba8c2ee1d9db1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4509575b94136d744c8679c3028b0db514688db5338c4bcc9f50ccd7d15c95f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xf8bd8807280892ca46c092b74f845d90f3a6b61b197a0594fa30686ca41a5f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x51443fc9bbe11d787df4afc59f4366629cfb3a14c80cda1caa1ce6107fd063f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x22cf3cd9fc0103158f7de369046ac0cff77c44c3f9c6ca942616fe7d59d6231",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x14714592154025f15704e279d2db4c70f545137269ccbd82c11fba275bacc85",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1d44a3f67a1142e7922f4329f775fec5f8bd2d32ef8ab41a00821e76fbaa89f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2d9f309e84716b322c26aa86a3fe3cb6ff230e0968dfc58b869268c751e510d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x64ff5a81d9e22197bb59e8cb340a0f44e22e226fed168f8b125d850bd727b7b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2cf1eefdbf254a549ddf4069288ea075d9aae074aac7853005b57c37c2039e5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x20d846afc1a11dae8646d542770f294b9c9f21f1196fba567f2f74d058ebc25",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4ce9244cd3966ce1a6fd7f8b85fb1c8751e35aa53032f8063535665ac3a69f6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x203ddf8cbfae2898d2d2f183cd0efd1c3f7db1b84b8e96e38f2b87b4bdad1bb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x37b40695420e59161b338e413a72daa6909f0e4f6f85426f8eeb6bd0dc3a1b5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x8554877281326c1c7e1f3a2f5e81341554ecea862c2677fa67ab2f88b3b03f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x619cb05e71db22ca1ef274bd0a7cdaf4fb79b3015b96f44814b490f048d2af0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x66af1f51f840c438b502c2a5ab689f9b38c2c96df36988710951bf185cb8501",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5486125e0ed23fdc42a4f8c96cb08d934b6f3b429c4af5f8396618e978e9811",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x41d785e118be2d27a159ed5216de66a84873e1f62088726d9607c6443a14090",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xa6117e45c1c561307d63895569d34fd7e3f2b2ea088dec37dc3a5527deffd4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3bfe2f1e8df44829fa27a78c46c223c7e64bda85be60d8a3a5d0e7d36c20e29",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3daea06a4a96480c4f7fff1082d95836964b63c14281ef942fa9a9890d8754c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3678ebeaffc3e64d76141f41be973ff36e8398a6aa0385eddaa0c4183e3646",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5cc1da57cf1059231e195a26b1323599c95f98e4b302d6e6f4bd41180b56d35",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4f4cd7f9fd5b694cc5ea6154d0738cdbac3978ce74a7314bcafea3dbc1da61d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x77bdb42e999e93273fa3cbb7ae7160522231680eccc4d38c1b8a83d2a0420a7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4e1cdd107e3116b4ff22720938a201eed2ea0b499bfde301562f5e39a42b066",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x513bd3eda9403f4167249972ce4947f3ac9e9da03a7b9ef557a65645b9616be",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x23d7ed01587af3b9aefeae8a627c6401d36245cafa9367631036d2bd7c47e26",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1ec8c3c39ec4705944ffa8b3b9b61f73c9ad759cb79a107dd93a125685f5119",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4d0db05514a8c0f152a8664579c004fb738cd3790214984bc3f21f31d494361",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2bc791bd7e68342116218ed9bb657b8b54e550022e39af11ce55b29ae49218b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7ea13011e0dce5c917be4cd36c8931f5969852109a16d7c5142e8fb3c8b7650",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x11a9029a5c07557ec347592ba7181acafbaf0f0c5c9e81d7e995a4de57fe566",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x359506efbff0e2b81d91cd6a5f808a6c65255e1bf06cc03dbaba94758b3acfd",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x491899cb7600abb42ac8cd91f2c775ec410469573f57c1030ed1582327eedb8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7a42c4e98f014e50dba6b25fc32401b7695fadb7bf271fe0a763712ee545c2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2f7d26f183c54146bd83514f5459bfd95ac635649d74225c2168a8e7baec082",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3ab952be650de0c679ddc0a35bac2907a6e58303059d4edb914e74c67d05226",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2b7f9df93ba787a9a5a7a0a3b5daba02e2ce65df16ada37575735697eda6c1d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xb04ac19a9f1483b8ee3b763be73814c9621fb3d23e6d874d9093d999d3d4eb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4f4df07e55d3ebf0ed955bd9f7c34de001f09a92c1ead17b0c1a485d48a4329",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1522043741ba933948d7298114b71322258a3d4e7cf2496590c35683dbe2a7c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xb489643a1aa2c181b4739d45582e2576a6f9bd51c81d300ebdc3a58b79bb2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4db70c63a1dac4e5ddde15e3626d009683aa8ea14face2c3fdb6ec97c8a86a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x309bca858a0f9fc5a468a57981c9c6b7c79636b1f31284938d1c6a21f006a33",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x625a1fce22a9fb7717107b137a0f5ea4ca059008f5cc6fdfb5cb5bb1734bd17",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6ae2e00f7827692b0d20f483d3c71594f61d50846b52abfee39f6697513c0d0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4b5acbaa0f7e360885677439654649256829cdd6d4a6c7ffa904a0683fb5fe7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x66a63b8ed2255586855fb30333ce0e2ff4eb2b4cd5d2125d8d20cd3fcfc1d04",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x55351e9d60f58241736330de978242e4e40c4209a7879d7ae3823c148abd82a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x588747248358bf8bdbd990996cb43468c89909cad0f8230cc939538b9b331df",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x67801dffe217a1a64e0b12f405157af52025266fcc391fddaebf3b6c7ab79a9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x411556b9c89186a2f9f79e55d045295790b28af97fab64e77777e3828532be5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5c01501e6a113ccca7cc9c723b1fad6ba60ec5b0a85b7d09c72120d3f733bd5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5b0d578cb7aa59ba02b0bb894848b745440c0cf562c2e635312c9bfc305e169",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3013a9c6094ab0086b1397621f93ac07bf45574ea26b09d3e4587afffe995ca",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7d9c679179dfab605ca04e1993b37ddff490c440665005698a47c442a1cc10c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4f6e24500755d20ec5f28480a41a0cf23baa1aa24202382e9f4ec8ec6d7596",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4e0a6e0c26f85c74373782bd2924f3bc0f6b4a2914c4f7f8850a79eab580566",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6caac68bec6ce4eff4f74c1f33dbc027165cc02cec8f69e9470ff99c0b132c3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1c867fa9ae031469be012c4f201ed3ad56573a22891351012ad1f7d300361f0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1761abb092f6c4e3eda770480fb4ab095e786bc3f1b1f960bc4c95232308b3a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4a35c4582c91999a39b553248bf2a39ae5825204085a9e98bd6ddab3bfcc0a4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6a4efc048a81614dede6c4f6181253e84f20d4a4f95f973147ee3fcd72077fa",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x65c14f7de75359a40c5f244f78b2920b61087fdbbf59aa507644d94f5bd210",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x9be8b219ca1684dfbef720a3e9f034b319e2d233aed85063924fc60aedf20e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7352e8793ed3f6283e492544b2944d6fea715980d8884f6821574d36868b0c7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x65c04013accf25a2cd1d9eb98689d71694ffb20dced009df5b9af167602b4c2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x679bf3101f8b2112eefab47d7372f0297507511c7cceb4478f2baf0541740f5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x162e6e8431b7280f8401ca08922c5452c7237132efe3a481a71b5c97183e9d0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xd9da926adbb5ffa493c54223f97fa1b0d141129d8736bc4f5768426c7e82a2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x711628cee8d673863e18f058cf82551ca8351486b9b210873b4e18447e11408",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6f6131c193cd7b3fdb4d0848df70474ba9e80529097311cd7c13e322205a1c0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2512f776d1b3d212be7c2adce1cfa083d1b2b9af1c6f3cc424b266bfa19aa06",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x400330fb079fb4cc8671ea9a996de8f5442f20b9b9a3bc9df8b81e01506c5ad",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x40cfb729788e16fa80b7d937f0088157d18ff2cf7c79b748d0e150c896d348f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x59786091e2d824242c7aa5dde34ffbac99f6a9a1aa5ecc8a395aa13e8aa55af",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4adf53e64235d5327822ee3e584674af053e496c5d92a6c8c43e1e8e7d327fb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xfe9d827d7e6387c7228d92f78574add4ceddddac1fbe71dec1258220c08402",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x37f1342e071f8a087c1405692443305d28d4c11b84d92bd7dedc563fc3ad329",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x74a39339d1d708a9ea407f03d8b0e5ab103c3251596258b78be1bd97ad06915",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x152c16cce8c1c782287b8908a790014fe3c51c57cefaef63e2c8dae5a7a5daa",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5b32dadeb15d554f39f227de4ad20600eea4b763fa4c90ffa1a41812ae43479",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3b4b0f9b88e16446a2de79c1d8c34865d5d6e581f08bbbc652ce67d8ac1d952",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x44041800e20fa7a15dd9274ea8283b09c30a0d900d9c165217004e669b39d99",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5609324fa7ef5213591c8d36c59dd42df8f5f26f84468bb84f843707a5c9c48",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6235547369b594514d2fa1ca9b06fd25f9d2764fe8b099c7d9671f542a01d46",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x614509129cebd380f416c4c9c7127ee7b53d878860905f047ad722a82147236",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4f847058896f8e2727ef3b4577e62d5f6a729696b8705fe217b97c73fd1afee",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xfee20b19c4437f06eeffccb05b88c4e236d18f8e3518ba124ab4eec844c496",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5897638208b8e9509d1128c29af87cf30c57942d47016819435b373c0a309d7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1fb19890707fa2e617de7dcea9ad35ce9960009f1e38aa2629c66fa5b8d5d19",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x45fb29b3ac673e9f525332c8bad73d76521985406fc09398078b30339c857b5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x528d041bf152aa3a0205430412a196619b68c81d7a706fea0fc090e0cc6a105",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1e149d42cd477212ab7f01fe40f76858f09ce2bdfc397df635ed8a453714e7e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x75f781602ada44803c0ca4bc8c1bd5064700762d18c309a2b9059dcd8c3dcca",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1571843ced13a8d342b63c63abc4b83d357eb286af04380edd1eaefcef3f1f8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x304103a8d35f43cf87d50682e86e473fffd71d13e0c783e596a59a62b06402d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2849ac77a2f5398eef51aeb8312dcef8b347b690728d4eb835bf4670301e6e7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x52a36a173c7ebc96cfc55bda4bbc73bc349657d39ebe096725e9cc4bff01def",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x43e371660fff35e52cd5dc08c9c347d8f7c64a116375d0e6e3ad3512d85a99a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x76594f29261e2aa9cf4a90b58b0f79c2aaa99d63c4ff64b4806cb8cfb0df316",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4cbae4979c7a1313c2d0f68b21f5734ec83f9e1a88c78b3976a6ef84a1b6dbd",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x76702a08064b5768ae2979aca07322782191172276f1bcfbc14cbaa3e758dc",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x64f8c462b308a1337bca235add2482fdc3607507b2c9c0f91b9187f5676303",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2ef5951aae064a7357b1e4ed49f05f17f778f2e8735f8d17b5cfb82faf3b848",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x66e2651e6f5758c334d1c1451d563b2df07b424b5d0125c739ada959479890e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xb3dd17f46d6b12bf4e5db184d6962c156bef94f9f73861e34d88503fbc517a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x19412ccd078bf5665579cbd16035a251e08f40722eca4452eedb31732488468",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x719df2a50d9c3f2eb3f0336665f2980e432191e21fc49f488854b8352fd94fe",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x459f095ba3b70f76e493c6afe2d4b6eebd21343f74bfe3390868612fc250fdc",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xdef6a0b2b71d97a59c674c052fe23f7d000a334e180b0793b6974fe29a64c3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x74e503d57e49daf6939077c0b4a4d68e66bc2425ce53b01b48f146295476401",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xd49f196e60ebea0eb13d85f05cffedff32477e83129bad30bd9dd555755429",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x298215f335fb63a11d31958d950d95c909bb94e144c113cc4ecc08488469097",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6232d26f420f9b4f119e64762927b5e8a21192575b200081b0545ad4e9a2c25",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2b51d7f94ec71f3a8e3e20d766a4a7f13d08d758a686ff86dbda48026c7ec3d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x45fb08bc21969d5ca9b1ec473cc92a4ad911de8b0607ddc12b9ee98c286d37f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x70c0f7da90cd889d8df06f9774de8a9a20c88e86753506c7afd0e1f6ef15e76",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1b16d94e84ffd3ad61286f5a79d5a6f7b5b5dd6442aea9013ad21467bf1281d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6a81925732161d4e5dc61ed6a10726027fa66d892aabbf46a477f4455072c02",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x13ed29a84c4875ac188521bc40e9258e03d83c9ceb8716c6fbeed065a5df73b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x184ee38f80fa532983fa248c14c0220c2a5691836e899a5c9b83c975b03608f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x65f4a97d2b1c90582859966540e839ac2d62ad2ea960aa2af36776b2d07ce34",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xe00ac968fe5a147fef45fbd626c540a194ec3dfb2c1cca7938e037349d4f34",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x96935be4e41797417259166181bb646a619ef95cc8978ffeca81d141d062f7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x61c61341c83517cb7d112a76864271492473e04130ce4ce23331f7300bd8c89",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x73eba0e9e52c3a93ab6dce26d5858b2d699d8401b2c43253616b5701aa803c5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x77434256511acfd027b41e03a571a9f56b0442dc675c139a2e1476fe716102c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x325f10662fc8bd38e591b0e8caa47b3fff46703656b2c5863d39c150d298fc8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3c835256339330b1c94cad78cfefda36a949b9c8553d918f3d8547cd1805ac5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7340540d0c9f9dd2c1142f03f408ab977afc7371934c62259fdd29f0652f8d0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x17d8ab17e403b1925b40206c11f8a6a29ed08217e1ef303906ecb354fdda1f3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x78562cbfb984ebea085472a1b004dbf86e7d99f4809a5020969246a84a9d165",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x60c93d3dac2628ad796e1dc80bc0796d054c991ea23094d699bffb43a630add",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x72bafd4641e6928ca65cb48e8001ee077944201f70d5bed524c69b709410d3e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6321e76192ba31cc63bf7c526c8ebbf4df5b705f01e4151068ee3dd658aa674",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7caaf4f7b073af26c036d8bab5c74fc3f752f9ecc01041787e9ddf773596189",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x66ada08fe725f364ca32c1055e1ab1216967856d6cd8762dd4ea915c2ed40e9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5b6477413bac2f0d370c0cdcdec4cea10fd322fbcd7b202d4ccbeb0581fd34f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x35c99bed31baaf7833ca759a9bea792965a87b42171259ac51b00d872d581fb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7ce96f5a3261a977f04ff70ef416a3d5c165100d19f551a6ac514e4d00fb18e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1c933d3449f6241d0f9d547db9e708fc2ee3e0598be5f87b675fb6736a15c39",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x687aac173963fe1e01f9e0d50eba0e95e1e8783eb21c0f6c1f45cd42408198d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2f2f5447e274a20d9d60615f83a18b2a4db300d5e199d7c8c6c6cfb754e8cbf",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x91cd6c3b8ddd8954a44e8a9cf6f7f183af8e6226849f05e6e6ebda2409e042",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x282ad11848887c771898b5a32ac6ca14cc2510830454aa8e194975e308fe042",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5455d2de2d7570fbeeb431a9a21187ecc049874b64a227bb543aab4af16e27b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2fcabf82bfd2529eac169a520cbdb2a0f8c205c5a9b1f1ac69bd3a44b25faa9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x33d4fbf9dae7a87cc13db3c95ed3976b50113f072e56a13e675e4af241bb864",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3cbf28927ecbda443555c9d51f40c294fb6688a17812cb0c85fb6501cdc0709",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4fefb86cfccbbea031f15d85033f10f92f2b6b689153e305bfa8821935979c3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x63f4ad31c4d59ee741b1b0ac99e022959df079b5b033ec7a1ecd3b4797f94d9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6c74e3b74559e12949b8c3b55369b2d275b2920b4442c536d63f91debd61499",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x45a5b88744e83d901f33da0d0de869381e7a125a6d8bd104cf72ded013ea4c6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x11243de8b4214dc3220693acfaa6b626cfc3b8c812140779af9b72dfb1b92f1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x123f50f65a68168d6b43c464270479801376ff6979b94f60252a47d9d7d34d2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x25446fd382a1b0f5350b91290b2dc35a6dabaf215d53cbb32d1732fc6ebfffb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x160d7d587f3a17673bf04189e0062c7bab764fb54ebd0f042fec72f953a91da",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1e0fcefcc1d1c5a69e81c4fdfe7de04d95b53c162a3b64b5956df8e59e1b93b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1925470cee5111eb991ccc8b0412be603c0b8df342d7b186a3aaeddae103bf3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2848af5bb20ab624881dc9244ea18b1d6939e14270714253a896e57cb0f63ea",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6c2a98464d6eb4038d55b57632bb283ab091eac255fd6797df41612cfe3ea1b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x31563499de399383464854a8679e0b073513c5bc46cdcc2a2107f00677e6356",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6da4d6fb1e6b2f1b42910a9dcc4702912002d7d36ac7100e19c7f298c7948a8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3d368784067d457e43cb63b3f526e721fab153949b090a99a128c5744fab4a2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x266b63657dbde655f034c014a8fb73b77138b52eb0e17eacbf402bb90305f10",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x179f2a40de3db251b95a60431e7cfe2dfa48dc8654bbf81add938e9f2f6725c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1118c09adef545b07e209d88b0a645673a103c9e71e8f671e74c84abf1a2a2d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6ebb7c4ee2d4212e6d7cea8c16f97c935f3bbbc2f400c9a738f1ebd37eec6ee",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x41d1ca4f756c80f197ba1635314a3dc756f9d8d9406af16538643d3e1021bd7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3129daa367a01a45fe3f0ccde215371f59c5643bfad33f4269a6478c8c8b7f8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1fefe6ebd886640df863e5f5c25e9b42fbc10adfa7ef07d1fda0eabafa60a6e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x37e79bd72d714d3de7ed2b1ba79e345f75646bf67efd8ea3050ddb357802a3c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x24eb6fb4dbac687e35d4168b970db6e7dc76c4c886dce0d4bad2e6544b8e6c6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6508b5fcc13191197f91407d5b1b21d321b7f311e55ede9ab8a6975308dcee",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x43e46c5f1cf3b5cac9722eeee991cbcf53af25a4a355a91ea9b8a4d4754d908",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7393709fd08807a84ca44526a2b8ec97bce5aad1adf00560d04110de6d9eda8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x690c8328ca161c48f3f8f37570e42095d1a0d9e101b3ec0ddc91426fc22facb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x41354bad5cbef57b0e7eacebef8f0176f3b70992ea5a418f502242acbc4a1ff",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3475f32b5bea9dbd19ec199ef34e531b696cac0461e644ffb41a5e99d0735fb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x33cfe02e240929353f193c6d3387f1117d04f116889f38d9a196abdf986e48a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x33e7ba5f7e56065e3f8b091578e8e7a7b118116de47237fa5a97e44e97b7f69",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5526001b8a8c2c6209e40b5d380836bcf63db4ef85c25fd5b72d749b0bd36de",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x64028a3945aa2866db68b304dd0d83d75ed0ba5c2f9d0b47e80d11d8da6526d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x378113f110b2404e7d185e920249519ded728cb1027fa8cc2843a588886a7ed",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5f4fd7854cf7b89a3983da1a39839d85c7331c3353b0a8cd218f7f4e1f780c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6467afeee167ea95feb4a85c48fabb2c7067de57acd5098692855189e21c57e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1e4bf5029043367487394808d7ee7df5ad1ad1da2c4710a1b2444ffde106f2a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x684a30c1084e8edf34a77bf8848fd2098459f5461bdf3352faf9c8801435b6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xe3f2bdc2de2b623c56390eb0044adb980766ae1a58d775e003c39724d1d6f7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x62dae59d425684ef78c1829e0454cd5e76f5d322ea8cb5ae5e911f545beeee1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5761ff2e0a250691a66dc36d372afbd6a8016726efe0c418d7899d60d26bde6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4dc27c76881bb820eb74814d1b69825e9048b1a3b064e603cca4bd4814b2243",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5a15d718a45959d16dd6e0b98badbb086e2a9741ac04086f078bc6951506e05",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x23cf69cfd7730dd096fd485b2d8bdfcd89ca6004689bcbcacbeff288f18ff9b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4d6c233f7bf3ade219a8e3a89e12d05beb7faccbfa811ebd930c391523f7b4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5a347e7937c7a178952905f499babbeda500a820ccfdf7f3a99589687a623e7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x43394095e27ddb7825c0671833a6ac9784f31626914c902c225f05ce42bbd9f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x66d91bcc591c880303ee4695475e8a8e402926f0c01ade8880c7b03c76998ee",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3348152349370ec1c4d753735ef255b50e54aa9a432f48a121c39b8887827e0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xfa17235a82497674de45bfa59e61a329b2d0e63eb18ab9b74aa46783e04c81",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x45ffc1ad229bf52b2531afadd1c5ba120c57b34def87149880d1e5cb6c5391c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x37f5f6b25ee428e91e886127b961856d9ebc52740ceb763baa7e71371b84364",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x58281d625ddb432caef06e485bc2b74cc077aea9ba5072198e76542f0c69dd0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3e35aebb590266ea1fdd8198cf3c23c77731dddd95d488a9d9f9837e3bd0f6f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2850977abc89355540e8abb804da7805ef88b12f40cbd9158ef330b767901eb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x72fb3298da88c470a2f93a391063810be01078c8375183b57a024c223f2f428",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x53f792c81d26c122898d70ed7fcfd8f02a8f5a9ec8b9868fc4490d3a46b4e8e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xa7da81afc9f3c93366b6e161b1fc7a497d6c770fb140bf4b64e5fc707cd3d3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x26601a459facdd83458b56099975d2b7dbbc431d41b53f5dd6ca2901dfaacfe",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4a048ce90e3a1eeedd4932ff37760fd8b1dc995aff7107bd66318652efd1032",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x287fed27ab81e5f721d2bd5aab0e69f53e94ce5dccc35c2dcc88e12465fadc2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x414215fbcef7f5af60f320e67a845e4a17b0a0eca39b4e18ba89fbe8a189491",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x208f0c2f5a114b2342f51e919e4fe44c2a42cd06382d9edc4ef58939b249bab",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5726e3dee7bbc5e5b4f3ad65f0fb17699efb5936d50ad380785f2b10fe8953b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6fa8562db8de26797e9c9905aa769e4881304b4f20cb64d718d271c182f44fc",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2a60396cf912573be2837653283a23702037f614e33e1c6fe2834eee9a1c7a6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5b6b3213744858ad659c4c07c9220380d63c01f680986191c8776eb703661c3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x709a47e72cda4fdf428bb9784f02f77c700086755d4bdb5b229d1b80a2ea4e5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x75e9c821cdd2e754759306283aa4af8bdbb0ed31f4e978dc550141fd10da6be",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2adfa41f72ba3b61b9dfa6f017b19682b0b0f8cd86be3d37374aba3ce990a55",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6e313bf82c34d3af1e7fc14d811dde163ca6e57accbe476875e4a967da00b8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x78516acc9d32f4e54f8925865c91f70b210f4ebc7533fc624685b3d5daa7b18",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3b8cca1c3fb2b26b7e206802d52d2ed1c725b8f95407e3ef295a7dd9ee0d45e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x287ef69d6f69ab853e4f0d24b22e4c15169d12c41706dbeede9fb49c61179c4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7c84837e6872bea4f0448183cecd6bb24a8574456ab91173b04b9423be8a64b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x78f890541865c12169233143f47a056a91dbd18222c5d31bfb2db19162c204c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x46cbc37eff4616daaa86160d5690f5473e24171441e29705ae564223a351c23",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x309f9698e38823c05e56d073d83ea551bfa80ace08e749aa4c83031a22360c2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1a2cd41155bbb7ceee94dbd01bd876140b1698f03b2ff8f8de3ba45b4ea14e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x10699899068f86fa3843b06693288630b9ac4b87be7b3726fdba32b41caac2b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2762878a5f6665bee609c26e750cd886e239c31caf1508d5a2a185b58576b77",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x57f7a737e643bd859d8a53e1b621c09be89fcca7b96f8e42333e46426f26a20",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7239c3b89513196e3cae91f8df8bd79f08033061ba63c089bd764644907479e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x38cb4f77410e9a33306f8a4b92b6f76bf239ba44e0ef45dab0bfcb75dfe4141",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x382114ce9d712af864a253d29471a436b83ee4f7b8ae3fe19ec3ab315e18d8a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x51ceb130c1908fdcfa6896756241fca8f74ab172d98c76facb7b8b931fa8812",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4f6b918e40f8022d2bda8d53214e8fd84743bc2280231d3ae772844bbcd1aac",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2d9af4ef0d50851ae1b0cdab3587a71728eaaa4e56e67803c5ff9126e722696",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4640cb2cbc73d7c9fd2a1783122cb5ee8c68e7c04b0b647d43a35cd4961e4ca",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1b3104591a23f262051182209c0f73caa30e8631fc4413a5bf97c9d51a70abb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2103ea7ff918748c4325a992c561b551b70fa9d97e48a52b3c157799d213693",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x69fb58adef701279dddeff71e1832aea01ae10a5128a9f744a5a945b5fff200",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x375e99f4993200342e6f6ad713711052d518e5dac24681b3999878bbad627d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xe051fab79733dd773d13f5bec04b1c20252df512d937f6b7352e4c4fa49cb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5d72a87fe662c05530c3ec822f925a10c121a44c4adecf24850fa2442cb4abb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6f2a9f716b1fa27c35675a57273feb79ffce02286bcb1e253a8e126c2cea357",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x9b419422a2083bf174263351640e009b56d6e2278552f9e7ee6a6004d45524",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3dee84f905f6a06940783bc3f322a0fc22a984dd244d00a85ea3a4295558377",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3dbd68c7c5945f48515d975002a1caf1c491c6743f151df31f95c5870c90fb3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7d52bb08c1d72a66c3e5c60f6742675ac788ec8b4f2178ff9990a04d22c076e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x107c8fc81a96a3c13d1ddf04b8bcce0450610c2ee6c127e0f47ce2ed2fa0613",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6d8f39b47e79d44503aa87a3fdf101b055f89c663bd7ec377d175280f3f8db9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6d5a755e91ed732dcf8afd32eac3b4875843bb116430a966ef88f17aad54c16",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x64035ce25716c9c7675ecce40d3cfb65ce3121439e10367fe29f2742cc02d85",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3e44a162d501fc521774c75994f4b55eb85878f5e867cacb75c7ff0b7efe941",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1b2441db55dbd9b87c45b1afba238ed28d1f2dfe9725d9a4cae3a45e3d59b63",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x8b54ed775cf8f3dd5b54fcdea07e2bcefae323f6212b8f54877a60e1f8026f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x338c001d0c722d793cc14219415d61c52de28d33ab8bfe5dd31674784f2b568",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4ea62ac09b98dcc34b5437f6bdb4fb9a681dac12d1ca7090011c73259dcef4b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6a226b1dabca8ff2fbb52f0adcf4267a47e0eed089774157f318b507361a0b8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6ba0329f670df105c31eb665f3b6f243ab5de7ed8aa59ce9b0683e6bdfd9019",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x77f62cb2f9db71ba7a9913be0a434ca045a26704681af5353b7c7860be6e774",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x770018fe3435297b82b391a3bd2d09151dd3949545d0ef111cdf9fece9f389c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4279b49402fada9fcb602f909bc138c3547baf384dfef9594e2fa488cfdf8b8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4dabf5afe371cde17b9fa6c54c1b38d603f345c58d4f66e06fedd8948b402b0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x15296a7d071a85f1358bb157d5e62b18a11e189415c16f594a18be7276ed2c7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x79708fbce6bbe1c862e988648dd25347d60c9e0981540dd81ccaf78054a12f8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x209866f9b8d946508db2df8eb9d30f65ede2c99ec8deb2e5a1b7093e9a62416",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4ab0da9c66ca2588350bdb85cc745b4c5e7226cf7c4fb69708cddf6e8145f29",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x54578b117a58f5beb0d511ba42110c4696f4fec165acfbbde208a4705045fc0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x55c9e37b0208bfbbb61e5e0e05c72111421b24b45ea53d3ddfad1cdfd243ff0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5889b4a99416f2f954450c60492129c5f7a36f875a56dde5188318e88d6032a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5919f2392a53f9b230145d1b5e6da28165dd1d8cc7d28d3310a805ebee721fd",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x43e49d0d9bfd165776eeab9118ea672c24a055a700e35a04426abe1b236506b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x40e394412097f7c06183ae2997707604273b0a4ec1add0030bd7e115c20ca70",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6337d741eb226911e37cc48087126cdd89f00941523cda2fa5e965dc4fa25e4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x64592b7d9a6a922f5cf5f74c56e167ec000436a6b3caec299bcefea25e5fdd1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1d608ffab983d8aa17db9385433abb0025c77e27357285448c4ff6a8438570a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x333793406d06dad0406a859ea2c203aff33e3cd906d6f04aabb0dffbabbc9c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3b5f2338d066753b2507a39884bddc2d0c5bef88e4bc3e79288331afe9a6234",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3fa0ff20cc486bd0b43f96826c66b070a6f6e3df3359ebd2970661f9c679e2e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x62cea6c83442875da8b98083d8bb18bce5d3d431a3301afc635600578b33506",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4835c753d4b5059c1b4186516851f562e63e348f8810714cc393be9810a1de8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xc230c4af49117fa614b1d4d74ef462211a5d55537ac71564ace080dd4b325",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1917c10cc63bc9f43116c3688542cd867e1a84ce0d3e58dfb0c11c4b0828748",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x13c6580dce66b35fd24183e1635fb6008a6deb6cb507bf48d531273d5b4c2e9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6c6dbd58b8657f8588bae8a4d990e6f9b0525af4eabe87512c5f6a655c92028",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x59daeadc724e9c227258a56b000c6a613db617da41bbeb694521c86323c93b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xead9e7eac2f6c388de28561955e6009f9f1ed098f70516f2bda28597c9ee03",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x54600ac31d014f7241c14e5aedefdc72b839cb0e98b84aa13f031316af48648",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5370b38ea84ca67c75ab50a4cb8f23f4017175a98b23df9e1c92f92c279e169",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x23ae2f35a2da5ff92426d59ce066e29a525ee1207de1c370023975b4403ac6d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5952b292edb661874ff2d3482fb968149f09982bd7a194d2b502ee3dd32927b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x30c7ab8fe6b61574f49c3d76b3173f76816f31beb33097d425a94beab6caaa2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4dbc9ceabbf1c8d5c679cf80d9bfc26ab696135792e83061e98b9c36ae6a4a0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3f1908469233dcf5c433790cb3574261ed6debca41fb55b912be7cf34adc187",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x51778c6b175ce13e994dc1604dac3b901990cbae0246b2cec2aecbe96dd2006",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6a91b3677713dc15cd110c71cb8e174c8ebd8d7df1a1b4120bb4b6b1683ad5c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3cfdc71122fdfc7807b2efe35fb6c7691985d2727401eb8a8132d0e0df3cdd6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5099f832fdec91fd27af0d221e009ed6770227d63bcee6e1802cdd122751260",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1793fa490096ddd67530e29cb3e8e9632d1885815be3f9d96375aa5946f511",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6d4bd8c4aa4a530d965180c18062d6bc440e6e70cbf0836d6af11235c7fde2d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3d7587a79c4ae9c24934a10a9c1398c04f3915fb6889b72b361505a85db2b69",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x39007630deec4a6eaf806518c109f4aff9cbfb8826d86f301e562ec560ff89",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x51e4a728ecd68dc30e4a1b5867a1022af5808edafc3cb12d26d43b495528f18",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7f921548c686f600b302290f692a66e9ececa142f691f9129c7d8bd2a06803f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x42cdd4f6ecbfd891fedb9ecb6d320f6adafdb274ee15cc11ef4c0436a4e9afb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x755d64434e4e4233388c34a90438764c568353cfde4311021b45e0f369b0db3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4b5e107dbcd02c0dbef4d3a77d66386a864d31109d0d0392847c8919d926fbe",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x373a1f2fbe36dae9a5f2c2b35febe59b53869e1678c8da23bd9e92c3c2ac0a7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x53bad76d22e1525dcec248b73438d6f444caf75794c26144e26803fe2bc7736",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xaeb135e456ad66bb5bb2b91a4aa429915f6f9951aa15bba78576744a698016",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x111d440a13cab69043e1072b61c1736cf3901941b4c57d7602b8effa7e74b3c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5fb8d87e82c3547e32ce316e4439d1aaf3723e4a906c91533ba8dd9631f1661",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1426a2050d240104b5c07a9cdaf7fce03c2accadb0ce98344ecb4942c434db",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x929eaf221c110efdaa57970581428d66d5866fb9547aab76e89e8971efc91b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xcb3220da969b95193a25d1d4d76d1cd1ec596040a7b31da7f64164809bdc4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3c3ea2cbc8ce544c6c98ad9053cb2c35326f4e502214e5f72c7951474b5a84c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x68892b41018bc73b541800d91f0bb2a8cd9fcfee8be13bacbaf7dff7aecdcd4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2a652d2592f5cc1197a206db79d06e3b74a55b1d4ec03c516a6957e87345cbf",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1e0c96f0b836d1e2df4e4063d56b78f38f2ad16040d61855b0f664c066d130c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x65d167bcce20a40b78583e4dcf7e3f44663e0c595e18f48f83ea4230b207047",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1668b919fcdf512b5683880ed048853e00f456adde728427fcde63ac9f59611",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1489d6012d4c9701b63f3610034fb5bfca185c7b01222907781eb104e031097",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x57ad5370b26ea1478f3fa0346d2e390e90feda8022c9820813d9ddd0f36e7ba",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x522f83a59f717f37b235c05338a02630ad83c3ed307838f6e795f9705cbc849",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xb6bca44a12ba7914e575f83cf8b9b8bcd4780622806901dcb9530ff9a454f8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6fb8324456f1dc4b423220d18d40de524a27dc4f35e4c780a042f6edc95f97d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2039c72c1c7c134fb300e82b104394f54a5b7ffe6f7f00e7c3e4ca6640841a9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x614a280377b9dc732773d969da5ddd8cc125262313eb7b2bd38b7668cdf00b4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x243a084aa8c82348102320b0ad19ede41b6bd7ffb3a7041339a13f34f6b5671",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x73e8c35fa646fce6bf10c33168dcf3d2e40af17ced70b1929826d0ca4ba2e99",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x64f6f50e51b19d5a90e6d2c9cfc3486dbf2b37c7f949cc4f8ac4dd988e5bdff",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1365aaaf8c72d7e9b250bd91ee2c2264362e87679abcf2df2b7a4e1eda1575f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x24a083b7cf164138ea0c468f33317d89c97b69378c906d918123f3ed5a02cf7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x25ea89f2d7ad620296fda2be181b5a6be626eade8974facd81e53df842c125b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2d32dd179cf74693057ede607e0054fbc3e4194efd6415156f3ec909c37ead2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3b74f537f03a28e72bae3bf1810f1a2fde1711eacd6bc64bf55f37b3bd9940b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x575b929bc0caa43939bfae95a6d5cd8d4082be7fe0934be4c08f7fd3cbe89c5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7ab2a5ce120c251b658bfe532880535e93cbf88aa60a1b384017195e6715706",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7a7d5ce80c8498175cdd4408e08cea457517e37dcba08d0a6cd2a4defcce34d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6475aad2a1631a6103b238548fe8a03934779ecadeaead2bc20a677c0c71c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x547ddf1021a2cbacc8081cbe3a5c89b8ae808942513cd6f6ad166b0306cee66",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x653405343098520984b06f707cee84ea765ecc932783cca87058b88d0f2bbe9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3f22422d66d77bda123b47b7f5bffe5527f95d331346f6a545c66887ad75ab9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1b18b73effab8a483156d16e87be4dfce1250333eafc784d76c6ee145978c48",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2843bf3d789d84faebdd6ceb0eed3ec0acd959732178b00b4242eb5cff0ef3a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x592d2fbca1f86935e587f6cfdacd0a221237bd378e2d1cbadc3d168c7a1756c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4c35c95cf7170d2ab6b9b6e3c1be66dba2de170638f27975fb5ec12c36a45d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x405f9011670f0f202814795cdf0251b665e8f39991dfe2282a1dd2acdbaccb1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x43b4be816239e45b4d22123c840717fe3e8f6ce53238fad4ad56e27c85f3e9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3a2c1769a49e0632c149dc9d3f30306f9d9cc00cdb426d58b2741c804c51af4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x422c03b47f25f698d3dfbb02556367c97b7d8e2657af2e45ebc61845aa2c52b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5a47f80b2d6e8c8e89f08c23e4eee09ae23882290a4dbdc5d0b09e713297124",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2059b2385d435959cedebbb68ab5c484441832a20d67889ff9974057cdbf874",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6b735de6be3ab4aa1425c328c838ba09dec586718729f1e172554cac036483b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3c05c93a63aad66725d8d25e62f76199a1e9f5743577777caa05832f4e79acc",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4245e03d0378593b2d4230b945a2a147b36ebfdf368f0dd5fc22e3b31ac1186",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4de104ea20937d5d6cb02c4ab4d7c4d03ab2eb16d1b837ccf0c2a05ea2873b1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x48876d457eaebe03383add02eb4c0c49a09923757428595a4f3ad6299d69cba",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6d6ed610ff1347a9252bf835af9666acc415b28796d968ab76353cdc1181733",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1da9b14257c5c5cbc1a97aff87690dfa51e82af9a11eaf5cb2538f595ea2105",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x416057baac3a1780d7d25b192188b9b3981bdcab0e2dffb2fd95456a5313201",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1706af2f962881d86f167571fcbb909b6f1e4fa386fca8d87b674335196f44b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6d9c76938c974418e62166285ade6564712e6a263357e11d70f3e1f2ae531e8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x771fefe011becb392f5c379dc9e902c41be8f1069ae3c5e0bf6016b7b1b3f55",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x40cbcae9364d8af8b767a72b260793922cf1ba2a03fedfc60d4eab1d5f00042",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x274b54e6342ced28b28c62edbc8a6cdb44d1530e0fba56e4940e55d806f437f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x77273f7030b86a46aee79ed44f0968feb0ffccfa0964ffff141e693fd0fb6d1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4dbe5188f23eedad88bab99323be5ac9bf747525c23d4c0665334dafd1f0c6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3410d8b91297b00cf8d438bea18b9ebd55ae441a2f6bac6623a15e43ad64d4d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4195cb2f46ca4e1ef5d93ab3a5decbdc9e74d0bb81d56abcf59304ecf79863c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6581a70ec64b4268a4741b4f7de866050d31b69005c782630f4bdc51a1650b2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x754535ea8702292678b57fbde36c97454994bed59e0d0e13cf8a6c3ef7a0324",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x71034c062fdc1b61e812617b037c5dd1e80d158a92bdae7ccaec162fff4edd3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x297d52739d69b228b057588496920930df6ada28e5e2a431b65502750a5bad7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x446d7e2595a1940ab7f6dec4c9f78953de9c0f4c67a130b55f1894779e73ac3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5b0a8465067d8f43cac5dbc1145110e1e79e0f32ba1d59d2514405a0a806860",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5bdc50def36283e003e9ccf2f1bed188326bec8bed554815f9e49062ed6da4a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3b591c6de6700576abbe4b4544de71cd3266a5dbb70740762d0c16a863bead8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7fe75f49544ac3cf237a17e58179851f5b3e7420330e5861ec505291d9a0380",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xe09e3870dab755cabbeac23076891b510207da569b75bf32d3f63c8ce08460",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x66bb11e034bb55410211b7cd410cf076db77f008bd93f0dc938f089e853f0ee",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x630cb6b8bcbe79e58025a699d489116a875f287fef6f1677b497b8702c3777d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x46ebc0bdf94c2f85023a0c1b29d229ef7a23e173d310b814f72c73904f6a5f9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3fdf21da099da6c005b076001c5a95f2fe26aeff47e2cb9e8e52166a22b643e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6c647f1e5e8e93fda4bc0ae5d513cb60558e2b44bf885484161bbfb5e093969",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x688dbf5c443560c219afd8c54a0b26bdc9284925f2cc0adc889c1de024d6ecd",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3c782f4a1a6d94adf1448fd7feef975f47af9c79bbf7e2d74940673704b828a",
        ));
    res
}

// Evaluates a periodic column at a &point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at &point**(2**k)
// instead.
pub fn eval_ecdsa_x<F: SimpleField>(point: F) -> F {
    let mut res = F::zero();
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x22aac295d2c9dd7e94269a4a72b2fb3c3af04a0cb42ed1f66cfd446fc505ee2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2bc4092c868bab2802fe0ba3cffdb1eed98b88a2a35d8c9b94a75f695bd3323",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xf685b119593168b5dc2b7887e7f1720165a1bd180b86185590ba3393987935",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5febf85978de1a675512012a9a5d5c89590284d93ae486a94b7bd8df0032421",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4e1b2bc38487c21db3fcea13aaf850884b9aafee1e3a9e045f204f24f4ed900",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x324182d53af0aa949e3b5ef1cda6d56bed021853be8bcef83bf87df8b308b5a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5d4c38bd21ee4c36da189b6114280570d274811852ed6788ba0570f2414a914",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x529414d56e9f6bf4ce8be38c8f79ffab78b185da61d606c411098f981f139a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x66d15398bbd83688bda1d5372e048536a27d011f0f54a6311971822f55f9c07",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x52e5e75be2c96802a958af156a9e171dc7d5cfa7f586d90ed45027e57c5fe92",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xb0e39f10e5433b2341ecef312e79ed95d5c8fe5a2e571490dd789dad41a2b9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x703dcca99c0a4f2b2b7f1b653dbbf907dd1958c248de5dcb35be82031f7d170",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x749e86688f11d3d0ef67e4f55535c715a475ceec08547c81d11de8884436d8d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x44a14e5af0c3454a97df201eb3e4c91b5925d06da6741c055504c10ea8a534d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6cd537aebc479350e63acbcf7b9da84f4b06c6c26a571d3a7dd416a94a956ca",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5e2909b1136e1d6608663e5cbabb616b28d2fd6f5dfb7cd03c4a7e719b7c53f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x51170abac6896de6a5b478741dd56f52b1d2a1feea59b1f26d060e09ed98b32",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1d0f94ce5d9d3beaa42ebed05a2f172aa2227e9a9fee0bf43a3fb068c1ac345",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x785dc572a88712cb4eddcc8a167bb1b62f9a79282f21ee92a0374af76169344",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x39d9d83e0ac884a5ee0f2d227f9eda71724a55002a41938458e45251e121308",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7fdc637318ea00385719f9ce50848d13cc955eef9f36a90b87e646dac85e3aa",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x72d82458ba49cd6c638f89d2e3a68e49944f486cdfb7d2848e51aa9f99292a4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4c484b2cc04747d8d812180ec716f779302231983fa17971b575274c0a9c378",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x62773dee1773834dbb324c4c0d48dcdf9bbf0511547feb1b2ab0f7af7fa2dc2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5b0343972ee9e17afaf76adc54e6797d54e6e47a7ea1167654ce076e3c6c360",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3ca8d84242dd2bd2a5d6e644fa1dc9f5082ee6131b6f0db8fd7d4f87109098b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6f4ab1f3bccea47669a4c93da36db05bd6f5197945b5ab29191a703312ed3a8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2d626ebcfae2d3618e350c190fc636495fbb04dd4a4e563680fb961a3d30d8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x42f0a74ce045e8194b7a5cac4e882b1f1a9face49c38fb3383cfd3d960806c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x34b7ebee71c5876183407c57610a0a8a33d3138ccd6ae416651cd505e5761d9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x150c633a21f3cfa157978e9561161f3953e180b9588347a0c819e4173afcfa8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4f8cecab5f743c7227a63fa7f320930ffa7cc52b0fff6c351d3e9d4c22f9f9a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x98ad9c2080ba0663fb302025e6224cff41d1d30c5c9101ad77a48a71d8ac",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x58a6d8229d82c192f190e55d28489f621cbcc64e4ef10c1ec5663c5384e60f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7850ac1ef437d1b99c026a910b2437c1b877242e605c8f31a456f10e2f78743",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x337092590652e19c23b48de3629ae0bd4157a5a72ecd3fcd17bb93f05814716",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x95fd265a2a87c42af5a20a199e6730ee3f0e3352a38a5e7e84ef46c621903d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x38ada3df52cd03154d66b7da4a8a01835a461e61a76ac9576649d8c00013610",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xc1bbae3cf2d414dc12119a0c746e3c10e148f8b522d574eff757d44d8b3a14",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7122e4b28d4ee35902b7f7b8ad5f525b6c70a2f2bb6b4ee4b9f0008845ffacf",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x75275c33b919425b271966642fabd9ea7c917e70e96eda669040935b1d49db6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x70af32c484244d3435bb65b0ed076f48d06abb45b7765de9c6f26c1c8e9156d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x47c78a993a13204796a2fca3b20c0f02c0601e7cc59f84570fa026c65796dc9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x55713c4cc9f91e9f158f70683238853d0bb7cbd8358ff72b01fb60808b5c1de",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x412fcd2551c0516392f685a62b54fb82b9a73bcffd42abecea4482b65aeea47",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6925415cd4dbae0ea5e9f41edcb503ff6f668da1cb13ec73eab6a99cd96752a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3a2a902a0e43ab33c19459984fe116fb215796cb40c48e254de6126b55e9c3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x552e18bfefab6c3362cec587f0a7433a914f1359e5767b4fe883f1ad902dd13",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5643c5a69044bb8e86d10d3248ea3f50f8598732b0c517b256fe108294e09f3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x542f931640d9010e906b7e1e375cd0481740157eb51500ea1e10afe77f26265",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2a2811098d68a747bebe9ca2eae06b604bb307e5f51a9bdac1636f380feabb5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x57d53073d66a528c88f24e40011321f74ce5bdbecd6ca319e5e770ae29b21da",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1a4393bce3924d765902469c715fedeea69adca566859b4c8c412b7d7cb566d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6dfc1fb08b981f73911dc43811caa0ed99749c2f0903f87f389c9a0e2a88126",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x224fe4f546c8f999947a5864ed0dbcd64fcac6f774ebce11667c2bbb7d8603",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1f7d548c5a6f2bc70ff6f8ee47f38221ae25dcb4f9b068054ee66227494f87",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x735f4476c2b51acb4f0dd9dbc4306108e37543538b2cd3cd2327ae5377a2e5d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x66ec70c796374a71b6aec5520467ebed547f645d1670b990dfa680a1b415cd",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x14ac38a4b82b4c65e4993726b58f32c74988997b8e8f7729fe9032cf187896d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x27092905558602aec9af09947b70bb974caa3dd7cb1cb991810e15d75194aa6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x44e3645cc1b135410b2a52a5b92bcb454985033615453a51ac46377885c4309",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x352b933e5d853527d2a4317db613d07117fad8115948957515bc07d72e161f5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x268c1e10f6f9969291b1d2f54289371a2f40a14cc67b3736e04eb891c1824ed",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xaa81707e389769aeb31cc8b45276af0370dd702ac79461bae0a4078cefb5df",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x67dec5ad6ddb1761ec61d2820533f7a2bb56d66f2fb8ecff9cbe28218990061",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x106911de08ef437acabf58d178db7c81ff4d7de25f3ef5cd2582f44176d449e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x356591a80d5c2e14c3d8a180c030a9529a8580a4f3be00a5a9eea83d0d585f0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x40a3ea8c4059a1b9138884234381d6d383e66dd48eac1bf05f5fcddd593c881",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x699e679a8f38a1ecb14c6695a2848c6abbab8a05003e43aa5cf4a9c6e6058f2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6b86f825e41b2c9934f71cc2cb08787d1bd4f2eefd2be9c44e37bf387b35940",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4eb2786b11bc602bbf773564eb9b057d7dc02daaf4359c015295d97b74e72bb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7b0ed28b968689517aaa216c0203e57f1cf56b22ff1213561499ae140d37fa2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x367ea925556a875faedf4d61bd2a95a31067bde6e682c50035bb3310cc54b03",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x781cf0ea1c0ba9cf908656aa2c5a9403d54c26c8ece401a2c13be8d3090f9c1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x20ffc2b4c6c318bee0cdfdca40b2c10f2c629d3b52472b17c1bfd909cb7b85a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xe5e89fde76daa211fadf1178785f0c25a94d47a468cda257a895b871a928c2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x164344bae5b9dca8f384612e7351fecde28adee3d245c98dc2f65509b181d8e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4063a6202df9488fe5384aaf7be7610b3e88a9c01486c1b88767ca36355340",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x546f65cf3367a004f10e9a4e47d71f6ec80086cb2be19d7b225825e01eb323",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x72c11bd84cd54152607e4c6e558a28e480a6487e374b865682c167484f8c29b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7a5d11f284ee7db72bed2338784d6467e05cae85f333e05c5610c018a57c2a7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x64c71feb673d2655bb1865f9c4bdfb16b1bcd0f278a911363056674dacb812f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x62334e7d6094be4431aeebefc420f7e656459d6fc2cb10455123ede054f4cdf",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5dd4b3dd252fa7eda7b46674369a2f8c5b00a891cf01ada0ea5aada8bfbf6d4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3373dcd7d0f0f8bb31ec396e1ec67e1f121121356dba549bce9fd4d3bbfbaad",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x42a6c571001e263b1ec8168805bf4d6cb65935cd0687c696ae3a6968fd28378",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x16f35b8d34d425a85fe48e66632d3e4af27d5d65cb180cb99047fdc2b908ea6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x24327b5849aaae0d313870c10e8010a115b70a99cf6b92925f51d2f05686287",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x45ba7e524d75c65ab27b57a6e0b90458c9b0eb651935f84898a5d3cd0db9b8e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x767d8839373a2e97b7e3de1be6f4c18df648806920e92fcc4da9ab6bd8525ce",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x741b0f4e1bf8ed4d6318f5dc5ebba8529089f5ef4a84cd727564c60cc11a96f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x68682814e1b4dd639cf396a9f60efe5ca035c6ccd75054b8911e8a15230efa7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x68edfc809bfa6534b583624db421a2cb885d2ce888e6f95eae85ad9cb38249d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5d6575134d1b37e610f25e65bc8b0b1ad7fd0cdcaa56fe573142a09707640b5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x327bd35b3ec38fb121c039f777669426d3d60df3922e688a408a06d4e7ee3a1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x29a66c93ef1fa5ac4b6f96ed329810085b294a7ab8e16c61b1e225fd7406236",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xac01d3129d24fe9b9209df8bfeb2526bc27e9c27d78f69eac16ce151b13540",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3a967c407600baaac716275b8fa16a08c22e928d895c762b2843d00496b3390",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4d111629c799fb16f602183ae372aee382e0b401312951eefe77a1674575242",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xa8a00bb9874fbb44ee3411814dfb9d4d6048f5e3af6f7f09fff4e9f0263901",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3abd943152451107f59aa81194e7bbbe37c4a86a6b41e20a02f8145dd32fa87",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x580bd7107af3afc93d0cfd1f0bd39f78f06ebe3a900f5d79943c25e980e5653",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7a615360e826e937db0c91cc1c9196086a3fd608cb01d20186ba1ce856904ed",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x21df6648e6f783b7361a20191b8d399a4373dcbcc83f6b4a9a40bf11956219c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2c82b2a99d198138ca2c4229a1929d044b113c1b0f693659712318ca7e7f804",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6dd74321080cc46d816a963c8a6f5dac42cb11e66c79831efba77433cce0d23",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1e54c3a5a3beca7932090ff58784aa43261075950feaab0e2a840f3801b81b9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3360af40b57c0a951da3219025643a76516f85119dfbb05f61874eb3b56b130",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x164d44fb88efb41e301934bf2c61a20e41c9bcb3f8e784ac5857063b4fc3d5a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x46efbcd0bd7f06d59a430ddeb9f239d66a24ce1fa72f5dbcc2bab48b707b2dd",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5ae517bdefe7b6785680842685de0b5cd972a22dae9ceb50a6ea3665feb06f0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x51f4698c121db3db4a5244334c5180cfba256dc80a59689e2c0f1f8d946e6c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x67d2681fae96c0b4bf22d10a73a1882c5bf4a5440f8d0458394d514ff7bd18b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3dab30754623b91aec7a165cc167e9003269ebab3e551781e4c8cfb73402de7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x44be18892438118a0b3fc099da7489a89cffd4206678abfd37b1e649ad19178",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xdb0ad3bd8a33b8daf1d53ff8604bbe5259b6620e3b547d5c6f392dbc10ccd5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1a36f20817da4dc0c2e8b62fa08ce15cd3cb50419acf5211d6948bd6b28c8ce",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5fa6f7f2a7a527880a5b58911dd7f3a491fc702f481cee30e67c4980092f851",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6f682eebabbcbfa3e7084b47b2a01acb693865749df222b4b8dee0ec41903cb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1fd7088411b30cb5762147b1d6749942485b36c68ea32f60ab83fdcbe987d83",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7172b43d0c88348e5453b0b26d54d4a7ad7e99e6b0c4b787341c8d89936197e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x34369f479f013d44dd5bb0d79d8a9effdb2ca36ce8b3d7e759bf707233c5bbe",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3054d35b59baf5b0a2078c23322de031b383033837cd6b978b6c060120b7fb3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x23f0124cd1c3f3605fa1ec36dc4d6cb6e229f8ba8998b138a44595f96f3bf21",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2300892e3f3c180333d091901ba99ab9e23c7947309b9e88ad47025847ec3a0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4182bea2ea16dcacb0194876cd5fe8c79e1a55836aff8aa6074d235af5f7b29",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x50f3e383aaf3533fc91b9633386542798abd69b79af893f47f6603d3cc35ea4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xc37f91c81a7006d6681cb511dab2e4d83928ccb78d1dc72c4c556e4cd72db8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2693f31fd4bb5a1ef9cacdc4f2b33c3d6d965b76e7bf289020ab1b6c6660d70",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2bebc90c59dc0e37e28c7c7d8254520ce08894637bf1a089aed26012690d119",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x17626d3869adf0fdd3fedd48e9fe1266bb33419bfe9046df43c6409b440980e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x734438bc30566591da45df9366f936415d29eaeaeab392488bcccb9acf0edcf",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x22a7b1c897f54da39a1db61b345b234969e36ef6ba0ea02f8d8b3e83b5c6242",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x22eef827b9d0b57649233c5d527b4641decab31df78347a20da21c705df093b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x193185be6e02dc0a07c0dced4ed031bf0a406219cce325e76408123406c318b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xb10494024548b14df121b738abc7babe56c12acc0490699443426a52f3a4f9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x375ce3766894524209e2043a150f10ad0bf4f726e3dc5453c3c757e56943a51",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1b1c82e5c561dc42f8c9c2a9f7db6bacd729b2646892a8ecfae9ead9a338aa6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4b540d0085be455b24f014bf51dc7d0eceb8c93bb644a5208fa02dc58c718ae",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x17f2709d2719458a9bf72a2b04463f0a6529fd9368a47715c628ba4e006cea",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x20e468bb2828fb774d5ab538ff7f93ada201c2e392936e05cec29cd5a7a462d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x29cc816e6be353f6ad5e2c390f37ed3940b0dd67610a7eeb0bcded94bdcf920",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4d3b0654fd74862a92aa716af33b5ad5ac20dc0460c724d95ca94fe6d8a9d7e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7ba5194da963f8224987db2720f16baa604ff62351e66a63c0c9dba00fbc7c4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x31d335bd885c9cdf2adc68ab45b8eecd2d3588cf85b93206896b2626eb1e369",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4efcba706a8b7868e32f363efac2696ad0625d046a3ef97917c710515016386",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x56017977a273ad0e91c7c26a702ae4508343e97968295b08447b3cc7f20522f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x16416cc193a5ced6ff213fc18c86bd6f08d17c576f26b9ebd00d2653bbd6444",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4237c41364975eb79919303fc0a381b934befe871fdbd72c18f97627292923e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1b389d976c22a3bfb42424896c9b135a3794048724c729968f81e04ce414194",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x62fc206aa283139f7451e54cdac873fe86b6e7e89214a3c0318fbcaf6016fa4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x88f6e5a835dfda9fa2e2ff248d9378352f4a89b6bf5935700da390baebadb7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4f9e975176d3aacd79c322d013c854c4b8829d1e469c9b242461f35e8dc6fed",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6e1143b147dd1bcc56dd43e6a3616c9a4016d6887cf0009ebf9f9796efc944a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x284c547c04ca83fdb01020cfc797eb362838317f09e5d25e1e4eef353ab7a7f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x60c327ef73c8468805ecace45a33ccc375fc91ffbf01b4b10a01ffd4b7aaefe",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x59cd87f8751437900e984a009c63fdf7461b177067760f30d4f648ab271660a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xdaf5a68420fa7ad811f6dc75c5b4e92173a5d89255dc75accb8cec80a9cd91",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1f573af6e3ad146eeaa582f540de6a8db237ff2f28423660de998a4275bf4d0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2830a6edb344b7fa86506557a0b2b0bd900429218fb35e7990951fe4fe869c6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x58f2e18613b3b25529935a623e7d5c8318ca9ff3fb180f16f7454ca9e348e35",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x42c8f0b5507417eb48ffeb1a7df8808633f193c27df8e2f44ee7bd62cb2c3bf",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x50d603bf9c2a456b828ae476092affde072ecd878877ec3f99ba8f574d263a2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x413fda31150aa8462deae8a6043fc5624599fb7f638c4d5c5f89472e1223c28",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x13fe84c8ecc2e3fd289560c0ada7a251fdd5fba24c076be4be465feec4262e6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2b2a0768e9a5f59e7f33ea449690794c8b409bacd1c808f7ee8065ed9d8648c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3030332e9cf430f72159914e59ab9af532bdfdafedc1be39691256c8084954e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x16617a52bfe5d2fd0eedb0d6411f5fafeb14a4ac17da0cc828c914acb500ce9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x553f8ab49053432bab53835480b6f4c416eeffb3470fb6bcf122741cac3d71d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3939ef0e572dcc3b67f0cb819fffc521df26e50814281621fa6982b1465f786",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x520b18e79de342aa7095ffe56be6222b0d2e44fc3c676a5c994f24e427b45e2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3f3ae3871460ac578f5030d925e91c138f3290f8f3cb6d4b560b4b16fbacd64",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xbffb0e4f7ccfff0cee519edd1004eefbc47024f92c4409bbdf688c133ad285",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x62d6874b6dcb1c4dc8ed797b9158da4359c6c49f27af4851a12908ecad2092e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x511c0ad7c0bfdcfcfaf925895a8ef5e8c5e0d147e29c9cdae45fbc998fce346",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x25199c11f7193e07191cd9b9108aa8b440ce1972dd1cbe5f0cc33b7783203a8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3cf3b95ba351a72019ed1bcadab32116adcf079e72800a9d88f15244e7743e0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x19cf240d04f4859941f9b6af4a7088729aa10307cd08aa75f01cb22e872543d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1cd528d070930aef19e0f928fc744e79ff57e227b6aa1bbfce15a79166aefd8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x475f8af086f7aa4ec3739f754f7dd291dc50decc7c7fb03de8aee3cf06824f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x28f86fe2d71f9410e14c17195ae19c2c5e623c525c979f4f74dec3ef8848eb5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x508243aa19e23cdb8ca0154055c05130462908c6a2691ae522e37ab9d6168f2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x37cf9640e321e7bccf1926d5fea92918d6888c5805e27193722995233a4adc5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x66336e2e2eeb939818f861fa4aa9b2576936470f511786f8fa3417850a6c2d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x19a0ff21908842e412addb744b0ca384a54bdde819f6337c4c672f682fea9cb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x44147236daf669f8a94b7ea353c3dd7e64312ece01ccc1d4dad67916591d50b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1d6cb5a655919a581078aa2f8a21d300425026ccd7d047302443d78dbc67abd",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4d8d9b92b38a45147bc9c87c071672edd93cbf5bdc8d85e608f26f1d82d172b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4acd125e74056ca611a1b07369166eb5c02af7a4cbf387b2bd584a362fa9e60",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3b4fdc8d965de1761e445ee88cb406f707f9d0b1ea3c069d12084c0ccba9b44",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1f27c20f47daaf01d4627d5e9bee0e9bd2aa5b75807064cd60ed87e307f677a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4758304a75f149e24563c2b22459151389b86d36108f5dfe11ea1fc7a64fd7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1c1216fe648d287c2645dfc5152e171f25483df5ef112b745c2e59b5d9ee07c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x24adf288d61c113e28d9a298d2642eb67586019adcb952abf274ebe1d30e24a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3e09706cb43c83143c9dc46f97e0e1ab4327de19ced69badaa8b2c80f68fb9b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x589a2e11637d0c90fe91bb9f4d55a80cd1a2df7f3431e8b8bdce8fe7d35126c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5f506aaae7ce6d94712c9e0ab02bd2a4ae09600608d54a8ca381b8e96222cf7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3f64b3a307276c6a7169c54297bb12aaeebadec98df6ba1184492a82effe353",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xc3e0400cbde1da659381240d9c84b977eef3cd70e3e4a1a8763a05e682eb3b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1fdb038204ac50e87e3e7239d8c1c0572893ba98e031c982e545e6de64cb8e0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xe0b21e37008355c35f7aee295a8b2b72465866b2bd68e72d36f032c34b38a0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2cc90219912af16cf9a39f57f8b8c514f797dd5d49dfed5eabdc278e31106a2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xab2147a23a826d5f7c6fea5bf889eaafb5531721f31ee0a9f02fd58f09f65c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xa5d4606609371577b0d17fadcd85ce659885b00245a67b038f902176d99a7c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xfc76b77f717a5b3ecafafadf29e7f886c8ae67a3a2bb30467c440472349953",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x40fb948f8a4a10d2b2e928a5d77b481f8d3068b47fa388a3ee65609aade1a41",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x13d322a0ecbe1e785921a7aa6f4d1135e0798e72f4c055226205314b8348144",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x20096a7aa30c6c42f1d5f1ed88de275d1d1610f2548711a75fbbd72d373a50e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x18f9cfeaf2c33e21d7c6fd9e15a3601a2fb3905588868167566e8c1f1dd30fa",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3e3aa48bb5db9e2b0dc6d294009ecd5d4ff6255dfcdde3f5b4e545032ea9b68",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x425b03b0356b92e66ca816869a76110d68862a0d8ad76f950fdb1d5c03279d1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7cdb723061223f33289237c7476e737ef0bbc5e2c1ed9a70566511fc2036ba5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x18b8b8d0f393950c9a2e674052150a328d214618049c7e2f58cbad76adbfbd5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xf19faf3accc43b56369dccdec35dc7b49c5b8f8976764886bd16dd2e155f92",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xe08853aabc9eb934b4470bb4ae1dbbe90c61d2093516df998ca7adc98afe10",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7736291268c775a82caea06004d53edb829be2566fc7c4053b1d850a8116cac",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1bc1186238f0d39e1c56185a8d2bf00c90c9c89647917d60a5b762932856524",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6cf772fa8050ad8eb87bc8f0c8fc511622b416fdb084cbc93b79501c96b0bda",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7417da24519b4c55ec0d698ecaceeb49711aa1e7f7d907102351e73388a0fa5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x66e1e25d1bcea87acd136f2c33498e3223fbf78bc6cc816ad6aaf68e961da0d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x60db5bf6f060d82c169a1c4ed6c548d5e8cdb6cfd2e3257c155bf11f48ca609",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2d5447623584d3a19e9993814622d6369248bc61813f067c4825c9b0a81551f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xac068a1aae938e26e125b35c88a87130044bf3637bf1acd797103e7388b33a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3ab2d353537697d4de9c5c4c0bc31e5e776cb93181029144f6c6d4b5ea4317b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6ca2dd473297a2852e68ea2b83faf8f71e5cb471adcc74a858132c6a823f0c0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1ec5264a5287f1c6de79b3df3adbfa157e8430e594078c3fba7002a077db447",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x50ee695deb5a4e63c5dd6de35621d1c0c5a496bf41fecbaa929b2b3e23f174a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x36f446f7e5a51114cbdd3b460431bacb5a42cd61f4690cf5e9d9f13e488318d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x195f98a85cfe403a7d229a6eb4533a1fea641c331db75a5807711fdf1e27dac",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x63d4964faab567e795024a17032ec564ff221a421bd2e42632d3770c73dbba1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x68d729620eca6b4d904198a0e6d241953b9b8c874a10b5ede5596146d560979",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1137975bab819ce0cbc73714305030fcd4a185f71d46c169908460390d56d18",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xf639bcd7777c1ffd41a693ac9f5a051bd124b7edce3d568f14304c9fd90a67",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2322f8d96071356feee538e0c53d857b1924134b94377af20ed5d0e8b3925b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x22cf65c6bbbf76765555748cc1ae91c83ea93ca2c8b34a59332567b5b3b0cd2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xd62eb553de83e5d51f78ddd9480d65870dc426f61153e732eb6cd62cee09cd",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6afb39d46d5a846e9d58a6ae27e6cdd83bee29c72754cd4cd3d3cae423f5c9d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x364889e46da58b66c827835a0c2807338eeb4431f2099f490d13bbad0777a01",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xaea6f7f915e4aec612029a9d02316baa3f6297ea4cfd38897f4c9859ec485e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5c180e2fbb2b51e053941d0e1611424fe60ced6d439115dd98530c8d79cca4a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1ab93f16e576b6a54598582eff5e2cfc33baeeb607826579680636b05046d16",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4de47e973af27fde9ad29f812de8a04855110118eb73fcdb46865390486a287",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x50be25e516e30f96d8b420a7c494506d2cd21d64f4d5ecb67d58c2ae99bf5e0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2aa45ec320ea12beb804e35af3684dc981324dc9bd044592d1c408c052a4322",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x26701dfe3cc76754a4ab893fef59886a43013ea6ba648efd82fd03941fa2910",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1773ba95dbeaab6e5e9fc79ac153d46be1e57828e92287d698a3f4f87ef4984",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4e92d5f575fcaac9adedb4e0c3549dc18f61bc40e3752e3506f3761c32c6e3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x580f9d95c2bd746c9210a87b0f9ed275afee1dde7a41d9ad5e69861ec0e43f6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3e2dbef5f162784e13b5ff4c33bcbc444ad1546922b293d6783b5de5c5aba78",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x596f89b6ca79194eb6a87c17692aa491f5b014da3cc7e5f05caf4fc1779c2dc",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7e84842d5fff1666e01505f62661bcc822dd3fa530ebd1e4089230a4045a04f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5626d2ae9581d1d335bfc3863a4eaf3568ec8e70fcdae93f50a15b0cf601b6b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x68371fc7cb3e0670a73eb3a7e773ddb63f231c26bf25bb1fc1fe6e93a7e3bd0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4d215dd42f87632a9cce2cb95081dc731e36796c3d2847dc96a3554231c6aef",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x421fac0e48da8e6355c07f6a64bcea96384848e8ea9a7113ab45f15b1dd15aa",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x679061e5f453c8bb1855dce8f7d61f2cb64b15d2c4e70b969ec4ead3fc6a226",
        ));
    res
}

// Evaluates a periodic column at a &point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at &point**(2**k)
// instead.
pub fn eval_ecdsa_y<F: SimpleField>(point: F) -> F {
    let mut res = F::zero();
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x23a2994e807cd40717d68f37e1d765f4354a81b12374c82f481f09f9faff31a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x49d16d6e3720b63f7d1e74ed7fd8ea759132735c094c112c0e9dd8cc4653820",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6c930134c99ac7200d41939eb29fb4f4e380b3f2a11437dd01d12fd9ebe8909",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x347dfb13aea22cacbef33972ad3017a5a9bab04c296295d5d372bad5e076a80",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x62e62fafc55013ee6450e33e81f6ba8524e37558ea7df7c06785f3784a3d9a8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x23b940cd5c4f2e13c6df782f88cce6294315a1b406fda6137ed4a330bd80e37",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xf524ffcb160c3dfcc72d40b12754e2dc26433a37b8207934f489a203628137",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x603e3a8698c5c3a0b0b40a79ba0fdff25e5971f0ef0d3242ead1d1a413e443b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xa401d8071183f0c7b4801d57de9ba6cda7bd67d7941b4507eab5a851a51b09",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x37d720cf4c846de254d76df8b6f92e93b839ee34bf528d059c3112d87080a38",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5057b804cff6566354ca744df3686abec58eda846cafdc361a7757f58bd336e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x76b4883fd523dff46e4e330a3dd140c3eded71524a67a56a75bd51d01d6b6ca",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1058ff85f121d7902521abfa5f3f5c953fee83e0f58e069545f2fc0f4eda1ba",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4eac8ffa98cdea2259f5c8ad87a797b29c9dccc28996aed0b545c075c17ebe1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6e7240c4a94fa3e10de72070fd2bf611af5429b7e83d53cfe1a758dee7d2a79",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x59fef071cf1eeff5303f28f4fe10b16471a2230766915d70b525d62871f6bc6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x480d06bb4222e222e39ab600b8aadf591db4c70bae30fe756b61564eec6c7e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7d2292c8660492e8a1ce3db5c80b743d60cdaac7f438b6feab02f8e2aade260",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5a593d928542a100c16f3dc5344734c9ef474609bd7099257675cef0392fab8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x761717d47600662a250116e2403b5115f4071de6e26e8dc231840eeb4484ec3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4b74b468c4ef808ddcc6e582393940111941abece8a285da201171dc50525c7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x14ef999212f88ca277747cc57dca607a1e7049232becedf47e98aca47c1d3fe",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x47b2a5ef58d331c30cfcd098ee011aaeae87781fd8ce2d7427c6b859229c523",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x59bd7fe1c9553495b493f875799d79fc86d0c26e794cce09c659c397c5c4778",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3ede75d46d49ceb580d53f8f0553a2e370138eb76ac5e734b39a55b958c847d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6e0bed1b41ee1cf8667c2924ebd460772a0cd97d68eaea63c6fa77bf73f9a9e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x739edb8cdd16692deaba7fb1bb03f55dd417891bacb39c7927969551f29cb37",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x247573f2f3fbd5386eac2d26851f9512cd57ad19773b8ca119d20852b9b6538",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2f6efb89f27d2c0a86ec1e6f231b225caf2af9be01aca173a15fa02b11fdf24",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x75a0f99a4dec1988f19db3f8b29eeef87836eb0c3d8493913b7502cfedcef28",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x278a7c68986adbe634d44c882a1242147e276fee7962d4c69ca4c8747b3e497",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x675532b80f5aaa605219de7fe8650e24fee1c3b0d36cdf4fb605f6215afacee",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5599e790bd325b322395d63d96cd0bd1494d4648e3d1991d54c23d24a714342",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x43545892bb5a364c0b9acd28e36371bede7fd05e59a9dcd875c44ff68275b2b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x38db61aa2a2b03053f5c51b155bc757b0634ce89baace113391369682fc1f74",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x71b2b6b03e8cc0365ac26c4dbf71e8d426167d79f8bd1af44738890c563062a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3a663fc27ec3ad56da89d407089bcec0971cebcb3edf0c393112501919643d7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1030d58878296e14b1c5bcafe7e817ebe4aa1039aa96b9d0dd7fc915b23f42a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xcb3335374cc2a2350fe53d2389f04952c4d634f489031742dfccca17be2e09",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6ae3ee97ea5dcfbb7c36cffd89665baf114fae391c0367be688db09861a8ca1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7b16c33c4a8ffcecbd83f382469e1d00a340ceab5e7d9c0bd4fd010b83f4310",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x10f236430f20aafda49d1c3e3759c510fdf0c0c19f89df6d5d71deac88b547b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x104b04e96151f5103118c4eb556cd79899148fd6656e73cb62f41b41d65e4d8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x253bf2869135f4bda4029cae2819b2f468ae88530f3ea771090b2727814c494",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x65d50aa3c1d84a3deee14057eec98656a1296cdcbe32250bfdaa50ffac4c5dc",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x76323f8567119897f10d58e1552c98f5a62f03a16d3737e20fc2b0a31a3a843",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xdaee1c7b34ecb34717b7313dc4a299dd1a161447e2e0249426a6fc33a72289",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2bfd1294f111a5a90842d19cffb97481aefbc09ab6c47d7dcf91ba228019c07",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4f63db02e10fbe428a5dda8d9093feef46cc19568a3c8ad2fce7e7519004095",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x761a240cd8aa2f135daf0760bfc2c9d5e896e93a45426571cdad9118722e2b0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x30a2e8ac9e6605fd722dffb4caca8c06dd4a8968a7bf41a5371cb1a07d11c00",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5ba89e0eb3830039d0f8a9ca00acef15db22374c965b01abc49dee46270a7d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7e8659c39d7a102a198f0e7c3814060926ec0410330dd1a13dfadeab4e74593",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5a00feeb391114d7b976654ab16ddf8360f05671b34d4a97da278c0aef34d76",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x33ff2d848bf237f536524da818598ae0f2516ebee526b77957448973eefacd3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4e0a5dd802deed7cb8d06527beb15dad32547bae77141c32473f4c8148912e3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x776459dfedbbdfcef7a31e0f60c6480fc0676b280fdb6290859fe586d6e6106",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x59d0d8ca9ecda81081dfcae7580ab3c08a72195438c1556000c0c1dbdc08174",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6eb66d366da57e4ae717307dfc3351579fe857c51aa82b95044473c9ed14377",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xa758a70ba6a0cbcbc65abfeca51359904f790752c3df55d42707253d8dea70",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2046e1b4fd4c108e8f832f5bcc4dd46abf0d19ef0237beaec29d6c12fb9832e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x472d99d1a6e1a6aef339eab1af3d53af7a8326e4d0a6bac73c3a159031c3686",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1b0fa36439192f135c239918bf47ad14b55ced699f4582d929a60dd227b34ff",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x728dd423dbf134972cbc7c934407424743843dd438e0f229afbcca6ce34d07d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4e42531395d8b35bf28ccc6fab19ea1f63c635e5a3683ac9147306c1640e887",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1ea9bd78c80641dbf20eddd35786028691180ddcf8df7c87552dee1525368ba",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x68a8c6f86a8c1ebaeb6aa72acef7fb5357b40700af043ce66d3dccee116510a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x25c5f348c260177cd57b483694290574a936a4d585ea7cf55d114a8005b17d0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x339b405bffb6dbb25bc0432e9c726b7f94e18cf1332ec7adfeb613345e935ab",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x23590dabe53e4ef12cba4a89b4741fcfaa232b7713d89df162031c8a627011e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x534a4f3cf71c93023e473f12e407558b6c24b712204fd59ddc18c7bcddd571e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2e1b2a3c32aebc0be30addd8929c01714783aaf01be8a1d35e830646e8a54f0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x605a244f646a825602891bf9ddffef80525010517b32625759b0bf5a7f2c386",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4f81a946bb92416d212e4d54f2be5fa8043be6fa482b417d772bfa90be4e273",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x655038ca08eba87484bc562e7fd50ce0584363278f9d716e31c650ee6989a2b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x44938959c2e944eb6e5c52fc4ee40b34df37905fa348fa109f6875c1aa18000",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x30b11c32e8aab0c5908651a8d445395de52d5ce6a1efe75f2ad5e2c8c854a30",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4a92733a733f225226a3d7f69297e7ff378b62c8a369e1bbf0accfd7fb0977e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1345876a6ab567477c15bf37cc95b4ec39ac287887b4407593203d76f853334",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x580550e76557c8ff3368e6578a0e3bed0bac53b88fefdde88f00d7089bc175d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7d7faca17be1da74cf132dda889a05fce6e710af72897a941625ea07caa8b01",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x62be425458d26cfedf8ec23961cdfd9f4abeb21f1debbe87bd51469013358fe",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xd77a8e8eed7ce4931a6d2a4774c21864e2c9f468d080af9aba6756433a1a8d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3e850e31c0345726c1ace38537dd88a50c85d6819ae98add1bbd62b618f7a1c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x375a5d9b11c83d06a04dc9f1908b8183adc6f04e5b2ceeaa23d3b68c973ee77",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x76640613af9ed1a125624e0c38252bee457ce87badb24fc4f961e55883d9077",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5428ff423f2bbabcb5f54aafa03d99a320b4b255115351f50b229eae5522178",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6dcfc3a99563a5ba4368ac4f11f43e830c5b620a7273330e841bedec0bfb5a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2652523cbbec2f84fae1a17397dac1965127650479e1d5ccfc6bfbfcbb67996",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xa737d6916aa6a869252d8ff294a55706e95e0844e6b047755704e37d978e09",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2833391a62030808228d14437d6f91b31c0038c14988a23742b45e16f9b84b5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x284f7815a7eabc1dcf56da511f7d739f1a199f8ffaf3474f645d2fc93327dc",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1e141c5429a369996563573bf61d7f713cb7d25baadff636ba2756c65a910ee",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x60bdb98c079bd5cef216803b056afce03f6ea41934275c965d6e196240fb953",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7f2abefac9e7f8109b0a2d25d0bd297059e45dd66798ac8b299f0a3e442dd2c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x41776c662b44a36c7075097c14b6010cb321591a4eca2866d58252eaf9471ac",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x573b13b32161c11c9b16eff7cf93fa770a3ef667547a27503e39092aeabf73e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x327319fcc0d34a0d64f5acab00244b43674a60bef754844fb2920c87c90cff0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x755f0e4c374e2fa4aa7eda10041e2139a4a7793eea44f415c73ad4fcba1758",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7b9cd3b277f00a75a17961d2d8e46e6a1838c8500c569cdcad08bd4e0cbae84",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x21f5ea8660d290f28b9300e02ed84e110d7338a74503b369ad144a11cf79f63",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5e4b0ecc6a6c15ed16c1c04e96538880785ff9b5bff350f37e83b6fed446f14",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3d8506e792fa9ac86ac9739d3d5bf63cfc13c456a99c8581adf590c8d9b72eb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1e3b6498f0daba2fd99c2ac65461c3fa519cb738b53cd6f002e97199fa4161c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x70930735d913d54915fba20c97f07cba8f33eb8f4f81fd869699a10e83264cd",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x16a36769ee50227c564bebce3d9cd7c4ca55702a7c7ccf403075f68f05a0c2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3aa748723229eb8b33354e0901f50ad052b6c1006916790c979133c4442be90",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5db8c52b6adb520496f9edd7105c92df67e8605ff4e0cc59992c3eb651ac7a4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4b2222d0aee638c7e5efd8ada791638ac155a01b78f3b532283574653998bb2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x26a4b2a61f40c1ad77737b99cb27d2f3118622be64f0120907e2589d2f25ebf",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5820792f23a13d58ddef0607950d422598bb1f21888dace88929fbe7d4828c4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3678de28b6896959edf5c9dc0caec59b02dfbbf54811f87939b32d0523f58bb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3cd13f84bb7ae6eeccc1012837d2f3e017f069e66cf047172bc70371f5aed38",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7af8995e2ceed8841e34d44365c7ca14f5980a6a5c67b9813fa7bfd74a9c1b1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6d7af6524127a117184a0c12a6ff30d28b14933a4e96bb3b738d2a36db72e84",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x73200d12e733294b5cbb8ffe7fb3977088135d0b0e335135f9076d04a653c58",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x229d7fc2a1bcfbe00d5773f8dadd70a2641d8578fa73e66263b3512d3e40491",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x29889daac66c404d6491ec3a435d810a2877d885df1a3a193697b79b4af39c4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x171f0638dedf0b69655fa9930bcbc91b257e299a6717bd8ea23ef550c8faff5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xded0f75cd0a6a5401a954d26880eaf12050ce6458d3254c9dd6354bf66278",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7fc7d854c9d0b3bfbf826c384b3521af0f29f975613e8ea6dc14f37d8beb54c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6d1c3edcf1de16a4e0ad7d8aa099a31fa2cfbf81f6d1a5798bd1ef93ff906af",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3444c0f008988c8f600270b365ff926f016e49a54ab35bac4f3b3a42a5879b1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2a48058c77edcd75dd4323d9bb9eccb854009b1184fd716a8202f8627bb5447",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x56cbe248ebbc2f57ca8b943b219ba245791592f687815293a4499ef598fa9b7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x658160ea7b654d786dc624b258c691f594e080610c2d41d6ebea0d8e3396849",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6fcc261ded0ba97b4defc7c9bcd32b5dac89e4c08cb55cef98c6b50f5a3a289",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7b74edd15d97b289da4040272cfc573f69a8c9a8b36d05e3e50b598508b7f9d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x19637a12aa8b822c4a3f3551ef6c538043371a12a962de1dc25d67e0a5ee561",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4c05a7abaaf08f21d93b2257d4f4a3ab2b44f4ac44ce0444418c864ca18470b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x657060a10db73c4a9b6aa6288dd6164e0b50a4e6efbc2ee599a0cf4fda33b81",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x688c61ee887c1497ffcef82163f1a81bf7778f2c314ffbd325627bf0b25dc5a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x54ab13ae1984dcc7d38c867a47f4a8cf786079ee07cc94ab5ec1962c21f638b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xccee381472bb7dcae008316038c87a44fd9295f730e389eff14e86442c41b8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x610bf9b7ea4557d72411ec90fb677f9a2ccb84c76f003954da4e7f439c9a84c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x51d6322f7d582892421e977464b49c4e6e64af2438da9a7f21a061c77712dc",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7d92a463e2aec09eb86f4647dc9ec241904135b5eb53ea272e809e58c0a271e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x17ab90241b58bd3bd90b8a5c7f30aa9e5afeedbe1c31f21ca86c46c497b573c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x199d80ad30b4b330fc8a063d1e87307993e1d98822a1729488ba8a586045691",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x601a139ed75acbecf557cd6513171385a119087585111c30bbc1b65cd6d30d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x77b10e23b08892ab18cc6b14dfda6f4be5c2fec94a12e3622622376edd0d6a8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2a17a5c34f9f598deb5bec334fde606eaa5601df908eb5825ecf70f9cecec3f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7e176a66dcfd58e240c4546cd760b7e5ad02e4f0265c6a2f38d710bbdf99d55",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x27e76848780aba5b12061bffefff1710995586618a2f32792d62771d31ed519",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x43f51dfe0f1cf290c9a522e2a5e734f79d220be80348438c676295c3d429e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xf1f93c3d919653f02fba06fcba1ab89497fff53eceff6a7d129887d5a9e3b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x79fd6f5f9b042ece36af6b10eae2eef9de9c9dd18752eb66868a0c301015dd9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1958435eb08883bd69b6a56a8f3103c22f8ae206a3d4deaf4a04118b4dd6a6c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x329230075f64ffbf631eb0c40b97d71b4dc38a08bd18b638f57e5644680068c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x219557f1604be8622e697e986c03d2a49e40cce558a264bf4f1ebe06493eceb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7238f034b8c57c8b59b0f744ababf9da8229152a051d4f3b3c4995233ac1111",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x201019c76d9aa29a00e6b18a4eeac7b1322b44285c57cf4c0b68a87120b1d31",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x69d95f3c7892a1cf65b45c324be2294c4c5459e05e0feaa0b8bb98cd8bc958f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x78aafbe80fa5ee9a846e991bf35b81567a6dcbb1b190e7ee47e53fc66422e84",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x15ba3c5a882d4dfe3e23db18368ade6b2d10ef52e34f12ce0d62e7183c10f7e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1a4bdaf2bff969eff8cef73e762b6346492b8d0f17b2e42956c526f625241ea",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4adaabee9ab3c6ee7fc67a2ddc09c5185755dcc76cc3b814a6b71aa7ae542ea",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2f47cde744314dc0502faffb0387a2e765e4354b0516ee9ab0b97a1b6c33ec2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x90b2b18b3fc2919a55b71ad6d6fa67dda752bd02c985b59e6554f557fe4a2e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6eba866251e1dca38a21c8b3fad0aa3c22a45dd89884c4c68bd7ef67de64f52",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xb8dd33ef8726747fb368aedf80c2f4a720bc1b5220f4a3f0e56e2fafb7e243",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1fac2f441d05a3b483675200cb1ebc6f4ca6ecc5ae60118fe8745f95217bf8b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6d28879c6f75c4ede18e1b94ffff964d08c79038fd9ba2e7873cbefb5f323db",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x28b38e0334fc06af4c94ec4f9434923d4149cc51817526597423fd4692c59ad",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x84add7269e2e41ea57aaed996f4c012ba7003ea2b994670cc0d554b7a8bd2a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x64d672ca00300ddd5e9c9d2db433d7623bb54c8eb2db51b235a07616f1517e5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7f71cb5526600d15d3413ec971ee3b133718224b3cbdc68171a53d7c8684382",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x38e5702bb10256e1856a5bfb03a06b231b89a36e2f84af80bcd2d027153d847",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1a8d4b2044b8e03b325c353f3f92283013920b92f479064b6e93159d2ed3ba0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x68384718bd3bb23f32999f1edcb2dbddd8136259e676c4492d0cafe80ffd856",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x31a77aa370bb597dbdd0422612a7dd947aae09a5b0b17d1996f13a85103d150",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x40a9cea0394d15ef057c2923d4185f290fe2347e00529d92f927ef506e3b5e7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2a427d70a34b6b5237894f065ef5d60a9872ba444d47d98648b080b8ddb2a68",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xe505592d606917f898c54a7afc45b328be3cd48121aee2e8f05185a3e23e5f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x45b4e74f19b293bc3d3d172a101e344558fcf4ccfe5eecefe31f45a45614df7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x68486394265c9dc8fae42c8fd39605d3179c981cb44cbe33740a3deb907bc59",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2868a08eae382c069047152ee964ac5ebd242b44267e97e578802440ef764f5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3159144c85f2c515eb806e5aedd908553057b69c556d226adc6e4511a35423c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4387edee6899d4a85883d2f8524978a4634ff82779f150b7b0c861bb315ed3f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x68c5830832f6270a189b074d7675fcbc1d1c5cc06ce9c478bf8f4d5ac1bf40",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x399c00b8ebb398248bb1f52528d5241e7366b73c2d89f57a11dc82c530cc57c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3238aeb8f6bea8bcaaa1bdd5b4f917ccfad8eab031785ccdc648b47d7ea4be8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x357bf5d87c973292381fa4320114551a837a1d6cb6e2bb0eeba534fb2e01742",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x360274f27df6eeec0b7b65fbb227a8214ac3e55cb37b1970e18489ef5b574e1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1cb6e2fba23730f5bf9d8e726569b6e8bf6b5ffe8520339503c5469cc3713a2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3924324af1994280f87f289fdae0b9a2d8cb9914ec37d319c18daf029211815",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3c4ad04a5a057e4411487858dbe16af8e3fc065ef7400749ffdc248bdb25bc5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x50c92b3e6848a21001be2a268615e1e26cb4918ecb09640efaaf1d8b71568fb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x47d21828025d0cbab84084965a49dd14c7833aac562b55de808a94777df2ea3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6207c6a2fd70c19a10430566c9efaad95eab8cbddf308f0057c81f3155a25a0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2d4acebd804035257147ad8d8419a5f5762b4b543c4846ef9acf41856e672ee",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x78f49c214872b5cce18ead0207a165fb741ea818a69cfe9647737323f70f4f5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x22aa8c5c5ff26f9a0edc768ae32ff4f71a71205b4e83cfa0cc687a1e02566ba",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5dd2e0680c7eff25211f31d3c30a9f454500d6eb09d46d87a75a42b190203cb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5ac4bcdb9c14634ab83c13a30822ddbabc54248cf1177b11cc2aed24d2d32f5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x77dee5f03389585fad0d1f2a8accfa4cb985344891b8befaee42f3462cb48a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x737dba18eb055a12d842bfae32fd146dcd2d7bb932a2591aa864458d6d652",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x23bf372b0b59abf250463697ef4b2096eb1c9674613918b4d0c79aa10d9fd59",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x73724274fdd351c378e597da1615dc51058e14994464cb7b318766199ac2a35",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2e14e83be58cde3ed5f3fec8ba6462493a4a2f0f7d6c846006220eccd49ef25",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4846d310812d81ffda3731e8289005e2f0e05411e76b1c84332c3ee9e831afb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x160abeb38bc4f22af5fe618c19c77c39903007900722bdbdeaee059f31544c8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x264a535ae10091157ed59b04955dff66897af74cae20456bb830336b803ae47",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x316ce6b23e720b8302e2d4bd968c0f140f69930e46a54784a7cee7e0b8a0c8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x775d95a0beb287c98663a3f9a9c577ffc67c1fe6fbe2db5b08829a2c3eac922",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2353c4a418bdc1e461be162140cc69c26eb9d99f08924991f85058f87f6df41",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x181ef9cde124459dc0e2aaf93512abd49a10328fb93dfc4d49ab671db64bbc4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7ff76956e0cd2b490b47a0a0497df5f874cf47f54c45f08101256429b48460",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x318e5a52d685eaa06e0f39159a344b3d97b52688b671d133954aeff0bc17707",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7616cfc6834643d4b95ed1cfec036f816a7c3d3b9800f301f98ddf341712ebf",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x59869515fb57ea7733567e5d849bcaa00c00e0f86f4ebbd2c7a6f4c0c77692b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xb806f4e19770279fab5427b8eaf5bc68bf984d6ccea1e878a7aaf32c9975d9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4fb0c93fe30da048576fe5e839483636218dfdda3d05f1d68847a4c0167597f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x685af2d7bbf30cd0c5c3d41c430a8657eeafeeb4596165faaa73d802087ad80",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6f617dce150ea148cb8c7488fe4caa920b2000bc8122cce1891e4b76cddc9d4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x47f02fc512b153462379f4f793c7cab9e659bfdb07d3439d29039f566b7236d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4ce0a14a5a9c30a38062eb8870eeb4ff3562db743c0f3eede2e3d3862a2eb7c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7b077d27c7007656025224fa4e528b4c4261f43c3da1e42bd1349403af55cbb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x50f5f6adbf0b9abc6e231b855018f4ec806a4f199cc511bed5c423ebef298e4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5fcfeb78685abb1ce610e516ab7e2aa210fd90844c8d1c89cd798f3d71bbcb3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4255a568f4597862e1dfe0c391b97059d179d7eb4d868f61364835e5028f9dd",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x206d7f23d0fe1b1c0967486ebb792d7fdf5b1691d2c2f9306e211d3b849526b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xc61c74cc988663ee09f4c725d5b1f04549bd342d3550ce17427ac75592b637",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x175a904681c7a91856bf7fcf8410d2c19eb8705267914489664a1ea2af5b8fe",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1bd842a4ec97e1489ceb542bd3161e5a00ce431547bfadfbced954d993b0a11",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x14899e0f97aac917d46ce5e9ddf11194fb846d2c52726af4085f27c570a98a9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x842955243a56778a332ba9be0b22b2af62efaa50068d3078675fb76c225e76",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6dff267c3bbce68474294da908df4f5cf2a4160c638f7cb45c098057e968f44",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x72c0dd24a576b47a84cdd1a20227773b5621f85b781c288625e3368e1cf738a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x728771890334d0c9b0f400543bdc13ea6890497bc87c509a04f8014916c13a5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x30632b3865a272a1a00270430744ee90b40ff16e1fc44515876ce8e36215ca0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x76d656560dac569683063278ea2dee47d935501c2195ff53b741efe81509892",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1dbdc2ea2e555309578eeb2352fbc47c8fd5ed77cc09903b577700f9a4d1be1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x119bcf6402ad9953851bac8e318d50af699b0cc75e2597aff0a2cc521975aa4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7c3234094dff9a45064a5b9abd0667c04dd76c62722984f7f8475e7cc344c06",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1495d40cf3f13c5fc90653c2b2f02e0b833790c07576286d3127f745ea920ae",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x40f019a18b8097235264cb8efee7d149321a199ccd32ffac43b5a778dfadda1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4e23809ce49747990e43b2d976083dc84d67e75cf22e5a76ad5b7a2dca50b3d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7f0a3bec1d34f2fd632993a3d9c6432401cec25ad9d6196b909f3672980bd05",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x9460aa25f77fc10cfcc4579e2011e39ce477a32a768aa553201e556ed2bbe1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x611384709c407d85c93256b6aff04c4ac515450c70cf507994165abfe2347b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2065bc7a4aa38d5fe86f9b593ccd060f8d4a5a19a9ca8b182c32199a4bd27be",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x9969a08d753e885857a5696d1cafd39f62bb193acc99089df76c240acd2fc0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6df73a948c95439f3230282814ba7e26203cfdc725901e4971ad9cff4db4396",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1cdf0446663046f35c26d51e45a5233a93c51f4f7f1985dfe130dd67addefa3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x59cbe680183d1dc3161ee7f945f38ab9461a5293748b2b7be84899e62c9860b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5030fda0c29a929e6cd634b9f3d1bf975c363012cfb439cae13495f8ce10225",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x49aadcf98ef59c0e5d2097845949988862b96194abc8c5453f056f232482892",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x319c68159cdf104c2543486ff784860f302187d77effb9a5fefe4e16f0ddc2c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x575531b404cdba72a63dbbd17aef7d9ae00f73eca7c6dcdaf5e0778c921be41",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x35ca7fa56aa38486833a976804899ba3c97fdaa0a23056cd2dc9bfdbcdd2e31",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x47dc0e209ee8d0b67f63d9e63837ff2ab462c4839bc14a1a3e802327ff0e31f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x744bdf0c2894072564f6eca2d26efc03ef001bc6e78b34bf6be3a1a91fd90fc",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x73c57ecea0c64a9bc087e50a97a28df974b294c52a0ef5854f53f69ef6773af",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7dd14b0299ff6064a96fe97e086df3f64a4c7e8b4a58a5bd5fe1b9cf7c61e7c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x562f636b49796e469dfe9e6748c4468f340e8f69e3f79cfe6925a261198dbb3",
        ));
    res
}

// Evaluates a periodic column at a &point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at &point**(2**k)
// instead.
pub fn eval_keccak_round_key0<F: SimpleField>(point: F) -> F {
    let mut res = F::zero();
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x25257ecfcf301b18da64254e59b151a549668b8c150b39f71eb3e3508224fbb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x13342c1639906e1ef17847fdf16d032ea663fdc4fed28da92f88381ebfa626b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x12125a111c43e7a0819677f13fcfb3d15bf9921cb098a11db3cac22813437ce",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x15c58abecd948a965601dc451075b0c1dcbfe1ad91c086e0f43865ebfeca111",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x73960e722af010fc2cbc3012de9ae4bc357e8728e7d6a15b0cb10b12bbebeeb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5cb77e4fe2751fb9cf145aea5d07503c518440d7bc7b85c91b6799f14fa0b82",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x624263f4821e9fd3af946554a890b0e20ee21c5db0eb4742f547f049d7265a5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x601b3d8d8eeed4743cab5f121b1c30874e95ad5883c331aa0fef0b8d9c4b17a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x50e3e5f09b36a9055c9992f6b89471bd122b785b6071483279bf6c45263f18b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6687d6c2e62115cff5db20d04e5fbcdf28e85d6acfab9e7b8869f46f388b16d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3fb1f146e0af9750e5d66c46244aa96306b4c4518e6311ccc4a730f289d4493",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4ea9551cbcfe5f18614ae86e485adba9f70255f312eb4386020432c1169a03b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1cae942c39310c3913c33d6e0c10c4db313621d4e70baf4ab17a4b71a0a2336",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x186c97cc34ee93808f32336d2afe9ca74ce30b932b416ed662b0287c7db52eb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x754e8d183db530c59c19c447e1b0fa66cac465db9ed0bec3576fe8723a88d5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x70000000000000ee00000000000000000000000000000000222222222222223",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5bbf43b8bcde1f233ee3b725e768ddbef1d6ee4e272c2cc365cf9d58b2cec06",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x322981f5cfa5a7aac59428b3062bad8dc08145c68082d791cce8fdf2287fbd2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3318691be977dcb75f1fac814c8d42814e8fae2640bb483f5ce5fab2fa9f063",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3a70f05c50491d642354e1df25c2b04cc06b790375c5dc7309838f0d17a9fc0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5d870d9e97b879e273beec4b5ea076338bebca91d4104ff4a5fa8a51cab1348",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3fb0c6c8b229297ff6644fdf9bb36a659c4a91dd060d65f243d23308fd2765a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x749e7d5e4eee9f1220ed1c8991d70dae19b3f742417b23c5acec221e971ff46",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2fe4c27271112cbdc354a0ede4e3cf78b16a52a77c3cce55cdeed2504192c65",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3a3cd9702057cb220459e07f39c2db92ef07599d1016c02a8f028c6a13572ab",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x53ad84fad4ed87e5606af2364d96d04735dba9a7a1f235a13b68a7496e22a73",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2b91e359964f3f5cc62de8673b400cd968e983184fccf3d2f71da3d4a0f442b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x21202fc82523fa85255e596d816cc3476bd24f5be58e592577b74fbd4a6966d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x462ecdd9bc89bf4cd186564983426380d0ef409db04df04e971d2e59f2bf32d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xb981951722e737e9e029e1148b76ad3ffa4d71a21426c17065ac1482f379a8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x315b9e0e2e5cd6c0490268bd5b95853a50961e5584289a0ee467e5f6bdee9dd",
        ));
    res = res * &point + F::from_stark_felt(Felt::from_hex_unchecked("0x1111111111111111"));
    res
}

// Evaluates a periodic column at a &point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at &point**(2**k)
// instead.
pub fn eval_keccak_round_key1<F: SimpleField>(point: F) -> F {
    let mut res = F::zero();
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x46c20a61e3dd5f8e8d4085bfcf8e52c9c50152285742b9ba783a4edb6608353",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x62db2649e22e5c49b68354e9a3801a9b695f7d1c8d03854b1edfa0e322ef902",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3420050a4dc6c773aa43eb3a5c8b2b88b3e33bc739775bc375933e2ff8e3846",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4bc9ed4ab2622c0cad2c8111a9a5598ddac8d9e1a89766c0bf0bfd7b750538c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2df8e6211399d12f3d33874172e235e92aae15d062516b5c1641ce309354f2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xb63ac706fb9d207b59fa7456f07c71fb193932810c68fd454a98d89d96e97a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x523ed890c4e0e53b280afa7ecfc9d32182829c79f4f90ee5352b2b1671933a8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x401b3d8d8eeed4303cab5f121b1c30874e95ad5883c331aa54334fd1e08f5be",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x44670acb048727d509f4189ff2322439252d747f2a78570bb8c20b28f8e8c1b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5c6bd13190fe93c3c7535bade15bf685833e3f846aafbe531aedf5e448bf4f6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4e9d1c063aa65b8d301ab440c23f7489eddf4d4d1d049fd6f817fbea9b9b843",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x73531922feb3a6f79490f130de30f0c2fadd1afe367970c43540a32112a5e8c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x40dfd1db116746967a4792685c8eb61ea90a2de3f9aa80975d4c6dc52e7025b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1a3b463bdc8408051f1aac5b7c31cecefcd32c6a6dc13b9b9cfee72fb3dad69",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xadc55da564cf8859d120b02ab9fb09ed6ae934330c42084a392a9688723630",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x70000000000000ee00000000000000000000000000000000222222222222223",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4481ca3e932177631ec3ceed8ffc6372aa9cf54186ed4be4ff4641a7807c127",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x592a0b7fa0c98ab8930a5ca88ac1cbac0dfa16ae244d58745171ce70fda046",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6cc4bd7e3e47730c6f03f139e48f03db875a3e1302c00af6caabfe34f7cbf37",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2392a163f404deefe6cf4481b3b183464db5160b40d56f427af356cc7ad1d37",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x740152f0bfd3a106dcae496a23399b3195eb3242ec4154529c8bb14120ccbb4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2a6899691636dd2f8e27428b7478962bcf7938103e12c462ebb4b1a0bed9d28",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x309e897530d9e68ebd3e9938bf89553a7fa2c5f8a26090b3cf280a70dd89e77",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xfe4c27271112c79c354a0ede4e3cf78b16a52a77c3cce561233169485d70a9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x20c3b86201349cc6d6c20bd29a2ad219855bcbc3c6db925113aa6d6dd0b57d2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3b935e02ce81c807eba625e65ee894418cd957563514ad82c0d77a87f5a37a9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5c83a35a80e818de30d8bf353017d8c6135384ab53836885cc65b88059b67af",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1d50582e5ae5502bd773493bc4783268dca4f514e019b93890c00896fd830b3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x17d0b504a0d0e37a287c72997d21dec2143836cf446b2563784e32746ee2c13",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6f8b7db461cffbbaaa70ed8b33dd11c847c952fb3458a9845a5e22e6ba27d89",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6640c6366c5b918d0369115b919baa50eabbbe778b22d0cacad4a7b7bb357be",
        ));
    res = res * &point + F::from_stark_felt(Felt::from_hex_unchecked("0x1111111111111111"));
    res
}

// Evaluates a periodic column at a &point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at &point**(2**k)
// instead.
pub fn eval_keccak_round_key3<F: SimpleField>(point: F) -> F {
    let mut res = F::zero();
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4a025b231a318123295e4fc2ad69f9552eec69c64066fac264b1cfcdd7d3158",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5460b40a6b82eea400c5f65223403b791182933281ab2654f3762718ea7272a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x577c87daa8b61681534eabd0626b9e6b73e5152876c214cd16515b620f42de",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x42cae55f34098448653815d656a3fe1e71f0b843a3a978717f2f02f69ec80c7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x54dbe71eb9ae525a52abca43b7aadeeac9664a351d711d8d43ed87957535f19",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x57276cf88395452ddfb9ecf810e6b6407e3f757e0186097c3484de9195304d7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1ea216e7fa1188997bf294ce196ff23a97bed74a3a74820555d2aee1819c839",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x601b3d8d8eeed4743cab5f121b1c30874e95ad5883c331aa0fef0b8d9c4b17a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x75d25626915653ec90adc5178d4a32f65ed1857d70c0a737b9fb5535837c9d0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x559b4e49955b69058393ead11679baa4ceebadafcd17d1214cd1fb92c9a67de",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x844af79594ffe6ea50d5c9e6ee3287cd7871390cca63a0797c3f2d2d3f53f3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xc52210e7d0c4dcddc855c6c31324c3263b53c9c3b675f1352fb7b83c6c0f2e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4fdd57886595821090659b2a53a9dc377a2a90315c901d58677c17a0a4cfb60",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x77b6d8762a4c0870d48f1bf7d4ef0674e1b51ca1a5f23186babfc02837bbac0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x71b4d4ca13e1018d2b54993a3950f29b7ffbd7f317e8186338bf32e552c2ae2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1000000000000021ffffffffffffffffffffffffffffffffdddddddddddddde",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6bd6af7759bad5fe5748f2ffe9d4cb5319aef5a5e59df200aa3df034cc06941",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xdd3d6e7af0c08df08799e48b8fadc745d590aba8fb7199d89ffc2e9d87bf31",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1b0a352b349c7cf1a5cb6b4faec5eaca91a91b99e1f633c145c994cd3cf8a58",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6cc8246a903b2e8ca81a6de13ceb3fc453b8925a4d49c0e5daae686c89a52ef",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x24bb40b69d54b9ce773c98f82908afd7181b1187d5051669025874cb5401c72",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x66a4081561dbc5b317067e6d12de31d212e4ec94ed17b69109c0f327639d08c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x59926196b781e5f56e60c792946b8853ef6ce8c17b5f3da6fcc8ede489bafb3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2fe4c27271112cbdc354a0ede4e3cf78b16a52a77c3cce55cdeed2504192c65",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x705bc2bf83594f7602f344e78e5f6f8774d43933d67103708446fabf41e59cb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x683026c45015a1db732c80940d4b2d6dc238b4632185eeebf173d6262f26d85",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x341a62cac79ab855999592a767fa1e3270800864554795a6a6405796a23b15b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x641ad527beaf01c116281fdc3b3e75ead6a178c5d3a567950ee2d4d4cc8d8da",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x59aa70b54354dfd8ac0ebc473fd8a9a613658b3025a989f4fd0af711bddb694",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6a7db27bf042f02234b078a3074c11788d26814b6b700e6bc2b629da8b32b9c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7a0f8f36b1ef9bbfd60fbda365eb2bafdc9749e3c50d90852773205d38a9f04",
        ));
    res = res * &point + F::from_stark_felt(Felt::from_hex_unchecked("0x1111111111111111"));
    res
}

// Evaluates a periodic column at a &point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at &point**(2**k)
// instead.
pub fn eval_keccak_round_key7<F: SimpleField>(point: F) -> F {
    let mut res = F::zero();
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x279b1137fbe58a227ed345e6b3136ea36aa5eabb7ebaac5f226c318ef599e60",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x63355b4be0dccf93a1fa0f44f29a9ddd83c010b942d979a530cecd6c74ba8c8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2a1210c6928cd6a942ae59da3af2423896f30acfc1174f1611cd90922303f40",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2950c0942b252300fdf68871a455ed05549929a79037834d6faa1d2dab1e731",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5b2ede5b73c60003b027071c1f36a59ff339069eccddc28f143ad374c13c67a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2bfbed668244b9211d0023eaf227f0a8a9a8aea53ba93b84d35be4acaf5e0b7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6d8c656cba303fd802db54d6b43bfcec1e4db34d0ee27d5bdd23686e5f50760",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xf9309c9c444b1810d5283b7938f3de2c5a94a9df0f339579e21afa76cb17f9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5b5d5cc26ebb21f4a1851a39f0e5f5df99c9304ceacdb7f10d81344a3d781dd",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x21cf87a7632c50d1d9c0477a319cf769e77bb36db2f4848848c7caea274d289",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x734b0fdf3a990e5ccec5ff6f72d41000033dfbd57586901e7ce5a9ce071992",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x76af3f6bd4dade530209778e5baa12faab66d6586fc87cb24c119e8e109d48c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x48639d021d476bb451319356bf88a82456f69947e4f11c446d5e847136e5ad8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xeff2fa639b2281167458555e9a07a0feb1b8d33ce88c64d2a84fa742c11b71",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x493fd6f738b14dbb2866f8dfb966aa9963463ab1edfae1b39749c44b8713519",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x60000000000000cc00000000000000000000000000000000444444444444445",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1e9cf906f9fbdf8e75a43cdad43c3f7e2aa6645eedb45f02d6c87332ca5b8a4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xeff2fa639b2281167458555e9a07a0feb1b8d33ce88c64d2a84fa742c11b71",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7df59f05447f800e840bac0aeba642399305955df706f73e5f11e645104258d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x571c35a210962caff4b6f3d6c81ad517e5bd8bba7ed5435ad0121108c60deb5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x16e29756eb5894d70e07ac474fd109610d3721dcd6dc8020d85a5c9832a1d7b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x21cf87a7632c50d1d9c0477a319cf769e77bb36db2f4848848c7caea274d289",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7a49136d756e1c2773a7c78b098e94291c2d1d10608723b18b334f988feb421",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x506cf6363bbb4f4af2ad7c486c70c21d3a56b5620f0cc6a8a622949cd792c4c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5e6a98fe9b63767a6a03630487ca5bfed0ea8098a8c33cacf94a26f40292721",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2bfbed668244b9211d0023eaf227f0a8a9a8aea53ba93b84d35be4acaf5e0b7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x50c39f36354a1a826c599a23e23a3a8dd5d38014f08950a9a7522e8bec481a3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x48e3ca5def69d4a40b490c2937e52ae81a427445812abca4eba9aab2f5add08",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x458aed4b839a0190f09fb945d16fa8daa8993e3c7754a10ba60c4b81d53c135",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x63355b4be0dccf93a1fa0f44f29a9ddd83c010b942d979a530cecd6c74ba8c8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4eeab02e97b059756115eabe88cec451623ef4f0a29b7d3f005f83ad89b0f69",
        ));
    res = res * &point + F::from_stark_felt(Felt::from_hex_unchecked("0x1111111111111111"));
    res
}

// Evaluates a periodic column at a &point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at &point**(2**k)
// instead.
pub fn eval_keccak_round_key15<F: SimpleField>(point: F) -> F {
    let mut res = F::zero();
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x24ec7e1a7b8bcb5b138102f4131d7fa12dd5fc5278251892aa929a46b50dec4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6fab9476126a16ec91fb0071a673f7c8b65b1dbd3d04502cbc915a3db9e8a1f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x12f9d780e40408e85cfd757fc76df0883a6aeb39adb7626b2ff9b582c551d3e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x76e0a1037b5bac3527a824fe3e4eb1f1841619c97aad3b04461cd97010cd690",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x51d69b95d84bf6cb03ae28cd575fd8b1c2fde0263a9cbb33b0c2d281d6593e5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x637549dd8a31e6eb931f74c374ae94c1344727397e1baa82702b01ca6765a5f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x259a0178365605d693f8d60441444981aa3782845295b3c795504192c100bd5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x501b3d8d8eeed4523cab5f121b1c30874e95ad5883c331aa32112dafbe6d39c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4cf3d65c0b542463ffbade568e57a5562c15810c7050d7fa1cffd718c535167",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xe735942acb168306ebc477871555c572af3741c59332ca97f034aaa040eef3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x231af732a4a2d73dd564d92560c1b4387d641e999f139c35a49dce4b95c40a8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6bc84bb37162e162131f843a2a2e6f1bfaf80796e8203e6c98b9007613eceea",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x34701ae77a897e00aecf2472198c4bbb5d7e5568c3b4439be3a5872607c4674",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2b785b28d1c810d0ae9d9ff6be5830d87f9eab4199991d2982faacbfee350f1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x9e00b3dfda8e20537eb42e591413bf4f6b41ff1af4904634a41461a23c0033",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x70000000000000ee00000000000000000000000000000000222222222222223",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3a3f52e03867160939f19be537d98bcc8972a660111194aa12ac42a7ec61d96",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x388bb770abf3374034623c1e4f7fde7ed6b8a6192abda75187874c71404ac37",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2fc8f5dccdd25a5950fbc1514105c89273300b1a94541a69944e88356b5a664",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x78e8e3e166c6ae785f011cdd8b78ecffdebe8b857dcc61a7339ea90e5036019",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5b858dbd345e2432e03866bcde0f9bc4e720c6c79f303f462219f77397d4815",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x728d7717199c70f029fe3331a50a2998e9f0feafd8440d09345598bfadbd37b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2d5b04f1e90c33a30180d67ba63c239c6b5eee519be4e624aa37fbd973a73b2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1fe4c27271112c9bc354a0ede4e3cf78b16a52a77c3cce55f010f47263b4e87",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x544d4edf7c744af8a57fff189322115956f891a315854171657d7a2f0a87826",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x49555ad694f14bc2cae67bf798b6cd6147f8c80d3f0adbd83ce40ea701bdab9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x59b545396dcb787789f473c12a59d08f9aaa35b00fd4204cced53d3d3fdad4a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x646e2f67ac7ac7a8663739ea0c09f1f2a233531a1f6624e76502f48302871e8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1ea0b1fbb487b7c25ff7c84c1d7501eb32b9b90b718b88928939dd1afb3a379",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7e84e3e28a699a839444b81427ef10cd62292ed510072b4ad884b8b5fca8138",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x62bdf821a73997783fed94521acd94cfb95eb9d6532f9b07adf1c5baade31da",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x60000000000000cc00000000000000000000000000000001555555555555556",
        ));
    res
}

// Evaluates a periodic column at a &point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at &point**(2**k)
// instead.
pub fn eval_keccak_round_key31<F: SimpleField>(point: F) -> F {
    let mut res = F::zero();
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5852cb7312b6299275e11e7bf04e6a604038b1398d93ed46cdfa41cb9503116",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3645c0d194b9983bc9c138a6e57651551a16c071ad1f5b0fc37946aa5b6a2cf",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x40c0d849a4523e6379cddf0bb9f7220b4ad778b81a7e873876700bc9d276ec4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x211f5efc84a4550dd857db01c1b14e0e7be9e6368552c4fb86aff35cbbff63e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x59c8aeb2a0ac85c5c13da60be839d4171a4a2c8808ecdf75ba6490bb81a9079",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x22aeb7c2e8c9fc5f1819601d752141f60e5e894d854be8a2bc8cd9791bb115e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x691f1c1adfc25c7591030cc775b01c76d305e385ef6528a6b2f29d656d8d181",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x851b8a8accc7b09b6021d3651549195ebc108098b4994fe630055dc081479f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1a8f201c55f7facb0cc93566ad5194662a91b7372cf67f0fd9c6bc400b89de1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3a3014183373f80fc4185542958defd2e934de274c76e6e4faf6922f7c26ca",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5471f4be94ccf3bb7be211193c1827b4070e4f2eea1ed7e828d4d4ca4a703f8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2c37b44c8e9d1fe0ece07bc5d5d190e40507f86917dfc1933413cc56b8dfde4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3ef97a843574486d4a4ca50dfea986f767c7fdd2a8ba4c9217eae4746ef650",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x507518e91a5aa1f164583e8bc6dfa7713e2bccb306b992611ac085e76070526",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5646153d03a487c880a5e3b7005a996b45570b60a5be6c4a0a8f545fa9900f0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x800000000000010ffffffffffffffffffffffffffffffffeeeeeeeeeeeeeef",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x32a29bfc37f6ff30148ef2f00017fecc0b8c87f6f22227f97ee3df56d231b3d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5d887530f485c860ea1cdab3580755179545c78a97de1c891f4d2598b28606c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x265b35f9701f2ec4b9f0c2ebc4aa23c969d376930985a63729470320ba328df",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1f171c1e993952caa0fee322748713002141747a82339e58992e23be7c96cb5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x198a2a2108558f4d2a79ee44f9de100d3c12f49fb5d3614a63091db177c83fd",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x67bd1f15f0224840b783710d5d0dac3eed90d875f3d84ece0501b738a170b52",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4b95badd4e47e63f49ff64ccced083d700959b8009f8dceda07f2b04cc47ed7",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x77ae47575333860649fde2c9aeab6e6a143ef7f674b66b019cffaa23f7eb862",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x167f58c665f83f3d48f50c40164294afe64889203f9bb2731e2b7cce7890efe",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x88ec8bbf38961364fe0675199297a9622102a21463b19f88945e655b6091b8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x86e1cac50243f817030f9dc914c0034e7a747fe0594b3e4f2a3c21a3d96b36",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3391d0985385399a99c8c615f3f60e0d5dccace5e099db1867c9d849ca45ae6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xd939b9e847fe38544d046dc62e475644b3450374885610484442e103f3105d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x451f103e0cb91bd2cc0af04966f16a59c5e4d1898022362ddf6ca5229e299e8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x702f07b05d77fca39f2b0536963dd44eceff845d2f124793dece58719d09399",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x180000000000003300000000000000000000000000000000dddddddddddddde",
        ));
    res
}

// Evaluates a periodic column at a &point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at &point**(2**k)
// instead.
pub fn eval_keccak_round_key63<F: SimpleField>(point: F) -> F {
    let mut res = F::zero();
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x13c65642d180be37bf6db1ca47e2884a87a9c17aaa35744910c1b0fccc4c50f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4dff8ed2557436c17e52a86eb24e65fcef37d2d5d0df503f27351b9fb114deb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5f4717961bc9bb0c1996dabb3c74988f05f5107081b80ff8c4c4299d22048f0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x581b3d8d8eeed4633cab5f121b1c30874e95ad5883c331aa21001c9ead5c28b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x216dc3dbb9063124640a3d5f603ca702781ff5264ef23cc5dfcec751f4a33d1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3ad1ff12fccb50a5de8e7672380cda75ed5bf08fe3d3cbecd1195b32d881025",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1d5c1c42d8ec54a42b1632618314889f63c11a24b80b67b9f587da4d671640f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x38367b1b1ddda7c77956be243638610e9d2b5ab10786635430ef282c49a7404",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6cc4bb82d7803f17da904038c2212dcef110ebbaa6234e67c5718db62a14207",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x69204e331102a9fadf461be4a4a5b34cf06aa4ce4167589c0dd4f22cc888ffe",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x45d4b9d9009554d081839e925dfddc46033bb5347b940cb21555dd388c66a56",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x27e4c27271112cacc354a0ede4e3cf78b16a52a77c3cce55deffe36152a3d76",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1ad6ca03d6d5422e58017c5e3cc08f74ddc9ad8bcca0774dd78b592dc16b98b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x722178334a9327c73bca71705e5c459530198bdca5be25f1d95c98d7190bf96",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6b62a10a0b8411be1c13352de351dbe55e625cf2569f2c61613d6c57d7039cc",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x800000000000010ffffffffffffffffffffffffffffffffeeeeeeeeeeeeeef",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x477aaaf598b4391c108cc2b3cf8fd238e20c8224820c59847541d59d2b5c101",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5bfdb03906bd7492c48faf9c33f8dba8f2900740d8c0f835344a49d639c843e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x653211d3237934d10c7bbcffcba0673cf8919f3d5695d32c1d40749e82bfc3f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x581b3d8d8eeed4633cab5f121b1c30874e95ad5883c331aa21001c9ead5c28b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1e2c6706b58c375bedd4d71b850b928686714f3c74987aced96858048d7f05f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1cf6b50644d7634d5b144cfdd1ff4f4285904b99b46a3c94eacdfe1e2d4b987",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x715d1152348922f2f7f84c0dbd3622bf0537cda35bb2dc2dc23bb5c6a103680",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x57c984e4e222596a86a941dbc9c79ef162d4a54ef8799cabaceeb5b194369db",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3bf31fc435aed4a4412cfe877384108788f7b288c0644c5f2314977430c3576",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6ce272c192cbade0ddd78c1075130b0d2dcd811b14f85eef96aba85d4c99ddc",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x38d10cd0c01529a85ec684600e23388d6d4f123c20cdeb6c6f2e4b5ab6739b6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x27e4c27271112cacc354a0ede4e3cf78b16a52a77c3cce55deffe36152a3d76",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x22701b06baaaeab34fc2b47969c12262b49396f2fb24f5d9802c38244c4b085",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3615d3b373ca26658a92cb1f979790b25cfa37f9c203d18c6abc0dd7e1276c0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x41eb54e16fa27062d52699248f4bdfe254e5d96302d92721efeccf4745da594",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x68000000000000dd00000000000000000000000000000001444444444444445",
        ));
    res
}

// Evaluates a periodic column at a &point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at &point**(2**k)
// instead.
pub fn eval_poseidon_poseidon_full_round_key0<F: SimpleField>(point: F) -> F {
    let mut res = F::zero();
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7ed6ec4a18e23340489e4e36db8f4fcebf6b6ebd56185c29397344c5deea4c8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5f3e9a55edfd3f6abac770ff5606fca5aaf7074bedae94ade74395453235e8e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x11eaccb2939fb9e21a2a44d6f1e0608aac4248f817bc9458cce8a56077a22b1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1b8c9c9cfe3c81279569f1130da6064cbf12c4b828d7e0cf60735514cf96c22",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7865d89fa1e9dce49da0ac14d7437366bd450fb823a4fd3d2d8b1726f924c8f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x712a2cab5d2a48c76a95de8f29a898d655cc216172a400ca054d6eb9950d698",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2574ea7cc37bd716e0ec143a2420103589ba7b2af9d6b07569af3b108450a90",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x47da67f078d657e777a79423be81a5d41f445f9455b207ec9768858cfd134f1",
        ));
    res
}

// Evaluates a periodic column at a &point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at &point**(2**k)
// instead.
pub fn eval_poseidon_poseidon_full_round_key1<F: SimpleField>(point: F) -> F {
    let mut res = F::zero();
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x143ce163d9e857b549efa236512d839954411bc04e888aa114215f991ee8a57",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4430620ab3eb75b8b2c3ee9c8bafd3408efbe93661f670002b3f96d354c2bc0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x16ba64f5ffc9bcb3a71b49f79a1c26ce608e33f1b6ce5fdfeae1c732b5d0b5",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x26315e8a17d10270d98790f94772ab99b185baeab1e0ec64e783de5c5b35859",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x21052369229137423604dbda64cdab20290c4da86882c0444750eaf0687d1c8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x67fa64d83009acfaae5a7a0e910d322b5d4dbc825090c1239dc68cd18338ed4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x17190a2c4fe2fb2a1c4061a3aaa8d89e8a363f653a905e43ab819ff47516c67",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x587584d86e310744ac2167594e87c72847cc1018d766c61b29b572ba4552a80",
        ));
    res
}

// Evaluates a periodic column at a &point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at &point**(2**k)
// instead.
pub fn eval_poseidon_poseidon_full_round_key2<F: SimpleField>(point: F) -> F {
    let mut res = F::zero();
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x565a88ff293c0a9c48cb67be157ad800604990d390e1b173e9bdc09abf9f788",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6217cc4bd0f62fec8a25f305b3914f3c6c2df7701aee105c60cd37ef815239a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x27a6021b1b06d9adf868d5ba9b068ecdee5e65fe62163095b96f7f4c2fa6c3e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x71273291cc9fb7c500b008872a8890e1e3917ea2b954d1f4a9af67427323126",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2fa9daffc6ffa8c6dd8cf633aa7c2d2a113a885f4ba935ff7f0198a4ea056cf",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x71a637fccbfdcc8da4828cb4734b6887fe9ebd78725ceb92d2756ea4e4c86fb",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x646004831088eedddafcec3518108e2033e3e613eb2b2b0ca972f75946901ba",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7d384f90e1f21f53dbafb1648ecdb97d8c020dbad501b0d79a491587484fefa",
        ));
    res
}

// Evaluates a periodic column at a &point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at &point**(2**k)
// instead.
pub fn eval_poseidon_poseidon_partial_round_key0<F: SimpleField>(point: F) -> F {
    let mut res = F::zero();
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3413bee8966e47edad4d25455e74664d547713650ae8ef6f7f4bd1d56077b55",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x624b1ba9e7d45d86f0a2ef7896a159e8e3d418234f3950ae2c1a1106b4d8e64",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xaba2f20ea6ee9cae2a9a5ffab6bb531cae756025a2039dbb3fdc7f6a7ea66a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x18e5bd14d527406ed33ef180f4351d66ba350fd42a210f14b13774666960edd",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6fba7ab30e117b743f154c4c1ef96007fbbff3b8cddbffbaa3cf1620dad0df2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x641cd514114aa297433e1ebb6f6fe8cf4c5b3816df09b39b38bf3851328781b",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x52208d8264d42061c7107f7945857541692a87bb1b4b4307c17d43193be3ad",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x44e2813694e35f41733099371352f930e87366ded64841028c54de5ae0cf86e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x58372f1bada3f7d38dee566363d48fc45a542d57a2357a00006f8c4508f3858",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x592bcd7384ba517197075eca669701a6d8eac3bdf21af499e3defd891fc8787",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xaa7db6d9cd63141d64bc671099b444013d3ac056afb7223fdf97319f7bd76f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1e8db0feaf54299f9e0daa802e5a00c5b43dc189f622dc9d0d8039fc8f4eb16",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1125b5ff47f1e0c4105a6b62e2a6dcf3d71812409c77b4c708825299e70bcfd",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7fdffd1f06d45f58c50609eddb9e4dcdf9845c3e13ae29fa3e6a4134615463a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x770f2c3dcb1befd2dbdd3e874a40ec38860828877139317823bc60ed3b69be4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x54beab500732d6102d1d501adac8f41fd04cf465e580d8664009c12e28fc5ed",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xc1e6049a1a088b613f8cb972734a8c4ee6d4bc5a359d5ebf272eff71312c01",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x24c477665b5e4b3843749877bcce106ac76c085f15b0759fe9d8f1d04b723be",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4d2b779ec13ff444eafa96e2e505999c3f79b06939f6ec492378d2ccb49c3dc",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6429e4ca7107ea29d85b4c45f4926f82d9d72206fdf33d7e499243b6a9ca81f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x42240cb9baca23c27a0eb13e654a5af7a490b95b51a152b8a2da5f0752226",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3c53eb4b33fc6cd4e86c4f3fbe866d358233a54b0f7c626f0ef3164ac48b189",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5cd8a4ce2b3274c77469ce2c328d9f56ed2bafe7992707f64ce99d42968f648",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2b23cce09410c815c33da25e53f0204d5d6f474f5f784647a19e9114e4cf753",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x398c6094de25847f31d6458f8bb9c6952ba9092ba7abc54d08050017ae2db64",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3a8053abe10aed5567dd7d40517596eb747cb829760fbc06f5bc322a0911c84",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x324ea07796ce3412e6f938ef1a2974abcb3f8ff7114fef8e0fef438b6e69b89",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4d5422ca4881cbaf9e99fe864068190670a6a1074e21de1382759182177eaf1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6db31dad71bfece85b88afc622cfdeaa557d4bfb3d3a313eaa4235dc7ec4ac9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0xb375c79888613ea49838515cb5f6842dea48d273b9699855c67d0978f13925",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2a584b677c86b2a15d48c57df9dad7188545a3a994fef603e86ac16ce1facfa",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4291c5f5cb048e49b20c5b3caa1fa12b99ef81488aa83663110b12abfe704d8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6f2eda70c3c0c744df7d7bdc1ded3d80f290f951649456874904374564edf90",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x489dffafde7fd6ef39e1542159c9d49bfdefe802fe6b358d6ddd1f28942ba69",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x163df55208e1561da127d03f6b63d46e0aa05a1ef3321cfd5711eb4d3fb3ff1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x293681f3dfad87cd19bd1cdf5c6244a5f943e411d7a035121621f8692fa77f9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x50c9a8d62edbd150d6090cf1f0831c066282b324ca794df5aca0fbc9e71714d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x18e0bd645ba4fec89f9991a934891217ba872651494fc08589186d6e6dda88d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x8bce38b2895c04a0c7620adef3a51a8319fc4e151359a52809b1509f48f662",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5add69d3f68df10a5d626eed04e8c34e83780c2c3eb9e07bc49ce7f1fe9f618",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x90916a638846883fc6bbc6c241dd630e4346567e5520bffeb17d0b05a17cbc",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6ec11adaf8da159cac400a273fdd7765371056001451e6441a6cc9da18beb31",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x58e3b2dc12d9ffe27bf5dc6c28a216e5612a7a0775f902c537806d2f60f4226",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3b09364e6fc149b3063a5442b78165712343e075297108206e246e0de596874",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2a05aa150252d7f810276589f79dbd0aa619289cd283f72ae0d34f141635a13",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7e9c35a05ef0ccb7cfe93272e0b46324e97e1512fa4c6e1d30ca2c00dc207b8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4a2ae027e432c0863e1749b62e8533e649ed78091e11155d341cfe47168bb0",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7f610dcb987484937a18dbae85c5b46f0bbc5f2cd845487501b2f1f7bf9357c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x41b07a5f9241075a4ff1b76a9a529c9315f2435f79bab965fce61c8f616badd",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x301f8e1e5f31d9f0546da692c88e007789002e56c4ccf68f3bd5fba12db838f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x410d9eaa6c615c482f890e4c738e555ac3e4892272617bc7a0ca80613e27fc9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x583d1f426394c7610a252cae8485a3e6fe2f5fcadc19fb5097a5c55c0787fd4",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5bd0655433a76820184b6dd6fa4f3a67ebc321c75d1f9bc7422fac69074e2ff",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x74155a89a923ea1e2a23985156091d435b5b815ae1e9fa573330f01d880e52f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7523eb70ea4b345b7be4f151bfff9cdbfd589120b63d0b7a21a5cdc3d36aed9",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4dda9dce889ee4e52e93f3ccd9f32bbfbad5a8e1768aecab88170f78d5f0de1",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1cdad5777ab21cdea2c8f5994456ce2253e8b020ef32d4d12714106b7d2f632",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1871b013899aedb3e2551a73c9f7f4189e86dddd5dfb8db56965e67812ace0a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4eecc6622ccb897afcd651f5bb655b47101430a53a29bf743f5b1041ac8ff13",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5432e64cc316b7f386cf5467af442acb9d986873c5c513bdbdd133259ad54e3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1da26a447725d6a61f31012a81300349baf580ddfaada24630d03ed293da28a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3bd2aeed72b102694fc4a99c25a5250c234c91b03680ef4212885989ba9267f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4ddf2d7e380560d571e765deec379fec8ae4d909848b18b5389ec295140687d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x47237ffdabc0cba010385bf48714bb06a6a2b9316394603c450330e743124ce",
        ));
    res
}

// Evaluates a periodic column at a &point.
// A periodic column of N values yields these values on the subgroup of size N.
// To simulate a periodic column with 2**k repetitions, one should evaluate at &point**(2**k)
// instead.
pub fn eval_poseidon_poseidon_partial_round_key1<F: SimpleField>(point: F) -> F {
    let mut res = F::zero();
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x26f2aa4059eb10ba60302d001cdf4a5482d43e2d7d05bd2b5486cd8c52ab9be",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x722090545903a2f0b654199a04a5db8fc128eb36cbad8255818bf1d5db2736d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7510614da9b9ad318575990ca2107d7b8b4e66622a28b08499b7444a86e0d37",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7658d45c2170beb301fdad273c8aea07d4add3b02890567fa38c0f6b5c1689e",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x38ea9c051a4621f17bf1d34344272953018e378f8b587aabac79157963d7a11",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x794689eb6cd1d1acb82b7d5741d61a961b28a3f5468403a1981ddbc21eca96d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4883c98a63a118749cc26ac47607af9d17fb8fd36ccb743e2b6dc13f202a42f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x8729cd967a805126fa9fd4136a390051c690dfc413f1de62f6fc13123f9586",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x17dae5cd6089cc03cafa39762a14985af1e7a05e9bbf55d3952c86839098c06",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7406ca984b25f47732349b87565103d2bbf220ebab93085c063ce5ef28e7337",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x30e1c6b719648866af8220a2220904dd632b089e54ca459dcab5d853043fc25",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x26993661e64b45b5787fd8b923ecfb6f681b554191429fbfd96f7010aba3115",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x57455541d2426a546ddc818e996e5e4120a233416ce5da3422b065b60c287d3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6ae8114b8a4b0e360f3108b4c4679c6e51d7870c05c3cc5504007a29c118b53",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x5acb7c9ee9cc689cf9ed6c611a1bd730f43c4ea34b94e07ed804fb6d2bb8d4f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x29a15985dd04254ad523298f35de868c8f4538f2d800d6005634b3a32bb00f2",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x292b8bad037db0033c816ef6752c1bb9d551215a498452832f721cd95519372",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x31a3edaa5ab567b05861b16a6e0da76ea8e159108d2fe83eb73ad7b8f86ef7a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2d35fd2bf29729904a91cc5ebd7d79362c34828e0c37e09aa4907de26a45fb3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3b1f311a53410f51c90fd8a9189465059ef46149b8fb7930963ead8eabaf53a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x230279d6296ebacdcc9cfa5bf60e5a1d4ebe3ff0ac8f8bf8318c988f5c99bf8",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x2369e96b64fea009a1f66290a5dfe08010918b4ce3bfc9066739a4dbe133a0d",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x13816f7acb88c6bf0356430faf0c4fed6972a9498b29919af38d9d5f5ae440a",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x787d78882592b85f1de17e47bad43712e69d0899fc94beff77d62d2c4a1375c",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x1619514ce1cabc2996036ec8d3e3476a8a2d9e83be3e8aa7a020ad11b548622",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x176e131cb6830fe0570f692f5cce9f3f37d3444e647a318f35d1138bb580133",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4a300aa8b63feafeec79c07fe87991c0e85737ae1fbe3aa7f60d285bcf89cf3",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x7845d76e25e1fe6f884a162b379bad9bd7e421befbd911bfd1810a6973ca552",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x32e59ba3c11289dfbca64ae5646d50270c6f78f070e0ed6f1b24f45ce6832a6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x3779baab792c4c306342b4e6640c4a1c315ff9d08e0fcc97576ef889dd657b6",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x6712fc14a35a8b515bfc975d8cf356c749b04d5d7abc78ba6b2aa0924d6146f",
        ));
    res = res * &point
        + F::from_stark_felt(Felt::from_hex_unchecked(
            "0x4b7fdbd1ae93d05ecb29e4702e1715f462cce519dba31b9f4b87107ada27016",
        ));
    res
}
