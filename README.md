Without AsyncDrop, awaiting at item being dropped is non-trivial.

This wrapper allows awaiting it, and can therein be used to perform async clean-up on the contents
of an Arc or similar, when it is no longer in use.

```rust
use dropbear::DropBear;
let (bearer, notify) = DropBear::new(x.clone());
drop(bearer);
assert_eq!(notify.await.unwrap(), x); // => true
```

```rust
use dropbear::DropBear;
let (bearer, notify) = DropBear::new(item);
let arc = Arc::new(bearer);
// Pass the arc elsewhere, handing ownership over
// Or clone it elsewhere, do some work here, then drop your copy
{
    // ...
}
// Receive the original item once the last Arc instance is dropped
let original_item = notify.await.unwrap();
// ... and perform async cleanup on the contents
perform_cleanup(original_item).await;
```
