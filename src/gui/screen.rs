use std::collections::HashSet;

use iced::{Alignment, Length, keyboard, padding, widget::tooltip};

use crate::{
    cloud::{Remote, RemoteChoice},
    gui::{
        badge::Badge,
        button,
        common::{BrowseFileSubject, BrowseSubject, Message, Operation, ScrollSubject, UndoSubject},
        editor,
        game_list::GameList,
        icon::Icon,
        search::CustomGamesFilter,
        shortcuts::TextHistories,
        style,
        widget::{
            Button, Column, Container, Element, IcedParentExt, Row, Tooltip, checkbox, number_input, pick_list, text,
        },
    },
    lang::{Language, TRANSLATOR},
    prelude::{AVAILABLE_PARALELLISM, STEAM_DECK},
    resource::{
        cache::Cache,
        config::{self, BackupFormat, CloudFilter, Config, SortKey, Theme, ZipCompression},
        manifest::{Manifest, Store},
    },
    scan::{DuplicateDetector, Duplication, OperationStatus, ScanChange, ScanKind, radar::UnknownSaveCandidate},
};

const RCLONE_URL: &str = "https://rclone.org/downloads";
const RELEASE_URL: &str = "https://github.com/mtkennerly/ludusavi/releases";

fn template(content: Column) -> Element {
    Container::new(content.spacing(15).align_x(Alignment::Center))
        .height(Length::Fill)
        .center_x(Length::Fill)
        .padding(padding::all(5))
        .into()
}

/// A quiet summary line. This is context, not the headline,
/// so it doesn't shout. See docs/design-system.md.
fn make_status_row<'a>(status: &OperationStatus, duplication: Duplication) -> Row<'a> {
    Row::new()
        .padding([0, 16])
        .align_y(Alignment::Center)
        .spacing(8)
        .push(text(TRANSLATOR.processed_games(status)).size(14))
        .push_if(status.changed_games.new > 0, || {
            Badge::new_entry_with_count(status.changed_games.new).view()
        })
        .push_if(status.changed_games.different > 0, || {
            Badge::changed_entry_with_count(status.changed_games.different).view()
        })
        .push(text("·").size(14))
        .push(text(TRANSLATOR.processed_bytes(status)).size(14))
        .push_if(!duplication.resolved(), || {
            Badge::new(&TRANSLATOR.badge_duplicates()).view()
        })
}

#[derive(Default)]
pub struct Backup {
    pub log: GameList,
    pub previewed_games: HashSet<String>,
    pub duplicate_detector: DuplicateDetector,
}

impl Backup {
    const SCAN_KIND: ScanKind = ScanKind::Backup;

    pub fn new(config: &Config, cache: &Cache) -> Self {
        Self {
            log: GameList::with_recent_games(Self::SCAN_KIND, config, cache),
            ..Default::default()
        }
    }

    pub fn view(
        &self,
        config: &Config,
        manifest: &Manifest,
        operation: &Operation,
        histories: &TextHistories,
        modifiers: &keyboard::Modifiers,
    ) -> Element {
        let sort = &config.backup.sort;

        let duplicatees = self.log.duplicatees(&self.duplicate_detector);

        let content = Column::new()
            .push(
                Row::new()
                    .padding([0, 20])
                    .spacing(20)
                    .align_y(Alignment::Center)
                    .push(button::backup_preview(operation, self.log.is_filtered()))
                    .push(button::backup(operation, self.log.is_filtered()))
                    .push(button::toggle_all_scanned_games(
                        self.log.all_visible_entries_selected(
                            config,
                            Self::SCAN_KIND,
                            manifest,
                            &self.duplicate_detector,
                            duplicatees.as_ref(),
                        ),
                        self.log.is_filtered(),
                    ))
                    .push(button::filter(self.log.search.show)),
            )
            .push(make_status_row(
                &self.log.compute_operation_status(
                    config,
                    Self::SCAN_KIND,
                    manifest,
                    &self.duplicate_detector,
                    duplicatees.as_ref(),
                ),
                self.duplicate_detector.overall(),
            ))
            .push(
                Row::new()
                    .padding([0, 20])
                    .spacing(20)
                    .align_y(Alignment::Center)
                    .push(text(TRANSLATOR.backup_target_label()))
                    .push(histories.input(UndoSubject::BackupTarget))
                    .push(button::choose_folder(BrowseSubject::BackupTarget, modifiers))
                    .push("|")
                    .push(text(TRANSLATOR.sort_label()))
                    .push(
                        pick_list(SortKey::ALL, Some(sort.key), Message::config(config::Event::SortKey))
                            .class(style::PickList::Primary),
                    )
                    .push(button::sort_order(sort.reversed)),
            )
            .push(self.log.view(
                Self::SCAN_KIND,
                config,
                manifest,
                &self.duplicate_detector,
                duplicatees.as_ref(),
                operation,
                histories,
                modifiers,
            ));

        template(content)
    }
}

