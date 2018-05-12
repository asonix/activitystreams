use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Accept)]
pub struct AcceptType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Add)]
pub struct AddType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Move)]
pub struct MoveType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Announce)]
pub struct AnnounceType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Arrive)]
pub struct ArriveType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Block)]
pub struct BlockType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Create)]
pub struct CreateType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Delete)]
pub struct DeleteType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Dislike)]
pub struct DislikeType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Flag)]
pub struct FlagType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Follow)]
pub struct FollowType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Ignore)]
pub struct IgnoreType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Invite)]
pub struct InviteType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Join)]
pub struct JoinType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Leave)]
pub struct LeaveType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Like)]
pub struct LikeType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Listen)]
pub struct ListenType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Offer)]
pub struct OfferType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Question)]
pub struct QuestionType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Real)]
pub struct ReadType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Reject)]
pub struct RejectType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Remove)]
pub struct RemoveType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(TentativeAccept)]
pub struct TentativeAcceptType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(TentativeReject)]
pub struct TentativeRejectType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Travel)]
pub struct TravelType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Undo)]
pub struct UndoType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Update)]
pub struct UpdateType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(View)]
pub struct ViewType;
