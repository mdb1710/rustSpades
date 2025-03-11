use warp::Filter;
use tokio::sync::mpsc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let games: Arc<Mutex<HashMap<String, Game>>> = Arc::new(Mutex::new(HashMap::new()));

    let game_routes = warp::path("game")
        .and(warp::ws())
        .and(with_games(games.clone()))
        .map(|ws: warp::ws::Ws, games| {
            ws.on_upgrade(move |socket| handle_connection(socket, games))
        });

    warp::serve(game_routes)
        .run(([127, 0, 0, 1], 8000))
        .await;
}
