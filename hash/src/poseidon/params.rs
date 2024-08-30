//! StarkWare's poseidon parameters. Source:
//! <https://github.com/starkware-industries/poseidon/blob/main/poseidon3.txt>
use swiftness_field::Fp;
use ark_ff::MontFp as Fp;
use ark_ff::Field;

pub const RATE: usize = 2;

pub const CAPACITY: usize = 1;

pub const NUM_FULL_ROUNDS: usize = 8;

pub const NUM_PARTIAL_ROUNDS: usize = 83;

pub const MDS_MATRIX: [[Fp; 3]; 3] = [
    [Fp!("3"), Fp!("1"), Fp!("1")],
    [Fp!("1"), Fp!("-1"), Fp!("1")],
    [Fp!("1"), Fp!("1"), Fp!("-2")],
];

pub const FULL_ROUND_KEYS_1ST_HALF: [[Fp; 3]; NUM_FULL_ROUNDS / 2] = [
    [
        Fp!("2950795762459345168613727575620414179244544320470208355568817838579231751791"),
        Fp!("1587446564224215276866294500450702039420286416111469274423465069420553242820"),
        Fp!("1645965921169490687904413452218868659025437693527479459426157555728339600137"),
    ],
    [
        Fp!("2782373324549879794752287702905278018819686065818504085638398966973694145741"),
        Fp!("3409172630025222641379726933524480516420204828329395644967085131392375707302"),
        Fp!("2379053116496905638239090788901387719228422033660130943198035907032739387135"),
    ],
    [
        Fp!("2570819397480941104144008784293466051718826502582588529995520356691856497111"),
        Fp!("3546220846133880637977653625763703334841539452343273304410918449202580719746"),
        Fp!("2720682389492889709700489490056111332164748138023159726590726667539759963454"),
    ],
    [
        Fp!("1899653471897224903834726250400246354200311275092866725547887381599836519005"),
        Fp!("2369443697923857319844855392163763375394720104106200469525915896159690979559"),
        Fp!("2354174693689535854311272135513626412848402744119855553970180659094265527996"),
    ],
];

pub const FULL_ROUND_KEYS_2ND_HALF: [[Fp; 3]; NUM_FULL_ROUNDS / 2] = [
    [
        Fp!("3515906794910381201365530594248181418811879320679684239326734893975752012109"),
        Fp!("148057579455448384062325089530558091463206199724854022070244924642222283388"),
        Fp!("1541588700238272710315890873051237741033408846596322948443180470429851502842"),
    ],
    [
        Fp!("147013865879011936545137344076637170977925826031496203944786839068852795297"),
        Fp!("2630278389304735265620281704608245039972003761509102213752997636382302839857"),
        Fp!("1359048670759642844930007747955701205155822111403150159614453244477853867621"),
    ],
    [
        Fp!("2438984569205812336319229336885480537793786558293523767186829418969842616677"),
        Fp!("2137792255841525507649318539501906353254503076308308692873313199435029594138"),
        Fp!("2262318076430740712267739371170174514379142884859595360065535117601097652755"),
    ],
    [
        Fp!("2792703718581084537295613508201818489836796608902614779596544185252826291584"),
        Fp!("2294173715793292812015960640392421991604150133581218254866878921346561546149"),
        Fp!("2770011224727997178743274791849308200493823127651418989170761007078565678171"),
    ],
];

