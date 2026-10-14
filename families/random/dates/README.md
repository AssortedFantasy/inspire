# Dates

Seven distinct temporal objects replace the former generator's five alternate
datetime spellings. Incoming weight remains 2; `civil`, `instants`, and
`intervals` have equal weight 1. Their children also have equal weight 1, so each
civil leaf receives 1/9 of date samples and each other leaf receives 1/6.

- `civil/gregorian`: a named weekday and unambiguous Gregorian calendar date.
- `civil/iso-week`: ISO week-year, week and weekday, with its calendar equivalent.
- `civil/ordinal`: year and day-of-year, with its calendar equivalent.
- `instants/utc`: a UTC RFC 3339 timestamp, sampled uniformly by day and second.
- `instants/offset`: a fixed-offset timestamp and the equivalent UTC instant.
- `intervals/calendar-days`: an ordered half-open date interval and exact day count.
- `intervals/elapsed-time`: two UTC endpoints and their exact elapsed duration.

Days span 1600-01-01 through 2199-12-31 in the proleptic Gregorian calendar.
This convention applies before local historical adoption. ISO week-years may
cross calendar-year boundaries. Offset instants reserve one day at each boundary
for conversion; offsets do not assert a region, timezone or daylight-saving rule.
Leap seconds are excluded. Duration days mean exactly 86,400 seconds; calendar
intervals contain whole dates and do not assert an elapsed duration in a timezone.

Calendar intervals select short (1–14), medium (15–180) and long (181–3660 day)
bands equally, then select length uniformly within that band and start uniformly
among fitting endpoints. Elapsed intervals equally select second-length bands
1–3600, 3601–86400, and 86401–2592000. Their start is a uniform second, with a
31-day upper margin. Point-day sampling is uniform over all 219,146 dates rather
than over months or years.

The former path `random/dates` remains a selectable branch. Select
`random/dates/instants/utc` for the former UTC timestamp object. Helper arithmetic
is copied beside each generator so leaves remain independently movable.
Exhaustive bijection tests validate every supported date; boundary tests check
leap centuries, week-years, rollover, interval length and duration decomposition.

Procedural arithmetic is appropriate here; importing date strings would introduce
finite duplication without improving calendar coverage. Rules and notation were
checked against [US Naval Observatory calendars](https://aa.usno.navy.mil/faq/calendars)
and [RFC 3339](https://www.rfc-editor.org/rfc/rfc3339).
