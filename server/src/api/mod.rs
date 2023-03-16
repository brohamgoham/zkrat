use serde::de::DeserializeOwned;
use warp::Filter;

//mod error;
//mod state;

pub mod routes;
//pub use error::handle_error;
//pub use state::{with_state, AppState};

// Decode request in 2 steps here and in api/routes/job.rs create_job

// reusable code; warp filter
pub fn json_body<T: DeserializeOwned + Send>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
