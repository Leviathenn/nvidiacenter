/**
 * @author Leviathenn
 */

use serde::Deserialize;

use super::author::Author;

#[derive(Deserialize)]
pub struct Asset {
    url: String,
    id: i32,
    node_id: String,
    name: String,
    label: Option<String>,
    uploader: Author,
    content_type: String,
    state: String,
    size: i32,
    download_count: i32,
    created_at: String,
    updated_at: String,
    browser_download_url: String,
}