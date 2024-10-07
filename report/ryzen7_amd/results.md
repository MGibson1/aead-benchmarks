# Criterion results
These results were obtained on an M2 mac with the following `lscpu`:

```
Architecture:                         x86_64
CPU op-mode(s):                       32-bit, 64-bit
Address sizes:                        48 bits physical, 48 bits virtual
Byte Order:                           Little Endian
CPU(s):                               16
On-line CPU(s) list:                  0-15
Vendor ID:                            AuthenticAMD
Model name:                           AMD Ryzen 7 PRO 6850U with Radeon Graphics
CPU family:                           25
Model:                                68
Thread(s) per core:                   2
Core(s) per socket:                   8
Socket(s):                            1
Stepping:                             1
CPU(s) scaling MHz:                   18%
CPU max MHz:                          4768.0000
CPU min MHz:                          400.0000
BogoMIPS:                             5390.01
Virtualization:                       AMD-V
L1d cache:                            256 KiB (8 instances)
L1i cache:                            256 KiB (8 instances)
L2 cache:                             4 MiB (8 instances)
L3 cache:                             16 MiB (1 instance)
NUMA node(s):                         1
NUMA node0 CPU(s):                    0-15

```

## 100 bytes
||AES-GCM|AES-GCM-SIV|ChaCha20-Poly1305|XChaCha20-Poly1305||
|---|---|---|---|---|---|
|decrypt|149.17 ns (670.70 ps)|385.67 ns (1.9002 ns)|1.3924 µs (6.2400 ns)|1.5994 µs (6.1313 ns)|
|encrypt|146.64 ns (2.9948 ns)|367.55 ns (1.9826 ns)|1.3586 µs (9.1087 ns)|1.5600 µs (8.2457 ns)|

<img src="encryption/100B.svg" width="800"/>

## 16 KibiBytes 
||AES-GCM|AES-GCM-SIV|ChaCha20-Poly1305|XChaCha20-Poly1305||
|---|---|---|---|---|---|
|decrypt|9.5935 µs (96.851 ns)|10.043 µs (37.483 ns)|9.3187 µs (68.844 ns)|9.7980 µs (35.852 ns)|
|encrypt|9.7378 µs (37.879 ns)|9.9770 µs (50.763 ns)|9.1658 µs (38.369 ns)|9.5546 µs (48.578 ns)|

<img src="encryption/16KiB.svg" width="800"/>

## 256 KibiBytes
||AES-GCM|AES-GCM-SIV|ChaCha20-Poly1305|XChaCha20-Poly1305||
|---|---|---|---|---|---|
|decrypt|157.66 µs (770.52 ns)|155.49 µs (1.5188 µs)|135.95 µs (530.76 ns)|134.98 µs (581.15 ns)|
|encrypt|155.92 µs (914.12 ns)|153.60 µs (724.83 ns)|133.99 µs (893.74 ns)|133.40 µs (627.32 ns)|

<img src="encryption/256KiB.svg" width="800"/>

## 1 MebiByte
||AES-GCM|AES-GCM-SIV|ChaCha20-Poly1305|XChaCha20-Poly1305||
|---|---|---|---|---|---|
|decrypt|621.80 µs (4.5602 µs)|632.24 µs (2.4292 µs)|536.84 µs (2.7317 µs)|545.60 µs (2.4655 µs)|
|encrypt|624.39 µs (3.4786 µs)|626.64 µs (3.0351 µs)|535.99 µs (3.0564 µs)|534.31 µs (3.2762 µs)|

<img src="encryption/1MiB.svg" width="800"/>
