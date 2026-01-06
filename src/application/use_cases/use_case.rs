use async_trait::async_trait;

#[async_trait]
pub trait UseCase {
    type Input;
    type Output;
    type Error;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;
}
