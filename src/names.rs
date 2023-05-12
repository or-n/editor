
pub fn toggle(name: &str) -> impl Fn(String) -> String + '_ {
    move |contents| { 
        let mut names: Vec<_> = contents.split('\n').collect();
        if names.contains(&name) {
            names.retain(|&x| x != name)
        } else {
            names.push(name)
        }
        names.join("\n")
    }
}
