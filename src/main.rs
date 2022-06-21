use gluesql::prelude::*;

fn main() {
    let storage = SledStorage::new("data/doc-db").unwrap();
    let mut glue = Glue::new(storage);
    let sqls = vec![
        "CREATE TABLE Contributor (name TEXT);",
        "INSERT INTO Contributor VALUES ('조기연');",
        "SELECT * FROM Contributor;",
    ];
    
    for sql in sqls {
        let output = glue.execute(sql).unwrap();
        println!("{:?}", output)
    }
}
