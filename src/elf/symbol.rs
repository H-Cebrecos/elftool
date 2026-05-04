pub mod symbol {
    pub type ElfSymbolType = u8;
    pub mod STT {
        use super::ElfSymbolType;

        pub const NOTYPE: ElfSymbolType = 0;
        pub const OBJECT: ElfSymbolType = 1;
        pub const FUNC: ElfSymbolType = 2;
        pub const SECTION: ElfSymbolType = 3;
        pub const FILE: ElfSymbolType = 4;
        pub const COMMON: ElfSymbolType = 5;
        pub const TLS: ElfSymbolType = 6;
        pub const LOOS: ElfSymbolType = 10;
        pub const HIOS: ElfSymbolType = 12;
        pub const LOPROC: ElfSymbolType = 13;
        pub const HIPROC: ElfSymbolType = 15;
    }

    pub type ElfSymbolBind = u8;
    pub mod STB {
        use super::ElfSymbolBind;

        pub const LOCAL: ElfSymbolBind = 0;
        pub const GLOBAL: ElfSymbolBind = 1;
        pub const WEAK: ElfSymbolBind = 2;
        pub const LOOS: ElfSymbolBind = 10;
        pub const HIOS: ElfSymbolBind = 12;
        pub const LOPROC: ElfSymbolBind = 13;
        pub const HIPROC: ElfSymbolBind = 15;
    }

    pub type ElfSymbolVis = u8;
    pub mod STV {
        use super::ElfSymbolVis;

        pub const DEFAULT: ElfSymbolVis = 0;
        pub const INTERNAL: ElfSymbolVis = 1;
        pub const HIDDEN: ElfSymbolVis = 2;
        pub const PROTECTED: ElfSymbolVis = 3;
        pub const EXPORTED: ElfSymbolVis = 4;
        pub const SINGLETON: ElfSymbolVis = 5;
        pub const ELIMINATE: ElfSymbolVis = 6;
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ElfSymbolAttr {
        Default,
        Abs,
        Common,
        Undef,
        Unknown,
    }

    pub struct ElfSymTabEntry {
        pub name_idx: u32,
        pub sym_type: ElfSymbolType,
        pub binding: ElfSymbolBind,
        pub visib: ElfSymbolVis,
        pub attr: ElfSymbolAttr,
        pub sec_idx: u64,
        pub value: u64,
        pub size: u64,
    }
}
