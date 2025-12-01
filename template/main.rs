use __DAY__::prelude::*;

fn main() -> Result<(), String> {
  let data = extract().map_err(|e| e.to_string())?;
  let result = transform(data).map_err(|e| e.to_string())?;
  load(Ok(result)).map_err(|e| e.to_string())
}
