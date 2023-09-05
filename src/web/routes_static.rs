use crete::config;

pub fn serve_dir() -> MethodRouter {
    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "RESOURCE NOT FOUND")
    }

    any_service(ServeDir::new(config().WEB_FOLDER)).not_found_service(handle_404.into_service());
}
