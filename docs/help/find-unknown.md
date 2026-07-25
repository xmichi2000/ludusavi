# Unknown saves
Ludusavi can look for save-like folders that don't belong to any known game,
so that nothing slips through the cracks
(for example, games without a manifest entry
or saves from unknown Steam emulators).

In the GUI, use the "find unknown saves" button on the custom games screen.
Each finding can be adopted as a custom game with one click,
or dismissed with the X button.
Dismissed folders are remembered (`scan.dismissedUnknownSaves` in the config file),
so they won't be reported again.

By default, Ludusavi also looks for unknown saves when it starts,
and notifies you if it finds any.
You can turn this off with the "look for unknown save folders on startup" option
on the "other" screen (`scan.findUnknownSavesOnStartup` in the config file).

## Command line
The `find-unknown` command does the same thing without the GUI:

```
ludusavi find-unknown
```

It looks one level deep inside these locations:

* Documents and Documents/My Games
* Saved Games
* AppData Roaming, Local, and LocalLow
* Public Documents
* Well-known emulator save locations
  (where child folders are named after Steam app IDs -
  unknown IDs are labeled as such)
* The games folder of each of your configured roots

Child folders are matched against the manifest by exact, normalized,
and fuzzy title matching (or by Steam ID inside emulator save locations).
Folders that match a known game are not reported.
Some well-known non-game folders (e.g., from common applications),
hidden/system folders, empty folders,
and folders whose name is on your blacklist are skipped as well.

For each candidate, Ludusavi prints the full path, file count, total size,
and last modified time, sorted with the most recently modified first.

If a reported folder does contain saves that you want to back up,
you can adopt it as a custom game:

```
ludusavi find-unknown --adopt "C:/path/to/folder" --name "Some Game"
```

This adds the path to a custom game with that name
(creating the custom game if it doesn't exist yet),
after which the game is included in normal backups.
