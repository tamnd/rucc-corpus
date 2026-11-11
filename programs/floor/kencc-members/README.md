# kencc-members

the unnamed members of kencc that the Plan 9 tree depends on. Part of the floor phase of the M4 plan.

17 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | also built with | axes | dialect | must print |
|---|---|---|---|---|
| [`kencc-members.ambiguous.kencc.c17.0e5e9ac4`](kencc-members.ambiguous.kencc.c17.0e5e9ac4.c) | `-fkencc` | shape=ambiguous, dialect=kencc | c17 | nothing, it must be rejected with a diagnostic mentioning `ambiguous structure element` |
| [`kencc-members.member-by-tag.kencc.c17.eaaf6fa2`](kencc-members.member-by-tag.kencc.c17.eaaf6fa2.c) | `-fkencc` | shape=member-by-tag, dialect=kencc | c17 | `2 3 4` |
| [`kencc-members.member-by-type-name.kencc.c17.45378229`](kencc-members.member-by-type-name.kencc.c17.45378229.c) | `-fkencc` | shape=member-by-type-name, dialect=kencc | c17 | `5 5 1` |
| [`kencc-members.member-by-type-name.plan9-extensions.c17.4314263d`](kencc-members.member-by-type-name.plan9-extensions.c17.4314263d.c) | `-fplan9-extensions` | shape=member-by-type-name, dialect=plan9-extensions | c17 | `5 5 1` |
| [`kencc-members.member-of-member.kencc.c17.dc6fe861`](kencc-members.member-of-member.kencc.c17.dc6fe861.c) | `-fkencc` | shape=member-of-member, dialect=kencc | c17 | `3 40` |
| [`kencc-members.member-of-member.plan9-extensions.c17.c95f8a21`](kencc-members.member-of-member.plan9-extensions.c17.c95f8a21.c) | `-fplan9-extensions` | shape=member-of-member, dialect=plan9-extensions | c17 | `3 40` |
| [`kencc-members.nested.kencc.c17.c8c5b39c`](kencc-members.nested.kencc.c17.c8c5b39c.c) | `-fkencc` | shape=nested, dialect=kencc | c17 | `5 5 6 ...` and 2 more lines |
| [`kencc-members.nested.plan9-extensions.c17.9e3211fb`](kencc-members.nested.plan9-extensions.c17.9e3211fb.c) | `-fplan9-extensions` | shape=nested, dialect=plan9-extensions | c17 | `5 5 6 ...` and 2 more lines |
| [`kencc-members.own-member-first.kencc.c17.7db565ab`](kencc-members.own-member-first.kencc.c17.7db565ab.c) | `-fkencc` | shape=own-member-first, dialect=kencc | c17 | `1 2` |
| [`kencc-members.pointer-argument.kencc.c17.49e776f9`](kencc-members.pointer-argument.kencc.c17.49e776f9.c) | `-fkencc` | shape=pointer-argument, dialect=kencc | c17 | `42 42` |
| [`kencc-members.pointer-argument.plan9-extensions.c17.0306b0bd`](kencc-members.pointer-argument.plan9-extensions.c17.0306b0bd.c) | `-fplan9-extensions` | shape=pointer-argument, dialect=plan9-extensions | c17 | `42 42` |
| [`kencc-members.pointer-assignment.kencc.c17.cdfd381f`](kencc-members.pointer-assignment.kencc.c17.cdfd381f.c) | `-fkencc` | shape=pointer-assignment, dialect=kencc | c17 | `10 10 4` |
| [`kencc-members.pointer-assignment.plan9-extensions.c17.c911a8b4`](kencc-members.pointer-assignment.plan9-extensions.c17.c911a8b4.c) | `-fplan9-extensions` | shape=pointer-assignment, dialect=plan9-extensions | c17 | `10 10 4` |
| [`kencc-members.same-name-twice.kencc.c17.3aaeb192`](kencc-members.same-name-twice.kencc.c17.3aaeb192.c) | `-fkencc` | shape=same-name-twice, dialect=kencc | c17 | `1 2 1 3` |
| [`kencc-members.union-member.kencc.c17.23e421de`](kencc-members.union-member.kencc.c17.23e421de.c) | `-fkencc` | shape=union-member, dialect=kencc | c17 | `1 7` |
| [`kencc-members.union-member.plan9-extensions.c17.96898e29`](kencc-members.union-member.plan9-extensions.c17.96898e29.c) | `-fplan9-extensions` | shape=union-member, dialect=plan9-extensions | c17 | `1 7` |
| [`kencc-members.value-conversion.kencc.c17.fd182069`](kencc-members.value-conversion.kencc.c17.fd182069.c) | `-fkencc` | shape=value-conversion, dialect=kencc | c17 | `9 8` |

