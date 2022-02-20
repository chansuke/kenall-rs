//* Handling the result output

use ansi_term::Colour::Green;
use cli_table::{format::Justify, print_stdout, Cell, CellStruct, Style, Table};

use crate::PostalCodeResponse;

pub trait Ui {
    fn display_address(&self);
    fn display_corporate(&self);
    fn justify_content(data: &str, position: Justify) -> CellStruct;
}

impl Ui for PostalCodeResponse {
    fn display_address(&self) {
        let rawdata = &self.data[0];
        let address_table = vec![
            vec![
                "Postal code".cell().justify(Justify::Center),
                Self::justify_content(&rawdata.postal_code, Justify::Right),
            ],
            vec![
                "Prefecture".cell().justify(Justify::Center),
                Self::justify_content(&rawdata.prefecture, Justify::Right),
            ],
            vec![
                "City".cell().justify(Justify::Center),
                Self::justify_content(&rawdata.city, Justify::Right),
            ],
            vec![
                "Town".cell().justify(Justify::Center),
                Self::justify_content(&rawdata.town, Justify::Right),
            ],
            vec![
                "Town raw".cell().justify(Justify::Center),
                Self::justify_content(&rawdata.town_raw, Justify::Right),
            ],
            vec![
                "Koaza".cell().justify(Justify::Center),
                Self::justify_content(&rawdata.koaza, Justify::Right),
            ],
            vec![
                "Kyoto street".cell().justify(Justify::Center),
                Self::justify_content(&rawdata.kyoto_street, Justify::Right),
            ],
            vec![
                "Building".cell().justify(Justify::Center),
                Self::justify_content(&rawdata.building, Justify::Right),
            ],
            vec![
                "Floor".cell().justify(Justify::Center),
                Self::justify_content(&rawdata.floor, Justify::Right),
            ],
            // Katakana below
            vec![
                "Pref kana".cell().justify(Justify::Center),
                Self::justify_content(&rawdata.prefecture_kana, Justify::Right),
            ],
            vec![
                "City kana".cell().justify(Justify::Center),
                Self::justify_content(&rawdata.city_kana, Justify::Right),
            ],
            vec![
                "Town kana".cell().justify(Justify::Center),
                Self::justify_content(&rawdata.town_kana, Justify::Right),
            ],
            vec![
                "Town kana raw".cell().justify(Justify::Center),
                Self::justify_content(&rawdata.town_kana_raw, Justify::Right),
            ],
        ]
        .table()
        .title(vec![
            "Name".cell().bold(true),
            "Data".cell().bold(true).justify(Justify::Center),
        ])
        .bold(true);

        let _ = print_stdout(address_table);
        Self::display_corporate(self);
    }

    fn display_corporate(&self) {
        let corporation = &self.data[0].corporation;
        match corporation {
            Some(cooporate) => {
                let cooporate_table = vec![
                    vec![
                        "Name".cell().justify(Justify::Center),
                        Self::justify_content(&cooporate.name, Justify::Right),
                    ],
                    vec![
                        "Name kana".cell().justify(Justify::Center),
                        Self::justify_content(&cooporate.name_kana, Justify::Right),
                    ],
                    vec![
                        "Block log".cell().justify(Justify::Center),
                        Self::justify_content(&cooporate.block_lot, Justify::Right),
                    ],
                    vec![
                        "Post office".cell().justify(Justify::Center),
                        Self::justify_content(&cooporate.post_office, Justify::Right),
                    ],
                ];
                let _ = print_stdout(cooporate_table);
            }
            None => {
                let _ = print_stdout(
                    vec![vec!["No Cooporate Data".cell().justify(Justify::Left)]]
                        .table()
                        .bold(true),
                );
            }
        };
    }

    fn justify_content(data: &str, position: Justify) -> CellStruct {
        Green.paint(data).cell().justify(position)
    }
}
