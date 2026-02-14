use leptos::prelude::ServerFnError;
use leptos::prelude::*;

use crate::application::domain::layout::Layout;

#[server]
pub async fn layout_controller() -> Result<Layout, ServerFnError> {
    use actix_web::web::Data;
    use leptos::logging::error;
    use leptos_actix::extract;

    use crate::adapters::driven::drupal_jsonapi::repositories::LayoutRepository;
    use crate::application::domain::core::AppError;
    use crate::application::ports::driver::ForDisplayingLayout;
    use crate::application::use_cases::GetLayoutUseCase;
    #[cfg(feature = "ssr")]
    use crate::helpers::Cache;
    #[cfg(feature = "ssr")]
    use crate::helpers::Http;

    let http_client: Data<Http> = extract().await?;
    let http_client = http_client.get_ref();

    let cache_client: Data<Cache> = extract().await?;
    let cache_client = cache_client.get_ref();

    let repository = LayoutRepository::new(http_client.clone(), cache_client.clone());
    let layout_service = GetLayoutUseCase::new(Box::new(repository));

    let layout = layout_service.execute().await.map_err(|e| {
        error!("{}", e.to_string());
        ServerFnError::<AppError>::ServerError(e.to_string())
    })?;

    Ok(layout)
}
