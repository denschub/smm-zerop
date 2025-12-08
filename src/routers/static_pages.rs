use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};
use minijinja::context;

use crate::{components::app_state::AppState, errors::ResponseError};

/// Builds the API router for SMM2.
/// Note to self: these used to be on a subdomain, and are now in a
/// subdirectory, so this needs to be redirected (or proxied) in prod for
/// compatibility.
pub fn build() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/about/", get(about))
        .route("/changelog/", get(changelog))
        .route("/smm1/random_level/", get(smm1_random_level))
}

/// This is just a happy little macro to make rendering completely static pages
/// from template files a little bit less verbose. Please move on, nothing to
/// see here.
macro_rules! static_page {
    ($fn_name:tt, $tpl:expr) => {
        #[axum::debug_handler]
        #[tracing::instrument(skip(app_state))]
        async fn $fn_name(
            State(app_state): State<AppState>,
        ) -> Result<impl IntoResponse, ResponseError> {
            Ok(Html(
                app_state
                    .template
                    .acquire_env()
                    .get_template(concat!("static/", $tpl, ".html"))?
                    .render(context! {})?,
            ))
        }
    };
}

static_page!(index, "index");
static_page!(about, "about");
static_page!(changelog, "changelog");
static_page!(smm1_random_level, "smm1_random_level");
