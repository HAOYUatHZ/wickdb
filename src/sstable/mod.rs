// Copyright 2019 Fullstop000 <fullstop1005@gmail.com>.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

// Copyright (c) 2011 The LevelDB Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file. See the AUTHORS file for names of contributors.

///
/// # Table
///
/// Table is consist of one or more data blocks, an optional filter block
/// a metaindex block, an index block and a table footer. Metaindex block
/// is a special block used to keep parameters of the table, such as filter
/// block name and its block handle. Index block is a special block used to
/// keep record of data blocks offset and length, index block use one as
/// restart interval. The key used by index block are the last key of preceding
/// block, shorter separator of adjacent blocks or shorter successor of the
/// last key of the last block. Filter block is an optional block contains
/// sequence of filter data generated by a filter generator.
///
/// ## Table data structure:
///
/// ```text
///                                                          + optional
///                                                         /
///     +--------------+--------------+--------------+------+-------+-----------------+-------------+--------+
///     | data block 1 |      ...     | data block n | filter block | metaindex block | index block | footer |
///     +--------------+--------------+--------------+--------------+-----------------+-------------+--------+
///
///     Each block followed by a 5-bytes trailer contains compression type and checksum.
///
/// ```
///
/// ## Common Table block trailer:
///
/// ```text
///
///     +---------------------------+-------------------+
///     | compression type (1-byte) | checksum (4-byte) |
///     +---------------------------+-------------------+
///
///     The checksum is a CRC-32 computed using Castagnoli's polynomial. Compression
///     type also included in the checksum.
///
/// ```
///
/// ## Table footer:
///
/// ```text
///
///       +------------------- 40-bytes -------------------+
///      /                                                  \
///     +------------------------+--------------------+------+-----------------+
///     | metaindex block handle / index block handle / ---- | magic (8-bytes) |
///     +------------------------+--------------------+------+-----------------+
///
///     The magic are first 64-bit of SHA-1 sum of "http://code.google.com/p/leveldb/".
///
/// ```
///
/// NOTE: All fixed-length integer are little-endian.
///
///
/// # Block
///
/// Block is consist of one or more key/value entries and a block trailer.
/// Block entry shares key prefix with its preceding key until a restart
/// point reached. A block should contains at least one restart point.
/// First restart point are always zero.
///
/// Block data structure:
///
/// ```text
///       + restart point                 + restart point (depends on restart interval)
///      /                               /
///     +---------------+---------------+---------------+---------------+------------------+----------------+
///     | block entry 1 | block entry 2 |      ...      | block entry n | restarts trailer | common trailer |
///     +---------------+---------------+---------------+---------------+------------------+----------------+
///
/// ```
/// Key/value entry:
///
/// ```text
///               +---- key len ----+
///              /                   \
///     +-------+---------+-----------+---------+--------------------+--------------+----------------+
///     | shared (varint) | not shared (varint) | value len (varint) | key (varlen) | value (varlen) |
///     +-----------------+---------------------+--------------------+--------------+----------------+
///
///     Block entry shares key prefix with its preceding key:
///     Conditions:
///         restart_interval=2
///         entry one  : key=deck,value=v1
///         entry two  : key=dock,value=v2
///         entry three: key=duck,value=v3
///     The entries will be encoded as follow:
///
///       + restart point (offset=0)                                                 + restart point (offset=16)
///      /                                                                          /
///     +-----+-----+-----+----------+--------+-----+-----+-----+---------+--------+-----+-----+-----+----------+--------+
///     |  0  |  4  |  2  |  "deck"  |  "v1"  |  1  |  3  |  2  |  "ock"  |  "v2"  |  0  |  4  |  2  |  "duck"  |  "v3"  |
///     +-----+-----+-----+----------+--------+-----+-----+-----+---------+--------+-----+-----+-----+----------+--------+
///      \                                   / \                                  / \                                   /
///       +----------- entry one -----------+   +----------- entry two ----------+   +---------- entry three ----------+
///
///     The block trailer will contains two restart points:
///
///     +------------+-----------+--------+
///     |     0      |    16     |   2    |
///     +------------+-----------+---+----+
///      \                      /     \
///       +-- restart points --+       + restart points length
///
/// ```
///
/// # Block restarts trailer
///
/// ```text
///
///       +-- 4-bytes --+
///      /               \
///     +-----------------+-----------------+-----------------+------------------------------+
///     | restart point 1 |       ....      | restart point n | restart points len (4-bytes) |
///     +-----------------+-----------------+-----------------+------------------------------+
///
/// ```
///
/// NOTE: All fixed-length integer are little-endian.
///
/// # Filter block
///
/// Filter block consist of one or more filter data and a filter block trailer.
/// The trailer contains filter data offsets, a trailer offset and a 1-byte base Lg.
///
/// Filter block data structure:
///
/// ```text
///
///       + offset 1      + offset 2      + offset n      + trailer offset
///      /               /               /               /
///     +---------------+---------------+---------------+---------+
///     | filter data 1 |      ...      | filter data n | trailer |
///     +---------------+---------------+---------------+---------+
///
/// ```
///
/// Filter block trailer:
///
/// ```text
///
///       +- 4-bytes -+
///      /             \
///     +---------------+---------------+---------------+-------------------------------+------------------+
///     | data 1 offset |      ....     | data n offset | data-offsets length (4-bytes) | base Lg (1-byte) |
///     +---------------+---------------+---------------+-------------------------------+------------------+
///
/// ```
///
/// NOTE: The filter block is not compressed
///
/// # Index block
///
/// Index block consist of one or more block handle data and a common block trailer.
/// The 'separator key' is the key just bigger than the last key in the data block which the 'block handle' pointed to
///
/// ```text
///
///     +---------------+--------------+
///     |      key      |    value     |
///     +---------------+--------------+
///     | separator key | block handle |---- a block handle points a data block starting offset and the its size
///     | ...           | ...          |
///     +---------------+--------------+
///
/// ```
///
/// NOTE: All fixed-length integer are little-endian.
///
/// # Meta block
///
/// This meta block contains a bunch of stats. The key is the name of the statistic. The value contains the statistic.
/// For the current implementation, the meta block only contains the filter meta data:
///
/// ```text
///
///     +-------------+---------------------+
///     |     key     |        value        |
///     +-------------+---------------------+
///     | filter name | filter block handle |
///     +-------------+---------------------+
///
/// ```
///
/// NOTE: All fixed-length integer are little-endian.
pub mod block;
mod filter_block;
pub mod table;

