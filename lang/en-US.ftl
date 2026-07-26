ludusavi = Ludusavi

language = Language
game-name = Name
total-games = Games
file-size = Size
file-location = Location
overall = Overall
status = Status

cli-unrecognized-games = No info for these games:
cli-unable-to-request-confirmation = Unable to request confirmation.
    .winpty-workaround = If you are using a Bash emulator (like Git Bash), try running winpty.
cli-backup-id-with-multiple-games = Cannot specify backup ID when restoring multiple games.
cli-invalid-backup-id = Invalid backup ID.
cli-blacklist-unrecognized-game = No info for this game, but blacklisting it anyway: {$game}
cli-blacklist-entry-not-found = Not on the blacklist: {$game}
cli-find-unknown-nothing-found = No unknown save folders found.
# One line per candidate folder reported by the `find-unknown` command.
cli-find-unknown-candidate = {$path} [files: {$files}, size: {$size}, modified: {$modified}]
# This labels a folder inside a known emulator save location
# whose Steam ID doesn't match any known game.
cli-find-unknown-steam-id = unknown Steam ID: {$id}
cli-find-unknown-adopt-hint = To back up one of these folders, adopt it as a custom game: ludusavi find-unknown --adopt <path> --name "Game Title"
cli-find-unknown-adopted = Added to custom games: {$game} ({$path})

badge-failed = Failed
badge-duplicates = Duplicates
badge-duplicated = Duplicated
badge-ignored = Ignored
badge-redirected-from = FROM: {$path}
badge-redirecting-to = TO: {$path}

some-entries-failed = Some entries failed to process; look for {badge-failed} in the output for details. Double check whether you can access those files or whether their paths are very long.

cli-game-line-item-redirected = Redirected from: {$path}
cli-game-line-item-redirecting = Redirecting to: {$path}

button-backup = Back up
button-preview = Preview
button-restore = Restore
button-nav-backup = Backups
button-nav-restore = Restoring
button-nav-custom-games = Custom games
button-nav-other = Settings
button-add-game = Add game
button-continue = Continue
button-cancel = Cancel
button-cancelling = Cancelling...
button-okay = Okay
button-select-all = Select all
button-deselect-all = Deselect all
button-enable-all = Enable all
button-disable-all = Disable all
button-customize = Customize
button-exit = Exit
button-comment = Comment
button-lock = Lock
button-unlock = Unlock
# This opens a download page.
button-get-app = Get {$app}
button-validate = Validate
button-override-manifest = Override manifest
button-extend-manifest = Extend manifest
button-sort = Sort
button-download = Download
button-upload = Upload
button-ignore = Ignore
button-blacklist = Add to blacklist
button-find-unknown-saves = Find unknown saves
# This registers a found save folder as a custom game.
button-adopt = Adopt
button-dismiss = Dismiss

# This heads the list of results from the unknown-saves scan.
label-unknown-saves = Unknown save folders:
no-unknown-saves-found = No unknown save folders found.

no-roots-are-configured = Add some roots to back up even more data.

config-is-invalid = Error: The config file is invalid.
manifest-is-invalid = Error: The manifest file is invalid.
manifest-cannot-be-updated = Error: Unable to check for an update to the manifest file. Is your Internet connection down?
cannot-prepare-backup-target = Error: Unable to prepare backup target (either creating or emptying the folder). If you have the folder open in your file browser, try closing it: {$path}
restoration-source-is-invalid = Error: The restoration source is invalid (either doesn't exist or isn't a directory). Please double check the location: {$path}
registry-issue = Error: Some registry entries were skipped.
unable-to-browse-file-system = Error: Unable to browse on your system.
unable-to-open-directory = Error: Unable to open directory:
unable-to-open-url = Error: Unable to open URL:
unable-to-configure-cloud = Unable to configure cloud.
unable-to-synchronize-with-cloud = Unable to synchronize with cloud.
cloud-synchronize-conflict = Your local and cloud backups are in conflict. Perform an upload or download to resolve this.

