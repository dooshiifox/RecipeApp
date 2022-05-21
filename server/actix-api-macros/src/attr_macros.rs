use syn::Ident;

#[derive(Debug, Clone)]
pub enum AttrFound<'a, T> {
    /// The [sub-]attribute exists and the value is of the correct type.
    ///
    /// Example
    ///
    /// ```ignore
    /// #[success(message = "Hello World")]
    /// ```
    FoundWithData(&'a Ident, T),
    /// The [sub-]attribute exists but has no value.
    ///
    /// Example
    ///
    /// ```ignore
    /// #[success(message)]
    /// ```
    Found(&'a Ident),
    /// The [sub-]attribute exists but the value has the wrong type.
    ///
    /// If a type is expected but none is given in the [sub-]attr,
    /// instead return `Found`.
    ///
    /// Example
    ///
    /// ```ignore
    /// #[success(message = 123)]
    /// // Message would expect a string but found an integer.
    /// ```
    FoundWrongType(&'a Ident),
    /// The [sub-]attribute was not found in the attribute.
    ///
    /// Example
    ///
    /// ```ignore
    /// #[success(json)]
    /// // Message would be missing.
    /// ```
    NotFound,
}

impl<'a, T> AttrFound<'a, T> {
    pub fn found(&self) -> Option<(&'a Ident, Option<&T>)> {
        match &self {
            AttrFound::FoundWithData(ident, data) => Some((ident, Some(data))),
            AttrFound::Found(ident) => Some((ident, None)),
            AttrFound::FoundWrongType(ident) => Some((ident, None)),
            AttrFound::NotFound => None,
        }
    }
}

macro_rules! attr {
    ($attr:ident == #[$name:tt]) => {{
        match &$attr {
            ::syn::Meta::Path(path) => {
                path.get_if_ident(stringify!($name))
            },
            _ => None
        }
    }};

    ($attr:ident == #[$name:tt(..)]) => {{
        match &$attr {
            ::syn::Meta::List(list) => {
                list.path.get_if_ident(stringify!($name)).map(|ident|
                    (
                        ident,
                        list.nested.iter().collect::<Punctuated<&NestedMeta, Comma>>()
                    )
                )
            },
            _ => None
        }
    }};

    ($attr:ident == #[$name:tt(..)?]) => {{
        if let Some(i) = attr!($attr == #[$name]) {
            AttrFound::Found(i)
        } else if let Some((i, m)) = attr!($attr == #[$name(..)]) {
            AttrFound::FoundWithData(i, m)
        } else {
            AttrFound::NotFound
        }
    }};

    // An attribute of the form
    //
    // ```
    // #[status_code = 12345]
    // #[message("Hello World")]
    // #[json = true]
    // ```
    //
    // Choice of parenthesis or equals sign is *up to the user*
    //
    // Returns a `FoundWithData<AttrFound<(Ident, $type)>>`
    ($attr:ident == #[$name:tt($type:tt)]) => {{
        match &$attr {
            Meta::NameValue(nv) => {
                match nv.path.get_if_ident(stringify!($name)) {
                    // Return FoundWithData(Ident, None) if lit! is invalid type.
                    Some(ident) => AttrFound::FoundWithData(
                        ident,
                        lit!(nv.lit.clone() => $type)
                    ),
                    None => AttrFound::NotFound
                }
            },
            Meta::Path(path) => {
                match path.get_if_ident(stringify!($name)) {
                    Some(ident) => AttrFound::Found(ident),
                    None => AttrFound::NotFound
                }
            },
            Meta::List(list) => {
                match list.path.get_if_ident(stringify!($name)) {
                    Some(ident) => {
                        // Check there is only one item in the list.
                        // Prevents things like `#[status_code(123, 456)]`
                        if list.nested.iter().len() == 1 {
                            // Check the item is a literal.
                            let nested = &list.nested;
                            match sub_attr!(nested has [:literal]) {
                                Some(lit) => {
                                    AttrFound::FoundWithData(
                                        ident,
                                        lit!(lit.clone() => $type)
                                    )
                                },
                                None => AttrFound::FoundWrongType(ident)
                            }
                        } else {
                            AttrFound::FoundWrongType(ident)
                        }
                    },
                    None => AttrFound::NotFound
                }
            },
        }
    }};

    // Returns an error if the type is not valid.
    ($attr:ident == #[$name:tt($type:tt!)]) => {{
        match attr!($attr == #[$name($type)]) {
            AttrFound::NotFound => Ok(None),
            AttrFound::FoundWrongType(ident) => err!(ident,
                "Expected an attribute of the form `#[{}({})]`", stringify!($name), stringify!($type)
            ),
            AttrFound::Found(ident) => err!(ident,
                "Expected an attribute of the form `#[{}({})]`", stringify!($name), stringify!($type)
            ),
            AttrFound::FoundWithData(ident, data) => {
                // Returns FoundWithData(Ident, None) if the inner value
                // is not the desired type.
                match data {
                    Some(data) => Ok(Some((ident, data))),
                    None => err!(ident,
                        "Type inside parenthesis is not valid - Expected {}", stringify!($type)
                    )
                }
            }
        }
    }};
}

