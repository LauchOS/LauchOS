use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use crate::{println, print, sys_programs::shell::shell};
use core::{pin::Pin, task::{Poll, Context}};
use futures_util::{stream::{Stream, StreamExt}, task::AtomicWaker};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

/// Queue for incoming scancodes.
pub struct ScancodeStream {
    _private: (),
}

lazy_static::lazy_static! {
    pub static ref SCANCODE_STREAM: spin::Mutex<ScancodeStream> = spin::Mutex::new({
        SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100))
                .expect("ScancodeStream::new should only be called once");
        ScancodeStream { _private: () }
    });
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE
            .try_get()
            .expect("scancode queue not initialized");

        if let Ok(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Ok(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}

/// Called by the keyboard interrupt handler.
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        println!("WARNING: scancode queue uninitialized");
    }
}