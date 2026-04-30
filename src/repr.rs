/*
 * Copyright (c) 2026 Hugo Cebrecos
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

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

pub mod ET {

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

pub mod EV {
    pub const NONE: u32 = 0; // Invalid
    pub const CURRENT: u32 = 1;
}

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

#[repr(C)]
#[derive(BinLayout)]
pub struct Elf32ProHdr {}

#[repr(C)]
#[derive(BinLayout)]
pub struct Elf64ProHdr {}

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
