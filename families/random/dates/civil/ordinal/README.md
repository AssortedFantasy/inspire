# Gregorian ordinal dates

Each output gives one date in `YYYY-DDD` notation, its calendar equivalent, and
the year's length. The day of year starts at `001`; the final day is `365` in a
common year or `366` in a leap year.

Sampling is uniform over all 219,146 days from 1600-01-01 through 2199-12-31 in
the proleptic Gregorian calendar. Consequently leap years receive one extra
day's probability. No runtime clock or external dataset is needed.
