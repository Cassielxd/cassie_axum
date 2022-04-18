use axum::extract::{rejection::HeadersAlreadyExtracted, RequestParts};
use reqwest::header;

pub mod auth;
pub mod event;
fn json_content_type<B>(req: &RequestParts<B>) -> Result<bool, HeadersAlreadyExtracted> {
    let content_type = if let Some(content_type) = req
        .headers()
        .ok_or_else(HeadersAlreadyExtracted::default)?
        .get(header::CONTENT_TYPE)
    {
        content_type
    } else {
        return Ok(false);
    };

    let content_type = if let Ok(content_type) = content_type.to_str() {
        content_type
    } else {
        return Ok(false);
    };

    let mime = if let Ok(mime) = content_type.parse::<mime::Mime>() {
        mime
    } else {
        return Ok(false);
    };

    let is_json_content_type = mime.type_() == "application"
        && (mime.subtype() == "json" || mime.suffix().map_or(false, |name| name == "json"));

    Ok(is_json_content_type)
}
