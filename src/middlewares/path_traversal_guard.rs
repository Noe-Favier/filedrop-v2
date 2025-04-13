use crate::config::AppConfig;
use crate::model::so::app_config::AppConfig;
use poem::{error::Forbidden, http::StatusCode, Endpoint, Middleware, Request, Result};
use std::path::Path;

pub struct PathTraversalGuard;

impl<E: Endpoint> Middleware<E> for PathTraversalGuard {
    type Output = PathTraversalGuardImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        PathTraversalGuardImpl { endpoint: ep }
    }
}

pub struct PathTraversalGuardImpl<E> {
    endpoint: E,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for PathTraversalGuardImpl<E>
where
    E::Output: Into<poem::Response>,
{
    type Output = poem::Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        if let Some(path) = req.params().get("path") {
            let app_config = req.data::<AppConfig>().ok_or_else(|| {
                poem::Error::from_status(StatusCode::INTERNAL_SERVER_ERROR)
            })?;

            let base = Path::new(&app_config.file_path);
            let full = base.join(path);

            let canon = match full.canonicalize() {
                Ok(p) => p,
                Err(_) => return Err(Forbidden("Invalid path").into()),
            };

            if !canon.starts_with(base) {
                return Err(Forbidden("Traversal detected").into());
            }
        }

        Ok(self.endpoint.call(req).await?.into())
    }
}
