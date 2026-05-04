use core::fmt::Display;
use binlayout::BinLayout;

pub const ELF_MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];

#[repr(C)]
#[derive(Debug, BinLayout)]
pub struct ElfInfo {
    pub magic: [u8; 4],
    pub ei_class: u8,
    pub ei_data: u8,
    pub ei_version: u8,
    pub ei_os_abi: u8,
    pub ei_abi_version: u8,
    pub pad: [u8; 7],
}

const _: () = {
    assert!(core::mem::align_of::<ElfInfo>() == 1);
};

#[derive(Debug, PartialEq)]
pub enum InfoClass {
    None,
    Class32,
    Class64,
    Reserved(u8),
}
impl InfoClass {
    pub const NONE: u8 = 0;
    pub const CLASS_32: u8 = 1;
    pub const CLASS_64: u8 = 2;
}
impl From<u8> for InfoClass {
    fn from(value: u8) -> Self {
        match value {
            InfoClass::NONE => Self::None,
            InfoClass::CLASS_32 => Self::Class32,
            InfoClass::CLASS_64 => Self::Class64,
            _ => Self::Reserved(value),
        }
    }
}
impl Display for InfoClass {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            InfoClass::None => write!(f, "Invalid"),
            InfoClass::Class32 => write!(f, "ELF32"),
            InfoClass::Class64 => write!(f, "ELF64"),
            InfoClass::Reserved(x) => write!(f, "Unknown value {x}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum InfoData {
    None,
    Little,
    Big,
    Reserved(u8),
}
impl InfoData {
    pub const NONE: u8 = 0;
    pub const LSB: u8 = 1;
    pub const MSB: u8 = 2;
}
impl From<u8> for InfoData {
    fn from(value: u8) -> Self {
        match value {
            InfoData::NONE => Self::None,
            InfoData::LSB => Self::Little,
            InfoData::MSB => Self::Big,
            _ => Self::Reserved(value),
        }
    }
}
impl Display for InfoData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            InfoData::None => write!(f, "Invalid"),
            InfoData::Little => write!(f, "2's complement, little endian"),
            InfoData::Big => write!(f, "2's complement, big endian"),
            InfoData::Reserved(x) => write!(f, "Unknown {x}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    None,
    Current,
    Reserved(u32),
}
impl Version {
    pub const NONE: u32 = 0; // Invalid
    pub const CURRENT: u32 = 1;
}
impl From<u32> for Version {
    fn from(value: u32) -> Self {
        match value {
            Version::NONE => Self::None,
            Version::CURRENT => Self::Current,
            _ => Self::Reserved(value),
        }
    }
}
impl From<u8> for Version {
    fn from(value: u8) -> Self {
        match u32::from(value) {
            Version::NONE => Self::None,
            Version::CURRENT => Self::Current,
            _ => Self::Reserved(u32::from(value)),
        }
    }
}
impl Display for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Version::None => write!(f, "None"),
            Version::Current => write!(f, "Current"),
            Version::Reserved(x) => write!(
                f,
                "This value was reserved by the spec for future use ({})",
                x
            ),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum OsABI {
    None,
    Other(u8),
}
impl OsABI {
    pub const NONE: u8 = 0; // This is the default value for most linkers
    /* You may add your application-specific ABIs here */
}
impl From<u8> for OsABI {
    fn from(value: u8) -> Self {
        match value {
            OsABI::NONE => Self::None,
            _ => Self::Other(value),
        }
    }
}
impl Display for OsABI {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            OsABI::None => write!(f, "Default"),
            OsABI::Other(x) => write!(f, "Unknown {x}"),
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum Type {
    None,
    Reloc,
    Exec,
    Dyn,
    Core,
    Reserved(u16),
    Os(u16),
    Proc(u16),
}
impl Type {
    pub const NONE: u16 = 0; // No type
    pub const REL: u16 = 1; // relocatable
    pub const EXEC: u16 = 2; // executable
    pub const DYN: u16 = 3; // shared object
    pub const CORE: u16 = 4; // Core file
    pub const LOOS: u16 = 0xfe00; // start of OS-specific range

    /* You may add your application's OS-specific types here */

    pub const HIOS: u16 = 0xfeff; //   end of OS-specific range
    pub const LOPROC: u16 = 0xff00; // start of Processor-specific range

    /* You may add your application's processor-specific types here */

