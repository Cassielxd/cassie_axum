use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::Parse,
    parse_macro_input,
    punctuated::Punctuated,
    token::{Else, Let, Semi},
    AttributeArgs, FnArg, Ident, ItemFn, LitBool, LitStr, Local, NestedMeta, Pat, PatIdent, PatType, Token,
};

#[proc_macro_attribute]
pub fn api_operation(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let func = syn::parse::<syn::ItemFn>(item.clone());
    match func {
        Ok(f) => impl_api_operation(&f, &args),
        Err(_) => panic!("只适用于cassie axum  api"),
    }
}

pub(crate) fn formate_params(args: &Vec<NestedMeta>) -> (bool, bool) {
    let mut result = (true, false);
    for arg in args {
        println!("{:#?}", arg);
        match arg {
            NestedMeta::Lit(lit) => match lit {
                syn::Lit::Str(s) => {
                    let data = s.value();
                    let params = data.split("|").collect::<Vec<&str>>();
                    for param in params {
                        let a = param.split("=").collect::<Vec<&str>>();
                        if a.len() > 1 {
                            if a[0] == "result" {
                                result.0 = false;
                            }
                            if a[0] == "return" {
                                result.1 = true;
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
    result
}

pub(crate) fn impl_api_operation(target_fn: &ItemFn, args: &AttributeArgs) -> TokenStream {
    //返回值类型
    let return_ty = find_return_type(&target_fn);
    //方法名称
    let func_name_ident = target_fn.sig.ident.to_token_stream();
    //方法参数
    let func_args_stream = target_fn.sig.inputs.to_token_stream();
    //根据参数构建 params_value
    let params = get_params_value(target_fn.sig.inputs.clone());
    //拿到原始方法体
    let fn_body = target_fn.block.to_token_stream();
    let (is_result, is_return) = formate_params(args);
    //如果 返回值是result类型
    let res = if is_result {
        quote! {
            RespVO::from_result(&result).resp_json()
        }
    } else {
        quote! {
            RespVO::from(&result).resp_json()
        }
    };
    //如果开启了 返回值透传到 jsRuntime中 则获取
    let rerurn = if is_return {
        quote! {
            let result_values=serde_json::to_value(result.clone()).unwrap();
        }
    } else {
        quote! {
            let result_values=serde_json::Value::Null;
        }
    };
    return quote! {
       pub async fn #func_name_ident(#func_args_stream) #return_ty {
         use cassie_common::RespVO;
         use crate::service::fire_event;
          #params
          let result =  #fn_body ;
          #rerurn
          //事件发送代码
          fire_script_event(parm, result_values).await;
          #res
         }
    }
    .into();
}

pub(crate) fn get_params_value(inputs: Punctuated<FnArg, Token![,]>) -> proc_macro2::TokenStream {
    let mut stram = quote!(
        use std::collections::HashMap;
        let mut parm :HashMap < String, serde_json :: Value > = HashMap :: new();
    );
    //参数个数不确定 循环拿到参数
    for fnd in inputs {
        if let FnArg::Typed(PatType { pat, .. }) = fnd {
            match *pat {
                Pat::Ident(i) => {
                    let dent = i.ident.clone();
                    let name = i.ident.clone().to_string();
                    let data = quote!(
                        parm.insert(#name.to_string(),serde_json::to_value(#dent.clone()).unwrap());
                    );
                    stram = quote!(
                        #stram
                        #data
                    );
                }
                _ => {}
            }
        }
    }
    stram
}

pub(crate) fn find_return_type(target_fn: &ItemFn) -> proc_macro2::TokenStream {
    let return_ty = target_fn.sig.output.to_token_stream();
    return_ty
}
