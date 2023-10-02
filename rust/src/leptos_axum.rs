use axum::{
    http::StatusCode,
    response::{IntoResponse, Response, Html},
};
use leptos::{html::ElementDescriptor, HtmlElement, Oco};

pub struct LeptosHtml(Oco<'static, str>);

impl From<Oco<'static, str>> for LeptosHtml {
    fn from(oco: Oco<'static, str>) -> Self {
        Self(oco)
    }
}

impl IntoResponse for LeptosHtml {
    fn into_response(self) -> Response {
        (StatusCode::OK, Html(self.0.to_string())).into_response()
    }
}

impl From<String> for LeptosHtml {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl<T> From<HtmlElement<T>> for LeptosHtml
where
    T: ElementDescriptor + 'static,
{
    fn from(value: HtmlElement<T>) -> Self {
        let out = leptos::ssr::render_to_string(move || value);
        let out = out.to_string();

        return out.into();
    }
}