#[derive(Default)]
pub struct Restore {
    pub log: GameList,
    pub duplicate_detector: DuplicateDetector,
}

impl Restore {
    const SCAN_KIND: ScanKind = ScanKind::Restore;

    pub fn new(config: &Config, cache: &Cache) -> Self {
        Self {
            log: GameList::with_recent_games(Self::SCAN_KIND, config, cache),
            ..Default::default()
        }
    }

    pub fn view(
        &self,
        config: &Config,
        manifest: &Manifest,
        operation: &Operation,
        histories: &TextHistories,
        modifiers: &keyboard::Modifiers,
    ) -> Element {
        let sort = &config.restore.sort;

        let duplicatees = self.log.duplicatees(&self.duplicate_detector);

        let content = Column::new()
            .push(
                Row::new()
                    .padding([0, 20])
                    .spacing(20)
                    .align_y(Alignment::Center)
                    .push(button::restore_preview(operation, self.log.is_filtered()))
                    .push(button::restore(operation, self.log.is_filtered()))
                    .push(button::toggle_all_scanned_games(
                        self.log.all_visible_entries_selected(
                            config,
                            Self::SCAN_KIND,
                            manifest,
                            &self.duplicate_detector,
                            duplicatees.as_ref(),
                        ),
                        self.log.is_filtered(),
                    ))
                    .push(button::validate_backups(operation))
                    .push(button::filter(self.log.search.show)),
            )
            .push(make_status_row(
                &self.log.compute_operation_status(
                    config,
                    Self::SCAN_KIND,
                    manifest,
                    &self.duplicate_detector,
                    duplicatees.as_ref(),
                ),
                self.duplicate_detector.overall(),
            ))
            .push(
                Row::new()
                    .padding([0, 20])
                    .spacing(20)
                    .align_y(Alignment::Center)
                    .push(text(TRANSLATOR.restore_source_label()))
                    .push(histories.input(UndoSubject::RestoreSource))
                    .push(button::choose_folder(BrowseSubject::RestoreSource, modifiers))
                    .push("|")
                    .push(text(TRANSLATOR.sort_label()))
                    .push(
                        pick_list(SortKey::ALL, Some(sort.key), Message::config(config::Event::SortKey))
                            .class(style::PickList::Primary),
                    )
                    .push(button::sort_order(sort.reversed)),
            )
            .push(self.log.view(
                Self::SCAN_KIND,
                config,
                manifest,
                &self.duplicate_detector,
                duplicatees.as_ref(),
                operation,
                histories,
                modifiers,
            ));

        template(content)
    }
}

/// A summary of how your backups are doing, shown on the dashboard.
#[derive(Clone, Debug, Default)]
pub struct DashboardStatus {
    /// Games that have at least one backup.
    pub games: usize,
    /// How many restore points there are in total.
    pub restore_points: usize,
    /// When the most recent backup of any game was made.
    pub latest: Option<chrono::DateTime<chrono::Utc>>,
    /// When the oldest still-retained backup was made.
    pub earliest: Option<chrono::DateTime<chrono::Utc>>,
}

/// How healthy the backups look at a glance.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DashboardHealth {
    /// Backed up recently.
    Good,
    /// It has been a while.
    Stale,
    /// Nothing has been backed up at all.
    Missing,
}

impl DashboardHealth {
    /// Backups older than this are worth pointing out.
    const STALE_DAYS: i64 = 7;

    fn evaluate(status: &DashboardStatus, now: chrono::DateTime<chrono::Utc>) -> Self {
        match status.latest {
            None => Self::Missing,
            Some(latest) if (now - latest).num_days() > Self::STALE_DAYS => Self::Stale,
            Some(_) => Self::Good,
        }
    }

    fn label(&self) -> String {
        match self {
            Self::Good => TRANSLATOR.dashboard_health_good(),
            Self::Stale => TRANSLATOR.dashboard_health_stale(),
            Self::Missing => TRANSLATOR.dashboard_health_missing(),
        }
    }

    /// Reuse the colors that the game list already uses for scan changes:
    /// added for healthy, positive for stale, negative for missing.
    fn color(&self) -> ScanChange {
        match self {
            Self::Good => ScanChange::New,
            Self::Stale => ScanChange::Different,
            Self::Missing => ScanChange::Removed,
        }
    }
}

impl DashboardStatus {
    pub fn gather(config: &Config) -> Self {
        let layout = crate::scan::layout::BackupLayout::new(config.backup.path.clone());

        let mut status = Self::default();
        for game in layout.restorable_games() {
            let game_layout = layout.game_layout(&game);
            if game_layout.backups().is_empty() {
                continue;
            }

            status.games += 1;
            for full in game_layout.backups() {
                status.restore_points += 1 + full.children.len();

                let newest = full.children.back().map(|x| x.when).unwrap_or(full.when);
                status.latest = Some(status.latest.map(|x| x.max(newest)).unwrap_or(newest));
                status.earliest = Some(status.earliest.map(|x| x.min(full.when)).unwrap_or(full.when));
            }
        }

        status
    }
}

