use vectorcade_core::{GameRng, GameRngExt, Xorshift64};

#[test]
fn deterministic() {
    let mut rng1 = Xorshift64::new(42);
    let mut rng2 = Xorshift64::new(42);

    for _ in 0..100 {
        assert_eq!(rng1.next_u64(), rng2.next_u64());
    }
}

#[test]
fn range_f32_bounds() {
    let mut rng = Xorshift64::new(123);
    for _ in 0..1000 {
        let v = rng.range_f32(-1.0, 1.0);
        assert!((-1.0..1.0).contains(&v));
    }
}

#[test]
fn range_i32_bounds() {
    let mut rng = Xorshift64::new(456);
    for _ in 0..1000 {
        let v = rng.range_i32(0, 10);
        assert!((0..10).contains(&v));
    }
}

#[test]
fn pick_index_works() {
    let mut rng = Xorshift64::new(789);
    for _ in 0..100 {
        let idx = rng.pick_index(5).unwrap();
        assert!(idx < 5);
    }
    assert!(rng.pick_index(0).is_none());
}

#[test]
fn pick_ext_works() {
    let mut rng = Xorshift64::new(789);
    let items = [1, 2, 3, 4, 5];
    for _ in 0..100 {
        let picked = rng.pick(&items).unwrap();
        assert!(items.contains(picked));
    }
}
