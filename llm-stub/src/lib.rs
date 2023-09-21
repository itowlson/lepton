use std::path::PathBuf;

use async_trait::async_trait;

pub struct LlmComponent;
pub struct LlmStub;

#[derive(Default)]
pub struct LLmOptions {  // yes!  watch the casing
    pub use_gpu: bool,
}

pub const AI_MODELS_KEY: &'static str = "ai-models";

impl LlmComponent {
    pub async fn new(_registry: PathBuf, _use_gpu: bool) -> Self {
        Self
    }
}

impl spin_core::HostComponent for LlmComponent {
    type Data = LlmStub;

    fn add_to_linker<T: Send>(
        linker: &mut spin_core::Linker<T>,
        get: impl Fn(&mut spin_core::Data<T>) -> &mut Self::Data + Send + Sync + Copy + 'static,
    ) -> spin_core::wasmtime::Result<()> {
        spin_world::llm::add_to_linker(linker, get)
    }

    fn build_data(&self) -> Self::Data {
        LlmStub
    }
}

impl spin_app::DynamicHostComponent for LlmComponent {
    fn update_data(
        &self,
        _data: &mut Self::Data,
        _component: &spin_app::AppComponent,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

#[async_trait]
impl spin_world::llm::Host for LlmStub {
    async fn infer(
        &mut self,
        _model: spin_world::llm::InferencingModel,
        _prompt: String,
        _params: Option<spin_world::llm::InferencingParams>,
    ) -> anyhow::Result<Result<spin_world::llm::InferencingResult, spin_world::llm::Error>> {
        Ok(Err(spin_world::llm::Error::RuntimeError("This Spin host does not support LLM".to_owned())))
    }

    async fn generate_embeddings(
        &mut self,
        _m: spin_world::llm::EmbeddingModel,
        _data: Vec<String>,
    ) -> anyhow::Result<Result<spin_world::llm::EmbeddingsResult, spin_world::llm::Error>> {
        Ok(Err(spin_world::llm::Error::RuntimeError("This Spin host does not support LLM".to_owned())))
    }
}