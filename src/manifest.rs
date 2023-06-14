use std::{net::SocketAddr, path::Path};

use anyhow::Context;
use serde::{Deserialize};

#[derive(Deserialize)]
struct RawManifest {
    apps: Vec<RawApp>,
}

#[derive(Deserialize)]
struct RawApp {
    reference: String,
    address: String,
    state_dir: String,
}

pub struct Manifest {
    pub apps: Vec<App>,
}

#[derive(Clone, Debug)]
pub struct App {
    pub reference: String,
    pub address: SocketAddr,
    pub state_dir: String,
}

impl Manifest {
    pub async fn load_from(path: &Path) -> anyhow::Result<Self> {
        let json = tokio::fs::read_to_string(path).await
            .with_context(|| format!("Failed to read file {}", path.display()))?;
        let raw: RawManifest = serde_json::from_str(&json)?;
        Self::from_raw(raw)
    }

    fn from_raw(raw: RawManifest) -> anyhow::Result<Self> {
        let apps = raw.apps.into_iter().map(App::from_raw).collect::<anyhow::Result<_>>()?;
        Ok(Self { apps })
    }
}

impl App {
    pub async fn load_from(path: &Path) -> anyhow::Result<Self> {
        let json = tokio::fs::read_to_string(path).await
            .with_context(|| format!("Failed to read file {}", path.display()))?;
        let raw: RawApp = serde_json::from_str(&json)?;
        Self::from_raw(raw)
    }

    fn from_raw(raw: RawApp) -> anyhow::Result<Self> {
        Ok(Self {
            reference: raw.reference,
            address: raw.address.parse()?,
            state_dir: raw.state_dir,
        })
    }
}
