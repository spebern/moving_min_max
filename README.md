# moving min max

<!-- cargo-sync-readme start -->

Keep track of the minimum or maximum value in a sliding window.

`moving min max` provides one data structure for keeping track of the
minimum value and one for keeping track of the maximum value in a sliding
window.

The algorithm is based on the description [here](https://stackoverflow.com/questions/4802038/implement-a-queue-in-which-push-rear-pop-front-and-get-min-are-all-consta).

The complexity of the operations are
- O(1) for getting the minimum/maximum
- O(1) for push
- amortized O(1) for pop

```rust
use moving_min_max::MovingMin;

let mut moving_min = MovingMin::<i32>::new();
moving_min.push(2);
moving_min.push(1);
moving_min.push(3);

assert_eq!(moving_min.min(), Some(&1));
assert_eq!(moving_min.pop(), Some(2));

assert_eq!(moving_min.min(), Some(&1));
assert_eq!(moving_min.pop(), Some(1));

assert_eq!(moving_min.min(), Some(&1));
assert_eq!(moving_min.pop(), Some(3));

assert_eq!(moving_min.min(), None);
assert_eq!(moving_min.pop(), None);
```

or

```rust
use moving_min_max::MovingMax;

let mut moving_max = MovingMax::<i32>::new();
moving_max.push(2);
moving_max.push(3);
moving_max.push(1);

assert_eq!(moving_max.max(), Some(&3));
assert_eq!(moving_max.pop(), Some(2));

assert_eq!(moving_max.max(), Some(&3));
assert_eq!(moving_max.pop(), Some(3));

assert_eq!(moving_max.max(), Some(&3));
assert_eq!(moving_max.pop(), Some(1));

assert_eq!(moving_max.max(), None);
assert_eq!(moving_max.pop(), None);
```

<!-- cargo-sync-readme end -->