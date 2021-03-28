//! failure有两个核心组件
//!
//! - Fail: 定制错误类型用的trait
//! - Error: 只要实现了Fail，就能转化为该结构体

#![allow(dead_code)]

use failure::format_err;
use failure::Error;
use failure::Fail;

#[derive(Debug, Fail)]
enum ToolchainError {
    #[fail(display = "invalid toolchain name: {}", _0)]
    InvalidToolchainName(String),
}

fn main() {
    use_failure_fail();
}

fn use_failure_fail() {
    let c = get_chain();
    if let Err(e) = c {
        println!("{}", e)
    }
}

fn get_chain() -> Result<String, ToolchainError> {
    return Err(ToolchainError::InvalidToolchainName("cathy".to_owned()));
}

/// 使用Error
/// 当一个函数中会返回多种错误时可以使用这一模式，其具有以下特点：
/// - 开始时不需要自定义类型
/// - 实现了Fail trait的类型只要使用 ？操作符就可以变为Error类型
/// - 当你引入新的依赖和新的错误类型时，你可以直接抛出它们 要使用该模式，只要把返回值设定为Result<_, Error>
fn use_failure_error() {
    let t = get_token("123");
    match t {
        Ok(t) => {
            println!("t = {}", t)
        }
        Err(e) => {
            println!("{}", e)
        }
    }
}

fn get_token(val: &str) -> Result<&str, Error> {
    if val == "12345" {
        Ok(val)
    } else {
        // 1.format_err宏可以生成一个Error
        // Err(format_err!("{} is below", val));

        // 2.将Fail转换成Error
        // Err(Error::from(ToolchainError::InvalidToolchainName(
        //     "cathy".to_owned(),
        // )));

        // 3.实现了Fail trait的类型使用 ？操作符可以变为Error类型
        Err(ToolchainError::InvalidToolchainName("cathy".to_owned()))?
    }
}
