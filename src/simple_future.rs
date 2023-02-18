use std::{future::Future, task::Poll, time::Instant, thread};

pub struct Delay {
    pub when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if Instant::now() > self.when {
            println!("Hello, World!");
            Poll::Ready("done")
        } else {
            let waker = cx.waker().clone();
            let when = self.when;

            // Spawn a timer thread
            thread::spawn(move || {
                let now = Instant::now();
                if now < when {
                    thread::sleep(when - now);
                }
                // The calling task is notified when the
                // duration has elapsed
                waker.wake();
            });

            Poll::Pending
        }
    }
}
