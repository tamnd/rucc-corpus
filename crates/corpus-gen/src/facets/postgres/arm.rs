//! The AArch64 halves of `crc32c` and `simd-lfind`.
//!
//! Postgres has an Arm path for both. `pg_crc32c_armv8.c` checksums with the CRC-32C
//! instructions from `arm_acle.h`, and `pg_crc32c_armv8_choose.c` asks `getauxval` whether the
//! processor has them when the build could not assume it. `port/simd.h` maps its vector
//! operations onto NEON, where there is no byte mask to test, so a comparison is reduced across
//! the vector with `vmaxvq_u8` or `vminvq_u8` instead, and `pg_lfind.h` is written on top of it.
//!
//! These cases are the same questions as the x86-64 ones, asked of the other back end. They
//! only build on AArch64, so every one of them carries the `aarch64` tag, and a run anywhere
//! else leaves them out with `--exclude-tag aarch64`.

use super::{CRC_BYTES, LCG_C, PG_SEED, PROVENANCE, crc_bytes, crc32c, lcg, slicing_by_eight};
use crate::Sink;
use crate::emit::Program;
use crate::lang::Ty;
use corpus_model::{Axes, Case, Dialect, Expect, Facet};

/// The tag on every case that only builds and runs on AArch64.
pub(super) const AARCH64: &str = "aarch64";

/// Emits both facets.
pub(super) fn generate(sink: &mut Sink<'_>) {
    crc32c_armv8(sink);
    simd_lfind_neon(sink);
}

/// The buffer lengths a `crc32c-armv8` case checksums.
///
/// Every length up to three words, so each combination of the halfword, word and byte tails
/// the loop leaves is there after each of the ways the head can be aligned, then the lengths
/// either side of the larger powers of two, up to a page less one.
const ARM_CRC_LENGTHS: &[usize] = &[
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 31,
    32, 33, 63, 64, 65, 100, 255, 256, 257, 1000, 1023, 4000, 4095,
];

/// How the program gets the right to use the CRC instructions, and whether it asks first.
///
/// `march` builds the whole file with `-march=armv8-a+crc` and calls the loop outright, which
/// is `USE_ARMV8_CRC32C`. `march-hwcap` builds the same way and asks `getauxval` first, which
/// is `USE_ARMV8_CRC32C_WITH_RUNTIME_CHECK`. `attribute-hwcap` asks the same way but builds
/// only the one function for the extension, with `__attribute__((target("+crc")))`, so the
/// rest of the file is plain ARMv8.
const SETUPS: &[&str] = &["march", "march-hwcap", "attribute-hwcap"];

/// CRC-32C with the ARMv8 instructions, checked against slicing by eight.
///
/// `step` is the loop: `b` is bytes only, `w` is words with a halfword and a byte for the tail,
/// `d` is doublewords from wherever the buffer starts, and `aligned` is `pg_comp_crc32c_armv8`,
/// which takes a byte, a halfword and a word as it needs to until the pointer is on an eight
/// byte boundary. `offset` is how far the buffer starts from an eight byte boundary, which
/// picks which of those three the aligned loop takes.
fn crc32c_armv8(sink: &mut Sink<'_>) {
    const STEPS: &[&str] = &["b", "w", "d", "aligned"];
    const OFFSETS: &[usize] = &[0, 1, 2, 3, 4, 7];
    for &setup in SETUPS {
        for &step in STEPS {
            for &offset in OFFSETS {
                if !sink.wants(Facet::Crc32cArmv8) {
                    return;
                }
                let program = crc32c_armv8_program(setup, step, offset);
                let axes =
                    Axes::of([("setup", setup), ("step", step), ("offset", &offset.to_string())]);
                let (source, expected) = program.finish();
                let flags = if setup.starts_with("march") {
                    vec!["-march=armv8-a+crc".to_owned()]
                } else {
                    Vec::new()
                };
                let case = Case::linked(
                    Facet::Crc32cArmv8,
                    axes,
                    Dialect::C17,
                    source,
                    Vec::new(),
                    flags,
                    Expect::Output(expected),
                )
                .tagged(&["gnu", "headers", AARCH64, PROVENANCE]);
                sink.push_case(case);
            }
        }
    }
}

