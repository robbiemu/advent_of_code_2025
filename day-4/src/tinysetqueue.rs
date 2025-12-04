// #![cfg_attr(not(feature = "std"), no_std)]

pub mod prelude {
  pub use super::{MembershipMode, PushResult, TinySetQueue};
}

pub enum PushResult {
  Inserted,
  AlreadyPresent,
}

pub enum MembershipMode {
  /// The queue tracks temporary membership.
  /// When an item is popped, it becomes eligible for re-enqueue.
  InQueue,

  /// The queue tracks permanent membership.
  /// Once pushed once, the item will never be re-queued.
  Visited,
}

pub struct TinySetQueue<'a, T> {
  buf: &'a mut [T],
  in_queue: &'a mut [bool],
  mode: MembershipMode,
  head: usize,
  tail: usize,
  len: usize,
}

impl<'a, T> TinySetQueue<'a, T>
where
  T: Copy + Into<usize>,
{
  /// Construct a queue with:
  /// - `buf`: the ring buffer storage (FIFO)
  /// - `in_queue`: membership tracking bitmap
  /// - `mode`: behavior on pop()
  ///
  /// The length of `in_queue` must exceed the largest `T::into()` value.
  pub fn new(
    buf: &'a mut [T],
    in_queue: &'a mut [bool],
    mode: MembershipMode,
  ) -> Self {
    TinySetQueue { buf, in_queue, mode, head: 0, tail: 0, len: 0 }
  }

  #[inline]
  pub fn capacity(&self) -> usize {
    self.buf.len()
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.len
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.len == 0
  }

  #[inline]
  pub fn is_full(&self) -> bool {
    self.len == self.buf.len()
  }

  /// Pushes a value into the queue unless it is already present.
  ///
  /// Returns:
  /// - `Ok(PushResult::Inserted)` when the value is scheduled
  /// - `Ok(PushResult::AlreadyPresent)` when deduplicated
  /// - `Err(value)` if the queue is full or the index is out-of-range
  pub fn push(&mut self, value: T) -> Result<PushResult, T> {
    let idx: usize = value.into();

    if idx >= self.in_queue.len() {
      return Err(value);
    }

    if self.in_queue[idx] {
      return Ok(PushResult::AlreadyPresent);
    }

    if self.is_full() {
      return Err(value);
    }

    self.buf[self.tail] = value;
    self.in_queue[idx] = true;

    self.tail = (self.tail + 1) % self.buf.len();
    self.len += 1;

    Ok(PushResult::Inserted)
  }

  /// Pops from the head of the queue.
  ///
  /// In `InQueue` mode, membership is cleared.
  /// In `Visited` mode, membership persists.
  pub fn pop(&mut self) -> Option<T> {
    if self.is_empty() {
      return None;
    }

    let value = self.buf[self.head];
    let idx: usize = value.into();

    match self.mode {
      MembershipMode::InQueue => {
        self.in_queue[idx] = false;
      }
      MembershipMode::Visited => {
        // keep membership true
      }
    }

    self.head = (self.head + 1) % self.buf.len();
    self.len -= 1;

    Some(value)
  }
}
