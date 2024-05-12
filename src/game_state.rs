#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameState {
    NotStarted,
    Playing,
    GameOver,
    GameWon,
}
