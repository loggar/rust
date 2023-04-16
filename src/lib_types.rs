pub fn get_type_of<T>(_: &T) -> &str {
    return std::any::type_name::<T>();
}
