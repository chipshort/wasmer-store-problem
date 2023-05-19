use wasmer::{Engine, Module, Store};

fn main() {
    const CONTRACT: &[u8] = include_bytes!("../../test_wasm.wasm");
    // compile wasm and serialize
    let module = compile(CONTRACT);
    let serialized = module.serialize().unwrap();

    // create new instance from serialized module
    let (instance, mut store) = instantiate(serialized);

    // call function
    let func = instance.exports.get_function("test").unwrap();
    func.call(&mut store, &[wasmer::Value::I32(0)]).unwrap(); // <- HeapAccessOutOfBounds
}

pub fn compile(code: &[u8]) -> Module {
    let store = Store::new(wasmer::Singlepass::default());
    Module::new(&store, code).unwrap()
}

pub fn instantiate(serialized_module: bytes::Bytes) -> (Box<wasmer::Instance>, Store) {
    // deserialize module with new engine
    let engine = Engine::headless();
    let module = unsafe { Module::deserialize(&engine, serialized_module) }.unwrap();

    // create new store for running
    // note: if I use the same engine as above (`Store::new(engine)`), it seems to work
    let mut store = Store::new(Engine::headless());
    let wasmer_instance =
        Box::from(wasmer::Instance::new(&mut store, &module, &wasmer::Imports::new()).unwrap());

    (wasmer_instance, store)
}