command-unlaunched = Command did not launch: {$command}
command-terminated = Command terminated abruptly: {$command}
command-failed = Command failed with code {$code}: {$command}

processed-games = {$total-games} {$total-games ->
    [one] game
    *[other] games
}
processed-games-subset = {$processed-games} of {$total-games} {$total-games ->
    [one] game
    *[other] games
}
processed-size-subset = {$processed-size} of {$total-size}

field-backup-target = Back up to:
field-restore-source = Restore from:
field-custom-files = Paths:
field-custom-registry = Registry:
field-sort = Sort:
field-redirect-source =
    .placeholder = Source (original location)
field-redirect-target =
    .placeholder = Target (new location)
field-roots = Roots:
field-backup-excluded-items = Backup exclusions:
# Games on the blacklist are completely hidden from scans and the interface.
field-blacklisted-games = Blacklisted games:
# These are extra save path templates (globs) checked for every game with a Steam ID.
# Placeholders like <steamId> and <winDocuments> should not be translated.
field-emulator-save-templates = Custom emulator save paths:
field-redirects = Redirects:
# This appears next to the number of full backups that you'd like to keep.
# A full backup includes all save files for a game.
field-retention-full = Full:
# This appears next to the number of differential backups that you'd like to keep.
# A differential backup includes only the files that have changed since the last full backup.
field-retention-differential = Differential:
nav-dashboard = Dashboard
button-refresh = Refresh
field-dashboard-games = Games with backups:
field-dashboard-restore-points = Restore points:
field-dashboard-latest = Latest backup:
field-dashboard-earliest = History goes back to:
field-dashboard-automatic-backups = Automatic backups:
field-dashboard-cloud = Cloud:
field-dashboard-cloud-synced = Last cloud sync:
field-dashboard-unknown-saves = Unknown save folders:
label-dashboard-on = On
label-dashboard-off = Off
check-find-unknown-saves-on-startup = Look for unknown save folders on startup
# This appears as a notification when Ludusavi finds save folders
# that don't match any known game.
notify-unknown-saves-found =
    { $total ->
        [one] Found 1 unknown save folder
       *[other] Found { $total } unknown save folders
    }
check-watch-at-login = Watch for games when I log in
autostart-failed = Unable to change whether Ludusavi runs at login: { $message }
check-watch-enabled = Back up automatically when a game closes
check-watch-notify = Show a notification after each automatic backup
check-watch-skip-running-games = Skip games that are currently running
field-watch-settle-seconds = Wait after closing (seconds):
field-watch-poll-seconds = Check every (seconds):
cli-backup-skipped-running-games = Skipped because these games are running: { $game }
cli-watch-started = Watching for running games. Press ctrl+C to stop.
cli-watch-nothing-running = No known games are running right now.
cli-watch-backing-up = Backing up { $game }...
cli-watch-backed-up = Backed up { $game }
cli-watch-backup-failed = Unable to back up { $game }
# This appears next to a checkbox for deleting old backups based on their age
# instead of based on the number of full backups to keep.
field-retention-time-based = Time-based:
# This appears next to the number of days for which every backup is kept.
field-retention-keep-all-days = Keep all (days):
# This appears next to the number of days for which one backup per day is kept.
field-retention-keep-daily-days = Daily (days):
# This appears next to the number of weeks for which one backup per week is kept.
field-retention-keep-weekly-weeks = Weekly (weeks):
field-backup-format = Format:
field-backup-compression = Compression:
# The compression level determines how much compresison we perform.
field-backup-compression-level = Level:

