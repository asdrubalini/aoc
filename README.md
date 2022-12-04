# Advent of Code 2022
Run tests: `cargo test`

Generate benchmarks: `cargo criterion --message-format=json | criterion-table >> README.md`

<!--- advent_readme_stars table --->

# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [One ](#one-)
    - [Two ](#two-)

## Benchmark Results

### One 

|        | ` Parse`                 | ` Part I`                         | ` Part II`                      |
|:-------|:-------------------------|:----------------------------------|:------------------------------- |
|        | `38.58 us` (✅ **1.00x**) | `661.00 ns` (🚀 **58.36x faster**) | `4.36 us` (🚀 **8.84x faster**)  |

### Two 

|        | ` Parse`                 | ` Part I`                       | ` Part II`                       |
|:-------|:-------------------------|:--------------------------------|:-------------------------------- |
|        | `73.06 us` (✅ **1.00x**) | `2.13 us` (🚀 **34.24x faster**) | `3.68 us` (🚀 **19.84x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

