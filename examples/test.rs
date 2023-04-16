//use std::collections::HashMap;
//use std::io;
use text_io::read;
fn main(){
//     println!("ini println1");
//     println!("ini println2");

//     print!("Ini print biasa");
//     println!("ini println");


//     println!("2 kali 3 = {}", 2*3);

//     // variable string
//     let bahasa: &str = "Rust";
//     println!("bahasa pemrograman dengan {}", bahasa);

//     //variable integer
//     let tahun: i32 = 2023;
//     println!("sekarang tahun {}", tahun);

//     //input
//     println!("berapa tahun umur anda?");
//     let umur:i32  = read!();
//     println!("umur anda adalah {}", umur);

//     //inputan persegi panjang
//     println!("berapa panjang nya?");
//     let _panjang: i32 = read!();
//     println!("berapa lebarnya nya?");
//     let _lebar: i32 = read!();

//     println!("jadi kelilingnya {}",  _panjang*_lebar);
    
    
//    for i in 1 .. 11{
//        println!("cek {}", i);
//    }

//    for i in 1 .. 11{
//         if i % 2 != 0 {
//             println!("perulangan ke  {}, sisa bagi 2 adalah = 1", i);
//         }
//     }



  
// let mut total_harga = 0; // Inisialisasi total harga awal

// loop {
//     // Input harga barang
//     println!("Masukkan harga item belanja:");
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//     let harga: u32 = input.trim().parse().unwrap();

//     // Tambahkan harga barang ke total harga
//     total_harga += harga;

//     // Tanya apakah ada lagi item yang dibeli
//     println!("Ada lagi? (y/t)");
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//     let jawaban = input.trim();

//     if jawaban != "y" {
//         break; // Keluar dari loop jika pengguna tidak ingin menambah item lagi
//     }
// }

// // Output total harga
// println!("Total Harga Belanja: {}", total_harga);

    // println!("Masukkan umur Anda:");
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();
    // let umur: u32 = input.trim().parse().unwrap();

    // // Kelompokkan umur berdasarkan rentang usia
    // let kelompok = if umur < 5 {
    //     "Balita"
    // } else if umur < 13 {
    //     "Anak-anak"
    // } else if umur < 18 {
    //     "Remaja"
    // } else if umur < 55 {
    //     "Dewasa"
    // } else {
    //     "Tua"
    // };

    // // Output kelompok umur
    // println!("Anda termasuk dalam kelompok {}.", kelompok);





    // //memunculkan nama ,tgl lahir, no hp
    // println!("masukan nama anda: ");
    // let nama: String = read!();
    // println!("masukan tanggal lahir: ");
    // let tahun_lahir: i32 = read!();
    // println!("masukan nomor hp: ");
    // let no_hp: String = read!();

    // print!("Nama saya adalah : {}", nama);
    // println!(" ,  Lahir pada tahun : {}.", tahun_lahir);
    // println!("Nomor HP saya adalah : {}", no_hp);


    // println!("masukan jumlah menit");
    // let jumlah_menit: u32 = read!();

    // let jam = jumlah_menit / 60;
    // let menit = jumlah_menit % 60;

    // println!("{} jam {} menit", jam, menit);


    // println!("Selamat datang di mini kalkulator!");


    // println!("Masukkan bilangan pertama:");
    // let num1: f32 = read!();
    // println!("Masukkan bilangan kedua:");
    // let num2: f32 = read!();

    // println!("Pilih operasi matematika yang diinginkan:");
    // println!("1. Penjumlahan (+)");
    // println!("2. Pengurangan (-)");
    // println!("3. Perkalian (*)");
    // println!("4. pembagian (/)");

    // let choice: i32 = read!();
    // let result = match choice {
    //     1 => num1 + num2,
    //     2 => num1 - num2,
    //     3 => num1 * num2,
    //     4 => num1 / num2,
    //     _ => {
    //         println!("Operasi yang dipilih tidak valid!");
    //         return;
    //     }
    // };
    // println!("Hasil dari operasi matematika adalah: {}", result);


     // Meminta input jumlah siswa
     print!("Masukkan jumlah siswa: ");
     let n: i32 = read!();
 
     let mut count_lulus = 1; // Menghitung jumlah siswa yang lulus
 
     // Meminta input nilai dari masing-masing siswa dan menghitung siswa yang lulus
     for i in 1..=n {
         print!("Masukkan nilai siswa {} : ", i);
         let nilai: f32 = read!();
 
         if nilai >= 6.0 {
             count_lulus == 1;
         }
     }
 
     // Menampilkan jumlah siswa yang lulus
     println!("Jumlah siswa yang lulus: {}", count_lulus);
    

   

}