extern crate build_script_file_gen;

use build_script_file_gen::gen_file;

fn main(){
    gen_file("hello.txt", "hello wold");
}