#[derive(Default)]
pub struct Dashboard {
    /// The last gathered status, if any.
    pub status: Option<DashboardStatus>,
    /// Whether we are gathering the status right now.
    pub refreshing: bool,
}

impl Dashboard {
    pub fn view<'a>(&self, config: &Config, cache: &Cache, unknown_saves: Option<usize>) -> Element<'a> {
        fn line<'a>(label: String, value: String) -> Row<'a> {
            Row::new()
                .spacing(15)
                .align_y(Alignment::Center)
                .push(Container::new(text(label)).align_right(220))
                .push(text(value))
        }

        fn when(value: Option<chrono::DateTime<chrono::Utc>>) -> String {
            match value {
                None => "-".to_string(),
                Some(value) => chrono::DateTime::<chrono::Local>::from(value)
                    .format("%Y-%m-%d %H:%M")
                    .to_string(),
            }
        }

        /// A headline number with a caption underneath.
        fn card<'a>(value: String, caption: String) -> Container<'a> {
            Container::new(
                Column::new()
                    .padding(16)
                    .spacing(8)
                    .align_x(Alignment::Center)
                    .push(text(value).size(28))
                    .push(text(caption).size(13)),
            )
            .center_x(200.0)
            .class(style::Container::GameListEntry)
        }

        let status = self.status.clone().unwrap_or_default();
        let health = DashboardHealth::evaluate(&status, chrono::Utc::now());

        let content = Column::new()
            .push(
                Container::new(
                    Row::new()
                        .padding(8)
                        .spacing(15)
                        .align_y(Alignment::Center)
                        .push(text(health.label()).size(16))
                        .push(text(when(status.latest)).size(16)),
                )
                .class(style::Container::ChangeBadge {
                    change: health.color(),
                    faded: false,
                }),
            )
            .push(
                Row::new()
                    .spacing(15)
                    .push(card(status.games.to_string(), TRANSLATOR.dashboard_games_label()))
                    .push(card(
                        status.restore_points.to_string(),
                        TRANSLATOR.dashboard_restore_points_label(),
                    ))
                    .push(card(
                        match unknown_saves {
                            Some(total) => total.to_string(),
                            None => "-".to_string(),
                        },
                        TRANSLATOR.dashboard_unknown_saves_label(),
                    )),
            )
            .push(
                Container::new(
                    Column::new()
                        .padding(5)
                        .spacing(10)
                        .push(line(TRANSLATOR.backup_target_label(), config.backup.path.render()))
                        .push(line(TRANSLATOR.dashboard_earliest_label(), when(status.earliest))),
                )
                .class(style::Container::GameListEntry),
            )
            .push(
                Container::new(
                    Column::new()
                        .padding(5)
                        .spacing(10)
                        .push(line(
                            TRANSLATOR.dashboard_automatic_backups_label(),
                            if config.watch.enabled {
                                TRANSLATOR.dashboard_on()
                            } else {
                                TRANSLATOR.dashboard_off()
                            },
                        ))
                        .push(line(
                            TRANSLATOR.dashboard_cloud_label(),
                            match config.cloud.remote.as_ref() {
                                Some(remote) => format!("{} ({})", remote.id(), config.cloud.path),
                                None => TRANSLATOR.dashboard_off(),
                            },
                        ))
                        .push_if(config.cloud.remote.is_some(), || {
                            line(TRANSLATOR.dashboard_cloud_synced_label(), when(cache.cloud.synced))
                        }),
                )
                .class(style::Container::GameListEntry),
            )
            .push(button::refresh_dashboard(self.refreshing));

        template(content)
    }
}

#[derive(Default)]
pub struct CustomGames {
    pub filter: CustomGamesFilter,
    /// Whether an unknown-saves scan is currently running.
    pub scanning_unknown_saves: bool,
    /// Results of the last unknown-saves scan, if any.
    pub unknown_saves: Option<Vec<UnknownSaveCandidate>>,
}

