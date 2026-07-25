# Backup retention
In the "other" screen's backup section,
you can configure how many backups to keep.
A full backup contains all save data for a game,
while a differential backup contains just the data that has changed since the last full backup.

When Ludusavi makes a new backup for a game, it will also remove any excess backups for that specific game.
When a full backup is deleted, its associated differential backups are deleted as well.

For example, if you configure a retention limit of 2 full and 2 differential,
then Ludusavi will create 2 differential backups for each full backup, like so:

* Backup #1: full
  * Backup #2: differential
  * Backup #3: differential
* Backup #4: full
  * Backup #5: differential
  * Backup #6: differential

When backup #7 is created, because the full retention is set to 2,
Ludusavi will delete backups 1 through 3.

If your full retention is only 1 and your differential retention is 1+,
then Ludusavi will keep the full backup and just delete the oldest differential as needed.

On the restore screen, you can use the three-dot menu next to a game to lock any of its backups.
Locked backups do not count toward the retention limits and are retained indefinitely.

## Time-based retention
If you enable the "time-based" option next to the retention limits,
then Ludusavi deletes old backup generations
(a full backup along with its differential backups)
based on their age instead of the full retention count:

* Every generation from the last `keep all` days is kept.
* Before that, only the newest generation per calendar day is kept,
  for `daily` days.
* Before that, only the newest generation per ISO week is kept,
  for `weekly` weeks.
* Anything older is deleted.

The newest generation is never deleted, even if it is older than every window.
Locked backups are still exempt.

The full and differential counts continue to control
how often Ludusavi starts a new full backup,
so a small differential count means more generations
(and therefore more restore points) within each window.

This is useful if you back up frequently, such as after each play session,
since it keeps recent history in detail
while thinning out older backups automatically.
