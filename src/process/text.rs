use crate::{get_reader, TextSignFormat};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::rngs::OsRng;
use ed25519_dalek::{SigningKey, VerifyingKey, Verifier, Signature, Signer};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use crate::process_genpass;
trait TextSign {
    // &dyn Read 动态分派
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}
trait TextVerify {
    // R 静态分派
    fn verify<R: Read>(&self, reader: R, sig: &[u8]) -> anyhow::Result<bool>;
}
trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self>
    where
        Self: Sized; // 约束 Self 必须是一个固定长度的类型
}

trait KeyGenerator {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>>;
}
struct Blake3 {
    key: [u8; 32],
}
struct Ed25519Signer {
    key: SigningKey,
}
struct Ed25519Verifier {
    key: VerifyingKey,
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<String> {
    let mut reader = get_reader(&input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
    };
    let sign = URL_SAFE_NO_PAD.encode(signed);
    println!("{}", sign);
    Ok(sign)
}

pub fn process_text_verify(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<bool> {
    let mut reader = get_reader(&input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();
    let signed = URL_SAFE_NO_PAD.decode(buf)?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut reader, &signed)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(&mut reader, &signed)?
        }
    };
    println!("{}", verified);
    Ok(verified)
}

pub fn process_text_generate(format: &TextSignFormat) -> anyhow::Result<Vec<Vec<u8>>> {
    let key = match format {
        TextSignFormat::Blake3 => {
            let key = Blake3::generate()?;
            key
        }
        TextSignFormat::Ed25519 => {
            let key = Ed25519Signer::generate()?;
            key
        }
    };
    Ok(key)
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify<R: Read>(&self, mut reader: R, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        Ok(hash == sig)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let key = process_genpass(32, false, false, false, false)?;
        Ok(vec![key.as_bytes().to_vec()])
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}
impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Blake3::new(key);
        Ok(signer)
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let sk: SigningKey = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key().to_bytes().to_vec();
        let sk = sk.to_bytes().to_vec();
        Ok(vec![sk, pk])
    }
}
impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec())
    }
}
impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}
impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = &key[..32];
        let key = SigningKey::from_bytes(key.try_into()?);
        let signer = Ed25519Signer::new(key);
        Ok(signer)
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify<R: Read>(&self, mut reader: R, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        let ret = self.key.verify(&buf, &sig).is_ok();
        Ok(ret)
    }
}
impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}
impl Ed25519Verifier {
    fn new(key: VerifyingKey) -> Self {
        Self { key }
    }
    fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        let verifier = Ed25519Verifier::new(key);
        Ok(verifier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_blake3_sign_verify() -> anyhow::Result<()> {
        let black3 = Blake3::load("fixtures/blake3.txt")?;

        let data = b"hello world";
        let sig = black3.sign(&mut &data[..])?;
        println!("sig: {:?}", URL_SAFE_NO_PAD.encode(&sig));
        assert!(black3.verify(&mut &data[..], &sig).unwrap());
        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> anyhow::Result<()> {
        let signer = Ed25519Signer::load("fixtures/ed25519.key")?;
        let verifier = Ed25519Verifier::load("fixtures/ed25519.pub")?;

        let data = b"hello world";
        let sig = signer.sign(&mut &data[..])?;
        println!("sig: {:?}", URL_SAFE_NO_PAD.encode(&sig));
        assert!(verifier.verify(&mut &data[..], &sig).unwrap());
        Ok(())
    }
}
