use std::cmp;

fn main() {
  let candles: [i8;6] = [1, 5, 3, 5, 3, 5];  
  println!("tallest candle count: {}",birthday_cake_candles(&candles));
  
}
fn birthday_cake_candles(arr: &[i8]) -> usize{
  let mut tallest = 0;
  let mut tallestArr: Vec<i8> = vec!(0);

  for &x in arr.iter() {

    tallest = cmp::max(x, tallest);
    if tallestArr[0]==x{
      tallestArr.push(tallest);  
    }
    if tallestArr[0] < tallest{
      tallestArr = Vec::new();
      tallestArr.push(tallest)
    }
  }
  tallestArr.len()
} 
