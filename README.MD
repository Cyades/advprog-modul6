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

## Commit 2 Reflection notes

2. What does the additional lines of code in `handle_connection` do?

   Baris-baris kode tambahan pada method `handle_connection` berfungsi untuk membangun dan mengirimkan respons HTTP ke klien. Berikut adalah detail fungsinya:
   
   - `let status_line = "HTTP/1.1 200 OK";` - Membuat baris status HTTP yang menandakan respons sukses (kode 200)
   - `let contents = fs::read_to_string("hello.html").unwrap();` - Membaca konten dari file "hello.html" dan menyimpannya sebagai string
   - `let length = contents.len();` - Menghitung panjang konten file dalam byte
   - Blok `format!()` - Menggabungkan semua elemen respons HTTP:
     * Baris status
     * Header Content-Length dengan ukuran file
     * Dua sequence CRLF (`\r\n\r\n`) untuk memisahkan header dan body
     * Konten file sebagai body respons
   - `stream.write_all(response.as_bytes()).unwrap();` - Menulis respons HTTP yang sudah diformat ke stream TCP untuk dikirim ke klien
   
   Secara keseluruhan, baris-baris ini melengkapi fungsionalitas server HTTP sederhana dengan mengirimkan file HTML sebagai respons terhadap setiap permintaan yang diterima.

   ![](/assets/images/commit2.png)
   