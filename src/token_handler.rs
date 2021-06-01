use std::fmt::Debug;

use async_trait::async_trait;

#[async_trait]
pub trait TokenHandler {
    type SaveErr: Debug;
    type LoadErr: Debug;

    async fn save(&mut self, token: &str) -> Result<(), Self::SaveErr>;
    async fn load(&self) -> Result<String, Self::LoadErr>;
}
