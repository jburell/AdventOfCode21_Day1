use std::collections::{VecDeque};
use std::collections::vec_deque::Iter;

pub fn count_number_of_increases(numbers: Vec<u32>) -> usize {
  let (count, _) = numbers
    .iter()
    .fold((0, u32::MAX),|(count, last), elem| {
        if last < *elem {
          (count + 1, *elem)
        } else {
          (count, *elem)
        }
      }
    );
  count
}

type Elem = u32;

pub fn count_number_of_increases_in_sliding_window(mut numbers: Vec<Elem>) -> usize {
  let rotate_ring_buffer = 
    |buf: &mut VecDeque<Elem>, value| {buf.push_front(value); buf.pop_back();};
  let count_value_of_buf = 
    |buf: Iter<Elem>| buf.fold(0, |a, v| a + *v);
  let first_triple: Vec<Elem> = numbers.drain(0..3).collect(); // Can panic if < 3 elems
  
  let (count, _) = 
    numbers
    .iter()
    .fold((0, VecDeque::from_iter(first_triple)),
      |(count, mut last), elem| {
        if count_value_of_buf(last.iter()) < count_value_of_buf(last.range(0..2)) + elem {
          rotate_ring_buffer(&mut last, *elem);
          (count + 1, last)
        } else {
          rotate_ring_buffer(&mut last, *elem);
          (count, last)
        }
      }
    );
  count
}