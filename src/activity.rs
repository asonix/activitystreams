/*
 * This file is part of ActivityStreams.
 *
 * Copyright © 2018 Riley Trautman
 *
 * ActivityStreams is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * ActivityStreams is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with ActivityStreams.  If not, see <http://www.gnu.org/licenses/>.
 */

//! Activity traits and types

pub use activitystreams_traits::{Activity, IntransitiveActivity};
pub use activitystreams_types::activity::{
    kind,
    properties::{self, ActivityExt},
    AMove, Accept, Add, Announce, Arrive, Block, Create, Delete, Dislike, Flag, Follow, Ignore,
    Invite, Join, Leave, Like, Listen, Offer, Question, Read, Reject, Remove, TentativeAccept,
    TentativeReject, Travel, Undo, Update, View,
};
