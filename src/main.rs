use terra::TerraType;

fn main() -> std::io::Result<()> {
    let mut app = terra::Terra::new(TerraType::Turttle);

    app.run()
}
