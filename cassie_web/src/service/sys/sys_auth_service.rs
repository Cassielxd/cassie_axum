use crate::service::ServiceContext;
use crate::APPLICATION_CONTEXT;
use cassie_common::error::Error;
use cassie_common::error::Result;
use cassie_common::utils::password_encoder::PasswordEncoder;
use cassie_config::config::ApplicationConfig;
use cassie_domain::dto::sign_in::SignInDTO;
use cassie_domain::entity::sys_entitys::SysUser;
use cassie_domain::vo::jwt::JWTToken;
use cassie_domain::vo::sign_in::SignInVO;
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use rbatis::DateTimeNative;
use std::time::Duration;

const REDIS_KEY_RETRY: &'static str = "login:login_retry";
/**
*struct:SysAuthService
*desc:用户权限服务  登录 错误重试
*author:String
*email:348040933@qq.com
*/
pub struct SysAuthService {}

impl Default for SysAuthService {
    fn default() -> Self {
        SysAuthService {}
    }
}

impl SysAuthService {
    /**sign_in
     *method:
     *desc:登陆后台
     *author:String
     *email:348040933@qq.com
     */
    pub async fn sign_in(&self, arg: &SignInDTO) -> Result<SignInVO> {
        self.is_need_wait_login_ex().await?;
        /*验证码 验证*/
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let user: Option<SysUser> = rb
            .fetch_by_wrapper(rb.new_wrapper().eq(SysUser::username(), &arg.username))
            .await?;
        let user = user.ok_or_else(|| {
            Error::from(format!(
                "账号:{} 不存在!",
                arg.username.clone().unwrap_or_default()
            ))
        })?;
        if user.status.eq(&Some(0)) {
            return Err(Error::from("账户被禁用!"));
        }
        let mut error = None;
        if !PasswordEncoder::verify(
            user.password
                .as_ref()
                .ok_or_else(|| Error::from("错误的用户数据，密码为空!"))?,
            arg.password
                .as_ref()
                .ok_or_else(|| Error::from("密码不能为空"))?,
        ) {
            error = Some(Error::from("密码不正确!"));
        }
        if error.is_some() {
            self.add_retry_login_limit_num().await?;
            return Err(error.unwrap());
        }
        let sign_in_vo = self.get_user_info(&user).await?;
        return Ok(sign_in_vo);
    }
    /**
     *method:is_need_wait_login_ex
     *desc:用户错误后 是否需要等待
     *author:String
     *email:348040933@qq.com
     */
    pub async fn is_need_wait_login_ex(&self) -> Result<()> {
        let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
        let context = APPLICATION_CONTEXT.get::<ServiceContext>();
        if cassie_config.login_fail_retry > 0 {
            let num: Option<u64> = context.cache_service.get_json(REDIS_KEY_RETRY).await?;
            if num.unwrap_or(0) >= cassie_config.login_fail_retry {
                let wait_sec: i64 = context.cache_service.ttl(REDIS_KEY_RETRY).await?;
                if wait_sec > 0 {
                    return Err(Error::from(format!(
                        "操作过于频繁，请等待{}秒后重试!",
                        wait_sec
                    )));
                }
            }
        }
        return Ok(());
    }

    /**
     *method:add_retry_login_limit_num
     *desc:增加redis重试记录
     *author:String
     *email:348040933@qq.com
     */
    pub async fn add_retry_login_limit_num(&self) -> Result<()> {
        let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
        let context = APPLICATION_CONTEXT.get::<ServiceContext>();
        if cassie_config.login_fail_retry > 0 {
            let num: Option<u64> = context.cache_service.get_json(REDIS_KEY_RETRY).await?;
            let mut num = num.unwrap_or(0);
            if num > cassie_config.login_fail_retry {
                num = cassie_config.login_fail_retry;
            }
            num += 1;
            context
                .cache_service
                .set_string_ex(
                    REDIS_KEY_RETRY,
                    &num.to_string(),
                    Some(Duration::from_secs(
                        cassie_config.login_fail_retry_wait_sec as u64,
                    )),
                )
                .await?;
        }
        return Ok(());
    }
    /**
     *method:get_user_info_by_token
     *desc:根据token获取 暂时没用到
     *author:String
     *email:348040933@qq.com
     */
    pub async fn get_user_info_by_token(&self, token: &JWTToken) -> Result<SignInVO> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let user: Option<SysUser> = rb
            .fetch_by_wrapper(rb.new_wrapper().eq(SysUser::id(), &token.id))
            .await?;
        let user = user.ok_or_else(|| Error::from(format!("账号:{} 不存在!", token.username)))?;
        return self.get_user_info(&user).await;
    }
    /**
     *method:get_user_info
     *desc:获取用户信息
     *author:String
     *email:348040933@qq.com
     */
    pub async fn get_user_info(&self, user: &SysUser) -> Result<SignInVO> {
        let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
        //去除密码，增加安全性
        let mut user = user.clone();
        user.password = None;
        let agency_code = user.agency_code.clone();
        let user_id = user
            .id
            .clone()
            .ok_or_else(|| Error::from("错误的用户数据，id为空!"))?;
        let mut sign_vo = SignInVO {
            user: Some(user.clone().into()),
            access_token: String::new(),
        };
        //提前查找所有权限，避免在各个函数方法中重复查找
        let jwt_token = JWTToken {
            id: user_id,
            super_admin: user.super_admin.clone().unwrap_or_default(),
            username: user.username.clone().unwrap_or(String::new()),
            agency_code: agency_code.unwrap_or_default(),
            exp: DateTimeNative::now().timestamp_millis() as usize,
        };
        sign_vo.access_token = jwt_token.create_token(&cassie_config.jwt_secret)?;
        return Ok(sign_vo);
    }
}
