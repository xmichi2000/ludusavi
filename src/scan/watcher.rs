use std::collections::{HashMap, HashSet};

use crate::{
    path::StrictPath,
    resource::{
        config::Config,
        manifest::{Manifest, Store},
    },
    scan::{Launchers, TitleFinder},
};

/// Folders that sit alongside games in a root, but aren't games themselves.
const NON_GAME_FOLDERS: &[&str] = &[
    "Launcher",
    "Epic Online Services",
    "DirectXRedist",
    "_CommonRedist",
    "CommonRedist",
    "Redist",
    "DirectX",
    "Steamworks Shared",
    "SteamVR",
    "Proton",
];

/// How long to keep the index of install directories before rebuilding it.
const INDEX_LIFETIME: chrono::TimeDelta = chrono::TimeDelta::minutes(10);

/// A game that we have seen running.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunningGame {
    pub started: chrono::DateTime<chrono::Utc>,
}

/// A game that has stopped running and is due for a backup.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FinishedGame {
    pub title: String,
    pub started: chrono::DateTime<chrono::Utc>,
    pub stopped: chrono::DateTime<chrono::Utc>,
}

impl FinishedGame {
    /// Human-readable summary of the play session, used as a backup comment.
    /// The backup itself is already labeled with its date,
    /// so this only describes how long the session lasted.
    pub fn label(&self) -> String {
        let minutes = (self.stopped - self.started).num_minutes().max(0);
        let duration = if minutes >= 60 {
            format!("{}h {}m", minutes / 60, minutes % 60)
        } else {
            format!("{minutes}m")
        };
        format!("Session ({duration})")
    }
}

/// Compare a game title to the name of a folder, ignoring spacing and punctuation.
///
/// Ludusavi's scans can afford to guess loosely at install directories,
/// since a wrong guess simply doesn't turn up any saves,
/// but the watcher needs to be sure that a process really belongs to a game,
/// or else it would treat unrelated programs (such as a launcher) as a game.
/// So we only accept a folder whose name is the title,
/// optionally with something appended (such as a release group's suffix).
fn folder_matches_title(title: &str, folder: &str) -> bool {
    fn key(value: &str) -> String {
        crate::scan::title::normalize_title(value)
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect()
    }

    let title = key(title);
    let folder = key(folder);

    !title.is_empty() && folder.starts_with(&title)
}

/// Maps executables to games by way of their install directories.
#[derive(Clone, Debug, Default)]
pub struct GameIndex {
    /// Install directory and the game it belongs to.
    entries: Vec<(StrictPath, String)>,
    /// Folders that hold games, used to notice programs we couldn't identify.
    /// Stores whose folders are managed by the system aren't included,
    /// since those are full of programs that aren't games.
    roots: Vec<StrictPath>,
    built: Option<chrono::DateTime<chrono::Utc>>,
}

impl GameIndex {
    pub fn build(config: &Config, manifest: &Manifest, title_finder: &TitleFinder) -> Self {
        let roots = config.expanded_roots();
        let titles: Vec<_> = manifest
            .primary_titles()
            .into_iter()
            .filter(|title| !config.is_game_blacklisted(title))
            .collect();
        let launchers = Launchers::scan(&roots, manifest, &titles, title_finder, None);

        let mut entries = vec![];
        for root in &roots {
            for title in &titles {
                for game in launchers.get_game(root, title) {
                    let Some(install_dir) = game.install_dir.as_ref() else {
                        continue;
                    };

                    let folder = install_dir.render();
                    let folder = folder.rsplit(['/', '\\']).next().unwrap_or_default();
                    if !folder_matches_title(title, folder) {
                        log::trace!("watcher: [{title}] ignoring uncertain install dir: {folder}");
                        continue;
                    }

                    entries.push((install_dir.clone(), title.clone()));
                }
            }
        }

        log::debug!("watcher: indexed {} install directories", entries.len());
        for (install_dir, title) in &entries {
            log::trace!("watcher: [{title}] install dir: {}", install_dir.render());
        }

        Self {
            entries,
            roots: roots
                .iter()
                .filter(|root| root.store() != Store::Microsoft)
                .map(|root| root.games_path())
                .collect(),
            built: Some(chrono::Utc::now()),
        }
    }

    /// The game folder that a program lives in, if it is inside one of your roots.
    /// This is the folder directly within the root,
    /// since that's what would become a custom game.
    pub fn game_folder_of(&self, executable: &StrictPath) -> Option<StrictPath> {
        let executable = executable.render();

        for root in &self.roots {
            let prefix = format!("{}/", root.render());
            let Some(rest) = executable.strip_prefix(&prefix) else {
                continue;
            };
            let Some(folder) = rest.split('/').next() else { continue };
            if folder.is_empty() || rest == folder || NON_GAME_FOLDERS.iter().any(|x| x.eq_ignore_ascii_case(folder)) {
                continue;
            }
            return Some(root.joined(folder));
        }

        None
    }

