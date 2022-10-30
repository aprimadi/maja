use macroquad::texture::{load_texture, Texture2D};

pub struct Assets {
    pub wraith1: Texture2D,
}

pub async fn load_assets() -> Assets {
    let wraith1 = load_texture("assets/wraith-1/png-sequences/idle/000.png").await.unwrap();

    Assets {
        wraith1,
    }
}

