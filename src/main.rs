use std::io;
use std::io::Write;
//use aes::Aes128;
//use aes::cipher::{
//    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
//    generic_array::GenericArray,
//};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut infile: String = String::new(); // input file name/path 
    let mut outfile: String = String::new(); // output file name/path
    let mut decrypt: bool = false; // Is decrypting
    let mut encrypt: bool = false; // Is encrypting
    if args.len() > 1 {
        for (index, arg) in args.iter().enumerate() {
            if arg == "-o" || arg == "--output" {
                outfile = args[index+1].clone();
            }
            if arg == "-d" || arg == "--decrypt" {
                decrypt = true;
            }
            if arg == "-e" || arg == "--encrypt" {
                encrypt = true;
            }
            if arg == "-h" || arg == "--help" {
                println!("Help: {} -o <output> [-d] [-e] <input_file>", args[0]);
                return
            }
        }

        infile = args[args.len()-1].clone();
        if encrypt && decrypt {
            println!("Error: cannot both encrypt and decrypt");
            return
        }
        else if encrypt == false && decrypt == false {
            println!("Error: you should specify decrypt or encrypt");
            return
        }
        else if !infile.is_empty() {
            if encrypt {
                encryptfile(infile, outfile);
            }
            else if decrypt {
                decryptfile(infile, outfile);
            }
        }
    }
    else {
        println!("Usage: {} -o <output> [-d] [-e] <input_file>", args[0])
    }
}

fn encryptfile(infile: String, mut outfile: String) {
    if outfile.is_empty() {
        outfile = {infile + "_encrypted"}.to_string();
    }
}

fn decryptfile(infile: String, mut outfile: String) {
    if outfile.is_empty() {
        outfile = {infile + "_decrypted"}.to_string();
    }
}