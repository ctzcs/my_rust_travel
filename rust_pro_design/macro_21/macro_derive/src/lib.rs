use proc_macro::TokenStream;
use quote::quote;

//自定义derive宏——过程宏
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input:TokenStream) -> TokenStream{
    //将输入的词条流通过syn构建a syntax tree
    let ast = syn::parse(input).unwrap();
    //实现trait
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast:&syn::DeriveInput) -> TokenStream{
    //获得语法树的标识符，可以通过其获取注解类型名字
    let name = &ast.ident;
    //通过quote!可以返回希望返回的代码
    let gen = quote!{
        impl HelloMacro for #name{
            fn hello_macro(){
                //stringify!可以在编译时将#name转换成字符串节省内存
                println!("Hello,Macro! My name is {}",stringify!(#name));
            }
        }
    };
    //将返回的代码转换成词条流
    gen.into()
}