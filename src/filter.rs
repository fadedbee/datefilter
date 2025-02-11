use chrono::{Date, Datelike, NaiveDate, TimeDelta};
use regex::Regex;

pub struct DateFilter {
    date_regex: Regex,
    today: NaiveDate,
    days: i64,
    months: i64,
    years: i64,
}

#[derive(PartialEq, Debug)]
pub enum Outcome {
    FailNoDate,
    FailUnparseable,
    FailFutureDate,
    FailWithinDays,
    FailWithonMonths,
    FailWithinYears,
    Pass,
}

impl DateFilter {
    pub fn new(today: NaiveDate, days: u32, months: u32, years: u32) -> DateFilter {
        let date_regex: Regex = Regex::new("[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap();
        DateFilter {
            date_regex,
            today,
            days: days.into(),
            months: months.into(),
            years: years.into(),
        }
    }

    pub fn check(&self, name: &str) -> Outcome {
        let opt_date_match = self.date_regex.find(name);

        /* If there is no YYYY-MM-DD date in the string, the we must filter it out. */
        let Some(date_match) = opt_date_match else {
            return Outcome::FailNoDate
        };

        /* If it doesn't parse as a date, filter it out. */
        let date = match NaiveDate::parse_from_str(date_match.as_str(), "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => return Outcome::FailUnparseable,
        };

        /* If the date is in the future, filter it out. */
        if date > self.today {
            return Outcome::FailFutureDate
        }

        /* If the date is within args.days, then filter it out. */
        let delta = self.today - date;
        if delta < TimeDelta::days(self.days) {
            return Outcome::FailWithinDays
        }

        /* If the date is the first of a month and within args.months, then filter it out. */
        if date.day() == 1 && delta < TimeDelta::days(self.months * 31) {
            return Outcome::FailWithonMonths
        }

        /* If the date is New Years Day and within args.years, then filter it out. */
        if date.day() == 1 && date.month() == 1 && delta < TimeDelta::days(self.years * 366) {
            return Outcome::FailWithinYears
        }

        /*
        * Otherwise the filename has no reason to exist and should be returned (for
        * deletion by the remainder of the pipeline). 
        */
        Outcome::Pass
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;

    use crate::filter::Outcome;

    use super::DateFilter;

    #[test]
    fn test_dates() {
        let today = NaiveDate::parse_from_str("2001-02-03", "%Y-%m-%d").unwrap();
        let filter = DateFilter::new(today, 31, 12, 3);

        assert_eq!(filter.check("asdf01-2345"), Outcome::FailNoDate);
        assert_eq!(filter.check("foo2000-01-01"), Outcome::FailWithinYears);
    }

}