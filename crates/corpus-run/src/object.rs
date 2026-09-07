//! How big the code is.
//!
//! The code quality claim in spec 16 is about the code the optimizer produced, so the number
//! that backs it has to be the size of the code and nothing else. The size of the executable
//! on disk is mostly the C runtime, the symbol table, the debug sections and whatever padding
//! the linker felt like: comparing on that measures the linker, and the linker is the same one
//! for every toolchain under test, so the difference it hides is exactly the difference we
//! care about.
//!
//! So the text section is read out of the object file directly. Shelling out to `size` would
//! be shorter, but `size` is not installed everywhere, its output format differs between the
//! GNU and the BSD versions, and a missing tool would silently turn the headline number into
//! nought. Two hundred lines of header reading is a better trade.
//!
//! ELF and Mach-O are both here because the corpus is developed on macOS and runs in CI on
//! Linux, and a number that only exists on one of them is a number nobody trusts.
//!
//! The code is the headline, but it is not the only thing the compiler decided. Three numbers
//! come back rather than one, because they answer different questions and adding them together
//! answers none of them. Code is what the optimizer emitted. Data is what it had to put in the
//! image to support that code, which is where a compiler that unrolls by materializing a table
//! shows up and where one that folds a constant does not. Zero filled data costs nothing on
//! disk and costs pages at run time, so it belongs in neither of the other two.

/// How much of each kind of thing an executable holds.
///
/// All three are sizes in the running image rather than sizes on disk. The file on disk also
/// holds the symbol table, the debug sections and the padding the linker chose, none of which
/// the optimizer decided, so it is recorded separately and never mixed in here.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Sizes {
    /// Allocated sections holding instructions.
    pub text: u64,
    /// Allocated sections holding initialized data, read only or writable.
    pub data: u64,
    /// Allocated sections that occupy no space in the file, which is to say `.bss`.
    pub bss: u64,
}

/// The size of the executable code in an object file or an executable.
///
/// Returns `None` when the format is not one of the two that are understood, or when the file
/// is truncated. The caller records nought and the report says the number is missing, which is
/// better than reporting a wrong size confidently.
#[must_use]
pub fn text_size(bytes: &[u8]) -> Option<u64> {
    Some(sizes(bytes)?.text)
}

/// Every section size in an object file or an executable, grouped three ways.
///
/// Returns `None` on the same terms as [`text_size`], and for the same reason: a size read out
/// of a file the reader did not understand is worse than no size at all.
#[must_use]
pub fn sizes(bytes: &[u8]) -> Option<Sizes> {
    if bytes.starts_with(b"\x7fELF") {
        return elf_sizes(bytes);
    }
    match bytes.get(..4) {
        // Mach-O, sixty four bit, in both byte orders. Thirty two bit Mach-O is not handled
        // because no target we build for uses it.
        Some([0xcf, 0xfa, 0xed, 0xfe]) => macho_sizes(bytes, false),
        Some([0xfe, 0xed, 0xfa, 0xcf]) => macho_sizes(bytes, true),
        _ => None,
    }
}

/// Reads the size of every allocated section in an ELF file, sorted into the three groups.
///
/// Sections are grouped by their flags rather than by their names, because a compiler is free
/// to put code in `.text.startup` or in a section of its own naming, and a measurement that
/// only counted the section called `.text` would flatter whichever compiler splits its output
/// up. The same argument applies to `.rodata.cst8` and to `.data.rel.ro`.
fn elf_sizes(bytes: &[u8]) -> Option<Sizes> {
    let sixty_four = *bytes.get(4)? == 2;
    let big = *bytes.get(5)? == 2;
    let read = Reader { bytes, big };

    let (shoff, shentsize, shnum) = if sixty_four {
        (read.u64(0x28)?, read.u16(0x3a)? as usize, read.u16(0x3c)? as usize)
    } else {
        (u64::from(read.u32(0x20)?), read.u16(0x2e)? as usize, read.u16(0x30)? as usize)
    };
    let shoff = usize::try_from(shoff).ok()?;
    if shentsize == 0 {
        return None;
    }

    // SHF_ALLOC is bit one and SHF_EXECINSTR is bit two, and SHT_NOBITS is the section type
    // that takes up room in memory and none in the file.
    const ALLOC: u64 = 0x2;
    const EXEC: u64 = 0x4;
    const NOBITS: u32 = 8;
    let mut total = Sizes::default();
    for index in 0..shnum {
        let at = shoff.checked_add(index.checked_mul(shentsize)?)?;
        let (kind, flags, size) = if sixty_four {
            (read.u32(at + 4)?, read.u64(at + 8)?, read.u64(at + 32)?)
        } else {
            (read.u32(at + 4)?, u64::from(read.u32(at + 8)?), u64::from(read.u32(at + 20)?))
        };
        if flags & ALLOC == 0 {
            continue;
        }
        let bucket = if flags & EXEC != 0 {
            &mut total.text
        } else if kind == NOBITS {
            &mut total.bss
        } else {
            &mut total.data
        };
        *bucket = bucket.checked_add(size)?;
    }
    Some(total)
}

