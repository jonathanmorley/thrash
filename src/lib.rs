#[macro_use]
extern crate failure;
extern crate itertools;
#[macro_use]
extern crate log;
#[macro_use]
extern crate percent_encoding;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rayon;
extern crate serde_json;
extern crate url;

pub mod auth;
pub mod client;
pub mod file;
pub mod group;
pub mod merging;
pub mod project;
pub mod repository;
pub mod user;