/// One `crc32c-armv8` case.
fn crc32c_armv8_program(setup: &str, step: &str, offset: usize) -> Program {
    let mut program = Program::new(format!(
        "CRC-32C with the {step} ARMv8 loop at offset {offset}, built and chosen by {setup}"
    ));
    let asks = setup.ends_with("hwcap");
    program.include("arm_acle.h");
    if asks {
        program.include("sys/auxv.h");
    }
    program.top(format!("static unsigned long long storage[{}];", CRC_BYTES / 8));
    slicing_by_eight(&mut program);

    // The loads go through memcpy rather than a cast, which is the same load on AArch64
    // without reading a buffer of one type as another.
    let load = |program: &mut Program, width: &str, ty: &str, bytes: usize| {
        program.top(format!("        {ty} v;"));
        program.top(format!("        __builtin_memcpy(&v, p, {bytes});"));
        program.top(format!("        crc = __crc32c{width}(crc, v);"));
        program.top(format!("        p += {bytes};"));
    };
    let tail_word = |program: &mut Program| {
        program.top("    if (pend - p >= 4) {");
        load(program, "w", "unsigned int", 4);
        program.top("    }");
    };
    let tail_half = |program: &mut Program| {
        program.top("    if (pend - p >= 2) {");
        load(program, "h", "unsigned short", 2);
        program.top("    }");
    };
    let tail_byte = |program: &mut Program| {
        program.top("    if (p < pend) {");
        program.top("        crc = __crc32cb(crc, *p);");
        program.top("    }");
    };
    let doubles = |program: &mut Program| {
        program.top("    while (pend - p >= 8) {");
        load(program, "d", "unsigned long long", 8);
        program.top("    }");
    };

    if setup == "attribute-hwcap" {
        program.top("__attribute__((target(\"+crc\")))");
    }
    program
        .top("static unsigned int crc_armv8(unsigned int crc, const unsigned char *p, int len) {");
    program.top("    const unsigned char *pend = p + len;");
    match step {
        "b" => {
            program.top("    while (p < pend) {");
            program.top("        crc = __crc32cb(crc, *p);");
            program.top("        p++;");
            program.top("    }");
        }
        "w" => {
            program.top("    while (pend - p >= 4) {");
            load(&mut program, "w", "unsigned int", 4);
            program.top("    }");
            tail_half(&mut program);
            tail_byte(&mut program);
        }
        "d" => {
            doubles(&mut program);
            tail_word(&mut program);
            tail_half(&mut program);
            tail_byte(&mut program);
        }
        _ => {
            // pg_comp_crc32c_armv8, head and all. Postgres writes each test as `p + 8 <= pend`,
            // and the difference says the same without forming a pointer past the end.
            program.top("    if (((unsigned long)p & 1) != 0 && pend - p >= 1) {");
            program.top("        crc = __crc32cb(crc, *p);");
            program.top("        p += 1;");
            program.top("    }");
            program.top("    if (((unsigned long)p & 3) != 0 && pend - p >= 2) {");
            load(&mut program, "h", "unsigned short", 2);
            program.top("    }");
            program.top("    if (((unsigned long)p & 7) != 0 && pend - p >= 4) {");
            load(&mut program, "w", "unsigned int", 4);
            program.top("    }");
            doubles(&mut program);
            tail_word(&mut program);
            tail_half(&mut program);
            tail_byte(&mut program);
        }
    }
    program.top("    return crc;");
    program.top("}");

    if asks {
        // pg_crc32c_armv8_available, on Linux, where glibc has getauxval.
        program.top("static int cpu_has_feature(void) {");
        program.top("    return (getauxval(AT_HWCAP) & HWCAP_CRC32) != 0;");
        program.top("}");
        program.top("static unsigned int (*comp_crc)(unsigned int, const unsigned char *, int);");
    } else {
        // What configure is looking for when it decides no run time check is needed.
        program.top("#if defined(__ARM_FEATURE_CRC32)");
        program.top("static const int feature_macro = 1;");
        program.top("#else");
        program.top("static const int feature_macro = 0;");
        program.top("#endif");
    }
    let call = if asks { "comp_crc" } else { "crc_armv8" };
    program.top("static unsigned int crc_of(const unsigned char *p, int len) {");
    program.top(format!("    return {call}(0xffffffffu, p, len) ^ 0xffffffffu;"));
    program.top("}");
    // COMP_CRC32C over a record written in three pieces.
    program.top("static unsigned int crc_in_pieces(const unsigned char *p, int len) {");
    program.top("    int first = len / 3;");
    program.top("    int second = (len - first) / 2;");
    program.top("    unsigned int crc = 0xffffffffu;");
    program.top(format!("    crc = {call}(crc, p, first);"));
    program.top(format!("    crc = {call}(crc, p + first, second);"));
    program.top(format!("    crc = {call}(crc, p + first + second, len - first - second);"));
    program.top("    return crc ^ 0xffffffffu;");
    program.top("}");

    program.input(Ty::U32, "seed", i128::from(PG_SEED));
    program.line("unsigned char *bytes = (unsigned char *)storage;");
    program.line("unsigned int x = seed;");
    program.line(format!("for (int i = 0; i < {CRC_BYTES}; i++) {{"));
    program.line_at(1, LCG_C);
    program.line_at(1, "bytes[i] = (unsigned char)(x >> 24);");
    program.line("}");
    program.line("crc_init();");
    if asks {
        program.line("comp_crc = cpu_has_feature() ? crc_armv8 : crc_sb8;");
    } else {
        program.check(Ty::I32, "feature_macro", 1);
    }
    program.line(format!("const unsigned char *base = bytes + {offset};"));
    program.line("int mismatches = 0;");
    program.line("unsigned int got = 0;");
    // The check value every CRC-32C implementation publishes, and two runs of one byte.
    program.line(
        "static const unsigned char digits[9] = { '1', '2', '3', '4', '5', '6', '7', '8', '9' };",
    );
    program.line("static const unsigned char zeros[48] = { 0 };");
    program.line("unsigned char ones[48];");
    program.line("for (int i = 0; i < 48; i++) {");
    program.line_at(1, "ones[i] = 0xff;");
    program.line("}");
    program.check(Ty::U32, "crc_of(digits, 9)", i128::from(crc32c(b"123456789")));
    program.check(Ty::U32, "crc_of(zeros, 48)", i128::from(crc32c(&[0u8; 48])));
    program.check(Ty::U32, "crc_of(ones, 48)", i128::from(crc32c(&[0xffu8; 48])));
    program.check(Ty::U32, "crc_of(ones + 1, 45)", i128::from(crc32c(&[0xffu8; 45])));
    let data = crc_bytes(PG_SEED);
    for &len in ARM_CRC_LENGTHS {
        program.blank();
        program.line(format!("got = crc_of(base, {len});"));
        program.check(Ty::U32, "got", i128::from(crc32c(&data[offset..offset + len])));
        program.line(format!(
            "if (got != (crc_sb8(0xffffffffu, base, {len}) ^ 0xffffffffu) || got != crc_in_pieces(base, {len})) {{"
        ));
        program.line_at(1, "mismatches = mismatches + 1;");
        program.line("}");
    }
    program.blank();
    program.check(Ty::I32, "mismatches", 0);
    program
}

