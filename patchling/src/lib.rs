// TODO: Consider using fixed point in here. Probably not worth it; Stellaris AFAIK uses a large
//       mix of formats, and our model only lets us use one generic one.

mod lua;
mod paths;
pub mod pdx;

pub fn test_load_lua() {
    lua::test_lua().unwrap()
}
