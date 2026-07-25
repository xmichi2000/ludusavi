# Emulator saves
Some game installations use a Steam emulator (such as Goldberg/GSE, SmartSteamEmu,
CODEX, RUNE, OnlineFix, or EMPRESS),
which stores saves in its own folder instead of the game's normal save location.
The primary manifest doesn't cover these folders,
so Ludusavi additionally checks a set of well-known emulator save locations
for every game that has a Steam ID.

The locations are keyed by the game's Steam app ID.
For example, with Steam app ID 123, Ludusavi would check (among others):

* `%APPDATA%/Goldberg SteamEmu Saves/123`
* `%APPDATA%/GSE Saves/123`
* `%APPDATA%/SmartSteamEmu/123`
* `<documents>/Steam/CODEX/123`
* `<documents>/Steam/RUNE/123`
* `<documents>/OnlineFix/123`
* `<documents>/EMPRESS/123`

(`<documents>` refers to both your personal Documents folder
and the Public Documents folder.)

Any saves found there are backed up like any other save,
and restores put them back in their original location automatically.

This option is enabled by default.
If you'd like to disable it,
you can uncheck "check emulator save locations" on the "other" screen
or set `scan.emulatorSaves: false` in the config file.

## Install directory saves
Some releases store saves directly inside the game's install directory instead,
such as `<GameDir>/saves/<appid>` for HyperVisor-style releases.
To cover these, Ludusavi also checks a few common save folders
inside each game's detected install directory:

* `<GameDir>/saves`
* `<GameDir>/save`
* `<GameDir>/savegames`
* `<GameDir>/saved`

Anything found in those folders (including nested subfolders) is backed up.

This option is enabled by default.
If you'd like to disable it,
you can uncheck "check common save folders inside each game's install directory" on the "other" screen
or set `scan.installDirSaves: false` in the config file.