    pub const HIPROC: u16 = 0xffff; //   end of Processor-specific range
}
impl From<u16> for Type {
    fn from(value: u16) -> Self {
        match value {
            Type::NONE => Self::None,
            Type::REL => Self::Reloc,
            Type::EXEC => Self::Exec,
            Type::DYN => Self::Dyn,
            Type::CORE => Self::Core,
            Type::LOOS..=Type::HIOS => Self::Os(value),
            Type::LOPROC..=Type::HIPROC => Self::Proc(value),
            _ => Self::Reserved(value),
        }
    }
}
impl Display for Type {
    /// Uses readelf-like identification
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Type::None => write!(f, "No Type"),
            Type::Reloc => write!(f, "REL (Relocatable file)"),
            Type::Exec => write!(f, "EXEC (Executable file)"),
            Type::Dyn => write!(f, "DYN (Shared object file)"),
            Type::Core => write!(f, "CORE (Core file)"),
            Type::Os(x) => write!(f, "OS specific value ({})", x),
            Type::Proc(x) => write!(f, "Processor specific value ({})", x),
            Type::Reserved(x) => write!(
                f,
                "Reserved. This value was reserved by the spec for future use({})",
                x
            ),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Machine {
    None,
    Other(u16),
}
impl Machine {
    pub const NONE: u16 = 0;
    /* You may add your application-specific machines here */
}
impl From<u16> for Machine {
    fn from(value: u16) -> Self {
        match value {
            Machine::NONE => Self::None,
            _ => Self::Other(value),
        }
    }
}
impl Display for Machine {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::None => write!(f, "No Machine"),
            Self::Other(x) => write!(f, "Unknown {x}"),
        }
    }
}

#[repr(C)]
#[derive(Debug, BinLayout)]
pub struct Elf32Hdr {
    pub e_type: u16,    // Type of ELF file
    pub e_machine: u16, // Architecture
    pub e_version: u32, // Always 1
    pub e_entry: u32,   // Entry point (virtual address)
    pub e_phoff: u32,   // Offset of program header table in the file (bytes)
    pub e_shoff: u32,   // Offset of section header table in the file (bytes)
    pub e_flags: u32,
    pub e_ehsize: u16,    // This header's size
    pub e_phentsize: u16, // Size of one entry in the program header table
    pub e_phnum: u16,     // Number of entries in the program header table
    pub e_shentsize: u16, // Size of one entry in the section header table
    pub e_shnum: u16,     // Number of entries in the section header table
    pub e_shstrndx: u16, // Index of the entry in the section table that points to the section names
}

#[repr(C)]
#[derive(Debug, BinLayout)]
pub struct Elf64Hdr {
    pub e_type: u16,    // Type of ELF file
    pub e_machine: u16, // Architecture
    pub e_version: u32, // Always 1
    pub e_entry: u64,   // Entry point (virtual address)
    pub e_phoff: u64,   // Offset of program header table in the file (bytes)
    pub e_shoff: u64,   // Offset of section header table in the file (bytes)
    pub e_flags: u32,
    pub e_ehsize: u16,    // This header's size
    pub e_phentsize: u16, // Size of one entry in the program header table
    pub e_phnum: u16,     // Number of entries in the program header table
    pub e_shentsize: u16, // Size of one entry in the section header table
    pub e_shnum: u16,     // Number of entries in the section header table
    pub e_shstrndx: u16, // Index of the entry in the section table that points to the section names
}

/// Abstract representation of the ELF header, it does not represent the real layout, instead provides a uniform view into the data.
/// Abstract representation of the ELF header.
/// Does not represent the real layout, instead provides a uniform view into the data.
#[derive(Debug)]
pub struct ElfHeader {
    /// 32 or 64 bit architecture
    pub ei_class: InfoClass,

    /// Endianness of the architecture
    pub ei_data: InfoData,

    /// Target platform's ABI
    pub ei_os_abi: OsABI,

    /// Target ABI version
    pub ei_abi_ver: Version,

    /// Padding
    pub padding: [u8; 7],

    /// Type of ELF file
    pub elf_type: Type,

    /// Architecture
    pub machine: Machine,

    /// Always 1
    pub version: Version,

    /// Entry point (virtual address)
    pub entry: u64,

    /// Offset of program header table in the file
    pub pro_hdr_off: u64,

    /// Offset of section header table in the file
    pub sec_hdr_off: u64,

    /// Flags
    pub flags: u32,

    /// Size of this header
    pub hdr_size: u16,

    /// Size of one entry in program header table
    pub ph_entry_sz: u16,

    /// Number of entries in program header table
    pub ph_entry_num: u16,

    /// Size of one entry in section header table
    pub sh_entry_sz: u16,

    /// Number of entries in section header table
    pub sh_entry_num: u16,

    /// Index of section name string table
    pub sec_str_idx: u16,
}

impl From<(&Elf32Hdr, &ElfInfo)> for ElfHeader {
    fn from(value: (&Elf32Hdr, &ElfInfo)) -> Self {
        let (hdr, info) = value;
        Self {
            ei_class: info.ei_class.into(),
            ei_data: info.ei_data.into(),
            ei_os_abi: info.ei_os_abi.into(),
            ei_abi_ver: info.ei_abi_version.into(),
            padding: info.pad,
            elf_type: hdr.e_type.into(),
            machine: hdr.e_machine.into(),
            version: hdr.e_version.into(),
            entry: hdr.e_entry as u64,
            pro_hdr_off: hdr.e_phoff as u64,
            sec_hdr_off: hdr.e_shoff as u64,
            flags: hdr.e_flags,
            hdr_size: hdr.e_ehsize,
            ph_entry_sz: hdr.e_phentsize,
            ph_entry_num: hdr.e_phnum,
            sh_entry_sz: hdr.e_shentsize,
            sh_entry_num: hdr.e_shnum,
            sec_str_idx: hdr.e_shstrndx,
        }
    }
}
impl From<(&Elf64Hdr, &ElfInfo)> for ElfHeader {
    fn from(value: (&Elf64Hdr, &ElfInfo)) -> Self {
        let (hdr, info) = value;
        Self {
            ei_class: info.ei_class.into(),
            ei_data: info.ei_data.into(),
            ei_os_abi: info.ei_os_abi.into(),
            ei_abi_ver: info.ei_abi_version.into(),
            padding: info.pad,
            elf_type: hdr.e_type.into(),
            machine: hdr.e_machine.into(),
            version: hdr.e_version.into(),
            entry: hdr.e_entry,
            pro_hdr_off: hdr.e_phoff,
            sec_hdr_off: hdr.e_shoff,
            flags: hdr.e_flags,
            hdr_size: hdr.e_ehsize,
            ph_entry_sz: hdr.e_phentsize,
            ph_entry_num: hdr.e_phnum,
            sh_entry_sz: hdr.e_shentsize,
            sh_entry_num: hdr.e_shnum,
            sec_str_idx: hdr.e_shstrndx,
        }
    }
}
