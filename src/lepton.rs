use std::path::PathBuf;

use clap::Parser;

use lepton::manifest::Manifest;
use lepton::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Lepton::parse().run().await
}

#[derive(Parser)]
struct Lepton {
    #[clap(default_value = "lepton.json")]
    source: PathBuf,
}

impl Lepton {
    async fn run(&self) -> anyhow::Result<()> {
        let manifest = Manifest::load_from(&self.source).await?;

        let mut running_apps = vec![];

        for app in &manifest.apps {
            running_apps.push(run::run(app).await?.into_handle());
        }

        let results = futures::future::join_all(running_apps).await;
        for result in results {
            if let Err(e) = &result {
                eprintln!("join error: {e:#}");
            }
            if let Ok(Err(e)) = &result {
                eprintln!("aborted: {e:#}");  // should never happen in the lepton demo
            }
            if let Ok(Ok(Err(e))) = &result {
                eprintln!("trigger error: {e:#}");
            }
        }

        Ok(())
    }
}
