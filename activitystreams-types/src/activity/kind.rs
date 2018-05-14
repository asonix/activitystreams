/*
 * This file is part of ActivityStreams Types.
 *
 * Copyright © 2018 Riley Trautman
 *
 * ActivityStreams Types is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * ActivityStreams Types is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with ActivityStreams Types.  If not, see <http://www.gnu.org/licenses/>.
 */

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Accept)]
pub struct AcceptType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Add)]
pub struct AddType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Move)]
pub struct MoveType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Announce)]
pub struct AnnounceType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Arrive)]
pub struct ArriveType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Block)]
pub struct BlockType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Create)]
pub struct CreateType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Delete)]
pub struct DeleteType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Dislike)]
pub struct DislikeType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Flag)]
pub struct FlagType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Follow)]
pub struct FollowType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Ignore)]
pub struct IgnoreType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Invite)]
pub struct InviteType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Join)]
pub struct JoinType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Leave)]
pub struct LeaveType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Like)]
pub struct LikeType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Listen)]
pub struct ListenType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Offer)]
pub struct OfferType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Question)]
pub struct QuestionType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Real)]
pub struct ReadType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Reject)]
pub struct RejectType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Remove)]
pub struct RemoveType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(TentativeAccept)]
pub struct TentativeAcceptType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(TentativeReject)]
pub struct TentativeRejectType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Travel)]
pub struct TravelType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Undo)]
pub struct UndoType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Update)]
pub struct UpdateType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(View)]
pub struct ViewType;
