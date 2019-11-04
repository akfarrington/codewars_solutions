fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut return_list: Vec<T::Item> = Vec::new();

    for item in sequence {
        if return_list.is_empty(){
            return_list.push(item);
        } else if &return_list[return_list.len() - 1] != &item {
            return_list.push(item)
        }
    }

    return_list
}
