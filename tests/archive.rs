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

mod tar {
    use aeruginous_io::PathBufLikeReader;
    use aeruginous_tbr::Tar;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn exists_failure() {
        assert!(!Tar::new("does_not_exist.tar").exists());
    }

    #[test]
    fn life_cycle() {
        let d = tempdir().unwrap();
        let d = d.path().to_str().unwrap();
        let tar = Tar::new(d.to_string() + "/archive.tar");

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
        assert!(Tar::new("does_not_exist.tar").remove().is_err());
    }

    #[test]
    fn symlink_does_not_exist() {
        let d = tempdir().unwrap();
        let d = d.path().to_str().unwrap();
        let tar = Tar::new(d.to_string() + "/archive.tar");

        std::fs::remove_file("no_such.txt").unwrap();

        std::os::unix::fs::symlink(
            "does_not_exist.txt",
            d.to_string() + "/no_such.txt",
        )
        .unwrap();

        assert!(tar
            .add_files(&["LICENSE".to_sting(), d.to_string() + "/no_such.txt"])
            .is_ok());
        assert_eq!(tar.list().unwrap(), [PathBuf::from("LICENSE")]);
        assert!(tar.remove().is_ok());
    }

    #[test]
    fn update() {
        let d = tempdir().unwrap();
        let d = d.path().to_str().unwrap();

        let tar = Tar::new(d.to_string() + "/archive.tar");

        assert!(!tar.exists());
        assert!(tar
            .add_files(&["Cargo.lock", "Cargo.toml", "LICENSE"])
            .is_ok());
        assert_eq!(
            tar.list().unwrap(),
            [
                PathBuf::from("Cargo.lock"),
                PathBuf::from("Cargo.toml"),
                PathBuf::from("LICENSE")
            ]
        );
        assert!(tar.add_files(&["LICENSE", "CITATION.cff"]).is_ok());
        assert_eq!(
            tar.list().unwrap(),
            [
                PathBuf::from("LICENSE"),
                PathBuf::from("CITATION.cff"),
                PathBuf::from("Cargo.lock"),
                PathBuf::from("Cargo.toml")
            ]
        );
        assert!(tar.remove().is_ok());
    }
}

/******************************************************************************/
