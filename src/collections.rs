//// https://stackoverflow.com/a/28392068/3747705
#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[test]
fn str_to_str_hashmap_created_successfully() {
    let result = hashmap!["a" => "b", "c" => "d", "e" => "f" ];
    assert_eq!(result.get("a").unwrap(), &"b");
    assert_eq!(result.get("c").unwrap(), &"d");
    assert_eq!(result.get("e").unwrap(), &"f");
}

#[test]
fn str_to_i32_hashmap_created_successfully() {
    let result = hashmap!["a" => 1i32, "c" => 345i32, "e" => 749i32 ];
    assert_eq!(result.get("a").unwrap(), &1);
    assert_eq!(result.get("c").unwrap(), &345);
    assert_eq!(result.get("e").unwrap(), &749);
}


#[test]
fn i32_to_str_hashmap_created_successfully() {
    let result = hashmap![1i32 => "one", 2i32 => "two", 6i32 => "six", 10i32 => "ten" ];
    assert_eq!(result.get(&1).unwrap(), &"one");
    assert_eq!(result.get(&2).unwrap(), &"two");
    assert_eq!(result.get(&6).unwrap(), &"six");
    assert_eq!(result.get(&10).unwrap(), &"ten");
}