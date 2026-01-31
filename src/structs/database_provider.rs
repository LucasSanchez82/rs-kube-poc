use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct DatabaseProvider<'a> {
    duration_min: u16,
    name: &'a str,
    password: &'a str,
    user: &'a str,
    address: Option<&'a str>,
}

// impl DatabaseProvider {}
