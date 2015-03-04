#![feature(path)]
#![feature(io)]
#![feature(fs)]
#![feature(os)]

use std::fs;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use std::env;

fn synchsafe_to_int(synchsafe_vec: &Vec<u8>) -> usize {
    let mut result: usize = 0;
    let length = synchsafe_vec.len();
    // This is wrong

    for i in (0..length) {
        let offset = (length - 1 - i) * 7;
        let digit = synchsafe_vec[i] as usize;
        println!("{}:{} (off: {}) = {}", i, digit, offset, digit << offset);
        result += digit << offset;
    }

    return result;
}

fn read_mp3(mp3_path: &PathBuf) {
    let mut file = match File::open(&mp3_path) {
        Err(why) => panic!("couldn't open {}: {}", mp3_path.display(), why),
        Ok(file) => file,
    };

    let mut contents = Vec::new();
    match file.read_to_end(&mut contents) {
        Err(why) => panic!("failed to read id3 tag from file: {}", why),
        _ => {},
    };

    let mut id3tag = String::new();
    let mut version = Vec::new();
    let mut flags: u8 = 0;
    let mut size_vec = Vec::new();
    let mut size = 0;
    let mut headers = String::new();

    // TODO: loop until we find the ID3 tag instead of assuming beginning
    // TODO: figure out how to read in just a few bytes of the file
    for byte in contents.into_iter() {
        if id3tag.len() < 3 {
            id3tag.push(byte as char);
            continue;
        }
        if version.len() < 2 {
            version.push(byte);
            continue;
        }
        if flags == 0 {
            flags = byte;
            continue;
        }
        if size_vec.len() < 4 {
            size_vec.push(byte);
            continue;
        }
        if size == 0 {
            size = synchsafe_to_int(&size_vec)
        }
        if headers.len() < size {
            headers.push(byte as char)
        }
    }

    println!("Version 2.{}.{} tag", version[0], version[1]);
    println!("Size vec {:?}", size_vec);
    println!("Remaining headers size {}", size);
}

fn main() {
    let args: Vec<String> = env::args().map(|x| x.to_string()).collect();
    let ref start = args[1];

    match fs::walk_dir(&Path::new(&start)) {
        Err(why) => panic!("! {:?}", why),
        Ok(entries) => for entry in entries {
            let path = match entry {
                Err(why) => panic!(why),
                Ok(entry) => entry.path(),
            };
            let extension = match path.extension() {
                None => continue,
                Some(extension) => extension,
            };
            let ext = match extension.to_str() {
                None => continue,
                Some(ext) => ext.as_slice(),
            };
            if ext == "mp3" {
                read_mp3(&path);
                break;
            }
        },
    }
}
