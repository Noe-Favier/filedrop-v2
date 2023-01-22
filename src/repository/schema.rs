// @generated automatically by Diesel CLI.

diesel::table! {
    fd_file (inode) {
        inode -> Integer,
        locked -> Bool,
        pass_hash -> Text,
        nb_dl -> Integer,
    }
}

diesel::table! {
    fd_user (id) {
        id -> Integer,
        username -> Text,
        pass_hash -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    fd_file,
    fd_user,
);
