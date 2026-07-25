# Automatic backups
Ludusavi can watch for running games and back up each one as soon as you close it,
so that you don't have to remember to do it yourself.

Enable it with the "back up automatically when a game closes" option on the "other" screen
(`watch.enabled` in the config file).

## How it works
Ludusavi periodically checks which programs are running
and compares them to the install directories of your games,
as determined by your roots and launchers.
When a game that was running is no longer running,
Ludusavi waits a moment for the game to finish writing its saves,
and then backs up that game exactly as though you had backed it up yourself,
including your retention, format, and filter settings.

Each automatic backup gets a comment with the date and how long you played,
such as `Session 2026-07-25 19:40 (2h 13m)`,
which you can see on the restore screen.

Options:

* "Show a notification after each automatic backup" (`watch.notify`)
  displays a desktop notification when a backup finishes or fails.
* "Wait after closing" (`watch.settleSeconds`)
  is how long to wait after a game closes before backing it up.
* "Check every" (`watch.pollSeconds`)
  is how often to look at the running programs. The minimum is 5 seconds.

Ludusavi only considers a program to belong to a game
when it runs from a folder named after that game
(possibly with something appended, such as a release group's suffix),
so that unrelated programs are not mistaken for games.

## Skipping running games
A game that is running may be in the middle of writing its saves,
so backing it up could store an incomplete save.
By default, Ludusavi skips games that are currently running
and tells you which ones it skipped.
You can change this with the "skip games that are currently running" option
(`watch.skipRunningGames`), which applies to manual backups as well.

## Command line
The `watch` command does the same thing without the GUI:

```
ludusavi watch
```

Use `ludusavi watch --once` to just report which games are running right now.

## Starting automatically with Windows
Enable the "watch for games when I log in" option on the "other" screen.
Ludusavi then registers itself among the programs that Windows starts at login,
running `ludusavi watch --background`,
which watches for games without showing a window.
Turning the option off removes that entry again.

You can also run it yourself at any time:

```
ludusavi watch --background
```
