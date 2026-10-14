# ISO week dates

Uniformly samples every Gregorian date from 1600-01-01 through 2199-12-31,
then emits its ISO week-year, week number, and weekday alongside the Gregorian
date and weekday name. Monday is day 1. Week 1 contains January 4; week-years
can differ from calendar years at either end of a year. The endpoints therefore
include ISO week-years 1599 and 2200.

This bounded arithmetic generator needs no dataset or runtime calendar service.
Tests check every date against an independent January 4 anchor calculation,
Monday transitions, the 53-week rule, century leap rules, and boundary fixtures.
Development review also compared all 219,146 results with Python 3.12's
`datetime.date.isocalendar()`.
