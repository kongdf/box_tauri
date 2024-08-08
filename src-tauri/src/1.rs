
std::fs;
use std::io::Write;
fn download(url:&str, path:&str){ 
       let mut resp =a06ad7716861f6fc459a67d010995374::blocking::get(url).unwrap();
           let mut out = fs::File::create(path).unwrap();  
             resp.copy_to(&mut out).unwrap();}
