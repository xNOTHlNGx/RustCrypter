use std::io;
use std::io::Write;
use std::env;
use openssl::symm::{encrypt, decrypt, Cipher};
use rand::Rng;
use std::fs;
use std::path::Path;


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut infile: String = String::new(); // input file name/path 
    let mut outfile: String = String::new(); // output file name/path
    let mut decrypt: bool = false; // Is decrypting
    let mut encrypt: bool = false; // Is encrypting
    for (index, arg) in args.iter().enumerate() {
        if {arg == "-o" || arg == "--output"} && outfile.is_empty() {
            outfile = if index+1 != args.len() && !args[index+1].starts_with("-") {args[index+1].clone()} else {println!("Error: output file isn't specified"); return};
            if Path::new(&outfile).is_file() {
                println!("Error: output file {} already exists", &outfile);
                return
            }
        }
        else if {arg == "-d" || arg == "--decrypt"} && index != args.len()-1 {
            decrypt = true;
        }
        else if {arg == "-e" || arg == "--encrypt"} && index != args.len()-1 {
            encrypt = true;
        }
        else if arg == "-h" || arg == "--help"  {
            println!("Help: {} -o <output> [-d] [-e] <input_file>", args[0]);
            return
        }
        else if index == args.len()-1 {
            if args[index].starts_with("-") {
                println!("Error: input file isn't specified");
                return
            }
            infile = arg.clone();
        }
    }
    if encrypt == decrypt {
        println!("Error: check arguments, must be either decrypt or encrypt, not both, not without it.\nFor more info type {} -h", args[0]);
        return
    }
    if args.len() == 3 || {args.len() == 5 && !infile.is_empty()} {
        if !infile.is_empty() {
            if encrypt {
                encryptfile(infile, outfile);
            }
            else if decrypt {
                decryptfile(infile, outfile);
            }
        }
    }
    else {
        println!("Usage: {} -o <output> [-d] [-e] <input_file>", args[0]);
        return
    }
}

fn encryptfile(infile: String, mut outfile: String) {
    let cipher = Cipher::aes_256_cbc();
    let key = rand::thread_rng().gen::<[u8; 32]>();
    let iv = rand::thread_rng().gen::<[u8; 16]>();
    let data = fs::read(&infile)
        .expect(&format!("Error: failed to read file {}", &infile));
    let dataenc = encrypt(
        cipher,
        &key,
        Some(&iv),
        &data).unwrap();

    if outfile.is_empty() {
        let infilename = Path::new(&infile).file_stem().unwrap().to_str().unwrap();
        let infileext = Path::new(&infile).extension().unwrap().to_str().unwrap();

        outfile = format!("{}.enc.{}", infilename, infileext);
    }
    if Path::new(&outfile).is_file() {
        println!("Error: output file {} already exists", &outfile);
        return
    }
    println!("Key (256 bit): {}\nIV (128 bit): {}",hex::encode(&key), hex::encode(&iv));
    fs::File::create(&outfile).expect(&format!("Unable to create file {}", &outfile));
    fs::write(&outfile, dataenc).expect(&format!("Unable to write file {}", &outfile));
}

fn decryptfile(infile: String, mut outfile: String) {
    let cipher = Cipher::aes_256_cbc();
    let data = fs::read(&infile)
    .expect(&format!("Error: failed to read file {}", &infile));
    let mut key = String::new();
    let mut iv = String::new();
    print!("Enter key(256 bit): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut key)
        .expect("Failed to read key");
    key = key.trim().to_string();

    print!("Enter iv(128 bit): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut iv)
        .expect("Failed to read IV");
    iv = iv.trim().to_string();

    let key = hex::decode(key).expect("Failed to raw get key");
    let iv = hex::decode(iv).expect("Failed to get raw IV");

    let datadec = decrypt(
        cipher,
        &key,
        Some(&iv),
        &data).unwrap();

    if outfile.is_empty() {
        let infilename = Path::new(&infile).file_stem().unwrap().to_str().unwrap();
        let infileext = Path::new(&infile).extension().unwrap().to_str().unwrap();

        outfile = format!("{}.dec.{}", infilename, infileext);
    }
    if Path::new(&outfile).is_file() {
        println!("Error: output file already exists {}", &outfile);
        return
    }
    fs::File::create(&outfile).expect(&format!("Unable to create file {}", &outfile));
    fs::write(&outfile, datadec).expect(&format!("Unable to write file {}", &outfile));
}