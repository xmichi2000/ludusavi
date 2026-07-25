# Blacklist
If you never want Ludusavi to touch a game
(for example, a multiplayer game without local saves),
then you can add it to the blacklist.
Blacklisted games are completely hidden:
they are not scanned at all,
and they do not appear in the backup/restore screens.

This is different from unchecking a game in the backup/restore screens,
which still shows the game (greyed out) and still scans it.

You can manage the blacklist in several ways:

* In the GUI, use the three-dot menu next to a game's title
  and select "Add to blacklist".
* In the GUI, use the "Blacklisted games" section on the "other" screen
  to add, edit, or remove entries.
* In the CLI, use the `config blacklist` command:

  ```
  ludusavi config blacklist add "Some Game"
  ludusavi config blacklist remove "Some Game"
  ludusavi config blacklist list
  ```

Entries are matched by exact game name,
using the game's canonical title from the manifest
(or the name of a custom game).
The CLI will resolve aliases and store IDs to the canonical title when possible;
if a name isn't recognized, it will still be added as-is,
so that you can blacklist custom or unknown games.

If you explicitly request a single game (e.g., `ludusavi backup "Some Game"`),
then it will still be processed even if it is blacklisted.
