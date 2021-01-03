# Hypervolume

A Rust library for computing hypervolumes.

Implements the WFG algorithm in "A fast way of calculating exact hypervolumes." Evolutionary Computation, IEEE Transactions on 16.1 (2012): 86-95.`, While, Lyndon, Lucas Bradstreet, and Luigi Barone.

```rust
let pts = vec![vec![0.3, 0.5], vec![0.6, 0.2], vec![0.8, 0.7]];
let ref_pt = vec![1.0, 1.0];
let hv_computed = hypervolume::compute(&pts, &ref_pt);
```
