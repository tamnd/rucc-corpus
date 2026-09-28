# frame-size

large frames with a simple answer, to hold against gcc -fstack-usage. Part of the backend phase of the M4 plan.

54 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`frame-size.32.0.0.calls.c17.6463c5ab`](frame-size.32.0.0.calls.c17.6463c5ab.c) | locals=32, array=0, structs=0, calls=calls | c17 | `745566455957951633 9793597966685362676 394885403703222103` |
| [`frame-size.32.0.0.leaf.c17.eec4296e`](frame-size.32.0.0.leaf.c17.eec4296e.c) | locals=32, array=0, structs=0, calls=leaf | c17 | `745566455957951633 9793597966685362676 394885403703222103` |
| [`frame-size.32.0.0.recursive.c17.9f29e71b`](frame-size.32.0.0.recursive.c17.9f29e71b.c) | locals=32, array=0, structs=0, calls=recursive | c17 | `17544215089135596182 13042404743448559650 8540594397761523118` |
| [`frame-size.32.0.4.calls.c17.fb12d74f`](frame-size.32.0.4.calls.c17.fb12d74f.c) | locals=32, array=0, structs=4, calls=calls | c17 | `7177815538058775743 6897364899668909658 6616914261279043573` |
| [`frame-size.32.0.4.leaf.c17.d517dca3`](frame-size.32.0.4.leaf.c17.d517dca3.c) | locals=32, array=0, structs=4, calls=leaf | c17 | `7177815538058775743 6897364899668909658 6616914261279043573` |
| [`frame-size.32.0.4.recursive.c17.e6ef12e2`](frame-size.32.0.4.recursive.c17.e6ef12e2.c) | locals=32, array=0, structs=4, calls=recursive | c17 | `18004756046062187166 9958783140959817738 1912810235857448310` |
| [`frame-size.32.16384.0.calls.c17.6a878ce6`](frame-size.32.16384.0.calls.c17.6a878ce6.c) | locals=32, array=16384, structs=0, calls=calls | c17 | `10286733058261425472 7364071826992407205 4441410595723388938` |
| [`frame-size.32.16384.0.leaf.c17.5a0836fd`](frame-size.32.16384.0.leaf.c17.5a0836fd.c) | locals=32, array=16384, structs=0, calls=leaf | c17 | `10286733058261425472 7364071826992407205 4441410595723388938` |
| [`frame-size.32.16384.0.recursive.c17.cc38c3c8`](frame-size.32.16384.0.recursive.c17.cc38c3c8.c) | locals=32, array=16384, structs=0, calls=recursive | c17 | `14605878548905459038 355367555857586930 4551600636519266438` |
| [`frame-size.32.16384.4.calls.c17.0d580825`](frame-size.32.16384.4.calls.c17.0d580825.c) | locals=32, array=16384, structs=4, calls=calls | c17 | `211108529292076526 2862482961921533579 5513857394550990632` |
| [`frame-size.32.16384.4.leaf.c17.94bc25bd`](frame-size.32.16384.4.leaf.c17.94bc25bd.c) | locals=32, array=16384, structs=4, calls=leaf | c17 | `211108529292076526 2862482961921533579 5513857394550990632` |
| [`frame-size.32.16384.4.recursive.c17.b5246490`](frame-size.32.16384.4.recursive.c17.b5246490.c) | locals=32, array=16384, structs=4, calls=recursive | c17 | `14153985134923419238 957015107617444314 6206789154021021006` |
| [`frame-size.32.256.0.calls.c17.46a8075e`](frame-size.32.256.0.calls.c17.46a8075e.c) | locals=32, array=256, structs=0, calls=calls | c17 | `3235119301884235072 8781508475764893093 14327897649645551114` |
| [`frame-size.32.256.0.leaf.c17.5faf4cba`](frame-size.32.256.0.leaf.c17.5faf4cba.c) | locals=32, array=256, structs=0, calls=leaf | c17 | `3235119301884235072 8781508475764893093 14327897649645551114` |
| [`frame-size.32.256.0.recursive.c17.179cbc6d`](frame-size.32.256.0.recursive.c17.179cbc6d.c) | locals=32, array=256, structs=0, calls=recursive | c17 | `1303485669281181534 11022921358625118450 2295612974259503750` |
| [`frame-size.32.256.4.calls.c17.dc31af8a`](frame-size.32.256.4.calls.c17.dc31af8a.c) | locals=32, array=256, structs=4, calls=calls | c17 | `14670523287881881070 17520835399659530635 1924403437727628584` |
| [`frame-size.32.256.4.leaf.c17.e881e70a`](frame-size.32.256.4.leaf.c17.e881e70a.c) | locals=32, array=256, structs=4, calls=leaf | c17 | `14670523287881881070 17520835399659530635 1924403437727628584` |
| [`frame-size.32.256.4.recursive.c17.f62151d1`](frame-size.32.256.4.recursive.c17.f62151d1.c) | locals=32, array=256, structs=4, calls=recursive | c17 | `8101870703100006 7723436081431891930 15438770292160683854` |
| [`frame-size.8.0.0.calls.c17.1c5916a3`](frame-size.8.0.0.calls.c17.1c5916a3.c) | locals=8, array=0, structs=0, calls=calls | c17 | `14294790576230715825 15244664867993328396 16194539159755940967` |
| [`frame-size.8.0.0.leaf.c17.084d6775`](frame-size.8.0.0.leaf.c17.084d6775.c) | locals=8, array=0, structs=0, calls=leaf | c17 | `14294790576230715825 15244664867993328396 16194539159755940967` |
| [`frame-size.8.0.0.recursive.c17.79f8ebe6`](frame-size.8.0.0.recursive.c17.79f8ebe6.c) | locals=8, array=0, structs=0, calls=recursive | c17 | `12294314515170516710 17036717953327481426 3332377317774894526` |
| [`frame-size.8.0.4.calls.c17.fc5dccb7`](frame-size.8.0.4.calls.c17.fc5dccb7.c) | locals=8, array=0, structs=4, calls=calls | c17 | `16192038721824925663 8916512363528364402 1640986005231803141` |
| [`frame-size.8.0.4.leaf.c17.f2b69523`](frame-size.8.0.4.leaf.c17.f2b69523.c) | locals=8, array=0, structs=4, calls=leaf | c17 | `16192038721824925663 8916512363528364402 1640986005231803141` |
| [`frame-size.8.0.4.recursive.c17.a683ef59`](frame-size.8.0.4.recursive.c17.a683ef59.c) | locals=8, array=0, structs=4, calls=recursive | c17 | `632521784743817966 273982260998848058 18362186810963429766` |
| [`frame-size.8.16384.0.calls.c17.b2e23f40`](frame-size.8.16384.0.calls.c17.b2e23f40.c) | locals=8, array=16384, structs=0, calls=calls | c17 | `5627256871788468832 1859992101732869053 16539471405386820890` |
| [`frame-size.8.16384.0.leaf.c17.0892709f`](frame-size.8.16384.0.leaf.c17.0892709f.c) | locals=8, array=16384, structs=0, calls=leaf | c17 | `5627256871788468832 1859992101732869053 16539471405386820890` |
| [`frame-size.8.16384.0.recursive.c17.f10c7c2e`](frame-size.8.16384.0.recursive.c17.f10c7c2e.c) | locals=8, array=16384, structs=0, calls=recursive | c17 | `11918958673968163758 345707603754418978 7219200607250225814` |
| [`frame-size.8.16384.4.calls.c17.35e3c0fd`](frame-size.8.16384.4.calls.c17.35e3c0fd.c) | locals=8, array=16384, structs=4, calls=calls | c17 | `17793915520301089038 16615326992426477987 15436738464551866936` |
| [`frame-size.8.16384.4.leaf.c17.ff007b32`](frame-size.8.16384.4.leaf.c17.ff007b32.c) | locals=8, array=16384, structs=4, calls=leaf | c17 | `17793915520301089038 16615326992426477987 15436738464551866936` |
| [`frame-size.8.16384.4.recursive.c17.670143fd`](frame-size.8.16384.4.recursive.c17.670143fd.c) | locals=8, array=16384, structs=4, calls=recursive | c17 | `16423959872060018870 303768801085335050 2630321803820202846` |
| [`frame-size.8.256.0.calls.c17.0fd2f735`](frame-size.8.256.0.calls.c17.0fd2f735.c) | locals=8, array=256, structs=0, calls=calls | c17 | `10541038721996720736 15349494482653062845 1711206169599853338` |
| [`frame-size.8.256.0.leaf.c17.b38ec1c8`](frame-size.8.256.0.leaf.c17.b38ec1c8.c) | locals=8, array=256, structs=0, calls=leaf | c17 | `10541038721996720736 15349494482653062845 1711206169599853338` |
| [`frame-size.8.256.0.recursive.c17.aaa57773`](frame-size.8.256.0.recursive.c17.aaa57773.c) | locals=8, array=256, structs=0, calls=recursive | c17 | `3024358552295710126 2770674995190202658 2516991438084695190` |
| [`frame-size.8.256.4.calls.c17.d540a488`](frame-size.8.256.4.calls.c17.d540a488.c) | locals=8, array=256, structs=4, calls=calls | c17 | `7246148777062428942 12754173708419558563 18262198639776688184` |
| [`frame-size.8.256.4.leaf.c17.e48fcb91`](frame-size.8.256.4.leaf.c17.e48fcb91.c) | locals=8, array=256, structs=4, calls=leaf | c17 | `7246148777062428942 12754173708419558563 18262198639776688184` |
| [`frame-size.8.256.4.recursive.c17.02cd7138`](frame-size.8.256.4.recursive.c17.02cd7138.c) | locals=8, array=256, structs=4, calls=recursive | c17 | `10624635573159822006 11747104840446911498 12869574107734000990` |
| [`frame-size.96.0.0.calls.c17.20d91518`](frame-size.96.0.0.calls.c17.20d91518.c) | locals=96, array=0, structs=0, calls=calls | c17 | `679976831999441553 17708332935389203124 16289944965069413079` |
| [`frame-size.96.0.0.leaf.c17.1be4b8ee`](frame-size.96.0.0.leaf.c17.1be4b8ee.c) | locals=96, array=0, structs=0, calls=leaf | c17 | `679976831999441553 17708332935389203124 16289944965069413079` |
| [`frame-size.96.0.0.recursive.c17.b7b16d1f`](frame-size.96.0.0.recursive.c17.b7b16d1f.c) | locals=96, array=0, structs=0, calls=recursive | c17 | `2218240954345130774 3671841702888010658 5125442451430890542` |
| [`frame-size.96.0.4.calls.c17.51d6c0fa`](frame-size.96.0.4.calls.c17.51d6c0fa.c) | locals=96, array=0, structs=4, calls=calls | c17 | `10235796923498717887 6476239967413903130 2716683011329088373` |
| [`frame-size.96.0.4.leaf.c17.583f24c4`](frame-size.96.0.4.leaf.c17.583f24c4.c) | locals=96, array=0, structs=4, calls=leaf | c17 | `10235796923498717887 6476239967413903130 2716683011329088373` |
| [`frame-size.96.0.4.recursive.c17.c3929b06`](frame-size.96.0.4.recursive.c17.c3929b06.c) | locals=96, array=0, structs=4, calls=recursive | c17 | `16601906061824879390 7784817870164517770 17414473752213707766` |
| [`frame-size.96.16384.0.calls.c17.323161ef`](frame-size.96.16384.0.calls.c17.323161ef.c) | locals=96, array=16384, structs=0, calls=calls | c17 | `1454702560877282112 3185088868589015909 4915475176300749706` |
| [`frame-size.96.16384.0.leaf.c17.b844ce06`](frame-size.96.16384.0.leaf.c17.b844ce06.c) | locals=96, array=16384, structs=0, calls=leaf | c17 | `1454702560877282112 3185088868589015909 4915475176300749706` |
| [`frame-size.96.16384.0.recursive.c17.b0c205d8`](frame-size.96.16384.0.recursive.c17.b0c205d8.c) | locals=96, array=16384, structs=0, calls=recursive | c17 | `15881677385859507678 1667557916620619378 5900182521091282694` |
| [`frame-size.96.16384.4.calls.c17.edf4e695`](frame-size.96.16384.4.calls.c17.edf4e695.c) | locals=96, array=16384, structs=4, calls=calls | c17 | `10300013017020627950 2527954364439603019 13202639785568129704` |
| [`frame-size.96.16384.4.leaf.c17.1077555f`](frame-size.96.16384.4.leaf.c17.1077555f.c) | locals=96, array=16384, structs=4, calls=leaf | c17 | `10300013017020627950 2527954364439603019 13202639785568129704` |
| [`frame-size.96.16384.4.recursive.c17.8ef2d50b`](frame-size.96.16384.4.recursive.c17.8ef2d50b.c) | locals=96, array=16384, structs=4, calls=recursive | c17 | `10060097691983533798 2590526114468926810 13567698610663871438` |
| [`frame-size.96.256.0.calls.c17.eaea2e23`](frame-size.96.256.0.calls.c17.eaea2e23.c) | locals=96, array=256, structs=0, calls=calls | c17 | `6203870374042415936 2017283055365183077 16277439810397501834` |
| [`frame-size.96.256.0.leaf.c17.578979a3`](frame-size.96.256.0.leaf.c17.578979a3.c) | locals=96, array=256, structs=0, calls=leaf | c17 | `6203870374042415936 2017283055365183077 16277439810397501834` |
| [`frame-size.96.256.0.recursive.c17.084efa59`](frame-size.96.256.0.recursive.c17.084efa59.c) | locals=96, array=256, structs=0, calls=recursive | c17 | `9174448434829187038 5328907116185091186 1483365797540995334` |
| [`frame-size.96.256.4.calls.c17.497e484a`](frame-size.96.256.4.calls.c17.497e484a.c) | locals=96, array=256, structs=4, calls=calls | c17 | `13296797690454027246 16674533579763542603 1605525395363506344` |
| [`frame-size.96.256.4.leaf.c17.e3149a1e`](frame-size.96.256.4.leaf.c17.e3149a1e.c) | locals=96, array=256, structs=4, calls=leaf | c17 | `13296797690454027246 16674533579763542603 1605525395363506344` |
| [`frame-size.96.256.4.recursive.c17.9bb705c7`](frame-size.96.256.4.recursive.c17.9bb705c7.c) | locals=96, array=256, structs=4, calls=recursive | c17 | `9756951144952301798 4099870008897803098 16889532946552856014` |

