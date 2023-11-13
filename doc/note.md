- @ques rust hack 的小技巧 -> 感觉深入到了 rust 的核心了

- `rust 中 [T]`

- @ques ` There’s a possible approaches here, like`

### end

- trait object safe 是什么意思

  - https://huonw.github.io/blog/2015/01/object-safety/
  - not return self + para not has generic type

- @ques 如何保证`raw pointer` 会有什么问题 ？ 如何去解决这个问题

  - 去某个值+改变某个值 同时操作 -> 空指针 ？
  - 空指针是什么 -> 怎么让空指针出现
  - 如果有一个读一个写在什么时候会出问题

- read 和 write 的冲突，可以搞一个队列

  - read 一群 read，write 一个个的 write

- @ques rust 为什么不容许多个 mut reference

- @ques raw pointer to own -> read

- @ques rust 为什么不能同时有多个`&mut`
- @ques UnsafeCell 是如何实现的
- @ques RefCell Cell

- @ques rust unsafe 如何获取内存地址

  - 获取内存地址然后把它改变

## 2023-09-08 09:03:27

- @ques `f: impl FnOnce() -> u32 + Send + 'static` 这种代码怎么处理

- @ques 如何 drop array

  - 直接 pop 就行 -> 或者倒序 remove

- @ques 怎么把信息从 thread 传到主进程中

  - mpsc::channel

- @ques channel 能设置为范性吗

```rs
let (tx, rx) = mpsc::channel();
```

## thread pool

- @ques 没啥用

  - thread pool 就不用频繁的创建销毁 thread
  - thread 创建消耗并不大，但是 mutex lock 影响整个的速度

- 流程

  - 创建多个 thread，可以自定义数目
  - 传递方法过去，运行
  - 将结果返回回来

- 结构

  - ThreadPool

- @ques 怎么知道 thread 现在是否是空闲状态

- @ques 如果出现排队的情况如何去处理

  - 需要先放到一个数组中，然后 thread 一个个的去运行

- @ques thread 要不要 join 到 main thread 上面

  - 如何不影响 main 运行的 join()

- @ques 只能通过信息传递方法 cross thread

  - 那么怎么知道哪个 thread 执行了呢？
  - 一个接受信息 其他的就都收不到了吗 -> 是的

- @ques 最后如何清理 thread

- @ques 要熟悉每一行代码的作用

## 2023-09-05 08:44:44

- @ques OnceLock 为什么可以跨线程使用 -> 因为他使用了 sync

- @ques thread 的 join 怎么去处理

- @ques rust 连接释放 连接池

### end

```ts
use send_wrapper::SendWrapper;
```

- 也许我创建一个 thread pool 能很好的优化效率
- @ques rust 异步编程

- @todo 优化性能

<<<<<<< HEAD

- @ques 同一个接口和浏览器的对比 ｜ 和 go 对比

  - 为啥浏览器只要 40ms rust 却要 80ms；差了一倍
  - 把接口拿出来 直接去请求

- 感觉 OnceLock 比较慢 这个耗时可能是 println 本身的耗时
- # @ques OnceLock is slow

  - 所有没有必要的类型转换

- @ques ArcSwap +...

- @ques rust get own outer arc
  > > > > > > > dev_arc_swap

## 2023-09-03 10:28:22

- @ques join 的顺序会影响执行的效率吗

- @ques go 的 route 会 block cpu 线程吗 -> 不会

  - go 一个 cpu 核执行多个 xiecheng

- @ques rust name cli 使用 名称 代 "-" 例如 whale-cli

  - 或者只是编译的时候生成这种

- @ques `impl` 是做什么的

- @ques get_tiers run_in_thread

### 2023-08-26 11:31:34

- @todo for loop 有返回值吗

rust show size affect by package

- @ques 怎么在 loop 中改变值 jnkjkijuikojikjnkm

```rs
let mut f: File;
    for item in path_arr {
        f = match File::open(item) {
            Ok(c) => c,
            Err(e) => {
                continue;
            }
        };
    }
```

- @todo 多线程

  - 先尝试使用多线程 优化

- @todo 分离任务

- @todo parseConfig 中的两层 match 怎么处理

- @todo 异步流程

  - 这样很多地方都可以优化 -> 现在所有接口都是 阻塞的

- @ques 有没有好的 match string to enum 的方法

  - 比方 `Cmd['deploy]`
  - 能不能直接比较 enum `cmd == Cmd.Copy` 这种

- @todo 优化性能

  - 所有没有必要的类型转换

- @todo TierInfo 优化

- @todo 使用 linjilei 的接口

- @ques dyn 是干嘛的
- @ques dialoguer title

- @ques std::io::Result vs Result

- @ques 如何转换错误类型？

- @ques Struct to HashMap

- @ques 错误的处理 ->

- @ques 如何将子模块的方法移动出去

- @ques mutex 怎么用

- @ques rust 的组件要怎么使用 ？

- @ques rust interfaces

  - Hashmap

- @ques 哪些包导致这个打包的大小

### end

- 也许我创建一个 thread pool 能很好的优化效率

  - 不用频繁的创建销毁 thread
  - 但是接口如何保证简单有效？-> 像原来一样的使用
    - fn 能不能传递给 thread -> 应该可以 普通的 thread 都有 move

- @ques main with libs

- @ques 如果不使用 dialoguer 会有多大
- @ques 一个 http 用到了 tokio 实在是太大了
- @todo 时间展示有问题
- @todo rust 如何 findLastIndex
- @todo rust 如何接受一个函数作为参数

  - 这样可以计算 fn 的耗时

- @todo thread 值能返回吗

  - 将多个运行的代码的结果返回
  - rec2.join() ?

- @todo rust 计算运行时间 -> 没找到

  - 写一个测试代码？

- @ques thread 的 join 怎么去处理

- @ques rust 连接释放 连接池

- @ques 同一个接口和浏览器的对比 ｜ 和 go 对比
  - 为啥浏览器只要 40ms rust 却要 80ms；差了一倍
  - 把接口拿出来 直接去请求
