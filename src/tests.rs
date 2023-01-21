use crate::mm::{
    frame_allocator::frame_allocator_test,
    heap_allocator::heap_test,
    memory_set::{framed_map_test, identical_map_test},
};

#[cfg(test)]
fn tests() {
    // heap
    heap_test();
    // frame
    frame_allocator_test();
    // mm
    identical_map_test();
    framed_map_test();
}


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
