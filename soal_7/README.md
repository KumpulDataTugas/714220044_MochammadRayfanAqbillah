# 📚 Perbedaan Antara `main.rs` dan `lib.rs` di Rust

Dalam proyek Rust, kita dapat membuat dua jenis file utama: `main.rs` dan `lib.rs`. Keduanya memiliki tujuan dan cara penggunaan yang berbeda.

---

## ✅ `main.rs` – File Utama Program

### 📌 Tujuan:
Untuk membuat program **yang bisa langsung dijalankan** (executable).

### 🧠 Karakteristik:
- Memiliki fungsi `fn main()` sebagai titik awal program.
- Bisa langsung dijalankan menggunakan `cargo run`.
- Digunakan saat kita ingin membuat aplikasi/CLI yang berdiri sendiri.

### 💡 Contoh:
```rust
// src/main.rs
fn main() {
    println!("Halo, ini adalah program utama!");
}
