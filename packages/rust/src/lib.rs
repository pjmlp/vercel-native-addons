use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    let result = String::from("Rust World");
    Ok(cx.string(result))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    Ok(())
}