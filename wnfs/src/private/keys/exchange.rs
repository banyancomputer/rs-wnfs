use crate::error::RsaError;
use anyhow::anyhow;
use anyhow::Result;
use async_trait::async_trait;
use rsa::{
    pkcs8::{LineEnding, EncodePrivateKey, DecodePrivateKey},
    traits::PublicKeyParts,
    BigUint, Oaep
};
use spki::{EncodePublicKey, DecodePublicKey};
use sha2::Sha256;
use sha1::{Sha1, Digest};

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

pub const RSA_KEY_SIZE: usize = 3072;
pub const PUBLIC_KEY_EXPONENT: u64 = 65537;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// The `ExchangeKey` trait defines methods for creating an RSA public key from a modulus and encrypting data with the public key.
/// Implementations of this trait can create an RSA public key using the `from_modulus` method, which takes a modulus as input.
///
/// Data can be encrypted with the public key using the `encrypt` method, which takes a slice of bytes as input and returns the encrypted data as a vector of bytes.
///
/// More on exchange keys [here][key].
///
/// [key]: https://github.com/wnfs-wg/spec/blob/main/spec/shared-private-data.md#2-exchange-keys-partition
#[async_trait(?Send)]
pub trait ExchangeKey {
    /// Creates an RSA public key from the public key modulus.
    ///
    /// The exponent is expected to be of the value [`PUBLIC_KEY_EXPONENT`](constant.PUBLIC_KEY_EXPONENT.html) constant.
    async fn from_modulus(modulus: &[u8]) -> Result<Self>
    where
        Self: Sized;

    /// Encrypts data with the public key.
    async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>>;
}

/// The `PrivateKey` trait represents a RSA private key type that can be used to decrypt data encrypted with corresponding public key.
#[async_trait(?Send)]
pub trait PrivateKey {
    /// Decrypts ciphertext with the private key.
    async fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>>;
}

pub type PublicKeyModulus = Vec<u8>;

// #[cfg(test)]
#[derive(Debug, Clone)]
pub struct RsaPublicKey(rsa::RsaPublicKey);

// #[cfg(test)]
#[derive(Debug, Clone)]
pub struct RsaPrivateKey(rsa::RsaPrivateKey);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

// #[cfg(test)]
impl RsaPublicKey {
    /// Gets the public key modulus.
    pub fn get_public_key_modulus(&self) -> Result<Vec<u8>> {
        Ok(self.0.n().to_bytes_le())
    }

    /// Get the sha1 fingerprint from the DER bytes of the public key.
    pub fn get_sha1_fingerprint(&self) -> Result<Vec<u8>> {
        let doc = self.0.to_public_key_der()?;
        let der_bytes = doc.as_bytes();
        let mut hasher = Sha1::new();
        hasher.update(&der_bytes);
        Ok(hasher.finalize().to_vec())
    }

    /// Writes the public key to a SPKI PEM file.
    /// # Arguments
    /// path - The path to the file to write to.
    pub fn to_pem_file(&self, path: impl AsRef<std::path::Path>) -> Result<()> {
        self.0
            .write_public_key_pem_file(path, LineEnding::LF)
            .map_err(|e| anyhow!(RsaError::ExportToPemFileFailed(anyhow!(e))))
    }

    /// Reads the public key from a SPKI PEM file.
    /// # Arguments
    /// path - The path to the file to read from.
    pub fn from_pem_file(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let key = rsa::RsaPublicKey::read_public_key_pem_file(path)?;
        Ok(Self(key))
    }

    /// Read the public key from DER bytes.
    /// # Arguments
    /// bytes - The DER bytes to read from.
    pub fn from_der(bytes: &[u8]) -> Result<Self> {
        let key = rsa::RsaPublicKey::from_public_key_der(bytes)?;
        Ok(Self(key))
    }
}

// #[cfg(test)]
impl RsaPrivateKey {
    /// Constructs a new 2048-bit RSA private key.
    pub fn new() -> Result<Self> {
        Ok(Self(rsa::RsaPrivateKey::new(
            &mut rand_core::OsRng,
            RSA_KEY_SIZE,
        )?))
    }

    /// Writes the private key to a PKCS#8 PEM file.
    /// 
    /// # Arguments
    /// path - The path to the file to write to.
    pub fn to_pem_file(&self, path: impl AsRef<std::path::Path>) -> Result<()> {
        self.0
            .write_pkcs8_pem_file(path, LineEnding::LF)
            .map_err(|e| anyhow!(RsaError::ExportToPemFileFailed(anyhow!(e))))
    }

