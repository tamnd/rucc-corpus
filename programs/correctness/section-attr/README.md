# section-attr

functions and data in named sections, walked from __start_ to __stop_. Part of the correctness phase of the M4 plan.

6 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`section-attr.aligned-entries.c17.15c0a95e`](section-attr.aligned-entries.c17.15c0a95e.c) | shape=aligned-entries | c17 | `6631` |
| [`section-attr.data-sections.c17.05652397`](section-attr.data-sections.c17.05652397.c) | shape=data-sections | c17 | `11606` |
| [`section-attr.init-text.c17.90ea2b5c`](section-attr.init-text.c17.90ea2b5c.c) | shape=init-text | c17 | `3857118503` |
| [`section-attr.initcall-levels.c17.f07e02d5`](section-attr.initcall-levels.c17.f07e02d5.c) | shape=initcall-levels | c17 | `473267159` |
| [`section-attr.initcall-table.c17.bd37f642`](section-attr.initcall-table.c17.bd37f642.c) | shape=initcall-table | c17 | `4191853971` |
| [`section-attr.param-table.c17.c0df2a5d`](section-attr.param-table.c17.c0df2a5d.c) | shape=param-table | c17 | `11841603` |

