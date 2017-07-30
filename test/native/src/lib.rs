#[macro_use]
extern crate neon;
extern crate neon_serde;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use neon::vm::{Call, JsResult};
use neon::js::JsValue;

#[derive(Serialize)]
struct AnObject<'a> {
    a: u32,
    b: Vec<f64>,
    c: &'a str,
}


macro_rules! make_test {
    ($name:ident, $val:expr) => {
        fn $name(call: Call) -> JsResult<JsValue> {
            let scope = call.scope;
            let value = $val;

            let handle = neon_serde::to_value(&value, scope)?;
            Ok(handle)
        }
    };
}

make_test!(make_num_77, 77i32);
make_test!(make_num_32, 32u8);
make_test!(make_str_hello, "Hello World");
make_test!(make_num_array, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
make_test!(
    make_obj,
    AnObject {
        a: 1,
        b: vec![0.1f64, 1.1, 2.2, 3.3],
        c: "Hi",
    }
);

register_module!(m, {
    m.export("make_num_77", make_num_77)?;
    m.export("make_num_32", make_num_32)?;
    m.export("make_str_hello", make_str_hello)?;
    m.export("make_num_array", make_num_array)?;
    m.export("make_obj", make_obj)?;
    Ok(())
});