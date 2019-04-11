use fancy_slice::FancySlice;

#[test]
fn test() {
    let data = vec!(4, 1, 3);
    let fancy_slice = FancySlice::new(&data);
    assert_eq!(fancy_slice.u8(0), 4);
    assert_eq!(fancy_slice.u8(1), 1);
    assert_eq!(fancy_slice.u8(2), 3);
    assert_eq!(fancy_slice.u16_be(0), 0x0401);
    assert_eq!(fancy_slice.u16_be(1), 0x0103);

    let inner_fancy_slice = fancy_slice.relative_fancy_slice(1..);
    assert_eq!(inner_fancy_slice.u8(0), 1);
    assert_eq!(inner_fancy_slice.u8(1), 3);
    assert_eq!(inner_fancy_slice.u16_be(0), 0x0103);
}

