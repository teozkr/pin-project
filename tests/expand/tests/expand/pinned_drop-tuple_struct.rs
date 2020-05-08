use pin_project::{pin_project, pinned_drop};
use std::pin::Pin;

#[pin_project(PinnedDrop)]
struct TupleStruct<T, U>(#[pin] T, U);

#[pinned_drop]
impl<T, U> PinnedDrop for TupleStruct<T, U> {
    fn drop(self: Pin<&mut Self>) {
        let _this = self;
    }
}

fn main() {}