/// One of the NEON search loops.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Neon {
    /// `pg_lfind32` with one vector per iteration.
    Find32One,
    /// `pg_lfind32` as Postgres writes it, four vectors per iteration folded with `vorrq_u32`.
    Find32Four,
    /// `pg_lfind8`, with `vector8_has` reduced by `vmaxvq_u8`.
    Find8,
    /// `pg_lfind8` over two vectors per iteration folded with `vorrq_u8`.
    Find8Two,
    /// `pg_lfind8_le` the way `vector8_has_le` does it on NEON, with `vminvq_u8`.
    Find8LeMin,
    /// `pg_lfind8_le` with the saturating subtract the SSE2 path uses, in NEON.
    Find8LeSubs,
    /// The index of the first match, through `vector8_highbit_mask`.
    Index8Mask,
    /// The index of the first match, through the narrowing shift and a 64 bit lane.
    Index8Shrn,
}

impl Neon {
    const ALL: [Self; 8] = [
        Self::Find32One,
        Self::Find32Four,
        Self::Find8,
        Self::Find8Two,
        Self::Find8LeMin,
        Self::Find8LeSubs,
        Self::Index8Mask,
        Self::Index8Shrn,
    ];

    const fn name(self) -> &'static str {
        match self {
            Self::Find32One => "lfind32-x1",
            Self::Find32Four => "lfind32-x4",
            Self::Find8 => "lfind8",
            Self::Find8Two => "lfind8-x2",
            Self::Find8LeMin => "lfind8-le-minv",
            Self::Find8LeSubs => "lfind8-le-subs",
            Self::Index8Mask => "index8-mask",
            Self::Index8Shrn => "index8-shrn",
        }
    }

    const fn wide(self) -> bool {
        matches!(self, Self::Find32One | Self::Find32Four)
    }

    const fn le(self) -> bool {
        matches!(self, Self::Find8LeMin | Self::Find8LeSubs)
    }

    const fn index(self) -> bool {
        matches!(self, Self::Index8Mask | Self::Index8Shrn)
    }

    /// What a search that found nothing returns.
    const fn miss(self) -> i64 {
        if self.index() { -1 } else { 0 }
    }

    const fn elem(self) -> &'static str {
        if self.wide() { "unsigned int" } else { "unsigned char" }
    }

    /// A value no element has, which is also what is planted at each position in turn.
    ///
    /// Each is on the far side of the sign bit from the data, so a loop that compared signed
    /// where it had to compare unsigned finds it. For the at-or-below searches it is below
    /// every element, so the planted one is the only match.
    const fn absent(self, high: bool) -> u32 {
        match (self.wide(), self.le(), high) {
            (true, _, false) => 0x8000_0001,
            (true, _, true) => 1,
            (false, false, false) => 0xc8,
            (false, false, true) => 0xff,
            (false, true, false) => 0,
            (false, true, true) => 0x7f,
        }
    }

    /// What the search returns over these elements, worked out one element at a time.
    fn scalar(self, key: u32, elems: &[u32]) -> i64 {
        let hit = |e: u32| if self.le() { e <= key } else { e == key };
        let first = elems.iter().position(|&e| hit(e));
        match (self.index(), first) {
            (true, Some(at)) => i64::try_from(at).unwrap_or(i64::MAX),
            (false, Some(_)) => 1,
            (_, None) => self.miss(),
        }
    }
}

