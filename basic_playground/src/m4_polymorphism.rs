#[cfg(test)]
mod test {
    use ethers::types::Address;
    use std::str::FromStr;

    trait EthereumAddress {
        fn convert_address(&self) -> Result<Address, &'static str>;
    }

    // Call the implementation of Ethereum Address for a string reference type
    impl EthereumAddress for &str {
        fn convert_address(&self) -> Result<Address, &'static str> {
            match Address::from_str(self) {
                Ok(address) => Ok(address),
                Err(_) => Err("Invalid Ethereum Address String"),
            }
        }
    }

   
    #[test]
   
}
