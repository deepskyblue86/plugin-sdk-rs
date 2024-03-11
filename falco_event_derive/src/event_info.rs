use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{braced, bracketed, parse_macro_input, Token};

pub(crate) enum LifetimeType {
    None,
    Ref,
    Generic,
}

pub(crate) fn lifetime_type(name: &str) -> LifetimeType {
    match name {
        "PT_CHARBUF" | "PT_BYTEBUF" | "PT_FSPATH" => LifetimeType::Ref,
        "PT_SOCKADDR"
        | "PT_SOCKTUPLE"
        | "PT_FSRELPATH"
        | "PT_CHARBUFARRAY"
        | "PT_CHARBUF_PAIR_ARRAY"
        | "PT_DYN_sockopt_dynamic_param" => LifetimeType::Generic,
        _ => LifetimeType::None,
    }
}

enum IdentOrNumber {
    Ident(Ident),
    Number(syn::LitInt),
}

impl Parse for IdentOrNumber {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::LitInt) {
            Ok(Self::Number(input.parse()?))
        } else {
            Ok(Self::Ident(input.parse()?))
        }
    }
}

type EventArgInfo = Option<(Token![,], IdentOrNumber, Option<(Token![,], Ident)>)>;

struct EventArg {
    _braces: syn::token::Brace,
    name: syn::LitStr,
    _comma1: Token![,],
    field_type: Ident,
    _comma2: Token![,],
    _field_format: Ident,
    info: EventArgInfo,
}

impl EventArg {
    fn final_field_type(&self) -> Ident {
        if let Some((_, IdentOrNumber::Ident(info), _)) = &self.info {
            Ident::new(
                &format!("{}_{}", self.field_type, info),
                self.field_type.span(),
            )
        } else {
            self.field_type.clone()
        }
    }
}

impl Parse for EventArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(EventArg {
            _braces: braced!(content in input),
            name: content.parse()?,
            _comma1: content.parse()?,
            field_type: content.parse()?,
            _comma2: content.parse()?,
            _field_format: content.parse()?,
            info: if content.peek(Token![,]) {
                Some((
                    content.parse()?,
                    content.parse()?,
                    if content.peek(Token![,]) {
                        Some((content.parse()?, content.parse()?))
                    } else {
                        None
                    },
                ))
            } else {
                None
            },
        })
    }
}

