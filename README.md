# hello-rust
rustを学んだログ

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

# pyo3

- https://github.com/PyO3/pyo3/blob/master/README.md  
``````
Rust	Python
i32, usize 等	int
f32, f64	float
bool	bool
Vec<T>	list
String	str
HashMap	dict
``````

# ファイル分割

分割したいよね
- https://qiita.com/skitaoka/items/753a519d720a1ccebb0d

``````
pub mod bar{
    pub fn hoge(){}
}
``````
として別ファイル名foo.rsにした上で
``````
mod foo;
foo::bar::hoge();
``````

ディレクトリにする時は、__init__.pyみたいにmod.rsを用意する必要があるっぽい


# クラス的な何か

struct, impl, traitをつかう
- https://qiita.com/nacika_ins/items/cf3782bd371da79def74
概ね理解

# 全体感
とりあえず以下一通り
- https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/learn-rust.html
    - diner/dinerとguessing-game/guessing_gameがこの文献のサンプル触ったログ
    
なんかつくってみる
