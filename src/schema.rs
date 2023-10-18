diesel::table! {
    exercise (id) {
        id -> Integer,
        name -> Varchar,
        deleted -> SmallInt,
        lastupdatedate -> Nullable<Varchar>,        
    }
}