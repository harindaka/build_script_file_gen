use std::{env};
use std::path::Path;
use std::io::{Write, BufWriter};
use std::fs::File;

pub fn gen_file(file_name: &str, content: &str){
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(&file_name);
    let mut f = BufWriter::new(File::create(&dest_path).unwrap());

    write!(f, "{}", &content).unwrap();
}

// pub fn read_file(file_name: &str) -> &'static str {
//     static LONG_STRING: &'static str = include_str!(concat!(env!("OUT_DIR"), "/hello_world.txt"));
//     return LONG_STRING;
// }




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
