use std::cell::{RefMut, Ref};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};


use super::world::gmWorld;
use super::*;

use comp::Component;
use events::gmEvent;
use resource::gmRes;
use commands::gmCommand;

pub type Fetch<'a, C: Component> = Ref<'a, C::STORAGE>;
pub type FetchMut<'a, C: Component> = RefMut<'a, C::STORAGE>;

pub type FetchRes<'a, R: gmRes> = Ref<'a, R>;
pub type FetchResMut<'a, R: gmRes> = RefMut<'a, R>;

pub type EventReader<'a, E: gmEvent> = Ref<'a, VecDeque<E>>;
pub type EventWriter<'a, E: gmEvent> = RefMut<'a, VecDeque<E>>;

pub type CommandWriter<'a> = RefMut<'a, Vec<Box<dyn gmCommand>>>;

/// # Query fetch trait
/// Required for `Query` to know what to fetch from the World
/// 
/// It is implemented by default on `&` and `&mut` Component references, as well as Tuples up to 4 elements
/// 
/// The return type `Item` is typically the type the trait gets implemented on
pub trait QueryData{
    type Item<'b>;

    /// Fetch the data from the world
    fn fetch<'a>(World: &'a gmWorld) -> Self::Item<'a>;
}

/// # World Query
/// Struct that queries the World and fetches the specified `QueryData`
pub struct Query<'a, C: QueryData>{
    data: C::Item<'a>
}
impl<'a, D: QueryData> Query<'a, D>{
    pub fn fetch(World: &'a gmWorld) -> Self{
        Self{
            data: D::fetch(World)
        }
    }
}
impl<'a, C:QueryData> Deref for Query<'a, C>{
    type Target = C::Item<'a>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<'a, C: QueryData> DerefMut for Query<'a, C>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T:Component> QueryData for &T{
    type Item<'b> = Ref<'b, T::STORAGE>;

    fn fetch<'a>(World: &'a gmWorld) -> Self::Item<'a> {
        World.fetch::<T>()
    }
}

impl<T: Component> QueryData for &mut T{
    type Item<'b> = RefMut<'b, T::STORAGE>;

    fn fetch<'a>(World: &'a gmWorld) -> Self::Item<'a> {
        World.fetchMut::<T>()
    }
}


impl QueryData for (){
    type Item<'b> = ();

    fn fetch<'a>(_World: &'a gmWorld) -> Self::Item<'a>{}
}
impl<A: QueryData, B: QueryData> QueryData for (A, B){
    type Item<'b> = (A::Item<'b>, B::Item<'b>);

    fn fetch<'a>(World: &'a gmWorld) -> Self::Item<'a> {
        (A::fetch(World), B::fetch(World))
    }
}
impl<A: QueryData, B: QueryData, C: QueryData> QueryData for (A, B, C){
    type Item<'b> = (A::Item<'b>, B::Item<'b>, C::Item<'b>);

    fn fetch<'a>(World: &'a gmWorld) -> Self::Item<'a> {
        (A::fetch(World), B::fetch(World), C::fetch(World))
    }
}
impl<A: QueryData, B: QueryData, C: QueryData, D: QueryData> QueryData for (A, B, C, D){
    type Item<'b> = (A::Item<'b>, B::Item<'b>, C::Item<'b>, D::Item<'b>);

    fn fetch<'a>(World: &'a gmWorld) -> Self::Item<'a> {
        (A::fetch(World), B::fetch(World), C::fetch(World), D::fetch(World))
    }
}