impl CustomGames {
    pub fn view<'a>(
        &'a self,
        config: &Config,
        manifest: &Manifest,
        operating: bool,
        histories: &'a TextHistories,
        modifiers: &keyboard::Modifiers,
    ) -> Element<'a> {
        let content = Column::new()
            .push(
                Row::new()
                    .padding([0, 20])
                    .spacing(20)
                    .align_y(Alignment::Center)
                    .push(button::add_game())
                    .push(button::find_unknown_saves(self.scanning_unknown_saves))
                    .push(button::toggle_all_custom_games(
                        self.all_visible_game_selected(config),
                        self.is_filtered(),
                    ))
                    .push(button::sort(config::Event::SortCustomGames))
                    .push(button::filter(self.filter.enabled)),
            )
            .push(self.filter.view(histories))
            .push_if(self.scanning_unknown_saves, || {
                Row::new()
                    .spacing(10)
                    .align_y(Alignment::Center)
                    .push(text(TRANSLATOR.unknown_saves_scanning()))
            })
            .push_if(!self.scanning_unknown_saves && self.unknown_saves.is_some(), || {
                self.view_unknown_saves()
            })
            .push(editor::custom_games(
                config,
                manifest,
                operating,
                histories,
                modifiers,
                &self.filter,
            ));

        template(content)
    }

    fn view_unknown_saves(&self) -> Element<'_> {
        let candidates = self.unknown_saves.as_deref().unwrap_or_default();

        let mut column = Column::new()
            .spacing(16)
            .push(text(TRANSLATOR.unknown_saves_label()).size(20));

        if candidates.is_empty() {
            column = column.push(text(TRANSLATOR.no_unknown_saves_found()));
        }

        for (index, candidate) in candidates.iter().enumerate() {
            let modified = candidate
                .modified
                .map(|x| x.format("%Y-%m-%d").to_string())
                .unwrap_or_else(|| "?".to_string());

            // The folder name leads; the full path lives in the tooltip.
            // See docs/design-system.md.
            let path = candidate.path.render();
            let name = candidate.path.leaf().unwrap_or_else(|| path.clone());
            let detail = format!(
                "{} · {} · {}",
                TRANSLATOR.adjusted_size(candidate.bytes),
                TRANSLATOR.processed_subset(candidate.files as usize, candidate.files as usize),
                modified,
            );

            column = column.push(
                Row::new()
                    .spacing(8)
                    .align_y(Alignment::Center)
                    .push(
                        Container::new(
                            Column::new()
                                .spacing(4)
                                .push(
                                    Tooltip::new(text(name).size(16), text(path), tooltip::Position::Top)
                                        .gap(4)
                                        .class(style::Container::Tooltip),
                                )
                                .push(text(detail).size(12)),
                        )
                        .width(Length::Fill),
                    )
                    .push_if(candidate.unknown_steam_id.is_some(), || {
                        Badge::new(
                            &TRANSLATOR.cli_find_unknown_steam_id(candidate.unknown_steam_id.unwrap_or_default()),
                        )
                        .view()
                    })
                    .push(button::adopt_unknown_save(index))
                    .push(button::dismiss_unknown_save(index)),
            );
        }

        Container::new(column).padding([0, 16]).into()
    }

    fn is_filtered(&self) -> bool {
        self.filter.enabled
    }

    pub fn visible_games(&self, config: &Config) -> Vec<usize> {
        config
            .custom_games
            .iter()
            .enumerate()
            .filter_map(|(i, game)| self.filter.qualifies(game).then_some(i))
            .collect()
    }

    fn all_visible_game_selected(&self, config: &Config) -> bool {
        config
            .custom_games
            .iter()
            .filter(|game| self.filter.qualifies(game))
            .all(|x| !x.ignore)
    }
}

