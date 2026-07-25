# Backup validation
On the restore screen, there is a "validate" button that will check the integrity
of the latest backup (full + differential, if any) for each game.
You won't normally need to use this, but it exists for troubleshooting purposes.

Specifically, this checks the following:

* Is mapping.yaml malformed?
* Is any file declared in mapping.yaml, but missing from the actual backup?
* Does any file in the backup still have the content that was backed up?
  Ludusavi records a checksum for each file when it makes a backup,
  and validation compares the stored files against those checksums,
  so it can also tell you when a backup has been damaged after the fact
  (for example, by a failing drive or an interrupted file transfer).
  This means that validation has to read the backed up files,
  so it can take a while if you have a lot of data.

If it finds problems, then it will prompt you to create new full backups for the games in question.
At this time, it will not remove the invalid backups, outside of your normal retention settings.
