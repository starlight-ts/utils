use neon::prelude::*;
use std::fs::read;

pub struct FileReaderTask {
    filepath: String,
}

impl FileReaderTask {
    pub fn read_file(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let filepath = cx.argument::<JsString>(0)?.value();
        let callback = cx.argument::<JsFunction>(1)?;

        (FileReaderTask { filepath }).schedule(callback);
        Ok(cx.undefined())
    }
}

impl Task for FileReaderTask {
    type Output = Vec<u8>;
    type Error = String;
    type JsEvent = JsBuffer;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        read(&self.filepath).map_err(|err| err.to_string())
    }

    fn complete(
        self,
        mut cx: TaskContext,
        result: Result<Self::Output, Self::Error>,
    ) -> JsResult<Self::JsEvent> {
		let filebytes = match result {
			Ok(file) => file,
			Err(err) => return cx.throw_type_error(format!("{}", err))
		};
        let buffer = cx.buffer(filebytes.len() as u32)?;
        for (i, byte) in filebytes.iter().enumerate() {
            let js_byte = cx.number(*byte);
            buffer.set(&mut cx, i as u32, js_byte).unwrap();
        }
        Ok(buffer)
    }
}
