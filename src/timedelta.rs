use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Default)]
pub struct Timedelta {
    years: u16,
    months: u16,
    days: u16,
    hours: u16,
    minutes: u16,
    seconds: u16,
    // decimal fraction of second?
    is_negative: bool,
}

impl Timedelta {
    pub fn new(
        years: Option<u16>,
        months: Option<u16>,
        days: Option<u16>,
        hours: Option<u16>,
        minutes: Option<u16>,
        seconds: Option<u16>,
        is_negative: Option<bool>,
    ) -> Self {
        Timedelta {
            years: years.unwrap_or_default(),
            months: months.unwrap_or_default(),
            days: days.unwrap_or_default(),
            hours: hours.unwrap_or_default(),
            minutes: minutes.unwrap_or_default(),
            seconds: seconds.unwrap_or_default(),
            is_negative: is_negative.unwrap_or_default(),
        }
    }
}

impl Debug for Timedelta {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f, 
            "Timedelta(years: {}, months: {}, days: {}, hours: {}, minutes: {}, seconds: {}, is_negative: {})", 
            self.years, self.months, self.days, self.hours, self.minutes, self.seconds, self.is_negative
        )
    }
}

impl Display for Timedelta {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.is_negative {
            write!(f, "-")?;
        }

        write!(
            f,
            "P{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
            self.years, self.months, self.days, self.hours, self.minutes, self.seconds
        )?;

        Ok(())
    }
}
