# Criterion results
These results were obtained on an M2 mac with the following hardware overview:

```
Windows 11
Version	10.0.22631 Build 22631

Processor	Intel(R) Core(TM) i9-14900KF, 3200 Mhz, 24 Core(s), 32 Logical Processor(s)
Installed Physical Memory (RAM)	32.0 GB (2x16 GB) 4800 Mbps SODIMM
```

## 100 bytes
||AES-GCM|AES-GCM-SIV|ChaCha20-Poly1305|XChaCha20-Poly1305||
|---|---|---|---|---|---|
|encrypt|125.44 ns (9.6145 ns)|269.80 ns (12.558 ns)|977.86 ns (23.978 ns)|1.0680 µs (30.585 ns)|
|decrypt|157.06 ns (4.1083 ns)|307.16 ns (4.3282 ns)|1.0004 µs (38.281 ns)|1.0731 µs (12.888 ns)|

<img src="encryption/100B.svg" width="800"/>

## 16 KibiBytes 
||AES-GCM|AES-GCM-SIV|ChaCha20-Poly1305|XChaCha20-Poly1305||
|---|---|---|---|---|---|
|encrypt|7.0482 µs (206.47 ns)|6.9756 µs (190.99 ns)|7.7871 µs (281.54 ns)|7.8629 µs (266.92 ns)|
|decrypt|6.8179 µs (231.66 ns)|7.1299 µs (170.81 ns)|7.7661 µs (381.64 ns)|8.1177 µs (319.24 ns)|

<img src="encryption/16KiB.svg" width="800"/>

## 256 KibiBytes
||AES-GCM|AES-GCM-SIV|ChaCha20-Poly1305|XChaCha20-Poly1305||
|---|---|---|---|---|---|
|encrypt|111.62 µs (3.7597 µs)|113.16 µs (4.4421 µs)|115.55 µs (5.4957 µs)|113.15 µs (2.8442 µs)|
|decrypt|108.14 µs (1.3167 µs)|108.08 µs (1.1938 µs)|113.93 µs (2.2202 µs)|114.59 µs (2.6999 µs)|

<img src="encryption/256KiB.svg" width="800"/>

## 1 MebiByte
||AES-GCM|AES-GCM-SIV|ChaCha20-Poly1305|XChaCha20-Poly1305||
|---|---|---|---|---|---|
|encrypt|653.40 µs (31.474 µs)|658.80 µs (26.286 µs)|653.40 µs (32.230 µs)|660.74 µs (19.871 µs)|
|decrypt|605.79 µs (7.7279 µs)|608.18 µs (6.3183 µs)|642.57 µs (16.350 µs)|646.09 µs (14.700 µs)|

<img src="encryption/1MiB.svg" width="800"/>
