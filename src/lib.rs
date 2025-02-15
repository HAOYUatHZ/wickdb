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

#![allow(clippy::unreadable_literal)]
#![allow(clippy::type_complexity)]
// See https://github.com/rust-lang/rust-clippy/issues/1608
#![allow(clippy::redundant_closure)]

extern crate libc;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate crc;
extern crate crossbeam_channel;
extern crate crossbeam_utils;
extern crate rand;
extern crate snap;

#[macro_use]
mod util;
pub mod batch;
pub mod cache;
mod compaction;
pub mod db;
pub mod filter;
mod iterator;
mod logger;
mod mem;
pub mod options;
mod record;
mod snapshot;
mod sstable;
pub mod storage;
mod table_cache;
mod version;

pub use batch::WriteBatch;
pub use cache::{Cache, HandleRef};
pub use compaction::ManualCompaction;
pub use db::{WickDB, DB};
pub use filter::bloom::BloomFilter;
pub use iterator::Iterator;
pub use log::{LevelFilter, Log};
pub use options::{CompressionType, Options, ReadOptions, WriteOptions};
pub use sstable::block::Block;
pub use storage::{File, Storage};
pub use util::comparator::Comparator;
pub use util::slice::Slice;
pub use util::status::{Result, Status, WickErr};
pub use util::varint::*;
