use std::ffi::OsString;


pub fn date_filter(name: &OsString) -> bool {
    /* If there is no YYYY-MM-DD date in the string, the we must filter it out. */

    /* If the date is within args.days, then filter it out. */

    /* If the date is the first of a month and within args.month, then filter it out. */

    /* If the date is New Years Day and withing args.years, then filter it out. */

    /*
     * Otherwise the filename has no reason to exist and should be returned (for
     * deletion by the remainder of the pipeline). 
     */
    true
}