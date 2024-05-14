use macroquad::texture::{load_texture, Texture2D};

#[derive(Debug, PartialEq)]
pub struct GameTextures {
    pub bomb: Texture2D,
    pub flag: Texture2D,
}

impl GameTextures {
    pub async fn new() -> GameTextures {
        let bomb: Texture2D = load_texture("textures/bomb.png").await.unwrap();
        let flag: Texture2D = load_texture("textures/flag.png").await.unwrap();

        GameTextures { bomb, flag }
    }
}
