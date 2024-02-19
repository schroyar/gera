# Uniswap V4 Hooks Salt Generator (`gera`)

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](https://github.com/schroyar/gera)

## Basic usage

```
let deployer_address: [u8; 20] = hex::decode("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").unwrap().try_into().unwrap();
let contract_bytecode = b"your_contract_bytecode_here";
let wanted_prefix = 0xF; // Desired prefix for the address

match get_salt(wanted_prefix, deployer_address, contract_bytecode) {
    Ok(salt) => println!("Generated Salt: {:?}", salt),
    Err(e) => eprintln!("Error generating salt: {}", e),
}
```
