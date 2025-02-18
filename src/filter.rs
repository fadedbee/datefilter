use chrono::{Datelike, Months, NaiveDate, TimeDelta};
use regex::Regex;

pub struct DateFilter {
    date_regex: Regex,
    today: NaiveDate,
    days: i64,
    months: u32,
    years: u32,
}

#[derive(PartialEq, Debug)]
pub enum Outcome {
    FailRegex,
    FailParse,
    FailFuture,
    FailDays,
    FailMonths,
    FailYears,
    Pass,
}

const MONTHS_IN_A_YEAR: u32 = 12;

impl DateFilter {
    pub fn new(today: NaiveDate, days: u32, months: u32, years: u32) -> DateFilter {
        let date_regex: Regex = Regex::new("[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap();
        DateFilter {
            date_regex,
            today,
            days: days.into(),
            months,
            years,
        }
    }

    pub fn check(&self, name: &str) -> Outcome {
        let opt_date_match = self.date_regex.find(name);

        /* If there is no YYYY-MM-DD date in the string, the we must filter it out. */
        let Some(date_match) = opt_date_match else {
            return Outcome::FailRegex;
        };

        /* If it doesn't parse as a date, filter it out. */
        let date = match NaiveDate::parse_from_str(date_match.as_str(), "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => return Outcome::FailParse,
        };

        /* If the date is in the future, filter it out. */
        if date > self.today {
            return Outcome::FailFuture;
        }

        /* If the date is within args.days, then filter it out. */
        if date + TimeDelta::days(self.days) > self.today {
            return Outcome::FailDays;
        }

        /* If the date is the first of a month and within args.months, then filter it out. */
        if date.day() == 1
            && date.checked_add_months(Months::new(self.months)).unwrap() > self.today
        {
            return Outcome::FailMonths;
        }

        /* If the date is New Years Day and within args.years, then filter it out. */
        if date.day() == 1
            && date.month() == 1
            && date
                .checked_add_months(Months::new(self.years * MONTHS_IN_A_YEAR))
                .unwrap()
                > self.today
        {
            return Outcome::FailYears;
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
    use super::DateFilter;
    use crate::filter::Outcome;
    use chrono::NaiveDate;

    #[test]
    fn test_dates() {
        let today = NaiveDate::parse_from_str("2001-02-03", "%Y-%m-%d").unwrap();
        let filter = DateFilter::new(today, 31, 12, 3);

        assert_eq!(filter.check("foo_12001-02-9.txt"), Outcome::FailRegex);
        assert_eq!(filter.check("foo_2001-02-99.txt"), Outcome::FailParse);
        assert_eq!(filter.check("foo_2001-02-04.txt"), Outcome::FailFuture);

        assert_eq!(filter.check("foo_2001-02-03.txt"), Outcome::FailDays);
        assert_eq!(filter.check("foo_2001-02-02.txt"), Outcome::FailDays);
        assert_eq!(filter.check("foo_2001-01-04.txt"), Outcome::FailDays);
        assert_eq!(filter.check("foo_2001-01-03.txt"), Outcome::Pass);

        assert_eq!(filter.check("foo_2001-01-01.txt"), Outcome::FailMonths);
        assert_eq!(filter.check("foo_2000-12-01.txt"), Outcome::FailMonths);
        assert_eq!(filter.check("foo_2000-11-01.txt"), Outcome::FailMonths);
        assert_eq!(filter.check("foo_2000-10-01.txt"), Outcome::FailMonths);
        assert_eq!(filter.check("foo_2000-09-01.txt"), Outcome::FailMonths);
        assert_eq!(filter.check("foo_2000-08-01.txt"), Outcome::FailMonths);
        assert_eq!(filter.check("foo_2000-07-01.txt"), Outcome::FailMonths);
        assert_eq!(filter.check("foo_2000-06-01.txt"), Outcome::FailMonths);
        assert_eq!(filter.check("foo_2000-05-01.txt"), Outcome::FailMonths);
        assert_eq!(filter.check("foo_2000-04-01.txt"), Outcome::FailMonths);
        assert_eq!(filter.check("foo_2000-03-01.txt"), Outcome::FailMonths);
        assert_eq!(filter.check("foo_2000-02-01.txt"), Outcome::Pass);

        assert_eq!(filter.check("foo_2000-01-01.txt"), Outcome::FailYears);
        assert_eq!(filter.check("foo_1999-01-01.txt"), Outcome::FailYears);
        assert_eq!(filter.check("foo_1998-01-01.txt"), Outcome::Pass);
    }
}
