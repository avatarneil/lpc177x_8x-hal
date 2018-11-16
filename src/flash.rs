// Flash implementation

use core::ptr;

use fhal::flash::{Locking, Read, WriteErase};
use hal::spi;

// WORD SIZE... looks like u16 in C driver
pub enum FlashStatus {
    Busy,
    Done,
    Ready,
    HVOngoing,
}

#[derive(Debug)]

pub enum FlashError {
    Locked,
    WriteProtect,
    Alignment,
    Size,
    ReadProtection,
    NotZero,
    FetchAbort,
    Unknown,
}

impl FlashError {}

pub struct Flash {}

impl Flash {}

impl Read for Flash {
    type Error = FlashError;

    // fn read<WORD>(&self, addr: usize) -> Result<Word, Self::Error> {}
}

impl WriteErase for Flash
where
    Flash: Locking,
{
    type Error = FlashError;
    type Status = FlashStatus;

    /// Return the current flash status
    fn status(&self) -> Result<Self::Status, Self::Error> {}

    fn erase_page(&mut self, address: usize) -> Result<(), Self::Error> {}

    fn program_word(&mut self, address: usize, value: u32) -> Result<(), Self::Error> {}
}

impl Locking for Flash {
    type Error = FlashError;

    fn is_locked(&self) -> bool {}

    fn lock(&mut self) {}

    fn unlock(&mut self) {}
}
