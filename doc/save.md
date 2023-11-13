## Cell RefCell Rc|Arc Mutex Rwlock

https://whiztal.io/rust-tips-rc-box-arc-cell-refcell-mutex/

## 为什么不容许单线程的 多个 mutable reference

https://manishearth.github.io/blog/2015/05/17/the-problem-with-shared-mutability/

- @ques Mutex 内部的 lock unlock 是如何实现的

## Object safety

https://huonw.github.io/blog/2015/01/object-safety/
这几条规则搞不清楚
Generic method + Static method 能大概理解
因为这篇文章中的实现，https://huonw.github.io/blog/2015/01/peeking-inside-trait-objects/
如果想搞懂这个 必须知道 vtable 本身是如何实现的 -> https://rust-lang.github.io/dyn-upcasting-coercion-initiative/design-discussions/vtable-layout.html

- Sized Self + References Self 无法理解

## 核心

- @ques 跨 thread

- @ques 条件判断 match `if let`

- @ques 循环

- @ques enum 类型转换

- @ques result option 转换，错误相互转换等等之类

- @ques life time

- @ques 异步
  - 自己写一个 future 运行时
  - 找一个 http future 库

## letsgetrusty

https://letsgetrusty.kartra.com/page/oGD35
https://product.letsgetrusty.com/bootcamp-training-2bf?r_done=1

```md
我给 content 添加格式化的时候做了下面的改动，要不要给 vap 同步这些修改

1. 把 prettier 升级 3.0.2；
2. 并且添加了 prettier-plugin-packagejson prettier-plugin-css-order 两个插件来给 css 和 package.json 排序
```

## match 嵌套

- @ques 多层 match 如何处理

```rs
// @ques error 最方便的处理方式
// 这个还比较麻烦
// ？ 就需要一种兼容格式的error
pub fn parse_config() -> Result<(), String> {
    let f: File = match File::open("./config.yaml") {
        Ok(c) => c,
        Err(e) => return Err(format!("File::open config.yaml err={}", e.to_string())),
    };

    let config: Config = match serde_yaml::from_reader(f) {
        Ok(c) => c,
        Err(e) => return Err(format!("serde_yaml::from_reader err={}", e.to_string())),
    };

    match CONFIG.set(Mutex::new(config)) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("OnceLock set err")),
    }
}
```

## request ureq

```rs
use std::collections::HashMap;

use serde::Serialize;

use crate::config::get_config;

pub type Data<T> = HashMap<String, HashMap<String, T>>;
pub fn request<T>(query: &str, variables: &T) -> Result<String, String>
where
    T: Sized + Serialize,
{
    let config = get_config();
    let graphql = String::from(&config.graphql);
    let token = String::from(config.get_token());
    drop(config);

    let resp: String = ureq::post(&graphql)
        .set("authorization", &token)
        .send_json(ureq::json!({
            "query": query,
            "variables": variables,
        }))
        .map_err(|e| e.to_string())
        .unwrap()
        .into_string()
        .map_err(|e| e.to_string())
        .unwrap();

    Ok(resp)
}

```

## 能终止吗

```rs
for x in rx1 {
    println!("Got: {}", x);
}
```
