use num_bigint::BigInt;
use structopt::StructOpt;








#[derive(StructOpt, Debug)]
enum Command{
    #[structopt(name = "str_to_felt")]
    StrToFelt{
        #[structopt(name= "INPUT_STRING")]
        input_string:String
    },

    // #[structopt(name = "felt_to_string")]
    // FeltToStr{
    //     #[structopt(name= "INPUT_FELT")]
    //     input_string:String
    // },

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


fn main() {
let command = Command::from_args();

    match command{
        Command::StrToFelt { input_string } =>{
            match str_to_felt(&input_string){
                Ok(result) => println!("Felt representation: {}", result),
                Err(error) => eprint!("Error: {}", error)
            }
        }
        // Command::FeltToStr { input_string } => {
        //     let input_strings: Vec<String> = input_string.split(',').map(|s| s.trim().to_string()).collect();
        //     match strings_to_bigints(input_strings) {
        //         Ok(bigints) => {
        //             let result = felt_to_string(&bigints);
        //             println!("String representation: {}", result);
        //         }
        //         Err(error) => eprint!("Error: {}", error),
        //     }
        // }
    }
}
