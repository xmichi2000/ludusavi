//! Radar for save-like folders that don't match any known game.
//!
//! This backs the CLI's `find-unknown` command and the GUI's
//! "find unknown saves" feature on the custom games screen.

use crate::{path::StrictPath, resource::config::Config, scan::TitleFinder, scan::TitleQuery};

/// A save-like folder that doesn't match any known game.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnknownSaveCandidate {
    pub path: StrictPath,
    /// Set when the folder sits in an emulator save location,
    /// but its Steam ID doesn't match any known game.
    pub unknown_steam_id: Option<u32>,
    pub files: u64,
    pub bytes: u64,
    pub modified: Option<chrono::DateTime<chrono::Local>>,
}

impl UnknownSaveCandidate {
    /// Default title to use when adopting this folder as a custom game.
    pub fn suggested_name(&self) -> String {
        match self.unknown_steam_id {
            Some(id) => format!("Steam {id}"),
            None => self.path.leaf().unwrap_or_else(|| self.path.render()),
        }
    }
}

/// Folder names that are known to contain non-game data.
const NOISE: &[&str] = &[
    "7-Zip",
    "Adobe",
    "Comms",
    "Custom Office Templates",
    "Discord",
    "Docker",
    "Downloaded Installers",
    "EA",
    "Epic",
    "Epic Games",
    "GOG.com",
    "Google",
    "JetBrains",
    "Microsoft",
    "Mozilla",
    "My Games",
    "Notepad++",
    "npm",
    "NVIDIA",
    "NVIDIA Corporation",
    "OpenVPN",
    "Origin",
    "Packages",
    "pip",
    "PowerShell",
    "Python",
    "Slack",
    "Spotify",
    "Steam",
    "TeamViewer",
    "Temp",
    "Ubisoft",
    "users",
    "VLC",
    "WindowsPowerShell",
    "WinRAR",
    "Zoom",
];

/// Folder name prefixes that are known to contain non-game data.
const NOISE_PREFIXES: &[&str] = &["OBS", "Visual Studio"];

fn is_noise(name: &str) -> bool {
    NOISE.iter().any(|x| x.eq_ignore_ascii_case(name))
        || NOISE_PREFIXES
            .iter()
            .any(|x| name.to_lowercase().starts_with(&x.to_lowercase()))
}

fn is_hidden(path: &StrictPath) -> bool {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::fs::MetadataExt;
        const HIDDEN: u32 = 0x2;
        const SYSTEM: u32 = 0x4;

        if let Ok(metadata) = path.metadata()
            && metadata.file_attributes() & (HIDDEN | SYSTEM) != 0
        {
            return true;
        }
    }

    path.leaf().map(|x| x.starts_with('.')).unwrap_or(false)
}

/// File count, total size, and latest modification time of a folder's content,
/// along with whether the only content is a `desktop.ini` file.
fn stats(path: &StrictPath) -> (u64, u64, Option<std::time::SystemTime>, bool) {
    let mut files = 0;
    let mut bytes = 0;
    let mut modified: Option<std::time::SystemTime> = None;
    let mut only_desktop_ini = true;

    let Ok(base) = path.as_std_path_buf() else {
        return (files, bytes, modified, only_desktop_ini);
    };

    for entry in walkdir::WalkDir::new(base)
        .max_depth(5)
        .follow_links(false)
        .into_iter()
        .flatten()
    {
        if !entry.file_type().is_file() {
            continue;
        }
        files += 1;
        if !entry.file_name().to_string_lossy().eq_ignore_ascii_case("desktop.ini") {
            only_desktop_ini = false;
        }
        if let Ok(metadata) = entry.metadata() {
            bytes += metadata.len();
            if let Ok(time) = metadata.modified()
                && modified.map(|old| time > old).unwrap_or(true)
            {
                modified = Some(time);
            }
        }
    }

    (files, bytes, modified, only_desktop_ini)
}

