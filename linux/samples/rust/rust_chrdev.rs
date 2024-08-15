// SPDX-License-Identifier: GPL-2.0

//! Rust character device sample.

use core::result::Result::Err;

use kernel::prelude::*;
use kernel::sync::Mutex;
use kernel::{chrdev, file};

const GLOBALMEM_SIZE: usize = 0x1000;

module! {
    type: RustChrdev,
    name: "rust_chrdev",
    author: "Rust for Linux Contributors",
    description: "Rust character device sample",
    license: "GPL",
}

// global memory buffer
static GLOBALMEM_BUF: Mutex<[u8;GLOBALMEM_SIZE]> = unsafe {
    Mutex::new([0u8;GLOBALMEM_SIZE])
};

// a file that represents the global memory buffer
struct RustFile {
    #[allow(dead_code)]
    inner: &'static Mutex<[u8;GLOBALMEM_SIZE]>,
}

#[vtable]
impl file::Operations for RustFile {
    type Data = Box<Self>;

    fn open(_shared: &(), _file: &file::File) -> Result<Box<Self>> {
        Ok(
            Box::try_new(RustFile {
                inner: &GLOBALMEM_BUF
            })?
        )
    }

    // TODO: when executing "Hello" > /dev/cicv, does it call read or write?
    fn write(this: &Self, _file: &file::File, reader: &mut impl kernel::io_buffer::IoBufferReader, offset:u64,) -> Result<usize> {
        // what is data size? file.inner.len()?
        // off + size is the last position we can read?
        
        // file.inner.len() is the global memory buffer size
        // reader reads data from IO, so data size is reader.len()
        // what is offset for?

        // New:
        // data size is the end position of a buffer for a specific device
        // size is the size that user application call wants to take starting at position offset
        // if the end buffer position is not enough, just return the end buffer position

        // data size is buffer size: this.inner.lock()
        let buf_start_pos = offset as usize;
        let mut buf = this.inner.lock();
        let buf_size = buf.len() - buf_start_pos;
        let data_size = reader.len();
        let buf_end_pos = core::cmp::min(buf_size, data_size);
        // TODO: what does reader do?
        // TODO: why it needs to be mutable reference?
        // TODO: why does it return a Result? where can I find the documentation?
        reader.read_slice(&mut buf[buf_start_pos..buf_end_pos])?;
        Ok(buf_end_pos)
    }

    fn read(this: &Self, _file: &file::File, writer: &mut impl kernel::io_buffer::IoBufferWriter, offset:u64,) -> Result<usize> {
        // what: this is to take the data from a rust file and write to IO console
        // how: a file not only contains its data, but also metadata, so the real file data length is: Rust file length - offset
        // writer.len() is the buffer size of the keneral writer (if the data you want to write to console is larger than this buffer,
        // then you have to write this message in multiple writer session)
        // Therefore, we need to read in slices from the data section of the rust file and the size is the smaller of 
        // writer.len() (or it cannot fit in the writer buffer) or the data size in the data section (you have read it all!).
        // Then we write slice to the writer buffer, starting from the offset and ends at starting position + data length.

        let buf_start_pos = offset as usize;
        let buf = this.inner.lock();
        // let buf_size = buf.len();
        // // let buf = this.inner.lock();
        let buf_size = buf.len() - buf_start_pos;
        // // TODO: what does writer.len() stands for?
        let data_size = writer.len();
        // // print out info
        // pr_info!("data size is {}", &data_size);
        // pr_info!("buffer size is {}", &buf_size);
        // pr_info!("buffer start position is {}", &buf_start_pos);
        let write_len = core::cmp::min(buf_size, data_size);
        let buf_end_pos = buf_start_pos + write_len;
        // // TODO: what does writer do?
        // // TODO: why it needs to be mutable reference?
        // // TODO: why does it return a Result? where can I find the documentation?
        // // writer.write_slice(&buf[buf_start_pos..buf_end_pos])?;
        // writer.write_slice(&buf[buf_start_pos..][..write_len])?;
        // Ok(write_len)

        // TODO: why this is good but the below is not?
        // writer.write_slice(&buf[buf_start_pos..] [..write_len])?;
        writer.write_slice(&buf[buf_start_pos..buf_end_pos])?;
        Ok(write_len)

    }
}

struct RustChrdev {
    _dev: Pin<Box<chrdev::Registration<2>>>,
}

impl kernel::Module for RustChrdev {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust character device sample (init)\n");

        let mut chrdev_reg = chrdev::Registration::new_pinned(name, 0, module)?;

        // Register the same kind of device twice, we're just demonstrating
        // that you can use multiple minors. There are two minors in this case
        // because its type is `chrdev::Registration<2>`
        chrdev_reg.as_mut().register::<RustFile>()?;
        chrdev_reg.as_mut().register::<RustFile>()?;

        Ok(RustChrdev { _dev: chrdev_reg })
    }
}

impl Drop for RustChrdev {
    fn drop(&mut self) {
        pr_info!("Rust character device sample (exit)\n");
    }
}
