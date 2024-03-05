#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn explodes_in_debug() {
    use new_debug_unreachable::debug_unreachable;
    unsafe { debug_unreachable!() }
}