/// Look one level deep inside common save locations
/// and report child folders that don't match any known game.
pub fn find_unknown_saves(config: &Config, title_finder: &TitleFinder) -> Vec<UnknownSaveCandidate> {
    use crate::path::CommonPath;

    // Each surface is a folder whose children we inspect,
    // plus whether its children are named after Steam app IDs.
    let mut surfaces: Vec<(StrictPath, bool)> = vec![];

    if let Some(documents) = CommonPath::Document.get() {
        surfaces.push((StrictPath::new(documents), false));
        surfaces.push((StrictPath::new(documents).joined("My Games"), false));
    }
    for base in [
        CommonPath::SavedGames,
        CommonPath::Data,
        CommonPath::DataLocal,
        CommonPath::DataLocalLow,
    ] {
        if let Some(path) = base.get() {
            surfaces.push((StrictPath::new(path), false));
        }
    }
    if let Some(public) = CommonPath::Public.get() {
        surfaces.push((StrictPath::new(public).joined("Documents"), false));
    }
    for parent in crate::scan::emulator_save_location_parents() {
        for globbed in StrictPath::new(parent).glob() {
            surfaces.push((globbed, true));
        }
    }
    for root in config.expanded_roots() {
        surfaces.push((root.games_path(), false));
    }

    let emulator_folders = crate::scan::emulator_save_location_names();

    let mut checked = std::collections::HashSet::<String>::new();
    let mut candidates = vec![];

    for (surface, emulator) in surfaces {
        let Ok(base) = surface.as_std_path_buf() else {
            continue;
        };

        for entry in walkdir::WalkDir::new(base)
            .min_depth(1)
            .max_depth(1)
            .follow_links(false)
            .into_iter()
            .flatten()
        {
            if !entry.file_type().is_dir() {
                continue;
            }

            let child = StrictPath::new(crate::path::render_pathbuf(entry.path()));
            let Some(name) = child.leaf() else {
                continue;
            };
            if !checked.insert(child.render().to_lowercase()) {
                continue;
            }
            if is_hidden(&child)
                || is_noise(&name)
                || emulator_folders.iter().any(|x| x.eq_ignore_ascii_case(&name))
                || config.is_game_blacklisted(&name)
                || config.is_unknown_save_dismissed(&child)
            {
                continue;
            }

            let mut unknown_steam_id = None;
            if emulator && let Ok(id) = name.parse::<u32>() {
                let known = title_finder
                    .find_one(TitleQuery {
                        steam_id: Some(id),
                        ..Default::default()
                    })
                    .is_some();
                if known {
                    continue;
                }
                unknown_steam_id = Some(id);
            } else {
                let known = title_finder
                    .find_one(TitleQuery {
                        names: vec![name.clone()],
                        normalized: true,
                        fuzzy: true,
                        ..Default::default()
                    })
                    .is_some();
                if known {
                    continue;
                }
            }

            let (files, bytes, modified, only_desktop_ini) = stats(&child);
            if files == 0 || only_desktop_ini {
                continue;
            }

            candidates.push(UnknownSaveCandidate {
                path: child,
                unknown_steam_id,
                files,
                bytes,
                modified: modified.map(chrono::DateTime::<chrono::Local>::from),
            });
        }
    }

    // Recently modified folders are the most likely to be active saves.
    candidates.sort_by(|a, b| {
        b.modified
            .cmp(&a.modified)
            .then_with(|| a.path.render().cmp(&b.path.render()))
    });
    candidates
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::testing::repo;

    #[test]
    fn can_detect_noise_folders() {
        assert!(is_noise("Steam"));
        assert!(is_noise("steam"));
        assert!(is_noise("7-zip"));
        assert!(is_noise("OBS Studio"));
        assert!(is_noise("Visual Studio 2022"));

        assert!(!is_noise("Celeste"));
        assert!(!is_noise("Saved Games"));
    }

    #[test]
    fn can_detect_hidden_folders() {
        assert!(is_hidden(&StrictPath::new(format!("{}/tests/home/.config", repo()))));
        assert!(!is_hidden(&StrictPath::new(format!("{}/tests/root1/game1", repo()))));
    }

    #[test]
    fn can_compute_stats() {
        let (files, bytes, modified, only_desktop_ini) =
            stats(&StrictPath::new(format!("{}/tests/root1/game1/subdir", repo())));

        assert_eq!(1, files);
        assert_eq!(2, bytes);
        assert!(modified.is_some());
        assert!(!only_desktop_ini);
    }

    #[test]
    fn can_suggest_candidate_names() {
        let candidate = UnknownSaveCandidate {
            path: StrictPath::new("C:/Users/foo/Documents/Celeste"),
            unknown_steam_id: None,
            files: 1,
            bytes: 1,
            modified: None,
        };
        assert_eq!("Celeste", candidate.suggested_name());

        let candidate = UnknownSaveCandidate {
            path: StrictPath::new("C:/Users/foo/AppData/Roaming/GSE Saves/504230"),
            unknown_steam_id: Some(504_230),
            files: 1,
            bytes: 1,
            modified: None,
        };
        assert_eq!("Steam 504230", candidate.suggested_name());
    }
}
