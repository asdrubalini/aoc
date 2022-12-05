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
|        | `67.26 us` (✅ **1.00x**) | `149.71 us` (❌ *2.23x slower*)   | `379.72 us` (❌ *5.65x slower*)   | `133.98 us` (❌ *1.99x slower*)    |

### Part I 

|        | ` One`                  | ` Two`                         | ` Three`                         | ` Four`                         |
|:-------|:------------------------|:-------------------------------|:---------------------------------|:------------------------------- |
|        | `1.32 us` (✅ **1.00x**) | `4.73 us` (❌ *3.59x slower*)   | `87.96 us` (❌ *66.76x slower*)   | `3.34 us` (❌ *2.53x slower*)    |

### Part II 

|        | ` One`                  | ` Two`                         | ` Three`                          | ` Four`                         |
|:-------|:------------------------|:-------------------------------|:----------------------------------|:------------------------------- |
|        | `6.93 us` (✅ **1.00x**) | `9.59 us` (❌ *1.38x slower*)   | `630.52 us` (❌ *90.94x slower*)   | `1.37 us` (🚀 **5.05x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

