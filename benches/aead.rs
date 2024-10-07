use aes_gcm::{aead::{Aead, OsRng}, AeadCore, Aes256Gcm, Key as AesGcmKey, KeyInit};
use aes_gcm_siv::{Aes256GcmSiv, Key as AesGcmSivKey};
use chacha20poly1305::{ChaCha20Poly1305, XChaCha20Poly1305};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

const CLEARTEXT_BUFFER: [u8; 1048576] = [0u8; 1048576]; // 1 MiB
const SIZES: [(&str, usize); 7] = [
    ("100 B", 100),
    ("1 KiB", 1024),
    ("4 KiB", 4096),
    ("16 KiB", 16384),
    ("64 KiB", 65536),
    ("256 KiB", 262144),
    ("1 MiB", 1048576),
];

fn encrypt(c: &mut Criterion) {
    let mut group = c.benchmark_group("aead encryption");
    group.measurement_time(std::time::Duration::from_secs(10));
    group.sample_size(50);

    // AES-GCM
    let key = Aes256Gcm::generate_key(OsRng);
    let key = AesGcmKey::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    for (size_str, size) in SIZES {
        let cleartext = &CLEARTEXT_BUFFER[..size];
        group.bench_with_input(BenchmarkId::new("AES-GCM", size_str), &size, |b, _| {
            b.iter(|| {
                let _ = cipher.encrypt(&nonce, black_box(cleartext)).unwrap();
            });
        });
    }

    // AES-GCM-SIV
    let key = Aes256GcmSiv::generate_key(OsRng);
    let key = AesGcmSivKey::<Aes256GcmSiv>::from_slice(&key);
    let cipher = Aes256GcmSiv::new(&key);
    let nonce = Aes256GcmSiv::generate_nonce(&mut OsRng);

    for (size_str, size) in SIZES {
        let cleartext = &CLEARTEXT_BUFFER[..size];
        group.bench_with_input(BenchmarkId::new("AES-GCM-SIV", size_str), &size, |b, _| {
            b.iter(|| {
                let _ = cipher.encrypt(&nonce, black_box(cleartext)).unwrap();
            });
        });
    }

    // ChaCha20-Poly1305
    let key = ChaCha20Poly1305::generate_key(OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    for (size_str, size) in SIZES {
        let cleartext = &CLEARTEXT_BUFFER[..size];
        group.bench_with_input(BenchmarkId::new("ChaCha20-Poly1305", size_str), &size, |b, _| {
            b.iter(|| {
                let _ = cipher.encrypt(&nonce, black_box(cleartext)).unwrap();
            });
        });
    }

    // XChaCha20-Poly1305
    let key = XChaCha20Poly1305::generate_key(OsRng);
    let cipher = XChaCha20Poly1305::new(&key);
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);

    for (size_str, size) in SIZES {
        let cleartext = &CLEARTEXT_BUFFER[..size];
        group.bench_with_input(BenchmarkId::new("XChaCha20-Poly1305", size_str), &size, |b, _| {
            b.iter(|| {
                let _ = cipher.encrypt(&nonce, black_box(cleartext)).unwrap();
            });
        });
    }
}

fn decrypt(c: &mut Criterion) {
    let mut group = c.benchmark_group("aead decryption");
    group.measurement_time(std::time::Duration::from_secs(10));
    group.sample_size(50);

    // AES-GCM
    let key = Aes256Gcm::generate_key(OsRng);
    let key = AesGcmKey::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    for (size_str, size) in SIZES {
        let cleartext = &CLEARTEXT_BUFFER[..size];
        let ciphertext = cipher.encrypt(&nonce, cleartext).unwrap();
        group.bench_with_input(BenchmarkId::new("AES-GCM", size_str), &size, |b, _| {
            b.iter(|| {
                let _ = cipher.decrypt(&nonce, black_box(ciphertext.as_ref())).unwrap();
            });
        });
    }

    // AES-GCM-SIV
    let key = Aes256GcmSiv::generate_key(OsRng);
    let key = AesGcmSivKey::<Aes256GcmSiv>::from_slice(&key);
    let cipher = Aes256GcmSiv::new(&key);
    let nonce = Aes256GcmSiv::generate_nonce(&mut OsRng);

    for (size_str, size) in SIZES {
        let cleartext = &CLEARTEXT_BUFFER[..size];
        let ciphertext = cipher.encrypt(&nonce, cleartext).unwrap();
        group.bench_with_input(BenchmarkId::new("AES-GCM-SIV", size_str), &size, |b, _| {
            b.iter(|| {
                let _ = cipher.decrypt(&nonce, black_box(ciphertext.as_ref())).unwrap();
            });
        });
    }

    // ChaCha20-Poly1305
    let key = ChaCha20Poly1305::generate_key(OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    for (size_str, size) in SIZES {
        let cleartext = &CLEARTEXT_BUFFER[..size];
        let ciphertext = cipher.encrypt(&nonce, cleartext).unwrap();
        group.bench_with_input(BenchmarkId::new("ChaCha20-Poly1305", size_str), &size, |b, _| {
            b.iter(|| {
                let _ = cipher.decrypt(&nonce, black_box(ciphertext.as_ref())).unwrap();
            });
        });
    }

    // XChaCha20-Poly1305
    let key = XChaCha20Poly1305::generate_key(OsRng);
    let cipher = XChaCha20Poly1305::new(&key);
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);

    for (size_str, size) in SIZES {
        let cleartext = &CLEARTEXT_BUFFER[..    size];
        let ciphertext = cipher.encrypt(&nonce, cleartext).unwrap();
        group.bench_with_input(BenchmarkId::new("XChaCha20-Poly1305", size_str), &size, |b, _| {
            b.iter(|| {
                let _ = cipher.decrypt(&nonce, black_box(ciphertext.as_ref())).unwrap();
            });
        });
    }
}

criterion_group!(benches, encrypt, decrypt);
criterion_main!(benches);
