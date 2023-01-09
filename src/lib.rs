use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
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
  let private_data = PrivateKey::<Testnet3>::from_str(priv_key);

  let private_key = match private_data {
    Err(_) => return Err(PyValueError::new_err("Failed to validate private key")),
    Ok(p_key) => p_key,
  };

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
  let signature_data = Signature::<Testnet3>::from_str(&signature);

  let sign_to_verify = match signature_data {
    Err(_) => return Err(PyValueError::new_err("Incorrect signature")),
    Ok(signature) => signature,
  };

  // get Address from string
  let address_data = Address::<Testnet3>::from_str(&address_key);

  let address_to_verify = match address_data {
    Err(_) => return Err(PyValueError::new_err("Incorrect public address")),
    Ok(address) => address,
  };  

  // verify signature
  let result_data = sign_to_verify.verify_bytes(&address_to_verify, message.as_bytes());

  let result = match result_data {
    false => false,
    true => true,
  };  

  Ok(result.to_string())
}


/// A Python module implemented in Rust
#[pymodule]
fn aleo_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(new_keys, m)?)?;
    m.add_function(wrap_pyfunction!(sign_message, m)?)?;
    m.add_function(wrap_pyfunction!(verify_message, m)?)?;
    Ok(())
}