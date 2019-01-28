# ドキュメント

## 概要
本コードはRustのFFIを用いてC言語で書かれたHTTPサーバー(server.c)を呼び出し、
POSTメソッドで内部カウンタを+1、GETメソッドで内部カウンタの現在の数値を出力するプログラムである。

### server.c/server.rsについて
- server.cは単純なTCPソケットを利用したHTTPサーバーであり、単独では機能せずserver.rsによって呼び出される。
- server.rsではC言語のインターフェースが定義されており、それぞれに対応するRust版のラッパー関数によってunsafeに呼び出される。
- server.rsとserver.c間の状態は、callbackを保持するContext構造体を介して共有される。Context構造体はプログラムの開始時に一度だけ呼び出される。
- server.cから渡された文字列はborrowedからownedな状態にしてserver.rs側で処理される。
server.rsからserver.cに渡す文字列(body)については、一旦`CString::new(body)`によって複製されownedした後その値のポインタを渡している。
- この点、オリジナルのプログラムではborrowedな状態の文字列(body: &str)を直接ポインタとして取り出そうとしていたところにバグがあったが、unsafeで囲まれていたためコンパイル等によるチェックがなされなかった。

### controller.rsについて
- callbackの実態はcontroller.rsで定義されるexecute_wrapper()関数である。
- クライアントからのリクエストが呼び出されるたびに、パターンマッチによってGET/POSTメソッドを判定し、POSTであれば内部のカウンタを＋１、GETであれば内部のカウンタの現在値を返す。
- 内部カウンタの値はMutexGuardで参照され、インクリメントは`*data = *data + 1`で行われる。

### counter.rsについて
- カウンタがスレッドセーフに機能するために、Mutexを用いて排他制御を行っている。
- Counterインスタンスがプログラム実行中に一つだけであるように、シングルトンデザインパターンが用いられている。
- COUNTERはCounterのポインタであり、毎回の呼び出し時にstatic領域にNullポインタとして確保される。
- 初回呼び出し時のみ初期値を0としたCounterがヒープ領域に割り当てられ、そのポインタがCOUNTERに格納される。
- COUNTERはstatic領域で定義されているため、この操作はunsafeとしてコンパイラに伝えなければならない。
- また、(*COUNTER)の操作もunsafeであるため、コンパイラに明示しなければならない。