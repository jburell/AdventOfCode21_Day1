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