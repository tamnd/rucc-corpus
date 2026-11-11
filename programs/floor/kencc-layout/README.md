# kencc-layout

the record layout and the bit-fields of kencc for each shape. Part of the floor phase of the M4 plan.

15 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | also built with | axes | dialect | must print |
|---|---|---|---|---|
| [`kencc-layout.bit-field-after-record.kencc.c17.3ff875e0`](kencc-layout.bit-field-after-record.kencc.c17.3ff875e0.c) | `-fkencc` `-fkencc-layout` | shape=bit-field-after-record, dialect=kencc | c17 | `16 7 1 1` |
| [`kencc-layout.bit-fields.kencc.c17.6bfce060`](kencc-layout.bit-fields.kencc.c17.6bfce060.c) | `-fkencc` `-fkencc-layout` | shape=bit-fields, dialect=kencc | c17 | `16 4 17 ...` and 3 more lines |
| [`kencc-layout.floating.kencc.c17.d5eb5e9e`](kencc-layout.floating.kencc.c17.d5eb5e9e.c) | `-fkencc` `-fkencc-layout` | shape=floating, dialect=kencc | c17 | `24 8 16` |
| [`kencc-layout.full-unit.kencc.c17.9490e8d1`](kencc-layout.full-unit.kencc.c17.9490e8d1.c) | `-fkencc` `-fkencc-layout` | shape=full-unit, dialect=kencc | c17 | `8 1 5 51` |
| [`kencc-layout.narrow-types.kencc.c17.8766e9bd`](kencc-layout.narrow-types.kencc.c17.8766e9bd.c) | `-fkencc` `-fkencc-layout` | shape=narrow-types, dialect=kencc | c17 | `8 11 4 1293` |
| [`kencc-layout.one-char.kencc.c17.67df437b`](kencc-layout.one-char.kencc.c17.67df437b.c) | `-fkencc` `-fkencc-layout` | shape=one-char, dialect=kencc | c17 | `8` |
| [`kencc-layout.record-array.kencc.c17.5fbc1f4c`](kencc-layout.record-array.kencc.c17.5fbc1f4c.c) | `-fkencc` `-fkencc-layout` | shape=record-array, dialect=kencc | c17 | `32 8 24` |
| [`kencc-layout.record-member.kencc.c17.9101e34a`](kencc-layout.record-member.kencc.c17.9101e34a.c) | `-fkencc` `-fkencc-layout` | shape=record-member, dialect=kencc | c17 | `24 8 16` |
| [`kencc-layout.scalar-array.kencc.c17.b873fe81`](kencc-layout.scalar-array.kencc.c17.b873fe81.c) | `-fkencc` `-fkencc-layout` | shape=scalar-array, dialect=kencc | c17 | `8 1 4` |
| [`kencc-layout.scalars.kencc.c17.3a927f68`](kencc-layout.scalars.kencc.c17.3a927f68.c) | `-fkencc` `-fkencc-layout` | shape=scalars, dialect=kencc | c17 | `16 2 4 8` |
| [`kencc-layout.union-bit-field.kencc.c17.aaeef49c`](kencc-layout.union-bit-field.kencc.c17.aaeef49c.c) | `-fkencc` `-fkencc-layout` | shape=union-bit-field, dialect=kencc | c17 | `8` |
| [`kencc-layout.union.kencc.c17.3579d337`](kencc-layout.union.kencc.c17.3579d337.c) | `-fkencc` `-fkencc-layout` | shape=union, dialect=kencc | c17 | `8 16` |
| [`kencc-layout.unnamed-field.kencc.c17.b602496a`](kencc-layout.unnamed-field.kencc.c17.b602496a.c) | `-fkencc` `-fkencc-layout` | shape=unnamed-field, dialect=kencc | c17 | `8 1 2` |
| [`kencc-layout.vlong.kencc.c17.f3538f6a`](kencc-layout.vlong.kencc.c17.f3538f6a.c) | `-fkencc` `-fkencc-layout` | shape=vlong, dialect=kencc | c17 | `24 8 16` |
| [`kencc-layout.zero-width.kencc.c17.c53b311a`](kencc-layout.zero-width.kencc.c17.c53b311a.c) | `-fkencc` `-fkencc-layout` | shape=zero-width, dialect=kencc | c17 | `8 3 1 13` |

