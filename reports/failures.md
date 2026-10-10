# What went wrong

176 findings, worst first. A finding is one case, one compiler, one level.

### `frame-size.32.0.0.recursive.c17.9f29e71b` at `O1` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 17544215089135596182\n13042404743448559650\n8540594397761523118
actual:   12817989783689332188\n12037310266167352651\n16787426633707461649
```

### `frame-size.32.0.0.recursive.c17.9f29e71b` at `Os` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 17544215089135596182\n13042404743448559650\n8540594397761523118
actual:   12817989783689332188\n12037310266167352651\n16787426633707461649
```

### `frame-size.32.0.4.recursive.c17.e6ef12e2` at `O1` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 18004756046062187166\n9958783140959817738\n1912810235857448310
actual:   114317629995517465\n18290657204083505103\n18020252704461941125
```

### `frame-size.32.0.4.recursive.c17.e6ef12e2` at `Os` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 18004756046062187166\n9958783140959817738\n1912810235857448310
actual:   114317629995517465\n18290657204083505103\n18020252704461941125
```

### `frame-size.32.16384.0.recursive.c17.cc38c3c8` at `O1` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 14605878548905459038\n355367555857586930\n4551600636519266438
actual:   12208367387447608303\n2496048629004182191\n17105885554550856057
```

### `frame-size.32.16384.0.recursive.c17.cc38c3c8` at `Os` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 14605878548905459038\n355367555857586930\n4551600636519266438
actual:   12208367387447608303\n2496048629004182191\n17105885554550856057
```

### `frame-size.32.16384.4.recursive.c17.b5246490` at `O1` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 14153985134923419238\n957015107617444314\n6206789154021021006
actual:   16315832771611730681\n8937481076744509875\n1559129381877289069
```

### `frame-size.32.16384.4.recursive.c17.b5246490` at `Os` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 14153985134923419238\n957015107617444314\n6206789154021021006
actual:   16315832771611730681\n8937481076744509875\n1559129381877289069
```

### `frame-size.32.256.0.recursive.c17.179cbc6d` at `O1` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 1303485669281181534\n11022921358625118450\n2295612974259503750
actual:   16057742581381467375\n5532991614815353775\n1950879230648326265
```

### `frame-size.32.256.0.recursive.c17.179cbc6d` at `Os` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 1303485669281181534\n11022921358625118450\n2295612974259503750
actual:   16057742581381467375\n5532991614815353775\n1950879230648326265
```

### `frame-size.32.256.4.recursive.c17.f62151d1` at `O1` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 8101870703100006\n7723436081431891930\n15438770292160683854
actual:   9367854141794588153\n267659735295455923\n9614209402505875309
```

### `frame-size.32.256.4.recursive.c17.f62151d1` at `Os` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 8101870703100006\n7723436081431891930\n15438770292160683854
actual:   9367854141794588153\n267659735295455923\n9614209402505875309
```

### `frame-size.8.0.4.recursive.c17.a683ef59` at `O1` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 632521784743817966\n273982260998848058\n18362186810963429766
actual:   9896143589705690705\n10404419519353576695\n10912695449001462685
```

### `frame-size.8.0.4.recursive.c17.a683ef59` at `Os` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 632521784743817966\n273982260998848058\n18362186810963429766
actual:   9896143589705690705\n10404419519353576695\n10912695449001462685
```

### `frame-size.8.16384.0.recursive.c17.f10c7c2e` at `O1` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 11918958673968163758\n345707603754418978\n7219200607250225814
actual:   2305861104240737580\n15075769856573468364\n12124698492521155087
```

### `frame-size.8.16384.0.recursive.c17.f10c7c2e` at `Os` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 11918958673968163758\n345707603754418978\n7219200607250225814
actual:   2305861104240737580\n15075769856573468364\n12124698492521155087
```

### `frame-size.8.16384.4.recursive.c17.670143fd` at `O1` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 16423959872060018870\n303768801085335050\n2630321803820202846
actual:   18192049486109150769\n14063656279224723419\n9935263072340296069
```

### `frame-size.8.16384.4.recursive.c17.670143fd` at `Os` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 16423959872060018870\n303768801085335050\n2630321803820202846
actual:   18192049486109150769\n14063656279224723419\n9935263072340296069
```

### `frame-size.8.256.0.recursive.c17.aaa57773` at `O1` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 3024358552295710126\n2770674995190202658\n2516991438084695190
actual:   3575651980501864748\n16412561827543995084\n14179085560825719567
```

### `frame-size.8.256.0.recursive.c17.aaa57773` at `Os` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 3024358552295710126\n2770674995190202658\n2516991438084695190
actual:   3575651980501864748\n16412561827543995084\n14179085560825719567
```

### `frame-size.8.256.4.recursive.c17.02cd7138` at `O1` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 10624635573159822006\n11747104840446911498\n12869574107734000990
actual:   5790465917139058993\n6721114599854817499\n7651763282570576005
```

### `frame-size.8.256.4.recursive.c17.02cd7138` at `Os` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 10624635573159822006\n11747104840446911498\n12869574107734000990
actual:   5790465917139058993\n6721114599854817499\n7651763282570576005
```