pub const PARTIAL_ROUND_KEYS: [[Fp; 3]; NUM_PARTIAL_ROUNDS] = [
    [
        Fp!("2404084503073127963385083467393598147276436640877011103379112521338973185443"),
        Fp!("950320777137731763811524327595514151340412860090489448295239456547370725376"),
        Fp!("2121140748740143694053732746913428481442990369183417228688865837805149503386"),
    ],
    [
        Fp!("2372065044800422557577242066480215868569521938346032514014152523102053709709"),
        Fp!("2618497439310693947058545060953893433487994458443568169824149550389484489896"),
        Fp!("3518297267402065742048564133910509847197496119850246255805075095266319996916"),
    ],
    [
        Fp!("340529752683340505065238931581518232901634742162506851191464448040657139775"),
        Fp!("1954876811294863748406056845662382214841467408616109501720437541211031966538"),
        Fp!("813813157354633930267029888722341725864333883175521358739311868164460385261"),
    ],
    [
        Fp!("71901595776070443337150458310956362034911936706490730914901986556638720031"),
        Fp!("2789761472166115462625363403490399263810962093264318361008954888847594113421"),
        Fp!("2628791615374802560074754031104384456692791616314774034906110098358135152410"),
    ],
    [
        Fp!("3617032588734559635167557152518265808024917503198278888820567553943986939719"),
        Fp!("2624012360209966117322788103333497793082705816015202046036057821340914061980"),
        Fp!("149101987103211771991327927827692640556911620408176100290586418839323044234"),
    ],
    [
        Fp!("1039927963829140138166373450440320262590862908847727961488297105916489431045"),
        Fp!("2213946951050724449162431068646025833746639391992751674082854766704900195669"),
        Fp!("2792724903541814965769131737117981991997031078369482697195201969174353468597"),
    ],
    [
        Fp!("3212031629728871219804596347439383805499808476303618848198208101593976279441"),
        Fp!("3343514080098703935339621028041191631325798327656683100151836206557453199613"),
        Fp!("614054702436541219556958850933730254992710988573177298270089989048553060199"),
    ],
    [
        Fp!("148148081026449726283933484730968827750202042869875329032965774667206931170"),
        Fp!("1158283532103191908366672518396366136968613180867652172211392033571980848414"),
        Fp!("1032400527342371389481069504520755916075559110755235773196747439146396688513"),
    ],
    [
        Fp!("806900704622005851310078578853499250941978435851598088619290797134710613736"),
        Fp!("462498083559902778091095573017508352472262817904991134671058825705968404510"),
        Fp!("1003580119810278869589347418043095667699674425582646347949349245557449452503"),
    ],
    [
        Fp!("619074932220101074089137133998298830285661916867732916607601635248249357793"),
        Fp!("2635090520059500019661864086615522409798872905401305311748231832709078452746"),
        Fp!("978252636251682252755279071140187792306115352460774007308726210405257135181"),
    ],
    [
        Fp!("1766912167973123409669091967764158892111310474906691336473559256218048677083"),
        Fp!("1663265127259512472182980890707014969235283233442916350121860684522654120381"),
        Fp!("3532407621206959585000336211742670185380751515636605428496206887841428074250"),
    ],
    [
        Fp!("2507023127157093845256722098502856938353143387711652912931112668310034975446"),
        Fp!("3321152907858462102434883844787153373036767230808678981306827073335525034593"),
        Fp!("3039253036806065280643845548147711477270022154459620569428286684179698125661"),
    ],
    [
        Fp!("103480338868480851881924519768416587261556021758163719199282794248762465380"),
        Fp!("2394049781357087698434751577708655768465803975478348134669006211289636928495"),
        Fp!("2660531560345476340796109810821127229446538730404600368347902087220064379579"),
    ],
    [
        Fp!("3603166934034556203649050570865466556260359798872408576857928196141785055563"),
        Fp!("1553799760191949768532188139643704561532896296986025007089826672890485412324"),
        Fp!("2744284717053657689091306578463476341218866418732695211367062598446038965164"),
    ],
    [
        Fp!("320745764922149897598257794663594419839885234101078803811049904310835548856"),
        Fp!("979382242100682161589753881721708883681034024104145498709287731138044566302"),
        Fp!("1860426855810549882740147175136418997351054138609396651615467358416651354991"),
    ],
    [
        Fp!("336173081054369235994909356892506146234495707857220254489443629387613956145"),
        Fp!("1632470326779699229772327605759783482411227247311431865655466227711078175883"),
        Fp!("921958250077481394074960433988881176409497663777043304881055317463712938502"),
    ],
    [
        Fp!("3034358982193370602048539901033542101022185309652879937418114324899281842797"),
        Fp!("25626282149517463867572353922222474817434101087272320606729439087234878607"),
        Fp!("3002662261401575565838149305485737102400501329139562227180277188790091853682"),
    ],
    [
        Fp!("2939684373453383817196521641512509179310654199629514917426341354023324109367"),
        Fp!("1076484609897998179434851570277297233169621096172424141759873688902355505136"),
        Fp!("2575095284833160494841112025725243274091830284746697961080467506739203605049"),
    ],
    [
        Fp!("3565075264617591783581665711620369529657840830498005563542124551465195621851"),
        Fp!("2197016502533303822395077038351174326125210255869204501838837289716363437993"),
        Fp!("331415322883530754594261416546036195982886300052707474899691116664327869405"),
    ],
    [
        Fp!("1935011233711290003793244296594669823169522055520303479680359990463281661839"),
        Fp!("3495901467168087413996941216661589517270845976538454329511167073314577412322"),
        Fp!("954195417117133246453562983448451025087661597543338750600301835944144520375"),
    ],
    [
        Fp!("1271840477709992894995746871435810599280944810893784031132923384456797925777"),
        Fp!("2565310762274337662754531859505158700827688964841878141121196528015826671847"),
        Fp!("3365022288251637014588279139038152521653896670895105540140002607272936852513"),
    ],
    [
        Fp!("1660592021628965529963974299647026602622092163312666588591285654477111176051"),
        Fp!("970104372286014048279296575474974982288801187216974504035759997141059513421"),
        Fp!("2617024574317953753849168721871770134225690844968986289121504184985993971227"),
    ],
    [
        Fp!("999899815343607746071464113462778273556695659506865124478430189024755832262"),
        Fp!("2228536129413411161615629030408828764980855956560026807518714080003644769896"),
        Fp!("2701953891198001564547196795777701119629537795442025393867364730330476403227"),
    ],
    [
        Fp!("837078355588159388741598313782044128527494922918203556465116291436461597853"),
        Fp!("2121749601840466143704862369657561429793951309962582099604848281796392359214"),
        Fp!("771812260179247428733132708063116523892339056677915387749121983038690154755"),
    ],
    [
        Fp!("3317336423132806446086732225036532603224267214833263122557471741829060578219"),
        Fp!("481570067997721834712647566896657604857788523050900222145547508314620762046"),
        Fp!("242195042559343964206291740270858862066153636168162642380846129622127460192"),
    ],
    [
        Fp!("2855462178889999218204481481614105202770810647859867354506557827319138379686"),
        Fp!("3525521107148375040131784770413887305850308357895464453970651672160034885202"),
        Fp!("1320839531502392535964065058804908871811967681250362364246430459003920305799"),
    ],
    [
        Fp!("2514191518588387125173345107242226637171897291221681115249521904869763202419"),
        Fp!("2798335750958827619666318316247381695117827718387653874070218127140615157902"),
        Fp!("2808467767967035643407948058486565877867906577474361783201337540214875566395"),
    ],
    [
        Fp!("3551834385992706206273955480294669176699286104229279436819137165202231595747"),
        Fp!("1219439673853113792340300173186247996249367102884530407862469123523013083971"),
        Fp!("761519904537984520554247997444508040636526566551719396202550009393012691157"),
    ],
    [
        Fp!("3355402549169351700500518865338783382387571349497391475317206324155237401353"),
        Fp!("199541098009731541347317515995192175813554789571447733944970283654592727138"),
        Fp!("192100490643078165121235261796864975568292640203635147901612231594408079071"),
    ],
    [
        Fp!("1187019357602953326192019968809486933768550466167033084944727938441427050581"),
        Fp!("189525349641911362389041124808934468936759383310282010671081989585219065700"),
        Fp!("2831653363992091308880573627558515686245403755586311978724025292003353336665"),
    ],
    [
        Fp!("2052859812632218952608271535089179639890275494426396974475479657192657094698"),
        Fp!("1670756178709659908159049531058853320846231785448204274277900022176591811072"),
        Fp!("3538757242013734574731807289786598937548399719866320954894004830207085723125"),
    ],
    [
        Fp!("710549042741321081781917034337800036872214466705318638023070812391485261299"),
        Fp!("2345013122330545298606028187653996682275206910242635100920038943391319595180"),
        Fp!("3528369671971445493932880023233332035122954362711876290904323783426765912206"),
    ],
    [
        Fp!("1167120829038120978297497195837406760848728897181138760506162680655977700764"),
        Fp!("3073243357129146594530765548901087443775563058893907738967898816092270628884"),
        Fp!("378514724418106317738164464176041649567501099164061863402473942795977719726"),
    ],
    [
        Fp!("333391138410406330127594722511180398159664250722328578952158227406762627796"),
        Fp!("1727570175639917398410201375510924114487348765559913502662122372848626931905"),
        Fp!("968312190621809249603425066974405725769739606059422769908547372904403793174"),
    ],
    [
        Fp!("360659316299446405855194688051178331671817370423873014757323462844775818348"),
        Fp!("1386580151907705298970465943238806620109618995410132218037375811184684929291"),
        Fp!("3604888328937389309031638299660239238400230206645344173700074923133890528967"),
    ],
    [
        Fp!("2496185632263372962152518155651824899299616724241852816983268163379540137546"),
        Fp!("486538168871046887467737983064272608432052269868418721234810979756540672990"),
        Fp!("1558415498960552213241704009433360128041672577274390114589014204605400783336"),
    ],
    [
        Fp!("3512058327686147326577190314835092911156317204978509183234511559551181053926"),
        Fp!("2235429387083113882635494090887463486491842634403047716936833563914243946191"),
        Fp!("1290896777143878193192832813769470418518651727840187056683408155503813799882"),
    ],
    [
        Fp!("1143310336918357319571079551779316654556781203013096026972411429993634080835"),
        Fp!("3235435208525081966062419599803346573407862428113723170955762956243193422118"),
        Fp!("1293239921425673430660897025143433077974838969258268884994339615096356996604"),
    ],
    [
        Fp!("236252269127612784685426260840574970698541177557674806964960352572864382971"),
        Fp!("1733907592497266237374827232200506798207318263912423249709509725341212026275"),
        Fp!("302004309771755665128395814807589350526779835595021835389022325987048089868"),
    ],
    [
        Fp!("3018926838139221755384801385583867283206879023218491758435446265703006270945"),
        Fp!("39701437664873825906031098349904330565195980985885489447836580931425171297"),
        Fp!("908381723021746969965674308809436059628307487140174335882627549095646509778"),
    ],
    [
        Fp!("219062858908229855064136253265968615354041842047384625689776811853821594358"),
        Fp!("1283129863776453589317845316917890202859466483456216900835390291449830275503"),
        Fp!("418512623547417594896140369190919231877873410935689672661226540908900544012"),
    ],
    [
        Fp!("1792181590047131972851015200157890246436013346535432437041535789841136268632"),
        Fp!("370546432987510607338044736824316856592558876687225326692366316978098770516"),
        Fp!("3323437805230586112013581113386626899534419826098235300155664022709435756946"),
    ],
    [
        Fp!("910076621742039763058481476739499965761942516177975130656340375573185415877"),
        Fp!("1762188042455633427137702520675816545396284185254002959309669405982213803405"),
        Fp!("2186362253913140345102191078329764107619534641234549431429008219905315900520"),
    ],
    [
        Fp!("2230647725927681765419218738218528849146504088716182944327179019215826045083"),
        Fp!("1069243907556644434301190076451112491469636357133398376850435321160857761825"),
        Fp!("2695241469149243992683268025359863087303400907336026926662328156934068747593"),
    ],
    [
        Fp!("1361519681544413849831669554199151294308350560528931040264950307931824877035"),
        Fp!("1339116632207878730171031743761550901312154740800549632983325427035029084904"),
        Fp!("790593524918851401449292693473498591068920069246127392274811084156907468875"),
    ],
    [
        Fp!("2723400368331924254840192318398326090089058735091724263333980290765736363637"),
        Fp!("3457180265095920471443772463283225391927927225993685928066766687141729456030"),
        Fp!("1483675376954327086153452545475557749815683871577400883707749788555424847954"),
    ],
    [
        Fp!("2926303836265506736227240325795090239680154099205721426928300056982414025239"),
        Fp!("543969119775473768170832347411484329362572550684421616624136244239799475526"),
        Fp!("237401230683847084256617415614300816373730178313253487575312839074042461932"),
    ],
    [
        Fp!("844568412840391587862072008674263874021460074878949862892685736454654414423"),
        Fp!("151922054871708336050647150237534498235916969120198637893731715254687336644"),
        Fp!("1299332034710622815055321547569101119597030148120309411086203580212105652312"),
    ],
    [
        Fp!("487046922649899823989594814663418784068895385009696501386459462815688122993"),
        Fp!("1104883249092599185744249485896585912845784382683240114120846423960548576851"),
        Fp!("1458388705536282069567179348797334876446380557083422364875248475157495514484"),
    ],
    [
        Fp!("850248109622750774031817200193861444623975329881731864752464222442574976566"),
        Fp!("2885843173858536690032695698009109793537724845140477446409245651176355435722"),
        Fp!("3027068551635372249579348422266406787688980506275086097330568993357835463816"),
    ],
    [
        Fp!("3231892723647447539926175383213338123506134054432701323145045438168976970994"),
        Fp!("1719080830641935421242626784132692936776388194122314954558418655725251172826"),
        Fp!("1172253756541066126131022537343350498482225068791630219494878195815226839450"),
    ],
    [
        Fp!("1619232269633026603732619978083169293258272967781186544174521481891163985093"),
        Fp!("3495680684841853175973173610562400042003100419811771341346135531754869014567"),
        Fp!("1576161515913099892951745452471618612307857113799539794680346855318958552758"),
    ],
    [
        Fp!("2618326122974253423403350731396350223238201817594761152626832144510903048529"),
        Fp!("2696245132758436974032479782852265185094623165224532063951287925001108567649"),
        Fp!("930116505665110070247395429730201844026054810856263733273443066419816003444"),
    ],
    [
        Fp!("2786389174502246248523918824488629229455088716707062764363111940462137404076"),
        Fp!("1555260846425735320214671887347115247546042526197895180675436886484523605116"),
        Fp!("2306241912153325247392671742757902161446877415586158295423293240351799505917"),
    ],
    [
        Fp!("411529621724849932999694270803131456243889635467661223241617477462914950626"),
        Fp!("1542495485262286701469125140275904136434075186064076910329015697714211835205"),
        Fp!("1853045663799041100600825096887578544265580718909350942241802897995488264551"),
    ],
    [
        Fp!("2963055259497271220202739837493041799968576111953080503132045092194513937286"),
        Fp!("2303806870349915764285872605046527036748108533406243381676768310692344456050"),
        Fp!("2622104986201990620910286730213140904984256464479840856728424375142929278875"),
    ],
    [
        Fp!("2369987021925266811581727383184031736927816625797282287927222602539037105864"),
        Fp!("285070227712021899602056480426671736057274017903028992288878116056674401781"),
        Fp!("3034087076179360957800568733595959058628497428787907887933697691951454610691"),
    ],
    [
        Fp!("469095854351700119980323115747590868855368701825706298740201488006320881056"),
        Fp!("360001976264385426746283365024817520563236378289230404095383746911725100012"),
        Fp!("3438709327109021347267562000879503009590697221730578667498351600602230296178"),
    ],
    [
        Fp!("63573904800572228121671659287593650438456772568903228287754075619928214969"),
        Fp!("3470881855042989871434874691030920672110111605547839662680968354703074556970"),
        Fp!("724559311507950497340993415408274803001166693839947519425501269424891465492"),
    ],
    [
        Fp!("880409284677518997550768549487344416321062350742831373397603704465823658986"),
        Fp!("6876255662475867703077362872097208259197756317287339941435193538565586230"),
        Fp!("2701916445133770775447884812906226786217969545216086200932273680400909154638"),
    ],
    [
        Fp!("425152119158711585559310064242720816611629181537672850898056934507216982586"),
        Fp!("1475552998258917706756737045704649573088377604240716286977690565239187213744"),
        Fp!("2413772448122400684309006716414417978370152271397082147158000439863002593561"),
    ],
    [
        Fp!("392160855822256520519339260245328807036619920858503984710539815951012864164"),
        Fp!("1075036996503791536261050742318169965707018400307026402939804424927087093987"),
        Fp!("2176439430328703902070742432016450246365760303014562857296722712989275658921"),
    ],
    [
        Fp!("1413865976587623331051814207977382826721471106513581745229680113383908569693"),
        Fp!("4879283427490523253696177116563427032332223531862961281430108575019551814"),
        Fp!("3392583297537374046875199552977614390492290683707960975137418536812266544902"),
    ],
    [
        Fp!("3600854486849487646325182927019642276644093512133907046667282144129939150983"),
        Fp!("2779924664161372134024229593301361846129279572186444474616319283535189797834"),
        Fp!("2722699960903170449291146429799738181514821447014433304730310678334403972040"),
    ],
    [
        Fp!("819109815049226540285781191874507704729062681836086010078910930707209464699"),
        Fp!("3046121243742768013822760785918001632929744274211027071381357122228091333823"),
        Fp!("1339019590803056172509793134119156250729668216522001157582155155947567682278"),
    ],
    [
        Fp!("1933279639657506214789316403763326578443023901555983256955812717638093967201"),
        Fp!("2138221547112520744699126051903811860205771600821672121643894708182292213541"),
        Fp!("2694713515543641924097704224170357995809887124438248292930846280951601597065"),
    ],
    [
        Fp!("2471734202930133750093618989223585244499567111661178960753938272334153710615"),
        Fp!("504903761112092757611047718215309856203214372330635774577409639907729993533"),
        Fp!("1943979703748281357156510253941035712048221353507135074336243405478613241290"),
    ],
    [
        Fp!("684525210957572142559049112233609445802004614280157992196913315652663518936"),
        Fp!("1705585400798782397786453706717059483604368413512485532079242223503960814508"),
        Fp!("192429517716023021556170942988476050278432319516032402725586427701913624665"),
    ],
    [
        Fp!("1586493702243128040549584165333371192888583026298039652930372758731750166765"),
        Fp!("686072673323546915014972146032384917012218151266600268450347114036285993377"),
        Fp!("3464340397998075738891129996710075228740496767934137465519455338004332839215"),
    ],
    [
        Fp!("2805249176617071054530589390406083958753103601524808155663551392362371834663"),
        Fp!("667746464250968521164727418691487653339733392025160477655836902744186489526"),
        Fp!("1131527712905109997177270289411406385352032457456054589588342450404257139778"),
    ],
    [
        Fp!("1908969485750011212309284349900149072003218505891252313183123635318886241171"),
        Fp!("1025257076985551890132050019084873267454083056307650830147063480409707787695"),
        Fp!("2153175291918371429502545470578981828372846236838301412119329786849737957977"),
    ],
    [
        Fp!("3410257749736714576487217882785226905621212230027780855361670645857085424384"),
        Fp!("3442969106887588154491488961893254739289120695377621434680934888062399029952"),
        Fp!("3029953900235731770255937704976720759948880815387104275525268727341390470237"),
    ],
    [
        Fp!("85453456084781138713939104192561924536933417707871501802199311333127894466"),
        Fp!("2730629666577257820220329078741301754580009106438115341296453318350676425129"),
        Fp!("178242450661072967256438102630920745430303027840919213764087927763335940415"),
    ],
    [
        Fp!("2844589222514708695700541363167856718216388819406388706818431442998498677557"),
        Fp!("3547876269219141094308889387292091231377253967587961309624916269569559952944"),
        Fp!("2525005406762984211707203144785482908331876505006839217175334833739957826850"),
    ],
    [
        Fp!("3096397013555211396701910432830904669391580557191845136003938801598654871345"),
        Fp!("574424067119200181933992948252007230348512600107123873197603373898923821490"),
        Fp!("1714030696055067278349157346067719307863507310709155690164546226450579547098"),
    ],
    [
        Fp!("2339895272202694698739231405357972261413383527237194045718815176814132612501"),
        Fp!("3562501318971895161271663840954705079797767042115717360959659475564651685069"),
        Fp!("69069358687197963617161747606993436483967992689488259107924379545671193749"),
    ],
    [
        Fp!("2614502738369008850475068874731531583863538486212691941619835266611116051561"),
        Fp!("655247349763023251625727726218660142895322325659927266813592114640858573566"),
        Fp!("2305235672527595714255517865498269719545193172975330668070873705108690670678"),
    ],
    [
        Fp!("926416070297755413261159098243058134401665060349723804040714357642180531931"),
        Fp!("866523735635840246543516964237513287099659681479228450791071595433217821460"),
        Fp!("2284334068466681424919271582037156124891004191915573957556691163266198707693"),
    ],
    [
        Fp!("1812588309302477291425732810913354633465435706480768615104211305579383928792"),
        Fp!("2836899808619013605432050476764608707770404125005720004551836441247917488507"),
        Fp!("2989087789022865112405242078196235025698647423649950459911546051695688370523"),
    ],
    [
        Fp!("68056284404189102136488263779598243992465747932368669388126367131855404486"),
        Fp!("505425339250887519581119854377342241317528319745596963584548343662758204398"),
        Fp!("2118963546856545068961709089296976921067035227488975882615462246481055679215"),
    ],
    [
        Fp!("2253872596319969096156004495313034590996995209785432485705134570745135149681"),
        Fp!("1625090409149943603241183848936692198923183279116014478406452426158572703264"),
        Fp!("179139838844452470348634657368199622305888473747024389514258107503778442495"),
    ],
    [
        Fp!("1567067018147735642071130442904093290030432522257811793540290101391210410341"),
        Fp!("2737301854006865242314806979738760349397411136469975337509958305470398783585"),
        Fp!("3002738216460904473515791428798860225499078134627026021350799206894618186256"),
    ],
    [
        Fp!("374029488099466837453096950537275565120689146401077127482884887409712315162"),
        Fp!("973403256517481077805460710540468856199855789930951602150773500862180885363"),
        Fp!("2691967457038172130555117632010860984519926022632800605713473799739632878867"),
    ],
];

