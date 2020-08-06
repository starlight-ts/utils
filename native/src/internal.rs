use neon::prelude::*;

#[macro_export]
macro_rules! node_error {
    ($res:expr, $cx:expr) => {{
        match $res {
            Ok(res) => res,
            Err(err) => return $cx.throw_type_error(format!("{}", err)),
        }
    }};
}

pub(crate) trait TaskRunner: Task {
    fn run(cx: FunctionContext) -> JsResult<JsUndefined>;
}
