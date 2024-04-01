use std::{env, path::Path, rc::Rc};

use deno_core::{anyhow::Ok, error::AnyError};

async fn run_rs(file_path: &str, current_dir: &Path) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path, current_dir)?;
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        ..Default::default()
    });

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
