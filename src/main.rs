use clap::Parser;
use futures::future;

use hex::FromHex;

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use crash::utils::product_repeat;

use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    hash: String,

    #[arg(short, long, default_value_t = String::from("md5"))]
    algo: String,

    #[arg(long, default_value_t = 6)]
    min_len: u16,

    #[arg(long, default_value_t = 12)]
    max_len: u16,

    #[arg(short, long, default_value_t = String::from("0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&\\'()*+,-./:;<=>?@[\\]^_`{|}~"))]
    dictionary: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let args = Args::parse();

    let supported_algos: Vec<String> = vec![
        "md5".to_string()
    ];

    if !supported_algos.contains(&args.algo) {
        error!("{} is not supported", &args.algo);
        error!("supported algorithms: {}", supported_algos.join(" "));
        process::exit(1);
    }

    if args.min_len > args.max_len {
        error!("min length of hashed text is bigger than max length of it");
        process::exit(1);
    }

    let hash = match Vec::<u8>::from_hex(&args.hash) {
        Ok(h) => h,
        Err(_) => {
            error!("invalid hash string");
            process::exit(1);
        }
    };

    let mut handles = Vec::new();

    for len in args.min_len..args.max_len + 1 {
        let h = hash.clone();
        let algo = args.algo.clone();
        let dict = args.dictionary.clone();

        handles.push(tokio::spawn(async move {
            bruteforce(h, algo, dict, len).await;
        }));
    }
    
    future::join_all(handles).await;

    Ok(())
}

async fn bruteforce(hash: Vec<u8>, algo: String, dictionary: String, length: u16) {
    let mut handles = Vec::new();

    for prod in product_repeat(dictionary.chars(), length.into()) {
        let prod: String = prod.into_iter().collect();
        let h = hash.clone();
        let a = algo.clone();

        handles.push(tokio::spawn(async move {
            compute(prod, h, a).await;
        }));
    }

    future::join_all(handles).await;
}

async fn compute(variant: String, hash: Vec<u8>, algo: String) {
    let mut result = Vec::<u8>::new();

    if algo == "md5" {
        result = md5::compute(variant.as_bytes()).to_vec();
    }

    if hash == result {
        println!("hash source found: {}", variant);
        process::exit(1);
    }
}
