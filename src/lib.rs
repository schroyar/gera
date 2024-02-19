use ethers::utils::keccak256;

pub fn get_salt(
    wanted_prefix: u8,
    deployer_address: [u8; 20],
    contract_bytecode: &[u8],
) -> anyhow::Result<[u8; 32]> {
    let mut salt = [0u8; 32];

    // Check if the address starts with the wanted prefix
    loop {
        // Turn salt into random bytes
        salt = rand::random();

        let current_address = get_address(deployer_address, salt, contract_bytecode).unwrap();

        if current_address.starts_with(&wanted_prefix.to_be_bytes()) {
            // Show data found
            println!("Address: {:?}", hex::encode(current_address));

            println!("Salt: {:?}", hex::encode(salt));

            break;
        }
    }

    Ok(salt)
}

fn get_address(
    deployer_address: [u8; 20],
    salt: [u8; 32],
    contract_bytecode: &[u8],
) -> anyhow::Result<[u8; 20]> {
    let mut ans: [u8; 85] = [0; 85];

    let hashed_bytecode = keccak256(contract_bytecode);

    ans[0] = 0xFF;
    ans[1..21].copy_from_slice(&deployer_address);
    ans[21..53].copy_from_slice(&salt);
    ans[53..85].copy_from_slice(&hashed_bytecode);

    let mut hashed_buf = [0u8; 32];

    // Now we hash buf
    hashed_buf = keccak256(&ans);

    let mut final_address = [0u8; 20];
    final_address.copy_from_slice(&hashed_buf[12..32]);

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
