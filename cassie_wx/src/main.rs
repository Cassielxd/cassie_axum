use cassie_wx::{sdk::aes_crypt::AesCrypt};

 fn main() {
   /* 
   let session_key = get_session_key(
        "wx726d55ff001bf506",
        "407659215044413ef53fcf67b0690ac0",
        "013gKHkl2Vwb394xKTkl2WH3Cl0gKHkg",
    )
    .await.unwrap();
   let a =  session_key.clone().to_string();
    */
    let key = base64::decode("pT/BtKBXiTy8V72e7yB3xA==").unwrap();
    let data ="0DK1VFZ97TM9UnCH+sLF376tLXx3wTbmoYMBLc9BqdprudSMzKkc3oz9gAAQ6tp/Dwqx9auuPNSEK0kNoq6cj2s8g/K4eAu3c3hW7V5EbTvdPoSlnG30GUBJh/OCjF2p1TaUFdDxgOianQATrjruvXNOYbtyYTABr+vZmAMu90CnMUOBGZ+Yu+EhxPbXeUtKzO4OYb86MIuI215P6B8QmR5AyouMU38sHvcnA/14lZaA1O5ig5rsKXir9t3Uh0z9Fb8oHgg47vLUKFxzF/fUuoggkEKlmLDZy9l/qtJ0AewjMvh7ZXQ/GMMF858XVEGw7DnOAFvwpolNaTZA5o0wHIq7aquY3kBWRpO+7YbpEKaVPvR7qlr5KBhubvryedSCgetx48pOsuhhuSG5IPFQu77ozwT6Y3PqiAe4VvP1iDtfxNh7BdqZTp/GaFwnjBViDnc7uQ1+QO3kn1CRZTf7pCCob07m8V+cDLve0RX9F/4=";
    let iv =base64::decode( "jHfF1mrkdTrG3Qeq58BGwA==").unwrap();
    let aes = AesCrypt::new(key, iv);
    let data = aes.decrypt(data.to_string());
    println!("{:?}", data);
}