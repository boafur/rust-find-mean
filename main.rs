fn get_average(marks: &[f32]) -> f32 {
  let mut num: f32 = 0.;
  for i in marks.iter() {
    num += i;
  }
  (num / (marks.len()) as f32).floor()
}

fn main() {
  println!("{0}", get_average(&[2., 2., 2., 2.]));
}
