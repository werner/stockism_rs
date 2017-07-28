macro_rules! get_last_scoped_id {
	($conn: ident, $table: ident, $struct: ident, $field: expr) => {
        {
            $table
                .order($field)
                .first::<$struct>(&*$conn)
                .expect("Error loading records")
                .scoped_id.unwrap_or(0) + 1
        }
	}
}


