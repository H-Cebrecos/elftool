
    use core::fmt::Display;

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

    impl From<u16> for Type {
        fn from(value: u16) -> Self {
            use crate::repr::ET;
            match value {
                ET::NONE => Self::None,
                ET::REL => Self::Reloc,
                ET::EXEC => Self::Exec,
                ET::DYN => Self::Dyn,
                ET::CORE => Self::Core,
                ET::LOOS..=ET::HIOS => Self::Os(value),
                ET::LOPROC..=ET::HIPROC => Self::Proc(value),
                _ => Self::Reserved(value),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum Version {
        None,
        Current,
        Reserved(u32),
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

    impl From<u32> for Version {
        fn from(value: u32) -> Self {
            use crate::repr::EV;
            match value {
                EV::NONE => Self::None,
                EV::CURRENT => Self::Current,
                _ => Self::Reserved(value),
            }
        }
    }

    impl From<u8> for Version {
        fn from(value: u8) -> Self {
            use crate::repr::EV;
            match u32::from(value) {
                EV::NONE => Self::None,
                EV::CURRENT => Self::Current,
                _ => Self::Reserved(u32::from(value)),
            }
        }
    }

    //TODO: use the same approach for the rest.

    /// Constants for valid [`ElfVersion`] values.
    ///
    /// Use these constants instead of raw numbers for clarity.


    pub type ElfMachine = u16;
    pub mod EM {
        use super::ElfMachine;

        pub const NONE: ElfMachine = 0;

        /* You may add your application-specific machines here */
    }

    // TODO: then extending machines and things is:
    //    use crate::ElfMachine;
    //    pub const EM_ARM: ElfMachine = 40;

    pub type EiClass = u8;
    pub mod ELFCLASS {
        use super::EiClass;

        pub const NONE: EiClass = 0;
        pub const CLASS_32: EiClass = 1;
        pub const CLASS_64: EiClass = 2;

        #[cfg(feature = "fmt")]
        pub fn to_str(v: EiClass) -> &'static str {
            match v {
                CLASS_32 => "ELF32",
                CLASS_64 => "ELF64",
                NONE => "Invalid",
                _ => "Unknown",
            }
        }
    }

    pub type EiData = u8;
    pub mod ELFDATA {
        use super::EiData;

        pub const NONE: EiData = 0;
        pub const LSB: EiData = 1;
        pub const MSB: EiData = 2;

        #[cfg(feature = "fmt")]
        pub fn to_str(v: EiData) -> &'static str {
            match v {
                LSB => "2's complement, little endian",
                MSB => "2's comeplement big endian",
                NONE => "Invalid",
                _ => "Unknown",
            }
        }
    }

    pub type ElfABI = u8;
    pub mod ELFOSABI {
        use super::ElfABI;

        pub const NONE: ElfABI = 0; // This is the default value for most linkers

        /* You may add your application-specific ABIs here */

        #[cfg(feature = "fmt")]
        pub fn to_str(v: ElfABI) -> &'static str {
            match v {
                NONE => "Default",
                _ => "Unknown",
            }
        }
    }

    /// Abstract representation of the ELF header, it does not represent the real layout, instead provides a uniform view into the data.
    /// Abstract representation of the ELF header.
    /// Does not represent the real layout, instead provides a uniform view into the data.
    #[derive(Debug)]
    pub struct ElfHeader {
        /// 32 or 64 bit architecture
        pub ei_class: EiClass,

        /// Endianness of the architecture
        pub ei_data: EiData,

        /// Target platform's ABI
        pub ei_os_abi: ElfABI,

        /// Target ABI version
        pub ei_abi_ver: Version,

        /// Padding
        pub padding: [u8; 7],

        /// Type of ELF file
        pub elf_type: Type,

        /// Architecture
        pub machine: ElfMachine,

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

    impl From<(&crate::repr::Elf32Hdr, &crate::repr::ElfInfo)> for ElfHeader {
        fn from(value: (&crate::repr::Elf32Hdr, &crate::repr::ElfInfo)) -> Self {
            let (hdr, info) = value;
            Self {
                ei_class: info.ei_class,
                ei_data: info.ei_data,
                ei_os_abi: info.ei_os_abi,
                ei_abi_ver: info.ei_abi_version.into(),
                padding: info.pad,
                elf_type: hdr.e_type.into(),
                machine: hdr.e_machine,
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
    impl From<(&crate::repr::Elf64Hdr, &crate::repr::ElfInfo)> for ElfHeader {
        fn from(value: (&crate::repr::Elf64Hdr, &crate::repr::ElfInfo)) -> Self {
            let (hdr, info) = value;
            Self {
                ei_class: info.ei_class,
                ei_data: info.ei_data,
                ei_os_abi: info.ei_os_abi,
                ei_abi_ver: info.ei_abi_version.into(),
                padding: info.pad,
                elf_type: hdr.e_type.into(),
                machine: hdr.e_machine,
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
