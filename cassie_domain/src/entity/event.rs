#[crud_table(table_name:sys_event_config)]
#[derive(Clone, Debug, Getters, Setters, Default)]
pub struct EventConfig {
  pub id: Option<i64>,
  pub status: Option<String>,
  pub extend1: Option<String>,
  pub extend2: Option<String>,
  pub extend3: Option<String>,
  pub description: Option<String>,
  pub agency_code: Option<String>,
  pub path: Option<String>,
  pub resource_name: Option<String>,
  pub oprate_description: Option<String>,
  pub event_type: Option<String>,
  pub event_code: Option<String>,
  pub event_name: Option<String>,
  pub lock_user_id: Option<String>,
  pub event_script: Option<String>,
  pub need_persist: Option<String>,
}
impl_field_name_method!(EventConfig {
  id,
  status,
  extend1,
  extend2,
  extend3,
  description,
  agency_code,
  path,
  resource_name,
  oprate_description,
  event_type,
  event_code,
  event_name,
  event_script,
  lock_user_id,
  need_persist
});
