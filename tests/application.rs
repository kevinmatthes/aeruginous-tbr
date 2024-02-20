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

mod application {
    mod create {
        use aeruginous_tbr::{Application, TarArchive};
        use clap::Parser;
        use std::path::PathBuf;

        #[test]
        fn tar_archive() {
            let d = tempfile::tempdir().unwrap();
            let d = d.path().to_str().unwrap();

            assert!(Application::parse_from(
                ("tbr create ".to_string() + d + "/tar_archive.tar Cargo.*")
                    .split_whitespace()
            )
            .main()
            .is_ok());

            let tar = TarArchive::new(d.to_string() + "/tar_archive.tar");

            assert!(tar.exists());
            assert_eq!(
                tar.list().unwrap(),
                &[PathBuf::from("Cargo.lock"), PathBuf::from("Cargo.toml")]
            );
            assert!(tar.remove().is_ok());
        }
    }

    mod list {
        use aeruginous_tbr::Application;
        use clap::Parser;

        #[test]
        fn no_extension() {
            assert!(Application::parse_from(
                "tbr list tests/assets/does_not_exist".split_whitespace()
            )
            .main()
            .is_err());
        }

        #[test]
        fn unsupported_archive_type() {
            assert!(Application::parse_from(
                "tbr list tests/assets/does_not_exist.zip".split_whitespace()
            )
            .main()
            .is_err());
        }
    }

    mod remove {
        use aeruginous_tbr::Application;
        use clap::Parser;

        #[test]
        fn file_does_not_exist() {
            assert!(Application::parse_from(
                "tbr remove tests/assets/does_not_exist.tar".split_whitespace()
            )
            .main()
            .is_err());
        }
    }
}

mod application_mode {
    mod clone {
        use aeruginous_tbr::ApplicationMode;

        #[test]
        fn content() {
            assert_eq!(
                ApplicationMode::Content.clone(),
                ApplicationMode::Content
            );
        }

        #[test]
        fn extraction() {
            assert_eq!(
                ApplicationMode::Extraction.clone(),
                ApplicationMode::Extraction
            );
        }

        #[test]
        fn removal() {
            assert_eq!(
                ApplicationMode::Removal.clone(),
                ApplicationMode::Removal
            );
        }

        #[test]
        fn update() {
            assert_eq!(
                ApplicationMode::Update.clone(),
                ApplicationMode::Update
            );
        }
    }

    mod debug {
        use aeruginous_tbr::ApplicationMode;

        #[test]
        fn content() {
            assert_eq!(format!("{:?}", ApplicationMode::Content), "Content");
        }

        #[test]
        fn extraction() {
            assert_eq!(
                format!("{:?}", ApplicationMode::Extraction),
                "Extraction"
            );
        }

        #[test]
        fn removal() {
            assert_eq!(format!("{:?}", ApplicationMode::Removal), "Removal");
        }

        #[test]
        fn update() {
            assert_eq!(format!("{:?}", ApplicationMode::Update), "Update");
        }
    }

    mod display {
        use aeruginous_tbr::ApplicationMode;

        #[test]
        fn content() {
            assert_eq!(
                format!("{}", ApplicationMode::Content),
                "archive content preview"
            );
        }

        #[test]
        fn extraction() {
            assert_eq!(
                format!("{}", ApplicationMode::Extraction),
                "archive extraction"
            );
        }

        #[test]
        fn removal() {
            assert_eq!(
                format!("{}", ApplicationMode::Removal),
                "archive removal"
            );
        }

        #[test]
        fn update() {
            assert_eq!(
                format!("{}", ApplicationMode::Update),
                "archive update"
            );
        }
    }

    mod from_str {
        use aeruginous_tbr::ApplicationMode;
        use std::str::FromStr;

        macro_rules! make_test {
            ( $( $v:ident { $( $s:ident ),+ } ),+ ) => {
                $(
                    $(
                        #[test]
                        fn $s() {
                            assert_eq!(
                                ApplicationMode::from_str(stringify!($s))
                                    .unwrap(),
                                ApplicationMode::$v,
                            );
                        }
                    )+
                )+
            };
        }

        make_test!(
            Content {
                content,
                info,
                list,
                show
            },
            Extraction {
                decompress,
                extract,
                uncompress,
                unpack
            },
            Removal { delete, remove },
            Update {
                compress,
                create,
                edit,
                update
            }
        );

        #[test]
        fn failure() {
            assert_eq!(
                ApplicationMode::from_str("").unwrap_err(),
                "'' is not supported, yet"
            );
        }
    }
}

/******************************************************************************/
