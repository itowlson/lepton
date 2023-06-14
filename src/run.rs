use super::manifest::App;

pub struct RunningApp {
    jh: tokio::task::JoinHandle<Result<Result<(), anyhow::Error>, futures::future::Aborted>>, //tokio::task::JoinHandle<anyhow::Result<()>>,
    abort_handle: futures::future::AbortHandle,
}

impl RunningApp {
    pub fn abort(&self) {
        self.abort_handle.abort();
    }

    pub fn into_handle(self) -> tokio::task::JoinHandle<Result<Result<(), anyhow::Error>, futures::future::Aborted>> {
        self.jh
    }
}

pub async fn run(app: &App) -> anyhow::Result<RunningApp> {
    let working_dir = tempfile::tempdir()?;

    let locked_app = prepare_app_from_oci(&app.reference, working_dir.path()).await?;
    let locked_url = write_locked_app(&locked_app, working_dir.path()).await?;

    let loader = spin_trigger::loader::TriggerLoader::new(working_dir.path(), false);
    let init_data = HostComponentInitData::default();

    let trigger = build_executor(&app, loader, locked_url, init_data).await?;

    let http_run_config = spin_trigger_http::CliArgs {
        address: app.address.clone(), tls_cert: None, tls_key: None
    };

    let run_fut = trigger.run(http_run_config);
    let (abortable, abort_handle) = futures::future::abortable(run_fut);

    let jh = tokio::task::spawn(async move {
        let _wd = working_dir;
        abortable.await
    });
    Ok(RunningApp { jh, abort_handle })
}

// Copied and trimmed down from spin trigger

use spin_app::Loader;
use spin_trigger::{HostComponentInitData, RuntimeConfig, TriggerExecutorBuilder, TriggerExecutor};
use spin_trigger_http::HttpTrigger;

async fn build_executor(
    app: &App,
    loader: impl Loader + Send + Sync + 'static,
    locked_url: String,
    init_data: HostComponentInitData,
) -> Result<HttpTrigger> {
    let runtime_config = build_runtime_config(&app.state_dir)?;

    let mut builder = TriggerExecutorBuilder::new(loader);
    builder.wasmtime_config_mut().cache_config_load_default()?;

    builder.build(locked_url, runtime_config, init_data).await
}

fn build_runtime_config(state_dir: impl Into<String>) -> Result<RuntimeConfig> {
    let mut config = RuntimeConfig::new(None);
    config.set_state_dir(state_dir);
    Ok(config)
}

// Copied and trimmed down from spin up

use anyhow::{anyhow, Context, Result};
use spin_app::locked::LockedApp;
use spin_oci::OciLoader;
use std::path::Path;
use url::Url;

async fn prepare_app_from_oci(reference: &str, working_dir: &Path) -> Result<LockedApp> {
    let mut client = spin_oci::Client::new(false, None)
        .await
        .context("cannot create registry client")?;

    OciLoader::new(working_dir)
        .load_app(&mut client, reference)
        .await
}

async fn write_locked_app(
    locked_app: &LockedApp,
    working_dir: &Path,
) -> Result<String, anyhow::Error> {
    let locked_path = working_dir.join("spin.lock");
    let locked_app_contents =
        serde_json::to_vec_pretty(&locked_app).context("failed to serialize locked app")?;
    tokio::fs::write(&locked_path, locked_app_contents)
        .await
        .with_context(|| format!("failed to write {:?}", locked_path))?;
    let locked_url = Url::from_file_path(&locked_path)
        .map_err(|_| anyhow!("cannot convert to file URL: {locked_path:?}"))?
        .to_string();

    Ok(locked_url)
}
