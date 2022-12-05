# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Input Parse ](#input-parse-)
    - [Part I ](#part-i-)
    - [Part II ](#part-ii-)

## Benchmark Results

### Input Parse 

|        | ` One`                   | ` Two`                           | ` Three`                         | ` Four`                           |
|:-------|:-------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
|        | `63.98 us` (✅ **1.00x**) | `124.27 us` (❌ *1.94x slower*)   | `328.82 us` (❌ *5.14x slower*)   | `120.39 us` (❌ *1.88x slower*)    |

### Part I 

|        | ` One`                  | ` Two`                         | ` Three`                         | ` Four`                         |
|:-------|:------------------------|:-------------------------------|:---------------------------------|:------------------------------- |
|        | `1.02 us` (✅ **1.00x**) | `7.09 us` (❌ *6.97x slower*)   | `73.33 us` (❌ *72.13x slower*)   | `3.80 us` (❌ *3.74x slower*)    |

### Part II 

|        | ` One`                  | ` Two`                         | ` Three`                          | ` Four`                         |
|:-------|:------------------------|:-------------------------------|:----------------------------------|:------------------------------- |
|        | `6.02 us` (✅ **1.00x**) | `8.89 us` (❌ *1.48x slower*)   | `593.91 us` (❌ *98.66x slower*)   | `1.18 us` (🚀 **5.09x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