/// The lengths a `simd-lfind-neon` case searches, which are the ones `simd-lfind` does.
fn neon_lengths() -> Vec<usize> {
    super::lfind_lengths()
}

/// The elements a `simd-lfind-neon` case fills its array with, as `simd-lfind` does.
fn neon_elems(search: Neon, high: bool) -> Vec<u32> {
    let mut x = PG_SEED;
    (0..super::LFIND_ELEMS)
        .map(|_| {
            x = lcg(x);
            let t = x >> 16;
            match (search.wide(), high) {
                (false, false) => 1 + t % 100,
                (false, true) => 0x80 + t % 127,
                (true, false) => 1 + (x >> 8) % 1_000_000,
                (true, true) => 0x8000_0000 + (x >> 8) % 1_000_000,
            }
        })
        .collect()
}

/// The C that fills the array, the same as [`neon_elems`].
const fn neon_fill(search: Neon, high: bool) -> &'static str {
    match (search.wide(), high) {
        (false, false) => "elems[i] = (unsigned char)(1 + (x >> 16) % 100);",
        (false, true) => "elems[i] = (unsigned char)(0x80 + (x >> 16) % 127);",
        (true, false) => "elems[i] = 1 + (x >> 8) % 1000000;",
        (true, true) => "elems[i] = 0x80000000u + (x >> 8) % 1000000;",
    }
}

