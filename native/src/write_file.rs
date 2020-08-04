use super::node_error;
use neon::prelude::*;
use std::fs::write;

pub struct FileWriterTask {
    filepath: String,
    data: Vec<u8>,
}

impl FileWriterTask {
    pub fn write_file(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let filepath = cx.argument::<JsString>(0)?.value();
        let buffer = cx.argument::<JsBuffer>(1)?;
        let callback = cx.argument::<JsFunction>(2)?;
        cx.borrow(&buffer, |data| {
            (FileWriterTask {
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