use crate::util::coding::{decode_fixed_64, put_fixed_64};
use crate::util::status::{Status, WickErr};
use crate::util::varint::{VarintU64, MAX_VARINT_LEN_U64};

const TABLE_MAGIC_NUMBER: u64 = 0xdb4775248b80fb57;

// 1byte compression type + 4bytes cyc
const BLOCK_TRAILER_SIZE: usize = 5;

// Maximum encoding length of a BlockHandle
const MAX_BLOCK_HANDLE_ENCODE_LENGTH: usize = 2 * MAX_VARINT_LEN_U64;

// Encoded length of a Footer.  Note that the serialization of a
// Footer will always occupy exactly this many bytes.  It consists
// of two block handles and a magic number.
const FOOTER_ENCODED_LENGTH: usize = 2 * MAX_BLOCK_HANDLE_ENCODE_LENGTH + 8;

/// `BlockHandle` is a pointer to the extent of a file that stores a data
/// block or a meta block.
#[derive(Eq, PartialEq, Debug)]
pub struct BlockHandle {
    offset: u64,
    // NOTICE: the block trailer size is not included
    size: u64,
}

impl BlockHandle {
    pub fn new(offset: u64, size: u64) -> Self {
        Self { offset, size }
    }

    #[inline]
    pub fn set_offset(&mut self, offset: u64) {
        self.offset = offset;
    }

    #[inline]
    pub fn set_size(&mut self, size: u64) {
        self.size = size
    }

    /// Appends varint encoded offset and size into given `dst`
    #[inline]
    pub fn encoded_to(&self, dst: &mut Vec<u8>) {
        VarintU64::put_varint(dst, self.offset);
        VarintU64::put_varint(dst, self.size);
    }

