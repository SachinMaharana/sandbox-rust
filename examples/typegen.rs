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

trait Searchable {
    type Key;
    type Count;

    fn contains(&self, key: Self::Key) -> bool;
    fn count(&self, key: Self::Key) -> Self::Count;
}

struct RecordWithId {
    id: u32,
    _descr: String,
}

struct NameSetWithId {
    data: Vec<RecordWithId>,
}

impl Searchable for NameSetWithId {
    type Key = u32;
    type Count = usize;
    fn contains(&self, key: Self::Key) -> bool {
        for record in self.data.iter() {
            if record.id == key {
                return true;
            }
        }
        false
    }

    fn count(&self, key: Self::Key) -> usize {
        let mut c = 0;
        for record in self.data.iter() {
            if record.id == key {
                c += 1
            }
        }
        c
    }
}

fn is_present<Collection>(coll: &Collection, id: <Collection as Searchable>::Key) -> bool
where
    Collection: Searchable,
{
    coll.contains(id)
}
