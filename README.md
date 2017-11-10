# build_script_file_gen
A Rust library which encapsulates convenience methods to generate files via build scripts and include their content within source files during build time.

1. In build.rs (build script) do,

```
extern crate build_script_file_gen;
use build_script_file_gen::gen_file_str;

fn main() {
    let string_content = "Hello World!";
    gen_file_str("hello.txt", &string_content);

    //or

    let rust_code = r#"println!("Hello World!");"#;
    gen_file_str("hello.rs", &rust_code);
}
```

2. In your module do,

```
#[macro_use] 
extern crate build_script_file_gen;
 
fn main() {
    //hello.txt contains the text: Hello World!;
    //which will make this function print Hello World! when compiled
    println!(include_file_str!("hello.txt"));

    //or

    //hello.rs contains the text: println!("Hello World!");
    //which will make this function print Hello World! when compiled
    include_file!("hello.rs");
}
```

