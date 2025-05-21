use async_trait::async_trait;

/// Base usecase trait.
/// define execute function with generic type
#[async_trait]
pub trait BaseUsecase<TRequest, TResponse> {
    async fn execute(&self, payload: &TRequest) -> TResponse;
}
