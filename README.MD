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

## Commit 3 Reflection notes

3. How to split between response and why the refactoring is needed

   Pada kode ini, pembuatan respons HTTP telah dipisahkan dari method `handle_connection` ke dalam method `response` tersendiri. Berikut proses pemisahan fungsi respons:
   
   - Method `response` baru menerima parameter `request_line` (baris pertama dari permintaan HTTP)
   - Method ini mengembalikan String yang berisi respons HTTP lengkap
   - Di dalam method `response`:
     * Dilakukan pengecekan apakah request merupakan "GET / HTTP/1.1"
     * Berdasarkan kondisi tersebut, status line dan filename yang berbeda ditentukan (200 OK atau 404 NOT FOUND)
     * File HTML dibaca dan disiapkan sebagai konten respons
     * Respons HTTP lengkap diformat dan dikembalikan ke pemanggil
   
   Alasan diperlukan refactoring:
   
   1. **Separation of Concerns (Pemisahan Tanggung Jawab)** - Method `handle_connection` sekarang berfokus hanya pada manajemen koneksi, sementara `response` menangani pembuatan format respons
   
   2. **Maintainability (Kemudahan Perawatan)** - Dengan memisahkan logika, kode menjadi lebih mudah dipahami dan dimodifikasi
   
   3. **Testability (Kemudahan Pengujian)** - Function `response` dapat diuji secara terpisah tanpa harus membuat koneksi TCP
   
   4. **Reusability (Dapat Digunakan Kembali)** - Function `response` dapat dipanggil dari berbagai tempat dalam kode jika diperlukan
   
   5. **Flexibility (Fleksibilitas)** - Memudahkan untuk menambahkan kemampuan routing dan jenis respons yang berbeda di masa depan
   
   Refactoring ini merupakan implementasi dari prinsip "Single Responsibility Principle" di mana setiap fungsi hanya memiliki satu tanggung jawab utama.

    ### Menambahkan page 404.html
    ```html
      <!DOCTYPE html>
    <html lang="en">
    <head>
    <meta charset="utf-8"> <title>Oops!</title>
    </head> 
    <body> 
        <h1>Oops!</h1> 
        <p>Sorry, I don't know what you're asking for.</p>
        <p>Rust is running from malvinscafi's machine.</p>
    </body>
    </html>
    ```
    ### Mengubah `handle_connection` dan menambahkan `response`
    ```rust
    fn handle_connection(mut stream:TcpStream){
        let buf_reader = BufReader::new(&mut stream);
        let http_request:Vec<_> = buf_reader
        .lines()
        .map(|result|result.unwrap())
        .take_while(|line|!line.is_empty()) 
        .collect();

        if http_request.is_empty(){
            return;
        }
        let request_line = http_request.get(0).unwrap();
        let response = response(request_line);
        stream.write_all(response.as_bytes()).unwrap();
    }

    fn response(request_line: &str) -> String {
        let get = "GET / HTTP/1.1"; 
        let (status_line, filename) = if request_line == get {
            ("HTTP/1.1 200 OK", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
        let contents = fs::read_to_string(filename).unwrap();
        let length = contents.len();
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}",)
    }
    ```

   ![](/assets/images/commit3.png)

## Commit 4 Reflection notes

4. See how it works and try to understand why it works like that. 

   ### 1. **Penambahan Import untuk Fungsionalitas Sleep**
   - **Sebelum:** Hanya mengimport library dasar (`fs`, `io`, `net`)
     ```rust
     use std::{
         fs,
         io::{prelude::*,BufReader},
         net::{TcpListener, TcpStream},
     };
     ```
   - **Sesudah:** Menambahkan import untuk `thread` dan `time::Duration`
     ```rust
     use std::{
         fs,
         io::{prelude::*, BufReader},
         net::{TcpListener, TcpStream},
         thread,               // Baru
         time::Duration,       // Baru
     };
     ```

   ### 2. **Perubahan Pemrosesan HTTP Request**
   - **Sebelum:** Membaca seluruh header HTTP request ke dalam vector
     ```rust
     let http_request:Vec<_> = buf_reader
         .lines()
         .map(|result|result.unwrap())
         .take_while(|line|!line.is_empty()) 
         .collect();
     ```
   - **Sesudah:** Hanya membaca baris pertama (request line) dari HTTP request
     ```rust
     let request_line = buf_reader.lines().next().unwrap().unwrap();
     ```

   ### 3. **Perubahan Mekanisme Routing**
   - **Sebelum:** Menggunakan kondisi if-else sederhana
     ```rust
     let get = "GET / HTTP/1.1"; 
     let (status_line, filename) = if request_line == get {
         ("HTTP/1.1 200 OK", "hello.html")
     } else {
         ("HTTP/1.1 404 NOT FOUND", "404.html")
     };
     ```
   - **Sesudah:** Menggunakan pattern matching dengan `match` yang lebih fleksibel
     ```rust
     return match &request_line[..] {
         "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
         "GET /sleep HTTP/1.1" => {
             thread::sleep(Duration::from_secs(10));
             ("HTTP/1.1 200 OK", "hello.html")
         }
         _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
     };
     ```

   ### 4. **Penambahan Endpoint Sleep**
   - **Sebelum:** Tidak ada endpoint yang mensimulasikan respons lambat
   - **Sesudah:** Menambahkan endpoint `/sleep` yang menunda respons selama 10 detik
     ```rust
     "GET /sleep HTTP/1.1" => {
         thread::sleep(Duration::from_secs(10));
         ("HTTP/1.1 200 OK", "hello.html")
     }
     ```

   ### 5. **Perubahan Nama dan Return Type Fungsi Respons**
   - **Sebelum:** Fungsi bernama `response` yang mengembalikan `String`
     ```rust
     fn response(request_line: &str) -> String { ... }
     ```
   - **Sesudah:** Fungsi bernama `response` yang mengembalikan tuple `(&str, &str)`
     ```rust
     fn response(request_line: &str) -> (&str, &str) { ... }
     ```

   ### Alasan dan Dampak Perubahan:

   1. **Demonstrasi Blocking:** Penambahan endpoint `/sleep` mendemonstrasikan bagaimana operasi yang lama pada satu koneksi dapat memblokir seluruh server, karena server berjalan dalam satu thread.

   2. **Efisiensi Pemrosesan:** Kode baru hanya membaca baris pertama request HTTP (yang berisi method, path, dan versi), cukup untuk keperluan routing sederhana.

   3. **Fleksibilitas Routing:** Penggunaan `match` memungkinkan penambahan endpoint baru dengan lebih mudah dan membuat kode lebih mudah dibaca untuk kasus kompleks.

   4. **Pemisahan Tanggung Jawab:** Fungsi `response` lebih jelas menggambarkan tanggung jawabnya, yaitu hanya menentukan status dan file yang akan digunakan, tanpa memformat seluruh respons.

   Secara keseluruhan, perubahan ini mempersiapkan codebase untuk demonstrasi kebutuhan multithreading dalam server web, karena menunjukkan bagaimana permintaan lambat dapat memblokir seluruh server.

