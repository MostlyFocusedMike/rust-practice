fn arr_rec(arr: [usize; 4], idx: usize) {
  if arr.len() < idx {
    println!("DONE!");
    return;
  }
  println!("{}", arr[idx]);
  arr_rec(arr, idx + 1);
}

fn main() {
  let arr = [1,2,3,4];
  arr_rec(arr, 0);
}
