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

use aeruginous_io::PathBufLikeReader;
use aeruginous_tbr::TarArchive;
use std::path::PathBuf;

#[test]
fn exists_failure() {
    assert!(!TarArchive::new("tests/assets/does_not_exist.tar").exists());
}

#[test]
fn life_cycle() {
    let d = tempfile::tempdir().unwrap();
    let d = d.path().to_str().unwrap();
    let tar = TarArchive::new(d.to_string() + "/life_cycle.tar");

    assert!(!tar.exists());
    assert!(tar.add_files(&["LICENSE"]).is_ok());
    assert_eq!(tar.list().unwrap(), [PathBuf::from("LICENSE")]);
    assert!(tar.add_files(&[".renovaterc.json5"]).is_ok());
    assert_eq!(
        tar.list().unwrap(),
        [PathBuf::from(".renovaterc.json5"), PathBuf::from("LICENSE")]
    );
    assert!(tar.extract(d).is_ok());
    assert_eq!(
        ".renovaterc.json5".read_silently().unwrap(),
        (d.to_string() + "/.renovaterc.json5")
            .read_silently()
            .unwrap()
    );
    assert_eq!(
        "LICENSE".read_silently().unwrap(),
        (d.to_string() + "/LICENSE").read_silently().unwrap()
    );
    assert!(tar.remove().is_ok());
}

#[test]
fn remove_failure() {
    assert!(TarArchive::new("tests/assets/does_not_exist.tar")
        .remove()
        .is_err());
}

/******************************************************************************/
