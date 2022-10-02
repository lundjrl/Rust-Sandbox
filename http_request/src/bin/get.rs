use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

// Just an example of a simple GET request in rust.
fn main() -> Result<()> {
    let uri = "https://www.rust-lang.org";
    let res = reqwest::blocking::get(uri);

    println!("{:?}", res);

    Ok(())
}
