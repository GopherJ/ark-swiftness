//! Periodic columns used by AIR
use ark_ff::MontFp as Fp;
use swiftness_field::Fp;

/// A periodic column, encoded as a polynomial, comprising of the Poseidon hash
/// round keys. This polynomial evaluates to the `i + 1`th row of full round
/// keys (for `state[0]`) when evaluated on the `i`th power of the 8th root of
/// unity. e.g.
///
/// ```text
/// ┌───────────┬────────────────────────────────────┐
/// │     X     │                P(X)                │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^0    │   FULL_ROUND_KEYS_1ST_HALF[1][0]   │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^1    │   FULL_ROUND_KEYS_1ST_HALF[2][0]   │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^2    │   FULL_ROUND_KEYS_1ST_HALF[3][0]   │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^3    │                 0                  │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^4    │   FULL_ROUND_KEYS_2ND_HALF[1][0]   │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^5    │   FULL_ROUND_KEYS_2ND_HALF[2][0]   │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^6    │   FULL_ROUND_KEYS_2ND_HALF[3][0]   │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^7    │                 0                  │
/// └───────────┴────────────────────────────────────┘
/// ```
///
/// NOTE: exact polynomial from StarkWare's solidity verifier:
/// https://etherscan.io/address/0x37070Fd8051f63E5A6D7E87026e086Cc19db1aBe#code
pub const FULL_ROUND_KEY_0_COEFFS: [Fp; 8] = [
    Fp!("2031256392032510728323540713732249591121305006506648848985382400586597610737"),
    Fp!("1058884251835032902902959934847754981443976379105821846535521887778003815056"),
    Fp!("3199116735214460673119860668797791295709497126247935323420345647237191227032"),
    Fp!("3403593029026780926333265226464133854272305860522439300723126785401542560911"),
    Fp!("778805400360822368207731183437512943743151741287620774689794956671172832290"),
    Fp!("506510838109096059220730854778109913863437083975464167557121102677882315441"),
    Fp!("2692520644938914523392225523713543975571066502545988597642254010798055841422"),
    Fp!("3585697187696786468041264729514461885587375226952972025977220400366736024776"),
];

/// A periodic column, encoded as a polynomial, comprising of the Poseidon hash
/// round keys. This polynomial evaluates to the `i + 1`th row of full round
/// keys (for `state[1]`) when evaluated on the `i`th power of the 8th root of
/// unity. e.g.
///
/// ```text
/// ┌───────────┬────────────────────────────────────┐
/// │     X     │                P(X)                │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^0    │   FULL_ROUND_KEYS_1ST_HALF[1][1]   │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^1    │   FULL_ROUND_KEYS_1ST_HALF[2][1]   │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^2    │   FULL_ROUND_KEYS_1ST_HALF[3][1]   │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^3    │                 0                  │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^4    │   FULL_ROUND_KEYS_2ND_HALF[1][1]   │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^5    │   FULL_ROUND_KEYS_2ND_HALF[2][1]   │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^6    │   FULL_ROUND_KEYS_2ND_HALF[3][1]   │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^7    │                 0                  │
/// └───────────┴────────────────────────────────────┘
/// ```
///
/// NOTE: exact polynomial from StarkWare's solidity verifier:
/// https://etherscan.io/address/0xb4711a4614368516529d6118C97905aB4B28e267#code
pub const FULL_ROUND_KEY_1_COEFFS: [Fp; 8] = [
    Fp!("2500698040461080674773139952381198839638778571438636910046275161256801675904"),
    Fp!("652964806634297091011052233362268594934430242798313942867095931791011310695"),
    Fp!("2939414448332072724875253100414078308043974631371534529978201600425249181396"),
    Fp!("933462664633036044541055050313690761446142252651413906931929096455205671368"),
    Fp!("1079694764967229138366396911064796417614767939275741410725217249097399097433"),
    Fp!("40157082147030768552107673477087688599112273433463576858312217366627733685"),
    Fp!("1927672438903793236031198529848894371333243975830731128569985457311697349568"),
    Fp!("572113961278945390624169048852605746055969372192753638936252529960127728215"),
];

