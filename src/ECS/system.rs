use super::*;

use world::*;
use fetch::*;

pub trait System{
    type QUERY: fetch::QueryData;
    const ID: &'static str;
    const DEPENDS: &'static [&'static str];

    fn new() -> Self;
    fn execute(&mut self, Data: Query<'_, Self::QUERY>);
}

pub trait SystemWrapper{
    fn id(&self) -> &'static str;
    fn depends(&self) -> &'static [&'static str];
    fn execute<'a>(&mut self, World: &'a mut gmWorld);
}

impl<T: System> SystemWrapper for T{
    fn id(&self) -> &'static str {
        T::ID
    }   
    fn depends(&self) -> &'static [&'static str] {
        T::DEPENDS
    }
    fn execute<'a>(&mut self, World: &'a mut gmWorld) {
        self.execute(Query::fetch(World));
    }
}