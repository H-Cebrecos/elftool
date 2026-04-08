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

use crate::elf::header::*;

fn read_u16(buf: &[u8], endianness: EiData) -> u16 {
    let bytes: [u8; 2] = [buf[0], buf[1]];

    match endianness {
      ELFDATA::MSB => u16::from_be_bytes(bytes),
                _  => u16::from_le_bytes(bytes),
    }
}

fn read_u32(buf: &[u8], endianness: EiData) -> u32 {
    let bytes: [u8; 4] = [buf[0], buf[1], buf[2], buf[3]];

    match endianness {
      ELFDATA::MSB => u32::from_be_bytes(bytes),
                _  => u32::from_le_bytes(bytes),
    }
}

fn read_u64(buf: &[u8], endianness: EiData) -> u64 {
    let bytes: [u8; 8] = [buf[0], buf[1], buf[2], buf[4], buf[5], buf[6], buf[7]];

    match endianness {
      ELFDATA::MSB => u64::from_be_bytes(bytes),
                _  => u64::from_le_bytes(bytes),
    }
}

