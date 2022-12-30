use pyo3::prelude::*;
use snarkvm_console::{account::{Address, PrivateKey, ViewKey}, network::Testnet3};

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

/// A Python module implemented in Rust
#[pymodule]
fn aleo_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(new_keys, m)?)?;
    Ok(())
}