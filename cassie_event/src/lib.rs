use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, AttributeArgs, ItemFn, Stmt, FnArg};

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
    

    //拿到方法体
    let fn_body = find_fn_body(target_fn);
    return quote! {
        pub  fn #func_name_ident(#func_args_stream) #return_ty {
            #fn_body
          }
     }
         .into();
}


pub(crate) fn find_return_type(target_fn: &ItemFn) -> proc_macro2::TokenStream {
    let  return_ty = target_fn.sig.output.to_token_stream();
    return_ty
}

pub(crate) fn find_fn_body(target_fn: &ItemFn) -> proc_macro2::TokenStream {
    //del todos
    let mut target_fn = target_fn.clone();
    let mut new_stmts = vec![];
    for x in &target_fn.block.stmts {
        let token = x.to_token_stream().to_string().replace("\n", "").replace(" ", "");
        if token.eq("todo!()") || token.eq("unimplemented!()") || token.eq("impled!()") {
           
        } else {            
            new_stmts.push(x.to_owned());
        }
    }
    let mut args_name = vec![];
    for x in &target_fn.sig.inputs {
        match x {
           
            FnArg::Typed(t) => {
                let ty_stream = t.pat.to_token_stream();
                
                args_name.push(ty_stream);
            }
            _=>{}
        }
    }
      
    target_fn.block.stmts = new_stmts;
    target_fn.block.to_token_stream()
}