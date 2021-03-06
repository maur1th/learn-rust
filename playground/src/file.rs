use std::io;
use std::io::Read;
use std::fs::File;

// pub fn read_username_from_file() -> Result<String, io::Error> {
//   let f = File::open("hello.txt");

//   let mut f = match f {
//       Ok(file) => file,
//       Err(e) => return Err(e),
//   };

//   let mut s = String::new();

//   match f.read_to_string(&mut s) {
//       Ok(_) => Ok(s),
//       Err(e) => Err(e),
//   }
// }

// // Using ? shortcut
// pub fn read_username_from_file() -> Result<String, io::Error> {
//     let mut s = String::new();
//     let mut f = File::open("hello.txt")?;
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// Shorter
pub fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
