# conditional-store

moving a store below a branch whose arms wrote the same place. Part of the local phase of the M4 plan.

16 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`conditional-store.atomic-arms.c17.8d7b15b7`](conditional-store.atomic-arms.c17.8d7b15b7.c) | shape=atomic-arms | c17 | `32768` |
| [`conditional-store.both-arms.i16.c17.36da89d9`](conditional-store.both-arms.i16.c17.36da89d9.c) | shape=both-arms, width=i16 | c17 | `78592` |
| [`conditional-store.both-arms.i32.c17.86a108f3`](conditional-store.both-arms.i32.c17.86a108f3.c) | shape=both-arms, width=i32 | c17 | `78592` |
| [`conditional-store.both-arms.i64.c17.982071f7`](conditional-store.both-arms.i64.c17.982071f7.c) | shape=both-arms, width=i64 | c17 | `78592` |
| [`conditional-store.both-arms.i8.c17.07848c02`](conditional-store.both-arms.i8.c17.07848c02.c) | shape=both-arms, width=i8 | c17 | `256` |
| [`conditional-store.call-on-one-arm.c17.2c62b38a`](conditional-store.call-on-one-arm.c17.2c62b38a.c) | shape=call-on-one-arm | c17 | `32768 128` |
| [`conditional-store.hoisted-address.c17.d325066b`](conditional-store.hoisted-address.c17.d325066b.c) | shape=hoisted-address | c17 | `32768` |
| [`conditional-store.one-armed.i16.c17.fd7eb2d7`](conditional-store.one-armed.i16.c17.fd7eb2d7.c) | shape=one-armed, width=i16 | c17 | `17152` |
| [`conditional-store.one-armed.i32.c17.7c08b1fa`](conditional-store.one-armed.i32.c17.7c08b1fa.c) | shape=one-armed, width=i32 | c17 | `17152` |
| [`conditional-store.one-armed.i64.c17.a9e5131b`](conditional-store.one-armed.i64.c17.a9e5131b.c) | shape=one-armed, width=i64 | c17 | `17152` |
| [`conditional-store.one-armed.i8.c17.29adb408`](conditional-store.one-armed.i8.c17.29adb408.c) | shape=one-armed, width=i8 | c17 | `512` |
| [`conditional-store.same-value.c17.0c99f680`](conditional-store.same-value.c17.0c99f680.c) | shape=same-value | c17 | `33408` |
| [`conditional-store.struct-field.c17.e4c755b8`](conditional-store.struct-field.c17.e4c755b8.c) | shape=struct-field | c17 | `32768` |
| [`conditional-store.two-addresses.c17.e4626938`](conditional-store.two-addresses.c17.e4626938.c) | shape=two-addresses | c17 | `34048` |
| [`conditional-store.unhoisted-address.c17.db791149`](conditional-store.unhoisted-address.c17.db791149.c) | shape=unhoisted-address | c17 | `32768` |
| [`conditional-store.volatile-arms.c17.92fe807f`](conditional-store.volatile-arms.c17.92fe807f.c) | shape=volatile-arms | c17 | `32768` |

