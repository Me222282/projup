use std::cmp::Ordering;

/// `keys` must be sorted by first string
pub fn parse(text: &str, keys: &[(String, String)]) -> Vec<u8>
{
    let mut result = Vec::with_capacity(text.len());
    
    let mut it = text.bytes().enumerate();
    while let Some((i, c)) = it.next()
    {
        let found = binary_search(keys, c, 0);
        if found.len() == 0
        {
            result.push(c);
            continue;
        }
        
        let s = find_value(&text.as_bytes()[i..], found);
        if let Some(v) = s
        {
            // append replacement
            result.extend_from_slice(v.1.as_bytes());
            // should skip bytes
            if v.0.len() > 1
            {
                // consume n + 1 elements
                it.nth(v.0.len() - 2);
            }
            continue;
        }
        // false alarm
        else
        {
            result.push(c);
        }
    }
    
    return result;
}

fn find_value<'a>(start: &[u8], mut sub_set: &'a [(String, String)]) -> Option<&'a (String, String)>
{
    // add extra zero so that keys which are at end can be found
    for (i, b) in start.iter().skip(1).chain([&0]).enumerate()
    {
        // due to skipped
        let i = i + 1;
        let found = binary_search(sub_set, *b, i);
        // end - 
        if found.len() == 0
        {
            // i is number of valid characters so far
            for s in sub_set
            {
                if s.0.len() == i
                {
                    return Some(s);
                }
            }
            // no keys were completed
            return None;
        }
        
        sub_set = found;
    }
    
    return None;
}
fn valid(value: &(String, String), c: u8, index: usize) -> bool
{
    return value.0.as_bytes().get(index).map_or(false, |b| *b == c);
}
fn binary_search(keys: &[(String, String)], c: u8, index: usize) -> &[(String, String)]
{
    let start = keys.binary_search_by(|s|
    {
        s.0.as_bytes().get(index).map_or(Ordering::Less, |b| b.cmp(&c))
    });
    if start.is_err()
    {
        return &[];
    }
    let start = start.unwrap();
    
    let mut last = start;
    for (i, k) in keys.iter().enumerate().skip(start + 1)
    {
        if valid(k, c, index)
        {
            last = i;
            continue;
        }
        break;
    }
    let mut first = start;
    for (i, k) in keys.iter().enumerate().rev().skip(keys.len() - start)
    {
        if valid(k, c, index)
        {
            first = i;
            continue;
        }
        break;
    }
    
    return &keys[first..(last + 1)];
}