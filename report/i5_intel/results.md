# Criterion results
These results were obtained on an M2 mac with the following hardware overview:

```
Model Name: MacBook Air
  Model Identifier: MacBookAir9,1
  Processor Name: Quad-Core Intel Core i5
  Processor Speed: 1.1 GHz
  Number of Processors: 1
  Total Number of Cores: 4
  L2 Cache (per Core): 512 KB
  L3 Cache: 6 MB
  Hyper-Threading Technology: Enabled
  Memory: 8 GB 3733 MHz LPDDR4X

MacOS Sonoma 14.7 (23H124)
```

## 100 bytes
||AES-GCM|AES-GCM-SIV|ChaCha20-Poly1305|XChaCha20-Poly1305||
|---|---|---|---|---|---|
|encrypt|289.11 ns (14.508 ns)|510.14 ns (15.464 ns)|1.9078 µs (57.322 ns)|2.2423 µs (80.757 ns)|
|decrypt|295.77 ns (9.0501 ns)|502.04 ns (12.978 ns)|1.8052 µs (29.116 ns)|2.0920 µs (48.362 ns)|

<img src="encryption/100B.svg" width="800"/>

## 16 KibiBytes 
||AES-GCM|AES-GCM-SIV|ChaCha20-Poly1305|XChaCha20-Poly1305||
|---|---|---|---|---|---|
|encrypt|21.232 µs (277.09 ns)|14.228 µs (135.56 ns)|14.735 µs (422.69 ns)|14.915 µs (370.05 ns)|
|decrypt|21.046 µs (478.67 ns)|13.626 µs (234.54 ns)|14.096 µs (217.35 ns)|14.321 µs (273.52 ns)|

<img src="encryption/16KiB.svg" width="800"/>

## 256 KibiBytes
||AES-GCM|AES-GCM-SIV|ChaCha20-Poly1305|XChaCha20-Poly1305||
|---|---|---|---|---|---|
|encrypt|347.05 µs (8.9228 µs)|231.34 µs (2.9376 µs)|224.01 µs (19.509 µs)|216.26 µs (4.0118 µs)|
|decrypt|351.68 µs (10.682 µs)|220.32 µs (11.959 µs)|206.13 µs (3.7855 µs)|207.49 µs (4.7636 µs)|

<img src="encryption/256KiB.svg" width="800"/>

## 1 MebiByte
||AES-GCM|AES-GCM-SIV|ChaCha20-Poly1305|XChaCha20-Poly1305||
|---|---|---|---|---|---|
|encrypt|1.4131 ms (30.089 µs)|951.03 µs (31.077 µs)|892.42 µs (31.437 µs)|885.14 µs (32.367 µs)|
|decrypt|1.3796 ms (79.465 µs)|880.39 µs (17.370 µs)|834.60 µs (16.033 µs)|845.95 µs (25.043 µs)|

<img src="encryption/1MiB.svg" width="800"/>
