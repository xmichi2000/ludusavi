//! Finding cover art for games.
//!
//! Covers come from several places, tried in order:
//! what Steam has already downloaded, Steam's own servers,
//! the metadata that your launchers keep, and optionally
//! SteamGridDB and IGDB if you've configured them.
//! Whatever we find is saved locally, so each game is only looked up once.

use crate::{
    path::StrictPath,
    prelude::{Security, app_dir, get_reqwest_client},
    resource::{
        config::Config,
        manifest::{Manifest, Store},
    },
};

/// Steam serves cover images for any game in its store, without needing an account.
const STEAM_CDN: &str = "https://cdn.cloudflare.steamstatic.com/steam/apps";

const STEAMGRIDDB_API: &str = "https://www.steamgriddb.com/api/v2";
const IGDB_TOKEN_URL: &str = "https://id.twitch.tv/oauth2/token";
const IGDB_API: &str = "https://api.igdb.com/v4";

/// Don't keep anything that clearly isn't an image.
const MIN_BYTES: usize = 1024;

/// Covers are stored in portrait, so that the game list keeps one shape.
/// See docs/design-system.md.
const ASPECT_WIDTH: u32 = 2;
const ASPECT_HEIGHT: u32 = 3;

/// Make an image portrait by cropping the middle out of it,
/// rather than squeezing it into shape.
fn to_portrait(image: image::DynamicImage) -> image::DynamicImage {
    let (width, height) = (image.width(), image.height());
    if width == 0 || height == 0 {
        return image;
    }

    let wanted_width = height * ASPECT_WIDTH / ASPECT_HEIGHT;
    if width > wanted_width {
        // Too wide, such as a Steam header image: take the centre.
        let x = (width - wanted_width) / 2;
        return image.crop_imm(x, 0, wanted_width, height);
    }

    let wanted_height = width * ASPECT_HEIGHT / ASPECT_WIDTH;
    if height > wanted_height {
        // Too tall: take the middle, favoring the upper part where art usually is.
        let y = (height - wanted_height) / 3;
        return image.crop_imm(0, y, width, wanted_height);
    }

    image
}

/// Where we keep the cover for a game.
/// Everything is stored as PNG, whatever format it arrived in.
pub fn cached_path(game: &str) -> StrictPath {
    app_dir()
        .joined("covers")
        .joined(format!("{}.png", crate::scan::layout::escape_folder_name(game)))
}

/// A marker saying that we looked for this game's cover and came up empty,
/// so that we don't ask the same servers over and over.
fn missing_marker(game: &str) -> StrictPath {
    app_dir()
        .joined("covers")
        .joined(format!("{}.missing", crate::scan::layout::escape_folder_name(game)))
}

/// Whether we already know the answer for this game, one way or the other.
pub fn is_resolved(game: &str) -> bool {
    cached_path(game).is_file() || missing_marker(game).is_file()
}

/// Take a cover that's already on this computer into our own cache,
/// cropped to portrait like everything else, so the list stays consistent.
pub fn adopt_local(game: &str, source: &StrictPath) -> Option<StrictPath> {
    if let Some(cached) = cached(game) {
        return Some(cached);
    }

    let image = image::open(source.as_std_path_buf().ok()?).ok()?;

    let target = cached_path(game);
    target.parent()?.create_dirs().ok()?;
    to_portrait(image)
        .save_with_format(target.as_std_path_buf().ok()?, image::ImageFormat::Png)
        .ok()?;

    Some(target)
}

/// The cover we have on hand for a game, if any.
pub fn cached(game: &str) -> Option<StrictPath> {
    let path = cached_path(game);
    path.is_file().then_some(path)
}

/// The Steam ID of a game, from the manifest or from Steam's own install records.
///
/// The manifest doesn't have an ID for every game,
/// but Steam records one for everything you have installed,
/// so the two together cover much more than either alone.
fn steam_id(config: &Config, manifest: &Manifest, game: &str) -> Option<u32> {
    if let Some(id) = manifest.0.get(game).and_then(|x| x.steam.id) {
        return Some(id);
    }

    installed_steam_id(config, game)
}

/// Steam keeps one `appmanifest_<id>.acf` file per installed game,
/// which pairs the ID with the game's name.
fn installed_steam_id(config: &Config, game: &str) -> Option<u32> {
    let wanted = crate::scan::title::normalize_title(game);

    for root in config.expanded_roots() {
        if root.store() != Store::Steam {
            continue;
        }

        let apps = root.path().joined("steamapps");
        let Ok(entries) = apps.read_dir() else { continue };

        for entry in entries.filter_map(|x| x.ok()) {
            let path = StrictPath::from(entry.path());
            let rendered = path.render();
            if !rendered.contains("appmanifest_") || !rendered.ends_with(".acf") {
                continue;
            }

            let Ok(content) = path.try_read() else { continue };
            let Some(name) = acf_value(&content, "name") else {
                continue;
            };
            if crate::scan::title::normalize_title(&name) != wanted {
                continue;
            }
            if let Some(id) = acf_value(&content, "appid").and_then(|x| x.parse().ok()) {
                return Some(id);
            }
        }
    }

    None
}

