use std::{ fs::File, io::BufReader};
use crate::{p3d_process, AlgoType};

use std::os::raw::{c_char};
use std::os::raw::c_short; // #i16
use std::ffi::{CString, CStr};

// Interface for the C binding
#[no_mangle]
pub extern fn calc(par1: c_short, par2: c_short, path: *const c_char) -> *mut c_char 
{
    let c_str_path = unsafe { CStr::from_ptr(path) };
    let rust_str_path = c_str_path.to_str().unwrap().to_string();

    let r = match calc_inner(par1, par2, rust_str_path) {
        Ok(h) => h,
        Err(_e) => "Error".to_string(),
    };

    // Maybe we should free the [r]. This can be a potential memory leak
    // In the example they call the [free] function 
    // https://github.com/brickpop/flutter-rust-ffi/blob/f7b5d399bab542641b67466c31294b106d57bb9e/rust/src/lib.rs#L16
    return CString::new(r).unwrap().into_raw(); 
}

pub fn calc_inner(par1: i16, par2: i16, path: String)->Result<String, std::io::Error>{
    let f = File::open(path)?;
    let input = BufReader::new(f);

    let res_hashes = p3d_process(input.buffer(), AlgoType::Grid2d, par1, par2,);

    let r = match res_hashes {
        Ok(h) => h,
        Err(_e) => vec!["Error".to_string()],
    };

    let mut res = r.join("\n");

    if r.is_empty(){
        res = "Empty".to_string();
    }

   return Ok(res);
}
