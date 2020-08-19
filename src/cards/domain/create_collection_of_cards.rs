use cards::domain::card::Cards;
use cards::domain::coup_card::CoupCard;
use cards::domain::pivotal_event_card::PivotalEventCard;
use cards::domain::regular_card::RegularCard;
use game_definitions::factions::Factions;
use std::collections::HashMap;

pub fn create_collection_of_cards() -> HashMap<u8, Cards> {
    [
        (
            1,
            RegularCard::new(
                1,
                "Gulf of Tonkin".to_string(),
                [Factions::US, Factions::NVA, Factions::ARVN, Factions::VC],
                None,
            )
            .into(),
        ),
        (
            2,
            RegularCard::new(
                2,
                "Kissinger".to_string(),
                [Factions::US, Factions::NVA, Factions::ARVN, Factions::VC],
                None,
            )
            .into(),
        ),
        (
            3,
            RegularCard::new(
                3,
                "Peace Talks".to_string(),
                [Factions::US, Factions::NVA, Factions::ARVN, Factions::VC],
                None,
            )
            .into(),
        ),
        (
            4,
            RegularCard::new(
                4,
                "Top Gun".to_string(),
                [Factions::US, Factions::NVA, Factions::ARVN, Factions::VC],
                Some(Factions::US),
            )
            .into(),
        ),
        (
            5,
            RegularCard::new(
                5,
                "Wild Weasels".to_string(),
                [Factions::US, Factions::NVA, Factions::ARVN, Factions::VC],
                None,
            )
            .into(),
        ),
        (
            6,
            RegularCard::new(
                6,
                "Aces".to_string(),
                [Factions::US, Factions::NVA, Factions::VC, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            7,
            RegularCard::new(
                7,
                "ADSID".to_string(),
                [Factions::US, Factions::NVA, Factions::VC, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            8,
            RegularCard::new(
                8,
                "Arc Light".to_string(),
                [Factions::US, Factions::NVA, Factions::VC, Factions::ARVN],
                Some(Factions::US),
            )
            .into(),
        ),
        (
            9,
            RegularCard::new(
                9,
                "Psychedelic Cookie".to_string(),
                [Factions::US, Factions::NVA, Factions::VC, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            10,
            RegularCard::new(
                10,
                "Rolling Thunder".to_string(),
                [Factions::US, Factions::NVA, Factions::VC, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            11,
            RegularCard::new(
                11,
                "Abrams".to_string(),
                [Factions::US, Factions::ARVN, Factions::NVA, Factions::VC],
                Some(Factions::US),
            )
            .into(),
        ),
        (
            12,
            RegularCard::new(
                12,
                "Capt Buck Adams".to_string(),
                [Factions::US, Factions::ARVN, Factions::NVA, Factions::VC],
                None,
            )
            .into(),
        ),
        (
            13,
            RegularCard::new(
                13,
                "Cobras".to_string(),
                [Factions::US, Factions::ARVN, Factions::NVA, Factions::VC],
                Some(Factions::US),
            )
            .into(),
        ),
        (
            14,
            RegularCard::new(
                14,
                "M-48 Patton".to_string(),
                [Factions::US, Factions::ARVN, Factions::NVA, Factions::VC],
                Some(Factions::US),
            )
            .into(),
        ),
        (
            15,
            RegularCard::new(
                15,
                "Medevac".to_string(),
                [Factions::US, Factions::ARVN, Factions::NVA, Factions::VC],
                None,
            )
            .into(),
        ),
        (
            16,
            RegularCard::new(
                16,
                "Blowtorch Komer".to_string(),
                [Factions::US, Factions::ARVN, Factions::VC, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            17,
            RegularCard::new(
                17,
                "Claymores".to_string(),
                [Factions::US, Factions::ARVN, Factions::VC, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            18,
            RegularCard::new(
                18,
                "Combined Action Platoons".to_string(),
                [Factions::US, Factions::ARVN, Factions::VC, Factions::NVA],
                Some(Factions::US),
            )
            .into(),
        ),
        (
            19,
            RegularCard::new(
                19,
                "CORDS".to_string(),
                [Factions::US, Factions::ARVN, Factions::VC, Factions::NVA],
                Some(Factions::US),
            )
            .into(),
        ),
        (
            20,
            RegularCard::new(
                20,
                "Laser Guided Bombs".to_string(),
                [Factions::US, Factions::ARVN, Factions::VC, Factions::NVA],
                Some(Factions::US),
            )
            .into(),
        ),
        (
            21,
            RegularCard::new(
                21,
                "Americal".to_string(),
                [Factions::US, Factions::VC, Factions::NVA, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            22,
            RegularCard::new(
                22,
                "Da Nang".to_string(),
                [Factions::US, Factions::VC, Factions::NVA, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            23,
            RegularCard::new(
                23,
                "Operation Attleboro".to_string(),
                [Factions::US, Factions::VC, Factions::NVA, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            24,
            RegularCard::new(
                24,
                "Operation Starlite".to_string(),
                [Factions::US, Factions::VC, Factions::NVA, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            25,
            RegularCard::new(
                25,
                "TF-116 Riverines".to_string(),
                [Factions::US, Factions::VC, Factions::NVA, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            26,
            RegularCard::new(
                26,
                "LRRP".to_string(),
                [Factions::US, Factions::VC, Factions::ARVN, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            27,
            RegularCard::new(
                27,
                "Phoenix Program".to_string(),
                [Factions::US, Factions::VC, Factions::ARVN, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            28,
            RegularCard::new(
                28,
                "Search and Destroy".to_string(),
                [Factions::US, Factions::VC, Factions::ARVN, Factions::NVA],
                Some(Factions::US),
            )
            .into(),
        ),
        (
            29,
            RegularCard::new(
                29,
                "Tribesmen".to_string(),
                [Factions::US, Factions::VC, Factions::ARVN, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            30,
            RegularCard::new(
                30,
                "USS New Jersey".to_string(),
                [Factions::US, Factions::VC, Factions::ARVN, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            31,
            RegularCard::new(
                31,
                "AAA".to_string(),
                [Factions::NVA, Factions::US, Factions::ARVN, Factions::VC],
                Some(Factions::NVA),
            )
            .into(),
        ),
        (
            32,
            RegularCard::new(
                32,
                "Long Range Guns".to_string(),
                [Factions::NVA, Factions::US, Factions::ARVN, Factions::VC],
                Some(Factions::NVA),
            )
            .into(),
        ),
        (
            33,
            RegularCard::new(
                33,
                "MiGs".to_string(),
                [Factions::NVA, Factions::US, Factions::ARVN, Factions::VC],
                Some(Factions::NVA),
            )
            .into(),
        ),
        (
            34,
            RegularCard::new(
                34,
                "SA-2s".to_string(),
                [Factions::NVA, Factions::US, Factions::ARVN, Factions::VC],
                Some(Factions::NVA),
            )
            .into(),
        ),
        (
            35,
            RegularCard::new(
                35,
                "Thanh Hoa".to_string(),
                [Factions::NVA, Factions::US, Factions::ARVN, Factions::VC],
                None,
            )
            .into(),
        ),
        (
            36,
            RegularCard::new(
                36,
                "Hamburger Hill".to_string(),
                [Factions::NVA, Factions::US, Factions::VC, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            37,
            RegularCard::new(
                37,
                "Khe Sanh".to_string(),
                [Factions::NVA, Factions::US, Factions::VC, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            38,
            RegularCard::new(
                38,
                "McNamara Line".to_string(),
                [Factions::NVA, Factions::US, Factions::VC, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            39,
            RegularCard::new(
                39,
                "Oriskany".to_string(),
                [Factions::NVA, Factions::US, Factions::VC, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            40,
            RegularCard::new(
                40,
                "PoWs".to_string(),
                [Factions::NVA, Factions::US, Factions::VC, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            41,
            RegularCard::new(
                41,
                "Bombing Pause".to_string(),
                [Factions::NVA, Factions::ARVN, Factions::US, Factions::VC],
                None,
            )
            .into(),
        ),
        (
            42,
            RegularCard::new(
                42,
                "Chou En Lai".to_string(),
                [Factions::NVA, Factions::ARVN, Factions::US, Factions::VC],
                None,
            )
            .into(),
        ),
        (
            43,
            RegularCard::new(
                43,
                "Economic Aid".to_string(),
                [Factions::NVA, Factions::ARVN, Factions::US, Factions::VC],
                None,
            )
            .into(),
        ),
        (
            44,
            RegularCard::new(
                44,
                "Ia Drang".to_string(),
                [Factions::NVA, Factions::ARVN, Factions::US, Factions::VC],
                None,
            )
            .into(),
        ),
        (
            45,
            RegularCard::new(
                45,
                "PT-76".to_string(),
                [Factions::NVA, Factions::ARVN, Factions::US, Factions::VC],
                Some(Factions::NVA),
            )
            .into(),
        ),
        (
            46,
            RegularCard::new(
                46,
                "559th Transport Grp".to_string(),
                [Factions::NVA, Factions::ARVN, Factions::VC, Factions::US],
                None,
            )
            .into(),
        ),
        (
            47,
            RegularCard::new(
                47,
                "Chu Luc".to_string(),
                [Factions::NVA, Factions::ARVN, Factions::VC, Factions::US],
                None,
            )
            .into(),
        ),
        (
            48,
            RegularCard::new(
                48,
                "Nam Dong".to_string(),
                [Factions::NVA, Factions::ARVN, Factions::VC, Factions::US],
                None,
            )
            .into(),
        ),
        (
            49,
            RegularCard::new(
                49,
                "Russian Arms".to_string(),
                [Factions::NVA, Factions::ARVN, Factions::VC, Factions::US],
                None,
            )
            .into(),
        ),
        (
            50,
            RegularCard::new(
                50,
                "Uncle Ho".to_string(),
                [Factions::NVA, Factions::ARVN, Factions::VC, Factions::US],
                None,
            )
            .into(),
        ),
        (
            51,
            RegularCard::new(
                51,
                "301st Supply Bn".to_string(),
                [Factions::NVA, Factions::VC, Factions::US, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            52,
            RegularCard::new(
                52,
                "RAND".to_string(),
                [Factions::NVA, Factions::VC, Factions::US, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            53,
            RegularCard::new(
                53,
                "Sappers".to_string(),
                [Factions::NVA, Factions::VC, Factions::US, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            54,
            RegularCard::new(
                54,
                "Son Tay".to_string(),
                [Factions::NVA, Factions::VC, Factions::US, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            55,
            RegularCard::new(
                55,
                "Trucks".to_string(),
                [Factions::NVA, Factions::VC, Factions::US, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            56,
            RegularCard::new(
                56,
                "Vo Nguyen Giap".to_string(),
                [Factions::NVA, Factions::VC, Factions::ARVN, Factions::US],
                None,
            )
            .into(),
        ),
        (
            57,
            RegularCard::new(
                57,
                "International Unrest".to_string(),
                [Factions::NVA, Factions::VC, Factions::ARVN, Factions::US],
                None,
            )
            .into(),
        ),
        (
            58,
            RegularCard::new(
                58,
                "Pathet Lao".to_string(),
                [Factions::NVA, Factions::VC, Factions::ARVN, Factions::US],
                None,
            )
            .into(),
        ),
        (
            59,
            RegularCard::new(
                59,
                "Plei Mei".to_string(),
                [Factions::NVA, Factions::VC, Factions::ARVN, Factions::US],
                None,
            )
            .into(),
        ),
        (
            60,
            RegularCard::new(
                60,
                "War Photographer".to_string(),
                [Factions::NVA, Factions::VC, Factions::ARVN, Factions::US],
                None,
            )
            .into(),
        ),
        (
            61,
            RegularCard::new(
                61,
                "Armored Cavalry".to_string(),
                [Factions::ARVN, Factions::US, Factions::NVA, Factions::VC],
                Some(Factions::ARVN),
            )
            .into(),
        ),
        (
            62,
            RegularCard::new(
                62,
                "Cambodian Civil War".to_string(),
                [Factions::ARVN, Factions::US, Factions::NVA, Factions::VC],
                None,
            )
            .into(),
        ),
        (
            63,
            RegularCard::new(
                63,
                "Fact Finding".to_string(),
                [Factions::ARVN, Factions::US, Factions::NVA, Factions::VC],
                None,
            )
            .into(),
        ),
        (
            64,
            RegularCard::new(
                64,
                "Honolulu Conference".to_string(),
                [Factions::ARVN, Factions::US, Factions::NVA, Factions::VC],
                None,
            )
            .into(),
        ),
        (
            65,
            RegularCard::new(
                65,
                "International Forces".to_string(),
                [Factions::ARVN, Factions::US, Factions::NVA, Factions::VC],
                None,
            )
            .into(),
        ),
        (
            66,
            RegularCard::new(
                66,
                "Ambassador Taylor".to_string(),
                [Factions::ARVN, Factions::US, Factions::VC, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            67,
            RegularCard::new(
                67,
                "Amphib Landing".to_string(),
                [Factions::ARVN, Factions::US, Factions::VC, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            68,
            RegularCard::new(
                68,
                "Green Berets".to_string(),
                [Factions::ARVN, Factions::US, Factions::VC, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            69,
            RegularCard::new(
                69,
                "MACV".to_string(),
                [Factions::ARVN, Factions::US, Factions::VC, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            70,
            RegularCard::new(
                70,
                "ROKs".to_string(),
                [Factions::ARVN, Factions::US, Factions::VC, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            71,
            RegularCard::new(
                71,
                "An Loc".to_string(),
                [Factions::ARVN, Factions::NVA, Factions::US, Factions::VC],
                None,
            )
            .into(),
        ),
        (
            72,
            RegularCard::new(
                72,
                "Body Count".to_string(),
                [Factions::ARVN, Factions::NVA, Factions::US, Factions::VC],
                None,
            )
            .into(),
        ),
        (
            73,
            RegularCard::new(
                73,
                "Great Society".to_string(),
                [Factions::ARVN, Factions::NVA, Factions::US, Factions::VC],
                None,
            )
            .into(),
        ),
        (
            74,
            RegularCard::new(
                74,
                "Lam Son 719".to_string(),
                [Factions::ARVN, Factions::NVA, Factions::US, Factions::VC],
                None,
            )
            .into(),
        ),
        (
            75,
            RegularCard::new(
                75,
                "Sihanouk".to_string(),
                [Factions::ARVN, Factions::NVA, Factions::US, Factions::VC],
                None,
            )
            .into(),
        ),
        (
            76,
            RegularCard::new(
                76,
                "Annam".to_string(),
                [Factions::ARVN, Factions::NVA, Factions::VC, Factions::US],
                None,
            )
            .into(),
        ),
        (
            77,
            RegularCard::new(
                77,
                "Détente".to_string(),
                [Factions::ARVN, Factions::NVA, Factions::VC, Factions::US],
                None,
            )
            .into(),
        ),
        (
            78,
            RegularCard::new(
                78,
                "General Lansdale".to_string(),
                [Factions::ARVN, Factions::NVA, Factions::VC, Factions::US],
                None,
            )
            .into(),
        ),
        (
            79,
            RegularCard::new(
                79,
                "Henry Cabot Lodge".to_string(),
                [Factions::ARVN, Factions::NVA, Factions::VC, Factions::US],
                None,
            )
            .into(),
        ),
        (
            80,
            RegularCard::new(
                80,
                "Light at the End of the Tunnel".to_string(),
                [Factions::ARVN, Factions::NVA, Factions::VC, Factions::US],
                None,
            )
            .into(),
        ),
        (
            81,
            RegularCard::new(
                81,
                "CIDG".to_string(),
                [Factions::ARVN, Factions::VC, Factions::US, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            82,
            RegularCard::new(
                82,
                "Domino Theory".to_string(),
                [Factions::ARVN, Factions::VC, Factions::US, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            83,
            RegularCard::new(
                83,
                "Election".to_string(),
                [Factions::ARVN, Factions::VC, Factions::US, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            84,
            RegularCard::new(
                84,
                "To Quoc".to_string(),
                [Factions::ARVN, Factions::VC, Factions::US, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            85,
            RegularCard::new(
                85,
                "USAID".to_string(),
                [Factions::ARVN, Factions::VC, Factions::US, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            86,
            RegularCard::new(
                86,
                "Mandate of Heaven".to_string(),
                [Factions::ARVN, Factions::VC, Factions::NVA, Factions::US],
                Some(Factions::ARVN),
            )
            .into(),
        ),
        (
            87,
            RegularCard::new(
                87,
                "Nguyen Chanh Thi".to_string(),
                [Factions::ARVN, Factions::VC, Factions::NVA, Factions::US],
                None,
            )
            .into(),
        ),
        (
            88,
            RegularCard::new(
                88,
                "Phan Quang Dan".to_string(),
                [Factions::ARVN, Factions::VC, Factions::NVA, Factions::US],
                None,
            )
            .into(),
        ),
        (
            89,
            RegularCard::new(
                89,
                "Tam Chau".to_string(),
                [Factions::ARVN, Factions::VC, Factions::NVA, Factions::US],
                None,
            )
            .into(),
        ),
        (
            90,
            RegularCard::new(
                90,
                "Walt Rostow".to_string(),
                [Factions::ARVN, Factions::VC, Factions::NVA, Factions::US],
                None,
            )
            .into(),
        ),
        (
            91,
            RegularCard::new(
                91,
                "Bob Hope".to_string(),
                [Factions::VC, Factions::US, Factions::NVA, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            92,
            RegularCard::new(
                92,
                "SEALORDS".to_string(),
                [Factions::VC, Factions::US, Factions::NVA, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            93,
            RegularCard::new(
                93,
                "Senator Fulbright".to_string(),
                [Factions::VC, Factions::US, Factions::NVA, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            94,
            RegularCard::new(
                94,
                "Tunnel Rats".to_string(),
                [Factions::VC, Factions::US, Factions::NVA, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            95,
            RegularCard::new(
                95,
                "Westmoreland".to_string(),
                [Factions::VC, Factions::US, Factions::NVA, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            96,
            RegularCard::new(
                96,
                "APC".to_string(),
                [Factions::VC, Factions::US, Factions::ARVN, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            97,
            RegularCard::new(
                97,
                "Brinks Hotel".to_string(),
                [Factions::VC, Factions::US, Factions::ARVN, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            98,
            RegularCard::new(
                98,
                "Long Tan".to_string(),
                [Factions::VC, Factions::US, Factions::ARVN, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            99,
            RegularCard::new(
                99,
                "Masher/White Wing".to_string(),
                [Factions::VC, Factions::US, Factions::ARVN, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            100,
            RegularCard::new(
                100,
                "Rach Ba Rai".to_string(),
                [Factions::VC, Factions::US, Factions::ARVN, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            101,
            RegularCard::new(
                101,
                "Booby Traps".to_string(),
                [Factions::VC, Factions::NVA, Factions::US, Factions::ARVN],
                Some(Factions::VC),
            )
            .into(),
        ),
        (
            102,
            RegularCard::new(
                102,
                "Cu Chi".to_string(),
                [Factions::VC, Factions::NVA, Factions::US, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            103,
            RegularCard::new(
                103,
                "Kent State".to_string(),
                [Factions::VC, Factions::NVA, Factions::US, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            104,
            RegularCard::new(
                104,
                "Main Force Bns".to_string(),
                [Factions::VC, Factions::NVA, Factions::US, Factions::ARVN],
                Some(Factions::VC),
            )
            .into(),
        ),
        (
            105,
            RegularCard::new(
                105,
                "Rural Pressure".to_string(),
                [Factions::VC, Factions::NVA, Factions::US, Factions::ARVN],
                None,
            )
            .into(),
        ),
        (
            106,
            RegularCard::new(
                106,
                "Binh Duong".to_string(),
                [Factions::VC, Factions::NVA, Factions::ARVN, Factions::US],
                None,
            )
            .into(),
        ),
        (
            107,
            RegularCard::new(
                107,
                "Burning Bonze".to_string(),
                [Factions::VC, Factions::NVA, Factions::ARVN, Factions::US],
                None,
            )
            .into(),
        ),
        (
            108,
            RegularCard::new(
                108,
                "Draft Dodgers".to_string(),
                [Factions::VC, Factions::NVA, Factions::ARVN, Factions::US],
                None,
            )
            .into(),
        ),
        (
            109,
            RegularCard::new(
                109,
                "Nguyen Huu Tho".to_string(),
                [Factions::VC, Factions::NVA, Factions::ARVN, Factions::US],
                None,
            )
            .into(),
        ),
        (
            110,
            RegularCard::new(
                110,
                "No Contact".to_string(),
                [Factions::VC, Factions::NVA, Factions::ARVN, Factions::US],
                None,
            )
            .into(),
        ),
        (
            111,
            RegularCard::new(
                111,
                "Agent Orange".to_string(),
                [Factions::VC, Factions::ARVN, Factions::US, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            112,
            RegularCard::new(
                112,
                "Colonel Chau".to_string(),
                [Factions::VC, Factions::ARVN, Factions::US, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            113,
            RegularCard::new(
                113,
                "Ruff Puff".to_string(),
                [Factions::VC, Factions::ARVN, Factions::US, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            114,
            RegularCard::new(
                114,
                "Tri Quang".to_string(),
                [Factions::VC, Factions::ARVN, Factions::US, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            115,
            RegularCard::new(
                115,
                "Typhoon Kate".to_string(),
                [Factions::VC, Factions::ARVN, Factions::US, Factions::NVA],
                None,
            )
            .into(),
        ),
        (
            116,
            RegularCard::new(
                116,
                "Cadres".to_string(),
                [Factions::VC, Factions::ARVN, Factions::NVA, Factions::US],
                Some(Factions::VC),
            )
            .into(),
        ),
        (
            117,
            RegularCard::new(
                117,
                "Corps Commanders".to_string(),
                [Factions::VC, Factions::ARVN, Factions::NVA, Factions::US],
                None,
            )
            .into(),
        ),
        (
            118,
            RegularCard::new(
                118,
                "Korean War Arms".to_string(),
                [Factions::VC, Factions::ARVN, Factions::NVA, Factions::US],
                None,
            )
            .into(),
        ),
        (
            119,
            RegularCard::new(
                119,
                "My Lai".to_string(),
                [Factions::VC, Factions::ARVN, Factions::NVA, Factions::US],
                None,
            )
            .into(),
        ),
        (
            120,
            RegularCard::new(
                120,
                "US Press Corps".to_string(),
                [Factions::VC, Factions::ARVN, Factions::NVA, Factions::US],
                None,
            )
            .into(),
        ),
        (
            121,
            PivotalEventCard::new(
                121,
                "Linebacker II".to_string(),
                [Factions::US, Factions::ARVN, Factions::VC, Factions::NVA],
            )
            .into(),
        ),
        (
            122,
            PivotalEventCard::new(
                122,
                "Easter Offensive".to_string(),
                [Factions::NVA, Factions::VC, Factions::ARVN, Factions::US],
            )
            .into(),
        ),
        (
            123,
            PivotalEventCard::new(
                123,
                "Vietnamization".to_string(),
                [Factions::ARVN, Factions::US, Factions::NVA, Factions::VC],
            )
            .into(),
        ),
        (
            124,
            PivotalEventCard::new(
                124,
                "Tet Offensive".to_string(),
                [Factions::VC, Factions::NVA, Factions::US, Factions::ARVN],
            )
            .into(),
        ),
        (125, CoupCard::new(125, "Nguyen Khanh".to_string()).into()),
        (126, CoupCard::new(126, "Young Turks".to_string()).into()),
        (127, CoupCard::new(127, "Nguyen Cao Ky".to_string()).into()),
        (
            128,
            CoupCard::new(128, "Nguyen Van Thieu".to_string()).into(),
        ),
        (129, CoupCard::new(129, "Failed Attempt".to_string()).into()),
        (130, CoupCard::new(130, "Failed Attempt".to_string()).into()),
    ]
    .iter()
    .cloned()
    .collect()
}
