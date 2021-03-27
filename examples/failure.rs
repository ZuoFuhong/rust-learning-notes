use failure::Fail;

#[derive(Debug, Fail)]
enum ToolchainError {
    #[fail(display = "invalid toolchain name: {}", _0)]
    InvalidToolchainName(String),
}

type Result<T> = std::result::Result<T, ToolchainError>;

fn main() {
    let t = get_token();
    if let Err(e) = t {
        println!("{}", e)
    }
}

fn get_token() -> Result<()> {
    return Err(ToolchainError::InvalidToolchainName("cathy".to_owned()));
}
