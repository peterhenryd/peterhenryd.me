use peterhenryd_me_lib::ConnectionPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: ConnectionPool
}