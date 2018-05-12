use object::{Object, ObjectProperties};

mod kind;
pub use self::kind::*;

pub trait Actor: Object {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Appliation {
    #[serde(rename = "type")]
    kind: ApplicationType,

    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Appliation {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    #[serde(rename = "type")]
    kind: GroupType,

    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Group {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    #[serde(rename = "type")]
    kind: OrganizationType,

    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Organization {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[serde(rename = "type")]
    kind: PersonType,

    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Person {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    #[serde(rename = "type")]
    kind: ServiceType,

    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Service {}
