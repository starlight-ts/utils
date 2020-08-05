use super::node_error;
use neon::prelude::*;
use std::fs::read;

pub fn read_file_sync(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let filepath = cx.argument::<JsString>(0)?.value();
    let file = read(filepath).map_err(|err| err.to_string());
    let bytes = node_error!(file, cx);
    let buffer = cx.buffer(bytes.len() as u32)?;
    for (i, byte) in bytes.iter().enumerate() {
        let js_byte = cx.number(*byte);
        buffer.set(&mut cx, i as u32, js_byte)?;
    }
    Ok(buffer)
}

#[derive(Debug)]
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
        let filebytes = node_error!(result, cx);
        let buffer = cx.buffer(filebytes.len() as u32)?;
        for (i, byte) in filebytes.iter().enumerate() {
            let js_byte = cx.number(*byte);
            buffer.set(&mut cx, i as u32, js_byte).unwrap();
        }
        Ok(buffer)
    }
}