impl ToTokens for EventArg {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut name = Ident::new(&self.name.value(), self.name.span());
        if syn::parse::<Ident>(quote!(#name).into()).is_err() {
            // #name is a keyword
            name = Ident::new(&format!("{}_", name), name.span());
        }

        let field_type = self.final_field_type();

        let (field_ref, field_lifetime) =
            if let Some((_, IdentOrNumber::Number(num), _)) = &self.info {
                // shortcut: only PT_FSRELPATH uses these and it takes a lifetime param,
                // so don't bother supporting const generics without lifetimes just yet
                (proc_macro2::TokenStream::new(), quote!(<'a, #num>))
            } else {
                match lifetime_type(&field_type.to_string()) {
                    LifetimeType::Ref => (quote!(&'a), proc_macro2::TokenStream::new()),
                    LifetimeType::Generic => (proc_macro2::TokenStream::new(), quote!(<'a>)),
                    LifetimeType::None => (
                        proc_macro2::TokenStream::new(),
                        proc_macro2::TokenStream::new(),
                    ),
                }
            };

        quote!(#[allow(non_snake_case)] pub #name: Option<#field_ref crate::event_derive::event_field_type:: #field_type #field_lifetime>)
            .to_tokens(tokens)
    }
}

struct EventInfo {
    _brackets: syn::token::Bracket,
    event_code: Ident,
    _eq: Token![=],
    _braces1: syn::token::Brace,
    name: syn::LitStr,
    _comma1: Token![,],
    _categories: syn::punctuated::Punctuated<Ident, Token![|]>,
    _comma2: Token![,],
    flags: syn::punctuated::Punctuated<Ident, Token![|]>,
    _comma3: Token![,],
    _arg_count: syn::LitInt,
    args: Option<(
        Token![,],
        syn::token::Brace,
        syn::punctuated::Punctuated<EventArg, Token![,]>,
    )>,
}

impl Parse for EventInfo {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident_group;
        let event;
        let args;
        Ok(EventInfo {
            _brackets: bracketed!(ident_group in input),
            event_code: ident_group.parse()?,
            _eq: input.parse()?,
            _braces1: braced!(event in input),
            name: event.parse()?,
            _comma1: event.parse()?,
            _categories: syn::punctuated::Punctuated::parse_separated_nonempty(&event)?,
            _comma2: event.parse()?,
            flags: syn::punctuated::Punctuated::parse_separated_nonempty(&event)?,
            _comma3: event.parse()?,
            _arg_count: event.parse()?,
            args: if event.peek(Token![,]) {
                Some((
                    event.parse()?,
                    braced!(args in event),
                    syn::punctuated::Punctuated::parse_terminated(&args)?,
                ))
            } else {
                None
            },
        })
    }
}

impl EventInfo {
    fn typedef(&self) -> proc_macro2::TokenStream {
        let event_code = &self.event_code;
        let event_type = Ident::new(
            &event_code.to_string().replace("PPME_", ""),
            event_code.span(),
        );

        let mut fields = Vec::new();
        let mut wants_lifetime = false;

        if let Some((_, _, args)) = self.args.as_ref() {
            fields = args.iter().map(|arg| arg.to_token_stream()).collect();
            wants_lifetime = !args.iter().all(|arg| {
                matches!(
                    lifetime_type(&arg.final_field_type().to_string()),
                    LifetimeType::None
                )
            })
        }

        let lifetime = if wants_lifetime {
            quote!(<'a>)
        } else {
            proc_macro2::TokenStream::new()
        };
        let is_large = self.flags.iter().any(|flag| *flag == "EF_LARGE_PAYLOAD");
        let name = &self.name;

        quote!(
        #[allow(non_camel_case_types)]
        #[derive(BinaryPayload)]
        #[derive(Debug)]
        pub struct #event_code #lifetime {
            #(#fields,)*
        }

        impl #lifetime crate::event_derive::EventPayload for #event_code #lifetime {
            const ID: EventType = EventType:: #event_type;
            const LARGE: bool = #is_large;
            const NAME: &'static str = #name;
        })
    }

    fn type_variant(&self) -> proc_macro2::TokenStream {
        let event_code = &self.event_code;
        let event_type = Ident::new(
            &event_code.to_string().replace("PPME_", ""),
            event_code.span(),
        );
        let raw_ident = Ident::new(
            &format!("ppm_event_code_{}", self.event_code),
            self.event_code.span(),
        );

        quote!(#event_type = crate::ffi::#raw_ident as u16)
    }

    fn enum_variant(&self) -> proc_macro2::TokenStream {
        let event_code = &self.event_code;
        let event_type = Ident::new(
            &event_code.to_string().replace("PPME_", ""),
            event_code.span(),
        );
        let mut wants_lifetime = false;

        if let Some((_, _, args)) = self.args.as_ref() {
            wants_lifetime = !args.iter().all(|arg| {
                matches!(
                    lifetime_type(&arg.final_field_type().to_string()),
                    LifetimeType::None
                )
            })
        }

        let lifetime = if wants_lifetime {
            quote!(<'a>)
        } else {
            proc_macro2::TokenStream::new()
        };

        quote!(#event_type(#event_code #lifetime))
    }

    fn enum_match(&self) -> proc_macro2::TokenStream {
        let event_code = &self.event_code;
        let event_type = Ident::new(
            &event_code.to_string().replace("PPME_", ""),
            event_code.span(),
        );
        let raw_ident = Ident::new(
            &format!("ppm_event_code_{}", self.event_code),
            self.event_code.span(),
        );
        quote!(crate::ffi:: #raw_ident => {
            let params = unsafe {
                if <#event_code as crate::event_derive::EventPayload>::LARGE {
                    <#event_code as crate::event_derive::PayloadFromBytes>::read(self.params::<u32>()?)?
                } else {
                    <#event_code as crate::event_derive::PayloadFromBytes>::read(self.params::<u16>()?)?
                }
            };
            AnyEvent::#event_type(params)
        })
    }
}

struct Events {
    events: syn::punctuated::Punctuated<EventInfo, Token![,]>,
}

impl Parse for Events {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Events {
            events: syn::punctuated::Punctuated::parse_terminated(input)?,
        })
    }
}

impl Events {
    fn typedefs(&self) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
        self.events.iter().map(|e| e.typedef())
    }

    fn type_variants(&self) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
        self.events.iter().map(|e| e.type_variant())
    }

    fn enum_variants(&self) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
        self.events.iter().map(|e| e.enum_variant())
    }

    fn enum_matches(&self) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
        self.events.iter().map(|e| e.enum_match())
    }
}

pub fn event_info(input: TokenStream) -> TokenStream {
    let events = parse_macro_input!(input as Events);
    let typedefs = events.typedefs();
    let type_variants = events.type_variants();
    let variants = events.enum_variants();
    let matches = events.enum_matches();
    quote!(
        use falco_event_derive::BinaryPayload;
        use crate::raw_event::RawEvent;

        #(#typedefs)*

        #[derive(Debug)]
        #[allow(non_camel_case_types)]
        #[repr(u16)]
        pub enum EventType {
            #(#type_variants,)*
        }

        #[derive(Debug)]
        #[allow(non_camel_case_types)]
        pub enum AnyEvent<'a> {
            #(#variants,)*
        }

        impl RawEvent<'_> {
            pub fn load_any(&self) -> crate::event_derive::FromBytesResult<crate::Event<AnyEvent>> {
                let any: AnyEvent = match self.event_type as u32 {
                    #(#matches,)*
                    _ => return Err(crate::event_derive::FromBytesError::UnsupportedEventType),
                };

                Ok(crate::Event {
                    metadata: self.metadata.clone(),
                    params: any,
                })
            }
        }
    )
    .into()
}
