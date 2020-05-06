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
processdaemon = { git= "https://github.com/dollarkillerx/Processdaemon/processdaemon" }
```

### examples
``` 
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