/// `$attr` should be a `Punctuated<NestedMeta, Comma>`
macro_rules! sub_attr {
    // Handles the `sub_attr!(ident has [whatever])` for
    // [name], [name = ..], and [:literal]
    ($attr:ident has $name:tt) => {{
        $attr.iter().find_map(|subattr| {
            sub_attr!(subattr is $name)
        })
    }};

    ($subattr:ident is [$name:tt]) => {{
        if let NestedMeta::Meta(Meta::Path(path)) = $subattr {
            path.get_if_ident(stringify!($name))
        } else {
            None
        }
    }};

    ($subattr:ident is [$name:tt = ..]) => {{
        if let NestedMeta::Meta(Meta::NameValue(n)) = $subattr {
            n.path.get_if_ident(stringify!($name)).map(|ident| (ident, &n.lit))
        } else {
            None
        }
    }};

    ($subattr:ident is [:literal]) => {{
        if let NestedMeta::Lit(lit) = $subattr {
            Some(lit)
        } else {
            None
        }
    }};

    ///////////////////////////////////////////////////////

    ($subattr:ident -> [$name:tt]) => {{
        if let NestedMeta::Meta(Meta::Path(path)) = $subattr {
            if let Some(ident) = path.get_if_ident(stringify!($name)) {
                AttrFound::Found(ident)
            } else {
                AttrFound::NotFound
            }
        } else if let NestedMeta::Meta(Meta::NameValue(n)) = $subattr {
            if let Some(ident) = n.path.get_if_ident(stringify!($name)) {
                AttrFound::FoundWithData(ident, n.lit)
            } else {
                AttrFound::NotFound
            }
        } else {
            AttrFound::NotFound
        }
    }};

    ($attr:ident => [$name:tt]) => {{
        $attr.iter().find(|subattr| {
            sub_attr!(subatrr -> [$name]).is_found()
        }).unwrap_or(AttrFound::NotFound)
    }};

    ///////////////////////////////////////////////////////
    ($subattr:ident -> [$name:tt = $type:tt]) => {{
        if let Some((ident, lit)) = sub_attr!($subattr is [$name = ..]) {
            match lit!(lit => $type) {
                Some(val) => AttrFound::FoundWithData(ident, val),
                None => AttrFound::FoundWrongType(ident),
            }
        } else if let Some(ident) = sub_attr!($subattr is [$name]) {
            AttrFound::Found(ident)
        } else {
            AttrFound::NotFound
        }
    }};

    // Finds an attribute with the given name and attempts to
    // convert to the given type.
    ($attr:ident => [$name:tt = $type:tt]) => {{
        $attr.iter().find(|subattr| {
            sub_attr!(subatrr -> [$name = $type]).is_found()
        }).unwrap_or(AttrFound::NotFound)
    }};

    ///////////////////////////////////////////////////////
    // Throws an error if the sub-attribute is not found or has an invalid type.
    // Else, returns `Ok((ident, $type))`
    //
    // Arrow is either `->` or `=>`
    ($attr:ident $arrow:tt [$name:tt = $type:tt], error if invalid) => {{
        match sub_attr!($attr $arrow [$name = $type]) {
            AttrFound::FoundWithData(ident, val) => Ok((ident, val)),
            AttrFound::Found(ident) => err!(
                ident,
                "Sub-attribute `{}` requires a value of type `{}`", stringify!($name), stringify!($type),
            ),
            AttrFound::NotFound => err!(
                ident,
                "Sub-attribute `{} = {}` not found", stringify!($name), stringify!($type),
            ),
        }
    }};

    // Throws an error if the sub-attribute is not found.
    // Else, returns `Ok((ident, Option<$type>))`
    //
    // Arrow is either `->` or `=>`
    ($attr:ident $arrow:tt [$name:tt = $type:tt], error if not found) => {{
        match sub_attr!($attr $arrow [$name = $type]) {
            AttrFound::FoundWithData(ident, val) => Ok((ident, Some(val))),
            AttrFound::Found(ident) => Ok((ident, None)),
            AttrFound::NotFound => err!(
                ident,
                "Sub-attribute `{}[ = {}]` not found", stringify!($name), stringify!($type),
            ),
        }
    }};

    ///////////////////////////////////////////////////////

    // Throws an error if the sub-attribute is an invalid type or has no value.
    // Returns `Ok(None)` if the sub-attribute is not found.
    // Returns `Ok(Some(ident, $type))` if the sub-attribute is found and has a valid type.
    ($subattr:ident -> [$name:tt = $type:tt]?) => {{
        match sub_attr!($subattr -> [$name = $type]) {
            AttrFound::FoundWithData(ident, val) => Ok(Some((ident, val))),
            AttrFound::NotFound => Ok(None),
            AttrFound::Found(ident) => err!(
                ident,
                "Sub-attribute `{}` requires a value of type `{}`", stringify!($name), stringify!($type)
            ),
            AttrFound::FoundWrongType(ident) => err!(
                ident,
                "Sub-attribute `{}` has an invalid type `{}`", stringify!($name), stringify!($type)
            ),
        }
    }};

    ///////////////////////////////////////////////////////
    // Checks if the sub-attribute is found.
    // If it is and it has a value, checks if it is bool.
    // Found        Not Bool    ..          Err(syn::Error)
    // Found        Bool        True        Ok(Some((ident, true)))
    // Found        Bool        False       Ok(Some((ident, false)))
    // Found        Empty                   Ok(Some((ident, true)))
    // Not Found                            Ok(None)
    //
    // Arrow is either `->` or `=>`
    ($attr:ident $arrow:tt [$name:tt is potential]) => {{
        match sub_attr!($attr $arrow [$name = bool]) {
            AttrFound::FoundWithData(ident, val) => {
                // val.0 is the span of the bool
                // val.1 is the value (true/false)
                Ok(Some((ident, val.1)))
            },
            AttrFound::Found(ident) => Ok(Some((ident, true))),
            AttrFound::NotFound => Ok(None),
            AttrFound::FoundWrongType(ident) => err!(
                ident,
                "Sub-attribute `{}` has a value but it is not a bool", stringify!($name)
            ),
        }
    }}
}

