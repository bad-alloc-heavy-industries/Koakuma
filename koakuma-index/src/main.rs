// SPDX-License-Identifier: BSD-3-Clause
/* koakuma-index : Koakuma document index daemon


*/
#[allow(unused_imports)]

use tracing::{debug, error, info, span, warn, instrument};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "koakuma-index", about = "Koakuma DMS indexing daemon")]
struct Options {
    #[structopt(short = "c", long = "config", default_value = r"/etc/koakuma.conf")]
    config_file: PathBuf,
}

fn main() {
    /* Setup logging and filter stuff */
    let filter = EnvFilter::from_default_env()
        .add_directive(LevelFilter::INFO.into());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();


    let default_settings = koakuma_common::KoakumaSettings { ..Default::default() };
    let opts = Options::from_args();
    debug!("Reading configuration file {:#?}", opts.config_file);


    info!("Starting koakuma-index daemon");
}
