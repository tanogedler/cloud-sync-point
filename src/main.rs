use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{watch, Mutex};
use tokio::time::{self, Duration, Instant};
use warp::Filter;

type SharedState = Arc<Mutex<HashMap<String, (watch::Sender<()>, Instant)>>>;

#[tokio::main]
async fn main() {
    /*
    The main function starts a web server that listens on port 3030.
    The server has a single endpoint /wait-for-second-party/:unique_id that accepts POST requests.
    */
    println!("Service Start ");
    println!("Awaiting for parties to connect.");

    let state = Arc::new(Mutex::new(HashMap::new()));

    let sync_state = state.clone();
    let sync_endpoint = warp::path!("wait-for-second-party" / String)
        .and(warp::post())
        .and(with_state(sync_state))
        .and_then(sync_handler);

    warp::serve(sync_endpoint).run(([0, 0, 0, 0], 3030)).await;

    println!("Web server stopped");
}

fn with_state(
    state: SharedState,
) -> impl Filter<Extract = (SharedState,), Error = std::convert::Infallible> + Clone {
    /*
    The with_state function creates a warp filter that injects the shared state into the request handler.
    */
    warp::any().map(move || state.clone())
}

async fn sync_handler(
    unique_id: String,
    state: SharedState,
) -> Result<impl warp::Reply, warp::Rejection> {
    /*
    The sync_handler function is the request handler for the /wait-for-second-party/:unique_id endpoint.
    It waits for two parties to connect with the same unique_id.
    If the second party connects within 10 seconds, it responds with "Second party arrived".
    If the second party doesn't connect within 10 seconds, it responds with "Timeout".
    */
    const TIMEOUT: u64 = 10; // Timeout in seconds

    println!("Received request for unique ID: {}", unique_id);

    let (tx, mut rx) = watch::channel(());
    let now = Instant::now();

    let mut state = state.lock().await;
    if let Some((existing_tx, _)) = state.remove(&unique_id) {
        // Second party arrived, notify the first party
        println!("Second party arrived for unique ID: {}", unique_id);
        drop(state);
        existing_tx.send(()).unwrap();
        Ok(warp::reply::html("Second party arrived"))
    } else {
        // First party arrives, wait for the second party or timeout
        println!("First party arrived for unique ID: {}", unique_id);
        state.insert(unique_id.clone(), (tx, now));
        drop(state.clone());

        let timeout = now + Duration::from_secs(TIMEOUT);
        tokio::select! {
        _ = rx.changed() => {
            // Second party has arrived
            println!("Second party arrived for unique ID: {}", unique_id);
            Ok(warp::reply::html("Second party arrived"))
        }
        _ = time::sleep_until(timeout) => {
            // Timeout occurred
            println!("Timeout occurred for unique ID: {}", unique_id);
            state.remove(&unique_id);
            Ok(warp::reply::html("Timeout"))
        }
        }
    }
}
