use url::Url;
use wry::{NewWindowResponse, PermissionResponse, WebViewBuilder};

fn allows_main_document(raw: &str, port: u16) -> bool {
    let Ok(url) = Url::parse(raw) else { return false };
    url.scheme() == "http"
        && url.host_str() == Some("127.0.0.1")
        && url.port_or_known_default() == Some(port)
        && url.username().is_empty()
        && url.password().is_none()
        && url.path() == "/"
        && url.query().is_none()
        && url.fragment().is_none()
}

fn candidate_builder(port: u16) -> WebViewBuilder<'static> {
    WebViewBuilder::new()
        .with_url(format!("http://127.0.0.1:{port}/"))
        .with_navigation_handler(move |url| allows_main_document(&url, port))
        .with_new_window_req_handler(|_, _| NewWindowResponse::Deny)
        .with_download_started_handler(|_, _| false)
        .with_permission_handler(|_| PermissionResponse::Deny)
        .with_devtools(false)
}

fn main() { let _ = candidate_builder(43123); }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn permits_only_the_fixed_loopback_document() {
        assert!(allows_main_document("http://127.0.0.1:43123/", 43123));
        for url in [
            "http://127.0.0.1:43124/", "http://localhost:43123/",
            "http://127.0.0.1.evil.test:43123/", "http://user@127.0.0.1:43123/",
            "http://127.0.0.1:43123/other", "http://127.0.0.1:43123/?scripted-test",
            "http://127.0.0.1:43123/#fragment", "https://127.0.0.1:43123/",
            "file:///tmp/index.html", "javascript:alert(1)", "not a url",
        ] { assert!(!allows_main_document(url, 43123), "unexpectedly allowed {url}"); }
    }
    #[test]
    fn restrictive_builder_can_be_constructed_without_opening_a_webview() {
        let _ = candidate_builder(43123);
    }
}
