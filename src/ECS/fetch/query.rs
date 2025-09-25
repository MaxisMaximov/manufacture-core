use std::{collections::BTreeMap, ops::{Deref, DerefMut}};

use crate::ECS::{self, storage::Storage};
use ECS::entity::Entity;
use ECS::world::World;
use ECS::comp::Component;
use super::{Fetch, FetchMut};

/// # Query fetch trait
/// Required for `Query` to know what to fetch from the World
/// 
/// It is implemented by default on `&` and `&mut` Component references, as well as Tuples up to 4 elements
/// 
/// The return type `Item` is typically the type the trait gets implemented on
pub trait QueryData{
    type Item<'b>;
    type AccItem<'b>;
    type MutAccItem<'b>;

    /// Fetch the data from the world
    fn fetch<'a>(World: &'a World) -> Self::Item<'a>;
    fn get<'a>(Fetched: &'a Self::Item<'a>, Index: &usize) -> Option<Self::AccItem<'a>>;
    fn get_mut<'a>(Fetched: &'a mut Self::Item<'a>, Index: &usize) -> Option<Self::MutAccItem<'a>>;
}

/// # World Query
/// Struct that queries the World and fetches the specified `QueryData`, usually Components
pub struct Query<'a, D: QueryData>{
    entities: &'a BTreeMap<usize, Entity>,
    data: D::Item<'a>
}
impl<'a, D: QueryData> Query<'a, D>{
    pub fn fetch(World: &'a World) -> Self{
        Self{
            entities: World.get_entities(),
            data: D::fetch(World)
        }
    }

    pub fn get(&'a self, Index: &usize) -> Option<D::AccItem<'a>>{
        if !self.entities.contains_key(Index){
            return None
        }

        D::get(&self.data, Index)
    }
    pub fn get_mut(&'a mut self, Index: &usize) -> Option<D::MutAccItem<'a>>{
        if !self.entities.contains_key(Index){
            return None
        }

        D::get_mut(&mut self.data, Index)
    }

    // pub fn iter(&self) -> impl Iterator<Item = D>{
    //     todo!("Still figuring this out")
    // }
}
impl<'a, D:QueryData> Deref for Query<'a, D>{
    type Target = D::Item<'a>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<'a, D: QueryData> DerefMut for Query<'a, D>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

///////////////////////////////////////////////////////////////////////////////
// Components
///////////////////////////////////////////////////////////////////////////////

impl<C:Component> QueryData for &C{
    type Item<'b> = Fetch<'b, C>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.fetch::<C>()
    }
    
    type AccItem<'b> = &'b C;
    type MutAccItem<'b> = &'b C;
    
    fn get<'a>(Fetched: &'a Self::Item<'a>, Index: &usize) -> Option<Self::AccItem<'a>> {
        Fetched.get(Index)
    }
    fn get_mut<'a>(Fetched: &'a mut Self::Item<'a>, Index: &usize) -> Option<Self::MutAccItem<'a>> {
        Fetched.get(Index)
    }    
}
impl<C: Component> QueryData for &mut C{
    type Item<'b> = FetchMut<'b, C>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.fetch_mut::<C>()
    }
    
    type AccItem<'b> = &'b C;
    type MutAccItem<'b> = &'b mut C;
    
    fn get<'a>(Fetched: &'a Self::Item<'a>, Index: &usize) -> Option<Self::AccItem<'a>> {
        Fetched.get(Index)
    }
    fn get_mut<'a>(Fetched: &'a mut Self::Item<'a>, Index: &usize) -> Option<Self::MutAccItem<'a>> {
        Fetched.get_mut(Index)
    }
}

///////////////////////////////////////////////////////////////////////////////
// Tuples
///////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! query_impl {
    ($($x:ident), *) => {
        impl<$($x: QueryData), *> QueryData for ($($x), *){
            type Item<'b> = ($($x::Item<'b>), *);

            fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
                ($($x::fetch(World)), *)
            }

            type AccItem<'b> = ($($x::AccItem<'b>), *);
            type MutAccItem<'b> = ($($x::MutAccItem<'b>), *);
            
            fn get<'a>(($($x), *): &'a Self::Item<'a>, Index: &usize) -> Option<Self::AccItem<'a>> {
                Some(
                    ($($x::get($x, Index)?), *)
                )
            }
            fn get_mut<'a>(($($x), *): &'a mut Self::Item<'a>, Index: &usize) -> Option<Self::MutAccItem<'a>> {
                Some(
                    ($($x::get_mut($x, Index)?), *)
                )
            }
        }
    }
}


query_impl!(A, B);
query_impl!(A, B, C);
query_impl!(A, B, C, D);
query_impl!(A, B, C, D, E);
query_impl!(A, B, C, D, E, F);
query_impl!(A, B, C, D, E, F, G);
query_impl!(A, B, C, D, E, F, G, H);
query_impl!(A, B, C, D, E, F, G, H, I);
query_impl!(A, B, C, D, E, F, G, H, I, J);
query_impl!(A, B, C, D, E, F, G, H, I, J, K);
query_impl!(A, B, C, D, E, F, G, H, I, J, K, L);