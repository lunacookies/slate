mod imp;
mod palette;

use mottle::theme::{ThemeBuilder, Type};
use std::io;

fn main() -> io::Result<()> {
    let palette = palette::Palette;

    let mut slate = ThemeBuilder::new("Slate".to_string(), Type::Dark);
    imp::add_rules(&mut slate, &palette);
    slate.build().save()?;

    Ok(())
}
