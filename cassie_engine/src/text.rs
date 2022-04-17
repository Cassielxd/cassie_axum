use cassie_engine::OpError;
use cassie_engine::*;
use cassie_engine::{error::AnyError, op, Extension, JsRuntime, RuntimeOptions};

#[op]
fn op_sum(nums: Vec<f64>) -> Result<f64, AnyError> {
    // Sum inputs
    let sum = nums.iter().fold(0.0, |a, v| a + v);
    // return as a Result<f64, AnyError>
    Ok(sum)
}

fn test() {
    // Build a cassie_engine::Extension providing custom ops
    let ext = Extension::builder()
        .ops(vec![
            // An op for summing an array of numbers
            // The op-layer automatically deserializes inputs
            // and serializes the returned Result & value
            op_sum::decl(),
        ])
        .build();

    // Initialize a runtime instance
    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![ext],
        ..Default::default()
    });

    // Now we see how to invoke the op we just defined. The runtime automatically
    // contains a Cassie.core object with several functions for interacting with it.
    // You can find its definition in core.js.
    runtime
        .execute_script(
            "<usage>",
            r#"

function print(value) {
  Cassie.core.print(value.toString()+"\n");
}

const arr = [1, 2, 3];
print("The sum of");
print(arr);
print("is");
print(Cassie.core.opSync('op_sum', arr));

// And incorrect usage
try {
  print(Cassie.core.opSync('op_sum', 0));
} catch(e) {
  print('Exception:');
  print(e);
}
"#,
        )
        .unwrap();
}
