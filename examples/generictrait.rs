fn main() {
    let names = NameSetWithId {
        data: vec![
            RecordWithId {
                id: 43,
                _descr: "John".to_string(),
            },
            RecordWithId {
                id: 78,
                _descr: "lol".to_string(),
            },
        ],
    };

    println!(
        "{} {} {} {}",
        is_present(&names, 43),
        is_present(&names, 89),
        names.count(34),
        names.count(43)
    )
}

trait Searchable<Key, Count> {
    fn contains(&self, key: Key) -> bool;
    fn count(&self, key: Key) -> Count;
}

fn is_present<Collection>(coll: &Collection, id: u32) -> bool
where
    Collection: Searchable<u32, usize>,
{
    coll.contains(id)
}

struct RecordWithId {
    id: u32,
    _descr: String,
}

struct NameSetWithId {
    data: Vec<RecordWithId>,
}

impl Searchable<u32, usize> for NameSetWithId {
    fn contains(&self, key: u32) -> bool {
        for record in self.data.iter() {
            if record.id == key {
                return true;
            }
        }
        false
    }

    fn count(&self, key: u32) -> usize {
        let mut c = 0;
        for record in self.data.iter() {
            if record.id == key {
                c += 1
            }
        }
        c
    }
}