pub fn other<'a>(
    updating_manifest: bool,
    config: &'a Config,
    cache: &'a Cache,
    operation: &Operation,
    histories: &'a TextHistories,
    modifiers: &keyboard::Modifiers,
) -> Element<'a> {
    let is_rclone_valid = config.apps.rclone.is_valid();
    let is_cloud_configured = config.cloud.remote.is_some();
    let is_cloud_path_valid = crate::cloud::validate_cloud_path(&config.cloud.path).is_ok();

    let content = Column::new()
        .push_if(*STEAM_DECK, || {
            Row::new()
                .padding([0, 20])
                .spacing(20)
                .align_y(iced::Alignment::Center)
                .push(
                    Button::new(text(TRANSLATOR.exit_button()).align_x(iced::alignment::Horizontal::Center))
                        .on_press(Message::Exit { user: true })
                        .width(125)
                        .class(style::Button::Negative)
                        .padding(5),
                )
        })
        .push({
            let content = Column::new()
                .spacing(20)
                .padding(padding::top(0).bottom(5).left(15).right(15))
                .width(Length::Fill)
                .push(
                    Row::new()
                        .align_y(iced::Alignment::Center)
                        .spacing(20)
                        .push(text(TRANSLATOR.field_language()))
                        .push(
                            pick_list(
                                Language::ALL,
                                Some(config.language),
                                Message::config(config::Event::Language),
                            )
                            .class(style::PickList::Primary),
                        ),
                )
                .push(
                    Row::new()
                        .align_y(iced::Alignment::Center)
                        .spacing(20)
                        .push(text(TRANSLATOR.field_theme()))
                        .push(
                            pick_list(Theme::ALL, Some(config.theme), Message::config(config::Event::Theme))
                                .class(style::PickList::Primary),
                        ),
                )
                .push(
                    Row::new()
                        .align_y(iced::Alignment::Center)
                        .spacing(20)
                        .push(checkbox(
                            TRANSLATOR.new_version_check(),
                            config.release.check,
                            Message::config(config::Event::CheckRelease),
                        ))
                        .push(button::open_url_icon(RELEASE_URL.to_string())),
                )
                .push(
                    Column::new().spacing(5).push(text(TRANSLATOR.scan_field())).push(
                        Container::new(
                            Column::new()
                                .padding(5)
                                .spacing(10)
                                .push({
                                    AVAILABLE_PARALELLISM.map(|max_threads| {
                                        Column::new()
                                            .spacing(5)
                                            .push(checkbox(
                                                TRANSLATOR.override_max_threads(),
                                                config.runtime.threads.is_some(),
                                                Message::config(config::Event::OverrideMaxThreads),
                                            ))
                                            .push({
                                                config.runtime.threads.map(|threads| {
                                                    Container::new(number_input(
                                                        threads.get() as i32,
                                                        TRANSLATOR.threads_label(),
                                                        1..=(max_threads.get() as i32),
                                                        Message::config(|x| config::Event::MaxThreads(x as usize)),
                                                    ))
                                                    .padding(padding::left(35))
                                                })
                                            })
                                    })
                                })
                                .push(
                                    checkbox(
                                        TRANSLATOR.explanation_for_exclude_store_screenshots(),
                                        config.backup.filter.exclude_store_screenshots,
                                        Message::config(config::Event::ExcludeStoreScreenshots),
                                    )
                                    .class(style::Checkbox),
                                )
                                .push(checkbox(
                                    TRANSLATOR.check_emulator_saves(),
                                    config.scan.emulator_saves,
                                    Message::config(config::Event::EmulatorSaves),
                                ))
                                .push(
                                    Column::new()
                                        .spacing(5)
                                        .padding(padding::left(35))
                                        .push(text(TRANSLATOR.emulator_save_templates_label()))
                                        .push(editor::emulator_save_templates(histories)),
                                )
                                .push(checkbox(
                                    TRANSLATOR.check_install_dir_saves(),
                                    config.scan.install_dir_saves,
                                    Message::config(config::Event::InstallDirSaves),
                                ))
                                .push(checkbox(
                                    TRANSLATOR.field(&TRANSLATOR.explanation_for_exclude_cloud_games()),
                                    config.backup.filter.cloud.exclude,
                                    Message::config(move |exclude| {
                                        config::Event::CloudFilter(CloudFilter {
                                            exclude,
                                            ..config.backup.filter.cloud
                                        })
                                    }),
                                ))
                                .push(
                                    Row::new()
                                        .padding(padding::left(35))
                                        .spacing(10)
                                        .push(
                                            checkbox(
                                                TRANSLATOR.store(&Store::Epic),
                                                config.backup.filter.cloud.epic,
                                                Message::config(move |epic| {
                                                    config::Event::CloudFilter(CloudFilter {
                                                        epic,
                                                        ..config.backup.filter.cloud
                                                    })
                                                }),
                                            )
                                            .class(style::Checkbox),
                                        )
                                        .push(
                                            checkbox(
                                                TRANSLATOR.store(&Store::Gog),
                                                config.backup.filter.cloud.gog,
                                                Message::config(move |gog| {
                                                    config::Event::CloudFilter(CloudFilter {
                                                        gog,
                                                        ..config.backup.filter.cloud
                                                    })
                                                }),
                                            )
                                            .class(style::Checkbox),
                                        )
                                        .push(
                                            checkbox(
                                                format!(
                                                    "{} / {}",
                                                    TRANSLATOR.store(&Store::Origin),
                                                    TRANSLATOR.store(&Store::Ea)
                                                ),
                                                config.backup.filter.cloud.origin,
                                                Message::config(move |origin| {
                                                    config::Event::CloudFilter(CloudFilter {
                                                        origin,
                                                        ..config.backup.filter.cloud
                                                    })
                                                }),
                                            )
                                            .class(style::Checkbox),
                                        )
                                        .push(
                                            checkbox(
                                                TRANSLATOR.store(&Store::Steam),
                                                config.backup.filter.cloud.steam,
                                                Message::config(move |steam| {
                                                    config::Event::CloudFilter(CloudFilter {
                                                        steam,
                                                        ..config.backup.filter.cloud
                                                    })
                                                }),
                                            )
                                            .class(style::Checkbox),
                                        )
                                        .push(
                                            checkbox(
                                                TRANSLATOR.store(&Store::Uplay),
                                                config.backup.filter.cloud.uplay,
                                                Message::config(move |uplay| {
                                                    config::Event::CloudFilter(CloudFilter {
                                                        uplay,
                                                        ..config.backup.filter.cloud
                                                    })
                                                }),
                                            )
                                            .class(style::Checkbox),
                                        ),
                                ),
                        )
                        .class(style::Container::GameListEntry),
                    ),
                )
                .push(
                    Column::new()
                        .spacing(5)
                        .push(text(TRANSLATOR.automatic_backups_field()))
                        .push(
                            Container::new(
                                Column::new()
                                    .padding(5)
                                    .spacing(10)
                                    .push(checkbox(
                                        TRANSLATOR.watch_enabled(),
                                        config.watch.enabled,
                                        Message::config(config::Event::WatchEnabled),
                                    ))
                                    .push_if(config.watch.enabled, || {
                                        Column::new()
                                            .spacing(10)
                                            .padding(padding::left(35))
                                            .push(checkbox(
                                                TRANSLATOR.watch_notify(),
                                                config.watch.notify,
                                                Message::config(config::Event::WatchNotify),
                                            ))
                                            .push_if(ludusavi::autostart::supported(), || {
                                                checkbox(
                                                    TRANSLATOR.watch_at_login(),
                                                    ludusavi::autostart::enabled(),
                                                    Message::SetAutostart,
                                                )
                                            })
                                            .push(
                                                Row::new()
                                                    .spacing(20)
                                                    .height(30)
                                                    .align_y(Alignment::Center)
                                                    .push(number_input(
                                                        config.watch.settle_seconds as i32,
                                                        TRANSLATOR.watch_settle_seconds(),
                                                        0..=600,
                                                        Message::config(|x| {
                                                            config::Event::WatchSettleSeconds(x as u32)
                                                        }),
                                                    ))
                                                    .push(number_input(
                                                        config.watch.poll_seconds as i32,
                                                        TRANSLATOR.watch_poll_seconds(),
                                                        5..=600,
                                                        Message::config(|x| config::Event::WatchPollSeconds(x as u32)),
                                                    )),
                                            )
                                    })
                                    .push(checkbox(
                                        TRANSLATOR.watch_skip_running_games(),
                                        config.watch.skip_running_games,
                                        Message::config(config::Event::WatchSkipRunningGames),
                                    ))
                                    .push(checkbox(
                                        TRANSLATOR.find_unknown_saves_on_startup(),
                                        config.scan.find_unknown_saves_on_startup,
                                        Message::config(config::Event::FindUnknownSavesOnStartup),
                                    )),
                            )
                            .class(style::Container::GameListEntry),
                        ),
                )
                // Both paths in one place, since they are easy to mix up.
                .push(
                    Column::new().spacing(5).push(text(TRANSLATOR.locations_field())).push(
                        Container::new(
                            Column::new()
                                .padding(5)
                                .spacing(10)
                                .push(
                                    Row::new()
                                        .spacing(10)
                                        .align_y(Alignment::Center)
                                        .push(Container::new(text(TRANSLATOR.backup_target_label())).align_right(120))
                                        .push(text(config.backup.path.render()).size(14))
                                        .push(button::choose_folder(BrowseSubject::BackupTarget, modifiers)),
                                )
                                .push(
                                    Row::new()
                                        .spacing(10)
                                        .align_y(Alignment::Center)
                                        .push(Container::new(text(TRANSLATOR.restore_source_label())).align_right(120))
                                        .push(text(config.restore.path.render()).size(14))
                                        .push(button::choose_folder(BrowseSubject::RestoreSource, modifiers)),
                                )
                                .push_if(config.backup.path != config.restore.path, || {
                                    text(TRANSLATOR.locations_differ_note()).size(13)
                                }),
                        )
                        .class(style::Container::GameListEntry),
                    ),
                )
                .push(
                    Column::new().spacing(5).push(text(TRANSLATOR.interface_field())).push(
                        Container::new(
                            Column::new()
                                .padding(5)
                                .spacing(10)
                                .push(checkbox(
                                    TRANSLATOR.show_covers(),
                                    config.covers.show,
                                    Message::config(config::Event::CoversShow),
                                ))
                                .push_if(config.covers.show, || {
                                    Column::new()
                                        .spacing(10)
                                        .padding(padding::left(35))
                                        .push(checkbox(
                                            TRANSLATOR.download_covers(),
                                            config.covers.download,
                                            Message::config(config::Event::CoversDownload),
                                        ))
                                        .push_if(config.covers.download, || {
                                            Column::new()
                                                .spacing(5)
                                                .push(text(TRANSLATOR.cover_databases_note()).size(13))
                                                .push(
                                                    Row::new()
                                                        .spacing(10)
                                                        .align_y(Alignment::Center)
                                                        .push(
                                                            Container::new(text(TRANSLATOR.steamgriddb_key_label()))
                                                                .align_right(160),
                                                        )
                                                        .push(histories.input(UndoSubject::SteamGridDbKey)),
                                                )
                                                .push(
                                                    Row::new()
                                                        .spacing(10)
                                                        .align_y(Alignment::Center)
                                                        .push(
                                                            Container::new(text(TRANSLATOR.igdb_client_id_label()))
                                                                .align_right(160),
                                                        )
                                                        .push(histories.input(UndoSubject::IgdbClientId)),
                                                )
                                                .push(
                                                    Row::new()
                                                        .spacing(10)
                                                        .align_y(Alignment::Center)
                                                        .push(
                                                            Container::new(text(TRANSLATOR.igdb_client_secret_label()))
                                                                .align_right(160),
                                                        )
                                                        .push(histories.input(UndoSubject::IgdbClientSecret)),
                                                )
                                        })
                                })
                                .push(checkbox(
                                    TRANSLATOR.show_disabled_games(),
                                    config.scan.show_deselected_games,
                                    Message::config(config::Event::ShowDeselectedGames),
                                ))
                                .push(checkbox(
                                    TRANSLATOR.show_unchanged_games(),
                                    config.scan.show_unchanged_games,
                                    Message::config(config::Event::ShowUnchangedGames),
                                ))
                                .push(checkbox(
                                    TRANSLATOR.show_unscanned_games(),
                                    config.scan.show_unscanned_games,
                                    Message::config(config::Event::ShowUnscannedGames),
                                )),
                        )
                        .class(style::Container::GameListEntry),
                    ),
                )
                .push(
                    Column::new().spacing(5).push(text(TRANSLATOR.backup_field())).push(
                        Container::new(
                            Column::new()
                                .padding(5)
                                .spacing(10)
                                .push(
                                    Row::new()
                                        .spacing(20)
                                        .height(30)
                                        .align_y(Alignment::Center)
                                        .push({
                                            number_input(
                                                config.backup.retention.full as i32,
                                                TRANSLATOR.full_retention(),
                                                1..=255,
                                                Message::config(|x| config::Event::FullRetention(x as u8)),
                                            )
                                        })
                                        .push({
                                            number_input(
                                                config.backup.retention.differential as i32,
                                                TRANSLATOR.differential_retention(),
                                                0..=255,
                                                Message::config(|x| config::Event::DiffRetention(x as u8)),
                                            )
                                        })
                                        .push(checkbox(
                                            TRANSLATOR.time_based_retention(),
                                            config.backup.retention.time_based.is_some(),
                                            Message::config(config::Event::TimeBasedRetentionEnabled),
                                        )),
                                )
                                .push_if(config.backup.retention.time_based.is_some(), || {
                                    let time_based = config.backup.retention.time_based.unwrap_or_default();
                                    Row::new()
                                        .spacing(20)
                                        .height(30)
                                        .align_y(Alignment::Center)
                                        .push(number_input(
                                            time_based.keep_all_days as i32,
                                            TRANSLATOR.retention_keep_all_days(),
                                            0..=3650,
                                            Message::config(|x| config::Event::TimeBasedRetentionKeepAllDays(x as u32)),
                                        ))
                                        .push(number_input(
                                            time_based.keep_daily_days as i32,
                                            TRANSLATOR.retention_keep_daily_days(),
                                            0..=3650,
                                            Message::config(|x| {
                                                config::Event::TimeBasedRetentionKeepDailyDays(x as u32)
                                            }),
                                        ))
                                        .push(number_input(
                                            time_based.keep_weekly_weeks as i32,
                                            TRANSLATOR.retention_keep_weekly_weeks(),
                                            0..=520,
                                            Message::config(|x| {
                                                config::Event::TimeBasedRetentionKeepWeeklyWeeks(x as u32)
                                            }),
                                        ))
                                })
                                .push(
                                    Row::new()
                                        .spacing(20)
                                        .align_y(Alignment::Center)
                                        .push(
                                            Row::new()
                                                .spacing(5)
                                                .align_y(Alignment::Center)
                                                .push(text(TRANSLATOR.backup_format_field()))
                                                .push(
                                                    pick_list(
                                                        BackupFormat::ALL,
                                                        Some(config.backup.format.chosen),
                                                        Message::config(config::Event::BackupFormat),
                                                    )
                                                    .class(style::PickList::Primary),
                                                ),
                                        )
                                        .push_if(config.backup.format.chosen == BackupFormat::Zip, || {
                                            Row::new()
                                                .spacing(5)
                                                .align_y(Alignment::Center)
                                                .push(text(TRANSLATOR.backup_compression_field()))
                                                .push(
                                                    pick_list(
                                                        ZipCompression::ALL,
                                                        Some(config.backup.format.zip.compression),
                                                        Message::config(config::Event::BackupCompression),
                                                    )
                                                    .class(style::PickList::Primary),
                                                )
                                        })
                                        .push(match (config.backup.format.level(), config.backup.format.range()) {
                                            (Some(level), Some(range)) => Some(number_input(
                                                level,
                                                TRANSLATOR.backup_compression_level_field(),
                                                range,
                                                Message::config(config::Event::CompressionLevel),
                                            )),
                                            _ => None,
                                        }),
                                )
                                .push(Row::new().spacing(5).align_y(Alignment::Center).push(checkbox(
                                    TRANSLATOR.skip_unconstructive_backups(),
                                    config.backup.only_constructive,
                                    Message::config(config::Event::OnlyConstructiveBackups),
                                ))),
                        )
                        .class(style::Container::GameListEntry),
                    ),
                )
                .push(
                    Column::new()
                        .spacing(5)
                        .push(
                            Row::new()
                                .align_y(iced::Alignment::Center)
                                .push(text(TRANSLATOR.manifest_label()).width(100))
                                .push(button::refresh(
                                    Message::UpdateManifest { force: true },
                                    updating_manifest,
                                )),
                        )
                        .push(editor::manifest(config, cache, histories, modifiers).padding(padding::top(10))),
                )
                .push(
                    Column::new()
                        .spacing(5)
                        .push(
                            Row::new()
                                .align_y(iced::Alignment::Center)
                                .push(text(TRANSLATOR.cloud_field()).width(100)),
                        )
                        .push(
                            Container::new({
                                let mut column = Column::new().spacing(5).push(
                                    Row::new()
                                        .spacing(20)
                                        .align_y(Alignment::Center)
                                        .push(text(TRANSLATOR.rclone_label()).width(70))
                                        .push(histories.input(UndoSubject::RcloneExecutable))
                                        .push_if(!is_rclone_valid, || {
                                            Icon::Error.text().width(Length::Shrink).class(style::Text::Failure)
                                        })
                                        .push(button::choose_file(BrowseFileSubject::RcloneExecutable, modifiers))
                                        .push(histories.input(UndoSubject::RcloneArguments)),
                                );

                                if is_rclone_valid {
                                    let choice: RemoteChoice = config.cloud.remote.as_ref().into();
                                    column = column
                                        .push({
                                            let mut row = Row::new()
                                                .spacing(20)
                                                .align_y(Alignment::Center)
                                                .push(text(TRANSLATOR.remote_label()).width(70))
                                                .push_if(!operation.idle(), || {
                                                    text(choice.to_string())
                                                        .height(30)
                                                        .align_y(iced::alignment::Vertical::Center)
                                                })
                                                .push_if(operation.idle(), || {
                                                    pick_list(
                                                        RemoteChoice::ALL,
                                                        Some(choice),
                                                        Message::EditedCloudRemote,
                                                    )
                                                });

                                            if let Some(Remote::Custom { .. }) = &config.cloud.remote {
                                                row = row
                                                    .push(text(TRANSLATOR.remote_name_label()))
                                                    .push(histories.input(UndoSubject::CloudRemoteId));
                                            }

                                            if let Some(description) =
                                                config.cloud.remote.as_ref().and_then(|x| x.description())
                                            {
                                                row = row.push(text(description));
                                            }

                                            row
                                        })
                                        .push_if(choice != RemoteChoice::None, || {
                                            Row::new()
                                                .spacing(20)
                                                .align_y(Alignment::Center)
                                                .push(text(TRANSLATOR.folder_label()).width(70))
                                                .push(histories.input(UndoSubject::CloudPath))
                                                .push_if(!is_cloud_path_valid, || {
                                                    Icon::Error.text().width(Length::Shrink).class(style::Text::Failure)
                                                })
                                        })
                                        .push_if(is_cloud_configured && is_cloud_path_valid, || {
                                            Row::new()
                                                .spacing(20)
                                                .align_y(Alignment::Center)
                                                .push(button::upload(operation))
                                                .push(button::download(operation))
                                                .push(checkbox(
                                                    TRANSLATOR.synchronize_automatically(),
                                                    config.cloud.synchronize,
                                                    Message::config(|_| config::Event::ToggleCloudSynchronize),
                                                ))
                                        })
                                        .push_if(!is_cloud_configured, || text(TRANSLATOR.cloud_not_configured()))
                                        .push_if(!is_cloud_path_valid, || {
                                            text(TRANSLATOR.prefix_warning(&TRANSLATOR.cloud_path_invalid()))
                                                .class(style::Text::Failure)
                                        });
                                } else {
                                    column = column
                                        .push(
                                            text(TRANSLATOR.prefix_warning(&TRANSLATOR.rclone_unavailable()))
                                                .class(style::Text::Failure),
                                        )
                                        .push(button::open_url(TRANSLATOR.get_rclone_button(), RCLONE_URL.to_string()));
                                }

                                column
                            })
                            .padding(5)
                            .class(style::Container::GameListEntry),
                        ),
                )
                .push(
                    Column::new().spacing(5).push(text(TRANSLATOR.roots_label())).push(
                        Container::new(
                            Column::new()
                                .padding(5)
                                .spacing(4)
                                .push(editor::root(config, histories, modifiers)),
                        )
                        .class(style::Container::GameListEntry),
                    ),
                )
                .push(
                    Column::new()
                        .push(text(TRANSLATOR.ignored_items_label()))
                        .push(editor::ignored_items(config, histories, modifiers).padding(padding::top(10))),
                )
                .push(
                    Column::new()
                        .push(text(TRANSLATOR.blacklisted_games_label()))
                        .push(editor::blacklisted_games(histories).padding(padding::top(10))),
                )
                .push(
                    Column::new()
                        .push(text(TRANSLATOR.redirects_label()))
                        .push(editor::redirect(config, histories, modifiers).padding(padding::top(10))),
                );
            ScrollSubject::Other.into_widget(content)
        });

    template(content)
}