/// A periodic column, encoded as a polynomial, comprising of the Poseidon hash
/// round keys. This polynomial evaluates to the `i + 1`th row of full round
/// keys (for `state[2]`) when evaluated on the `i`th power of the 8th root of
/// unity. e.g.
///
/// ```text
/// ┌───────────┬────────────────────────────────────┐
/// │     X     │                P(X)                │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^0    │   FULL_ROUND_KEYS_1ST_HALF[1][2]   │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^1    │   FULL_ROUND_KEYS_1ST_HALF[2][2]   │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^2    │   FULL_ROUND_KEYS_1ST_HALF[3][2]   │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^3    │                 0                  │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^4    │   FULL_ROUND_KEYS_2ND_HALF[1][2]   │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^5    │   FULL_ROUND_KEYS_2ND_HALF[2][2]   │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^6    │   FULL_ROUND_KEYS_2ND_HALF[3][2]   │
/// ├───────────┼────────────────────────────────────┤
/// │    ω^7    │                 0                  │
/// └───────────┴────────────────────────────────────┘
/// ```
///
/// NOTE: exact polynomial from StarkWare's solidity verifier:
/// https://etherscan.io/address/0x4FB05b7CC348C5a72C59a3f307baf66e3CA1F835#code
pub const FULL_ROUND_KEY_2_COEFFS: [Fp; 8] = [
    Fp!("3539912415782779599122645682228323725845348737630462191562757603420971269882"),
    Fp!("2837558332314068247341924887451449731336046919722520548593080808680890892730"),
    Fp!("3212814682131296835268738489319086132340321330913282582597217989594356418299"),
    Fp!("1347425782113004596959884742203547293270903553049680363963920329314389743311"),
    Fp!("3198787996468404797054919162695519790615152785742672594891965171169724870950"),
    Fp!("1120844515109881529308595795213214924647761105714342715524064761247549779006"),
    Fp!("2773044165244186651230214283470825130260620864240013282273579979977860129690"),
    Fp!("2441179170663939450438781661794771519027802813305140163656909544306356385672"),
];

/// TODO
pub const PARTIAL_ROUND_KEY_0_COEFFS: [Fp; 64] = [
    Fp!("2011058453588713720249123693775956922878707212487922032862264007270822126798"),
    Fp!("2201400638612229879162360768067398810413954949723023944021920055901410060413"),
    Fp!("1691168953466643624865245449195988054903726721572677745282185974497468294783"),
    Fp!("837752204021605327715160395330797487102481208539228487427947956029869630090"),
    Fp!("2380263194087140445105962283337933941330390837238505085577084716209620735203"),
    Fp!("2231171705584461626219509193605920937154699735111651166324180876986249969427"),
    Fp!("691023582400368151425499578644682002993815109358501180253153836927178952202"),
    Fp!("815712857098752335465822874251324712683954559804455434201967167181304297010"),
    Fp!("2200896946459156009846999085508750344518635263077243168217378202438594399713"),
    Fp!("3311504242880559186906289974222556212177173749439946052400816630613769891545"),
    Fp!("3281626193282231458894797057713120383292667019437105138719321469870604870959"),
    Fp!("2595542047312967823969139776095094215792736848354262072270364139034663772927"),
    Fp!("2494470255641204693373951627990755478939566816743530326817044130885076025300"),
    Fp!("1839024952517621240148375577443119360154339660198916304905856489632607403977"),
    Fp!("1360423116113108511511497884143773970748909336603644522105658835738197459855"),
    Fp!("1857009051935306160156068085116029921162702946378227598456855109159215348445"),
    Fp!("3600950696686588582476467502787179945215441888071387185774270376638527649148"),
    Fp!("131042599369273776049953022479346094604946968292827874807385621310020684720"),
    Fp!("3579213573733953387118074698596977327254071670778683287052996808576059180984"),
    Fp!("1187946733693976194680866767164361748278602816744612270201356897672845548051"),
    Fp!("1668920906183966718844909027436064328219511118180304667818990347333708376180"),
    Fp!("2512864962690964498518583493949462908552960782966303680210470792426995794470"),
    Fp!("3130975011033880246968733267409908092405714799218937413525105832416250686257"),
    Fp!("255429598784696064225798137849386054805272155656354071337626616328297217212"),
    Fp!("2568709998230648299478741631398126812268414080300466333973067163918513337880"),
    Fp!("247015030310741953830313401436147267923682996291128770579600739705064846946"),
    Fp!("703286827763390949543255191325140556123276201303350118564927150503707388045"),
    Fp!("2283833088401962940862491743337449926009876591639572950835450509414336917837"),
    Fp!("1165070839594609882231448434734576346131360204877699424190750181455879108601"),
    Fp!("628772092455570545634696948020074778619306970147697868698037435933629562865"),
    Fp!("2052855298369150222140895326223812853776817706882609748443636410350664071785"),
    Fp!("3143094298659700197468180571237029106493115704312009498966004520694515163024"),
    Fp!("1881887943842872994654544963993447050594380908093260031674969742778453918936"),
    Fp!("1197071412697646109932927118826948403758665168613087916893460643944415079674"),
    Fp!("317078510010863808711921435221323999995766710781080621703998431703493327141"),
    Fp!("3101160684179089998010485418744819435886550128325017664383233963733629553353"),
    Fp!("2186046537954612104058961354922357721563971378723505054389774170636091386609"),
    Fp!("1422160250234177761995064844685640629304074194008588974227878184853642320777"),
    Fp!("1653804945057406761664299960167559592652054181513275033282602974300244876420"),
    Fp!("1626866096214457447353131310595846121199682496375598478019302407996384140132"),
    Fp!("1219544134201476223608458198059663034072645073160261956712213627034699364179"),
    Fp!("2624722405021981927284628209472631505272477323263035288175552820883694876232"),
    Fp!("1705440197452178348031493305623047936665439567174750229763897038569453695369"),
    Fp!("7303794513927036150443526957467421187171381334229685273052173015499219494"),
    Fp!("2831581540247649530344309751279789484933757764180014236421382117214379616287"),
    Fp!("2181555584533462839506660165587102437288422319970987885731920777554390795228"),
    Fp!("1039399290065386888059670728989329530711068617616029629109914913851840078782"),
    Fp!("342589009230579100869655403449881228285514990926465815232434050390798969857"),
    Fp!("2395697661217587689960413117415101752342752392813229403589615155648100550125"),
    Fp!("3365752314385391825538688991497681317826270293686345335138725158762212400100"),
    Fp!("3614967852738983401943551688676402047460348453115672964108326419370951919162"),
    Fp!("484746741610260848805047653419797384523026975808170816357775138125394263293"),
    Fp!("863733279204969122578485458230775871077735743688346649390956030361991441174"),
    Fp!("301231648953226191687796737757800634493022525241985058737339210813981579119"),
    Fp!("2520827245004312951028763345126579130423272832294677441491626515661846251399"),
    Fp!("2493814524510849468472603754000003895776214459806037625542524562549093906520"),
    Fp!("1947342058568602493578731388026571104173560600163872855912734417815497144430"),
    Fp!("145106130279481286282841880720188329803589955317433265140585959745746887597"),
    Fp!("2830139199320765337361906216820940731535745103903732116715972175533579991067"),
    Fp!("3158512911697354645267907609611608675370361599838845963888630757176317382130"),
    Fp!("703838833471174089158455984460528878306827610165194726969716803706729926365"),
    Fp!("303255456835421199388721782102376723575438809947380810971746663123581314666"),
    Fp!("2778710226175958524264360794670288591259207376969629073233667082388335595108"),
    Fp!("1472197238932273074617280345699913126639197451199647801548108788096142113621"),
];

