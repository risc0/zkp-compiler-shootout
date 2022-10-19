# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Sudoku: compile](#sudoku:-compile)
    - [Sudoku: prove](#sudoku:-prove)
    - [Sudoku: verify](#sudoku:-verify)
    - [Sudoku:](#sudoku:)
    - [fibonacci: compile](#fibonacci:-compile)
    - [fibonacci: prove](#fibonacci:-prove)
    - [fibonacci: verify](#fibonacci:-verify)
    - [fibonacci:](#fibonacci:)
    - [fibonacci large: compile](#fibonacci-large:-compile)
    - [fibonacci large: prove](#fibonacci-large:-prove)
    - [fibonacci large: verify](#fibonacci-large:-verify)
    - [fibonacci large:](#fibonacci-large:)

## Benchmark Results

### Sudoku: compile

|      | `Miden`                 | `Plonk: 3 by 3`                 | `Risc`                           | `Halo: 3 by 3`                   |
| :--- | :---------------------- | :------------------------------ | :------------------------------- | :------------------------------- |
|      | `3.77 ms` (✅ **1.00x**) | `202.05 ms` (❌ *53.57x slower*) | `526.57 us` (🚀 **7.16x faster**) | `488.94 ms` (❌ *129.63x slower*) |

### Sudoku: prove

|      | `Miden`                   | `Plonk: 3 by 3`                  | `Risc`                           | `Halo: 3 by 3`                   |
| :--- | :------------------------ | :------------------------------- | :------------------------------- | :------------------------------- |
|      | `387.45 ms` (✅ **1.00x**) | `131.34 ms` (🚀 **2.95x faster**) | `428.52 ms` (✅ **1.11x slower**) | `218.70 ms` (✅ **1.77x faster**) |

### Sudoku: verify

|      | `Miden`                 | `Plonk: 3 by 3`               | `Risc`                         | `Halo: 3 by 3`               |
| :--- | :---------------------- | :---------------------------- | :----------------------------- | :--------------------------- |
|      | `2.26 ms` (✅ **1.00x**) | `16.39 ms` (❌ *7.25x slower*) | `2.27 ms` (✅ **1.00x slower**) | `4.84 ms` (❌ *2.14x slower*) |

### Sudoku:

|      | `Miden`                   | `Plonk: 3 by 3`                  | `Risc`                           | `Halo: 3 by 3`                 |
| :--- | :------------------------ | :------------------------------- | :------------------------------- | :----------------------------- |
|      | `395.03 ms` (✅ **1.00x**) | `361.01 ms` (✅ **1.09x faster**) | `353.68 ms` (✅ **1.12x faster**) | `703.97 ms` (❌ *1.78x slower*) |

### fibonacci: compile

|      | `Miden: iter-93`          | `Miden: fixed-92`                | `Miden: fixed-50`               | `Risc0: iter-93`               | `Risc0: iter-50`               | `Risc0: fixed-50`              | `Risc0: fixed-92`              |
| :--- | :------------------------ | :------------------------------- | :------------------------------ | :----------------------------- | :----------------------------- | :----------------------------- | :----------------------------- |
|      | `159.59 us` (✅ **1.00x**) | `131.04 us` (✅ **1.22x faster**) | `99.47 us` (✅ **1.60x faster**) | `407.19 us` (❌ *2.55x slower*) | `452.48 us` (❌ *2.84x slower*) | `422.78 us` (❌ *2.65x slower*) | `432.46 us` (❌ *2.71x slower*) |

### fibonacci: prove

|      | `Miden: iter-93`          | `Miden: fixed-92`                | `Miden: fixed-50`                | `Risc0: iter-93`                 | `Risc0: iter-50`               | `Risc0: fixed-50`                | `Risc0: fixed-92`                |
| :--- | :------------------------ | :------------------------------- | :------------------------------- | :------------------------------- | :----------------------------- | :------------------------------- | :------------------------------- |
|      | `393.16 ms` (✅ **1.00x**) | `186.38 ms` (🚀 **2.11x faster**) | `188.14 ms` (🚀 **2.09x faster**) | `409.87 ms` (✅ **1.04x slower**) | `532.66 ms` (❌ *1.35x slower*) | `382.13 ms` (✅ **1.03x faster**) | `148.94 ms` (🚀 **2.64x faster**) |

### fibonacci: verify

|      | `Miden: iter-93`        | `Miden: fixed-92`              | `Miden: fixed-50`              | `Risc0: iter-93`               | `Risc0: iter-50`               | `Risc0: fixed-50`              | `Risc0: fixed-92`              |
| :--- | :---------------------- | :----------------------------- | :----------------------------- | :----------------------------- | :----------------------------- | :----------------------------- | :----------------------------- |
|      | `3.57 ms` (✅ **1.00x**) | `2.48 ms` (✅ **1.44x faster**) | `2.22 ms` (✅ **1.61x faster**) | `2.19 ms` (✅ **1.63x faster**) | `3.02 ms` (✅ **1.18x faster**) | `3.13 ms` (✅ **1.14x faster**) | `3.08 ms` (✅ **1.16x faster**) |

### fibonacci:

|      | `Miden: iter-93`          | `Miden: fixed-92`                | `Miden: fixed-50`                | `Risc0: iter-93`                 | `Risc0: iter-50`                 | `Risc0: fixed-50`                | `Risc0: fixed-92`                |
| :--- | :------------------------ | :------------------------------- | :------------------------------- | :------------------------------- | :------------------------------- | :------------------------------- | :------------------------------- |
|      | `399.86 ms` (✅ **1.00x**) | `207.72 ms` (🚀 **1.92x faster**) | `193.42 ms` (🚀 **2.07x faster**) | `408.10 ms` (✅ **1.02x slower**) | `199.59 ms` (🚀 **2.00x faster**) | `334.50 ms` (✅ **1.20x faster**) | `220.36 ms` (🚀 **1.81x faster**) |

### fibonacci large: compile

|      | `Miden: iter-1000`        | `Risc0: iter-1000`             |
| :--- | :------------------------ | :----------------------------- |
|      | `166.64 us` (✅ **1.00x**) | `446.35 us` (❌ *2.68x slower*) |

### fibonacci large: prove

|      | `Miden: iter-1000`     | `Risc0: iter-1000`               |
| :--- | :--------------------- | :------------------------------- |
|      | `3.37 s` (✅ **1.00x**) | `698.39 ms` (🚀 **4.82x faster**) |

### fibonacci large: verify

|      | `Miden: iter-1000`      | `Risc0: iter-1000`             |
| :--- | :---------------------- | :----------------------------- |
|      | `2.72 ms` (✅ **1.00x**) | `2.28 ms` (✅ **1.19x faster**) |

### fibonacci large:

|      | `Miden: iter-1000`     | `Risc0: iter-1000`               |
| :--- | :--------------------- | :------------------------------- |
|      | `3.38 s` (✅ **1.00x**) | `731.00 ms` (🚀 **4.62x faster**) |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

