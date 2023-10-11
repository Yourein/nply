#[allow(non_snake_case, non_upper_case_globals, dead_code)]
pub(crate) mod ENDPOINT {
    pub mod drive {
        pub mod files {
            pub const create: &str = "drive/files/create";
            pub const find_by_hash: &str = "drive/files/find-by-hash";
            pub const find: &str = "drive/files/find";
        }
    }

    pub mod notes {
        pub const create: &str = "notes/create";
    }
}
