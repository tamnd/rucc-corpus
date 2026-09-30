# crc32c-armv8

CRC-32C through the ARMv8 CRC instructions against slicing by eight. Part of the correctness phase of the M4 plan.

72 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | also built with | axes | dialect | must print |
|---|---|---|---|---|
| [`crc32c-armv8.attribute-hwcap.aligned.0.c17.4ea39daf`](crc32c-armv8.attribute-hwcap.aligned.0.c17.4ea39daf.c) | nothing extra | setup=attribute-hwcap, step=aligned, offset=0 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.aligned.1.c17.f0681be5`](crc32c-armv8.attribute-hwcap.aligned.1.c17.f0681be5.c) | nothing extra | setup=attribute-hwcap, step=aligned, offset=1 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.aligned.2.c17.47d2eca3`](crc32c-armv8.attribute-hwcap.aligned.2.c17.47d2eca3.c) | nothing extra | setup=attribute-hwcap, step=aligned, offset=2 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.aligned.3.c17.bf574186`](crc32c-armv8.attribute-hwcap.aligned.3.c17.bf574186.c) | nothing extra | setup=attribute-hwcap, step=aligned, offset=3 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.aligned.4.c17.dac1fa99`](crc32c-armv8.attribute-hwcap.aligned.4.c17.dac1fa99.c) | nothing extra | setup=attribute-hwcap, step=aligned, offset=4 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.aligned.7.c17.5cb26577`](crc32c-armv8.attribute-hwcap.aligned.7.c17.5cb26577.c) | nothing extra | setup=attribute-hwcap, step=aligned, offset=7 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.b.0.c17.da5c9f8e`](crc32c-armv8.attribute-hwcap.b.0.c17.da5c9f8e.c) | nothing extra | setup=attribute-hwcap, step=b, offset=0 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.b.1.c17.e6222978`](crc32c-armv8.attribute-hwcap.b.1.c17.e6222978.c) | nothing extra | setup=attribute-hwcap, step=b, offset=1 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.b.2.c17.2173194d`](crc32c-armv8.attribute-hwcap.b.2.c17.2173194d.c) | nothing extra | setup=attribute-hwcap, step=b, offset=2 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.b.3.c17.b7352b1e`](crc32c-armv8.attribute-hwcap.b.3.c17.b7352b1e.c) | nothing extra | setup=attribute-hwcap, step=b, offset=3 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.b.4.c17.ce14e311`](crc32c-armv8.attribute-hwcap.b.4.c17.ce14e311.c) | nothing extra | setup=attribute-hwcap, step=b, offset=4 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.b.7.c17.c4d8420f`](crc32c-armv8.attribute-hwcap.b.7.c17.c4d8420f.c) | nothing extra | setup=attribute-hwcap, step=b, offset=7 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.d.0.c17.99140bd9`](crc32c-armv8.attribute-hwcap.d.0.c17.99140bd9.c) | nothing extra | setup=attribute-hwcap, step=d, offset=0 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.d.1.c17.2e963fe3`](crc32c-armv8.attribute-hwcap.d.1.c17.2e963fe3.c) | nothing extra | setup=attribute-hwcap, step=d, offset=1 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.d.2.c17.ab0f5848`](crc32c-armv8.attribute-hwcap.d.2.c17.ab0f5848.c) | nothing extra | setup=attribute-hwcap, step=d, offset=2 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.d.3.c17.e6048001`](crc32c-armv8.attribute-hwcap.d.3.c17.e6048001.c) | nothing extra | setup=attribute-hwcap, step=d, offset=3 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.d.4.c17.89edb02c`](crc32c-armv8.attribute-hwcap.d.4.c17.89edb02c.c) | nothing extra | setup=attribute-hwcap, step=d, offset=4 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.d.7.c17.aff2329c`](crc32c-armv8.attribute-hwcap.d.7.c17.aff2329c.c) | nothing extra | setup=attribute-hwcap, step=d, offset=7 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.w.0.c17.29a6c4ca`](crc32c-armv8.attribute-hwcap.w.0.c17.29a6c4ca.c) | nothing extra | setup=attribute-hwcap, step=w, offset=0 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.w.1.c17.4f99c85f`](crc32c-armv8.attribute-hwcap.w.1.c17.4f99c85f.c) | nothing extra | setup=attribute-hwcap, step=w, offset=1 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.w.2.c17.1ecea778`](crc32c-armv8.attribute-hwcap.w.2.c17.1ecea778.c) | nothing extra | setup=attribute-hwcap, step=w, offset=2 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.w.3.c17.fd4dcebe`](crc32c-armv8.attribute-hwcap.w.3.c17.fd4dcebe.c) | nothing extra | setup=attribute-hwcap, step=w, offset=3 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.w.4.c17.1bf09df7`](crc32c-armv8.attribute-hwcap.w.4.c17.1bf09df7.c) | nothing extra | setup=attribute-hwcap, step=w, offset=4 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.attribute-hwcap.w.7.c17.cd256859`](crc32c-armv8.attribute-hwcap.w.7.c17.cd256859.c) | nothing extra | setup=attribute-hwcap, step=w, offset=7 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.aligned.0.c17.4813632c`](crc32c-armv8.march-hwcap.aligned.0.c17.4813632c.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=aligned, offset=0 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.aligned.1.c17.275d4b9d`](crc32c-armv8.march-hwcap.aligned.1.c17.275d4b9d.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=aligned, offset=1 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.aligned.2.c17.a22b4298`](crc32c-armv8.march-hwcap.aligned.2.c17.a22b4298.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=aligned, offset=2 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.aligned.3.c17.03e90930`](crc32c-armv8.march-hwcap.aligned.3.c17.03e90930.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=aligned, offset=3 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.aligned.4.c17.c2f46baf`](crc32c-armv8.march-hwcap.aligned.4.c17.c2f46baf.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=aligned, offset=4 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.aligned.7.c17.db2a77c5`](crc32c-armv8.march-hwcap.aligned.7.c17.db2a77c5.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=aligned, offset=7 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.b.0.c17.5f60f80c`](crc32c-armv8.march-hwcap.b.0.c17.5f60f80c.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=b, offset=0 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.b.1.c17.76c15849`](crc32c-armv8.march-hwcap.b.1.c17.76c15849.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=b, offset=1 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.b.2.c17.ea7b4274`](crc32c-armv8.march-hwcap.b.2.c17.ea7b4274.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=b, offset=2 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.b.3.c17.070759e4`](crc32c-armv8.march-hwcap.b.3.c17.070759e4.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=b, offset=3 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.b.4.c17.df8d6435`](crc32c-armv8.march-hwcap.b.4.c17.df8d6435.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=b, offset=4 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.b.7.c17.26e65567`](crc32c-armv8.march-hwcap.b.7.c17.26e65567.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=b, offset=7 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.d.0.c17.663ec327`](crc32c-armv8.march-hwcap.d.0.c17.663ec327.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=d, offset=0 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.d.1.c17.537fd5f1`](crc32c-armv8.march-hwcap.d.1.c17.537fd5f1.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=d, offset=1 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.d.2.c17.000041a6`](crc32c-armv8.march-hwcap.d.2.c17.000041a6.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=d, offset=2 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.d.3.c17.f028e3a9`](crc32c-armv8.march-hwcap.d.3.c17.f028e3a9.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=d, offset=3 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.d.4.c17.ac695e7b`](crc32c-armv8.march-hwcap.d.4.c17.ac695e7b.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=d, offset=4 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.d.7.c17.86f4ec16`](crc32c-armv8.march-hwcap.d.7.c17.86f4ec16.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=d, offset=7 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.w.0.c17.8708544f`](crc32c-armv8.march-hwcap.w.0.c17.8708544f.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=w, offset=0 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.w.1.c17.a3b5c5db`](crc32c-armv8.march-hwcap.w.1.c17.a3b5c5db.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=w, offset=1 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.w.2.c17.b8d3fdfa`](crc32c-armv8.march-hwcap.w.2.c17.b8d3fdfa.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=w, offset=2 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.w.3.c17.befd8d7c`](crc32c-armv8.march-hwcap.w.3.c17.befd8d7c.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=w, offset=3 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.w.4.c17.ea3ae467`](crc32c-armv8.march-hwcap.w.4.c17.ea3ae467.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=w, offset=4 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march-hwcap.w.7.c17.e57548fd`](crc32c-armv8.march-hwcap.w.7.c17.e57548fd.c) | `-march=armv8-a+crc` | setup=march-hwcap, step=w, offset=7 | c17 | `3808858755 680278713 594227642 ...` and 41 more lines |
| [`crc32c-armv8.march.aligned.0.c17.80e7655f`](crc32c-armv8.march.aligned.0.c17.80e7655f.c) | `-march=armv8-a+crc` | setup=march, step=aligned, offset=0 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.aligned.1.c17.f6d41aeb`](crc32c-armv8.march.aligned.1.c17.f6d41aeb.c) | `-march=armv8-a+crc` | setup=march, step=aligned, offset=1 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.aligned.2.c17.09a77851`](crc32c-armv8.march.aligned.2.c17.09a77851.c) | `-march=armv8-a+crc` | setup=march, step=aligned, offset=2 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.aligned.3.c17.1251eecc`](crc32c-armv8.march.aligned.3.c17.1251eecc.c) | `-march=armv8-a+crc` | setup=march, step=aligned, offset=3 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.aligned.4.c17.692214ee`](crc32c-armv8.march.aligned.4.c17.692214ee.c) | `-march=armv8-a+crc` | setup=march, step=aligned, offset=4 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.aligned.7.c17.5636ee03`](crc32c-armv8.march.aligned.7.c17.5636ee03.c) | `-march=armv8-a+crc` | setup=march, step=aligned, offset=7 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.b.0.c17.039cb716`](crc32c-armv8.march.b.0.c17.039cb716.c) | `-march=armv8-a+crc` | setup=march, step=b, offset=0 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.b.1.c17.0b8f2721`](crc32c-armv8.march.b.1.c17.0b8f2721.c) | `-march=armv8-a+crc` | setup=march, step=b, offset=1 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.b.2.c17.cfc01adc`](crc32c-armv8.march.b.2.c17.cfc01adc.c) | `-march=armv8-a+crc` | setup=march, step=b, offset=2 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.b.3.c17.cc800ff5`](crc32c-armv8.march.b.3.c17.cc800ff5.c) | `-march=armv8-a+crc` | setup=march, step=b, offset=3 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.b.4.c17.b28bd448`](crc32c-armv8.march.b.4.c17.b28bd448.c) | `-march=armv8-a+crc` | setup=march, step=b, offset=4 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.b.7.c17.e2550062`](crc32c-armv8.march.b.7.c17.e2550062.c) | `-march=armv8-a+crc` | setup=march, step=b, offset=7 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.d.0.c17.7dd34f4f`](crc32c-armv8.march.d.0.c17.7dd34f4f.c) | `-march=armv8-a+crc` | setup=march, step=d, offset=0 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.d.1.c17.d52005ec`](crc32c-armv8.march.d.1.c17.d52005ec.c) | `-march=armv8-a+crc` | setup=march, step=d, offset=1 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.d.2.c17.9b50baf8`](crc32c-armv8.march.d.2.c17.9b50baf8.c) | `-march=armv8-a+crc` | setup=march, step=d, offset=2 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.d.3.c17.bd8d13ce`](crc32c-armv8.march.d.3.c17.bd8d13ce.c) | `-march=armv8-a+crc` | setup=march, step=d, offset=3 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.d.4.c17.c0075bdc`](crc32c-armv8.march.d.4.c17.c0075bdc.c) | `-march=armv8-a+crc` | setup=march, step=d, offset=4 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.d.7.c17.d36a32dd`](crc32c-armv8.march.d.7.c17.d36a32dd.c) | `-march=armv8-a+crc` | setup=march, step=d, offset=7 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.w.0.c17.4776f4e3`](crc32c-armv8.march.w.0.c17.4776f4e3.c) | `-march=armv8-a+crc` | setup=march, step=w, offset=0 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.w.1.c17.4ba753a7`](crc32c-armv8.march.w.1.c17.4ba753a7.c) | `-march=armv8-a+crc` | setup=march, step=w, offset=1 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.w.2.c17.63225a09`](crc32c-armv8.march.w.2.c17.63225a09.c) | `-march=armv8-a+crc` | setup=march, step=w, offset=2 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.w.3.c17.c4001e42`](crc32c-armv8.march.w.3.c17.c4001e42.c) | `-march=armv8-a+crc` | setup=march, step=w, offset=3 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.w.4.c17.53228b71`](crc32c-armv8.march.w.4.c17.53228b71.c) | `-march=armv8-a+crc` | setup=march, step=w, offset=4 | c17 | `1 3808858755 680278713 ...` and 42 more lines |
| [`crc32c-armv8.march.w.7.c17.1a6fc30f`](crc32c-armv8.march.w.7.c17.1a6fc30f.c) | `-march=armv8-a+crc` | setup=march, step=w, offset=7 | c17 | `1 3808858755 680278713 ...` and 42 more lines |

