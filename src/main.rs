use std::env;
use std::fs;
use std::fs::File;
use std::io::{Write};

fn main()  -> Result<(), Box<dyn std::error::Error>> {    
    let args: Vec<String> = env::args().collect();
    if args.len() >= 4 {
        if &args[1].to_lowercase() == "encode" {
            rle_encode_file(&args[2], &args[3])?;
        } else if &args[1].to_lowercase() == "decode" {
            rle_decode_file(&args[2], &args[3])?;
        }
    } 
    Ok(())
}

fn rle_encode_file(ifilename: &String, ofilename: &String) -> Result<(), Box<dyn std::error::Error>> {
    let buffer: Vec<u8> = fs::read(ifilename)?;
    let encoded = rle_encode(&buffer);
    let mut out_file = File::create(ofilename)?;
    out_file.write_all(&encoded)?;
    Ok(())
}

fn rle_decode_file(ifilename: &String, ofilename: &String) -> Result<(), Box<dyn std::error::Error>> {
    let buffer: Vec<u8> = fs::read(ifilename)?;
    let decoded = rle_decode(&buffer);
    let mut out_file = File::create(ofilename)?;
    out_file.write_all(&decoded)?;
    Ok(())
}

fn rle_encode(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    let mut prev = b' ';
    let mut cnt: u16 = 0;

    for &byte in input {
        if cnt == 0 {
            prev = byte;
            cnt = 1;
        }
        else if byte == prev && cnt < 259 && !(cnt == 258 && prev == 0xFF) {
            cnt += 1;
        } else {
            if cnt >= 4 {                
                output.push(0xFF);
                output.push(prev);       
                if prev == 0xFF {output.push(0xFF);}
                output.push((cnt - 4) as u8);
            } else {
                if prev == 0xFF {cnt *= 2;}
                output.extend(std::iter::repeat(prev).take(cnt as usize));
            }
            prev = byte;
            cnt = 1;
        }
    }

    if cnt >= 4 {
        output.push(0xFF);
        output.push(prev);
        if prev == 0xFF {output.push(0xFF);}
        output.push((cnt - 4) as u8);
    } else {
        if prev == 0xFF {cnt *= 2;}
        output.extend(std::iter::repeat(prev).take(cnt as usize));
    }

    output
}

fn rle_decode(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    let mut i = 0;

    while i < input.len() {
        let byte = input[i];
        let mut push_out = true;                
        if byte == 0xFF && i + 1 < input.len() {             
            let next_byte = input[i+1];
            if next_byte == 0xFF {
                if i + 3 < input.len() && input[i+2] == 0xFF { 
                    if input[i+3] != 0xFF {
                        let count = input[i+3] as u16 + 4;
                        output.extend(std::iter::repeat(next_byte).take(count as usize));
                    } else {
                        output.extend(std::iter::repeat(next_byte).take(2));                                    
                    }
                    i += 4;
                    push_out = false;
                } else {i += 1;}
            } else if i + 2 < input.len() {                
                let count = input[i+2] as u16 + 4;
                output.extend(std::iter::repeat(next_byte).take(count as usize));
                push_out = false;
                i += 3;
            }
        }
        if push_out {
            output.push(byte);
            i += 1;
        }        
    }
    output
}