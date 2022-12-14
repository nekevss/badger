//! Implement a stripped parser for the [MS-OVBA] spec
//!
//! Goal: clarity - std libraries only
//!

// VBA Project File Structure 2.2
//
// Project Root Storage
// -- /VBA
// ---- /_VBA_Project (Dependent Project Data)
// ---- /dir (Independent Project Data)
// ---- /<Module Streams>
// ---- /<SRP Streams> (designated _SRP_<n> where n is the number of SRP streams)
// -- /<VBA Form Storages>
// -- /PROJECTlk
// -- /PROJECTwm
// -- /PROJECT.
#![allow(dead_code)]

use cfb::CompoundFile;
use std::io::{Cursor, Read};

pub mod error;
pub mod ovba_module;
pub mod parser;
pub(crate) mod utils;
pub mod ovba;
pub mod project_storage;
pub mod nodes;

pub use crate::ovba_module::OvbaModule;
pub use crate::ovba::Ovba;
pub use crate::project_storage::OvbaProjectStorage;

use error::Error;
use nodes::DirStream;

use crate::parser::Parser;

pub struct BadgerOvba {
    independent_info: DirStream,
    modules: Vec<OvbaModule>,
}

impl BadgerOvba {
    /// Creates a [`BadgerOvba`] from a file buffer.
    ///
    /// Note: this file uses an external dependency
    pub fn read_from_file<R>(mut file: R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer)?;

        // Initialize the cursor for the passed File and pass it to CompoundFile
        let cursor = Cursor::new(buffer);
        let compound_file = CompoundFile::open(cursor)?;

        // Feed the CompoundFile to Ovba
        let selfie = Self::read_from_compound_file(compound_file)?;
        Ok(selfie)
    }

    pub fn read_from_compound_file(
        compound_file: CompoundFile<Cursor<Vec<u8>>>,
    ) -> Result<Self, Error> {
        let project_storage = OvbaProjectStorage::new(compound_file);
        let mut ovba = Parser::new(project_storage);

        let independent_info = ovba.parse_dir_stream()?;
        let modules = ovba.parse_modules(&independent_info)?;

        Ok(Self {
            independent_info,
            modules,
        })
    }

    pub fn modules(&self) -> Vec<OvbaModule> {
        self.modules.clone()
    }

    pub fn project_info(&self) -> DirStream {
        self.independent_info.clone()
    }
}
