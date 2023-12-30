use error_chain::error_chain;
use std::io::Read;

error_chain!{
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()>{
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body = String::new();
    let _ = res.read_to_string(&mut body);

    print!("Status: {}",res.status());
    print!("Headers: \n{:#?}",res.headers());
    print!("Body: \n{}",body);
    Ok(())
}