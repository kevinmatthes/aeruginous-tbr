/*********************** GNU General Public License 3.0 ***********************\
|                                                                              |
|  Copyright (C) 2024 Kevin Matthes                                            |
|                                                                              |
|  This program is free software: you can redistribute it and/or modify        |
|  it under the terms of the GNU General Public License as published by        |
|  the Free Software Foundation, either version 3 of the License, or           |
|  (at your option) any later version.                                         |
|                                                                              |
|  This program is distributed in the hope that it will be useful,             |
|  but WITHOUT ANY WARRANTY; without even the implied warranty of              |
|  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the               |
|  GNU General Public License for more details.                                |
|                                                                              |
|  You should have received a copy of the GNU General Public License           |
|  along with this program.  If not, see <https://www.gnu.org/licenses/>.      |
|                                                                              |
\******************************************************************************/

use std::path::PathBuf;
use sysexits::{ExitCode, Result};

/// Interact with Brotli, TAR, TAR.BR, and TBR archives.
#[derive(clap::Parser, Clone)]
pub struct Application {
    /// The operation to perform on the considered archive.
    mode: Mode,

    /// The archive to interact with.
    archive: PathBuf,

    /// The file(s) to add to the considered archive.
    files: Vec<PathBuf>,

    /// The directory to unpack the archive's files into; defaulting to the
    /// current working directory, if omitted.
    #[arg(long, short)]
    destination: Option<PathBuf>,
}

impl Application {
    /// Interact with the given archive.
    ///
    /// # Errors
    ///
    /// See [`sysexits::ExitCode`].
    pub fn main(&self) -> Result<()> {
        self.wrap().main()
    }

    fn wrap(&self) -> Logic {
        Logic {
            cli: self.clone(),
            paths: Vec::new(),
        }
    }
}

struct Logic {
    cli: Application,
    paths: Vec<PathBuf>,
}

impl Logic {
    fn main(&mut self) -> Result<()> {
        self.resolve_files()?;

        self.cli.archive.extension().map_or_else(
            || {
                eprintln!(
                    "Please specify the archive to work on with its extension."
                );
                Err(ExitCode::Usage)
            },
            |e| {
                if e.to_str() == Some("tar") {
                    self.tar_archive()
                } else {
                    eprintln!("This archive type is not supported.");
                    Err(ExitCode::Usage)
                }
            },
        )
    }

    fn resolve_files(&mut self) -> Result<()> {
        for file in &self.cli.files {
            for path in
                glob::glob(file.as_path().to_str().ok_or(ExitCode::DataErr)?)
                    .map_or(Err(ExitCode::DataErr), Ok)?
            {
                self.paths.push(path.map_or(Err(ExitCode::DataErr), Ok)?);
            }
        }

        Ok(())
    }

    fn tar_archive(&self) -> Result<()> {
        let tar = crate::Tar::new(&self.cli.archive);

        match self.cli.mode {
            Mode::Content => {
                for path in tar.list()? {
                    println!("{}", path.display());
                }

                Ok(())
            }
            Mode::Extraction => tar.extract(
                self.cli
                    .destination
                    .as_ref()
                    .map_or(".", |d| d.as_path().to_str().map_or(".", |s| s)),
            ),
            Mode::Removal => tar.remove(),
            Mode::Update => tar.add_files(&self.paths),
        }
    }
}

/// The possible ways to interact with an archive.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    /// Show this archive's contents.
    Content,

    /// Extract this archive's contents.
    Extraction,

    /// Remove this archive from the file system.
    Removal,

    /// Update this archive.
    Update,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Content => "archive content preview",
                Self::Extraction => "archive extraction",
                Self::Removal => "archive removal",
                Self::Update => "archive update",
            }
        )
    }
}

impl std::str::FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "compress" | "create" | "edit" | "update" => Ok(Self::Update),
            "content" | "info" | "list" | "show" => Ok(Self::Content),
            "delete" | "remove" => Ok(Self::Removal),
            "decompress" | "extract" | "uncompress" | "unpack" => {
                Ok(Self::Extraction)
            }
            _ => Err(format!("'{s}' is not supported, yet")),
        }
    }
}

/******************************************************************************/
