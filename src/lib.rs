use std::{env};
use std::path::Path;
use std::io::{Write, BufWriter};
use std::fs::File;

pub fn gen_file_str(file_name: &str, content: &str){
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(&file_name);
    let mut f = BufWriter::new(File::create(&dest_path).unwrap());

    write!(f, "{}", &content).unwrap();
}

#[macro_export]
macro_rules! include_file_str {  
    ($file:expr) => {         
        //include_str!(concat!("", $file));
        //let out_dir = env::var("OUT_DIR")
        include_str!(concat!(env!("OUT_DIR"), $file));         
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