macro_rules! lit {
    ($lit:expr => str) => {{
        match $lit {
            syn::Lit::Str(s) => Some((s.span(), s.value())),
            _ => None,
        }
    }};

    ($lit:expr => bytestr) => {{
        match $lit {
            syn::Lit::ByteStr(b) => Some((b.span(), b.value())),
            _ => None,
        }
    }};

    ($lit:expr => byte) => {{
        match $lit {
            syn::Lit::Byte(b) => Some((b.span(), b.value())),
            _ => None,
        }
    }};

    ($lit:expr => char) => {{
        match $lit {
            syn::Lit::Char(c) => Some((c.span(), c.value())),
            _ => None,
        }
    }};

    ($lit:expr => int) => {{
        match $lit {
            syn::Lit::Int(i) => Some((i.span(), i.base10_parse().unwrap())),
            _ => None,
        }
    }};

    ($lit:expr => float) => {{
        match $lit {
            syn::Lit::Float(f) => Some((f.span(), f.base10_parse().unwrap())),
            _ => None,
        }
    }};

    ($lit:expr => bool) => {{
        match $lit {
            syn::Lit::Bool(b) => Some((b.span(), b.value())),
            _ => None,
        }
    }};

    ($lit:expr => verbatim) => {{
        match $lit {
            syn::Lit::Verbatim(v) => Some((v.span(), v)),
            _ => None,
        }
    }};
}

macro_rules! err {
    ($spannable:expr, $msg:expr) => {{
        Err(syn::Error::new($spannable.span(), $msg))
    }};

    ($spannable:expr, $msg:expr, $($arg:expr),*) => {{
        err!($spannable, format!($msg, $($arg),*))
    }};
}
