use std::str::FromStr;

pub trait InputConversions {
  fn to<T>(self) -> Vec<T>
  where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug;
}

impl<T, S> InputConversions for T
where
  T: IntoIterator<Item = S>,
  S: ToString,
{
  fn to<O>(self) -> Vec<O>
  where
    O: FromStr,
    <O as FromStr>::Err: std::fmt::Debug,
  {
    self.into_iter().map(|item| item.to_string().parse::<O>().unwrap()).collect()
  }
}