/// The NEON search loops from `port/simd.h` and `port/pg_lfind.h`, against scalar loops.
///
/// Each program searches every length for keys at the start, the middle and the end and one
/// that is absent, then plants the absent value at every position of every length in turn and
/// searches for it, so every lane of every vector and every position of every tail is the one
/// that has to be found at some point. `offset` makes every load an unaligned one.
fn simd_lfind_neon(sink: &mut Sink<'_>) {
    const VALUES: &[&str] = &["low", "high"];
    const OFFSETS: &[usize] = &[0, 1, 3];
    for search in Neon::ALL {
        for &values in VALUES {
            for &offset in OFFSETS {
                if !sink.wants(Facet::SimdLfindNeon) {
                    return;
                }
                let program = neon_program(search, values == "high", offset);
                let axes = Axes::of([
                    ("search", search.name()),
                    ("values", values),
                    ("offset", &offset.to_string()),
                ]);
                sink.push_tagged(
                    Facet::SimdLfindNeon,
                    axes,
                    Dialect::C17,
                    program,
                    &["gnu", "headers", AARCH64, PROVENANCE],
                );
            }
        }
    }
}

/// Writes `search_simd`, the NEON loop for one search, and its scalar tail.
fn neon_search(program: &mut Program, search: Neon) {
    let elem = search.elem();
    if search == Neon::Index8Mask {
        // vector8_highbit_mask from port/simd.h.
        program.top("static unsigned int highbit_mask(const uint8x16_t v) {");
        program.top("    static const unsigned char mask[16] = {");
        program.top("        1 << 0, 1 << 1, 1 << 2, 1 << 3, 1 << 4, 1 << 5, 1 << 6, 1 << 7,");
        program.top("        1 << 0, 1 << 1, 1 << 2, 1 << 3, 1 << 4, 1 << 5, 1 << 6, 1 << 7,");
        program.top("    };");
        program.top(
            "    uint8x16_t masked = vandq_u8(vld1q_u8(mask), (uint8x16_t) vshrq_n_s8((int8x16_t) v, 7));",
        );
        program.top("    uint8x16_t maskedhi = vextq_u8(masked, masked, 8);");
        program
            .top("    return (unsigned int) vaddvq_u16((uint16x8_t) vzip1q_u8(masked, maskedhi));");
        program.top("}");
    }
    program.top(format!(
        "static int search_simd({elem} key, const {elem} *base, unsigned int nelem) {{"
    ));
    program.top("    unsigned int i = 0;");
    let found = "            return 1;";
    match search {
        Neon::Find32One => {
            program.top("    const uint32x4_t keys = vdupq_n_u32(key);");
            program.top("    const unsigned int tail_idx = nelem & ~(4u - 1);");
            program.top("    for (i = 0; i < tail_idx; i += 4) {");
            program.top("        const uint32x4_t vals = vld1q_u32(&base[i]);");
            // vector32_is_highbit_set, which is vector8_is_highbit_set through a cast.
            program.top("        if (vmaxvq_u8((uint8x16_t) vceqq_u32(keys, vals)) > 0x7f) {");
            program.top(found);
        }
        Neon::Find32Four => {
            program.top("    const unsigned int nelem_per_vector = 4;");
            program.top("    const unsigned int nelem_per_iteration = 4 * nelem_per_vector;");
            program.top("    const unsigned int tail_idx = nelem & ~(nelem_per_iteration - 1);");
            program.top("    const uint32x4_t keys = vdupq_n_u32(key);");
            program.top("    for (i = 0; i < tail_idx; i += nelem_per_iteration) {");
            for k in 1..=4 {
                program.top(format!(
                    "        const uint32x4_t vals{k} = vld1q_u32(&base[i + {}]);",
                    4 * (k - 1)
                ));
            }
            for k in 1..=4 {
                program
                    .top(format!("        const uint32x4_t result{k} = vceqq_u32(keys, vals{k});"));
            }
            program.top("        const uint32x4_t tmp1 = vorrq_u32(result1, result2);");
            program.top("        const uint32x4_t tmp2 = vorrq_u32(result3, result4);");
            program.top("        const uint32x4_t result = vorrq_u32(tmp1, tmp2);");
            program.top("        if (vmaxvq_u8((uint8x16_t) result) > 0x7f) {");
            program.top(found);
        }
        Neon::Find8Two => {
            program.top("    const uint8x16_t keys = vdupq_n_u8(key);");
            program.top("    const unsigned int tail_idx = nelem & ~(32u - 1);");
            program.top("    for (i = 0; i < tail_idx; i += 32) {");
            program.top("        const uint8x16_t chunk1 = vld1q_u8(&base[i]);");
            program.top("        const uint8x16_t chunk2 = vld1q_u8(&base[i + 16]);");
            program.top(
                "        const uint8x16_t result = vorrq_u8(vceqq_u8(chunk1, keys), vceqq_u8(chunk2, keys));",
            );
            program.top("        if (vmaxvq_u8(result) > 0x7f) {");
            program.top(found);
        }
        _ => {
            program.top("    const uint8x16_t keys = vdupq_n_u8(key);");
            program.top("    const unsigned int tail_idx = nelem & ~(16u - 1);");
            program.top("    for (i = 0; i < tail_idx; i += 16) {");
            program.top("        const uint8x16_t chunk = vld1q_u8(&base[i]);");
            match search {
                Neon::Find8 => {
                    program.top("        if (vmaxvq_u8(vceqq_u8(chunk, keys)) > 0x7f) {");
                    program.top(found);
                }
                // vector8_has_le on NEON: the smallest byte is at or below the key.
                Neon::Find8LeMin => {
                    program.top("        if (vminvq_u8(chunk) <= key) {");
                    program.top(found);
                }
                Neon::Find8LeSubs => {
                    program.top(
                        "        if (vmaxvq_u8(vceqq_u8(vqsubq_u8(chunk, keys), vdupq_n_u8(0))) > 0x7f) {",
                    );
                    program.top(found);
                }
                Neon::Index8Mask => {
                    program.top(
                        "        const unsigned int mask = highbit_mask(vceqq_u8(chunk, keys));",
                    );
                    program.top("        if (mask != 0) {");
                    program.top("            return (int)(i + (unsigned int)__builtin_ctz(mask));");
                }
                _ => {
                    program.top(
                        "        const uint8x8_t narrowed = vshrn_n_u16(vreinterpretq_u16_u8(vceqq_u8(chunk, keys)), 4);",
                    );
                    program.top(
                        "        const unsigned long long mask = vget_lane_u64(vreinterpret_u64_u8(narrowed), 0);",
                    );
                    program.top("        if (mask != 0) {");
                    program.top(
                        "            return (int)(i + ((unsigned int)__builtin_ctzll(mask) >> 2));",
                    );
                }
            }
        }
    }
    program.top("        }");
    program.top("    }");
    let compare = if search.le() { "base[i] <= key" } else { "base[i] == key" };
    let hit = if search.index() { "            return (int)i;" } else { found };
    program.top("    for (; i < nelem; i++) {");
    program.top(format!("        if ({compare}) {{"));
    program.top(hit);
    program.top("        }");
    program.top("    }");
    program.top(format!("    return {};", search.miss()));
    program.top("}");
    program.top(format!(
        "static int search_scalar({elem} key, const {elem} *base, unsigned int nelem) {{"
    ));
    program.top("    for (unsigned int i = 0; i < nelem; i++) {");
    program.top(format!("        if ({compare}) {{"));
    program.top(if search.index() {
        "            return (int)i;"
    } else {
        "            return 1;"
    });
    program.top("        }");
    program.top("    }");
    program.top(format!("    return {};", search.miss()));
    program.top("}");
}

