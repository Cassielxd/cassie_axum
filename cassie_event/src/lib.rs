use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, AttributeArgs, ItemFn, FnArg, Local, token::{Let, Semi}, PatIdent, parse::Parse, punctuated::Punctuated, Pat, Ident, PatType, Token};

#[proc_macro_attribute]
pub fn api_operation(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let func = syn::parse::<syn::ItemFn>(item.clone()).expect("只适用于函数");
    let stream = impl_api_operation(&func, &args);
    
    stream
}


pub(crate) fn impl_api_operation(target_fn: &ItemFn, args: &AttributeArgs) -> TokenStream {
    //返回值类型
    let return_ty =  find_return_type(&target_fn);
    //方法名称
    let func_name_ident = target_fn.sig.ident.to_token_stream();
    //方法参数
    let func_args_stream = target_fn.sig.inputs.to_token_stream();
    let params = get_params_value(target_fn.sig.inputs.clone());
    //拿到方法体
    let fn_body = target_fn.block.to_token_stream();
    return quote! {
        pub async fn #func_name_ident(#func_args_stream) #return_ty {
          use cassie_common::RespVO;
          use crate::service::fire_event;
           #params
           let result =  #fn_body ;
           let result_values=serde_json::to_value(result.clone()).unwrap();
           println!("{:#?}",parm);
           //事件发送代码
           fire_script_event(parm, result_values).await;
           RespVO::from_result(&result).resp_json()
          }
     }
         .into();
}

pub(crate) fn get_params_value(inputs: Punctuated<FnArg, Token![,]>)-> proc_macro2::TokenStream{
    let mut stram = quote!(
        use std::collections::HashMap;
        let mut parm :HashMap < String, serde_json :: Value > = HashMap :: new(); 
    );
    for fnd in inputs{
        if let FnArg::Typed(PatType { pat,..}) = fnd{
            match *pat {
                Pat::Ident(i) => {
                    let dent=i.ident.clone();
                    let name=i.ident.clone().to_string();
                   let data = quote!(
                        parm.insert(#name.to_string(),serde_json::to_value(#dent.clone()).unwrap());
                    );
                    stram = quote!(
                        #stram
                        #data
                        
                    );
                },
                _ => {},
            }
        }
    }
    stram
}


pub(crate) fn find_return_type(target_fn: &ItemFn) -> proc_macro2::TokenStream {
    let  return_ty = target_fn.sig.output.to_token_stream();
    return_ty
}