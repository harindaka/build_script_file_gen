//! This module encapsulates convenience methods to generate files via build scripts and include their content within source files during build time. 
//!
//! # Examples
//! ```
//! //Step 1: In build.rs (build script) do,
//! extern crate build_script_file_gen;
//! use build_script_file_gen::gen_file_str;
//!
//! fn main() {
//!     let string_content = "Hello World!";
//!     gen_file_str("hello.txt", &string_content);
//!
//!     //or
//!
//!     let rust_code = r#"println!("Hello World!");"#;
//!     gen_file_str("hello.rs", &rust_code);
//! }
//! ```
//!
//! ```
//! //Step 2: In your module do,
//! #[macro_use] 
//! extern crate build_script_file_gen;
//! 
//! fn main() {
//!     //hello.txt contains the text: Hello World!;
//!     //which will make this function print Hello World! when compiled
//!     println!(include_file_str!("hello.txt"));
//!
//!     //or
//!
//!     //hello.rs contains the text: println!("Hello World!");
//!     //which will make this function print Hello World! when compiled
//!     include_file!("hello.rs");
//! }
//! ```

use std::{env};
use std::path::Path;
use std::io::{Write, BufWriter};
use std::fs::File;

/// When used inside a build script (build.rs), generates a file under the specified file name and includes the specified utf8-encoded string as its content.
///
/// The file is created relative to the path denoted by the OUT_DIR environment variable at build time.
///
/// # Examples
/// 
/// ```
/// extern crate build_script_file_gen;
/// use build_script_file_gen::gen_file_str;
///
/// fn main() {
///     let string_content = "Hello World!";
///     gen_file_str("hello.txt", &string_content);
/// }
/// ```
/// ```
/// extern crate build_script_file_gen;
/// use build_script_file_gen::gen_file_str;
///
/// fn main() {
///     let rust_code = r#"println!("Hello World!");"#;
///     gen_file_str("hello.rs", &rust_code);
/// }
/// ```
pub fn gen_file_str(file_name: &str, content: &str){
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(&file_name);
    let mut f = BufWriter::new(File::create(&dest_path).unwrap());

    write!(f, "{}", &content).unwrap();
}

/// Includes the utf8-encoded content of the specified file as a string.
///
/// The file is located relative to the path denoted by the OUT_DIR environment variable at build time.
/// This macro will yield an expression of type &'static str which is the contents of the file.
///
/// # Examples
///
/// ```
/// #[macro_use] 
/// extern crate build_script_file_gen;
/// 
/// fn main() {
///     //hello.txt contains the text: Hello World!;
///     //which will make this function print Hello World! when compiled
///     println!(include_file_str!("hello.txt"));
/// }
/// ```
#[macro_export]
macro_rules! include_file_str {  
    ($file:expr) => {         
        //include_str!(concat!("", $file));
        //let out_dir = env::var("OUT_DIR")
        include_str!(concat!(env!("OUT_DIR"), "/", $file));         
    }
}

/// Places the content of the specified file in the surrounding code unhygienically. i.e. This can be used to inject Rust code into your module.
///
/// The file is located relative to the path denoted by the OUT_DIR environment variable at build time.
///
/// # Examples
///
/// ```
/// #[macro_use] 
/// extern crate build_script_file_gen;
/// 
/// fn main() {
///     //hello.rs contains the text: println!("Hello World!");
///     //which will make this function print Hello World! when compiled
///     include_file!("hello.rs");
/// }
/// ```
#[macro_export]
macro_rules! include_file {  
    ($file:expr) => {         
        //include_str!(concat!("", $file));
        //let out_dir = env::var("OUT_DIR")
        include!(concat!(env!("OUT_DIR"), "/", $file));         
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