/// One `simd-lfind-neon` case.
fn neon_program(search: Neon, high: bool, offset: usize) -> Program {
    let elem = search.elem();
    let mut program = Program::new(format!(
        "NEON {} over {} values starting {offset} elements in, against a scalar loop",
        search.name(),
        if high { "high" } else { "low" }
    ));
    program.include("arm_neon.h");
    program.top(format!("static {elem} elems[{}];", super::LFIND_ELEMS + 16));
    neon_search(&mut program, search);

    let miss = search.miss();
    program.top("static int found[5];");
    program.top("static unsigned long long pattern;");
    program.top("static int mismatches;");
    program.top("static void tally(int k, int got, int want) {");
    program.top(format!("    found[k] = found[k] + (got != {miss});"));
    program.top("    pattern = pattern * 31u + (unsigned long long)(got + 1);");
    program.top("    if (got != want) {");
    program.top("        mismatches = mismatches + 1;");
    program.top("    }");
    program.top("}");

    let lengths = neon_lengths();
    let list = lengths.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ");
    program.top(format!("static const unsigned int lengths[{}] = {{ {list} }};", lengths.len()));

    let absent = search.absent(high);
    program.input(Ty::U32, "seed", i128::from(PG_SEED));
    program.line("unsigned int x = seed;");
    program.line(format!("for (int i = 0; i < {}; i++) {{", super::LFIND_ELEMS));
    program.line_at(1, LCG_C);
    program.line_at(1, neon_fill(search, high));
    program.line("}");
    program.line(format!("{elem} *base = elems + {offset};"));
    program.line(format!("for (int n = 0; n < {}; n++) {{", lengths.len()));
    program.line_at(1, "unsigned int len = lengths[n];");
    program.line_at(1, format!("{elem} keys[4];"));
    // As in simd-lfind: elements at the start, the middle and the end, or one less than them
    // for the at-or-below searches, and one that is absent.
    let less = if search.le() { " - 1" } else { "" };
    program.line_at(1, format!("keys[0] = ({elem})(base[0]{less});"));
    program.line_at(1, format!("keys[1] = ({elem})(base[len / 2]{less});"));
    program.line_at(1, format!("keys[2] = ({elem})(base[len > 0 ? len - 1 : 0]{less});"));
    program.line_at(1, format!("keys[3] = {absent:#x};"));
    program.line_at(1, "for (int k = 0; k < 4; k++) {");
    program.line_at(
        2,
        "tally(k, search_simd(keys[k], base, len), search_scalar(keys[k], base, len));",
    );
    program.line_at(1, "}");
    // The absent value at every position in turn, so it is the only match and it is there.
    program.line_at(1, "for (unsigned int p = 0; p < len; p++) {");
    program.line_at(2, format!("{elem} saved = base[p];"));
    program.line_at(2, format!("base[p] = {absent:#x};"));
    program.line_at(
        2,
        format!(
            "tally(4, search_simd({absent:#x}, base, len), search_scalar({absent:#x}, base, len));"
        ),
    );
    program.line_at(2, "base[p] = saved;");
    program.line_at(1, "}");
    program.line("}");
    program.blank();

    let mut all = neon_elems(search, high);
    let mask = if search.wide() { u32::MAX } else { 0xff };
    let mut found = [0i128; 5];
    let mut pattern = 0u64;
    let mut tally = |k: usize, got: i64| {
        found[k] += i128::from(got != miss);
        pattern = pattern.wrapping_mul(31).wrapping_add(u64::try_from(got + 1).unwrap_or(0));
    };
    for &len in &lengths {
        let elems = &mut all[offset..];
        let at = |index: usize| {
            if search.le() { elems[index].wrapping_sub(1) & mask } else { elems[index] }
        };
        let keys = [at(0), at(len / 2), at(len.saturating_sub(1)), absent];
        for (k, &key) in keys.iter().enumerate() {
            tally(k, search.scalar(key, &elems[..len]));
        }
        for p in 0..len {
            let saved = elems[p];
            elems[p] = absent;
            tally(4, search.scalar(absent, &elems[..len]));
            elems[p] = saved;
        }
    }
    for (k, count) in found.iter().enumerate() {
        program.check(Ty::I32, &format!("found[{k}]"), *count);
    }
    program.check(Ty::U64, "pattern", i128::from(pattern));
    program.check(Ty::I32, "mismatches", 0);
    program
}

