use std::collections::HashMap;
use std::sync::LazyLock;

static EXT_GROUPS: LazyLock<HashMap<&str, usize>> = LazyLock::new(|| {
    let mut id = 0;
    let mut map = HashMap::new();

    macro_rules! group {
        ($($ext:literal),+ $(,)?) => {
            id += 1;
            $(map.insert($ext, id);)+
        };
    }

    group!["jpg", "jpeg"];

    map
});

pub fn is_match(ext_a: &str, ext_b: &str) -> Option<bool> {
    if ext_a.eq_ignore_ascii_case(ext_b) {
        return Some(true);
    }

    let ext_a = ext_a.to_ascii_lowercase();
    let ext_b = ext_b.to_ascii_lowercase();
    match (
        EXT_GROUPS.get(ext_a.as_str()),
        EXT_GROUPS.get(ext_b.as_str()),
    ) {
        (Some(id_a), Some(id_b)) => Some(id_a == id_b),
        _ => Some(ext_a == ext_b),
    }
}
