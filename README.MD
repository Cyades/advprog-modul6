# Reflection Module 6

- *Nama*    : Malvin Scafi
- *NPM*     : 2306152430
- *Kelas*   : AdvProg A

## Commit 1 Reflection notes

1. What is inside the `handle_connection` method

   Method `handle_connection` berfungsi untuk menangani koneksi TCP dan memproses permintaan HTTP yang masuk.
   
   Isi dari method tersebut:
   
   - Menerima parameter `stream` bertipe `TcpStream` sebagai parameter mutable
   - Membuat `buf_reader` menggunakan `BufReader` untuk membaca data dari stream TCP secara efisien
   - Membuat variabel `http_request` berupa vector yang:
     * Membaca stream baris per baris dengan method `lines()`
     * Menggunakan `map()` untuk mengambil nilai dari setiap baris (dengan `unwrap()`)
     * Menggunakan `take_while()` untuk membaca hingga menemukan baris kosong (penanda akhir header HTTP)
     * Mengumpulkan semua baris ke dalam vector dengan `collect()`
   - Mencetak isi permintaan HTTP ke konsol dengan format debug `{:#?}`
   
   Pada dasarnya, method ini membaca dan menampilkan header-header HTTP yang diterima dari koneksi TCP.

