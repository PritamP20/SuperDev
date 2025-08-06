use proc_macro::{Ident, TokenStream};
use quote::quote;
use syn::{token::Struct, Data, DeriveInput, Fields};


#[proc_macro_derive(SerializeNumberStruct)]
pub fn serialise_number_struct(input: TokenStream) -> TokenStream {
    let ast:DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let serialize_fields = match &ast.data {
        Data::Struct(data_struct)=>{
            match &data_struct.fields {
                Fields::Named(fields)=>{
                    let fieldSerialization = fields.named.iter().map(|field|{
                        let field_name = &field.ident;
                        quote!{
                            result.extend_from_slice(&self.#field_name.to_be_bytes());
                        }
                    });
                    quote! {
                        #(#fieldSerialization)*
                    }
                }
                _ => panic!("Only named fields are supported")
            }
        }
        _ => panic!("Only structs are supported")
    };

    let gen = quote! {
        impl serialize_macro_traits::Serialize for #name {
            fn serialize(&self) -> Vec<u8> {
                let mut result = Vec::new();
                #serialize_fields
                result
            }
        }
    };

    gen.into()
    
}

#[proc_macro_derive(DeserializeNumberStruct)]
pub fn deserialize_number_struct(input: TokenStream) -> TokenStream {
    let ast:DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let (field_reads, field_names, total_size) = match &ast.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    let mut offset = 0usize;
                    let mut reads = Vec::new();
                    let mut names = Vec::new();

                    for field in &fields.named {
                        let fname = &field.ident;
                        let size = 4; // i32 = 4 bytes
                        let start = offset;
                        let end = offset + size;

                        reads.push(quote! {
                            let #fname = {
                                let slice: [u8; 4] = base[#start..#end].try_into().map_err(|_| std::fmt::Error)?;
                                i32::from_be_bytes(slice)
                            };
                        });

                        names.push(quote! { #fname });
                        offset = end;
                    }

                    (reads, names, offset)
                }
                _ => panic!("Only named fields are supported"),
            }
        }
        _ => panic!("Only structs are supported"),
    };

    let generated = quote! {
        impl serialize_macro_traits::Deserialize for #name {
            fn deserialize(base: &[u8]) -> Result<Self, std::fmt::Error> {
                if base.len() < #total_size {
                    return Err(std::fmt::Error);
                }

                #(#field_reads)*

                Ok(#name {
                    #(#field_names),*
                })
            }
        }
    };

    generated.into()
}

