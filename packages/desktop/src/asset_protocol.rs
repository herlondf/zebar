use std::{
  collections::HashMap,
  fs,
  path::{Path, PathBuf},
  sync::{LazyLock, Mutex},
};

use tauri::{
  http::{header, Request, Response, StatusCode},
  Runtime, UriSchemeContext,
};

use crate::common::{glob_util, PathExt};

/// Scheme that widgets are served over.
///
/// Reaches the webview as `zebar://localhost/...`, which Windows and
/// Android rewrite to `http://zebar.localhost/...`.
pub const SCHEME: &str = "zebar";

/// Which files a given widget window is allowed to read.
#[derive(Clone, Debug)]
struct WidgetAssets {
  /// Base directory of the widget pack.
  base_dir: PathBuf,

  /// File patterns the widget pack declares.
  file_patterns: Vec<String>,
}

/// Widget windows by Tauri label.
///
/// The label is what tells two widgets apart here. It comes from the
/// webview making the request, so unlike a cookie it cannot be overwritten
/// by another widget.
static WIDGET_ASSETS: LazyLock<Mutex<HashMap<String, WidgetAssets>>> =
  LazyLock::new(|| Mutex::new(HashMap::new()));

/// Grants a widget window access to its pack's files.
///
/// Must happen before the window is built, since the webview starts asking
/// for assets during construction.
pub fn register_widget(
  label: &str,
  base_dir: &Path,
  file_patterns: Vec<String>,
) {
  let assets = WidgetAssets {
    base_dir: base_dir.to_path_buf(),
    file_patterns,
  };

  if let Ok(mut widgets) = WIDGET_ASSETS.lock() {
    widgets.insert(label.to_string(), assets);
  }
}

/// Revokes a widget window's access.
pub fn unregister_widget(label: &str) {
  if let Ok(mut widgets) = WIDGET_ASSETS.lock() {
    widgets.remove(label);
  }
}

/// URL a widget window should open.
pub fn widget_url(
  base_dir: &Path,
  html_path: &Path,
) -> anyhow::Result<tauri::Url> {
  let relative_path = html_path
    .strip_prefix(base_dir)?
    .to_unicode_string()
    .replace('\\', "/");

  Ok(tauri::Url::parse(&format!(
    "{SCHEME}://localhost/{relative_path}"
  ))?)
}

/// Serves a widget's own files, plus the handful of assets Zebar injects.
pub fn handle<R: Runtime>(
  ctx: UriSchemeContext<'_, R>,
  request: Request<Vec<u8>>,
) -> Response<Vec<u8>> {
  let path = percent_decode(request.uri().path());

  match path.as_str() {
    "/__zebar/sw.js" => {
      return Response::builder()
        .header(header::CONTENT_TYPE, "text/javascript")
        // Lets the worker registered from `/__zebar/` claim the whole
        // origin, which is where the widget's own pages live.
        .header("Service-Worker-Allowed", "/")
        .body(include_str!("../resources/sw.js").into())
        .unwrap_or_else(|_| empty(StatusCode::INTERNAL_SERVER_ERROR));
    }
    "/__zebar/zebar.js" => {
      return serve_bytes(
        include_str!("../../client-api/dist/zebar.js").as_bytes(),
        "text/javascript",
      );
    }
    "/__zebar/normalize.css" => {
      return serve_bytes(
        include_str!("../resources/normalize.css").as_bytes(),
        "text/css",
      );
    }
    _ => {}
  }

  let label = ctx.webview_label();

  let Some(assets) = WIDGET_ASSETS
    .lock()
    .ok()
    .and_then(|widgets| widgets.get(label).cloned())
  else {
    error!("No widget registered for webview {label:?}.");
    return empty(StatusCode::FORBIDDEN);
  };

  // An empty path is the widget's root, same as a directory index.
  let relative_path =
    PathBuf::from(path.trim_start_matches('/')).to_path_buf();

  let relative_path = if relative_path.as_os_str().is_empty() {
    PathBuf::from("index.html")
  } else {
    relative_path
  };

  let Ok(absolute_path) =
    assets.base_dir.join(&relative_path).canonicalize_pretty()
  else {
    error!(
      "No such asset: {} within {}.",
      relative_path.display(),
      assets.base_dir.display()
    );

    return empty(StatusCode::NOT_FOUND);
  };

  // Allow access if:
  // - The asset path is within the base directory.
  // - The asset path matches any of the file patterns of the widget pack.
  if !absolute_path.starts_with(&assets.base_dir) {
    error!(
      "Asset {} is outside of {}.",
      absolute_path.display(),
      assets.base_dir.display()
    );

    return empty(StatusCode::FORBIDDEN);
  }

  if !glob_util::is_match(&relative_path, &assets.file_patterns)
    .unwrap_or(false)
  {
    error!(
      "Asset {} does not match the widget pack's file patterns: {:?}.",
      relative_path.display(),
      assets.file_patterns
    );

    return empty(StatusCode::FORBIDDEN);
  }

  match fs::read(&absolute_path) {
    Ok(bytes) => serve_bytes(&bytes, content_type(&absolute_path)),
    Err(err) => {
      error!("Unable to read {}: {err}.", absolute_path.display());
      empty(StatusCode::NOT_FOUND)
    }
  }
}

