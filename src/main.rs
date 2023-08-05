use num_bigint::BigInt;
use std::str::FromStr;
use structopt::StructOpt;








#[derive(StructOpt, Debug)]
enum Command{
    #[structopt(name = "str_to_felt")]
    StrToFelt{
        #[structopt(name= "INPUT_STRING")]
        input_string:String
    },

    #[structopt(name = "felt_to_str")]
    FeltToStr{
        #[structopt(name= "INPUT_FELT")]
        input_string:String
    },

}
/// Converts a string to a felt (short string).
/// A felt represents up to 31 utf-8 characters.
/// If the string contains more than 31 characters, it will return an error.
/// # Arguments
///
/// * `str` - The string to convert
///
/// # Returns
///
/// The string converted to a felt (short string) as a BigInt.
///
/// # Panics
///
/// This function will panic if the given string contains non-UTF-8 characters.

#[allow(dead_code)]
fn str_to_felt(str: &str) -> Result<BigInt, &'static str> {
    if str.len() > 31 {
        return Err("unable to convert to felt: string greater than 31 chars");
    }

    let ss: String = str.chars().map(|c| format!("{:x}", c as u32)).collect();
    let felt = BigInt::parse_bytes(ss.as_bytes(), 16).ok_or("Invalid BigInt")?;

    Ok(felt)
}


#[allow(dead_code)]
fn felt_to_str(felt: &BigInt) -> String {
    felt.to_str_radix(16)
}


fn main() {
let command = Command::from_args();

    match command{
        Command::StrToFelt { input_string } =>{
            match str_to_felt(&input_string){
                Ok(result) => println!("Felt representation: {}", result),
                Err(error) => eprint!("Error: {}", error)
            }
        }
        Command::FeltToStr { input_string } => {
            let felt = BigInt::from_str(&input_string).unwrap();
             println!("String representation: {}",felt)
            
        }
    }
}
