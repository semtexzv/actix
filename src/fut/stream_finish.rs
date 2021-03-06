use std::future::Future;
use std::task::Poll;

use crate::actor::Actor;
use crate::fut::{ActorFuture, ActorStream};
use std::task;
use std::pin::Pin;

/// A combinator used to convert stream into a future, future resolves
/// when stream completes.
///
/// This structure is produced by the `ActorStream::finish` method.
#[pin_project]
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct StreamFinish<S>( #[pin] S);


pub fn new<S>(s: S) -> StreamFinish<S>
where
    S: ActorStream,
{
    StreamFinish(s)
}

impl<S> ActorFuture for StreamFinish<S>
where
    S: ActorStream,
{
    type Item = ();
    type Actor = S::Actor;

    fn poll(
        mut self: Pin<&mut Self>,
        act: &mut S::Actor,
        ctx: &mut <S::Actor as Actor>::Context,
        task : &mut task::Context<'_>
    ) -> Poll<()> {
        loop {
            match self.project().0.poll(act, ctx, task) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(None) => return Poll::Ready(()),
                Poll::Ready(Some(_)) => (),
            };
        }
    }
}
