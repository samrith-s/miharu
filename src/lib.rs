#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::time::Duration;

use notify_debouncer_full::{ notify::*, new_debouncer, DebounceEventResult, Debouncer };

#[napi(js_name = "Miru")]
pub struct Miru {
  notify: Debouncer,
}

#[napi]
pub impl Miru {
  #[napi(constructor)]
  pub fn new() -> Self {
    Miru {
      notify: new_debouncer(Duration::from_millis(200), None),
    }
  }
}

// Select recommended watcher for debouncer.
// Using a callback here, could also be a channel.

// let mut debouncer = new_debouncer(Duration::from_secs(2), None, |result: DebounceEventResult| {
//   match result {
//     Ok(events) => events.iter().for_each(|event| println!("{event:?}")),
//     Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
//   }
// }).unwrap();

// Add a path to be watched. All files and directories at that path and
// below will be monitored for changes.
// debouncer.watcher().watch(Path::new("."), RecursiveMode::Recursive).unwrap();

// Add the same path to the file ID cache. The cache uses unique file IDs
// provided by the file system and is used to stich together rename events
// in case the notification back-end doesn't emit rename cookies.
// debouncer.cache().add_root(Path::new("."), RecursiveMode::Recursive);
