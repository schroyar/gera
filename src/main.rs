use clap::{Arg, Command};
use v4_create2::get_salt;

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
        .get_one::<String>("deployer_address")
        .expect("Failed: deployer address");

    let contract_bytecode = matches
        .get_one::<String>("contract_bytecode")
        .expect("Failed: contract bytecode");

    let into_int = u8::from_str_radix(&bits, 2)?;

    let wanted_prefix = hex::encode(&into_int.to_be_bytes());

    let in_20: [u8; 20] = hex::decode(deployer.clone()).unwrap().try_into().unwrap();

    dbg!(&in_20);

    let ans = get_salt(
        wanted_prefix.as_bytes(),
        in_20,
        &contract_bytecode.as_bytes(),
    );

    match ans {
        Ok(salt) => println!("Salt: {:?}", salt.to_vec()),
        Err(e) => println!("Failed: {}", e),
    }

    Ok(())
}
