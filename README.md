## example
 
because `PolishError` implemented `fmt::Display`, you can print errorcode easy.

receive an exoression in Polish notation as `&str`, then `pn` return ans as `f64` or `PolishError`.

---

`PolishError`が`fmt::Display`を実装しているため、エラーコードを簡単に出力できます。

ポーランド記法の式を`&str`として受け取り、`pn`は結果を`f64`または`PolishError`として返します。
```
use polish_notation::PolishError;
use polish_notation::pn;

match pn("+ 5 1") {
    Ok(result) => println!("{}", result),
    Err(e) => eprintln!("{}", e),
};
```
