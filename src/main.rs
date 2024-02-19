use clap::{Arg, Command};
use v4_create2::get_salt;

fn main() -> anyhow::Result<()> {
    // let matches = Command::new("Prefix")
    //     .arg(
    //         Arg::new("initial_bits")
    //             .help("16 bits of the address prefix. Example: 11111111")
    //             .required(true)
    //             .index(1),
    //     )
    //     .arg(
    //         Arg::new("deployer_address")
    //             .help("Example: ")
    //             .required(true)
    //             .index(2),
    //     )
    //     .arg(
    //         Arg::new("contract_bytecode")
    //             .help("Example: ")
    //             .required(true)
    //             .index(3),
    //     )
    //     .get_matches();

    // let bits = matches
    //     .get_one::<u8>("initial_bits")
    //     .expect("Failed: initial bits");

    // let deployer = matches
    //     .get_one::<String>("deployer_address")
    //     .expect("Failed: deployer address");

    // let contract_bytecode = matches
    //     .get_one::<String>("contract_bytecode")
    //     .expect("Failed: contract bytecode");

    // let in_20: [u8; 20] = hex::decode(deployer.clone()).unwrap().try_into().unwrap();

    // let ans = get_salt(*bits, in_20, &contract_bytecode.as_bytes());

    // match ans {
    //     Ok(salt) => println!("Salt: {:?}", salt.to_vec()),
    //     Err(e) => println!("Failed: {}", e),
    // }

    Ok(())
}
