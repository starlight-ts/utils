use neon::prelude::*;

#[macro_export]
macro_rules! node_error {
    ($cx:expr, $res:expr) => {
        match $res {
            Ok(res) => res,
            Err(err) => return $cx.throw_error(format!("{}", err)),
        }
	};
	($cx:expr, $res:expr, type) => {
		match $res {
			Ok(res) => res,
			Err(err) => return $cx.throw_type_error(format!("{}", err))
		}
	};
	($cx:expr, $res:expr, range) => {
		match $res {
			Ok(res) => res,
			Err(err) => return $cx.throw_range_error(format!("{}", err))
		}
	}
}

pub trait TaskRunner: Task {
    fn run(cx: FunctionContext) -> JsResult<JsUndefined>;
}
