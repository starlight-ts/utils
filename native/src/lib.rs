use neon::register_module;
use read_file::FileReaderTask;
use write_file::FileWriterTask;

mod errors;
mod read_file;
mod write_file;

register_module!(mut m, {
    m.export_function("readFile", FileReaderTask::read_file)?;
    m.export_function("writeFile", FileWriterTask::write_file)?;
    Ok(())
});
