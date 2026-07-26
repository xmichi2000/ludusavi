//! Cover art for games, taken from what is already on your computer.
//!
//! Steam stores cover images for your games in its own cache,
//! so we can reuse those instead of downloading anything.

use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use crate::{
    path::StrictPath,
    resource::{
        config::Config,
        manifest::{Manifest, Store},
    },
};

/// File names to look for, in order of preference.
/// The first is a portrait cover, and the second is a wide header image.
const CANDIDATES: &[&str] = &["library_600x900.jpg", "header.jpg"];

/// Looking for a cover means touching the file system,
/// which is too slow to repeat while drawing the game list.
static CACHE: LazyLock<Mutex<HashMap<String, Option<StrictPath>>>> = LazyLock::new(Mutex::default);

/// Forget the covers we've found so far, such as when the roots change.
pub fn clear_cache() {
    if let Ok(mut cache) = CACHE.lock() {
        cache.clear();
    }
}

/// Where Steam keeps the cover images it has downloaded.
fn steam_cover(config: &Config, manifest: &Manifest, game: &str) -> Option<StrictPath> {
    let steam_id = manifest.0.get(game)?.steam.id?;

    for root in config.expanded_roots() {
        if root.store() != Store::Steam {
            continue;
        }

        for candidate in CANDIDATES {
            let path = root
                .path()
                .joined("appcache")
                .joined("librarycache")
                .joined(steam_id.to_string())
                .joined(candidate);
            if path.is_file() {
                return Some(path);
            }
        }
    }

    None
}

/// Knowing where games are installed is expensive to work out,
/// so we only do it once, and only if we need it.
static INSTALLS: LazyLock<Mutex<Option<crate::scan::watcher::GameIndex>>> = LazyLock::new(Mutex::default);

/// Which program to take an icon from for a game.
fn game_executable(config: &Config, manifest: &Manifest, game: &str) -> Option<StrictPath> {
    let mut installs = INSTALLS.lock().ok()?;
    let index = installs.get_or_insert_with(|| {
        // Only the launchers need this, and they just map their own IDs to titles.
        let title_finder = crate::scan::TitleFinder::new(config, manifest, Default::default());
        crate::scan::watcher::GameIndex::build(config, manifest, &title_finder)
    });
    let install_dir = index.install_dir_of(game)?;

    let mut best: Option<(u64, StrictPath)> = None;
    for entry in install_dir.read_dir().ok()?.filter_map(|x| x.ok()) {
        let path = StrictPath::from(entry.path());
        if !path.render().to_lowercase().ends_with(".exe") {
            continue;
        }

        // Bigger programs tend to be the game itself rather than a helper.
        let size = entry.metadata().map(|x| x.len()).unwrap_or(0);
        if best.as_ref().map(|(best, _)| size > *best).unwrap_or(true) {
            best = Some((size, path));
        }
    }

    best.map(|(_, path)| path)
}

/// An icon taken from the game's own program, saved next to our other data.
fn executable_icon(config: &Config, manifest: &Manifest, game: &str) -> Option<StrictPath> {
    let target = crate::prelude::app_dir()
        .joined("covers")
        .joined(format!("{}.png", crate::scan::layout::escape_folder_name(game)));
    if target.is_file() {
        return Some(target);
    }

    let executable = game_executable(config, manifest, game)?;
    crate::gui::exe_icon::save_as_png(&executable, &target)?;
    Some(target)
}

fn look_up(config: &Config, manifest: &Manifest, game: &str) -> Option<StrictPath> {
    steam_cover(config, manifest, game).or_else(|| executable_icon(config, manifest, game))
}

/// The cover image for a game, if we can find one on this computer.
pub fn find(config: &Config, manifest: &Manifest, game: &str) -> Option<StrictPath> {
    let mut cache = CACHE.lock().ok()?;

    if let Some(cached) = cache.get(game) {
        return cached.clone();
    }

    let found = look_up(config, manifest, game);
    cache.insert(game.to_string(), found.clone());
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This only checks the lookup itself; there may not be any covers on the test machine.
    #[test]
    fn ignores_games_without_a_steam_id() {
        let config = Config::default();
        let manifest = Manifest::default();
        assert_eq!(None, look_up(&config, &manifest, "Nonexistent Game"));
    }
}
