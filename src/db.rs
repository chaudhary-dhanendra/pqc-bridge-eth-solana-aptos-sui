use std::sync::Arc;

use anyhow::Result;
use rocksdb::{ColumnFamilyDescriptor, Options, WriteBatch, DB};

use crate::types::{Block, BlockHeader};
use revm::primitives::B256;

/// Column family names.
const CF_BLOCKS: &str = "blocks";
const CF_TXS: &str = "txs";

/// Special key under `CF_BLOCKS` storing the current head block number.
const KEY_HEAD_NUMBER: &[u8] = b"head_number";

#[derive(Clone)]
pub struct ChainStore {
    db: Arc<DB>,
}

impl ChainStore {
    /// Open (or create) the RocksDB at the given path.
    pub fn open(path: &str) -> Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);

        let cfs = vec![
            ColumnFamilyDescriptor::new(CF_BLOCKS, Options::default()),
            ColumnFamilyDescriptor::new(CF_TXS, Options::default()),
        ];

        let db = DB::open_cf_descriptors(&opts, path, cfs)?;
        Ok(Self { db: Arc::new(db) })
    }

    fn cf(&self, name: &str) -> &rocksdb::ColumnFamily {
        self.db
            .cf_handle(name)
            .unwrap_or_else(|| panic!("missing column family {name}"))
    }

    pub fn put<T: serde::Serialize>(&self, cf: &str, key: &[u8], value: &T) -> Result<()> {
        let cf_handle = self.cf(cf);
        let bytes = bincode::serialize(value)?;
        self.db.put_cf(cf_handle, key, bytes)?;
        Ok(())
    }

    pub fn get<T: serde::de::DeserializeOwned>(
        &self,
        cf: &str,
        key: &[u8],
    ) -> Result<Option<T>> {
        let cf_handle = self.cf(cf);
        match self.db.get_cf(cf_handle, key)? {
            Some(bytes) => Ok(Some(bincode::deserialize(&bytes)?)),
            None => Ok(None),
        }
    }

    pub fn batch_put(&self, cf: &str, kvs: &[(Vec<u8>, Vec<u8>)]) -> Result<()> {
        let cf_handle = self.cf(cf);
        let mut batch = WriteBatch::default();
        for (k, v) in kvs {
            batch.put_cf(cf_handle, k, v);
        }
        self.db.write(batch)?;
        Ok(())
    }

    /// Persist an entire block, keyed by its L2 block number.
    pub fn put_block(&self, block: &Block) -> Result<()> {
        let key = block.header.number.to_be_bytes();
        self.put(CF_BLOCKS, &key, block)
    }

    /// Update the canonical head pointer.
    pub fn put_head(&self, number: u64, _hash: B256) -> Result<()> {
        // For now we only store the number; callers can load the block to get the hash.
        self.put(CF_BLOCKS, KEY_HEAD_NUMBER, &number)
    }

    /// Return the current canonical head header, if any.
    pub fn get_head_header(&self) -> Result<Option<BlockHeader>> {
        let head_number: Option<u64> = self.get(CF_BLOCKS, KEY_HEAD_NUMBER)?;
        if let Some(n) = head_number {
            if let Some(block) = self.get::<Block>(CF_BLOCKS, &n.to_be_bytes())? {
                Ok(Some(block.header))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Return the current canonical head number (0 if the chain is empty).
    pub fn get_head_number(&self) -> Result<u64> {
        let head_number: Option<u64> = self.get(CF_BLOCKS, KEY_HEAD_NUMBER)?;
        Ok(head_number.unwrap_or(0))
    }

    /// Load a full block by its L2 number.
    pub fn get_block(&self, number: u64) -> Result<Option<Block>> {
        self.get(CF_BLOCKS, &number.to_be_bytes())
    }
}
