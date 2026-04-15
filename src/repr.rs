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
pub struct ElfInfo {
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

#[repr(C)]
#[derive(BinLayout)]
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
#[derive(BinLayout)]
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
pub struct Elf32SecHdr {}
#[repr(C)]
#[derive(BinLayout)]
pub struct Elf64SecHdr {}
