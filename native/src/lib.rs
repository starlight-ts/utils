use neon::register_module;
use read_file::FileReaderTask;

mod read_file;
mod errors;

register_module!(mut m, {
	m.export_function("readFile", FileReaderTask::read_file)?;
    Ok(())
});
