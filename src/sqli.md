## fixes

### basic login/search sqli

vuln:

```rust id="1jlwmv"
format!(
    "SELECT * FROM users WHERE username='{}'",
    input
)
```

fix:

```rust id="2jlwmx"
conn.prepare(
    "SELECT * FROM users WHERE username = ?"
)
```

---

### order by injection

vuln:

```rust id="3jlwmc"
format!(
    "SELECT * FROM users ORDER BY {}",
    sort
)
```

fix:

```rust id="4jlwmn"
let allowed = ["username", "email"];

if allowed.contains(&sort) {
    let query = format!(
        "SELECT * FROM users ORDER BY {}",
        sort
    );
}
```

prepared statements usually dont protect column/table names.

---

### table name injection

vuln:

```rust id="5jlwmv"
format!(
    "SELECT * FROM {}",
    table
)
```

fix:

```rust id="6jlwmk"
match table {
    "users" => "users",
    "products" => "products",
    _ => return,
}
```

use allowlists.

---

### like query injection

vuln:

```rust id="7jlwmz"
format!(
    "SELECT * FROM products WHERE name LIKE '%{}%'",
    query
)
```

fix:

```rust id="8jlwmq"
let pattern = format!("%{}%", query);

stmt.query([pattern]);
```

---

### union based sqli

vuln:

```rust id="9jlwmr"
format!(
    "SELECT name FROM products WHERE id='{}'",
    input
)
```

payload:

```txt id="ajlwmj"
' UNION SELECT username FROM users --
```

fix:
prepared statements.

```rust id="bjlwmh"
conn.prepare(
    "SELECT name FROM products WHERE id = ?"
)
```

---

### stacked queries

vuln payload:

```txt id="cjlwmm"
'; DROP TABLE users; --
```

fix:

* prepared statements
* disable multi statement execution
* least privilege db user

---

### blind sqli

payloads:

```txt id="djlwmw"
' AND 1=1 --
```

```txt id="ejlwmx"
' AND 1=2 --
```

fix:
same prepared statement approach.

---

### second order sqli

problem:
malicious payload stored safely first,
executed later in another query.

fix:

* parameterize EVERY query
* even internal/admin queries
* never trust stored data

---

### dynamic query builder vuln

vuln:

```rust id="fjlwmn"
format!(
    "SELECT * FROM users WHERE {}",
    filter
)
```

fix:
dont let users control raw sql fragments.

use:

* query builders
* allowlists
* explicit conditions

---

main fix in almost every case:

* never concatenate untrusted input into sql
* separate sql structure from user data
* use prepared statements everywhere possible
