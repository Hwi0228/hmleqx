# hmleq

한컴오피스 한글(HWP/HWPX) **수식 스크립트**(`x = {-b +- sqrt{b^2 - 4ac}} over {2a}` 형태의
수식 편집기 입력 언어)를 파싱해 **serde 직렬화 가능한 AST**로 만드는 Rust 크레이트입니다.
LaTeX 변환기는 기본 피처로 함께 제공됩니다.

- 언어 레퍼런스: [`docs/REFERENCE.md`](docs/REFERENCE.md) -- 토큰 규칙, 키워드 매칭
  방식(구분자 기반 토큰화 + 낱말 전체 일치), 전체 명령어 목록
- 구현 설계/출력 규약: [`docs/DESIGN.md`](docs/DESIGN.md)

## 피처

| 피처 | 기본 | 내용 |
|---|---|---|
| `latex` | ✅ | `latex` 모듈, `to_latex` / `eq_to_latex` |
| `serde` | -- | AST 전체 `Serialize`/`Deserialize` (라운드트립 보장) |

파서/AST만 필요하면 `default-features = false`로 의존성 0개의 최소 빌드가 됩니다:

```toml
hmleq = { version = "0.2", default-features = false, features = ["serde"] }
```

## 사용법

라이브러리 -- AST가 일급 산출물입니다:

```rust
let ast = hmleq::parse("1 over pi")?;

// feature "serde": AST ↔ JSON 라운드트립. 심볼은 정준 키워드 이름으로 직렬화되고
// 역직렬화 때 키워드 테이블로 되해석됩니다.
let json = serde_json::to_string(&ast)?;
assert_eq!(json, r#"{"Frac":{"num":{"Number":"1"},"den":{"Symbol":"pi"},"bar":true}}"#);
assert_eq!(serde_json::from_str::<hmleq::Node>(&json)?, ast);

// feature "latex" (기본): LaTeX 변환.
let latex = hmleq::eq_to_latex("sum _{n=1} ^{inf} {1 over n^2} = {pi^2} over 6")?;
assert_eq!(latex, r"\sum_{n = 1}^{\infty} \dfrac{1}{n^{2}} = \dfrac{\pi^{2}}{6}");
```

CLI (`cli/`의 `hmleq-cli` 크레이트, 바이너리 이름은 `hmleq`):

```console
$ cargo run -q -p hmleq-cli -- 'lim _{x rarrow 0} {sin x} over x = 1'
\lim_{x \rightarrow 0} \dfrac{\sin x}{x} = 1

$ cargo run -q -p hmleq-cli -- --json '1 over pi'
{"Frac":{"num":{"Number":"1"},"den":{"Symbol":"pi"},"bar":true}}

$ echo 'pmatrix { a & b # c & d }' | cargo run -q -p hmleq-cli
\begin{pmatrix} a & b \\ c & d \end{pmatrix}
```

## 언어의 핵심 규칙 (요약)

- 빈칸·`{}`·`^`/`_` 등 **구분자로 낱말을 자른 뒤, 낱말 전체를 키워드 표에서 정확 일치로
  조회**한다. `sinh`는 sinh 하나이고, `sinx`는 키워드가 아니므로 이탤릭 *sinx*다.
- `over`/`atop`은 바로 앞·뒤 항에 붙는다: `{분자} over {분모}`.
- `#`은 줄바꿈, `&`는 정렬/행렬 열 구분, `~`/`` ` ``는 출력용 공백.
- `lim`/`Lim`, `rarrow`/`RARROW`처럼 대소문자가 의미를 바꾸는 키워드만 대소문자를 구분하고,
  나머지는 구분하지 않는다.

자세한 것은 `docs/REFERENCE.md` 참조.

## 테스트

```console
$ cargo test                  # 기본 (latex)
$ cargo test --all-features   # + serde 라운드트립
```

`tests/examples.rs`가 `docs/DESIGN.md` §6의 정준(canonical) 변환 표 전체를,
`tests/serde_roundtrip.rs`가 AST의 JSON 라운드트립을 검증합니다.


## hmleqx improvements

The fork extends the original parser while retaining its Rust library and CLI
interfaces. Current additions include:

| Hancom script | LaTeX output |
|---|---|
| `sqrt 3 of 5` | `\\sqrt[3]{5}` |
| `bar AD` | `\\overline{AD}` |
| `A cap B` | `A \\cap B` |
| `A cup B` | `A \\cup B` |

Ordinary fractions use `\\dfrac`, while fractions nested in superscripts or
subscripts use `\\frac`. Scripted limits and large operators are emitted in
display style when appropriate. See [REFERENCE.md](docs/REFERENCE.md) for the
current syntax and output conventions.

## Python package

The `hmleqx` Python package is a thin binding over the existing Rust parser.
PyO3 exposes the same parser and renderer; there is no second Python
implementation of the lexer, parser, AST, or LaTeX emitter.

### Install from the repository

A Rust toolchain is required when building from source:

```console
python -m pip install .
```

For an editable development build, install Maturin and compile the extension
into the active virtual environment:

```console
python -m pip install maturin
maturin develop
```

Build a wheel without installing it:

```console
maturin build --release
```

### Python API

```python
import hmleqx

latex = hmleqx.to_latex("1 over 2")
print(latex)  # \dfrac{1}{2}

# convert() and eq_to_latex() are aliases of to_latex().
latex = hmleqx.convert("sum _{n=1} ^{inf} 1 over n^2")

# parse() returns the Rust AST as ordinary Python dict/list/scalar objects.
ast = hmleqx.parse("1 over pi")
print(ast)
```

Parsing errors are raised as `ValueError`; invalid Python argument types
raise `TypeError`. The native extension targets the CPython stable ABI from
Python 3.9 onward.

Python API tests can be run with:

```console
python -m unittest discover -s tests/python -v
```
