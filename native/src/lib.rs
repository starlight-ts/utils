use neon::register_module;
use read_file::FileReaderTask;

mod read_file;

register_module!(mut m, {
    m.export_function("readFile", FileReaderTask::read_file)?;
    Ok(())
});
