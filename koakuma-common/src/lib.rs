// SPDX-License-Identifier: BSD-3-Clause
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KoakumaIndexerSettings {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct KoakumaAPISettings {
    pub interface: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KoakumaDatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KoakumaSettings {
    /* Global settings for the API and Indexer */
    pub index_directory: PathBuf,

    pub database: KoakumaDatabaseSettings,

    /* Component specific settings */
    pub api: KoakumaAPISettings,
    pub indexer: KoakumaIndexerSettings,
}

impl Default for KoakumaSettings {
    fn default() -> KoakumaSettings {
        KoakumaSettings {
            /* Global settings for the API and Indexer */
            index_directory: PathBuf::from(r"/var/lib/koakuma/index"),

            database: KoakumaDatabaseSettings {
                username: "koakuma".to_string(),
                password: "koakuma".to_string(),
                host: "127.0.0.1".to_string(),
                name: "koakuma".to_string(),
            },

            /* Component specific settings */
            api: KoakumaAPISettings {
                interface: "127.0.0.1".to_string(),
                port: 8008,
            },

            indexer: KoakumaIndexerSettings {
                // path: PathBuf::from(r"."),
                // port: 10069,
                // interface: "127.0.0.1".to_string(),
            },



        }
    }
}
