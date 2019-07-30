# ActivityStreams Types
__A base set of types from the Activity Streams specification.__

- [Read the documentation on docs.rs](https://docs.rs/activitystreams-types)
- [Find the crate on crates.io](https://crates.io/crates/activitystreams-types)
- [Join the discussion on Matrix](https://matrix.to/#/!fAEcHyTUdAaKCzIKCt:asonix.dog?via=asonix.dog)

## Usage
First, add the crate to your cargo.toml
```toml
# Cargo.toml

activitystreams-types = "0.2"
```

Then use it in your project!
```rust
// in your project

extern crate activitystreams_types;
extern crate failure;
extern crate serde_json;

use activitystreams_types::{context, link::Mention};
use failure::Error;

fn run() -> Result<(), Error> {
    /// A Mention is the only predefined Link type in the Activity Streams spec
    let mut mention = Mention::default();
    mention.link_props.set_context_object(context())?;

    let mention_string = serde_json::to_string(&mention)?;

    let mention: Mention = serde_json::from_str(&mention_string)?;

    Ok(())
}
```

## Contributing
Feel free to open issues for anything you find an issue with. Please note that any contributed code will be licensed under the GPLv3.

## License

Copyright © 2018 Riley Trautman

ActivityStreams Types is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

ActivityStreams Types is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details. This file is part of ActivityStreams Types.

You should have received a copy of the GNU General Public License along with ActivityStreams Types. If not, see [http://www.gnu.org/licenses/](http://www.gnu.org/licenses/).
