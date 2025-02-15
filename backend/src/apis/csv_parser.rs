use std::{error::Error, fs::File, path::Path};

use ethers::{
    abi::AbiEncode,
    types::{Bytes, U256},
};
use serde::{Deserialize, Serialize};

use crate::contracts::generated::summa_contract::{AddressOwnershipProof, Asset};

#[derive(Debug, Deserialize, Serialize)]
pub struct SignatureRecord {
    chain: String,
    address: String,
    signature: String,
    message: String,
}

impl SignatureRecord {
    pub fn new(chain: String, address: String, signature: String, message: String) -> Self {
        Self {
            chain,
            address,
            signature,
            message,
        }
    }
}

pub fn parse_signature_csv<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<AddressOwnershipProof>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_reader(file);

    let mut address_ownership_proofs = Vec::<AddressOwnershipProof>::new();

    for result in rdr.deserialize() {
        let record: SignatureRecord = result?;

        address_ownership_proofs.push(AddressOwnershipProof {
            cex_address: record.address.to_string(),
            chain: record.chain.to_string(),
            signature: record.signature.parse()?,
            message: Bytes::from(record.message.encode()),
        });
    }

    Ok(address_ownership_proofs)
}

#[derive(Debug, Deserialize)]
struct AssetRecord {
    chain: String,
    asset_name: String,
    amount: String,
}

pub fn parse_asset_csv<P: AsRef<Path>, const N_ASSETS: usize>(
    path: P,
) -> Result<[Asset; N_ASSETS], Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_reader(file);

    let mut assets_vec = Vec::with_capacity(N_ASSETS);

    for result in rdr.deserialize() {
        let record: AssetRecord = result?;

        assets_vec.push(Asset {
            asset_name: record.asset_name,
            chain: record.chain,
            amount: U256::from_dec_str(&record.amount)?,
        });
    }

    let assets_array: [Asset; N_ASSETS] = assets_vec.try_into().map_err(|v: Vec<Asset>| {
        format!(
            "The number of assets in CSV file does not match the expected count {:?}",
            v
        )
    })?;

    Ok(assets_array)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv_to_signature() {
        let path = "src/apis/csv/signatures.csv";
        let address_ownership = parse_signature_csv(path).unwrap();

        let first_address_ownership = AddressOwnershipProof {
            chain: "ETH".to_string(),
            cex_address: "0x70997970C51812dc3A010C7d01b50e0d17dc79C8".to_string(),
            signature:
              ("0x089b32327d332c295dc3b8873c205b72153211de6dc1c51235782b091cefb9d06d6df2661b86a7d441cd322f125b84901486b150e684221a7b7636eb8182af551b").parse().unwrap(),
              message:  "Summa proof of solvency for CryptoExchange".encode().into(),
          };

        assert_eq!(address_ownership[0], first_address_ownership);
    }

    #[test]
    fn test_parse_csv_to_assets() {
        let path = "src/apis/csv/assets.csv";
        let assets = parse_asset_csv::<&str, 2>(path).unwrap();

        assert_eq!(
            assets[0],
            Asset {
                chain: "ETH".to_string(),
                asset_name: "ETH".to_string(),
                amount: U256::from(556863),
            }
        );
        assert_eq!(
            assets[1],
            Asset {
                chain: "ETH".to_string(),
                asset_name: "USDT".to_string(),
                amount: U256::from(556863),
            }
        );
    }
}
