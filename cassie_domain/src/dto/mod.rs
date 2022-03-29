pub mod asi_dto;
pub mod sign_in;
pub mod sys_auth_dto;
pub mod sys_dict_dto;
pub mod sys_menu_dto;
pub mod sys_params_dto;
pub mod sys_role_dto;
pub mod sys_user_dto;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmptyDTO {}

/// IdDTO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IdDTO {
    pub id: String,
}
