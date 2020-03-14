# hello-rust
rust

# install

xonshrc : $PATH.append(expanduser('~/.cargo/bin'))  
curl https://sh.rustup.rs -sSf | sh  
rustup update  

# make project

cargo new sample_cargo  
cargo build  
cargo run  

# python連携
  
PyO3というのを使うらしい  
https://ohke.hateblo.jp/entry/2020/02/15/230000  
https://qiita.com/osanshouo/items/671888bdd6afeec1e939  

nightlyが必要  
rustup default nightly  
  
buildしていく  
cargo +nightly build --release  
  
ld: symbol(s) not found for architecture x86_64  
clang: error: linker command failed with exit code 1 (use -v to see invocation)  
  
https://github.com/PyO3/pyo3/issues/172  
pyo3のREADMEによると.cargo/configに追記が必要らしい  
https://github.com/PyO3/pyo3/blob/master/README.md  

vim ~/.cargo/config  
``````
[target.x86_64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]
``````

libhoge.dylibをhoge.soにする  
``````
$ cp target/release/librustpy.dylib target/release/rustpy.so  
$ cd target/release/
$ python
>>> import rustpy
>>> rustpy.pi_times(10)
[0.0, 3.141592653589793, 6.283185307179586, 9.42477796076938, 12.566370614359172, 15.707963267948966, 18.84955592153876, 21.991148575128552, 25.132741228718345, 28.27433388
2308138]
``````

