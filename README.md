# datefilter
Filters a list of filenames to exclude filename for dates which should be kept.

This a useful part of pipelines for removing old dated files or ZFS snapshots.

## Usage
$ datefilter --days=31 --months=12 --years=10 /my/path

The command above will list all file and directory names which contain a YYYY-MM-DD substring, where that date is NOT:
* a day in the last 31 days, and not
* the first of a month in the last 12 months, and not
* New Years Day in the last 10 years.

All filesnames which contain a date not matching the above criteria will be passed through.