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
    let mut infile: String;
    let mut outfile: String;
    let mut decrypt: bool = false;
    let mut encrypt: bool = false;
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
        }
        if encrypt && decrypt {
            println!("Error: cannot both encrypt and decrypt");
        }
    }
    else if args.len() < 1 {
        println!("Usage: {} -o <output> [-d] [-e] <input_file>", args[0])
    }
}
