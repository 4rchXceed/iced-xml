use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Field, ItemStruct, LitStr, Result, Token, Type,
    parse::{Parse, ParseStream, Parser},
    parse_macro_input,
};

struct CssWatcherMacroArgs {
    state_type: Type,
    css_watcher_file: LitStr,
}

impl Parse for CssWatcherMacroArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let state_type = input.parse()?;
        let _comma: Token![,] = input.parse()?;
        let css_watcher_file = input.parse()?;

        Ok(CssWatcherMacroArgs {
            state_type,
            css_watcher_file,
        })
    }
}

#[proc_macro_attribute]
pub fn window(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let state_type = parse_macro_input!(args as Type);

    let struct_name = item_struct.ident.clone();

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        let query_builder_field = match Field::parse_named.parse2(quote! {
            qb: iced_xml::dom::query_builder::QueryBuilder<Self, #state_type>
        }) {
            Ok(f) => f,
            Err(e) => return e.to_compile_error().into(),
        };

        let engine_field = match Field::parse_named.parse2(quote! {
            engine: iced_xml::xml_engine::XmlEngine
        }) {
            Ok(f) => f,
            Err(e) => return e.to_compile_error().into(),
        };

        fields.named.push(query_builder_field);
        fields.named.push(engine_field);
    }

    let expanded = quote! {
        impl iced_xml::window_wrapper::WindowTemplate<Self, #state_type> for #struct_name {
            fn get_objects(&mut self) -> iced_xml::window_wrapper::Objects<'_, Self, #state_type> {
                return iced_xml::window_wrapper::Objects {
                    engine: &mut self.engine,
                    qb: &mut self.qb,
                };
            }

            fn get_objects_read_only(&self) -> iced_xml::window_wrapper::ObjectsReadOnly<'_, Self, #state_type> {
                return iced_xml::window_wrapper::ObjectsReadOnly {
                    engine: &self.engine,
                    qb: &self.qb,
                };
            }

            fn get_self(&mut self) -> &mut Self {
                return self;
            }

            fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
                return self;
            }
        }
    };

    quote! {
        #item_struct
        #expanded
    }
    .into()
}

#[proc_macro_attribute]
pub fn window_css_watcher(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let args = parse_macro_input!(args as CssWatcherMacroArgs);

    let state_type = args.state_type;
    let css_watcher_file = args.css_watcher_file;

    let struct_name = item_struct.ident.clone();

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        let qb_field = match Field::parse_named.parse2(quote! {
            qb: iced_xml::dom::query_builder::QueryBuilder<Self, #state_type>
        }) {
            Ok(f) => f,
            Err(e) => return e.to_compile_error().into(),
        };

        let engine_field = match Field::parse_named.parse2(quote! {
            engine: iced_xml::xml_engine::XmlEngine
        }) {
            Ok(f) => f,
            Err(e) => return e.to_compile_error().into(),
        };

        let css_rx_field = match Field::parse_named.parse2(quote! {
            css_rx: Option<iced_xml::window_wrapper::CssRx>
        }) {
            Ok(f) => f,
            Err(e) => return e.to_compile_error().into(),
        };

        fields.named.push(qb_field);
        fields.named.push(engine_field);
        fields.named.push(css_rx_field);
    }

    let expanded = quote! {
        impl iced_xml::window_wrapper::WindowTemplate<Self, #state_type> for #struct_name {
            fn get_objects(&mut self) -> iced_xml::window_wrapper::Objects<'_, Self, #state_type> {
                return iced_xml::window_wrapper::Objects {
                    engine: &mut self.engine,
                    qb: &mut self.qb,
                    css_path: #css_watcher_file,
                    css_watcher_rx: self.css_rx.as_mut()
                };
            }

            fn get_objects_read_only(&self) -> iced_xml::window_wrapper::ObjectsReadOnly<'_, Self, #state_type> {
                return iced_xml::window_wrapper::ObjectsReadOnly {
                    engine: &self.engine,
                    qb: &self.qb,
                };
            }

            fn set_css_watcher_rx(&mut self, rx: iced_xml::window_wrapper::CssRx) {
                self.css_rx = Some(rx);
            }

            fn get_self(&mut self) -> &mut Self {
                return self;
            }

            fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
                return self;
            }
        }
    };

    quote! {
        #item_struct
        #expanded
    }
    .into()
}
