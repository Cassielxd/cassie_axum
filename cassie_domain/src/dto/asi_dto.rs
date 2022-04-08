use crate::entity::asi_entitys::{AsiGroup, AsiGroupColumn, AsiGroupValues};
use crate::request::tree::TreeModel;
use serde::{Deserialize, Serialize};
use validator_derive::Validate;

#[derive(Clone, Debug, Serialize, Validate, Deserialize, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct AsiGroupDTO {
    id: Option<i64>,
    #[validate(required)]
    cate_id: Option<i64>,
    #[validate(length(min = 4, message = "名称最少4个字"))]
    name: Option<String>,

    info: Option<String>,
    #[validate(length(min = 6, message = "分组编码最少6个字"))]
    group_code: Option<String>,

    agency_code: Option<String>,

    group_type: Option<String>,

    parent_group_code: Option<String>,

    children: Option<Vec<AsiGroupDTO>>,
}

impl TreeModel for AsiGroupDTO {
    fn get_pid(&self) -> Option<String> {
        Some(self.parent_group_code.clone().unwrap())
    }

    fn get_id(&self) -> Option<String> {
        Some(self.group_code.clone().unwrap())
    }
}

impl Into<AsiGroup> for AsiGroupDTO {
    fn into(self) -> AsiGroup {
        AsiGroup {
            id: self.id,
            cate_id: self.cate_id,
            name: self.name,
            info: self.info,
            group_code: self.group_code,
            agency_code: self.agency_code,
            group_type: self.group_type,
            parent_group_code: self.parent_group_code,
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
            group_code: arg.group_code,
            agency_code: arg.agency_code,
            group_type: arg.group_type,
            parent_group_code: arg.parent_group_code,
            children: Some(Vec::<AsiGroupDTO>::new()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct AsiGroupColumnDTO {
    id: Option<i64>,

    agency_code: Option<String>,

    product_code: Option<String>,

    group_code: Option<String>,
    #[validate(required)]
    #[validate(length(min = 2, message = "最少2个字符"))]
    column_code: Option<String>,
    #[validate(required)]
    #[validate(length(min = 2, message = "最少2个字符"))]
    column_name: Option<String>,
    #[validate(required)]
    data_type: Option<String>,
    #[validate(required)]
    example_value: Option<String>,
    #[validate(required)]
    max_length: Option<i64>,
    #[validate(required)]
    is_required: Option<String>,

    display_order: Option<i8>,

    default_value: Option<String>,

    display_length: Option<i64>,

    is_display: Option<i8>,
}

impl Into<AsiGroupColumn> for AsiGroupColumnDTO {
    fn into(self) -> AsiGroupColumn {
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
            is_display: self.is_display,
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
            is_display: arg.is_display,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct AsiGroupValuesDTO {
    id: Option<i64>,

    agency_code: Option<String>,

    product_code: Option<String>,

    group_code: Option<String>,

    #[validate(required)]
    column_code: Option<String>,

    #[validate(required)]
    column_value: Option<String>,
    #[validate(required)]
    ref_id: Option<i64>,
}

impl Into<AsiGroupValues> for AsiGroupValuesDTO {
    fn into(self) -> AsiGroupValues {
        AsiGroupValues {
            id: self.id,
            agency_code: self.agency_code,
            product_code: self.product_code,
            group_code: self.group_code,
            column_code: self.column_code,
            column_value: self.column_value,
            ref_id: self.ref_id,
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
            ref_id: arg.ref_id,
        }
    }
}
