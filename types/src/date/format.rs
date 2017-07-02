use chrono::{self, Utc, NaiveDateTime, NaiveDate, NaiveTime};
use chrono::format::{Item, DelayedFormat};
use std::ops::Deref;
use std::marker::PhantomData;
use std::borrow::Borrow;
use std::error::Error;
use std::fmt::{Display, Result as FmtResult, Formatter};
use std::vec::IntoIter;
use super::ChronoDateTime;

/*
TODO: 

- Use `DateValue` for passing around dates that don't have mapping or formats
- Don't support constructing `DateValue`s directly? Seems reasonable, or `IntoDateValueWithFormat` wouldn't be very useful
- Expect owned values for `DateValue`
- Still support borrowed values for formatting

- `Date` is the field type you use for mapping fields
- `DateTime<Utc>` is the field type you use if you don't care about mapping or formats
- `DateValue` is the type you use for raw date values
- `IntoDateValue` has a `Format` parameter that can be used to constrain inputs
*/

/** 
A date value produced and consumed by date formats.

You probably won't need to use this type directly.
*/
#[derive(Debug, Clone, PartialEq)]
pub struct DateValue(ChronoDateTime);

impl DateValue {
    pub fn now() -> Self {
        DateValue(Utc::now())
    }

    pub fn build(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32, milli: u32) -> Self {
        let ndate = NaiveDate::from_ymd(year, month, day);
        let ntime = NaiveTime::from_hms_milli(hour, minute, second, milli);

        let date = ChronoDateTime::from_utc(NaiveDateTime::new(ndate, ntime), Utc);

        DateValue(date)
    }
}

impl<F> From<FormattableDateValue<F>> for DateValue {
    fn from(date: FormattableDateValue<F>) -> Self {
        date.0
    }
}

impl PartialEq<ChronoDateTime> for DateValue {
    fn eq(&self, other: &ChronoDateTime) -> bool {
        PartialEq::eq(&self.0, other)
    }

    fn ne(&self, other: &ChronoDateTime) -> bool {
        PartialEq::ne(&self.0, other)
    }
}

impl PartialEq<DateValue> for ChronoDateTime {
    fn eq(&self, other: &DateValue) -> bool {
        PartialEq::eq(self, &other.0)
    }

    fn ne(&self, other: &DateValue) -> bool {
        PartialEq::ne(self, &other.0)
    }
}

impl Borrow<ChronoDateTime> for DateValue {
    fn borrow(&self) -> &ChronoDateTime {
        &self.0
    }
}

// TODO: Remove this
impl Deref for DateValue {
    type Target = ChronoDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/** 
A date value paired with a format.

This type provides a convenient way to parse and format a date value with a fixed format.
*/
#[derive(Debug, Clone, PartialEq)]
pub struct FormattableDateValue<F>(DateValue, PhantomData<F>);

impl<F> FormattableDateValue<F> where F: DateFormat {
    pub fn format<'a>(&'a self) -> FormattedDate<'a> {
        F::format(&self.0)
    }

    pub fn parse(date: &str) -> Result<Self, ParseError> {
        let date = F::parse(date)?;

        Ok(FormattableDateValue::from(date))
    }

    pub fn reformat<FInto>(self) -> FormattableDateValue<FInto> {
        FormattableDateValue(self.0, PhantomData)
    }
}

impl<F> Borrow<ChronoDateTime> for FormattableDateValue<F> {
    fn borrow(&self) -> &ChronoDateTime {
        &self.0
    }
}

impl<F> PartialEq<ChronoDateTime> for FormattableDateValue<F> {
    fn eq(&self, other: &ChronoDateTime) -> bool {
        PartialEq::eq(&self.0, other)
    }

    fn ne(&self, other: &ChronoDateTime) -> bool {
        PartialEq::ne(&self.0, other)
    }
}

impl<F> PartialEq<FormattableDateValue<F>> for ChronoDateTime {
    fn eq(&self, other: &FormattableDateValue<F>) -> bool {
        PartialEq::eq(self, &other.0)
    }

    fn ne(&self, other: &FormattableDateValue<F>) -> bool {
        PartialEq::ne(self, &other.0)
    }
}

impl<F> From<DateValue> for FormattableDateValue<F> {
    fn from(date: DateValue) -> Self {
        FormattableDateValue(date.into(), PhantomData)
    }
}

impl From<ChronoDateTime> for DateValue {
    fn from(date: ChronoDateTime) -> Self {
        DateValue(date)
    }
}