/// Read one value out of Steam's simple `"key" "value"` format.
fn acf_value(content: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    content.lines().find_map(|line| {
        let rest = line.trim().strip_prefix(&needle)?;
        let rest = rest.trim();
        let inner = rest.strip_prefix('"')?;
        let end = inner.find('"')?;
        Some(inner[..end].to_string())
    })
}

/// Image URLs to try for a game, in order of preference, without asking any API.
fn direct_urls(config: &Config, manifest: &Manifest, game: &str) -> Vec<String> {
    let mut urls = vec![];

    if let Some(steam_id) = steam_id(config, manifest, game) {
        // The portrait cover first, then the wide header as a fallback.
        urls.push(format!("{STEAM_CDN}/{steam_id}/library_600x900.jpg"));
        urls.push(format!("{STEAM_CDN}/{steam_id}/header.jpg"));
    }

    urls.extend(launcher_urls(config, game));

    urls
}

/// Cover URLs that your launchers have already recorded locally.
fn launcher_urls(config: &Config, game: &str) -> Vec<String> {
    let mut urls = vec![];

    for root in config.expanded_roots() {
        if root.store() != Store::Heroic {
            continue;
        }

        let library = root.path().joined("store_cache").joined("legendary_library.json");
        if let Some(url) = epic_cover_url(&library, game) {
            urls.push(url);
        }
    }

    urls
}

/// Heroic stores Epic's own artwork URLs alongside your library.
fn epic_cover_url(library: &StrictPath, game: &str) -> Option<String> {
    #[derive(serde::Deserialize)]
    struct Library {
        library: Vec<Entry>,
    }

    #[derive(serde::Deserialize)]
    struct Entry {
        title: Option<String>,
        art_square: Option<String>,
    }

    let content = library.try_read().ok()?;
    let parsed: Library = serde_json::from_str(&content).ok()?;

    let wanted = crate::scan::title::normalize_title(game);
    parsed
        .library
        .into_iter()
        .find(|entry| {
            entry
                .title
                .as_ref()
                .map(|title| crate::scan::title::normalize_title(title) == wanted)
                .unwrap_or(false)
        })
        .and_then(|entry| entry.art_square)
        .filter(|url| !url.is_empty())
}

/// Ask SteamGridDB for a cover. This needs a key, which you can get for free.
async fn steamgriddb_url(config: &Config, manifest: &Manifest, game: &str, security: Security) -> Option<String> {
    #[derive(serde::Deserialize)]
    struct Response<T> {
        success: bool,
        data: Option<T>,
    }

    #[derive(serde::Deserialize)]
    struct Grid {
        url: String,
    }

    #[derive(serde::Deserialize)]
    struct Found {
        id: u32,
    }

    let key = config.covers.steamgriddb_key.as_ref()?;
    if key.trim().is_empty() {
        return None;
    }

    let client = get_reqwest_client(security);
    let authorized = |url: String| client.get(url).bearer_auth(key);

    // Looking up by Steam ID is exact, so try that before searching by name.
    if let Some(steam_id) = steam_id(config, manifest, game) {
        let url = format!("{STEAMGRIDDB_API}/grids/steam/{steam_id}?dimensions=600x900&types=static");
        if let Ok(response) = authorized(url).send().await
            && let Ok(parsed) = response.json::<Response<Vec<Grid>>>().await
            && parsed.success
            && let Some(grid) = parsed.data.and_then(|x| x.into_iter().next())
        {
            return Some(grid.url);
        }
    }

    let url = format!("{STEAMGRIDDB_API}/search/autocomplete/{}", urlencode(game));
    let response = authorized(url).send().await.ok()?;
    let parsed = response.json::<Response<Vec<Found>>>().await.ok()?;
    let found = parsed.data?.into_iter().next()?;

    let url = format!(
        "{STEAMGRIDDB_API}/grids/game/{}?dimensions=600x900&types=static",
        found.id
    );
    let response = authorized(url).send().await.ok()?;
    let parsed = response.json::<Response<Vec<Grid>>>().await.ok()?;
    parsed.data?.into_iter().next().map(|grid| grid.url)
}

