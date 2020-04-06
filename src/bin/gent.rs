fn main() {
    type Number = f32;

    fn f1(x: Number) -> Number {
        x
    }

    fn f2(x: Number) -> Number {
        x
    }

    let a: Number = 2.3;
    let b: Number = 3.4;

    println!("{} {}", f1(a), f2(b));

    let names = NameSetWithId {
        data: vec![
            RecordWithId {
                id: 34,
                _descr: "John".to_string(),
            },
            RecordWithId {
                id: 49,
                _descr: "Jane".to_string(),
            },
        ],
    };

    println!("{} {}", is_present(&names, 48), is_present(&names, 49));
}

trait Searchable<Key> {
    fn contains(&self, key: Key) -> bool;
}

struct RecordWithId {
    id: u32,
    _descr: String,
}

struct NameSetWithId {
    data: Vec<RecordWithId>,
}

impl Searchable<u32> for NameSetWithId {
    fn contains(&self, key: u32) -> bool {
        for record in self.data.iter() {
            if record.id == key {
                return true;
            }
        }
        false
    }
}

fn is_present<Collection>(coll: &Collection, id: u32) -> bool
where
    Collection: Searchable<u32>,
{
    coll.contains(id)
}
