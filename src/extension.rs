use crate::{error::ErrorCodes, storage::StoragePool, Value};
use lazy_static::lazy_static;
use log::error;
use std::sync::RwLock;

lazy_static! {
    static ref STORAGE_POOL: RwLock<StoragePool> = RwLock::new(StoragePool::new("."));
}

#[derive(PartialEq)]
enum Function {
    ErrorCodes,
    Open,
    Close,
    Read,
    Write,
    Get,
    Set,
    Erase,
    Exists,
    GetFiles,
}

pub fn ext(function: &str) -> (ErrorCodes, Value) {
    let function = match function {
        "errorCodes" => Function::ErrorCodes,
        _ => {
            return (
                ErrorCodes::UnknownFunction,
                Value::String(function.to_owned()),
            )
        }
    };

    match function {
        Function::ErrorCodes => (
            ErrorCodes::Ok,
            Value::Array(vec![
                Value::Array(vec![Value::Number(0.), Value::String("No Error".into())]),
                Value::Array(vec![
                    Value::Number(2.),
                    Value::String("The function you passed is unknown".into()),
                ]),
                Value::Array(vec![
                    Value::Number(10.),
                    Value::String("Missing required argument".into()),
                ]),
                Value::Array(vec![
                    Value::Number(11.),
                    Value::String("Argument is empty".into()),
                ]),
                Value::Array(vec![
                    Value::Number(20.),
                    Value::String("A storage error occured".into()),
                ]),
            ]),
        ),
        _ => unreachable!(),
    }
}

/// Execute a function with arguments
///
pub fn ext_args(function: &str, args: Vec<&str>) -> (ErrorCodes, Value) {
    if args.is_empty() {
        return (
            ErrorCodes::MissingArgument,
            Value::String("function".into()),
        );
    }

    if function.is_empty() {
        ext_args_alt(function, args)
    } else {
        ext_args_std(function, args)
    }
}

pub fn ext_args_std(function_name: &str, args: Vec<&str>) -> (ErrorCodes, Value) {
    let function = match function_name {
        "open" => Function::Open,
        "close" => Function::Close,
        "read" => Function::Read,
        "write" => Function::Write,
        "get" => Function::Get,
        "set" => Function::Set,
        "erase" | "eraseKey" => Function::Erase,
        "exists" => Function::Exists,
        "getFiles" | "storages" => Function::GetFiles,
        _ => {
            return (
                ErrorCodes::UnknownFunction,
                Value::String(function_name.into()),
            )
        }
    };

    (ErrorCodes::Ok, Value::Void)
}

pub fn ext_args_alt(data: &str, args: Vec<&str>) -> (ErrorCodes, Value) {
    let function_name = args[0].trim_matches('"');

    if function_name.is_empty() {
        return (ErrorCodes::EmptyArgument, Value::String("function".into()));
    }

    let function = match function_name {
        "open" => Function::Open,
        "close" => Function::Close,
        "read" => Function::Read,
        "write" => Function::Write,
        "get" => Function::Get,
        "set" => Function::Set,
        "erase" | "eraseKey" => Function::Erase,
        "exists" => Function::Exists,
        "getFiles" | "storages" => Function::GetFiles,
        _ => {
            return (
                ErrorCodes::UnknownFunction,
                Value::String(function_name.into()),
            )
        }
    };

    if args.len() < 2 {
        return (ErrorCodes::MissingArgument, Value::String("name".into()));
    }

    let name = args[1].trim_matches('"');

    if name.is_empty() {
        return (ErrorCodes::EmptyArgument, Value::String("name".into()));
    }

    match function {
        Function::Open => match STORAGE_POOL.write().unwrap().open(name) {
            Ok(_) => (ErrorCodes::Ok, Value::Void),
            Err(err) => {
                error!("Could not open storage: {:?}", err);
                (
                    ErrorCodes::StorageError,
                    Value::String(format!("Error: {:?}", err)),
                )
            }
        },
        Function::Close => match STORAGE_POOL.write().unwrap().close(name) {
            Ok(_) => (ErrorCodes::Ok, Value::Void),
            Err(err) => {
                error!("Could not close storage: {:?}", err);
                (
                    ErrorCodes::StorageError,
                    Value::String(format!("Error: {:?}", err)),
                )
            }
        },
        Function::Read => match STORAGE_POOL.write().unwrap().read(name) {
            Ok(_) => (ErrorCodes::Ok, Value::Void),
            Err(err) => {
                error!("Could not read storage: {:?}", err);
                (
                    ErrorCodes::StorageError,
                    Value::String(format!("Error: {:?}", err)),
                )
            }
        },
        Function::Write => match STORAGE_POOL.write().unwrap().write(name) {
            Ok(_) => (ErrorCodes::Ok, Value::Void),
            Err(err) => {
                error!("Could not write storage: {:?}", err);
                (
                    ErrorCodes::StorageError,
                    Value::String(format!("Error: {:?}", err)),
                )
            }
        },
        Function::Get => todo!(),
        Function::Set => todo!(),
        Function::Erase => todo!(),
        Function::Exists => todo!(),
        Function::GetFiles => todo!(),
        _ => unreachable!(),
    }
}
