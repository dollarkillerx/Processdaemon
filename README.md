# Processdaemon
Process daemon  Rust 通过监听 signal 实现类似进程守护

``` 
SIGHUP       1          /* Hangup (POSIX).  */                          终止进程     终端线路挂断
SIGINT       2          /* Interrupt (ANSI).  */                        终止进程     中断进程 Ctrl+C
SIGQUIT      3          /* Quit (POSIX).  */                            建立CORE文件终止进程，并且生成core文件 Ctrl+\
SIGTERM      15         /* Termination (ANSI).  */                      终止进程     软件终止信号
```
我们主要监听以上信号

### use
``` 
[dependencies]
processdaemon = { git= "https://github.com/dollarkillerx/Processdaemon" }
```

### examples
```rust
//! process daemon
//! Stable operation on systems without nohup

/// process_daemon
/// # Examples
/// ```
/// use std::thread;
/// use processdaemon::process_daemon;
/// thread::spawn(process_daemon);
/// ```
```

### 在后台运行
思路: 主进程创建一个子程序   然后主进程退出
```rust
use std::process::Command;
use std::thread;
use std::env;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        if &args[1] == "start" {
            // 主进程启动 子进程
            let child = Command::new(&args[0])
                .spawn().expect("Child process failed to start.");
            println!("child pid: {}", child.id());
            // 主进程退出
            // child.forget() No Child Left Behind
        }
    } else {
        // 这里为业务逻辑....
        let mut i = 1;
        loop {
            i += 1;
            println!("i: {}",i);
            thread::sleep(Duration::from_millis(500));
        }
    }
}
```
