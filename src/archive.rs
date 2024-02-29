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

use std::{
    ffi::OsStr,
    fs::{remove_file, File},
    path::{Path, PathBuf},
};
use sysexits::{ExitCode, Result};
use tar::{Archive, Builder};

/// The abstraction of a Brotli archive.
///
/// This abstraction can be used to interact with Brotli archives in the file
/// system.  It supports transactions such as creation, updating, extraction,
/// removal, and content information.
pub struct Brotli {
    path: PathBuf,
}

impl Brotli {
    /// Compress the given file to create a Brotli archive in the file system.
    ///
    /// The compression will be done basically using the default settings (see
    /// [`brotli::enc::BrotliEncoderParams::default`]) despite the following
    /// customisations:
    ///
    /// - quality:  11 (best possible compression rate).
    ///
    /// # Errors
    ///
    /// See [`sysexits::ExitCode`].
    pub fn compress<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        brotli::BrotliCompress(
            &mut File::open(path)?,
            &mut File::create(&self.path)?,
            &brotli::enc::BrotliEncoderParams {
                quality: 11,
                ..Default::default()
            },
        )?;

        Ok(())
    }

    /// Decompress this Brotli archive into the given directory.
    ///
    /// # Errors
    ///
    /// See [`sysexits::ExitCode`].
    pub fn decompress<P>(&self, destination: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let source = self.path
                .file_name()
                .ok_or(ExitCode::DataErr)?
                .to_str()
                .ok_or(ExitCode::DataErr)?;
        let target = destination
            .as_ref()
            .to_str()
            .ok_or(ExitCode::DataErr)?
            .to_string()
            + source
                .strip_suffix(".br")
                .map_or(source, |s| s);

        eprintln!("{source} → {target}");

        Ok(brotli::BrotliDecompress(
            &mut File::open(&self.path)?,
            &mut File::create(target)?,
        )?)
    }

    /// Whether this Brotli archive already exists in the file system.
    #[must_use]
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Create a new instance.  This method **does not** create a new Brotli
    /// archive in the file system.
    pub fn new<P>(path: P) -> Self
    where
        PathBuf: From<P>,
    {
        Self { path: path.into() }
    }

    /// Remove this Brotli archive from the file system.
    ///
    /// # Errors
    ///
    /// See [`sysexits::ExitCode`].
    pub fn remove(&self) -> Result<()> {
        Ok(remove_file(&self.path)?)
    }
}

/// The abstraction of a TAR archive.
///
/// This abstraction can be used to interact with TAR archives in the file
/// system.  It supports transactions such as creation, updating, extraction,
/// removal, and content information.
pub struct Tar {
    path: PathBuf,
}

impl Tar {
    /// Add a file to this TAR archive.
    ///
    /// # Errors
    ///
    /// See [`sysexits::ExitCode`].
    pub fn add_files<P>(&self, paths: &[P]) -> Result<()>
    where
        P: AsRef<OsStr> + AsRef<Path>,
    {
        let mut directories = Vec::new();
        let mut files = Vec::new();
        let mut symlinks = Vec::new();

        for path in paths {
            let path = PathBuf::from(path);

            if path.is_dir() {
                directories.push(path.clone());
                files.push(path);
            } else if path.is_file() {
                files.push(path);
            } else if path.is_symlink() && path.read_link()?.exists() {
                symlinks.push(path.read_link()?);
            }
        }

        if self.exists() {
            self.update(&files)
        } else {
            self.create(&files)
        }?;

        for directory in directories {
            let mut entries = Vec::new();

            for entry in directory.read_dir()? {
                entries.push(entry?.path());
            }

            if !entries.is_empty() {
                self.add_files(&entries)?;
            }
        }

        if symlinks.is_empty() {
            Ok(())
        } else {
            self.add_files(&symlinks)
        }
    }

    fn create<P>(&self, paths: &[P]) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let mut archive = Builder::new(File::create(&self.path)?);

        for path in paths {
            archive.append_path(path).map_or_else(
                |e| {
                    archive.finish()?;
                    eprintln!("{e}");
                    Err(e)
                },
                Ok,
            )?;
        }

        Ok(archive.finish()?)
    }

    /// Whether this TAR archive already exists in the file system.
    #[must_use]
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Extract this TAR archive's files.
    ///
    /// # Errors
    ///
    /// See [`sysexits::ExitCode`].
    pub fn extract<P>(&self, destintation: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        Ok(Archive::new(File::open(&self.path)?).unpack(destintation)?)
    }

    /// List the content of this TAR archive.
    ///
    /// This method will return a list of [`std::path::PathBuf`]s of all entries
    /// of this TAR archive.
    ///
    /// # Errors
    ///
    /// See [`sysexits::ExitCode`].
    pub fn list(&self) -> Result<Vec<PathBuf>> {
        let mut result = Vec::new();

        for entry in Archive::new(File::open(&self.path)?).entries()? {
            result.push(PathBuf::from(entry?.path()?));
        }

        Ok(result)
    }

    /// Create a new instance.  This method **does not** create a new TAR
    /// archive in the file system.
    pub fn new<P>(path: P) -> Self
    where
        PathBuf: From<P>,
    {
        Self { path: path.into() }
    }

    /// Remove this TAR archive from the file system.
    ///
    /// # Errors
    ///
    /// See [`sysexits::ExitCode`].
    pub fn remove(&self) -> Result<()> {
        Ok(remove_file(&self.path)?)
    }

    fn update<P>(&self, paths: &[P]) -> Result<()>
    where
        P: AsRef<OsStr> + AsRef<Path>,
    {
        let directory =
            tempfile::tempdir().map_or(Err(ExitCode::Unavailable), Ok)?;
        let directory = directory.path().to_str().ok_or(ExitCode::DataErr)?;
        let mut files = Vec::new();
        let new_path = format!(
            "{directory}/{}",
            self.path
                .file_name()
                .ok_or(ExitCode::DataErr)?
                .to_str()
                .ok_or(ExitCode::DataErr)?
        );

        for path in paths {
            files.push(PathBuf::from(path));
        }

        let files = &files;
        let mut new_archive = Builder::new(File::create(&new_path)?);
        let mut old_archive = Archive::new(File::open(&self.path)?);

        for file in files {
            new_archive.append_path(file).map_or_else(
                |e| {
                    new_archive.finish()?;
                    eprintln!("{e}");
                    Err(e)
                },
                Ok,
            )?;
        }

        for entry in old_archive.entries()? {
            let entry = entry?;
            let path = entry.path()?.into_owned();

            if !files.contains(&path) {
                let mut header = entry.header().clone();

                new_archive
                    .append_data(&mut header, path, entry)
                    .map_or_else(
                        |e| {
                            new_archive.finish()?;
                            eprintln!("{e}");
                            Err(e)
                        },
                        Ok,
                    )?;
            }
        }

        new_archive.finish()?;

        Ok(std::fs::rename(new_path, &self.path)?)
    }
}

/******************************************************************************/
