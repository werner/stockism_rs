macro_rules! get_last_scoped_id {
	($conn: ident, $table: ident, $struct: ident, $field: expr) => {
        {
            let record = 
                $table
                    .order($field)
                    .first::<$struct>(&*$conn);

            if let Err(_) = record {
                return 1
            }
            record.unwrap().scoped_id.unwrap() + 1
        }
	}
}


