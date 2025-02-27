use jsonwebtoken::{encode, EncodingKey, Header, decode, DecodingKey, Validation};
use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize, Deserialize)]
struct JwtPayload {
    name: String,
    age: u8,
    exp: usize,
}

pub async fn process_jwt_encode(name: String, age: u8) -> anyhow::Result<()> {
    let header = Header::default();
    let key = EncodingKey::from_secret("secret".as_ref());
    let payload = JwtPayload { name, age, exp: 1000000000000000000 };
    let token = encode(&header, &payload, &key)?;
    println!("{}", token);
    Ok(())
}

pub async fn process_jwt_decode(jwt: String) -> anyhow::Result<()> {
    let key = DecodingKey::from_secret("secret".as_ref());
    let data = decode::<JwtPayload>(&jwt, &key, &Validation::default())?;
    println!("{:?}", data);
    Ok(())
}