label-manifest = Manifest
# This shows the time when we checked for an update to the manifest.
label-checked = Checked
# This shows the time when we found an update to the manifest.
label-updated = Updated
label-new = New
label-removed = Removed
label-comment = Comment
label-unchanged = Unchanged
label-backup = Backup
label-scan = Scan
label-filter  = Filter
label-unique = Unique
label-complete = Complete
label-partial = Partial
label-enabled = Enabled
label-disabled = Disabled
# https://en.wikipedia.org/wiki/Thread_(computing)
label-threads = Threads
label-cloud = Cloud
# A "remote" is what Rclone calls cloud systems like Google Drive.
label-remote = Remote
label-remote-name = Remote name
label-folder = Folder
# An executable file
label-executable = Executable
# Options given to a command line program
label-arguments = Arguments
label-url = URL
# https://en.wikipedia.org/wiki/Host_(network)
label-host = Host
# https://en.wikipedia.org/wiki/Port_(computer_networking)
label-port = Port
label-username = Username
label-password = Password
# This is a specific website or service that provides some cloud functionality.
# For example, Nextcloud and Owncloud are providers of WebDAV services.
label-provider = Provider
label-custom = Custom
label-none = None
label-change-count = Changes: {$total}
label-unscanned = Unscanned
# This refers to a local file on the computer
label-file = File
label-game = Game
# Aliases are alternative titles for the same game.
label-alias = Alias
label-original-name = Original name
# Which manifest a game's data came from
label-source = Source
# This refers to the main Ludusavi manifest: https://github.com/mtkennerly/ludusavi-manifest
label-primary-manifest = Primary manifest
# This refers to how we integrate a custom game with the manifest data.
label-integration = Integration
# This is a folder name where a specific game is installed
label-installed-name = Installed name

store-ea = EA
store-epic = Epic
store-gog = GOG
store-gog-galaxy = GOG Galaxy
store-heroic = Heroic
store-legendary = Legendary
store-lutris = Lutris
store-microsoft = Microsoft
store-origin = Origin
store-prime = Prime Gaming
store-steam = Steam
store-uplay = Uplay
store-other-home = Home folder
# This would be a folder acting as a virtual C: drive, created by Wine.
store-other-wine = Wine prefix
# This would be a folder with typical Windows system folders,
# like "Program Files (x86)" and "Users".
store-other-windows = Windows drive
# This would be a folder with typical Linux system folders,
# like "home" and "opt".
store-other-linux = Linux drive
# This would be a folder with typical Mac system folders,
# like "Applications" and "Users".
store-other-mac = Mac drive
store-other = Other

backup-format-simple = Simple
backup-format-zip = Zip

compression-none = None
# "Deflate" is a proper noun: https://en.wikipedia.org/wiki/Deflate
compression-deflate = Deflate
compression-bzip2 = Bzip2
compression-zstd = Zstd

theme = Theme
theme-light = Light
theme-dark = Dark

redirect-bidirectional = Bidirectional
reverse-redirects-when-restoring = Reverse sequence of redirects when restoring

show-disabled-games = Show disabled games
show-unchanged-games = Show unchanged games
show-unscanned-games = Show unscanned games
check-emulator-saves = Check emulator save locations (Goldberg, CODEX, etc.)
check-install-dir-saves = Check common save folders inside each game's install directory (saves, save, savegames, saved)
override-max-threads = Override max threads
synchronize-automatically = Synchronize automatically
prefer-alias-display = Display alias instead of original name
skip-unconstructive-backups = Skip backup when data would be removed, but not added or updated

explanation-for-exclude-store-screenshots =
    In backups, exclude store-specific screenshots

explanation-for-exclude-cloud-games =
    Do not back up games with cloud support on these platforms

consider-doing-a-preview =
    If you haven't already, consider doing a preview first so that there
    are no surprises.

confirm-backup =
    Are you sure you want to proceed with the backup? {$path-action ->
        [merge] New save data will be merged into the target folder:
        *[create] The target folder will be created:
    }

confirm-restore =
    Are you sure you want to proceed with the restoration?
    This will overwrite any current files with the backups from here:

confirm-cloud-upload =
    Do you want to replace your cloud files with your local files?
    Your cloud files ({$cloud-path}) will become an exact copy of your local files ({$local-path}).
    Files in the cloud will be updated or deleted as necessary.

confirm-cloud-download =
    Do you want to replace your local files with your cloud files?
    Your local files ({$local-path}) will become an exact copy of your cloud files ({$cloud-path}).
    Local files will be updated or deleted as necessary.

