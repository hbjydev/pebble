use axum::{http::StatusCode, response::IntoResponse};

const ROOT_STR: &'static str = "
            _     _     _      
           | |   | |   | |     
 _ __   ___| |__ | |__ | | ___ 
| '_ \\ / _ \\ '_ \\| '_ \\| |/ _ \\
| |_) |  __/ |_) | |_) | |  __/
| .__/ \\___|_.__/|_.__/|_|\\___|
| |                            
|_|                            

you've reached hayden's pds. he wrote it himself in
rust, but it's probably not the _best_ implementation
of a pds out there.

github   https://github.com/hbjydev/pebble
bsky     @hayden.moe
";

pub async fn handler() -> impl IntoResponse {
    (StatusCode::OK, ROOT_STR)
}
