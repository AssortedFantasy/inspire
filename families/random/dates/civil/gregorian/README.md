# Gregorian dates

Each of the 219,146 days from 1600-01-01 through 2199-12-31 has equal probability.
Output chooses an ISO date, an ISO date with weekday, or a written date with
equal probability, showing one spelling at a time.
Dates use the proleptic Gregorian calendar regardless of historical local adoption.

The bounded arithmetic generator needs no runtime clock or external dataset.
Tests exhaustively check calendar/ordinal round trips and the 146 leap days, plus
century transitions, known weekdays, and ISO week-year boundaries.
