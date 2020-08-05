use neon::register_module;
use read_file::{read_file_sync, FileReaderTask};
use write_file::{write_file_sync, FileWriterTask};

mod errors;
mod read_file;
mod write_file;

register_module!(mut m, {
    m.export_function("readFile", FileReaderTask::read_file)?;
    m.export_function("readFileSync", read_file_sync)?;
    m.export_function("writeFile", FileWriterTask::write_file)?;
    m.export_function("writeFileSync", write_file_sync)?;
    Ok(())
});
