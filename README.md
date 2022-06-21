# gluesql
오픈소스 컨트리뷰톤 GlueSQL 우대 사항을 위한 repo입니다.
## 소스코드
```rust
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
```
## 실행결과
![run](/Run.png)
