use syn::Ident;

pub trait GetIfIdent {
    fn get_if_ident<I: ?Sized>(&self, i: &I) -> Option<&Ident>
    where
        Ident: PartialEq<I>;
}

impl GetIfIdent for syn::Path {
    fn get_if_ident<I: ?Sized>(&self, ident: &I) -> Option<&Ident>
    where
        Ident: PartialEq<I>,
    {
        if self.is_ident(ident) {
            self.get_ident()
        } else {
            None
        }
    }
}

impl GetIfIdent for syn::Meta {
    fn get_if_ident<I: ?Sized>(&self, i: &I) -> Option<&Ident>
    where
        Ident: PartialEq<I>,
    {
        match self {
            syn::Meta::Path(path) => path.get_if_ident(i),
            syn::Meta::List(list) => list.path.get_if_ident(i),
            syn::Meta::NameValue(nv) => nv.path.get_if_ident(i),
        }
    }
}

impl GetIfIdent for syn::NestedMeta {
    fn get_if_ident<I: ?Sized>(&self, i: &I) -> Option<&Ident>
    where
        Ident: PartialEq<I>,
    {
        match self {
            syn::NestedMeta::Meta(meta) => meta.get_if_ident(i),
            syn::NestedMeta::Lit(_) => None,
        }
    }
}
