use super::{node_error, TaskRunner};
use neon::prelude::*;
use std::fs::write;

pub fn write_file_sync(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let filepath = cx.argument::<JsString>(0)?.value();
    let buffer = cx.argument::<JsBuffer>(1)?;
    let data = cx.borrow(&buffer, |raw| Vec::from(raw.as_slice::<u8>()));
    let err = write(filepath, data).map_err(|err| err.to_string());
    node_error!(err, cx);
    Ok(cx.undefined())
}

#[derive(Debug)]
pub struct FileWriterTask {
    filepath: String,
    data: Vec<u8>,
}

impl TaskRunner for FileWriterTask {
    fn run(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let filepath = cx.argument::<JsString>(0)?.value();
        let buffer = cx.argument::<JsBuffer>(1)?;
        let callback = cx.argument::<JsFunction>(2)?;
        cx.borrow(&buffer, |data| {
            (Self {
                filepath,
                data: Vec::from(data.as_slice::<u8>()),
            })
            .schedule(callback);
        });
        Ok(cx.undefined())
    }
}

impl Task for FileWriterTask {
    type Output = ();
    type Error = String;
    type JsEvent = JsUndefined;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        write(&self.filepath, &self.data).map_err(|err| err.to_string())
    }

    fn complete(
        self,
        mut cx: TaskContext,
        result: Result<Self::Output, Self::Error>,
    ) -> JsResult<Self::JsEvent> {
        node_error!(result, cx);
        Ok(cx.undefined())
    }
}
