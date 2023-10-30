pub struct Directory<'a> {
    path: &'a str,
    files: Option<Vec<File<'a>>>,
    dir: Option<Vec<Directory<'a>>>
}

impl Directory<'_>{
    fn root<'a>() -> Directory<'a>{
        Directory { path: "/", files: None, dir: None }
    }

}

struct File<'a> {
    name: &'a str,
    size: u32,
}

impl File<'_> {
    fn create<'a, 'b: 'a>(name: &'b str, size: u32) -> File<'a> {
        File { name, size }
    }
}


