/**
 * @author Leviathenn
 */

use serde::Deserialize;

use super::{assets::Asset, author::Author, reactions::Reactions};

#[derive(Deserialize)]
pub struct Release {
    pub url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub html_url: String,
    pub id: i32,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub draft: bool,
    pub author: Author,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub assets: Vec<Asset>,
    pub tarball_url: String,
    pub zipball_url: String,
    pub body: String,
    pub reactions: Reactions,
    pub mentions_count: i32,
    pub discussion_url: String,
    pub state: String,
    pub body_html: String,
}