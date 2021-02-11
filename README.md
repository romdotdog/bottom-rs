maps are for losers

Original bench:

```
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
decode/30               time:   [469.81 ns 485.41 ns 501.64 ns]
                        thrpt:  [57.034 MiB/s 58.941 MiB/s 60.897 MiB/s]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
decode/109              time:   [929.52 ns 970.04 ns 1.0180 us]
                        thrpt:  [102.12 MiB/s 107.16 MiB/s 111.83 MiB/s]
Found 10 outliers among 100 measurements (10.00%)
  10 (10.00%) high mild
decode/184              time:   [1.0789 us 1.1114 us 1.1462 us]
                        thrpt:  [153.09 MiB/s 157.89 MiB/s 162.64 MiB/s]
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) high mild
  8 (8.00%) high severe
decode/355              time:   [1.1052 us 1.1564 us 1.2153 us]
                        thrpt:  [278.58 MiB/s 292.77 MiB/s 306.33 MiB/s]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
decode/32946            time:   [134.36 us 138.91 us 143.98 us]
                        thrpt:  [218.22 MiB/s 226.18 MiB/s 233.84 MiB/s]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
```

current:

````
decode/30               time:   [125.07 ns 131.63 ns 139.13 ns]
                        thrpt:  [205.63 MiB/s 217.35 MiB/s 228.75 MiB/s]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
decode/109              time:   [187.50 ns 199.28 ns 213.62 ns]
                        thrpt:  [486.61 MiB/s 521.63 MiB/s 554.41 MiB/s]
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
decode/184              time:   [335.90 ns 347.45 ns 359.53 ns]
                        thrpt:  [488.07 MiB/s 505.04 MiB/s 522.40 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high mild
decode/355              time:   [354.66 ns 378.65 ns 405.52 ns]
                        thrpt:  [834.86 MiB/s 894.10 MiB/s 954.58 MiB/s]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
decode/32946            time:   [42.029 us 45.353 us 48.953 us]
                        thrpt:  [641.83 MiB/s 692.79 MiB/s 747.57 MiB/s]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe```
````
