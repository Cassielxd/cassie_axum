use crate::entity::asi_entitys::{AsiGroup, AsiGroupColumn, AsiGroupValues};
use serde::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Clone, Debug, Serialize,Validate, Deserialize)]
pub struct AsiGroupDTO {
    pub id: Option<i64>,
    #[validate(required)]
    pub cate_id: Option<i64>,
    #[validate(length(min = 4, message="名称最少4个字"))]
    pub name: Option<String>,
    pub info: Option<String>,
    #[validate(length(min = 6, message="分组编码最少6个字"))]
    pub group_code: Option<String>,
    pub agency_code: Option<String>,
}

impl_field_name_method!(AsiGroupDTO {
    id,
    cate_id,
    name,
    info,
    group_code,
    agency_code
});

impl Into< AsiGroup> for AsiGroupDTO {
    fn into(self) ->  AsiGroup {
        AsiGroup {
            id: self.id,
            cate_id: self.cate_id,
            name: self.name,
            info: self.info,
            group_code: self.group_code,
            agency_code: self.agency_code
        }
    }
}

impl From<AsiGroup> for AsiGroupDTO {
    fn from(arg: AsiGroup) -> Self {
        Self {
            id: arg.id,
            cate_id: arg.cate_id,
            name: arg.name,
            info: arg.info,
            group_code: arg.agency_code,
            agency_code: arg.group_code
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AsiGroupColumnDTO{
    pub id: Option<i64>,
    pub agency_code: Option<String>,
    pub product_code: Option<String>,
    pub group_code: Option<String>,
    pub column_code: Option<String>,
    pub column_name: Option<String>,
    pub data_type: Option<String>,
    pub example_value: Option<String>,
    pub max_length: Option<i64>,
    pub is_required: Option<String>,
    pub display_order: Option<i8>,
    pub default_value: Option<String>,
    pub display_length: Option<i64>,
    pub is_display: Option<i8>,
}
impl_field_name_method!(AsiGroupColumnDTO {
    id,
    agency_code,
    product_code,
    group_code,
    column_code,
    column_name,
    data_type,
    example_value,
    max_length,
    is_required,
    display_order,
    default_value,
    display_length,
    is_display
});
impl Into< AsiGroupColumn> for AsiGroupColumnDTO {
    fn into(self) ->  AsiGroupColumn {
        AsiGroupColumn {
            id: self.id,
            agency_code: self.agency_code,
            product_code: self.product_code,
            group_code: self.group_code,
            column_code: self.column_code,
            column_name: self.column_name,
            data_type: self.data_type,
            example_value: self.example_value,
            max_length: self.max_length,
            is_required: self.is_required,
            display_order: self.display_order,
            default_value: self.default_value,
            display_length: self.display_length,
            is_display: self.is_display
        }
    }
}

impl From<AsiGroupColumn> for AsiGroupColumnDTO {
    fn from(arg: AsiGroupColumn) -> Self {
        Self {
            id: arg.id,
            agency_code: arg.agency_code,
            product_code: arg.product_code,
            group_code: arg.group_code,
            column_code: arg.column_code,
            column_name: arg.column_name,
            data_type: arg.data_type,
            example_value: arg.example_value,
            max_length: arg.max_length,
            is_required: arg.is_required,
            display_order: arg.display_order,
            default_value: arg.default_value,
            display_length: arg.display_length,
            is_display: arg.is_display
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AsiGroupValuesDTO{
    pub id: Option<i64>,
    pub agency_code: Option<String>,
    pub product_code: Option<String>,
    pub group_code: Option<String>,
    pub column_code: Option<String>,
    pub column_value: Option<String>,
    pub ref_id: Option<i64>,
}
impl_field_name_method!(AsiGroupValuesDTO {
    id,
    agency_code,
    product_code,
    group_code,
    column_code,
    column_value,
    ref_id
});
impl Into< AsiGroupValues> for AsiGroupValuesDTO {
    fn into(self) ->  AsiGroupValues {
        AsiGroupValues {
            id: self.id,
            agency_code: self.agency_code,
            product_code: self.product_code,
            group_code: self.group_code,
            column_code: self.column_code,
            column_value: self.column_value,
            ref_id: self.ref_id
        }
    }
}

impl From<AsiGroupValues> for AsiGroupValuesDTO {
    fn from(arg: AsiGroupValues) -> Self {
        Self {
            id: arg.id,
            agency_code: arg.agency_code,
            product_code: arg.product_code,
            group_code: arg.group_code,
            column_code: arg.column_code,
            column_value: arg.column_value,
            ref_id: arg.ref_id
        }
    }
}