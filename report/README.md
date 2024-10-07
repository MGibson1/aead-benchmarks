# Reports
Reports have been collated for a number of systems

- [Intel i5](i5_intel/results.md)
- [Apple M2](m2_mac/results.md)
- [Intel i7](i7_intel/results.md)
- [AMD Ryzen7](ryzen7_amd/results.md)

It would appear the the x86 machines have some serious hardware acceleration going on for AES. At lower sizes, the AES variants are significantly faster than ChaCha variants. This lead diminishes and reverses at larger sizes. Arm Machines have ChaCha leading throughout.
 > AES speed is going to be heavily dependent on the device deployed on.

Additionally, AES-GCM and AES-GCM-SIV have no meaningful performance difference. The same can be said for ChaCha20-Poly1305 and XChaCha20-Poly1305
