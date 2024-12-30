
pub mod fonts;
pub mod images;
mod sounds;

use tetra::audio::Sound;
use tetra::graphics::text::{Font, Text, VectorFontBuilder};
use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::Context;
pub struct Assets {
    bgm:Sound,
    soft_drop_fx: Sound,
    hard_drop_fx: Sound,
    line_clear_fx: Sound,
    game_over_fx: Sound,

    font_16: Font,
    font_36: Font,

    backdrop: Texture,
    block: Texture,
}

impl Assets {
    fn load(ctx: &mut Context) -> tetra::Result<Assets> {
        let font = VectorFontBuilder::new("./examples/resources/DejaVuSansMono.ttf")?;

        Ok(Assets {
            bgm: Sound::new("./examples/resources/bgm.ogg")?,
            soft_drop_fx: Sound::new("./examples/resources/softdrop.ogg")?,
            hard_drop_fx: Sound::new("./examples/resources/harddrop.ogg")?,
            line_clear_fx: Sound::new("./examples/resources/lineclear.ogg")?,
            game_over_fx: Sound::new("./examples/resources/gameover.ogg")?,

            font_16: font.with_size(ctx, 16.0)?,
            font_36: font.with_size(ctx, 36.0)?,

            backdrop: Texture::new(ctx, "./examples/resources/backdrop.png")?,
            block: Texture::new(ctx, "./examples/resources/block.png")?,
        })
    }
}