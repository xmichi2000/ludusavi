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

fn look_up(config: &Config, manifest: &Manifest, game: &str) -> Option<StrictPath> {
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