/// Reads every section of every loadable segment in a Mach-O file, sorted into the same three
/// groups.
///
/// `__LINKEDIT` is skipped because it is the symbol table and the relocations rather than
/// anything the program can address, and `__PAGEZERO` is skipped because it holds nothing at
/// all. Both would otherwise land in the data column and swamp it.
fn macho_sizes(bytes: &[u8], big: bool) -> Option<Sizes> {
    const LC_SEGMENT_64: u32 = 0x19;
    const HEADER: usize = 32;
    const SEGMENT_COMMAND: usize = 72;
    const SECTION: usize = 80;
    // S_ATTR_PURE_INSTRUCTIONS is 0x8000_0000 and S_ATTR_SOME_INSTRUCTIONS is 0x0000_0400,
    // both in the top byte of the section flags word.
    const INSTRUCTIONS: u32 = 0x8000_0000 | 0x0000_0400;
    // The low byte of the same word is the section type. Three of them are zero filled.
    const TYPE: u32 = 0xff;
    const ZEROFILL: [u32; 3] = [0x1, 0xc, 0x12];

    let read = Reader { bytes, big };
    let ncmds = read.u32(16)? as usize;
    let mut at = HEADER;
    let mut total = Sizes::default();
    for _ in 0..ncmds {
        let cmd = read.u32(at)?;
        let cmdsize = read.u32(at + 4)? as usize;
        if cmdsize < 8 {
            return None;
        }
        if cmd == LC_SEGMENT_64 && !skipped_segment(bytes.get(at + 8..at + 24)?) {
            let nsects = read.u32(at + 64)? as usize;
            for index in 0..nsects {
                let sect = at.checked_add(SEGMENT_COMMAND.checked_add(index * SECTION)?)?;
                let size = read.u64(sect + 40)?;
                let flags = read.u32(sect + 64)?;
                let bucket = if flags & INSTRUCTIONS != 0 {
                    &mut total.text
                } else if ZEROFILL.contains(&(flags & TYPE)) {
                    &mut total.bss
                } else {
                    &mut total.data
                };
                *bucket = bucket.checked_add(size)?;
            }
        }
        at = at.checked_add(cmdsize)?;
    }
    Some(total)
}

/// Whether a segment name is one of the two that hold nothing the program runs on.
fn skipped_segment(name: &[u8]) -> bool {
    let end = name.iter().position(|b| *b == 0).unwrap_or(name.len());
    matches!(&name[..end], b"__LINKEDIT" | b"__PAGEZERO")
}

/// Pulls fixed width numbers out of a byte slice in whichever order the file uses.
struct Reader<'a> {
    bytes: &'a [u8],
    big: bool,
}

impl Reader<'_> {
    fn u16(&self, at: usize) -> Option<u16> {
        let raw: [u8; 2] = self.bytes.get(at..at + 2)?.try_into().ok()?;
        Some(if self.big { u16::from_be_bytes(raw) } else { u16::from_le_bytes(raw) })
    }

    fn u32(&self, at: usize) -> Option<u32> {
        let raw: [u8; 4] = self.bytes.get(at..at + 4)?.try_into().ok()?;
        Some(if self.big { u32::from_be_bytes(raw) } else { u32::from_le_bytes(raw) })
    }

    fn u64(&self, at: usize) -> Option<u64> {
        let raw: [u8; 8] = self.bytes.get(at..at + 8)?.try_into().ok()?;
        Some(if self.big { u64::from_be_bytes(raw) } else { u64::from_le_bytes(raw) })
    }
}

#[cfg(test)]
mod tests {
    use super::{sizes, text_size};

    #[test]
    fn something_that_is_not_an_object_file_is_refused_rather_than_guessed_at() {
        assert_eq!(text_size(b""), None);
        assert_eq!(text_size(b"int main(void) { return 0; }"), None);
        assert_eq!(text_size(&[0; 64]), None);
    }

    #[test]
    fn a_truncated_object_file_is_refused_rather_than_read_off_the_end() {
        let mut header = vec![0x7f, b'E', b'L', b'F', 2, 1];
        header.resize(20, 0);
        assert_eq!(text_size(&header), None);
    }

    #[test]
    fn something_that_is_not_an_object_file_has_no_sizes_either() {
        assert_eq!(sizes(b"int main(void) { return 0; }"), None);
    }

    #[test]
    fn the_code_of_this_very_binary_can_be_measured() {
        // The test binary was built by the same rust toolchain that is running the test, so
        // it is in whichever of the two formats this machine uses. If the reader cannot size
        // it, the reader is broken on the platform the corpus is being developed on, and no
        // amount of synthetic fixtures would have told us that.
        let path = std::env::current_exe().unwrap();
        let bytes = std::fs::read(path).unwrap();
        let size = text_size(&bytes).expect("this platform is neither ELF nor Mach-O");
        assert!(size > 4096, "the test binary claims to hold {size} bytes of code");
        assert!(size < bytes.len() as u64, "the code cannot be larger than the file it is in");
    }

    #[test]
    fn every_part_of_this_very_binary_can_be_measured_and_the_parts_agree_with_the_whole() {
        // Same argument as the test above. A grouping that is wrong on the platform the corpus
        // is developed on is a grouping no fixture would have caught, because the fixture
        // would have been written from the same misreading of the format.
        let bytes = std::fs::read(std::env::current_exe().unwrap()).unwrap();
        let parts = sizes(&bytes).expect("this platform is neither ELF nor Mach-O");
        assert_eq!(parts.text, text_size(&bytes).unwrap(), "the two readers disagree");
        assert!(parts.data > 0, "a rust binary holds string literals and a panic table");
        let image = parts.text + parts.data + parts.bss;
        assert!(image > parts.text, "grouping put everything in one bucket");
        assert!(
            parts.text + parts.data < bytes.len() as u64,
            "what is in the file cannot be larger than the file"
        );
    }
}
