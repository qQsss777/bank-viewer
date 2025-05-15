use async_trait::async_trait;

#[async_trait]
pub trait BaseUsecase<TRequest, TResponse> {
    async fn execute(&self, payload: &TRequest) -> TResponse;
}
