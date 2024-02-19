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
    use std::path::PathBuf;

    use aeruginous_tbr::{Application, TarArchive};
    use clap::Parser;

    #[test]
    fn tbr_create_tar_archive() {
        assert!(Application::parse_from(
            "\
tbr
create
tests/application!application!tbr_create_tar_archive.tar
LICENSE
"
            .split_whitespace()
        )
        .main()
        .is_ok());

        let tar = TarArchive::new(
            "tests/application!application!tbr_create_tar_archive.tar",
        );

        assert!(tar.exists());
        assert_eq!(tar.list().unwrap(), &[PathBuf::from("LICENSE")]);
        assert!(tar.remove().is_ok());
    }

    #[test]
    fn tbr_help() {
        assert!(Application::parse_from("tbr --help".split_whitespace())
            .main()
            .is_ok());
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
