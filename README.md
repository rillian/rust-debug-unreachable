# new_debug_unreachable

> unreachable!() in debug, std::intrinsics::unreachable() in release.

This is a fork of [`debug_unreachable`](https://crates.io/crates/debug_unreachable).

## [Documentation](https://docs.rs/new_debug_unreachable)

## Usage

Use the crates.io repository; add this to your `Cargo.toml` along
with the rest of your dependencies:

```toml
[dependencies]
new_debug_unreachable = "2.0"
```

In your Rust code you can add debug-only assertions like this:

```rust
use new_debug_unreachable::debug_unreachable;

fn main() {
    if 0 > 100 {
        // Can't happen!
        unsafe { debug_unreachable!() }
    } else {
        println!("Good, 0 <= 100.");
    }
}
```

This will panic if the unexpected branch is actually taken in a
a debug build, so the failure can be analyzed. In a release build
the macro will allow silent failure, hinting to the compiler that
the branch is unlikely, optimizing for performance.

## Updating from earlier versions

The v1 releases of `new_debug_unreachable` tried to be a drop-in
replacement for the v0.1 `debug_unreachable` crate, exporting
the same name so only Cargo.toml needed to changed, not the
referring Rust code.

Since v2, this crate exports the `debug_unreachable!` macro under
its default crate namespace, which is less confusing. This means
that code upgrading will need to change statements like

    use debug_unreachable::debug_unreachable;

to

    use new_debug_unreachable::debug_unreachable;

when updating to v2 or later.

## Author

[Jonathan Reem](https://medium.com/@jreem) is the original author of debug-unreachable.

[Matt Brubeck](https://limpet.net/mbrubeck/) is the maintainer of this fork.

## License

MIT
