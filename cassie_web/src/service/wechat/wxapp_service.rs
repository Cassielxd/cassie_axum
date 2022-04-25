use cassie_domain::{
    dto::user_dto::{UserDTO, WechatUserDTO},
    entity::user_entity::{User, WechatUser},
};
use rbatis::{crud::CRUD, rbatis::Rbatis};

use crate::{
    service::{
        api::user_service::{UserService, WechatUserService},
        crud_service::CrudService,
    },
    APPLICATION_CONTEXT,
};

//新增或更新用户
pub async fn save_or_update_user(user: WechatUserDTO) -> i64 {
    let user_service = APPLICATION_CONTEXT.get::<UserService>();
    let wechat_user_service = APPLICATION_CONTEXT.get::<WechatUserService>();
    //判断unionid  存在根据unionid判断
    let mut uid = if let Some(unionid) = user.unionid() {
        let list = wechat_user_service
            .fetch_list_by_column(WechatUser::unionid(), &vec![unionid.clone()])
            .await
            .unwrap();

        if list.len() > 0 {
            let uid = list.get(0).unwrap().id().clone().unwrap();
            //执行更新逻辑
            wechat_user_service
                .update_by_id(uid.to_string(), &mut user.clone().into())
                .await;
            user_service
                .update_by_id(uid.to_string(), &mut build_user_info(&user).into())
                .await;
            uid
        } else {
            0
        }
    } else {
        0
    };
    //如果unionid不存在 则根据openid判断
    if uid == 0 {
        uid = if let Some(routine_openid) = user.routine_openid() {
            let list = wechat_user_service
                .fetch_list_by_column(WechatUser::routine_openid(), &vec![routine_openid.clone()])
                .await
                .unwrap();
            if list.len() > 0 {
                let uid = list.get(0).unwrap().id().clone().unwrap();
                //执行更新逻辑
                wechat_user_service
                    .update_by_id(uid.to_string(), &mut user.clone().into())
                    .await;
                user_service
                    .update_by_id(uid.to_string(), &mut build_user_info(&user).into())
                    .await;
                return uid;
            }
            0
        } else {
            0
        };
        if uid == 0 {
            //执行新增逻辑
            uid = insert_user(user).await;
        }
    }
    uid
}

fn build_user_info(user: &WechatUserDTO) -> UserDTO {
    let mut user_info = UserDTO::default();
    user_info.set_avatar(user.headimgurl().clone());
    user_info.set_nickname(user.nickname().clone());
    user_info
}

pub async fn insert_user(mut user: WechatUserDTO) -> i64 {
    let rb = APPLICATION_CONTEXT.get::<Rbatis>();
    let user_dto = build_user_info(&user);
    //插入用户
    let result = rb.save::<User>(&mut user_dto.into(), &[]).await;
    match result {
        Ok(d) => {
            user.set_id(d.last_insert_id.clone());
            //插入微信用户
            rb.save::<WechatUser>(&mut user.into(), &[]).await;
            d.last_insert_id.unwrap()
        }
        Err(_) => 0,
    }
}