    /// Reads the private key from a PKCS#8 PEM file.
    /// 
    /// # Arguments
    /// path - The path to the file to read from.
    pub fn from_pem_file(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let key = rsa::RsaPrivateKey::read_pkcs8_pem_file(path)?;
        Ok(Self(key))
    }

    /// Reads the private key from DER bytes.
    /// 
    /// # Arguments
    /// bytes - The DER bytes to read from.
    pub fn from_der(bytes: &[u8]) -> Result<Self> {
        let key = rsa::RsaPrivateKey::from_pkcs8_der(bytes)?;
        Ok(Self(key))
    }

    /// Gets the public key.
    pub fn get_public_key(&self) -> RsaPublicKey {
        RsaPublicKey(self.0.to_public_key())
    }
}

// #[cfg(test)]
#[async_trait(?Send)]
impl ExchangeKey for RsaPublicKey {
    async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let padding = Oaep::new::<Sha256>();
        self.0
            .encrypt(&mut rand_core::OsRng, padding, data)
            .map_err(|e| anyhow!(RsaError::EncryptionFailed(anyhow!(e))))
    }

    async fn from_modulus(modulus: &[u8]) -> Result<Self> {
        let n = BigUint::from_bytes_le(modulus);
        let e = BigUint::from(PUBLIC_KEY_EXPONENT);

        Ok(Self(
            rsa::RsaPublicKey::new(n, e).map_err(|e| RsaError::InvalidPublicKey(anyhow!(e)))?,
        ))
    }
}

// #[cfg(test)]
#[async_trait(?Send)]
impl PrivateKey for RsaPrivateKey {
    async fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        let padding = Oaep::new::<Sha256>();
        self.0
            .decrypt(padding, ciphertext)
            .map_err(|e| anyhow!(RsaError::DecryptionFailed(anyhow!(e))))
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;
    use base64::{Engine as _, engine::general_purpose};
    use hex_literal::hex;

    #[async_std::test]
    #[ignore]
    async fn test_rsa_key_pair() {
        let priv_key = RsaPrivateKey::new().unwrap();
        let pub_key = priv_key.get_public_key();

        let plaintext = b"Hello, world!";
        let ciphertext = pub_key.encrypt(plaintext).await.unwrap();
        let decrypted = priv_key.decrypt(&ciphertext).await.unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }

    #[async_std::test]
    async fn test_rsa_priv_key_from_pem_file() {
        let priv_key = RsaPrivateKey::new().unwrap();
        let pub_key = priv_key.get_public_key();
        let plaintext = b"Hello, world!";
        let path = "private_key.pem";
        
        priv_key.to_pem_file(path).unwrap();
        let priv_key_from_file = RsaPrivateKey::from_pem_file(path).unwrap();

        // Remove the file containing the private key
        std::fs::remove_file(path).unwrap();

        let ciphertext_from_file = pub_key.encrypt(plaintext).await.unwrap();
        let decrypted_from_file = priv_key_from_file.decrypt(&ciphertext_from_file).await.unwrap();

        assert_eq!(plaintext, &decrypted_from_file[..]);
    }

    #[async_std::test]
    async fn test_rsa_pub_key_from_pem_file() {
        let priv_key = RsaPrivateKey::new().unwrap();
        let pub_key = priv_key.get_public_key();
        let plaintext = b"Hello, world!";
        let path = "public_key.pem";
        
        pub_key.to_pem_file(path).unwrap();
        let pub_key_from_file = RsaPublicKey::from_pem_file(path).unwrap();

        // Remove the file containing the private key
        std::fs::remove_file(path).unwrap();

        let ciphertext_from_file = pub_key_from_file.encrypt(plaintext).await.unwrap();
        let decrypted_from_file = priv_key.decrypt(&ciphertext_from_file).await.unwrap();

        assert_eq!(plaintext, &decrypted_from_file[..]);
    }

