use crate::{
    dom::update::ApplyGlobalContent, structure::MapVisit, update_with::SpecificUpdate, UpdateWith,
};

use super::RenderObject;

type MapOutput<'a, T> = <T as MapVisit<ApplyGlobalContent<'a>>>::Output;

pub trait ChildrenNodes<'a>
where
    Self: MapVisit<ApplyGlobalContent<'a>, Output = Self::AliasMapOutput>,
{
    type AliasMapOutput: SpecificUpdate<UpdateTo = Self::AliasUpdateTo>;
    type AliasUpdateTo: RenderObject + UpdateWith<MapOutput<'a, Self>> + 'static;
}

impl<'a, T> ChildrenNodes<'a> for T
where
    T: MapVisit<ApplyGlobalContent<'a>>,
    T::Output: SpecificUpdate,
    <MapOutput<'a, T> as SpecificUpdate>::UpdateTo:
        RenderObject + UpdateWith<MapOutput<'a, T>> + 'static,
{
    type AliasMapOutput = T::Output;
    type AliasUpdateTo = <Self::AliasMapOutput as SpecificUpdate>::UpdateTo;
}