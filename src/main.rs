use std::fs;
use badgelib::{Badge, Color};

fn main() {

  // Generate "NO AI" SVG file.
  let badge = Badge::default()
    .for_version("version", "1.0.0")
    .label("NO")
    .value("AI")
    .label_color(Color::Orange)
    .value_color(Color::White)
    .to_svg();

  fs::write("./badge.svg", badge).expect("");
}
