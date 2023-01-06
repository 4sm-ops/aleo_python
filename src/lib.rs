use pyo3::prelude::*;
use snarkvm_console::{account::{Address, PrivateKey, ViewKey, Signature}, network::Testnet3};

// use snarkvm::prelude::test_crypto_rng;

use snarkvm_utilities::TestRng;

use std::str::FromStr;

#[macro_use]
extern crate json;

/// Create new Aloe keys
#[pyfunction]
fn new_keys() -> PyResult<String> {

  let private_key = PrivateKey::<Testnet3>::new(&mut rand::thread_rng()).unwrap();

  let view_key = ViewKey::try_from(&private_key).unwrap();
  let address = Address::try_from(&private_key).unwrap();

  // println!("{}", private_key.to_string());
  // println!("{}", address.to_string());
  // println!("{}", view_key.to_string());

  let output_aleo_keys_json = object!{
    "AleoPrivateKey": private_key.to_string(),
    "AleoViewKey": view_key.to_string(),
    "AleoAddress": address.to_string()
  };

  Ok(output_aleo_keys_json.to_string())
}

#[pyfunction]
fn sign_message(priv_key: &str, message: &str) -> PyResult<String> {

  let rng = &mut TestRng::default();
  // let rng = &mut test_crypto_rng();

  // Get Private Key from priv_key string
  let private_key = PrivateKey::<Testnet3>::from_str(priv_key).unwrap();

  // Get Address from Private Key
  let address = Address::try_from(&private_key).unwrap();

  // Sign message under Private Kye
  let result = private_key.sign_bytes(message.as_bytes(), rng);
  assert!(result.is_ok(), "Failed to generate a signature");

  // Get Signature
  let signature = result.unwrap();

  let output_signature_json = object!{
    "Message to Sign": message,
    "Aleo Address": address.to_string(),
    "Signature": signature.to_string()
  };

  Ok(output_signature_json.to_string())

}

#[pyfunction]
fn verify_message(address_key: &str, message: &str, signature: &str) -> PyResult<String> {

  // get Signature from string
  let sign_to_verify = Signature::<Testnet3>::from_str(&signature).unwrap();

  // get Address from string
  let address_to_verify = Address::<Testnet3>::from_str(&address_key).unwrap();

  // verify signature
  let result = sign_to_verify.verify_bytes(&address_to_verify, message.as_bytes());
  assert!(result, "Failed to execute signature verification");

  // return verification result
  Ok(result.to_string())
  // Ok("123".to_string())
}


/// A Python module implemented in Rust
#[pymodule]
fn aleo_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(new_keys, m)?)?;
    m.add_function(wrap_pyfunction!(sign_message, m)?)?;
    m.add_function(wrap_pyfunction!(verify_message, m)?)?;
    Ok(())
}