#[cfg(test)]
mod tests {
    use super::{AARCH64, Neon, neon_elems};
    use crate::{Options, Sink};
    use corpus_model::{Case, Facet};

    fn cases() -> Vec<Case> {
        let opts = Options::all();
        let mut sink = Sink::new(&opts);
        super::generate(&mut sink);
        sink.into_cases()
    }

    #[test]
    fn every_case_is_tagged_for_aarch64_and_only_the_march_ones_carry_the_flag() {
        let cases = cases();
        assert_eq!(cases.iter().filter(|case| case.facet == Facet::Crc32cArmv8).count(), 72);
        assert_eq!(cases.iter().filter(|case| case.facet == Facet::SimdLfindNeon).count(), 48);
        for case in &cases {
            assert!(case.has_tag(AARCH64), "{}", case.id);
            assert!(case.has_tag("provenance:postgres"), "{}", case.id);
            assert!(!case.has_tag("x86-64"), "{}", case.id);
            let march = case.axes.get("setup").is_some_and(|setup| setup.starts_with("march"));
            assert_eq!(case.flags == ["-march=armv8-a+crc"], march, "{}", case.id);
            assert_eq!(!case.flags.is_empty(), march, "{}", case.id);
        }
    }

    #[test]
    fn only_the_attribute_cases_mark_a_function_and_only_the_hwcap_ones_ask() {
        for case in cases().iter().filter(|case| case.facet == Facet::Crc32cArmv8) {
            let setup = case.axes.get("setup").unwrap();
            assert_eq!(
                case.source.contains("target(\"+crc\")"),
                setup == "attribute-hwcap",
                "{}",
                case.id
            );
            assert_eq!(case.source.contains("getauxval(AT_HWCAP)"), setup.ends_with("hwcap"));
        }
    }