    pub fn is_stale(&self, now: &chrono::DateTime<chrono::Utc>) -> bool {
        match self.built {
            None => true,
            Some(built) => *now - built > INDEX_LIFETIME,
        }
    }

    /// Find the game that an executable belongs to.
    /// When install directories are nested, the most specific one wins.
    pub fn find(&self, executable: &StrictPath) -> Option<&str> {
        self.entries
            .iter()
            .filter(|(install_dir, _)| install_dir.is_prefix_of(executable))
            .max_by_key(|(install_dir, _)| install_dir.raw().len())
            .map(|(_, title)| title.as_str())
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Where a game is installed, if we know.
    pub fn install_dir_of(&self, game: &str) -> Option<&StrictPath> {
        self.entries
            .iter()
            .find(|(_, title)| title == game)
            .map(|(path, _)| path)
    }
}

/// Games that were running before, but aren't anymore.
pub fn find_stopped(
    previous: &HashMap<String, RunningGame>,
    current: &HashSet<String>,
    now: &chrono::DateTime<chrono::Utc>,
) -> Vec<FinishedGame> {
    let mut stopped: Vec<_> = previous
        .iter()
        .filter(|(title, _)| !current.contains(*title))
        .map(|(title, running)| FinishedGame {
            title: title.clone(),
            started: running.started,
            stopped: *now,
        })
        .collect();
    stopped.sort_by(|x, y| x.title.cmp(&y.title));
    stopped
}

/// Which games are running right now, based on the executables of live processes.
pub fn detect_running(index: &GameIndex) -> HashSet<String> {
    look_at_processes(index).0
}

/// Programs running from your game folders that don't belong to a known game.
/// These are worth telling the user about,
/// since they may be games that Ludusavi can't identify by folder name.
pub fn detect_unidentified(index: &GameIndex) -> Vec<StrictPath> {
    look_at_processes(index).1
}

fn look_at_processes(index: &GameIndex) -> (HashSet<String>, Vec<StrictPath>) {
    let mut system = sysinfo::System::new();
    system.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    let mut running = HashSet::new();
    let mut unidentified = vec![];
    for process in system.processes().values() {
        // Some processes are inaccessible, which is fine; we just skip them.
        let Some(executable) = process.exe() else { continue };
        let executable = StrictPath::from(executable);

        match index.find(&executable) {
            Some(title) => {
                log::debug!("watcher: [{title}] is running: {}", executable.render());
                running.insert(title.to_string());
            }
            None => {
                if let Some(folder) = index.game_folder_of(&executable)
                    && !unidentified.contains(&folder)
                {
                    log::debug!("watcher: unidentified program in: {}", folder.render());
                    unidentified.push(folder);
                }
            }
        }
    }

    unidentified.sort_by_key(|x| x.render());
    (running, unidentified)
}

/// Track which games are running, so that we can react when they stop.
#[derive(Clone, Debug, Default)]
pub struct Watcher {
    index: GameIndex,
    running: HashMap<String, RunningGame>,
}

impl Watcher {
    pub fn new() -> Self {
        Self::default()
    }

    /// Programs from your game folders that we couldn't match to a game.
    pub fn unidentified_programs(&self) -> Vec<StrictPath> {
        detect_unidentified(&self.index)
    }

    pub fn running_games(&self) -> Vec<String> {
        let mut games: Vec<_> = self.running.keys().cloned().collect();
        games.sort();
        games
    }

    /// Refresh the index of install directories if it has gotten too old.
    pub fn refresh_index(&mut self, config: &Config, manifest: &Manifest, title_finder: &TitleFinder) {
        let now = chrono::Utc::now();
        if self.index.is_stale(&now) {
            self.index = GameIndex::build(config, manifest, title_finder);
        }
    }

