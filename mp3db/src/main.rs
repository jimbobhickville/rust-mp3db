#![feature(io)]
#![feature(fs_walk)]
#![feature(core)]

use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::env;
use std::str;

fn synchsafe_to_int(synchsafe_vec: &mut[u8; 4]) -> usize {
    let mut result: usize = 0;
    let length = synchsafe_vec.len();

    for i in (0..length) {
        let offset = (length - 1 - i) * 7;
        let digit = synchsafe_vec[i] as usize;
        result += digit << offset;
    }

    return result;
}

fn read_mp3(mp3_path: &Path) {
    let mut file = match File::open(&mp3_path) {
        Err(why) => panic!("couldn't open {}: {}", mp3_path.display(), why),
        Ok(file) => file,
    };

    let id3tag_arr = &mut[0u8; 3];
    match file.read(id3tag_arr) {
        Err(why) => panic!("{}", why),
        _ => {},
    };

    let id3tag_str = match str::from_utf8(id3tag_arr) {
        Ok(e) => e,
        Err(why) => panic!("{}", why),
    };

    let version_arr = &mut[0u8; 2];
    match file.read(version_arr) {
        Err(why) => panic!("{}", why),
        _ => {},
    };

    let flags_arr = &mut[0u8; 1];
    match file.read(flags_arr) {
        Err(why) => panic!("{}", why),
        _ => {},
    };
    let flags = flags_arr[0];

    let unsync = (flags & (1 << 7)) == 1;
    let has_extended_header = (flags & (1 << 6)) == 1;
    let has_footer = (flags & (1 << 4)) == 1;

    let synchsafe_arr = &mut[0u8; 4];
    match file.read(synchsafe_arr) {
        Err(why) => panic!("{}", why),
        _ => {},
    };

    let mut size = synchsafe_to_int(synchsafe_arr);
    if has_footer {
        size += 10;
    }

    let mut remaining_vec = Vec::new();
    let remaining_iter = file.take(size as u64).bytes();
    for remaining_char in remaining_iter {
        match remaining_char {
            Err(why) => panic!("{}", why),
            Ok(c) => remaining_vec.push(c),
        };
    }

    println!("path = {}", mp3_path.display());
    println!("flags = {}", flags);
    println!("unsync = {}", unsync);
    println!("has_extended_header = {}", has_extended_header);
    println!("has_footer = {}", has_footer);
    println!("id3tag = {:?}", id3tag_str);
    println!("Version 2.{}.{} tag", version_arr[0], version_arr[1]);
    println!("Size vec {:?}", synchsafe_arr);
    println!("Remaining headers size {}", size);
    if size < 200 {
        println!("Remaining headers {:?}", remaining_vec);
    }
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
                // break;
            }
        },
    }
}
