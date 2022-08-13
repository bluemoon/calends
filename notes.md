## 12.2 Selection notes

- N Selection rule sets
- Builds up explicit syntax of time scale components from 4.3 and ISO8601-1:2019, 4.3

L{1,3,5}KN = Mondays, Wednesdays and Fridays, a disjoint calendar

### Selection Rules

**12.2.1 Selection of calendar month of year:**

```
3M = March
12M = December
```

**12.2.2 Weeks:**

```
10W 10th calendar week
-2W the second last week of the calendar year
```

NOTE: week 53 can only occur when Thursday is Jan 1 or if it is a leap yaer and Wednesday is Jan 1,
in accordance with ISO 8601-1:2019, 4.2.2

**12.3 Selection of calendar day of month:**

18D represents the eighteenth day of the calendar month
-10D represents the tenth to last day of the calendar month

**12.4 Selection of Day of week:**

- With a monthly repeat rule [eligible-time-intervals] is [monthE] '1K' represents all mondays within a 
calendar month
- In a yearly context '5K' represents all Fridays

**12.5 Selection of ordinal days in calendar year**

- '-1O' represents the last calendar day of the year
- '-307O' represents the 307th to the last day of the year (Feb 28th in the common year)


**12.9 Selection of position**

The position rule if applied should be applied last, and only when there is at least one selection rule
preceding it.

```
positionSR = [position]["I"]
```

where position is a positive or negative integer

- With a monthly repeat rule, "the last work day of the calendar month" can be represented as there
selection rule 'L{1..31}D{1..5}K-1IN'
  - It looks like L and D are groups?
  - This is still a bit confusing for me


**12.10 Selection with time interval**

The selection rule allows extending a selection set with a duration, which applicaes to each of the 
elements in the selection set.

```
timeIntervalSR = [selection]["/"][duration]
```

where `[selection]` is a rule set specified in 12.2

e.g. 

`LL3K4IN/P5DN` selects a time interval set with the start on "the fourth Wednesday" for a duration 
of 5 days. where `L3K4IN/P5D` is a selection with a time interval
