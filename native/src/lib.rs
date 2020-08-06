#![deny(clippy::all)]
#![warn(clippy::nursery, clippy::pedantic)]
// This is enabled so that we can cast to the stricter numbers that neon requires.
#![allow(clippy::cast_possible_truncation, clippy::module_name_repetitions)]

use neon::register_module;
use read_file::{read_file_sync, FileReaderTask};
use write_file::{write_file_sync, FileWriterTask};

pub(crate) use internal::NodeTaskRunner;

mod internal;
mod read_file;
mod write_file;

register_module!(mut m, {
    m.export_function("readFile", FileReaderTask::run)?;
    m.export_function("readFileSync", read_file_sync)?;
    m.export_function("writeFile", FileWriterTask::run)?;
    m.export_function("writeFileSync", write_file_sync)?;
    Ok(())
});
