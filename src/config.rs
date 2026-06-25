use g5k::Site;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Bench<'a> {
    pub cpu: &'a str,
    pub site: Site,
    pub node: &'a str,
}

#[derive(Deserialize)]
pub struct Config<'a> {
    #[serde(borrow)]
    pub bench: Vec<Bench<'a>>,
}
