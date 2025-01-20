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
    for (index, arg) in args.iter().enumerate() {
        if {arg == "-o" || arg == "--output"} && outfile.is_empty() {
            outfile = if index+1 != args.len() && !args[index+1].starts_with("-") {args[index+1].clone()} else {println!("Error: output file isn't specified"); return};
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
    if outfile.is_empty() {
        outfile = {infile + "_encrypted"}.to_string();
    }
}

fn decryptfile(infile: String, mut outfile: String) {
    if outfile.is_empty() {
        outfile = {infile + "_decrypted"}.to_string();
    }
}