### `frame-size.96.0.0.recursive.c17.b7b16d1f` at `O1` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 2218240954345130774\n3671841702888010658\n5125442451430890542
actual:   7648506854538237253\n8717350164974408075\n9786193475410578897
```

### `frame-size.96.0.0.recursive.c17.b7b16d1f` at `Os` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 2218240954345130774\n3671841702888010658\n5125442451430890542
actual:   7648506854538237253\n8717350164974408075\n9786193475410578897
```

### `frame-size.96.0.4.recursive.c17.c3929b06` at `O1` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 16601906061824879390\n7784817870164517770\n17414473752213707766
actual:   12392023311882398937\n17533324470129818639\n4227881554667686725
```

### `frame-size.96.0.4.recursive.c17.c3929b06` at `Os` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 16601906061824879390\n7784817870164517770\n17414473752213707766
actual:   12392023311882398937\n17533324470129818639\n4227881554667686725
```

### `frame-size.96.16384.0.recursive.c17.b0c205d8` at `O1` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 15881677385859507678\n1667557916620619378\n5900182521091282694
actual:   14643018168355081775\n7200463496194211567\n10831763918390150925
```

### `frame-size.96.16384.0.recursive.c17.b0c205d8` at `Os` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 15881677385859507678\n1667557916620619378\n5900182521091282694
actual:   14643018168355081775\n7200463496194211567\n10831763918390150925
```

### `frame-size.96.16384.4.recursive.c17.8ef2d50b` at `O1` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 10060097691983533798\n2590526114468926810\n13567698610663871438
actual:   1308201026109292985\n5047732187305412083\n8787263348501531181
```

### `frame-size.96.16384.4.recursive.c17.8ef2d50b` at `Os` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 10060097691983533798\n2590526114468926810\n13567698610663871438
actual:   1308201026109292985\n5047732187305412083\n8787263348501531181
```

### `frame-size.96.256.0.recursive.c17.084efa59` at `O1` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 9174448434829187038\n5328907116185091186\n1483365797540995334
actual:   16031135575141127983\n1831728599637841903\n16390429875549831693
```

### `frame-size.96.256.0.recursive.c17.084efa59` at `Os` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 9174448434829187038\n5328907116185091186\n1483365797540995334
actual:   16031135575141127983\n1831728599637841903\n16390429875549831693
```

### `frame-size.96.256.4.recursive.c17.9bb705c7` at `O1` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 9756951144952301798\n4099870008897803098\n16889532946552856014
actual:   11555661745954632889\n9438312839369006835\n7320963932783380781
```

### `frame-size.96.256.4.recursive.c17.9bb705c7` at `Os` on `rucc`

rucc printed the wrong answer for a case about large frames with a simple answer, to hold against gcc -fstack-usage

Facet [`frame-size`](../programs/backend/frame-size/README.md).

```
expected: 9756951144952301798\n4099870008897803098\n16889532946552856014
actual:   11555661745954632889\n9438312839369006835\n7320963932783380781
```

### `null-pointer-constant.conditional-type.c17.b33903a7` at `O0` on `rucc`

rucc printed the wrong answer for a case about __is_constexpr, is_const and the null pointer constants they rest on

Facet [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md).

```
expected: 34503
actual:   238279
```

### `null-pointer-constant.conditional-type.c17.b33903a7` at `O1` on `rucc`

rucc printed the wrong answer for a case about __is_constexpr, is_const and the null pointer constants they rest on

Facet [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md).

```
expected: 34503
actual:   238279
```

### `null-pointer-constant.conditional-type.c17.b33903a7` at `O2` on `rucc`

rucc printed the wrong answer for a case about __is_constexpr, is_const and the null pointer constants they rest on

Facet [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md).

```
expected: 34503
actual:   238279
```

### `null-pointer-constant.conditional-type.c17.b33903a7` at `O3` on `rucc`

rucc printed the wrong answer for a case about __is_constexpr, is_const and the null pointer constants they rest on

Facet [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md).

```
expected: 34503
actual:   238279
```

### `null-pointer-constant.conditional-type.c17.b33903a7` at `Os` on `rucc`

rucc printed the wrong answer for a case about __is_constexpr, is_const and the null pointer constants they rest on

Facet [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md).

```
expected: 34503
actual:   238279
```

### `null-pointer-constant.is-const.c17.0749e5ff` at `O0` on `rucc`

rucc printed the wrong answer for a case about __is_constexpr, is_const and the null pointer constants they rest on

Facet [`null-pointer-constant`](../programs/correctness/null-pointer-constant/README.md).

```
expected: 2515880359
actual:   2563465191
```

And 136 more. The whole list is in `findings.sarif` and `report.json`, both of which are uploaded as artifacts by the run that produced this page.

## What the compiler says it has not built yet

Its own section rather than a line in the failures, because the response is different. A failure is somebody debugging tonight. This is a list of features, and the useful form of it is one line each, grouped so that twenty cases blocked on the same missing thing read as one missing thing.

Nothing. No compiler in this run refused a case on those terms.
