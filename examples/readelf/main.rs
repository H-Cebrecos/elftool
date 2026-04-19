use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

use elftool::{
    elf::{ElfErr, ElfReader, header::*},
    reader::ReaderCtx,
};

#[derive(Debug)]
pub struct FileReader {
    file: File,
}

impl FileReader {
    pub fn open(path: &str) -> Result<Self, ElfErr> {
        let file = File::open(path).map_err(|_| ElfErr::IoError)?;
        Ok(Self { file })
    }
}

impl ElfReader for FileReader {
    fn read(&mut self, offset: u64, buf: &mut [u8]) -> Result<(), ElfErr> {
        self.file
            .seek(SeekFrom::Start(offset))
            .map_err(|_| ElfErr::IoError)?;

        self.file.read_exact(buf).map_err(|_| ElfErr::IoError)?;

        Ok(())
    }
}

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "readelf")]
#[command(about = "Simple ELF inspection tool")]
struct Args {
    /// Show ELF header
    #[arg(short = 'H', long = "header")]
    header: bool,

    /// Input ELF file
    file: PathBuf,
}

fn main() -> Result<(), ElfErr> {
    let args = Args::parse();

    let reader = FileReader::open(args.file.to_str().unwrap())?;
    let ctx = ReaderCtx::new(reader)?;

    if args.header {
        print_hdr(ctx.get_hdr());
    }

    Ok(())
}

fn print_hdr(hdr: &ElfHeader) {
    println!("ELF Header:");
    println!(
        "  Class:                             {}",
        hdr.ei_class
    );
    println!(
        "  Data:                              {}",
        hdr.ei_data
    );
    println!(
        "  Version:                           {} {}",
        hdr.version,
        hdr.version
    );
    println!(
        "  OS/ABI:                            {}",
        hdr.ei_os_abi
    );
    println!("  ABI Version:                       {}", hdr.ei_abi_ver);
    println!(
        "  Type:                              {}",
        hdr.elf_type
    );
    println!("  Machine:                           {:?}", hdr.machine); //TODO
    println!("  Version:                           0x{:x}", hdr.version);
    println!("  Entry point address:               0x{:x}", hdr.entry);
    println!(
        "  Start of program headers:          {} (bytes into file)",
        hdr.pro_hdr_off
    );
    println!(
        "  Start of section headers:          {} (bytes into file)",
        hdr.sec_hdr_off
    );
    println!("  Flags:                             0x{:x}", hdr.flags);
    println!(
        "  Size of this header:               {} (bytes)",
        hdr.hdr_size
    );
    println!(
        "  Size of program headers:           {} (bytes)",
        hdr.ph_entry_sz
    );
    println!("  Number of program headers:         {}", hdr.ph_entry_num);
    println!(
        "  Size of section headers:           {} (bytes)",
        hdr.sh_entry_sz
    );
    println!("  Number of section headers:         {}", hdr.sh_entry_num);
    println!("  Section header string table index: {}", hdr.sec_str_idx);
}
