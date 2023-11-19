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

    impl EthereumAddress for Address {
        fn convert_address(&self) -> Result<Address, &'static str> {
            Ok(*self)
        }
    }

    fn get_ethereum_data<T: EthereumAddress>(address: T) -> Address {
        let converted_address: Address = address.convert_address().unwrap();

        // do something else...

        converted_address
    }

    #[test]
   
}
