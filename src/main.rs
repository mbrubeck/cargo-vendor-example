use smallvec::SmallVec;

fn main() {
    let v: SmallVec<[u8; 1]> = SmallVec::new();
    dbg!(v);
}