    #[test]
    fn the_absent_value_is_absent_and_the_planted_one_is_the_only_match() {
        for search in Neon::ALL {
            for high in [false, true] {
                let elems = neon_elems(search, high);
                let absent = search.absent(high);
                assert!(elems.iter().all(|&e| e != absent), "{search:?}");
                if search.le() {
                    assert!(elems.iter().all(|&e| e > absent), "{search:?}");
                }
                for p in [0, 5, 16, 17, 999] {
                    let mut planted = elems.clone();
                    planted[p] = absent;
                    let want = if search.index() { i64::try_from(p).unwrap() } else { 1 };
                    assert_eq!(search.scalar(absent, &planted[..1000]), want, "{search:?} {p}");
                }
            }
        }
    }

    #[test]
    fn every_key_kind_is_found_somewhere_and_missed_somewhere() {
        for case in cases().iter().filter(|case| case.facet == Facet::SimdLfindNeon) {
            let lines: Vec<i128> =
                case.expect.text().lines().map(|line| line.parse().unwrap()).collect();
            // found[0] to found[2], the absent key, the planted one, the pattern, the mismatches.
            assert!(lines[..3].iter().all(|&n| n > 0), "{}", case.id);
            assert_eq!(lines[3], 0, "{}", case.id);
            assert!(lines[4] > 1000, "{}", case.id);
            assert_eq!(lines[6], 0, "{}", case.id);
        }
    }
}
