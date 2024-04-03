use std::{env, path::Path, rc::Rc};
use std::result::Result::Ok;

use deno_core::{error::AnyError, Extension, Op, op2};

#[op2(async)]
#[string]
async fn op_read_file(#[string] path: String) -> Result<String, AnyError> {
    let content = tokio::fs::read_to_string(path).await?;
    Ok(content)
}

async fn run_rs(file_path: &str, current_dir: &Path) -> Result<(), AnyError> {
    let ext = Extension {
        name: "my_ext",
        ops: std::borrow::Cow::Borrowed(&[op_read_file::DECL]),
        ..Default::default()
    };
    let main_module = deno_core::resolve_path(file_path, current_dir)?;
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![ext],
        ..Default::default()
    });
    js_runtime.execute_script("[joyjs:runtime.js]", include_str!("./runtime.js")).unwrap();
    let mod_id = js_runtime.load_main_es_module(&main_module).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime
        .run_event_loop(deno_core::PollEventLoopOptions::default())
        .await?;

    result.await?;
    Ok(())
}

fn main() {
    let mut args = env::args();
    let current_working_dir = env::current_dir().expect("Failed to get current working dir");

    args.next();
    let file_path = args.next().expect("Missing script path");

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    if let Err(error) = runtime.block_on(run_rs(&file_path, current_working_dir.as_path())) {
        eprintln!("error: {}", error);
    }
}
