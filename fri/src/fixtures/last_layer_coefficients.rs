use alloc::vec;
use alloc::vec::Vec;
use starknet_crypto::Felt;
use swiftness_field::SimpleField;
use swiftness_hash::poseidon::PoseidonHash;

pub fn get<F: SimpleField + PoseidonHash>() -> Vec<F> {
    vec![
        F::from_stark_felt(Felt::from_hex_unchecked("0x6941a93343d0fd81ade2e819b1ae68566ef80fb395641e8aa6c06ec5447b177",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x5fbcb82412fa24abc26fe1832696dd0ed8a80ff3c4a663938fd437b5d81ce3",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x3c8dd619b4d99925a1cd396f9ae026b91637e4fa461a3d0f03631d32468b96c",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x2470e6f572dc854afea72b5a7f9047229bf427e298a743ffd2a1bc1b5f40eb",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x31fd68df1cd80fd9ccfbc388fa418f88608753d1a7bc34f78bbbcacf48034aa",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x519b77cefdee5f7f7144645b386493f21b85b162fd35df9fbec7f919f3e063d",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x4f47066c4cd421a7ec1bb0867d9080dc22dca08b8e82e98d075aca89a908fa5",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x30f255e37775bafd2a09c4b246e8d5a13d48889dfe769325931abda001b9e72",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x6f04b106d2db6e8b71cf6ae9d4715e9dc7b8f2bd0aaa64043d2b705663e0e68",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x743abbc6f56856ed661a8e3b20c506b4cece681d246a58d887dbcc741f912b9",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x1311f043905d5624912fc38bb9a50f5895d3c0655e49169de5e9f056c5a53")),
        F::from_stark_felt(Felt::from_hex_unchecked("0x6f67d8c8e41df84736932a4826f1cfe8d9684f4be42d9d5e791bacd5ddd3bbc",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x39c71754e4f444187bcc05be10dfa0cb7f08a6bb8fc73212a23397ebf9cd1b1",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x16e18d7ae271baf9b1f4edc5c653185deeaaffaa91d9ddcd18c548c248c82f8",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x5a8e97bb7b658d6e7ad1f9bd7f2bb6c9a879ee18ead93f6c5bac85922af236f",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x33cb6a5ea465e680525c3d3dd7929692b83bd3272697ca80d2992bd954621bf",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x2a2f4dc498ffe80a5569c884b3721d9c50fc2ab6de3c0b788252fb688debd36",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0xfbfd3e53746a5015ff7d0d09f38b460665f30b8c5e0b8f58a4225ae494eb82",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x21f90d3bffaab12acbace536fb1fb2f086de968aa45e5837a0185a29185afa9",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x259d7444a36296a6973067d5966ada7a586280080215027c5a63b1f816e1de1",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x5bd38fe35f4f53e08f62afc698b5d577ac416c520be60f159b49942d0c4987f",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x7ee38bc6dbfe4bfaae780c305576f832944d89f18a8b90495867156c297cc0f",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x6fef7507ee6c2e79a090d26c819236c8f3dc244bd984d14dde9d1ed0ab0a990",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x31b5ad1284877c267e9fd768be5ea13edc7ac3d333e59f82048411cceff0c28",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x2b1e49575690a90a4516fab31c23a4acb4563fda921a4fa39866e58829afd39",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x62e029f2bf850c91cbbacab4bee8668a415d9bd688e2e53f2b94cabf1208b6c",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x271dd1bff618916927029a3ddb25e5767b49a24c9fa3487d3cb06f0c1f053b6",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0xe831a3f4b58836a96ece4aa2ef509650d708ab7f88777f3df82451e5a820a0",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x6c9eb3e99d45533f8226bd83f4906e25a8c8b9de27d6a0c2ec917d2630365e8",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0xb48d45a3125021d62a0633f19bface0e1bbae598b3a4a1657e61f53c5a83c3",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x3972e95029706f0a128414e368dbd8947d2dc9939e6d95be37c19c6ad2d699f",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x3080c04c35bef712753f7f5ae51b7254238317c5e5300ab634798655ffbbeec",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x4687393433fbfb28bf3f639e2c799e9aec264cfd23a5212062729f13dd350cd",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x3b703d64a34e49dbc5f7cf096bfdb5795ed83e40e56bfa579de3117016e06f4",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x6ec675e1e57b707083411eb65a1225a037946bc989629428f5d81e324f7053",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x7d7700030cfd434a4ccd6c51be14c62d9bd5f7eda699377d0902cbcf3cfad9e",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x24b6606a3105d80b1c74488cbf93b7884b72d4b1b343eb5e4214d76780bc4bd",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x586282f2ef88a6e30cd4e4d59b18d960d76c9b46b01a58503c55917b39139c3",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x126e2903676c0c8e9a7120831ab374e4ed8e9ba866abf1d2c24d3bdbd20d2f0",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x30d664b7904f1fa5b8a3732d8d9ae451bae683c9484350f7bd21d8c7c853fff",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x16ddc2a2973831423895befc12630bf67123cb894f67206b8069e33f74593b8",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x656a60984e9f9049f7230531a96bdd7c42c6ff14d3a4d7f936555168780c317",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x494108eba7ca282dbd0d4dcd483aaa6add595956eba408e9fca9ae112eeced6",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x1755a80daa5033617f6a9d6c2d55c11d77df28024485cd20e405fdc1a05167",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x4c8acdcc258acba4571d62762d949443d18949c98f4480fe221d25c0dc36b57",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x7e22a481fc3e158baa2ccf58b9e2d28072bd5d4bead1b653ff4cadf3f5cd914",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x395728bb2b2bc71ac89943e9dbb6bce86709e5e3a837a2be63ea930fb85d02",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x154ed29bdbb29510479d4848ebcfbec5e0833b340b072d56b893bf454c87e1f",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x69ec99523c444b2bb97a0c9c2dbd44a0837daacbf5fb671c115546523fc5ccd",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x64dad8a61a13b51e33b44a19118b16c0ccadbfa8d54b1f4574af9405df52ef3",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x8940347cd6329e0c2d5b51926fcb74dd21efe9c7ec43d12eee8645d1a6638c",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x44f0dbbab208b6fb6ccae4ae76024b9603813d045a2aabccbf65089e7afe2c7",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x65802c4ac8934ba0ad679bbd27436a53496fd4d3c360c0f10da57da1dc5bd32",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x1c3bd67b83a6addd84173e2c2d1ec5aef787870a2c8e5ac36581dfbe8680e9",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x145a4a9c4b31b8b820ea968ef283f921829449f5212632d4664f37ab3b21fb5",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x23b31a415752266662de10951cd77a13748443576e7f4f454353ab6e22f9b45",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x7b70615f00a50f0a9e409320632ccea88cc630340a422d04816c7415b439fa8",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x2e8ca47dd99d262eb4fc31eddbce7849a2b7106646440d614f7d1fcff6564f3",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x4877139f4be966d0b5f1c639ab2a4767e943d968d285083a23b344d26a4b7e0",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x2b639cbb541e190301fc7053a1988b601bf8a893f9b10e090138692e9773a6",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x62b24c7c01920bad289e09d129f6b12f59e84c2835ce46e8ac53968cc6ac256",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x427a59a36529be08b60ab576bcfb3357bdb0b08e3207aab21f7063cf3187efb",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x677147bec29e2b730faa90842c11be38c2671c9085e5693c41808a70c5df58f",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x95454f47fa5082c2cb704074e744605e8605f641dba556569a0a662cea999b",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x54741faee3d357938c194821379b3302c495e9ee7496dafdcb904098f0128d3",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x3176fb29fe6ac9be1db0a5098183d6536a92f962338c63ed607626681e1b35a",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x4a52e04c05ec0cf7e71915afdc387d51a90f3d4bbbcf2e045f051d73ad10090",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x48f5aec4465815948faa86fd4ef02f14329b635f349fb37e308340be186c51e",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x2bbd34b7b0723a48bbaa417832c344a20a7e92b50500e68e6436a7dd4dc8994",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x25a13efa83b3001566671837e46948bb566e3ceba8d585329b435be43a8b728",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x3bbb58bd592ccfd005279cb6ec1c348aceee95ae45af9a6f2318d6c37daf867",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x7dfd6e7ed067c74393600e5eef1f661491af769717de80e3e00224cc7fe4e3b",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x796d55c239dd89fb68921154a64ada73c6c6ccedc1d5c22fb49d2b0fde670d7",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x78f066282c9c732a1e4a3aed3dfb9f1ce0ebfb9721a66a227e84c93f85ecad5",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x37c1fd0eb78a9d8c075f59bd61f612bd32b72489710d9afaa682c3d0343c92",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0xe6b299cceddc1ea2661be2a9e908dd05752db500efb4130f037cffdbb9c39d",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x46f6349c939d2db2601bbf7eba1d880de9bbbef08a8c202b1f67c047b406d1f",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x644d0987150874e7b73a8854f88b191265ba8767ad4e2f81f29ac0fe3360f2c",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x77c2a45ace6b09bb59e7b3977d3de9cafe0ed6c34798f6fb313aed7363a3e81",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x6ac07135bad1997cc12b2db37ab118332ce759d34f45b54a15641c59a89f413",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x14014466f85d38c43e42d5899e4a0c0402b0179ac6a0bee8fe09780b319a84d",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x6e23cd3beddac094b31b51637afe591eb4c138f77a501e2a17652c91684ae8",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x43d31d1bd7cb81ca009e7a4e7060d25210dc6119d37dde86e8de310a832778b",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x3abb659515d348d1a7240ed4d6a15c8d6c2d7b9df6749761256a67abb234d2f",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x2ddee93b2cdf39daa47b3237f114eabc989c5c7804788d22f2d675c704a106d",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x4f50a970402337946ad09971d632f5d88d073e2ce82359d235368b62e4fdeaa",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x72abce782a5ad5bbca5ee0f6b4fa63530b55029af1a2bf1f16e47b0a7d8c42a",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x26ce949e4568ec346cbbf00119612c172bcdcfae3f826de5e695b2afe91c77e",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x381c3c6ca4cd735288628e0404c5c5dd90ea327654f157c36991e45cd8482eb",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x3b357022e6ea13dec14bd293f38eb0169e2c532d65a2de8c2861d921f1af5eb",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x369407531954511abab373e2c1ea7fa5e31418e2151b68a55b79ccbdd58bc2d",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x7a44467efcbc0e449a5f7e50b7c6b80cfc499619ce191247f491ddbf27e004a",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x632d1f0d836e0cd02bb5c9730e9e45cccee15f344fa26bea2f1cf6859052f75",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x51c8617c044214bb880d94c3f7cb82d16540b4f662ee98db0d0d209fe0e3deb",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x637af67a96f4172da4e5ae9e7b4be37138c564eb1d399cbb6724c542a08794f",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x3662dc7d5dc04739b12d4e5fe560316ee09a83cadb585f6a2dbb1c48706300c",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x32cfd04156d47d583fc69561552d3b21898c1d37e573b3ab58d868f7d499b17",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x1fc7740f3e433101592139ae15a43e8425ef4c4337a656e33d2df020b6db12f",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x52c4276660707491ca98a2c7ef340c81c90ac6376099f36cc0723c1512ba36a",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x2e8339a3c53ada702f74cf665047791d4c5ed255c957c44cca4e462ef03138e",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x750fefcbfbe90c659600a944fd714419d4849bf4a61d32ecf6ef409270afc2b",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x1355ba033a7d3de5c3c3f9182f860df0f8221b7e0f82ddd191146c6ce9a022d",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0xe2014f54036c9c508eb3537313a734e339d44457ec5fe4a5a08a6483370f58",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x717f45c9e82c5006475de324430f9a78824b2055bb38ca81bfa757de04b4342",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x29ad87e21ea3e24941567910d16b33ac39071d6125733741a9f1073a9642c60",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x56d73e5d4a9e37505ee7b780931f1c0e06566e5134a2beae6562070edebed83",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x62a70a3d5f22becaf2470cb849ceccc10a252bb45272677b3bc48e12d1a5f81",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x76ec29ca8d65854de9e286d3767fbb092924c8cb2806ec41e360aa6f0012077",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0xc8d3643aa158cbffe3b4959ea244c0caa8fe2c79d5c810670a8c8cd32cb8fb",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x5cdc78954befd42c9cd8add559c83115dba4661df534d49f763429ca7f78724",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x41949d3462bf855903561ac747276b8ee9fc7048dfc56ae4438abaafa045b14",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x6e4c322dcfea29f0df24961ce188489d3606ed856cbf4fe66428c66a9d20cc7",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x197155cfc0bfb026ee34b339cda8b1a67f335a9ebf89544c0fca32fba060483",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x1fe58a64e1eca0dc2ebb81024394918f6f3ae03f29cab5e04860fd6f36a09b1",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x30448f6e656829d3fc4d501aa9653975caf32c1b626f1433d297f1fbdcfaedd",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x3f37b5986d17808d5306015f286687e41a0e7a27eaff4e244eb18a6dc72bc94",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x7f74f057dbf7f2e36def6db9b1c9d6cbeb881356fc326d7281251b6cf45bae2",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x6b3b573168cb93848f03874d1a27735783f6f70a8c08521a00583f305488cee",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x1eb880a292c84e350523ddd4622e88c7d7edcd39032afeced4134d16779a9b",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x79fb4ef5da73a39ba608bcf249a8308d3c4b2f25b3788f452dfcf5c1fe5102c",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x7721d970803cfc3976ffa86981e0703c24419c30e208192de61401265d96601",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x619ea2f2c228b3440481a5fe239e0f9c522ae7b4656de9474da10b9e89a95b2",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x2f2c99abb1869f10559249295ba924d9f8edd01b9aecd90c2e0f30f5e331131",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x2f55d5dfcd8cc1d47758e64334c8305115c3b8a144d5c44792e477dfa7b4512",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x7ae7c3e5b90234d7d0300b7e41aa380a09307a28f6afbeeddd3887d488ec694",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x68908ab4c2e20bc459a3358e93608429f9bf185ee7e58c63004da7bb7e0f51",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x66fe1941a61b8da3eb7b4e7db5e76628986542ebeb957b19fe677ae95f85339",)),
        F::from_stark_felt(Felt::from_hex_unchecked("0x38eccc03d4f88d02405296ceecd40e11106618e30aa9db7e924619ba3c096f3",)),
    ]
}
