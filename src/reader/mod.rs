use binlayout::BinLayout;
use binlayout::Endian;

use crate::elf::header::*;
use crate::elf::section::*;
use crate::elf::segment::*;
use crate::elf::*;

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

        if info.ei_version != Version::CURRENT as u8 {
            return Err(ElfErr::BadVersion(info.ei_version));
        }

        let endianess = match info.ei_data {
            InfoData::LSB => Endian::Little,
            InfoData::MSB => Endian::Big,
            _ => return Err(ElfErr::BadEndianness),
        };

        let class;
        let hdr: ElfHeader = match info.ei_class {
            InfoClass::CLASS_32 => {
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
            InfoClass::CLASS_64 => {
                class = Class::Bit64;
                let mut hdr_buf = [0u8; size_of::<Elf64Hdr>()];
                reader.read(INFO_SIZE as u64, &mut hdr_buf)?;
                let hdr = Elf64Hdr::parse(&hdr_buf, endianess);

                if hdr.e_ehsize as usize != (size_of::<Elf64Hdr>() + INFO_SIZE)
                    || (hdr.e_phnum > 0 && hdr.e_phentsize as usize != size_of::<Elf64ProHdr>())
                    || (hdr.e_shnum > 0 && hdr.e_shentsize as usize != size_of::<Elf64SecHdr>())
                {
                    return Err(ElfErr::BadSize);
                }

                (&hdr, &info).into()
            }
            _ => return Err(ElfErr::BadClass),
        };

        if (hdr.ph_entry_num > 0 && hdr.pro_hdr_off == 0)
            || (hdr.sh_entry_num > 0 && hdr.sec_hdr_off == 0)
        {
            return Err(ElfErr::BadHeader);
        }

        /* Detect special indexes */
        if (hdr.sh_entry_num == sec_idx::SHN_UNDEF) || (hdr.sec_str_idx == sec_idx::SHN_XINDEX) {
            if hdr.sec_hdr_off == 0 {
                return Err(ElfErr::BadHeader);
            }

            //TODO: wee need to get the null section but we don't have the context intialized.
        }

        Ok(Self {
            reader,
            class,
            endianess,
            hdr,
        })
    }

    pub fn get_hdr(&self) -> &ElfHeader {
        &self.hdr
    }

    pub fn get_program_hdr(&mut self, idx: u16) -> Result<ProgramHeader, ElfErr> {
        if idx >= self.hdr.ph_entry_num {
            return Err(ElfErr::BadIndex);
        }

        let segment: ProgramHeader;
        match self.class {
            Class::Bit32 => {
                let mut seg_buf = [0u8; size_of::<Elf32ProHdr>()];
                self.reader.read(
                    self.hdr.pro_hdr_off + (idx * (self.hdr.ph_entry_sz)) as u64,
                    &mut seg_buf,
                )?;
                let seg = Elf32ProHdr::parse(&seg_buf, self.endianess);
                segment = (&seg).into();
            }
            Class::Bit64 => {
                let mut seg_buf = [0u8; size_of::<Elf64ProHdr>()];
                self.reader.read(
                    self.hdr.pro_hdr_off + (idx * (self.hdr.ph_entry_sz)) as u64,
                    &mut seg_buf,
                )?;
                let seg = Elf64ProHdr::parse(&seg_buf, self.endianess);
                segment = (&seg).into();
            }
        }

        //TODO: checks
        //TODO: note segments
        return Ok(segment);
    }

    pub fn get_sec_hdr(&self) -> &ElfSecHeader {
        todo!()
    }

    //pub fn get_sec_name(&self, sec: &ElfSecHeader, ...)

    // the other functions go here
}
