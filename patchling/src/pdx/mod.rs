// TODO: Consider using fixed point in here. Probably not worth it; Stellaris AFAIK uses a large
//       mix of formats, and our model only lets us use one generic one.

mod export;
mod model;
mod parser;
mod walk;

pub use model::*;
