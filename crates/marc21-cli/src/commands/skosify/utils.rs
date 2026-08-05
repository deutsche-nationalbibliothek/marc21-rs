use sophia::api::ns::Namespace;

pub(crate) fn default_rdf_ns() -> Namespace<&'static str> {
    Namespace::new_unchecked_const(
        "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
    )
}

pub(crate) fn default_skos_ns() -> Namespace<&'static str> {
    Namespace::new_unchecked_const(
        "http://www.w3.org/2004/02/skos/core#",
    )
}
