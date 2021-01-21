use dropbear::DropBear;

#[tokio::test]
async fn drop_notifications() {
  let x = ();
  let (container, notify) = DropBear::new(&x as *const ());
  assert_eq!(*container, &x as *const ());
  drop(container);
  let res = notify.await.unwrap();
  assert_eq!(res, &x as *const _);
}

#[tokio::test]
async fn into_inner_notifies_abort() {
  let x = ();
  let (container, notify) = DropBear::new(&x as *const ());
  assert_eq!(container.into_inner(), &x as *const _);
  // We intentionally avoid calling drop here, to encode that into_inner must consume the value
  notify.await.err().unwrap();
}

#[tokio::test]
async fn drop_concurrent_with_await() {
  let (container, notify) = DropBear::new(());
  futures::future::join(notify, async move {
    tokio::task::yield_now().await;
    drop(container);
    ()
  })
  .await
  .0
  .unwrap();
}