pub const ROUND_KEYS: [[Fp; 3]; NUM_FULL_ROUNDS + NUM_PARTIAL_ROUNDS] = {
    let mut res = [[Fp::ZERO; 3]; NUM_FULL_ROUNDS + NUM_PARTIAL_ROUNDS];

    let mut round = 0;

    // load in full rounds keys (1st half)
    let mut i = 0;
    while i < NUM_FULL_ROUNDS / 2 {
        res[round] = FULL_ROUND_KEYS_1ST_HALF[i];
        round += 1;
        i += 1;
    }

    // load in partial rounds keys
    let mut i = 0;
    while i < NUM_PARTIAL_ROUNDS {
        res[round] = PARTIAL_ROUND_KEYS[i];
        round += 1;
        i += 1;
    }

    // load in full rounds keys (2nd half)
    let mut i = 0;
    while i < NUM_FULL_ROUNDS / 2 {
        res[round] = FULL_ROUND_KEYS_2ND_HALF[i];
        round += 1;
        i += 1;
    }

    res
};

/// TODO: understand the optimized poseidon more
/// TODO: AFAIK this is to do with section B of the poseidon paper
/// Source: <https://github.com/CryptoExperts/poseidon/blob/main/sage/poseidon_variant.sage#L158>
pub const PARTIAL_ROUND_KEYS_OPTIMIZED: [Fp; NUM_PARTIAL_ROUNDS] = [
    Fp!("2121140748740143694053732746913428481442990369183417228688865837805149503386"),
    Fp!("3254199758946794255547849145804552040191238405486150107506335017016791887254"),
    Fp!("946702499093737433653859753350563194535167982146779656201695802468403244714"),
    Fp!("22219251323453881656920813138471378292478852052854377954733532508863958126"),
    Fp!("2337489154008330275378504550238520231652045615496167167049463825691779314366"),
    Fp!("1167037765977224186483394258663932763764678232927208577549062146321043467125"),
    Fp!("500636100858491656460564651839613972261145307252436547234215140171677458532"),
    Fp!("2128825297978446892994756114680560837820177419346509015674459492689885449578"),
    Fp!("167719178494076614114377700814889587706376623182032126125283499313286486724"),
    Fp!("2941357722192004021095782161863208304275411959785674070504940037122017431641"),
    Fp!("821139144894338912575785190371549715123905148183007403898484854841364992083"),
    Fp!("1248779758829817786166052038439209181330580364802890969358765414729785734244"),
    Fp!("1506986792917538912200792369514079124928449213387824336705492413628080432482"),
    Fp!("1381578206447887317344495828113990646031246967777877864867202481221584821845"),
    Fp!("2333668151468814131335486070028088155534431787713915985000600576822072862584"),
    Fp!("1816477153678973333700131761203104005001621420144804027708421967328218782048"),
    Fp!("99030127382357949859446761826779882035470382792486336777793453313319627320"),
    Fp!("813297404092921099745668004485430051216419749355318107277765202583270808437"),
    Fp!("2463930986097532200876517071633166744907933587477991898439520493266890954578"),
    Fp!("423520937513583012741735534915286768255016893446586399236864431677291731617"),
    Fp!("908252263617839603970395806639645423078889517061114964305086611745646955161"),
    Fp!("3533162957039156162503622719939369032897298085276549604536612435905780159111"),
    Fp!("1987911960223769209504917927492316115736343134017835993954600726832746427881"),
    Fp!("380687595226251796928193989788312178750297832654872550746038451151134842055"),
    Fp!("723836106711573195482752463222284837305525396531378358686111899267045035688"),
    Fp!("1949186998620783242170083311790699047271689269680028707268183140167354023534"),
    Fp!("3570231800456752607220971892191312212606484323780228143874595723085978119159"),
    Fp!("1533841673689181946594779956497065326837083571757064880081445329845793975952"),
    Fp!("1749780176677017454688058184001431732734295557956462485446465335896177587507"),
    Fp!("2782021729458502807102203171348415841988172892345299056612457249042796562836"),
    Fp!("2884469171981894881540530726109989671058863772398420239318194945977497591998"),
    Fp!("502865423705112461415478258998711804268834029736933168660919307935283389206"),
    Fp!("2767128958007328376619289477242953690382839868910105295896878592981339613911"),
    Fp!("1851965269056002803870754040465990009048457313315344119550048118329521192392"),
    Fp!("2319859312032543508070087341520588798143043747003274627732227175700760546161"),
    Fp!("1481851172635764396648365312410893899768237823172771806594092674585342554017"),
    Fp!("547036975458952633246987998601847237422209340538022088453240608323312019151"),
    Fp!("2029040076978321975583451791425842570825289495712697408276393779094800406133"),
    Fp!("2111666052120725475009694903084767583512396896241364439582898786636757483944"),
    Fp!("1638313159675579284798742164391382322581933490187820706133272095667924642245"),
    Fp!("1943334517983487712604857408487292437235900060448039792386021465478286499922"),
    Fp!("2279439709686910055313517626578662663069422685955948285215850306440646185867"),
    Fp!("2613744865202132600521251288482622278516262684072772393918143577136626238379"),
    Fp!("1271046499203308984245936639421115668458896858240120402572600677467133334028"),
    Fp!("1247402822240089277812837489004444618314281250104731857922853833706905025033"),
    Fp!("1723958042307430047369975214277995039902630801406033526846688500864468834465"),
    Fp!("1533584905443657050354436771709571562292231931283916242463173889321434022553"),
    Fp!("3237040419489511449610640535311097631523703826130788507531451982277420015096"),
    Fp!("1805688920160940208305338731961140654373846204702392898819566332928773251105"),
    Fp!("3594217041681714937541668807710257522867050846770592516586136077930203182153"),
    Fp!("2577667437062141583866181613527841955533043304602050579503127691238410706699"),
    Fp!("298360685626170262043744752141398618330583734776737906162831308609720345150"),
    Fp!("1281197941663063015352952771329377076126324458271281919046044364819943591777"),
    Fp!("3077378676982681080623829657020596666204600488414707435106417676235575429862"),
    Fp!("1748451361476916587571057334543442824224179571208744687141910117058056395763"),
    Fp!("2676739626683060991468050581747570829182189232279704806260935727663779579868"),
    Fp!("2111406761959185880649453307787229397571964702420106577260074179491820341222"),
    Fp!("2166938499858821084516582100808426278072281425656820667434712211230541410848"),
    Fp!("3107730607835556247617842981413401192521001707456266270689094704861620026671"),
    Fp!("2515316456140241497324967413301163552693702198011789683490168850163933049222"),
    Fp!("2027424506634793494100687247002514545014800857224413261907682804838189168178"),
    Fp!("686611978160956747476540594855185145758616845149919521842660587456792707852"),
    Fp!("1852435510043218004936376823742181821269846802379038978692192781198078457548"),
    Fp!("973093133228175997253426609822216788300580654709609240022375781254619695879"),
    Fp!("877967322807568496624711159265026109563061956880454131565483776614153765115"),
    Fp!("281272373020914342887109639781082339308146271877896207627957708090436162915"),
    Fp!("2127372253436081931004122019516254627871961358111209455604779897472993187794"),
    Fp!("709209773465173719688709113929876004342449434606403229952473163671243033503"),
    Fp!("699734237641622314765330412575760189293247876990116441757311413522367149072"),
    Fp!("1518385199739191600262893710402302262632783930361077392560755658518956984323"),
    Fp!("760806111534053979632457606818127694614027577816707025268112145784633577941"),
    Fp!("1764871896938929024636668407642670740592076939499011813111918260363201766523"),
    Fp!("699253396683419051870723559228437425690024464550961997647242273284382374131"),
    Fp!("1257359693980757201384422893362124203205229468320860619748365802142472987885"),
    Fp!("2364896125328894023157389470973536908156590393711231968298065243872757173207"),
    Fp!("2182967142037621407059035107654706795605016088217146929228750866491415252649"),
    Fp!("1990321497039705801432309385212088530141372102903323553272287975787775587789"),
    Fp!("79126479487832908984311442127162955688326570002976886475288267750665196063"),
    Fp!("3030466492259175286745846236737838233115764962471209462998669600396874325088"),
    Fp!("1768666173325223797893467185524333800553517622040181630592523089202392400547"),
    Fp!("1787421318415185832976863053538828653270487864094131451867390483344614213354"),
    Fp!("585330999921339551612560202913644389398045408137860113737866261031794825461"),
    Fp!("2615206611404365180082501914128343648505511220446326785601565995140319286511"),
];
