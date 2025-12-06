mod args;

use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::token::Brace;
use syn::{Fields, Ident, ItemStruct, LitStr, Path, parse_macro_input};

#[derive(Debug, FromMeta)]
#[darling(rename_all = "PascalCase")]
enum ExecutorType {
    Compiled,
    Script,
}

#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
struct ExecutorArgs {
    bin: LitStr,
    ty: ExecutorType,
    default_compile_args: Option<Vec<LitStr>>,
    self_test: Path,
}

#[proc_macro_attribute]
pub fn executor(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ExecutorArgs);
    let mut input = parse_macro_input!(input as ItemStruct);
    let mut f_exec_path = syn::Field::parse_named
        .parse2(quote! { exec_path: String })
        .unwrap();
    let mut f_version = syn::Field::parse_named
        .parse2(quote! { version: String })
        .unwrap();
    let name = input.ident.clone();

    match &mut input.fields {
        Fields::Named(fields) => {
            fields.named.push(f_exec_path);
            fields.named.push(f_version);
        }
        Fields::Unit => {
            input.fields = Fields::Named(syn::FieldsNamed {
                brace_token: Brace::default(),
                named: {
                    let mut p = syn::punctuated::Punctuated::new();
                    p.push(f_exec_path);
                    p.push(f_version);
                    p
                },
            });
            input.semi_token = None;
        }
        Fields::Unnamed(fields) => {
            f_exec_path.ident = None;
            f_version.ident = None;
            fields.unnamed.push(f_exec_path);
            fields.unnamed.push(f_version);
        }
    }

    let body = compiled(name.clone(), args);

    TokenStream::from(quote! {
        #input

        impl #name {
            pub(super) fn new() -> Self {
                Self {
                    exec_path: Default::default(),
                    version: Default::default(),
                }
            }
        }

        #[async_trait::async_trait]
        impl _Executor for #name {
            #body
        }
    })
}

fn compiled(name: Ident, args: ExecutorArgs) -> proc_macro2::TokenStream {
    let ExecutorArgs {
        bin,
        default_compile_args,
        self_test,
        ..
    } = args;
    quote! {
        // async fn judge(&self, env: &'static BoxedEnv, sub: &Submission) -> Result<(), String> {
        //     let span = debug_span!("judge");
        //     // super::compiled::judge(env, sub.time_limit, sub.mem_limit);
        //     tracing::debug!(name = #name, time_limit = sub.time_limit, mem_limit = sub.mem_limit);
        //     Ok(())
        // }

        async fn self_test(&mut self, &env: &'static BoxedEnv) -> Result<String, SelfTestErr> {
            match crate::util::which(#bin) {
                Some(p) => self.exec_path = p.to_string_lossy().to_string(),
                None => return Some(SelfTestErr::NotFound),
            }
            if !crate::util::is_executable(self.exec_path.clone()) {
                return Some(SelfTestErr::NotExecutable);
            }
            match #self_test(self.exec_path.clone()).await {
                Some(ver) => self.version = ver,
                None => return Some(SelfTestErr::Corrupted),
            }
            None
        }
    }
}
