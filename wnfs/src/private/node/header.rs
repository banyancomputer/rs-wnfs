use super::{SnapshotKey, TemporalKey};
use crate::private::RevisionRef;
use anyhow::Result;
use libipld::{Cid, Ipld, IpldCodec};
use rand_core::RngCore;
use serde::{Deserialize, Serialize};
use sha3::Sha3_256;
use skip_ratchet::Ratchet;
use std::{collections::BTreeMap, fmt::Debug};
use wnfs_common::{utils, BlockStore, HashOutput, HASH_BYTE_SIZE};
use wnfs_hamt::Hasher;
use wnfs_namefilter::Namefilter;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

pub type INumber = HashOutput;

/// This is the header of a private node. It contains secret information about the node which includes
/// the inumber, the ratchet, and the namefilter.
///
/// # Examples
///
/// ```
/// use wnfs::{
///     private::PrivateFile,
///     namefilter::Namefilter,
///     traits::Id
/// };
/// use chrono::Utc;
/// use rand::thread_rng;
///
/// let rng = &mut thread_rng();
/// let file = PrivateFile::new(
///     Namefilter::default(),
///     Utc::now(),
///     rng,
/// );
///
/// println!("Header: {:?}", file.header);
/// ```
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrivateNodeHeader {
    /// A unique identifier of the node.
    pub(crate) inumber: INumber,
    /// Used both for versioning and deriving keys for that enforces privacy.
    pub(crate) ratchet: Ratchet,
    /// Used for ancestry checks and as a key for the private forest.
    pub(crate) bare_name: Namefilter,
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl PrivateNodeHeader {
    /// Creates a new PrivateNodeHeader.
    pub(crate) fn new(parent_bare_name: Namefilter, rng: &mut impl RngCore) -> Self {
        let inumber = utils::get_random_bytes::<HASH_BYTE_SIZE>(rng);
        let ratchet_seed = utils::get_random_bytes::<HASH_BYTE_SIZE>(rng);

        Self {
            bare_name: {
                let mut namefilter = parent_bare_name;
                namefilter.add(&inumber);
                namefilter
            },
            ratchet: Ratchet::zero(ratchet_seed),
            inumber,
        }
    }

    /// Creates a new PrivateNodeHeader with provided seed.
    pub(crate) fn with_seed(
        parent_bare_name: Namefilter,
        ratchet_seed: HashOutput,
        inumber: HashOutput,
    ) -> Self {
        Self {
            bare_name: {
                let mut namefilter = parent_bare_name;
                namefilter.add(&inumber);
                namefilter
            },
            ratchet: Ratchet::zero(ratchet_seed),
            inumber,
        }
    }

    /// Advances the ratchet.
    pub(crate) fn advance_ratchet(&mut self) {
        self.ratchet.inc();
    }

    /// Updates the bare name of the node.
    pub(crate) fn update_bare_name(&mut self, parent_bare_name: Namefilter) {
        self.bare_name = {
            let mut namefilter = parent_bare_name;
            namefilter.add(&self.inumber);
            namefilter
        };
    }

    /// Resets the ratchet.
    pub(crate) fn reset_ratchet(&mut self, rng: &mut impl RngCore) {
        self.ratchet = Ratchet::zero(utils::get_random_bytes(rng))
    }

    /// Derives the revision ref of the current header.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use wnfs::{
    ///     private::PrivateFile,
    ///     namefilter::Namefilter,
    ///     traits::Id
    /// };
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let file = Rc::new(PrivateFile::new(
    ///     Namefilter::default(),
    ///     Utc::now(),
    ///     rng,
    /// ));
    /// let revision_ref = file.header.derive_revision_ref();
    ///
    /// println!("Private ref: {:?}", revision_ref);
    /// ```
    pub fn derive_revision_ref(&self) -> RevisionRef {
        let temporal_key = self.derive_temporal_key();
        let saturated_name_hash = self.get_saturated_name_hash();

        RevisionRef {
            saturated_name_hash,
            temporal_key,
        }
    }

    /// Returns the label used for identifying the revision in the PrivateForest.
    #[inline]
    pub fn get_saturated_name_hash(&self) -> HashOutput {
        Sha3_256::hash(&self.get_saturated_name())
    }

    /// Derives the temporal key.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use wnfs::{
    ///     private::PrivateFile,
    ///     namefilter::Namefilter,
    ///     traits::Id
    /// };
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let file = Rc::new(PrivateFile::new(
    ///     Namefilter::default(),
    ///     Utc::now(),
    ///     rng,
    /// ));
    /// let temporal_key = file.header.derive_temporal_key();
    ///
    /// println!("Temporal Key: {:?}", temporal_key);
    /// ```
    #[inline]
    pub fn derive_temporal_key(&self) -> TemporalKey {
        TemporalKey::from(&self.ratchet)
    }

    /// Gets the saturated namefilter for this node using the provided ratchet key.
    pub(crate) fn get_saturated_name_with_key(&self, temporal_key: &TemporalKey) -> Namefilter {
        let mut name = self.bare_name.clone();
        name.add(&temporal_key.0.as_bytes());
        name.saturate();
        name
    }

    /// Gets the saturated namefilter for this node.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::rc::Rc;
    /// use wnfs::{
    ///     private::{PrivateFile, AesKey},
    ///     namefilter::Namefilter
    /// };
    /// use chrono::Utc;
    /// use rand::thread_rng;
    ///
    /// let rng = &mut thread_rng();
    /// let file = Rc::new(PrivateFile::new(
    ///     Namefilter::default(),
    ///     Utc::now(),
    ///     rng,
    /// ));
    /// let saturated_name = file.header.get_saturated_name();
    ///
    /// println!("Saturated name: {:?}", saturated_name);
    /// ```
    #[inline]
    pub fn get_saturated_name(&self) -> Namefilter {
        self.get_saturated_name_with_key(&self.derive_temporal_key())
    }

    /// Encrypts this private node header in an block, then stores that in the given
    /// BlockStore and returns its CID.
    pub async fn store(&self, store: &impl BlockStore) -> Result<Cid> {
        let temporal_key = self.derive_temporal_key();
        let snapshot_key = TemporalKey(temporal_key.derive_snapshot_key().0);

        let inumber_bytes =
            snapshot_key.key_wrap_encrypt(&serde_ipld_dagcbor::to_vec(&self.inumber)?)?;
        let ratchet_bytes =
            temporal_key.key_wrap_encrypt(&serde_ipld_dagcbor::to_vec(&self.ratchet)?)?;
        let bare_name_bytes =
            snapshot_key.key_wrap_encrypt(&serde_ipld_dagcbor::to_vec(&self.bare_name)?)?;

        let inumber_cid = store.put_block(inumber_bytes, IpldCodec::Raw).await?;
        let ratchet_cid = store.put_block(ratchet_bytes, IpldCodec::Raw).await?;
        let bare_name_cid = store.put_block(bare_name_bytes, IpldCodec::Raw).await?;

        let mut map = <BTreeMap<String, Ipld>>::new();
        map.insert("inumber".to_string(), Ipld::Link(inumber_cid));
        map.insert("ratchet".to_string(), Ipld::Link(ratchet_cid));
        map.insert("bare_name".to_string(), Ipld::Link(bare_name_cid));

        let ipld_bytes = serde_ipld_dagcbor::to_vec(&Ipld::Map(map))?;
        store.put_block(ipld_bytes, IpldCodec::Raw).await
    }

    // async fn load_bytes(cid: &Cid, store: &impl BlockStore) -> Result<(Vec<u8>)> {

    // }

    /// Loads a private node header from a given CID linking to the ciphertext block
    /// to be decrypted with given key.
    pub(crate) async fn load_temporal(
        cid: &Cid,
        temporal_key: &TemporalKey,
        store: &impl BlockStore,
    ) -> Result<PrivateNodeHeader> {
        let snapshot_key = temporal_key.derive_snapshot_key();

        let ipld_bytes = store.get_block(cid).await?;
        let Ipld::Map(map) = serde_ipld_dagcbor::from_slice(&ipld_bytes)? else {
            return Err(anyhow::anyhow!("Unable to deserialize ipld map"));
        };

        let Some(Ipld::Link(inumber_cid)) = map.get("inumber") else {
            return Err(anyhow::anyhow!("Missing inumber_cid"));
        };
        let Some(Ipld::Link(ratchet_cid)) = map.get("ratchet") else {
            return Err(anyhow::anyhow!("Missing ratchet_cid"));
        };
        let Some(Ipld::Link(bare_name_cid)) = map.get("bare_name") else {
            return Err(anyhow::anyhow!("Missing bare_name_cid"));
        };

        let inumber_bytes = TemporalKey(snapshot_key.0.to_owned())
            .key_wrap_decrypt(&store.get_block(inumber_cid).await?)?;
        let ratchet_bytes = temporal_key.key_wrap_decrypt(&store.get_block(ratchet_cid).await?)?;
        let bare_name_bytes = TemporalKey(snapshot_key.0.to_owned())
            .key_wrap_decrypt(&store.get_block(bare_name_cid).await?)?;

        let inumber: [u8; HASH_BYTE_SIZE] = serde_ipld_dagcbor::from_slice(&inumber_bytes)?;
        let ratchet: Ratchet = serde_ipld_dagcbor::from_slice(&ratchet_bytes)?;
        let bare_name: Namefilter = serde_ipld_dagcbor::from_slice(&bare_name_bytes)?;

        Ok(Self {
            inumber,
            ratchet,
            bare_name,
        })
    }

    pub(crate) async fn load_snapshot(
        cid: &Cid,
        snapshot_key: &SnapshotKey,
        store: &impl BlockStore,
    ) -> Result<PrivateNodeHeader> {
        let ipld_bytes = store.get_block(cid).await?;
        let Ipld::Map(map) = serde_ipld_dagcbor::from_slice(&ipld_bytes)? else {
            return Err(anyhow::anyhow!("Unable to deserialize ipld map"));
        };

        let Some(Ipld::Link(inumber_cid)) = map.get("inumber") else {
            return Err(anyhow::anyhow!("Missing inumber_cid"));
        };
        let Some(Ipld::Link(bare_name_cid)) = map.get("bare_name") else {
            return Err(anyhow::anyhow!("Missing bare_name_cid"));
        };

        let inumber_bytes = TemporalKey(snapshot_key.0.to_owned())
            .key_wrap_decrypt(&store.get_block(inumber_cid).await?)?;
        let bare_name_bytes = TemporalKey(snapshot_key.0.to_owned())
            .key_wrap_decrypt(&store.get_block(bare_name_cid).await?)?;

        let inumber: [u8; HASH_BYTE_SIZE] = serde_ipld_dagcbor::from_slice(&inumber_bytes)?;
        let bare_name: Namefilter = serde_ipld_dagcbor::from_slice(&bare_name_bytes)?;

        Ok(Self {
            inumber,
            ratchet: Ratchet::zero([0; 32]),
            bare_name,
        })
    }
}

impl Debug for PrivateNodeHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut inumber_str = String::from("0x");
        for byte in self.inumber {
            inumber_str.push_str(&format!("{byte:02X}"));
        }

        f.debug_struct("PrivateRef")
            .field("inumber", &inumber_str)
            .field("ratchet", &self.ratchet)
            .field("bare_name", &self.bare_name)
            .finish()
    }
}
