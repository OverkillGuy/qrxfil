//! Command line argument parsing and command running

use clap::Clap;
use std::io;

mod exfil;
use exfil::Exfil;

/// Transfer/backup files as a sequence of QR codes
#[derive(Debug, Clap)]
pub enum Command {
    Exfil(Exfil),
}

impl Default for Command {
    fn default() -> Self {
        Self::parse()
    }
}

impl Command {
    pub fn run(&self) -> io::Result<()> {
        match self {
            Self::Exfil(exfil) => exfil.run(),
        }
    }
}
