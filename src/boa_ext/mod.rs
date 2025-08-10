use boa_engine::value::{TryFromJs, TryIntoJs};

pub trait TryFromAndIntoJs
where
    Self: TryFromJs + TryIntoJs,
{
}
