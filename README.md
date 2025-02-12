# datefilter
Filters a list of filenames to exclude filenames for dates which should be kept, and non-dates.

This a useful part of pipelines for removing old dated files or ZFS snapshots.

## Usage
$ ls *.backup | datefilter --days=31 --months=12 --years=10 | xargs rm 

The pipeline above will delete all *.backup files which contain a YYYY-MM-DD substring in their filename, where that date is NOT:
* a day in the last 31 days, and not
* the first day of a month in the last 12 months, and not
* New Years Day in the last 10 years.

All filesnames which contain a date, not matching the above criteria, will be written to stdout (and deleted by xargs/rm in the example above).
