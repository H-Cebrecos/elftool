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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ElfHeaderRaw {
    pub ei_class: u8,
    pub ei_data: u8,
    pub ei_os_abi: u8,
    pub ei_abi_ver: u8,
    pub padding: [u8; 7],
    pub elf_type: u16,
    pub machine: u16,
    pub version: u32,
    pub entry: u64,
    pub pro_hdr_off: u64,
    pub sec_hdr_off: u64,
    pub flags: u32,
    pub hdr_size: u16,
    pub ph_entry_sz: u16,
    pub ph_entry_num: u16,
    pub sh_entry_sz: u16,
    pub sh_entry_num: u16,
    pub sec_str_idx: u16,
}