fn serve_bytes(bytes: &[u8], content_type: &str) -> Response<Vec<u8>> {
  Response::builder()
    .header(header::CONTENT_TYPE, content_type)
    .body(bytes.to_vec())
    .unwrap_or_else(|_| empty(StatusCode::INTERNAL_SERVER_ERROR))
}

fn empty(status: StatusCode) -> Response<Vec<u8>> {
  Response::builder()
    .status(status)
    .body(Vec::new())
    .expect("Response with an empty body is always valid.")
}

/// Content type for a file, by extension.
///
/// Only covers what widgets actually ship; anything else is handed over as
/// bytes and left to the webview to sniff.
fn content_type(path: &Path) -> &'static str {
  match path
    .extension()
    .and_then(|ext| ext.to_str())
    .map(str::to_lowercase)
    .as_deref()
  {
    Some("html" | "htm") => "text/html",
    Some("js" | "mjs") => "text/javascript",
    Some("css") => "text/css",
    Some("json") => "application/json",
    Some("wasm") => "application/wasm",
    Some("svg") => "image/svg+xml",
    Some("png") => "image/png",
    Some("jpg" | "jpeg") => "image/jpeg",
    Some("gif") => "image/gif",
    Some("webp") => "image/webp",
    Some("avif") => "image/avif",
    Some("ico") => "image/x-icon",
    Some("woff2") => "font/woff2",
    Some("woff") => "font/woff",
    Some("ttf") => "font/ttf",
    Some("otf") => "font/otf",
    Some("mp3") => "audio/mpeg",
    Some("mp4") => "video/mp4",
    Some("webm") => "video/webm",
    Some("txt") => "text/plain",
    _ => "application/octet-stream",
  }
}

/// Decodes `%20` and friends in a URI path.
fn percent_decode(path: &str) -> String {
  let bytes = path.as_bytes();
  let mut out = Vec::with_capacity(bytes.len());
  let mut index = 0;

  while index < bytes.len() {
    let decoded = (bytes[index] == b'%' && index + 2 < bytes.len())
      .then(|| {
        std::str::from_utf8(&bytes[index + 1..index + 3])
          .ok()
          .and_then(|hex| u8::from_str_radix(hex, 16).ok())
      })
      .flatten();

    match decoded {
      Some(byte) => {
        out.push(byte);
        index += 3;
      }
      None => {
        out.push(bytes[index]);
        index += 1;
      }
    }
  }

  String::from_utf8_lossy(&out).into_owned()
}
