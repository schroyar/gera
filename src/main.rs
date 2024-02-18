use clap::{Arg, Command};
use v4_create2::calc_address;

fn main() -> anyhow::Result<()> {
    let matches = Command::new("Prefix")
        .arg(
            Arg::new("initial_bits")
                .help("Example: 01001")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("deployer_address")
                .help("Example: ")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("contract_bytecode")
                .help("Example: ")
                .required(true)
                .index(3),
        )
        .get_matches();

    let bits = matches
        .get_one::<String>("initial_bits")
        .expect("Failed: initial bits");

    let deployer = matches
        .get_one::<[u8; 20]>("deployer_address")
        .expect("Failed: deployer address");

    let contract_bytecode = matches
        .get_one::<Vec<u8>>("contract_bytecode")
        .expect("Failed: contract bytecode");

    let into_int = u8::from_str_radix(&bits, 2)?;

    let wanted_prefix = hex::encode(&into_int.to_be_bytes());

    let ans = calc_address(wanted_prefix, *deployer, &contract_bytecode);

    match ans {
        Ok(salt) => println!("Salt: {:?}", salt.to_vec()),
        Err(e) => println!("Failed: {}", e),
    }

    Ok(())
}
