use crate::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentVariables(crate::types::Map<String, String>);

pub struct Directory(String);

// pub struct UserId {
//     pub uid: u32,
//     pub gid: u32,
//     pub groups: Vec<u32>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub struct Context {
//     /// set working directory
//     pub working_dir: Option<String>,

//     /// set environment variables
//     pub environment_variables: Option<ReplaceOrAdd<EnvironmentVariables>>,

//     /// set the user. this is linux specific
//     pub as_user: Option<UserId>,
// }