    #[async_std::test]
    async fn test_rsa_key_pair_from_base64_strings() {
        const SPKI_STRING: &str = "MIIBojANBgkqhkiG9w0BAQEFAAOCAY8AMIIBigKCAYEApgs5TkpXDqjye2KoU1ERu8QRs8lHkJb/YULlnPR3JuAUfdpj6TwifLZTFF3Duh5CRUXEa0p37EzRaA3rXCfBSldD4sm1uZ8xpc+wlNT0ZufRHY2PaFreXECDo1HtFMsaB6eGKF2KY3RhYlqUrmUYomm3M/G8qBG1TnvICZJxFuCpzE7Wrh3Bxw5BRzuclaatpa3bnJ/6NDmBqFsZvanlrKKoSdKsa/t274UXoWuAFtjRumbJYnu7o3QkVwFjCREXd2oDVu9EnrqRHr11zE9KH8wh2qk0dbliPXvB9BlwBZHLhWd7bhCtdhf8T+tWVfprkM74h91SRfZTLa66B4PUcphte4gw4hCaboZIedLG0En45shMl3/rYh+YEYoJJ18qBziFUMq+CrWzTPuvdMyWBrbimy8TEkzR83UXwpncPkDh1qJJHyw6PGhhXyiYPtNwXnrkr5Bl1NRs3rfbi7Rk4mbTZJ92LFtbDNAoZnZXNmrq+ZQZ/lLJUqd1G2xt1yaFAgMBAAE=";
        const PKCS8_STRING: &str = "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";
        let plaintext = b"Hello, world!";
        let spki_bytes = general_purpose::STANDARD.decode(SPKI_STRING).unwrap();
        let pkcs8_bytes = general_purpose::STANDARD.decode(PKCS8_STRING).unwrap();

        let pub_key = RsaPublicKey::from_der(&spki_bytes).unwrap();
        let priv_key = RsaPrivateKey::from_der(&pkcs8_bytes).unwrap();
        let pub_key_from_priv_key = priv_key.get_public_key();

        assert!(pub_key.0.n() == pub_key_from_priv_key.0.n());

        let ciphertext = pub_key.encrypt(plaintext).await.unwrap();
        let decrypted = priv_key.decrypt(&ciphertext).await.unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }

    #[test]
    fn test_rsa_pub_key_fingerprint() {
        const SPKI_STRING: &str = "MIIBojANBgkqhkiG9w0BAQEFAAOCAY8AMIIBigKCAYEApgs5TkpXDqjye2KoU1ERu8QRs8lHkJb/YULlnPR3JuAUfdpj6TwifLZTFF3Duh5CRUXEa0p37EzRaA3rXCfBSldD4sm1uZ8xpc+wlNT0ZufRHY2PaFreXECDo1HtFMsaB6eGKF2KY3RhYlqUrmUYomm3M/G8qBG1TnvICZJxFuCpzE7Wrh3Bxw5BRzuclaatpa3bnJ/6NDmBqFsZvanlrKKoSdKsa/t274UXoWuAFtjRumbJYnu7o3QkVwFjCREXd2oDVu9EnrqRHr11zE9KH8wh2qk0dbliPXvB9BlwBZHLhWd7bhCtdhf8T+tWVfprkM74h91SRfZTLa66B4PUcphte4gw4hCaboZIedLG0En45shMl3/rYh+YEYoJJ18qBziFUMq+CrWzTPuvdMyWBrbimy8TEkzR83UXwpncPkDh1qJJHyw6PGhhXyiYPtNwXnrkr5Bl1NRs3rfbi7Rk4mbTZJ92LFtbDNAoZnZXNmrq+ZQZ/lLJUqd1G2xt1yaFAgMBAAE=";
        let spki_bytes = general_purpose::STANDARD.decode(SPKI_STRING).unwrap();
        let pub_key = RsaPublicKey::from_der(&spki_bytes).unwrap();
        let fingerprint = pub_key.get_sha1_fingerprint().unwrap();
        assert_eq!(fingerprint, hex!("d2b0c3e8873d95b95fe9195952eb016b9d5e5125"));
    }

    #[async_std::test]
    #[ignore]
    async fn test_rsa_key_pair_from_public_key_modulus() {
        let priv_key = RsaPrivateKey::new().unwrap();
        let pub_key = priv_key.get_public_key();

        let public_key_modulus = pub_key.get_public_key_modulus().unwrap();
        let key_pair_from_modulus = RsaPublicKey::from_modulus(&public_key_modulus)
            .await
            .unwrap();

        let plaintext = b"Hello, world!";
        let ciphertext = key_pair_from_modulus.encrypt(plaintext).await.unwrap();
        let decrypted = priv_key.decrypt(&ciphertext).await.unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }
}
