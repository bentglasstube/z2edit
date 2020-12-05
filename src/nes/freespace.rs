use serde::{Deserialize, Serialize};

use super::Address;
use crate::errors::*;

pub mod config {
    use super::*;
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct FreeSpace(pub Address, pub usize);

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Config(pub Vec<FreeSpace>);
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct FreeSpaceRange {
    bank: isize,
    address: u16,
    length: u16,
}

impl FreeSpaceRange {
    fn contains(&self, address: Address) -> bool {
        match address {
            Address::Prg(bank, addr) => {
                (bank == self.bank && addr >= self.address && addr < self.address + self.length)
            }
            _ => false,
        }
    }

    fn adjacent(&self, other: &FreeSpaceRange) -> bool {
        self.bank == other.bank && self.address + self.length == other.address
    }

    fn extend(&mut self, other: &FreeSpaceRange) -> bool {
        if self.adjacent(other) {
            self.length += other.length;
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct FreeSpace {
    freelist: Vec<FreeSpaceRange>,
}

impl FreeSpace {
    pub fn new(config: &config::Config) -> Result<Self> {
        let mut space = FreeSpace::default();
        for item in config.0.iter() {
            space.register(item.0, item.1 as u16)?;
        }
        Ok(space)
    }

    fn contains(&self, address: Address) -> bool {
        for f in self.freelist.iter() {
            if f.contains(address) {
                return true;
            }
        }
        false
    }

    fn coalesce(&mut self) {
        let mut i = 0;
        while i < self.freelist.len() - 1 {
            if self.freelist[i].length == 0 {
                self.freelist.remove(i);
                continue;
            }
            if self.freelist[i].adjacent(&self.freelist[i + 1]) {
                let next = self.freelist.remove(i + 1);
                self.freelist[i].extend(&next);
                continue;
            }
            i += 1;
        }
    }

    pub fn register(&mut self, address: Address, length: u16) -> Result<()> {
        if self.contains(address) {
            return Err(ErrorKind::FreeSpaceError(format!(
                "Address {:?} already in freespace",
                address
            ))
            .into());
        }

        if self.contains(address + length) {
            return Err(ErrorKind::FreeSpaceError(format!(
                "Address {:?}+{} already in freespace",
                address, length
            ))
            .into());
        }
        match address {
            Address::Prg(bank, addr) => {
                info!("FreeSpace::register: {:x?} {:?}", address, length);
                self.freelist.push(FreeSpaceRange {
                    bank: bank,
                    address: addr,
                    length: length,
                });
                self.freelist.sort();
                self.coalesce();
                Ok(())
            }
            _ => Err(ErrorKind::FreeSpaceError(format!(
                "Address must by of type Prg: {:?}",
                address
            ))
            .into()),
        }
    }

    pub fn alloc_exact_fit(&mut self, bank: isize, length: u16) -> Result<Address> {
        let mut i = 0;
        let len = self.freelist.len();

        while i < len {
            if self.freelist[i].bank == bank && self.freelist[i].length == length {
                let space = self.freelist.remove(i);
                return Ok(Address::Prg(space.bank, space.address));
            }
            i += 1;
        }
        Err(ErrorKind::OutOfMemory(bank).into())
    }

    pub fn alloc_first_fit(&mut self, bank: isize, length: u16) -> Result<Address> {
        for f in self.freelist.iter_mut() {
            if f.bank == bank && f.length >= length {
                f.length -= length;
                return Ok(Address::Prg(f.bank, f.address + f.length));
            }
        }
        Err(ErrorKind::OutOfMemory(bank).into())
    }

    pub fn alloc_near(&mut self, address: Address, length: u16) -> Result<Address> {
        let bank = if let Some(b) = address.bank() {
            b.1
        } else {
            return Err(ErrorKind::FreeSpaceError(format!(
                "Address must by of type Prg: {:?}",
                address
            ))
            .into());
        };
        // Generate (delta from requested address, freespace index) tuples.
        let mut nearness = Vec::new();
        for (i, f) in self.freelist.iter().enumerate() {
            if f.bank == bank && f.length >= length {
                let delta = f.address as isize - address.raw() as isize;
                nearness.push((delta.abs(), i));
            }
        }
        if nearness.len() > 0 {
            // Order the tuples by nearness and allocate.
            nearness.sort();
            let index = nearness[0].1;
            let result = self.freelist[index].address;
            self.freelist[index].address += length;
            self.freelist[index].length -= length;
            Ok(Address::Prg(bank, result))
        } else {
            Err(ErrorKind::OutOfMemory(bank).into())
        }
    }

    pub fn alloc(&mut self, bank: isize, length: u16) -> Result<Address> {
        let mut result = self.alloc_exact_fit(bank, length);
        if result.is_err() {
            result = self.alloc_first_fit(bank, length);
        }
        result
    }

    pub fn free(&mut self, address: Address, length: u16) {
        match self.register(address, length) {
            Err(e) => panic!("{:?}", e),
            _ => {}
        };
    }
}