/**
A format used for parsing and formatting dates.

The format is specified as two functions: `parse` and `format`.
A general `DateValue` is used as an intermediate value passed as input and produced as output for formatting.

# Examples

The easiest way to implement `DateFormat` is to derive `ElasticDateFormat`
on a unit struct:

```
# #[macro_use]
# extern crate elastic_types;
# #[macro_use]
# extern crate elastic_types_derive;
# extern crate chrono;
# use elastic_types::prelude::*;
# fn main() {
#[derive(Default, ElasticDateFormat)]
#[elastic(date_format="yyyy-MM-dd'T'HH:mm:ss")]
struct MyFormat;
# }
```

The `#[elastic(date_format)]` attribute is required,
and must contain a valid [format string](http://www.joda.org/joda-time/apidocs/org/joda/time/format/DateTimeFormat.html).

> NOTE: Only a small subset of the Joda time format is supported.

You can customise the indexed format name by adding an `#[elastic(date_format_name)]` attribute:

```
# #[macro_use]
# extern crate elastic_types;
# #[macro_use]
# extern crate elastic_types_derive;
# extern crate chrono;
# use elastic_types::prelude::*;
# fn main() {
#[derive(Default, ElasticDateFormat)]
#[elastic(date_format="yyyyMMdd'T'HHmmssZ", date_format_name="basic_date_time_no_millis")]
struct MyFormat;
# }
```
*/
pub trait DateFormat
    where Self: Default
{
    /** Parses a date string to a `chrono::DateTime<Utc>` result. */
    fn parse(date: &str) -> Result<DateValue, ParseError>;

    /** Formats a given `chrono::DateTime<Utc>` as a string. */
    fn format<'a>(date: &'a DateValue) -> FormattedDate<'a>;

    /**
    The name of the format.
    
    This is the string used when defining the format in the field mapping.
    */
    fn name() -> &'static str;
}

/**
A formatted date.

This type can avoid allocating strings for date formats.
*/
pub struct FormattedDate<'a> {
    inner: FormattedDateInner<'a>,
}

enum FormattedDateInner<'a> {
    Delayed(DelayedFormat<IntoIter<Item<'a>>>),
    Buffered(String),
    Number(i64),
}

impl<'a> Display for FormattedDateInner<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        fn fmt_inner<T>(inner: &T, f: &mut Formatter) -> FmtResult
            where T: Display
        {
            inner.fmt(f)
        }

        match *self {
            FormattedDateInner::Delayed(ref inner) => fmt_inner(inner, f),
            FormattedDateInner::Buffered(ref inner) => fmt_inner(inner, f),
            FormattedDateInner::Number(ref inner) => fmt_inner(inner, f),
        }
    }
}

impl<'a> Display for FormattedDate<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.inner.fmt(f)
    }
}

impl<'a> From<DelayedFormat<IntoIter<Item<'a>>>> for FormattedDate<'a> {
    fn from(formatted: DelayedFormat<IntoIter<Item<'a>>>) -> Self {
        FormattedDate { inner: FormattedDateInner::Delayed(formatted) }
    }
}

impl<'a> From<String> for FormattedDate<'a> {
    fn from(formatted: String) -> Self {
        FormattedDate { inner: FormattedDateInner::Buffered(formatted) }
    }
}

impl<'a> From<i64> for FormattedDate<'a> {
    fn from(formatted: i64) -> Self {
        FormattedDate { inner: FormattedDateInner::Number(formatted) }
    }
}

/** Represents an error encountered during parsing. */
#[derive(Debug)]
pub struct ParseError {
    kind: ParseErrorKind,
}

#[derive(Debug)]
enum ParseErrorKind {
    Chrono(chrono::ParseError),
    Other(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self.kind {
            ParseErrorKind::Chrono(ref err) => write!(f, "Chrono error: {}", err),
            ParseErrorKind::Other(ref err) => write!(f, "Error: {}", err),
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match self.kind {
            ParseErrorKind::Chrono(ref err) => err.description(),
            ParseErrorKind::Other(ref err) => &err[..],
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self.kind {
            ParseErrorKind::Chrono(ref err) => Some(err),
            ParseErrorKind::Other(_) => None,
        }
    }
}

impl From<chrono::ParseError> for ParseError {
    fn from(err: chrono::ParseError) -> ParseError {
        ParseError { kind: ParseErrorKind::Chrono(err) }
    }
}

impl From<String> for ParseError {
    fn from(err: String) -> ParseError {
        ParseError { kind: ParseErrorKind::Other(err) }
    }
}
