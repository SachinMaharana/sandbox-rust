fn main() {
    let fav: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = fav {
        println!("{}", color);
    } else if is_tuesday {
        println!("Tuesday");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("30")
        } else {
            println!(" less 30")
        }
    } else {
        println!("Som,e");
    }
}

// kubectl exec -n sand -it redis-cluster-0 -- redis-cli --cluster create --cluster-replicas 1 \
// $(kubectl get pods -n sand -l app=redis-cluster -o jsonpath='{range.items[*]}{.status.podIP}:6379 ')
