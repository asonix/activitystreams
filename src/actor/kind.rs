use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Application)]
pub struct ApplicationType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Group)]
pub struct GroupType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Organization)]
pub struct OrganizationType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Person)]
pub struct PersonType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Service)]
pub struct ServiceType;
