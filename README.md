# maps are for losers

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
encode/1                time:   [76.773 ns 78.573 ns 80.665 ns]
                        thrpt:  [11.823 MiB/s 12.137 MiB/s 12.422 MiB/s]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe
encode/4                time:   [268.96 ns 282.54 ns 303.39 ns]
                        thrpt:  [12.574 MiB/s 13.501 MiB/s 14.183 MiB/s]
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe
encode/8                time:   [381.83 ns 389.42 ns 397.52 ns]
                        thrpt:  [19.193 MiB/s 19.592 MiB/s 19.981 MiB/s]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
encode/12               time:   [491.09 ns 500.00 ns 510.59 ns]
                        thrpt:  [22.413 MiB/s 22.888 MiB/s 23.304 MiB/s]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
encode/16               time:   [508.20 ns 517.08 ns 527.56 ns]
                        thrpt:  [28.923 MiB/s 29.509 MiB/s 30.025 MiB/s]
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe
encode/1533             time:   [12.013 us 12.330 us 12.689 us]
                        thrpt:  [115.22 MiB/s 118.58 MiB/s 121.70 MiB/s]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

decode/1                time:   [97.074 ns 98.806 ns 100.63 ns]
                        thrpt:  [284.30 MiB/s 289.56 MiB/s 294.73 MiB/s]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
decode/4                time:   [180.52 ns 189.41 ns 198.15 ns]
                        thrpt:  [524.60 MiB/s 548.81 MiB/s 575.82 MiB/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
decode/8                time:   [278.45 ns 288.68 ns 300.54 ns]
                        thrpt:  [583.87 MiB/s 607.86 MiB/s 630.18 MiB/s]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high severe
decode/16               time:   [287.54 ns 295.97 ns 305.45 ns]
                        thrpt:  [1.0824 GiB/s 1.1171 GiB/s 1.1498 GiB/s]
Found 8 outliers among 100 measurements (8.00%)
  7 (7.00%) high mild
  1 (1.00%) high severe
decode/1533             time:   [34.829 us 35.875 us 36.949 us]
                        thrpt:  [850.36 MiB/s 875.81 MiB/s 902.11 MiB/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
```