/// Ask IGDB for a cover. This needs a Twitch client ID and secret.
async fn igdb_url(config: &Config, game: &str, security: Security) -> Option<String> {
    #[derive(serde::Deserialize)]
    struct Token {
        access_token: String,
    }

    #[derive(serde::Deserialize)]
    struct Game {
        cover: Option<Cover>,
    }

    #[derive(serde::Deserialize)]
    struct Cover {
        image_id: String,
    }

    let id = config.covers.igdb_client_id.as_ref()?;
    let secret = config.covers.igdb_client_secret.as_ref()?;
    if id.trim().is_empty() || secret.trim().is_empty() {
        return None;
    }

    let client = get_reqwest_client(security);

    let url = format!("{IGDB_TOKEN_URL}?client_id={id}&client_secret={secret}&grant_type=client_credentials");
    let token = client.post(url).send().await.ok()?.json::<Token>().await.ok()?;

    let body = format!("search \"{}\"; fields cover.image_id; limit 1;", game.replace('"', ""));
    let response = client
        .post(format!("{IGDB_API}/games"))
        .header("Client-ID", id)
        .bearer_auth(&token.access_token)
        .body(body)
        .send()
        .await
        .ok()?;

    let games = response.json::<Vec<Game>>().await.ok()?;
    let image_id = games.into_iter().next()?.cover?.image_id;
    Some(format!(
        "https://images.igdb.com/igdb/image/upload/t_cover_big/{image_id}.jpg"
    ))
}

fn urlencode(value: &str) -> String {
    value
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '~') {
                c.to_string()
            } else {
                c.to_string().bytes().map(|b| format!("%{b:02X}")).collect()
            }
        })
        .collect()
}

/// Find and save a cover for a game. Returns where it ended up.
pub async fn fetch(config: &Config, manifest: &Manifest, game: &str) -> Option<StrictPath> {
    if let Some(cached) = cached(game) {
        return Some(cached);
    }

    let security = config.runtime.network_security;
    let client = get_reqwest_client(security);

    let mut urls = direct_urls(config, manifest, game);
    if urls.is_empty() || config.covers.always_check_databases {
        if let Some(url) = steamgriddb_url(config, manifest, game, security).await {
            urls.push(url);
        }
        if let Some(url) = igdb_url(config, game, security).await {
            urls.push(url);
        }
    }

    for url in urls {
        log::trace!("[{game}] trying cover: {url}");
        let Ok(response) = client.get(&url).send().await else {
            continue;
        };
        if !response.status().is_success() {
            continue;
        }
        let Ok(bytes) = response.bytes().await else { continue };
        if bytes.len() < MIN_BYTES {
            continue;
        }

        // Decoding also confirms that we really got an image and not an error page.
        let Ok(image) = image::load_from_memory(&bytes) else {
            log::debug!("[{game}] not an image: {url}");
            continue;
        };

        let target = cached_path(game);
        if target.parent().map(|x| x.create_dirs().is_err()).unwrap_or(true) {
            return None;
        }
        if to_portrait(image)
            .save_with_format(target.as_std_path_buf().ok()?, image::ImageFormat::Png)
            .is_ok()
        {
            log::debug!("[{game}] saved cover from {url}");
            return Some(target);
        }
    }

    // Remember that we came up empty, so we don't ask again every time.
    let marker = missing_marker(game);
    if let Some(parent) = marker.parent() {
        let _ = parent.create_dirs();
    }
    let _ = marker.create();

    None
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn can_build_steam_urls_for_a_game_with_an_id() {
        let mut manifest = Manifest::default();
        manifest.0.insert(
            "Some Game".to_string(),
            crate::resource::manifest::Game {
                steam: crate::resource::manifest::SteamMetadata { id: Some(123) },
                ..Default::default()
            },
        );

        assert_eq!(
            vec![
                format!("{STEAM_CDN}/123/library_600x900.jpg"),
                format!("{STEAM_CDN}/123/header.jpg"),
            ],
            direct_urls(&Config::default(), &manifest, "Some Game")
        );
    }

    #[test]
    fn has_no_direct_urls_without_a_steam_id_or_launcher() {
        assert_eq!(
            Vec::<String>::new(),
            direct_urls(&Config::default(), &Manifest::default(), "Some Game")
        );
    }

    #[test]
    fn can_read_steams_install_records() {
        let content =
            "\"AppState\"\n{\n\t\"appid\"\t\t\"1049590\"\n\t\"universe\"\t\t\"1\"\n\t\"name\"\t\t\"Eternal Return\"\n}";

        assert_eq!(Some("1049590".to_string()), acf_value(content, "appid"));
        assert_eq!(Some("Eternal Return".to_string()), acf_value(content, "name"));
        assert_eq!(None, acf_value(content, "nonexistent"));
    }

    #[test]
    fn can_encode_a_search_term() {
        assert_eq!("Some%20Game%3A%20Part%202", urlencode("Some Game: Part 2"));
        assert_eq!("plain-name_1.0~", urlencode("plain-name_1.0~"));
    }
}
