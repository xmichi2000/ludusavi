# Cover art
Ludusavi can show cover art next to each game in the list.
Turn it on or off with the "show cover art for games" option on the "other" screen
(`covers.show` in the config file).

## Where covers come from
Ludusavi tries these in order, and stops at the first one that works:

1. **Covers Steam has already downloaded**, from its own cache on your computer.
2. **Steam's servers**, using the game's Steam ID.
   The ID comes from Ludusavi's data set, or from Steam's own records
   of the games you have installed, which fills in the gaps.
   No account or key is needed for this.
3. **Your launchers' artwork.**
   Heroic records the cover URL for each of your Epic games, for example.
4. **SteamGridDB and IGDB**, if you've set up a key (see below).
   These cover games that Steam doesn't list, such as older or non-store releases.
5. **The icon of the game's own program**, as a last resort.
   This is only an icon, not a real cover, so it looks plainer than the rest.

Downloads happen in the background, a few games at a time,
and each cover is saved under the `covers` folder in
[Ludusavi's application folder](/docs/help/application-folder.md),
so a game is only ever looked up once.
If nothing is found, Ludusavi remembers that too, instead of asking again every time.

You can turn the downloads off entirely with the "download missing covers" option
(`covers.download`), leaving only what's already on your computer.

## Cover databases
Two optional services can find covers for games that the steps above miss:

* **SteamGridDB** — create a free account at
  [steamgriddb.com](https://www.steamgriddb.com) and generate an API key
  in your account preferences, then paste it into the "SteamGridDB key" field.
* **IGDB** — register an application at
  [dev.twitch.tv](https://dev.twitch.tv/console/apps) to get a client ID and secret,
  then paste both into the matching fields.

If you'd rather always use these (they often have nicer portrait covers),
set `covers.alwaysCheckDatabases` to true in the config file.

## Using your own image
To use a specific image for a game, put it in the `covers` folder
inside Ludusavi's application folder, named after the game,
as a PNG file. For example, a game called `Some Game: Part 2`
would use `Some Game_ Part 2.png`, since characters that aren't allowed
in file names are replaced with an underscore.
Ludusavi prefers whatever is in that folder, so your own image always wins.
