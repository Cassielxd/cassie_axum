use rhai::{Engine, EvalAltResult};

pub fn main() -> Result<(), Box<EvalAltResult>>                          
{
    let engine = Engine::new();
    let script = "print(40 + 2);";
    engine.run(script)?;
    Ok(())
}
