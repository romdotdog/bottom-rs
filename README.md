# warp-bottom-rs

bottom at warp speed! Faster than upstream and bottom-go. The huge decode speed increase doesn't fit in the spec, so this is technically an unofficial repository.

## Just tell me the improvement

```diff
  encode/1        102.42 ns → 106.82 ns
+ encode/4        409.85 ns → 142.99 ns
+ encode/8        907.71 ns → 164.99 ns
+ encode/16       768.60 ns → 216.65 ns
+ encode/1533     22.045 us → 12.392 us (-9653 ns)

+ decode/1        419.02 ns → 108.90 ns
+ decode/4        792.87 ns → 167.41 ns
+ decode/8        960.98 ns → 212.52 ns
+ decode/16       948.25 ns → 348.30 ns
+ decode/1533     116.75 us → 37.182 us (-79568 ns)
```

## Upstream bench

```
encode/1                time:   [98.484 ns 102.42 ns 107.29 ns]
                        thrpt:  [8.8886 MiB/s 9.3117 MiB/s 9.6836 MiB/s]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe
encode/4                time:   [383.83 ns 409.85 ns 441.76 ns]
                        thrpt:  [8.6351 MiB/s 9.3076 MiB/s 9.9385 MiB/s]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
encode/8                time:   [838.15 ns 907.71 ns 979.21 ns]
                        thrpt:  [7.7913 MiB/s 8.4051 MiB/s 9.1026 MiB/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
encode/16               time:   [737.86 ns 768.60 ns 801.59 ns]
                        thrpt:  [19.036 MiB/s 19.853 MiB/s 20.680 MiB/s]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
encode/1533             time:   [20.097 us 22.045 us 24.246 us]
                        thrpt:  [60.297 MiB/s 66.319 MiB/s 72.745 MiB/s]

decode/30               time:   [520.87 ns 538.17 ns 555.75 ns]
                        thrpt:  [51.480 MiB/s 53.162 MiB/s 54.928 MiB/s]
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe
decode/109              time:   [955.98 ns 1.0086 us 1.0714 us]
                        thrpt:  [97.020 MiB/s 103.06 MiB/s 108.74 MiB/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
decode/184              time:   [1.1835 us 1.2498 us 1.3203 us]
                        thrpt:  [132.91 MiB/s 140.40 MiB/s 148.26 MiB/s]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
decode/355              time:   [1.1050 us 1.1363 us 1.1725 us]
                        thrpt:  [288.75 MiB/s 297.95 MiB/s 306.37 MiB/s]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
decode/32946            time:   [144.88 us 148.53 us 152.30 us]
                        thrpt:  [206.31 MiB/s 211.54 MiB/s 216.86 MiB/s]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
```

## Current bench

```bench
encode/1                time:   [102.93 ns 106.82 ns 111.01 ns]
                        thrpt:  [8.5907 MiB/s 8.9283 MiB/s 9.2657 MiB/s]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
encode/4                time:   [133.86 ns 142.99 ns 153.96 ns]
                        thrpt:  [24.778 MiB/s 26.678 MiB/s 28.499 MiB/s]
Found 10 outliers among 100 measurements (10.00%)
  8 (8.00%) high mild
  2 (2.00%) high severe
encode/8                time:   [155.31 ns 164.99 ns 176.74 ns]
                        thrpt:  [43.167 MiB/s 46.243 MiB/s 49.125 MiB/s]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
encode/12               time:   [191.53 ns 200.49 ns 209.83 ns]
                        thrpt:  [54.540 MiB/s 57.081 MiB/s 59.752 MiB/s]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
encode/16               time:   [206.32 ns 216.65 ns 228.51 ns]
                        thrpt:  [66.775 MiB/s 70.432 MiB/s 73.958 MiB/s]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
encode/1533             time:   [11.791 us 12.392 us 13.098 us]
                        thrpt:  [111.62 MiB/s 117.98 MiB/s 123.99 MiB/s]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

decode/1                time:   [143.43 ns 150.56 ns 157.90 ns]
                        thrpt:  [181.19 MiB/s 190.02 MiB/s 199.48 MiB/s]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
decode/4                time:   [350.89 ns 390.45 ns 432.75 ns]
                        thrpt:  [240.21 MiB/s 266.23 MiB/s 296.25 MiB/s]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
decode/8                time:   [469.08 ns 546.30 ns 627.37 ns]
                        thrpt:  [278.18 MiB/s 319.46 MiB/s 372.05 MiB/s]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe
decode/12               time:   [523.65 ns 555.48 ns 593.79 ns]
                        thrpt:  [525.19 MiB/s 561.41 MiB/s 595.53 MiB/s]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
decode/16               time:   [474.00 ns 505.46 ns 546.14 ns]
                        thrpt:  [616.41 MiB/s 666.02 MiB/s 710.23 MiB/s]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
decode/1533             time:   [104.09 us 113.19 us 123.93 us]
                        thrpt:  [253.53 MiB/s 277.58 MiB/s 301.85 MiB/s]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
```
