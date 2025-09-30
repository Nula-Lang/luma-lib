mod lib;
mod example;

use example::ExampleModel;
use lib::Program;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = ExampleModel::new();
    let mut program = Program::new(model);
    program.run()?;
    Ok(())
}
