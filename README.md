# maps are for losers

## Just tell me the improvement

```diff
- encode/1        71.543 ns → 76.775 ns
+ encode/4        281.22 ns → 97.319 ns
+ encode/8        405.03 ns → 122.38 ns
+ encode/16       539.60 ns → 163.72 ns
+ encode/1533     12.038 us → 9.4148 us (-2623.2 ns)

+ decode/1        419.02 ns → 108.90 ns
+ decode/4        792.87 ns → 167.41 ns
+ decode/8        960.98 ns → 212.52 ns
+ decode/16       948.25 ns → 348.30 ns
+ decode/1533     116.75 us → 37.182 us (-79568 ns)
```

## Upstream bench

```
encode/1                time:   [70.594 ns 71.543 ns 72.604 ns]
                        thrpt:  [13.135 MiB/s 13.330 MiB/s 13.509 MiB/s]
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe
encode/4                time:   [275.93 ns 281.22 ns 286.61 ns]
                        thrpt:  [13.310 MiB/s 13.565 MiB/s 13.825 MiB/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
encode/8                time:   [396.79 ns 405.03 ns 413.74 ns]
                        thrpt:  [18.440 MiB/s 18.837 MiB/s 19.228 MiB/s]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild
encode/16               time:   [521.69 ns 539.60 ns 569.62 ns]
                        thrpt:  [26.788 MiB/s 28.278 MiB/s 29.249 MiB/s]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe
encode/1533             time:   [11.797 us 12.038 us 12.307 us]
                        thrpt:  [118.79 MiB/s 121.45 MiB/s 123.93 MiB/s]
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe

decode/1                time:   [403.84 ns 419.02 ns 438.45 ns]
                        thrpt:  [65.254 MiB/s 68.279 MiB/s 70.845 MiB/s]
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe
decode/4                time:   [760.76 ns 792.87 ns 831.97 ns]
                        thrpt:  [124.95 MiB/s 131.11 MiB/s 136.64 MiB/s]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe
decode/8                time:   [936.57 ns 960.98 ns 990.45 ns]
                        thrpt:  [177.17 MiB/s 182.60 MiB/s 187.36 MiB/s]
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) high mild
  12 (12.00%) high severe
decode/16               time:   [927.79 ns 948.25 ns 969.42 ns]
                        thrpt:  [349.24 MiB/s 357.03 MiB/s 364.90 MiB/s]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild
decode/1533             time:   [115.57 us 116.75 us 118.22 us]
                        thrpt:  [265.77 MiB/s 269.11 MiB/s 271.87 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  5 (5.00%) high severe
```

## Current bench

```bench
encode/1                time:   [74.662 ns 76.775 ns 79.281 ns]
                        thrpt:  [12.029 MiB/s 12.422 MiB/s 12.773 MiB/s]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
encode/4                time:   [95.267 ns 97.319 ns 99.706 ns]
                        thrpt:  [38.260 MiB/s 39.198 MiB/s 40.042 MiB/s]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) high mild
  7 (7.00%) high severe
encode/8                time:   [119.36 ns 122.38 ns 126.23 ns]
                        thrpt:  [60.439 MiB/s 62.342 MiB/s 63.919 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe
encode/12               time:   [141.64 ns 147.15 ns 153.31 ns]
                        thrpt:  [74.649 MiB/s 77.770 MiB/s 80.795 MiB/s]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild
encode/16               time:   [159.95 ns 163.72 ns 167.75 ns]
                        thrpt:  [90.961 MiB/s 93.198 MiB/s 95.399 MiB/s]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
encode/1533             time:   [9.1695 us 9.4148 us 9.7094 us]
                        thrpt:  [150.57 MiB/s 155.28 MiB/s 159.44 MiB/s]
Found 10 outliers among 100 measurements (10.00%)

decode/1                time:   [104.65 ns 108.90 ns 113.73 ns]
                        thrpt:  [251.56 MiB/s 262.73 MiB/s 273.39 MiB/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
decode/4                time:   [159.38 ns 167.41 ns 176.26 ns]
                        thrpt:  [589.76 MiB/s 620.94 MiB/s 652.20 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe
decode/8                time:   [202.57 ns 212.52 ns 223.19 ns]
                        thrpt:  [781.94 MiB/s 821.19 MiB/s 861.54 MiB/s]
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild
decode/12               time:   [336.27 ns 354.73 ns 374.71 ns]
                        thrpt:  [832.24 MiB/s 879.13 MiB/s 927.39 MiB/s]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
decode/16               time:   [324.04 ns 348.30 ns 374.83 ns]
                        thrpt:  [898.14 MiB/s 966.54 MiB/s 1.0146 GiB/s]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
decode/1533             time:   [35.624 us 37.182 us 38.778 us]
                        thrpt:  [810.24 MiB/s 845.04 MiB/s 881.98 MiB/s]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
```