    /// Check what is running and report any games that have just stopped.
    pub fn tick(&mut self) -> Vec<FinishedGame> {
        let now = chrono::Utc::now();
        let current = detect_running(&self.index);
        let stopped = find_stopped(&self.running, &current, &now);

        for title in &current {
            self.running
                .entry(title.clone())
                .or_insert_with(|| RunningGame { started: now });
        }
        for game in &stopped {
            self.running.remove(&game.title);
        }

        stopped
    }
}

/// Show a desktop notification, if the platform supports it.
pub fn notify(summary: &str, body: &str) {
    log::info!("notification: {summary} - {body}");

    #[cfg(target_os = "windows")]
    {
        use tauri_winrt_notification::{Duration, Toast};

        if let Err(e) = Toast::new(Toast::POWERSHELL_APP_ID)
            .title(summary)
            .text1(body)
            .duration(Duration::Short)
            .show()
        {
            log::warn!("unable to show notification: {e:?}");
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use velcro::hash_map;

    use super::*;

    fn index(entries: &[(&str, &str)]) -> GameIndex {
        GameIndex {
            entries: entries
                .iter()
                .map(|(path, title)| (StrictPath::new(*path), title.to_string()))
                .collect(),
            roots: vec![],
            built: Some(chrono::Utc::now()),
        }
    }

    fn time(hour: u32, minute: u32) -> chrono::DateTime<chrono::Utc> {
        chrono::NaiveDate::from_ymd_opt(2024, 6, 30)
            .unwrap()
            .and_hms_opt(hour, minute, 0)
            .unwrap()
            .and_local_timezone(chrono::Utc)
            .unwrap()
    }

    #[test]
    fn can_map_an_executable_to_a_game() {
        let index = index(&[
            ("C:/Games/Some Game", "Some Game"),
            ("C:/Games/Another Game", "Another Game"),
        ]);

        assert_eq!(
            Some("Some Game"),
            index.find(&StrictPath::new("C:/Games/Some Game/bin/game.exe"))
        );
        assert_eq!(
            Some("Another Game"),
            index.find(&StrictPath::new("C:/Games/Another Game/game.exe"))
        );
        assert_eq!(None, index.find(&StrictPath::new("C:/Windows/explorer.exe")));
        assert_eq!(None, index.find(&StrictPath::new("C:/Games/Third Game/game.exe")));
    }

    #[test]
    fn prefers_the_most_specific_install_directory() {
        let index = index(&[("C:/Games", "Some Collection"), ("C:/Games/Some Game", "Some Game")]);

        assert_eq!(
            Some("Some Game"),
            index.find(&StrictPath::new("C:/Games/Some Game/game.exe"))
        );
    }

    #[test]
    fn can_tell_whether_a_folder_belongs_to_a_game() {
        // Spacing and punctuation may differ.
        assert!(folder_matches_title("Black Myth: Wukong", "BlackMythWukong"));
        assert!(folder_matches_title(
            "Jujutsu Kaisen: Cursed Clash",
            "Jujutsu Kaisen - Cursed Clash"
        ));
        assert!(folder_matches_title("Forspoken", "FORSPOKEN"));
        // Release groups may append something.
        assert!(folder_matches_title(
            "Assassin's Creed: Black Flag Resynced",
            "Assassins Creed Black Flag Resynced HV"
        ));
        // Unrelated folders must not match.
        assert!(!folder_matches_title("Fate/Hollow Ataraxia Remastered", "Launcher"));
        assert!(!folder_matches_title("JOYDOOR", "Steamworks Shared"));
        assert!(!folder_matches_title("Some Game", "Some Other Game"));
    }

    #[test]
    fn can_identify_the_game_folder_of_a_program() {
        let index = GameIndex {
            entries: vec![],
            roots: vec![StrictPath::new("C:/Games")],
            built: Some(chrono::Utc::now()),
        };

        assert_eq!(
            Some(StrictPath::new("C:/Games/Some Game")),
            index.game_folder_of(&StrictPath::new("C:/Games/Some Game/bin/game.exe"))
        );
        // Support programs alongside games are not games.
        assert_eq!(
            None,
            index.game_folder_of(&StrictPath::new("C:/Games/Launcher/launcher.exe"))
        );
        // A program directly in the root has no game folder of its own.
        assert_eq!(None, index.game_folder_of(&StrictPath::new("C:/Games/stray.exe")));
        // Elsewhere on the computer.
        assert_eq!(None, index.game_folder_of(&StrictPath::new("C:/Windows/explorer.exe")));
    }

    #[test]
    fn can_find_stopped_games() {
        let previous = hash_map! {
            "Still Running".to_string(): RunningGame { started: time(10, 0) },
            "Just Stopped".to_string(): RunningGame { started: time(9, 30) },
        };
        let current = HashSet::from_iter(["Still Running".to_string()]);

        assert_eq!(
            vec![FinishedGame {
                title: "Just Stopped".to_string(),
                started: time(9, 30),
                stopped: time(11, 0),
            }],
            find_stopped(&previous, &current, &time(11, 0))
        );
    }

    #[test]
    fn can_describe_a_session() {
        assert_eq!(
            "Session (1h 30m)",
            FinishedGame {
                title: "Some Game".to_string(),
                started: time(9, 30),
                stopped: time(11, 0),
            }
            .label()
        );

        assert_eq!(
            "Session (25m)",
            FinishedGame {
                title: "Some Game".to_string(),
                started: time(9, 30),
                stopped: time(9, 55),
            }
            .label()
        );
    }
}
