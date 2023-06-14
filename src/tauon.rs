use std::{path::{PathBuf, Path}, collections::HashMap};

use clap::Parser;
use tokio::sync::RwLock;

use lepton::manifest;
use lepton::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Tauon::parse().run().await
}

#[derive(Parser)]
struct Tauon {
    #[clap(default_value = "tauon")]
    dir: PathBuf,
}

const WATCH_DEBOUNCE_MS: u64 = 250;

impl Tauon {
    async fn run(&self) -> anyhow::Result<()> {
        let runner = TauonRunner::new(&self.dir);
        runner.run().await?;
        Ok(())
    }
}

impl TauonRunner {
    fn new(dir: &Path) -> Self {
        Self {
            dir: dir.to_owned(),
            running_apps: RwLock::new(HashMap::new()),
        }
    }

    async fn run(&self) -> anyhow::Result<()> {
        tokio::fs::create_dir_all(&self.dir).await?;

        let timeout = std::time::Duration::from_millis(WATCH_DEBOUNCE_MS);
        let (tx, rx) = std::sync::mpsc::channel();

        let mut debouncer = notify_debouncer_mini::new_debouncer(timeout, None, tx)?;
        debouncer.watcher().watch(&self.dir, notify_debouncer_mini::notify::RecursiveMode::NonRecursive)?;

        loop {
            match rx.recv() {
                Ok(Ok(events)) => self.process_watch_events(events).await,
                Ok(Err(e)) => eprintln!("watch error: {e:?}"),
                Err(_) => break,
            }
        }

        Ok(())
    }

    async fn process_watch_events(&self, events: Vec<notify_debouncer_mini::DebouncedEvent>) {
        let mut map = self.running_apps.write().await;
        for event in events {
            // event.kind is uninformative so let's do this the hard way
            let path = event.path;
            if path.is_file() {

                if let Some(ra) = map.get(&path) {
                    println!("{path:?} changed");
                    ra.abort();  // Not a smooth handover, but we need to stop the old one before starting the new one because possible port clash. We do this before loading the new one because we don't want an old app hanging around when the file has been replaced.
                } else {
                    println!("{path:?} added");
                }

                let app = match manifest::App::load_from(&path).await {
                    Ok(a) => a,
                    Err(e) => {
                        eprintln!("ERROR! Failed to load app from {path:?}: {e:#}");
                        continue;
                    }
                };

                match run::run(&app).await {
                    Ok(jh) => { map.insert(path, jh); },
                    Err(e) => eprintln!("ERROR! Failed to run app from {path:?}: {e:#}"),
                }

            } else {
                println!("{path:?} deleted");
                if let Some(ra) = map.remove(&path) {
                    ra.abort();
                }
            }
        }
    }
}

struct TauonRunner {
    dir: PathBuf,
    running_apps: RwLock<HashMap<PathBuf, run::RunningApp>>,
}
