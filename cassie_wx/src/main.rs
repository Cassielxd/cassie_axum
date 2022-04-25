use cassie_wx::{
    sdk::aes_crypt::AesCrypt,
    wxapp::{auth::get_session_key, resolve_data, WxappSessionKey},
};

#[tokio::main]
async fn main() {
    let data = get_session_key(
        "wx726d55ff001bf506",
        "407659215044413ef53fcf67b0690ac0",
        "043DBFkl2bmD394DPGkl2i97bx3DBFko",
    )
    .await
    .unwrap();
    let d: WxappSessionKey = serde_json::from_value(data).unwrap();

    println!("aaa:{:?}", d);

    let data ="ePXMRdWQAJUCB5SRF3llJuHp1B4leLSx0pYxuhcKyJ6hgd2tsbysjPCAfgVbLTgmwR97yKR+ktM5VRoswVm2L7N8yKso0Zn+0WrEGSXpm55EUEvIpy0DcwCeznR+uQIKw7qIjeuE16mTn8g3nZM3uYn2yID3XnWE8hVIdrAd2YJ6F5aNdYPC0KQgR4oN9nX3snqg/DJZkdOA6EKe29fTXz4xp43ZZxT8pcyAyTlCdsAIJlqwEg8WJqJBIh/Bm6ECepxk8xiqPUoj7UWqtr6W5udZ45uiHL0FuwD0XexlQFP57vxbcj8Czl4Us42AoYXnh7rUHCYvNPPDHQM/DbRJHyyvBaKCoCBjzutaNAXNhDb2MOZHc8XzFMrfJMVBdnN5LyPQ4lY4rp8NzO7IY8T0DaQ2VSC6tJdJ1sJPvcaicbeqmtb3ym8hFmdzuW4sLvDyVHTGy2f1AYKeyr+dYqXZrfJf7qcL08xTU3GPoCsibSY=";

    let d = resolve_data(
        "TSaaBQMoZp7jg2+cYpiUDw==".to_string(),
        "fMNK7JnhIWKn3o2Akog04Q==".to_string(),
        data.to_string(),
    );
    println!("{:?}", d);
}
