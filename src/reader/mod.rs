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

use core::mem::size_of;

use binlayout::BinLayout;
use binlayout::Endian;

use crate::elf::header::*;
use crate::elf::section::ElfSecHeader;
use crate::elf::*;
use crate::repr::*;

#[derive(Debug, Default)]
enum Class {
    #[default]
    Bit64,
    Bit32,
}

#[derive(Debug)]
pub struct ReaderCtx<R> {
    reader: R,
    class: Class,
    endianess: Endian,
    hdr: ElfHeader,
}

//TODO: Doc that it actually reads the file
impl<R: ElfReader> ReaderCtx<R> {
    pub fn new(reader: R) -> Result<Self, ElfErr> {
        /*
         * This function checks for:
         *      - Correct Magic         ("ELF")
         *      - Correct info version  (1)
         *      - Correct class         (32/64)
         *      - Correct endiannes     (little/big)
         *      - Correct hdr version   (1)
         *      - Correct hdr size
         *      - Correct phdr size
         *      - Correct shdr size
         *      - Zero phdr off with non-zero phdr cnt
         *      - Zero shdr off with non-zero shdr cnt
         *      - Hdr has special index but shdr off is zero
         *      - Hdr has special index but null sec is not of SHT_NULL type
         *
         * This function doesn't check for:
         *      - null section is not of SHT_NULL type if there is no need to access the section (lazy check)
         */
        let mut reader = reader;

        /* Parse identification header */
        const INFO_SIZE: usize = size_of::<ElfInfo>();
        let mut buf = [0u8; INFO_SIZE];
        reader.read(0, &mut buf)?;

        let info = ElfInfo::parse_ne(&buf);
        if info.magic != ELF_MAGIC {
            return Err(ElfErr::BadMagic);
        }

        if info.ei_version != EV::CURRENT as u8 {
            return Err(ElfErr::BadVersion(info.ei_version));
        }

        //TODO: a "native" cfg that only allows native size/endianness and errors otherwise
        let endianess = match info.ei_data {
            ELFDATA::LSB => Endian::Little,
            ELFDATA::MSB => Endian::Big,
            _ => return Err(ElfErr::BadEndianness),
        };

        let class;
        let hdr: ElfHeader = match info.ei_class {
            ELFCLASS::CLASS_32 => {
                class = Class::Bit32;
                let mut hdr_buf = [0u8; size_of::<Elf32Hdr>()];
                reader.read(INFO_SIZE as u64, &mut hdr_buf)?;
                let hdr = Elf32Hdr::parse(&hdr_buf, endianess);

                if hdr.e_ehsize as usize != (size_of::<Elf32Hdr>() + INFO_SIZE)
                    || (hdr.e_phnum > 0 && hdr.e_phentsize as usize != size_of::<Elf32ProHdr>())
                    || ((hdr.e_shnum > 0) && (hdr.e_shentsize as usize != size_of::<Elf32SecHdr>()))
                {
                    return Err(ElfErr::BadSize);
                }

                (&hdr, &info).into()
            }
            ELFCLASS::CLASS_64 => {
                class = Class::Bit64;
                let mut hdr_buf = [0u8; size_of::<Elf64Hdr>()];
                reader.read(INFO_SIZE as u64, &mut hdr_buf)?;
                let hdr = Elf64Hdr::parse(&hdr_buf, endianess);

                if hdr.e_ehsize as usize != (size_of::<Elf64Hdr>() + INFO_SIZE)
                    || (hdr.e_phnum > 0 && hdr.e_phentsize as usize != size_of::<Elf64ProHdr>())
                    || (hdr.e_shnum > 0 && hdr.e_shentsize as usize != size_of::<Elf64SecHdr>())
                {
                    assert!(hdr.e_ehsize as usize == (size_of::<Elf64Hdr>() + INFO_SIZE), "bad size");
                    return Err(ElfErr::BadSize);
                }

                (&hdr, &info).into()
            }
            _ => return Err(ElfErr::BadClass),
        };

        if (hdr.ph_entry_num > 0 && hdr.pro_hdr_off == 0)
        || (hdr.sh_entry_num > 0 && hdr.sec_hdr_off == 0) {
            return Err(ElfErr::BadHeader)
        }

        /* Detect special indexes */
        if (hdr.sh_entry_num == sec_idx::SHN_UNDEF)
        || (hdr.sec_str_idx == sec_idx::SHN_XINDEX) {
            if hdr.sec_hdr_off == 0 {
                return Err(ElfErr::BadHeader);
            }

            //TODO: wee need to get the null section but we don't have the context intialized.
        }

        Ok(Self { reader, class, endianess, hdr })
    }

    pub fn get_hdr(&self) -> &ElfHeader {
        &self.hdr
    }

    pub fn get_sec_hdr(&self) -> &ElfSecHeader {
        todo!()
    }

    //pub fn get_sec_name(&self, sec: &ElfSecHeader, ...)

    // the other functions go here
}
