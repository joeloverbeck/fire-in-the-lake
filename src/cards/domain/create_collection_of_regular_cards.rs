use game_definitions::factions::Factions;
use std::collections::HashMap;

pub fn create_collection_of_regular_cards() -> HashMap<u8, (String, [Factions; 4])> {
    [
        (
            1,
            (
                "Gulf of Tonkin".to_string(),
                [Factions::US, Factions::NVA, Factions::ARVN, Factions::VC],
            ),
        ),
        (
            2,
            (
                "Kissinger".to_string(),
                [Factions::US, Factions::NVA, Factions::ARVN, Factions::VC],
            ),
        ),
        (
            3,
            (
                "Peace Talks".to_string(),
                [Factions::US, Factions::NVA, Factions::ARVN, Factions::VC],
            ),
        ),
        (
            4,
            (
                "Top Gun".to_string(),
                [Factions::US, Factions::NVA, Factions::ARVN, Factions::VC],
            ),
        ),
        (
            5,
            (
                "Wild Weasels".to_string(),
                [Factions::US, Factions::NVA, Factions::ARVN, Factions::VC],
            ),
        ),
        (
            6,
            (
                "Aces".to_string(),
                [Factions::US, Factions::NVA, Factions::VC, Factions::ARVN],
            ),
        ),
        (
            7,
            (
                "ADSID".to_string(),
                [Factions::US, Factions::NVA, Factions::VC, Factions::ARVN],
            ),
        ),
        (
            8,
            (
                "Arc Light".to_string(),
                [Factions::US, Factions::NVA, Factions::VC, Factions::ARVN],
            ),
        ),
        (
            9,
            (
                "Psychedelic Cookie".to_string(),
                [Factions::US, Factions::NVA, Factions::VC, Factions::ARVN],
            ),
        ),
        (
            10,
            (
                "Rolling Thunder".to_string(),
                [Factions::US, Factions::NVA, Factions::VC, Factions::ARVN],
            ),
        ),
        (
            11,
            (
                "Abrams".to_string(),
                [Factions::US, Factions::ARVN, Factions::NVA, Factions::VC],
            ),
        ),
        (
            12,
            (
                "Capt Buck Adams".to_string(),
                [Factions::US, Factions::ARVN, Factions::NVA, Factions::VC],
            ),
        ),
        (
            13,
            (
                "Cobras".to_string(),
                [Factions::US, Factions::ARVN, Factions::NVA, Factions::VC],
            ),
        ),
        (
            14,
            (
                "M-48 Patton".to_string(),
                [Factions::US, Factions::ARVN, Factions::NVA, Factions::VC],
            ),
        ),
        (
            15,
            (
                "Medevac".to_string(),
                [Factions::US, Factions::ARVN, Factions::NVA, Factions::VC],
            ),
        ),
        (
            16,
            (
                "Blowtorch Komer".to_string(),
                [Factions::US, Factions::ARVN, Factions::VC, Factions::NVA],
            ),
        ),
        (
            17,
            (
                "Claymores".to_string(),
                [Factions::US, Factions::ARVN, Factions::VC, Factions::NVA],
            ),
        ),
        (
            18,
            (
                "Combined Action Platoons".to_string(),
                [Factions::US, Factions::ARVN, Factions::VC, Factions::NVA],
            ),
        ),
        (
            19,
            (
                "CORDS".to_string(),
                [Factions::US, Factions::ARVN, Factions::VC, Factions::NVA],
            ),
        ),
        (
            20,
            (
                "Laser Guided Bombs".to_string(),
                [Factions::US, Factions::ARVN, Factions::VC, Factions::NVA],
            ),
        ),
        (
            21,
            (
                "Americal".to_string(),
                [Factions::US, Factions::VC, Factions::NVA, Factions::ARVN],
            ),
        ),
        (
            22,
            (
                "Da Nang".to_string(),
                [Factions::US, Factions::VC, Factions::NVA, Factions::ARVN],
            ),
        ),
        (
            23,
            (
                "Operation Attleboro".to_string(),
                [Factions::US, Factions::VC, Factions::NVA, Factions::ARVN],
            ),
        ),
        (
            24,
            (
                "Operation Starlite".to_string(),
                [Factions::US, Factions::VC, Factions::NVA, Factions::ARVN],
            ),
        ),
        (
            25,
            (
                "TF-116 Riverines".to_string(),
                [Factions::US, Factions::VC, Factions::NVA, Factions::ARVN],
            ),
        ),
        (
            26,
            (
                "LRRP".to_string(),
                [Factions::US, Factions::VC, Factions::ARVN, Factions::NVA],
            ),
        ),
        (
            27,
            (
                "Phoenix Program".to_string(),
                [Factions::US, Factions::VC, Factions::ARVN, Factions::NVA],
            ),
        ),
        (
            28,
            (
                "Search and Destroy".to_string(),
                [Factions::US, Factions::VC, Factions::ARVN, Factions::NVA],
            ),
        ),
        (
            29,
            (
                "Tribesmen".to_string(),
                [Factions::US, Factions::VC, Factions::ARVN, Factions::NVA],
            ),
        ),
        (
            30,
            (
                "USS New Jersey".to_string(),
                [Factions::US, Factions::VC, Factions::ARVN, Factions::NVA],
            ),
        ),
        (
            31,
            (
                "AAA".to_string(),
                [Factions::NVA, Factions::US, Factions::ARVN, Factions::VC],
            ),
        ),
        (
            32,
            (
                "Long Range Guns".to_string(),
                [Factions::NVA, Factions::US, Factions::ARVN, Factions::VC],
            ),
        ),
        (
            33,
            (
                "MiGs".to_string(),
                [Factions::NVA, Factions::US, Factions::ARVN, Factions::VC],
            ),
        ),
        (
            34,
            (
                "SA-2s".to_string(),
                [Factions::NVA, Factions::US, Factions::ARVN, Factions::VC],
            ),
        ),
        (
            35,
            (
                "Thanh Hoa".to_string(),
                [Factions::NVA, Factions::US, Factions::ARVN, Factions::VC],
            ),
        ),
        (
            36,
            (
                "Hamburger Hill".to_string(),
                [Factions::NVA, Factions::US, Factions::VC, Factions::ARVN],
            ),
        ),
        (
            37,
            (
                "Khe Sanh".to_string(),
                [Factions::NVA, Factions::US, Factions::VC, Factions::ARVN],
            ),
        ),
        (
            38,
            (
                "McNamara Line".to_string(),
                [Factions::NVA, Factions::US, Factions::VC, Factions::ARVN],
            ),
        ),
        (
            39,
            (
                "Oriskany".to_string(),
                [Factions::NVA, Factions::US, Factions::VC, Factions::ARVN],
            ),
        ),
        (
            40,
            (
                "PoWs".to_string(),
                [Factions::NVA, Factions::US, Factions::VC, Factions::ARVN],
            ),
        ),
        (
            41,
            (
                "Bombing Pause".to_string(),
                [Factions::NVA, Factions::ARVN, Factions::US, Factions::VC],
            ),
        ),
        (
            42,
            (
                "Chou En Lai".to_string(),
                [Factions::NVA, Factions::ARVN, Factions::US, Factions::VC],
            ),
        ),
        (
            43,
            (
                "Economic Aid".to_string(),
                [Factions::NVA, Factions::ARVN, Factions::US, Factions::VC],
            ),
        ),
        (
            44,
            (
                "Ia Drang".to_string(),
                [Factions::NVA, Factions::ARVN, Factions::US, Factions::VC],
            ),
        ),
        (
            45,
            (
                "PT-76".to_string(),
                [Factions::NVA, Factions::ARVN, Factions::US, Factions::VC],
            ),
        ),
        (
            46,
            (
                "559th Transport Grp".to_string(),
                [Factions::NVA, Factions::ARVN, Factions::VC, Factions::US],
            ),
        ),
        (
            47,
            (
                "Chu Luc".to_string(),
                [Factions::NVA, Factions::ARVN, Factions::VC, Factions::US],
            ),
        ),
        (
            48,
            (
                "Nam Dong".to_string(),
                [Factions::NVA, Factions::ARVN, Factions::VC, Factions::US],
            ),
        ),
        (
            49,
            (
                "Russian Arms".to_string(),
                [Factions::NVA, Factions::ARVN, Factions::VC, Factions::US],
            ),
        ),
        (
            50,
            (
                "Uncle Ho".to_string(),
                [Factions::NVA, Factions::ARVN, Factions::VC, Factions::US],
            ),
        ),
        (
            51,
            (
                "301st Supply Bn".to_string(),
                [Factions::NVA, Factions::VC, Factions::US, Factions::ARVN],
            ),
        ),
        (
            52,
            (
                "RAND".to_string(),
                [Factions::NVA, Factions::VC, Factions::US, Factions::ARVN],
            ),
        ),
        (
            53,
            (
                "Sappers".to_string(),
                [Factions::NVA, Factions::VC, Factions::US, Factions::ARVN],
            ),
        ),
        (
            54,
            (
                "Son Tay".to_string(),
                [Factions::NVA, Factions::VC, Factions::US, Factions::ARVN],
            ),
        ),
        (
            55,
            (
                "Trucks".to_string(),
                [Factions::NVA, Factions::VC, Factions::US, Factions::ARVN],
            ),
        ),
        (
            56,
            (
                "Vo Nguyen Giap".to_string(),
                [Factions::NVA, Factions::VC, Factions::ARVN, Factions::US],
            ),
        ),
        (
            57,
            (
                "International Unrest".to_string(),
                [Factions::NVA, Factions::VC, Factions::ARVN, Factions::US],
            ),
        ),
        (
            58,
            (
                "Pathet Lao".to_string(),
                [Factions::NVA, Factions::VC, Factions::ARVN, Factions::US],
            ),
        ),
        (
            59,
            (
                "Plei Mei".to_string(),
                [Factions::NVA, Factions::VC, Factions::ARVN, Factions::US],
            ),
        ),
        (
            60,
            (
                "War Photographer".to_string(),
                [Factions::NVA, Factions::VC, Factions::ARVN, Factions::US],
            ),
        ),
        (
            61,
            (
                "Armored Cavalry".to_string(),
                [Factions::ARVN, Factions::US, Factions::NVA, Factions::VC],
            ),
        ),
        (
            62,
            (
                "Cambodian Civil War".to_string(),
                [Factions::ARVN, Factions::US, Factions::NVA, Factions::VC],
            ),
        ),
        (
            63,
            (
                "Fact Finding".to_string(),
                [Factions::ARVN, Factions::US, Factions::NVA, Factions::VC],
            ),
        ),
        (
            64,
            (
                "Honolulu Conference".to_string(),
                [Factions::ARVN, Factions::US, Factions::NVA, Factions::VC],
            ),
        ),
        (
            65,
            (
                "International Forces".to_string(),
                [Factions::ARVN, Factions::US, Factions::NVA, Factions::VC],
            ),
        ),
        (
            66,
            (
                "Ambassador Taylor".to_string(),
                [Factions::ARVN, Factions::US, Factions::VC, Factions::NVA],
            ),
        ),
        (
            67,
            (
                "Amphib Landing".to_string(),
                [Factions::ARVN, Factions::US, Factions::VC, Factions::NVA],
            ),
        ),
        (
            68,
            (
                "Green Berets".to_string(),
                [Factions::ARVN, Factions::US, Factions::VC, Factions::NVA],
            ),
        ),
        (
            69,
            (
                "MACV".to_string(),
                [Factions::ARVN, Factions::US, Factions::VC, Factions::NVA],
            ),
        ),
        (
            70,
            (
                "ROKs".to_string(),
                [Factions::ARVN, Factions::US, Factions::VC, Factions::NVA],
            ),
        ),
        (
            71,
            (
                "An Loc".to_string(),
                [Factions::ARVN, Factions::NVA, Factions::US, Factions::VC],
            ),
        ),
        (
            72,
            (
                "Body Count".to_string(),
                [Factions::ARVN, Factions::NVA, Factions::US, Factions::VC],
            ),
        ),
        (
            73,
            (
                "Great Society".to_string(),
                [Factions::ARVN, Factions::NVA, Factions::US, Factions::VC],
            ),
        ),
        (
            74,
            (
                "Lam Son 719".to_string(),
                [Factions::ARVN, Factions::NVA, Factions::US, Factions::VC],
            ),
        ),
        (
            75,
            (
                "Sihanouk".to_string(),
                [Factions::ARVN, Factions::NVA, Factions::US, Factions::VC],
            ),
        ),
        (
            76,
            (
                "Annam".to_string(),
                [Factions::ARVN, Factions::NVA, Factions::VC, Factions::US],
            ),
        ),
        (
            77,
            (
                "Détente".to_string(),
                [Factions::ARVN, Factions::NVA, Factions::VC, Factions::US],
            ),
        ),
        (
            78,
            (
                "General Lansdale".to_string(),
                [Factions::ARVN, Factions::NVA, Factions::VC, Factions::US],
            ),
        ),
        (
            79,
            (
                "Henry Cabot Lodge".to_string(),
                [Factions::ARVN, Factions::NVA, Factions::VC, Factions::US],
            ),
        ),
        (
            80,
            (
                "Light at the End of the Tunnel".to_string(),
                [Factions::ARVN, Factions::NVA, Factions::VC, Factions::US],
            ),
        ),
        (
            81,
            (
                "CIDG".to_string(),
                [Factions::ARVN, Factions::VC, Factions::US, Factions::NVA],
            ),
        ),
        (
            82,
            (
                "Domino Theory".to_string(),
                [Factions::ARVN, Factions::VC, Factions::US, Factions::NVA],
            ),
        ),
        (
            83,
            (
                "Election".to_string(),
                [Factions::ARVN, Factions::VC, Factions::US, Factions::NVA],
            ),
        ),
        (
            84,
            (
                "To Quoc".to_string(),
                [Factions::ARVN, Factions::VC, Factions::US, Factions::NVA],
            ),
        ),
        (
            85,
            (
                "USAID".to_string(),
                [Factions::ARVN, Factions::VC, Factions::US, Factions::NVA],
            ),
        ),
        (
            86,
            (
                "Mandate of Heaven".to_string(),
                [Factions::ARVN, Factions::VC, Factions::NVA, Factions::US],
            ),
        ),
        (
            87,
            (
                "Nguyen Chanh Thi".to_string(),
                [Factions::ARVN, Factions::VC, Factions::NVA, Factions::US],
            ),
        ),
        (
            88,
            (
                "Phan Quang Dan".to_string(),
                [Factions::ARVN, Factions::VC, Factions::NVA, Factions::US],
            ),
        ),
        (
            89,
            (
                "Tam Chau".to_string(),
                [Factions::ARVN, Factions::VC, Factions::NVA, Factions::US],
            ),
        ),
        (
            90,
            (
                "Walt Rostow".to_string(),
                [Factions::ARVN, Factions::VC, Factions::NVA, Factions::US],
            ),
        ),
        (
            91,
            (
                "Bob Hope".to_string(),
                [Factions::VC, Factions::US, Factions::NVA, Factions::ARVN],
            ),
        ),
        (
            92,
            (
                "SEALORDS".to_string(),
                [Factions::VC, Factions::US, Factions::NVA, Factions::ARVN],
            ),
        ),
        (
            93,
            (
                "Senator Fulbright".to_string(),
                [Factions::VC, Factions::US, Factions::NVA, Factions::ARVN],
            ),
        ),
        (
            94,
            (
                "Tunnel Rats".to_string(),
                [Factions::VC, Factions::US, Factions::NVA, Factions::ARVN],
            ),
        ),
        (
            95,
            (
                "Westmoreland".to_string(),
                [Factions::VC, Factions::US, Factions::NVA, Factions::ARVN],
            ),
        ),
        (
            96,
            (
                "APC".to_string(),
                [Factions::VC, Factions::US, Factions::ARVN, Factions::NVA],
            ),
        ),
        (
            97,
            (
                "Brinks Hotel".to_string(),
                [Factions::VC, Factions::US, Factions::ARVN, Factions::NVA],
            ),
        ),
        (
            98,
            (
                "Long Tan".to_string(),
                [Factions::VC, Factions::US, Factions::ARVN, Factions::NVA],
            ),
        ),
        (
            99,
            (
                "Masher/White Wing".to_string(),
                [Factions::VC, Factions::US, Factions::ARVN, Factions::NVA],
            ),
        ),
        (
            100,
            (
                "Rach Ba Rai".to_string(),
                [Factions::VC, Factions::US, Factions::ARVN, Factions::NVA],
            ),
        ),
        (
            101,
            (
                "Booby Traps".to_string(),
                [Factions::VC, Factions::NVA, Factions::US, Factions::ARVN],
            ),
        ),
        (
            102,
            (
                "Cu Chi".to_string(),
                [Factions::VC, Factions::NVA, Factions::US, Factions::ARVN],
            ),
        ),
        (
            103,
            (
                "Kent State".to_string(),
                [Factions::VC, Factions::NVA, Factions::US, Factions::ARVN],
            ),
        ),
        (
            104,
            (
                "Main Force Bns".to_string(),
                [Factions::VC, Factions::NVA, Factions::US, Factions::ARVN],
            ),
        ),
        (
            105,
            (
                "Rural Pressure".to_string(),
                [Factions::VC, Factions::NVA, Factions::US, Factions::ARVN],
            ),
        ),
        (
            106,
            (
                "Binh Duong".to_string(),
                [Factions::VC, Factions::NVA, Factions::ARVN, Factions::US],
            ),
        ),
        (
            107,
            (
                "Burning Bonze".to_string(),
                [Factions::VC, Factions::NVA, Factions::ARVN, Factions::US],
            ),
        ),
        (
            108,
            (
                "Draft Dodgers".to_string(),
                [Factions::VC, Factions::NVA, Factions::ARVN, Factions::US],
            ),
        ),
        (
            109,
            (
                "Nguyen Huu Tho".to_string(),
                [Factions::VC, Factions::NVA, Factions::ARVN, Factions::US],
            ),
        ),
        (
            110,
            (
                "No Contact".to_string(),
                [Factions::VC, Factions::NVA, Factions::ARVN, Factions::US],
            ),
        ),
        (
            111,
            (
                "Agent Orange".to_string(),
                [Factions::VC, Factions::ARVN, Factions::US, Factions::NVA],
            ),
        ),
        (
            112,
            (
                "Colonel Chau".to_string(),
                [Factions::VC, Factions::ARVN, Factions::US, Factions::NVA],
            ),
        ),
        (
            113,
            (
                "Ruff Puff".to_string(),
                [Factions::VC, Factions::ARVN, Factions::US, Factions::NVA],
            ),
        ),
        (
            114,
            (
                "Tri Quang".to_string(),
                [Factions::VC, Factions::ARVN, Factions::US, Factions::NVA],
            ),
        ),
        (
            115,
            (
                "Typhoon Kate".to_string(),
                [Factions::VC, Factions::ARVN, Factions::US, Factions::NVA],
            ),
        ),
        (
            116,
            (
                "Cadres".to_string(),
                [Factions::VC, Factions::ARVN, Factions::NVA, Factions::US],
            ),
        ),
        (
            117,
            (
                "Corps Commanders".to_string(),
                [Factions::VC, Factions::ARVN, Factions::NVA, Factions::US],
            ),
        ),
        (
            118,
            (
                "Korean War Arms".to_string(),
                [Factions::VC, Factions::ARVN, Factions::NVA, Factions::US],
            ),
        ),
        (
            119,
            (
                "My Lai".to_string(),
                [Factions::VC, Factions::ARVN, Factions::NVA, Factions::US],
            ),
        ),
        (
            120,
            (
                "US Press Corps".to_string(),
                [Factions::VC, Factions::ARVN, Factions::NVA, Factions::US],
            ),
        ),
        (
            121,
            (
                "Linebacker II".to_string(),
                [Factions::US, Factions::ARVN, Factions::VC, Factions::NVA],
            ),
        ),
        (
            122,
            (
                "Easter Offensive".to_string(),
                [Factions::NVA, Factions::VC, Factions::ARVN, Factions::US],
            ),
        ),
        (
            123,
            (
                "Vietnamization".to_string(),
                [Factions::ARVN, Factions::US, Factions::NVA, Factions::VC],
            ),
        ),
        (
            124,
            (
                "Tet Offensive".to_string(),
                [Factions::VC, Factions::NVA, Factions::US, Factions::ARVN],
            ),
        ),
    ]
    .iter()
    .cloned()
    .collect()
}
