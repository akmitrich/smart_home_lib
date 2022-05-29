use std::error::Error;
type FetchResult = Result<(), Box<dyn Error>>;

mod smart_home;

mod smart_room;

mod smart_device;

#[cfg(test)]
mod tests {}
