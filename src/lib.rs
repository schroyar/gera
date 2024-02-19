use ethers::utils::keccak256;
use std::time;
use colored::Colorize;

pub fn get_salt(
    wanted_prefix: u8,
    deployer_address: [u8; 20],
    contract_bytecode: &[u8],
) -> anyhow::Result<[u8; 32]> {
    let start = time::Instant::now();

    loop {
        let salt: [u8; 32] = rand::random();

        match get_address(deployer_address, salt, contract_bytecode) {
            Ok(current_address) => {
                if current_address.starts_with(&[wanted_prefix]) {
                    println!(
                        "\nAddress: {:?}",
                        format!("0x{}", hex::encode(current_address))
                    );

                    println!("Salt that gives us wanted prefix: {:?}", hex::encode(salt));


                    let duration = start.elapsed();
                    println!("\n{} {:?}\n","Time to find salt was:".bold().green(), duration);

                    return Ok(salt);
                }
            }
            Err(e) => return Err(e),
        }
    }
}

fn get_address(
    deployer_address: [u8; 20],
    salt: [u8; 32],
    contract_bytecode: &[u8],
) -> anyhow::Result<[u8; 20]> {
    // 1 byte for 0xff (according to the EIP), 20 for address, 32 for salt, 32 for hashed bytecode
    let mut ans = [0u8; 1 + 20 + 32 + 32];

    let hashed_bytecode = keccak256(contract_bytecode);

    ans[0] = 0xFF;
    ans[1..21].copy_from_slice(&deployer_address);
    ans[21..53].copy_from_slice(&salt);
    ans[53..85].copy_from_slice(&hashed_bytecode);

    let hashed_buf = keccak256(&ans);

    let final_address: [u8; 20] = hashed_buf[12..32]
        .try_into()
        .expect("failed to fit address into array");

    Ok(final_address)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_salt() {
        let deployer_address: [u8; 20] = hex::decode("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045")
            .unwrap()
            .try_into()
            .unwrap();
        let contract_bytecode = b"dummy";
        let wanted_prefix: &[u8; 2] = &[0b1111, 0b1111]; // binary: 1111 -> hex: 0xF -> int: 15
        let combined = (wanted_prefix[0] << 4) | wanted_prefix[1];

        match get_salt(combined, deployer_address, contract_bytecode) {
            Ok(_) => {}
            Err(e) => panic!("Fail salt: {}", e),
        }
    }
}