    /// Returns bytes for a encoded BlockHandle
    #[inline]
    pub fn encoded(&self) -> Vec<u8> {
        let mut v = vec![];
        self.encoded_to(&mut v);
        v
    }

    /// Decodes a BlockHandle from bytes
    ///
    /// # Error
    ///
    /// If varint decoding fails, return `Status::Corruption` with relative messages
    #[inline]
    pub fn decode_from(src: &[u8]) -> Result<(Self, usize), WickErr> {
        if let Some((offset, n)) = VarintU64::read(src) {
            if let Some((size, m)) = VarintU64::read(&src[n..]) {
                Ok((Self::new(offset, size), m + n))
            } else {
                Err(WickErr::new(Status::Corruption, Some("bad block handle")))
            }
        } else {
            Err(WickErr::new(Status::Corruption, Some("bad block handle")))
        }
    }
}

/// `Footer` encapsulates the fixed information stored at the tail
/// end of every table file.
#[derive(Debug)]
pub struct Footer {
    meta_index_handle: BlockHandle,
    index_handle: BlockHandle,
}

impl Footer {
    #[inline]
    pub fn new(meta_index_handle: BlockHandle, index_handle: BlockHandle) -> Self {
        Self {
            meta_index_handle,
            index_handle,
        }
    }

    /// Decodes a `Footer` from the given `src` bytes and returns the decoded length
    ///
    /// # Error
    ///
    /// Returns `Status::Corruption` when decoding meta index or index handle fails
    ///
    pub fn decode_from(src: &[u8]) -> Result<(Self, usize), WickErr> {
        let magic = decode_fixed_64(&src[FOOTER_ENCODED_LENGTH - 8..]);
        if magic != TABLE_MAGIC_NUMBER {
            return Err(WickErr::new(
                Status::Corruption,
                Some("not an sstable (bad magic number)"),
            ));
        };
        let (meta_index_handle, n) = BlockHandle::decode_from(src)?;
        let (index_handle, m) = BlockHandle::decode_from(&src[n..])?;
        Ok((
            Self {
                meta_index_handle,
                index_handle,
            },
            m + n,
        ))
    }

    /// Encodes footer and returns the encoded bytes
    pub fn encoded(&self) -> Vec<u8> {
        let mut v = vec![];
        self.meta_index_handle.encoded_to(&mut v);
        self.index_handle.encoded_to(&mut v);
        v.resize(2 * MAX_BLOCK_HANDLE_ENCODE_LENGTH, 0);
        put_fixed_64(&mut v, TABLE_MAGIC_NUMBER);
        assert_eq!(
            v.len(),
            FOOTER_ENCODED_LENGTH,
            "[footer] the length of encoded footer is {}, expect {}",
            v.len(),
            FOOTER_ENCODED_LENGTH
        );
        v
    }
}

#[cfg(test)]
mod test_footer {
    use crate::sstable::{BlockHandle, Footer};
    use crate::util::status::Status;
    use std::error::Error;

    #[test]
    fn test_footer_corruption() {
        let footer = Footer::new(BlockHandle::new(300, 100), BlockHandle::new(401, 1000));
        let mut encoded = footer.encoded();
        let last = encoded.last_mut().unwrap();
        *last += 1;
        let r1 = Footer::decode_from(&encoded);
        assert!(r1.is_err());
        let e1 = r1.unwrap_err();
        assert_eq!(e1.status(), Status::Corruption);
        assert_eq!(e1.description(), "not an sstable (bad magic number)");
    }

    #[test]
    fn test_encode_decode() {
        let footer = Footer::new(BlockHandle::new(300, 100), BlockHandle::new(401, 1000));
        let encoded = footer.encoded();
        let (footer, _) = Footer::decode_from(&encoded).expect("footer decoding should work");
        assert_eq!(footer.index_handle, BlockHandle::new(401, 1000));
        assert_eq!(footer.meta_index_handle, BlockHandle::new(300, 100));
    }
}
