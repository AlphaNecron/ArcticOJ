//
// enum ExecutorType {
//     Compiled,
//     Script
// }
//
// pub(super) struct ExecutorArgs {
//     pub(super) bin: LitStr,
//     pub(super) ty: ExecutorType,
//     pub(super) default_compile_args: Punctuated<LitStr, Token![,]>,
//     pub(super) get_version: Ident,
// }
//
// impl Parse for ExecutorArgs {
//     fn parse(inp: ParseStream) -> syn::Result<Self> {
//         let content;
//         Ok(ExecutorArgs {
//             bin: inp.parse()?,
//             ty: inp.parse()?,
//             default_compile_args: content.parse_terminated(LitStr::parse, Token![,])?,
//             get_version: inp.parse()?,
//         })
//     }
// }