pub const PARTIAL_ROUND_KEY_1_COEFFS: [Fp; 32] = [
    Fp!("2134335647277236274172649243940335615164845847046811650862308089928274309142"),
    Fp!("2913860402982745297192222836556238327676350387704743365206163352943615939695"),
    Fp!("1568267719665639329068021097686449424596763578883734209116294890374401775542"),
    Fp!("1438832787030345458159564327372021276446704853049296657680414856659331461798"),
    Fp!("3400058820156868235423381073658840048586987919336376284621360292676805371218"),
    Fp!("2097252063762419661158195314647303504235218184214541115244166149117675347187"),
    Fp!("662355037613787361248520866919088369589127966654473231732832401906009112883"),
    Fp!("624725934977501428908126150949682646058306899152541041649324164767007671842"),
    Fp!("3406201849572533639242164999792192263169824072501379198799385503000569132892"),
    Fp!("551414799926945114935931957038676882674275258651094086893483468013116998666"),
    Fp!("1001129977779070338184465124492598129855598351649956917265470863964279880205"),
    Fp!("989707767477736519138567160031823854690440421522390350577615124351647587320"),
    Fp!("1671348076294883574173629413733381569234769582548940048836611998536819209530"),
    Fp!("1278091775458215357337740799277013101497153998886847902142053204572133154739"),
    Fp!("1403310372422924025511615925475740554489460975480439543509413849788758552442"),
    Fp!("1163860326430000718046134830422603091806053130321254448183955127128856564594"),
    Fp!("1176869189607295923833069625214293402348267208207586729072901074552450908402"),
    Fp!("2566730401717775950645324738467461257934959667889273733557224286834682203471"),
    Fp!("3022199364693918131568583135019020773935797752643072673410681091087385332563"),
    Fp!("2467107418574549863183112932087925494933901313922495113400766347022199588819"),
    Fp!("1091161948796597579795135900067697303554663397335240530598707491663288873237"),
    Fp!("1381870550230811989378850214588398558335671987688920999343514478753433779237"),
    Fp!("3280018111039722005715710254587103942170658383725019294422846454704244683575"),
    Fp!("674372138400372680741447662832556892494101211344401445609878547300002401286"),
    Fp!("238812867981604900242730730962904790833095078985847249523658401722016961926"),
    Fp!("2049960815341355063267765714253140081115130937556658598531520181157671117871"),
    Fp!("3428405366212921189464692166285931213219252511553063377650769767569797458285"),
    Fp!("1609002408988616425227773974819821829315526569077095807693412587743519537681"),
    Fp!("3345616520536764426311449776963625327065910414283226943957811553324037662878"),
    Fp!("3309346525029019614570826948798124563777478018649485974112340052591392984375"),
    Fp!("3226324998128064023550162643984510276265270254846795904002410473231727358829"),
    Fp!("1101040016726595443223915982531692887649984393932412609031414028907016010174"),
];

