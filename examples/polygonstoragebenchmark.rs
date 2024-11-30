#[allow(dead_code)]
extern crate oxygenrustengine as oe;

fn main(){
    use oe::oe::types::polygonstorage::*;
    test_dynamic_polygon_storage_large(10000);
    test_dynamic_polygon_storage_large(100000);
    test_dynamic_polygon_storage_large(400000);
}