confirm-add-missing-roots = Add these roots?
no-missing-roots = No additional roots found.
loading = Loading...
preparing-backup-target = Preparing backup directory...
updating-manifest = Updating manifest...
no-cloud-changes = No changes to synchronize
backups-are-valid =
    Your backups are valid.
backups-are-invalid =
    These games' backups appear to be invalid.
    Do you want to create new full backups for these games?

saves-found = Save data found.
no-saves-found = No save data found.

# This is tacked on to form something like "Back up (no confirmation)",
# meaning we would perform an action without asking the user if they're sure.
suffix-no-confirmation = no confirmation

# This is shown when a setting will only take effect after closing and reopening Ludusavi.
suffix-restart-required = restart required

prefix-error = Error: {$message}
prefix-warning = Warning: {$message}

cloud-app-unavailable = Cloud backups are disabled because {$app} is not available.
cloud-not-configured = Cloud backups are disabled because no cloud system is configured.
cloud-path-invalid = Cloud backups are disabled because the backup path is invalid.

game-is-unrecognized = Ludusavi does not recognize this game.
game-has-nothing-to-restore = This game does not have a backup to restore.
launch-game-after-error = Launch the game anyway?
game-did-not-launch = Game failed to launch.
backup-is-newer-than-current-data = The existing backup is newer than the current data.
backup-is-older-than-current-data = The existing backup is older than the current data.

back-up-specific-game =
    .confirm = Back up save data for {$game}?
    .failed = Failed to back up save data for {$game}
restore-specific-game =
    .confirm = Restore save data for {$game}?
    .failed = Failed to restore save data for {$game}

new-version-check = Check for application updates automatically
new-version-available = An application update is available: {$version}. Would you like to view the release notes?

custom-game-will-override = This custom game overrides a manifest entry
custom-game-will-extend = This custom game extends a manifest entry

operation-will-only-include-listed-games = This will only process the games that are currently listed
check-show-covers = Show cover art for games
cli-validate-all-good =
    { $total ->
        [one] The backup for 1 game is intact.
       *[other] The backups for { $total } games are intact.
    }
cli-validate-faulty =
    { $total ->
        [one] The backup for 1 game is damaged. Back up that game again to replace it.
       *[other] The backups for { $total } games are damaged. Back those games up again to replace them.
    }
label-unknown-saves-scanning = Looking for unknown save folders...
field-automatic-backups = Automatic backups
field-interface = Interface
label-dashboard-health-good = Backups are up to date
label-dashboard-health-stale = Backups are getting old
label-dashboard-health-missing = Nothing has been backed up yet
label-no-games-scanned-yet = No games scanned yet. Use "preview" to see what would be backed up.
label-no-backups-yet = No backups yet. Back up some games first.
label-no-games-match-filter = No games match the current filter.
cli-watch-unidentified = These programs run from your game folders, but no game matched them. If any of them is a game, add it as a custom game so that Ludusavi can watch it:
field-locations = Locations
label-locations-differ = Backups are written to the first folder, but the restore screen reads the second one. Set them to the same folder unless you mean to keep them apart.
check-download-covers = Download missing covers
label-cover-databases = Optional: keys for cover databases, which cover games that Steam does not list. SteamGridDB offers free keys; IGDB uses a Twitch client ID and secret.
field-steamgriddb-key = SteamGridDB key:
field-igdb-client-id = IGDB client ID:
field-igdb-client-secret = IGDB client secret:
button-delete-custom-game = Delete this custom game
# Shown when the list is filtered to one game and the games that share its saves.
label-showing-duplicates-of = Showing only games that share saves with { $game }.
button-show-all-games = Show all games
button-choose-cover-file = Choose cover image...
button-choose-cover-url = Cover image from address...
cover-not-usable = That file could not be used as a cover image.
label-dashboard-automatic-backups-explanation = Ludusavi watches which games are running and backs one up as soon as you close it, so you never have to remember. Details are on the settings screen.
label-dashboard-cloud-never-synced = Not yet