#[cfg(test)]
mod tests {
    use super::FULL_ROUND_KEY_0_COEFFS;
    use crate::poseidon::params::FULL_ROUND_KEYS_1ST_HALF;
    use crate::poseidon::params::FULL_ROUND_KEYS_2ND_HALF;
    use crate::poseidon::params::NUM_FULL_ROUNDS;
    use crate::poseidon::periodic::FULL_ROUND_KEY_1_COEFFS;
    use crate::poseidon::periodic::FULL_ROUND_KEY_2_COEFFS;
    use ark_ff::Field;
    use ark_poly::EvaluationDomain;
    use ark_poly::Radix2EvaluationDomain;
    use swiftness_field::Fp;

    #[test]
    fn full_round_keys0_match() {
        let domain = Radix2EvaluationDomain::<Fp>::new(NUM_FULL_ROUNDS).unwrap();
        let mut full_round_keys_1st_half = FULL_ROUND_KEYS_1ST_HALF.map(|r| r[0]);
        let mut full_round_keys_2nd_half = FULL_ROUND_KEYS_2ND_HALF.map(|r| r[0]);
        full_round_keys_1st_half.rotate_left(1);
        full_round_keys_2nd_half.rotate_left(1);
        *full_round_keys_1st_half.last_mut().unwrap() = Fp::ZERO;
        *full_round_keys_2nd_half.last_mut().unwrap() = Fp::ZERO;

        let evals = domain.fft(&FULL_ROUND_KEY_0_COEFFS);
        let (evals_1st_half, evals_2nd_half) = evals.split_at(NUM_FULL_ROUNDS / 2);

        assert_eq!(&full_round_keys_1st_half, evals_1st_half);
        assert_eq!(&full_round_keys_2nd_half, evals_2nd_half);
    }

    #[test]
    fn full_round_keys1_match() {
        let domain = Radix2EvaluationDomain::<Fp>::new(NUM_FULL_ROUNDS).unwrap();
        let mut full_round_keys_1st_half = FULL_ROUND_KEYS_1ST_HALF.map(|r| r[1]);
        let mut full_round_keys_2nd_half = FULL_ROUND_KEYS_2ND_HALF.map(|r| r[1]);
        full_round_keys_1st_half.rotate_left(1);
        full_round_keys_2nd_half.rotate_left(1);
        *full_round_keys_1st_half.last_mut().unwrap() = Fp::ZERO;
        *full_round_keys_2nd_half.last_mut().unwrap() = Fp::ZERO;

        let evals = domain.fft(&FULL_ROUND_KEY_1_COEFFS);
        let (evals_1st_half, evals_2nd_half) = evals.split_at(NUM_FULL_ROUNDS / 2);

        assert_eq!(&full_round_keys_1st_half, evals_1st_half);
        assert_eq!(&full_round_keys_2nd_half, evals_2nd_half);
    }

    #[test]
    fn full_round_keys2_match() {
        let domain = Radix2EvaluationDomain::<Fp>::new(NUM_FULL_ROUNDS).unwrap();
        let mut full_round_keys_1st_half = FULL_ROUND_KEYS_1ST_HALF.map(|r| r[2]);
        let mut full_round_keys_2nd_half = FULL_ROUND_KEYS_2ND_HALF.map(|r| r[2]);
        full_round_keys_1st_half.rotate_left(1);
        full_round_keys_2nd_half.rotate_left(1);
        *full_round_keys_1st_half.last_mut().unwrap() = Fp::ZERO;
        *full_round_keys_2nd_half.last_mut().unwrap() = Fp::ZERO;

        let evals = domain.fft(&FULL_ROUND_KEY_2_COEFFS);
        let (evals_1st_half, evals_2nd_half) = evals.split_at(NUM_FULL_ROUNDS / 2);

        assert_eq!(&full_round_keys_1st_half, evals_1st_half);
        assert_eq!(&full_round_keys_2nd_half, evals_2nd_half);
    }

    // TODO: partial round keys
}
