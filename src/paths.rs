// src/paths.rs
//
// path constants

use crate::die;
use lazy_static::lazy_static;
use std::env;
use std::path::PathBuf;

fn wrap(var: &str) -> PathBuf {
    PathBuf::from(env::var(var).unwrap_or_else(|_| die!("Set ${}", var)))
}

lazy_static! {
    pub static ref REPO: PathBuf = PathBuf::from(env::var("RIDREPO").unwrap());

    pub static ref RIDHOME: PathBuf = wrap("RIDHOME");
    pub static ref META: PathBuf = wrap("RIDMETA");
    pub static ref SOURCES: PathBuf = wrap("RIDSOURCES");
    pub static ref PKGSJSON: PathBuf = wrap("RIDPKGSJSON"); // !!
    pub static ref SETS: PathBuf = RIDHOME.join("sets");
    pub static ref BIN: PathBuf = RIDHOME.join("bin");
    // pub static ref PKGSJSON: PathBuf = RIDHOME.join(format!("pkgs/{}.json", REPO.display()));
    pub static ref TMPRID: PathBuf = wrap("RIDTMP");
    pub static ref BUILDING: PathBuf = wrap("RIDBUILDING");
    pub static ref EXTRACTION: PathBuf = wrap("RIDEXTRACTION");
    pub static ref TRASH: PathBuf = wrap("RIDTRASH");
    pub static ref DEST: PathBuf = wrap("RIDDEST");
    pub static ref FAILED: PathBuf = wrap("RIDFAILED");

}
