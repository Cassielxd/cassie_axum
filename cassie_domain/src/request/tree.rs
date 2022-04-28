use std::collections::HashMap;
/**
 * @description:  TreeService 树节点生成
 * @author String
 * @date 2022/3/22 18:01
 * @email 348040933@qq.com
 */
pub trait TreeService<Entity, Dto>
where
  Entity: Clone + TreeModel,
  Dto: From<Entity> + TreeModel,
{
  fn build(&self, list: Vec<Entity>) -> Vec<Dto> {
    let mut result = HashMap::with_capacity(list.capacity());
    let mut data = vec![];
    for x in list {
      result.insert(x.get_id().clone().unwrap_or_default(), x);
    }
    for (k, v) in &result {
      if v.get_pid().unwrap() == "0" {
        let mut top = Dto::from(v.clone());
        self.find_childs(&mut top, &result);
        data.push(top);
      }
    }
    data
  }

  fn find_childs(&self, arg: &mut Dto, all: &HashMap<String, Entity>) {
    let mut childs = vec![];
    for (key, x) in all {
      if x.get_pid().is_some() && x.get_pid().eq(&arg.get_id()) {
        let mut item = Dto::from(x.clone());
        self.find_childs(&mut item, all);
        childs.push(item);
      }
    }
    if !childs.is_empty() {
      self.set_children(arg, Some(childs));
    }
  }

  fn set_children(&self, arg: &mut Dto, childs: Option<Vec<Dto>>);
}
/**
 * @description:  TreeModel 需要生成 Tree 必须实现 TreeModel
 * @author String
 * @date 2022/3/22 18:03
 * @email 348040933@qq.com
 */
pub trait TreeModel {
  fn get_pid(&self) -> Option<String>;
  fn get_id(&self) -> Option<String>;
}
