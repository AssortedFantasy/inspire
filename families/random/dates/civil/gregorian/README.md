# Gregorian dates

Each of the 219,146 days from 1600-01-01 through 2199-12-31 has equal probability.
Output includes the weekday, a written date, and its ISO calendar-date spelling.
Dates use the proleptic Gregorian calendar regardless of historical local adoption.

The bounded arithmetic generator needs no runtime clock or external dataset.
Tests exhaustively check calendar/ordinal round trips and the 146 leap days, plus
century transitions, known weekdays, and ISO week-year boundaries.
