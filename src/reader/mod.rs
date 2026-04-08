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

#![no_std]

use crate::elf::header::*;
use crate::elf::*;
use crate::repr::*;

enum Endianess {
    Lit,
    Big,
}

enum Class {
    Bit64,
    Bit32,
}

pub struct ReaderCtx<R> {
    reader: R,
    class: Class,
    endianess: Endianess,
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

        let mut ctx = Self {
            reader: reader,
            class: Class::Bit64,
            endianess: Endianess::Lit,
        };

        /* Parse identification header */
        let mut buf = [0u8; core::mem::size_of::<ElfHeaderRaw>()];
        ctx.reader.read(0, &mut buf)?;

        if buf[0..4] != ELF_MAGIC {
            return Err(ElfErr::BadMagic);
        }

        let info: &ElfInfo = unsafe {
            // Safety: ElfInfo is #[repr(C)] and matches the ELF layout exactly
            &*(buf[4..].as_ptr() as *const ElfInfo)
        };

        if info.ei_abi_version != EV::CURRENT as u8 {
            return Err(ElfErr::BadVersion);
        }
        // TODO: continue here with the class, note that it is cached in the ctx
        // TODO: convert this and the other into an enum to store
        ctx.class = match info.ei_class {
            ELFCLASS::CLASS_32 => {
                //todo!("get buffer for 32 bit");
                Class::Bit32
            }
            ELFCLASS::CLASS_64 => {
                //todo!("get buffer to 64 bit");
                Class::Bit64
            }
            _ => return Err(ElfErr::BadClass),
        };

        ctx.endianess = match info.ei_data {
            ELFDATA::LSB => Endianess::Lit, //TODO: endianess is still a WIP
            ELFDATA::MSB => Endianess::Big,
            _ => return Err(ElfErr::BadEndianness),
        };

        //TODO: start testing with some files

        //TODO: make non-native endiannes support optional, and try to make it into the type system
        //      that if you try to compile code that doesn't check that the endianness is supported
        //      you get an error.
        Ok(ctx)
    }

    // the other functions go here
}
