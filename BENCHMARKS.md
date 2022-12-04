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
|        | `61.49 us` (✅ **1.00x**) | `130.41 us` (❌ *2.12x slower*)   | `329.17 us` (❌ *5.35x slower*)   | `119.72 us` (❌ *1.95x slower*)    |

### Part I 

|        | ` One`                  | ` Two`                         | ` Three`                         | ` Four`                         |
|:-------|:------------------------|:-------------------------------|:---------------------------------|:------------------------------- |
|        | `1.18 us` (✅ **1.00x**) | `3.23 us` (❌ *2.73x slower*)   | `77.00 us` (❌ *65.06x slower*)   | `2.81 us` (❌ *2.38x slower*)    |

### Part II 

|        | ` One`                  | ` Two`                         | ` Three`                          | ` Four`                         |
|:-------|:------------------------|:-------------------------------|:----------------------------------|:------------------------------- |
|        | `5.97 us` (✅ **1.00x**) | `7.91 us` (❌ *1.32x slower*)   | `569.36 us` (❌ *95.35x slower*)   | `1.18 us` (🚀 **5.05x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

