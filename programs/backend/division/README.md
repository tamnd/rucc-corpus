# division

a division by a constant done often enough to time how it was lowered. Part of the backend phase of the M4 plan.

72 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`division.i16.10.quotient.c17.e18a1eca`](division.i16.10.quotient.c17.e18a1eca.c) | type=i16, divisor=10, op=quotient | c17 | `97751040` |
| [`division.i16.10.remainder.c17.d0901b02`](division.i16.10.remainder.c17.d0901b02.c) | type=i16, divisor=10, op=remainder | c17 | `163840` |
| [`division.i16.7.quotient.c17.bcd7df39`](division.i16.7.quotient.c17.bcd7df39.c) | type=i16, divisor=7, op=quotient | c17 | `139658240` |
| [`division.i16.7.remainder.c17.90131eb8`](division.i16.7.remainder.c17.90131eb8.c) | type=i16, divisor=7, op=remainder | c17 | `66560` |
| [`division.i16.8.quotient.c17.15263fb6`](division.i16.8.quotient.c17.15263fb6.c) | type=i16, divisor=8, op=quotient | c17 | `122217472` |
| [`division.i16.8.remainder.c17.2ef9c239`](division.i16.8.remainder.c17.2ef9c239.c) | type=i16, divisor=8, op=remainder | c17 | `18446744073709486080` |
| [`division.i16.minus-7.quotient.c17.2bcd5ce1`](division.i16.minus-7.quotient.c17.2bcd5ce1.c) | type=i16, divisor=minus-7, op=quotient | c17 | `18446744073569893376` |
| [`division.i16.minus-7.remainder.c17.16c3877d`](division.i16.minus-7.remainder.c17.16c3877d.c) | type=i16, divisor=minus-7, op=remainder | c17 | `66560` |
| [`division.i32-widened.10.quotient.c17.5f5eb975`](division.i32-widened.10.quotient.c17.5f5eb975.c) | type=i32-widened, divisor=10, op=quotient | c17 | `4377881300992` |
| [`division.i32-widened.10.remainder.c17.abf5ed3a`](division.i32-widened.10.remainder.c17.abf5ed3a.c) | type=i32-widened, divisor=10, op=remainder | c17 | `227328` |
| [`division.i32-widened.7.quotient.c17.b53a3070`](division.i32-widened.7.quotient.c17.b53a3070.c) | type=i32-widened, divisor=7, op=quotient | c17 | `6254116158464` |
| [`division.i32-widened.7.remainder.c17.a6e87ac6`](division.i32-widened.7.remainder.c17.a6e87ac6.c) | type=i32-widened, divisor=7, op=remainder | c17 | `128000` |
| [`division.i32-widened.8.quotient.c17.70862d04`](division.i32-widened.8.quotient.c17.70862d04.c) | type=i32-widened, divisor=8, op=quotient | c17 | `5472351636480` |
| [`division.i32-widened.8.remainder.c17.204655fc`](division.i32-widened.8.remainder.c17.204655fc.c) | type=i32-widened, divisor=8, op=remainder | c17 | `145408` |
| [`division.i32-widened.minus-7.quotient.c17.b7bdee48`](division.i32-widened.minus-7.quotient.c17.b7bdee48.c) | type=i32-widened, divisor=minus-7, op=quotient | c17 | `18446737819593393152` |
| [`division.i32-widened.minus-7.remainder.c17.2c203da7`](division.i32-widened.minus-7.remainder.c17.2c203da7.c) | type=i32-widened, divisor=minus-7, op=remainder | c17 | `128000` |
| [`division.i32.10.quotient.c17.8ca6bd46`](division.i32.10.quotient.c17.8ca6bd46.c) | type=i32, divisor=10, op=quotient | c17 | `4377881300992` |
| [`division.i32.10.remainder.c17.d87678cf`](division.i32.10.remainder.c17.d87678cf.c) | type=i32, divisor=10, op=remainder | c17 | `227328` |
| [`division.i32.7.quotient.c17.0031e7f8`](division.i32.7.quotient.c17.0031e7f8.c) | type=i32, divisor=7, op=quotient | c17 | `6254116158464` |
| [`division.i32.7.remainder.c17.8e64b4f2`](division.i32.7.remainder.c17.8e64b4f2.c) | type=i32, divisor=7, op=remainder | c17 | `128000` |
| [`division.i32.8.quotient.c17.dea8532d`](division.i32.8.quotient.c17.dea8532d.c) | type=i32, divisor=8, op=quotient | c17 | `5472351636480` |
| [`division.i32.8.remainder.c17.bd4fe7f2`](division.i32.8.remainder.c17.bd4fe7f2.c) | type=i32, divisor=8, op=remainder | c17 | `145408` |
| [`division.i32.minus-7.quotient.c17.71472560`](division.i32.minus-7.quotient.c17.71472560.c) | type=i32, divisor=minus-7, op=quotient | c17 | `18446737819593393152` |
| [`division.i32.minus-7.remainder.c17.97593078`](division.i32.minus-7.remainder.c17.97593078.c) | type=i32, divisor=minus-7, op=remainder | c17 | `128000` |
| [`division.i64.10.quotient.c17.ad3bca20`](division.i64.10.quotient.c17.ad3bca20.c) | type=i64, divisor=10, op=quotient | c17 | `5625343137770297344` |
| [`division.i64.10.remainder.c17.849341bc`](division.i64.10.remainder.c17.849341bc.c) | type=i64, divisor=10, op=remainder | c17 | `233472` |
| [`division.i64.7.quotient.c17.f96dc035`](division.i64.7.quotient.c17.f96dc035.c) | type=i64, divisor=7, op=quotient | c17 | `2765706175754849280` |
| [`division.i64.7.remainder.c17.46a82009`](division.i64.7.remainder.c17.46a82009.c) | type=i64, divisor=7, op=remainder | c17 | `158720` |
| [`division.i64.8.quotient.c17.bacc4a1e`](division.i64.8.quotient.c17.bacc4a1e.c) | type=i64, divisor=8, op=quotient | c17 | `2419992903785488384` |
| [`division.i64.8.remainder.c17.146ea4d9`](division.i64.8.remainder.c17.146ea4d9.c) | type=i64, divisor=8, op=remainder | c17 | `196608` |
| [`division.i64.minus-7.quotient.c17.9850eb47`](division.i64.minus-7.quotient.c17.9850eb47.c) | type=i64, divisor=minus-7, op=quotient | c17 | `15681037897954702336` |
| [`division.i64.minus-7.remainder.c17.b5a7e307`](division.i64.minus-7.remainder.c17.b5a7e307.c) | type=i64, divisor=minus-7, op=remainder | c17 | `158720` |
| [`division.i8.10.quotient.c17.8e5f9a71`](division.i8.10.quotient.c17.8e5f9a71.c) | type=i8, divisor=10, op=quotient | c17 | `18446744073709246464` |
| [`division.i8.10.remainder.c17.9925c9cb`](division.i8.10.remainder.c17.9925c9cb.c) | type=i8, divisor=10, op=remainder | c17 | `18446744073709334528` |
| [`division.i8.7.quotient.c17.911ef20c`](division.i8.7.quotient.c17.911ef20c.c) | type=i8, divisor=7, op=quotient | c17 | `18446744073709091840` |
| [`division.i8.7.remainder.c17.f81b1a56`](division.i8.7.remainder.c17.f81b1a56.c) | type=i8, divisor=7, op=remainder | c17 | `18446744073709501440` |
| [`division.i8.8.quotient.c17.cc8a33e5`](division.i8.8.quotient.c17.cc8a33e5.c) | type=i8, divisor=8, op=quotient | c17 | `18446744073709158400` |
| [`division.i8.8.remainder.c17.56d837c0`](division.i8.8.remainder.c17.56d837c0.c) | type=i8, divisor=8, op=remainder | c17 | `18446744073709428736` |
| [`division.i8.minus-7.quotient.c17.affa7f2e`](division.i8.minus-7.quotient.c17.affa7f2e.c) | type=i8, divisor=minus-7, op=quotient | c17 | `459776` |
| [`division.i8.minus-7.remainder.c17.abdb10db`](division.i8.minus-7.remainder.c17.abdb10db.c) | type=i8, divisor=minus-7, op=remainder | c17 | `18446744073709501440` |
| [`division.pointer.12.exact.c17.8fba7d3f`](division.pointer.12.exact.c17.8fba7d3f.c) | type=pointer, divisor=12, op=exact | c17 | `2450432` |
| [`division.pointer.7.exact.c17.ae293368`](division.pointer.7.exact.c17.ae293368.c) | type=pointer, divisor=7, op=exact | c17 | `2450432` |
| [`division.u16.10.quotient.c17.84932864`](division.u16.10.quotient.c17.84932864.c) | type=u16, divisor=10, op=quotient | c17 | `6948644864` |
| [`division.u16.10.remainder.c17.c3bf222e`](division.u16.10.remainder.c17.c3bf222e.c) | type=u16, divisor=10, op=remainder | c17 | `9375744` |
| [`division.u16.7.quotient.c17.69298b2d`](division.u16.7.quotient.c17.69298b2d.c) | type=u16, divisor=7, op=quotient | c17 | `9927083008` |
| [`division.u16.7.remainder.c17.d4571da4`](division.u16.7.remainder.c17.d4571da4.c) | type=u16, divisor=7, op=remainder | c17 | `6243328` |
| [`division.u16.8.quotient.c17.019d0f13`](division.u16.8.quotient.c17.019d0f13.c) | type=u16, divisor=8, op=quotient | c17 | `8686058496` |
| [`division.u16.8.remainder.c17.c8f9391d`](division.u16.8.remainder.c17.c8f9391d.c) | type=u16, divisor=8, op=remainder | c17 | `7356416` |
| [`division.u32-widened.10.quotient.c17.d9123e4e`](division.u32-widened.10.quotient.c17.d9123e4e.c) | type=u32-widened, divisor=10, op=quotient | c17 | `442863117540352` |
| [`division.u32-widened.10.remainder.c17.306d249d`](division.u32-widened.10.remainder.c17.306d249d.c) | type=u32-widened, divisor=10, op=remainder | c17 | `9404416` |
| [`division.u32-widened.7.quotient.c17.6de7858c`](division.u32-widened.7.quotient.c17.6de7858c.c) | type=u32-widened, divisor=7, op=quotient | c17 | `632661596946432` |
| [`division.u32-widened.7.remainder.c17.adce5299`](division.u32-widened.7.remainder.c17.adce5299.c) | type=u32-widened, divisor=7, op=remainder | c17 | `6182912` |
| [`division.u32-widened.8.quotient.c17.449877b2`](division.u32-widened.8.quotient.c17.449877b2.c) | type=u32-widened, divisor=8, op=quotient | c17 | `553578897167360` |
| [`division.u32-widened.8.remainder.c17.3670455b`](division.u32-widened.8.remainder.c17.3670455b.c) | type=u32-widened, divisor=8, op=remainder | c17 | `7469056` |
| [`division.u32.10.quotient.c17.4433e19b`](division.u32.10.quotient.c17.4433e19b.c) | type=u32, divisor=10, op=quotient | c17 | `442863117540352` |
| [`division.u32.10.remainder.c17.e6060c31`](division.u32.10.remainder.c17.e6060c31.c) | type=u32, divisor=10, op=remainder | c17 | `9404416` |
| [`division.u32.7.quotient.c17.5bd3d47f`](division.u32.7.quotient.c17.5bd3d47f.c) | type=u32, divisor=7, op=quotient | c17 | `632661596946432` |
| [`division.u32.7.remainder.c17.7deb785f`](division.u32.7.remainder.c17.7deb785f.c) | type=u32, divisor=7, op=remainder | c17 | `6182912` |
| [`division.u32.8.quotient.c17.0f26eea8`](division.u32.8.quotient.c17.0f26eea8.c) | type=u32, divisor=8, op=quotient | c17 | `553578897167360` |
| [`division.u32.8.remainder.c17.26ab56c9`](division.u32.8.remainder.c17.26ab56c9.c) | type=u32, divisor=8, op=remainder | c17 | `7469056` |
| [`division.u64.10.quotient.c17.03864c89`](division.u64.10.quotient.c17.03864c89.c) | type=u64, divisor=10, op=quotient | c17 | `1935994323027474432` |
| [`division.u64.10.remainder.c17.be6accd5`](division.u64.10.remainder.c17.be6accd5.c) | type=u64, divisor=10, op=remainder | c17 | `9359360` |
| [`division.u64.7.quotient.c17.55388905`](division.u64.7.quotient.c17.55388905.c) | type=u64, divisor=7, op=quotient | c17 | `130457022366883840` |
| [`division.u64.7.remainder.c17.402a4df6`](division.u64.7.remainder.c17.402a4df6.c) | type=u64, divisor=7, op=remainder | c17 | `6365184` |
| [`division.u64.8.quotient.c17.3d23fca4`](division.u64.8.quotient.c17.3d23fca4.c) | type=u64, divisor=8, op=quotient | c17 | `2419992903784595456` |
| [`division.u64.8.remainder.c17.05cb5be2`](division.u64.8.remainder.c17.05cb5be2.c) | type=u64, divisor=8, op=remainder | c17 | `7340032` |
| [`division.u8.10.quotient.c17.9e0722f7`](division.u8.10.quotient.c17.9e0722f7.c) | type=u8, divisor=10, op=quotient | c17 | `26027008` |
| [`division.u8.10.remainder.c17.ca999f38`](division.u8.10.remainder.c17.ca999f38.c) | type=u8, divisor=10, op=remainder | c17 | `9353216` |
| [`division.u8.7.quotient.c17.3c35775b`](division.u8.7.quotient.c17.3c35775b.c) | type=u8, divisor=7, op=quotient | c17 | `37630976` |
| [`division.u8.7.remainder.c17.b306d7aa`](division.u8.7.remainder.c17.b306d7aa.c) | type=u8, divisor=7, op=remainder | c17 | `6206464` |
| [`division.u8.8.quotient.c17.6a9b3bf6`](division.u8.8.quotient.c17.6a9b3bf6.c) | type=u8, divisor=8, op=quotient | c17 | `32783360` |
| [`division.u8.8.remainder.c17.133ded3f`](division.u8.8.remainder.c17.133ded3f.c) | type=u8, divisor=8, op=remainder | c17 | `7356416` |

