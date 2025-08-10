use wgpu_engine::run;
// mod vertex_handler;
// use vertex_handler::parse_json;

fn main() -> anyhow::Result<()> {
    run()?;
    // parse_json("src/vertices.json");
    Ok(())
}
