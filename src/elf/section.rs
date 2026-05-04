use binlayout::BinLayout;
pub type SecIdxType = u16;
pub mod sec_idx {
    use super::SecIdxType;
    pub const SHN_UNDEF: SecIdxType = 0;
    pub const SHN_LORESERVE: SecIdxType = 0xff00; // if    e_shnum is >= than this value the field is zero       and the null sh contains the real value in sh_size
    // if e_shstrndx is >= than this value the field is SHN_XINDEX and the null sh contains the real value in sh_link

    /* Technically psABIs could define additional values here  */
    pub const SHN_HIPROC: SecIdxType = 0xff1f;
    pub const SHN_LOOS: SecIdxType = 0xff20;
    /* Technically OS ABIs could define additional values here  */
    pub const SHN_HIOS: SecIdxType = 0xff3f;
    pub const SHN_ABS: SecIdxType = 0xfff1; // Symbols relative to this section are absolute and not relocatable
    pub const SHN_COMMON: SecIdxType = 0xfff2; // Symbols realitve to this section are common symbols (FORTRAN COMMON or unalloc extern C vars)
    pub const SHN_XINDEX: SecIdxType = 0xffff;
}

#[derive(Debug)]
pub struct ElfSecHeader {
    // Index into the section header string table section.
    pub name_idx: u32,

    /// Type of section (e.g., SHT_PROGBITS, SHT_SYMTAB, etc.).
    // pub section_type: ElfSectionType, // FIXME

    /// Flags describing section properties (e.g., SHF_WRITE, SHF_ALLOC).
    pub flags: u64,

    /// If the section is in the memory image of a process, this is the first address.
    pub address: u64,

    /// Section is stored at `<offset>` from the beginning of this file.
    pub offset: u64,

    /// Size of the section in bytes.
    pub size: u64,

    /// Index in the section header table of an associated section (e.g., for linking).
    pub link: u32,

    /// Additional section information (depends on section type).
    pub info: u32,

    /// Alignment constraints of the address field.
    pub alignment: u64,

    /// If the section is a table of fixed-size entries, this is the size of one entry.
    pub entry_size: u64,
}
pub mod section {
    pub type ElfSectionType = u32;
    pub mod SHT {
        use super::ElfSectionType;

        pub const NULL: ElfSectionType = 0;
        pub const PROGBITS: ElfSectionType = 1;
        pub const SYMTAB: ElfSectionType = 2;
        pub const STRTAB: ElfSectionType = 3;
        pub const RELA: ElfSectionType = 4;
        pub const HASH: ElfSectionType = 5;
        pub const DYNAMIC: ElfSectionType = 6;
        pub const NOTE: ElfSectionType = 7;
        pub const NOBITS: ElfSectionType = 8;
        pub const REL: ElfSectionType = 9;
        pub const SHLIB: ElfSectionType = 10;
        pub const DYNSYM: ElfSectionType = 11;
        pub const INIT_ARRAY: ElfSectionType = 14;
        pub const FINI_ARRAY: ElfSectionType = 15;
        pub const PREINIT_ARRAY: ElfSectionType = 16;
        pub const GROUP: ElfSectionType = 17;
        pub const SYMTAB_SHNDX: ElfSectionType = 18;
        pub const RELR: ElfSectionType = 19;
        pub const LOOS: ElfSectionType = 0x60000000;

        /* You may add your application's OS-specific types here */

        pub const HIOS: ElfSectionType = 0x6fffffff;
        pub const LOPROC: ElfSectionType = 0x70000000;

        /* You may add your application's processor-specific types here */

        pub const HIPROC: ElfSectionType = 0x7fffffff;
        pub const LOUSER: ElfSectionType = 0x80000000;
        pub const HIUSER: ElfSectionType = 0xffffffff;
    }

    pub type ElfSectionFlag = u64;
    pub mod SHF {
        use super::ElfSectionFlag;

        pub const WRITE: ElfSectionFlag = 0x1;
        pub const ALLOC: ElfSectionFlag = 0x2;
        pub const EXECINSTR: ElfSectionFlag = 0x4;
        pub const MERGE: ElfSectionFlag = 0x10;
        pub const STRINGS: ElfSectionFlag = 0x20;
        pub const INFO_LINK: ElfSectionFlag = 0x40;
        pub const LINK_ORDER: ElfSectionFlag = 0x80;
        pub const OS_NONCONFORMING: ElfSectionFlag = 0x100;
        pub const GROUP: ElfSectionFlag = 0x200;
        pub const TLS: ElfSectionFlag = 0x400;
        pub const COMPRESSED: ElfSectionFlag = 0x800;
        pub const MASKOS: ElfSectionFlag = 0x0ff00000;

        /* You may add your application's OS-specific flags here */

        pub const MASKPROC: ElfSectionFlag = 0xf0000000;

        /* You may add your application's processor-specific flags here */
    }

    pub type SecGrpFlag = u64;
    pub mod GRP {
        use super::SecGrpFlag;

        pub const COMDAT: SecGrpFlag = 0x1;
        pub const MASKOS: SecGrpFlag = 0x0ff00000;

        /* You may add your application's OS-specific flags here */

        pub const MASKPROC: SecGrpFlag = 0xf0000000;

        /* You may add your application's processor-specific flags here */
    }

    /// Abstract representation of an ELF section header.
    pub struct ElfSecHeader {
        /// Index into the section header string table section.
        pub name_idx: u32,

        /// Type of section (e.g., SHT_PROGBITS, SHT_SYMTAB, etc.).
        pub section_type: ElfSectionType,

        /// Flags describing section properties (e.g., SHF_WRITE, SHF_ALLOC).
        pub flags: u64,

        /// If the section is in the memory image of a process, this is the first address.
        pub address: u64,

        /// Section is stored at `<offset>` from the beginning of this file.
        pub offset: u64,

        /// Size of the section in bytes.
        pub size: u64,

        /// Index in the section header table of an associated section (e.g., for linking).
        pub link: u32,

        /// Additional section information (depends on section type).
        pub info: u32,

        /// Alignment constraints of the address field.
        pub alignment: u64,

        /// If the section is a table of fixed-size entries, this is the size of one entry.
        pub entry_size: u64,
    }
}

#[repr(C)]
#[derive(BinLayout)]
pub struct Elf32SecHdr {
    pub sh_name: u32, // Index into the section header string table section
    pub sh_type: u32, // Type of section
    pub sh_flags: u32,
    pub sh_addr: u32, // If the section is in the memory img of a process this is the first address
    pub sh_offset: u32, // Section is stored at <offset> from the begining of this file
    pub sh_size: u32, // Size of the section
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u32, // Alignment constraints of Address field
    pub sh_entsize: u32,   // If the section is a table of fixed-size entries this is the entry size
}
#[repr(C)]
#[derive(BinLayout)]
pub struct Elf64SecHdr {}
