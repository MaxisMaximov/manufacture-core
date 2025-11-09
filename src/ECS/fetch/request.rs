use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::ECS;
use crate::ECS::fetch::{WorldQuery, QueryData, QueryFilter};
use ECS::world::World;
use ECS::resource::Resource;
use ECS::events::Event;
use super::{FetchRes, FetchResMut};
use super::{EventReader, EventWriter};
use super::{CommandWriter, TriggerWriter};

/// # Request fetch trait
/// Required for `Request` to know what system resources to fetch from the World
/// 
/// It is implemented by default on `&` and `&mut` Resource references, 
/// Event Readers and Writers, the Command and Trigger Writers, as well as Tuples up to 4 elements
/// 
/// The return type `Item` is typically the type the trait gets implemented on
pub trait RequestData{
    type Item<'b>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a>;
}

/// # System resource Request
/// Struct that requests desired system resources from the World  
/// Such as Resources, Event Readers/Writers and Trigger and Command Writers
pub struct Request<'a, D: RequestData>{
    data: D::Item<'a>
}
impl<'a, D: RequestData> Request<'a, D>{
    pub fn fetch(World: &'a World) -> Self{
        Self{
            data: D::fetch(World),
        }
    }
}
impl<'a, D: RequestData> Deref for Request<'a, D>{
    type Target = D::Item<'a>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<'a, D: RequestData> DerefMut for Request<'a, D>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

/// # Query request
/// An identifier for `WorldQuery` to make data acquisition easier
/// 
/// **Below documentation for `WorldQuery`**
/// 
/// Struct that queries the World and fetches the specified `QueryData`, usually Components
/// 
/// You can specify filters for the Query to use when getting Entities, such as `With` and `Without`.  
/// Any type implementing `QueryFilter` can be used
/// 
/// To get a specific Entity's set of components, use `get`, `get_mut`, and their Token variations.  
/// Token variations of getters are preferred over normal getters
/// 
/// To iterate over all entities with all queried components, use `iter` and `iter_mut`
/// 
/// To access the underlying Storages directly, use a dereference `*`.  
/// Note that Filters will not apply if you do this
/// 
/// Query automatically validates Tokens in Getter functions, they can also be  
/// manually validated via `validate_token`
pub struct Query<D: QueryData, F: QueryFilter>(PhantomData<(D, F)>);
impl <D: QueryData, F: QueryFilter> RequestData for Query<D, F>{
    type Item<'b> = WorldQuery<'b, D, F>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        WorldQuery::fetch(World)
    }
}

///////////////////////////////////////////////////////////////////////////////
// Resources
///////////////////////////////////////////////////////////////////////////////

impl<R: Resource> RequestData for &R{
    type Item<'b> = FetchRes<'b, R>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.fetch_res()
    }
}
impl<R: Resource> RequestData for &mut R{
    type Item<'b> = FetchResMut<'b, R>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.fetch_res_mut()
    }
}

///////////////////////////////////////////////////////////////////////////////
// Events
///////////////////////////////////////////////////////////////////////////////

pub struct ReadEvent<E: Event>(PhantomData<E>);

impl<E: Event> RequestData for ReadEvent<E>{
    type Item<'b> = EventReader<'b, E>;
    
    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.get_event_reader()
    }
}

pub struct WriteEvent<E: Event>(PhantomData<E>);
impl<E: Event> RequestData for WriteEvent<E>{
    type Item<'b> = EventWriter<'b, E>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.get_event_writer()
    }
}

///////////////////////////////////////////////////////////////////////////////
// Writers
///////////////////////////////////////////////////////////////////////////////

pub struct Commands;
pub struct Triggers;

impl RequestData for Commands{
    type Item<'b> = CommandWriter<'b>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.get_command_writer()
    }
}
impl RequestData for Triggers{
    type Item<'b> = TriggerWriter<'b>;

    fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
        World.get_trigger_writer()
    }
}

///////////////////////////////////////////////////////////////////////////////
// Tuples
///////////////////////////////////////////////////////////////////////////////

macro_rules! request_impl {
    ($($x:ident), *) => {
        #[allow(non_snake_case)]
        impl<$($x: RequestData), *> RequestData for ($($x), *){
            type Item<'b> = ($($x::Item<'b>), *);

            fn fetch<'a>(World: &'a World) -> Self::Item<'a> {
                ($($x::fetch(World)), *)
            }
        }
    }
}

request_impl!(A, B);
request_impl!(A, B, C);
request_impl!(A, B, C, D);
request_impl!(A, B, C, D, E);
request_impl!(A, B, C, D, E, F);
request_impl!(A, B, C, D, E, F, G);
request_impl!(A, B, C, D, E, F, G, H);
request_impl!(A, B, C, D, E, F, G, H, I);
request_impl!(A, B, C, D, E, F, G, H, I, J);
request_impl!(A, B, C, D, E, F, G, H, I, J, K);
request_impl!(A, B, C, D, E, F, G, H, I, J, K, L);