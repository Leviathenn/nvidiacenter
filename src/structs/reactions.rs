/**
 * @author Leviathenn
 */

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Reactions {
    total_count: u32,
    #[serde(rename = "+1")]
    plus_one: u32,
    #[serde(rename = "-1")]
    minus_one: u32,
    laugh: u32,
    hooray: u32,
    confused: u32,
    heart: u32,
    rocket: u32,
    eyes: u32,
}