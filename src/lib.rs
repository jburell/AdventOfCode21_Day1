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

type Elem = Option<u32>;

pub fn count_number_of_increases_in_sliding_window(numbers: Vec<u32>) -> usize {
  let rotate_ring_buffer = 
    |buf: &mut VecDeque<_>, value| {buf.push_front(Some(value)); buf.pop_back()};
  let count_value_of_buf = 
    |buf: Iter<Elem>| buf.fold(0, |a, v| v.map_or(a, |n| a + n));
  
  let (count, _) = numbers
    .iter()
    .fold((0, VecDeque::from([None, None, None])),|(count, mut last), elem| {
      if last.iter().any(|v| *v == None) {
        rotate_ring_buffer(&mut last, *elem);
        (count, last)
      } else {
        if count_value_of_buf(last.iter()) < count_value_of_buf(last.range(0..2)) + *elem {
          rotate_ring_buffer(&mut last, *elem);
          (count + 1, last)
        } else {
          rotate_ring_buffer(&mut last, *elem);
          (count, last)
        }
      }
    }
    );
  count
}