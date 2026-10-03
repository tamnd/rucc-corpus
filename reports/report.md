# rucc corpus report

230 case results did not come out as expected. They are listed below, worst first.

The corpus holds 3524 programs, each written for one named transformation and each carrying the answer the generator worked out before any C was compiled. Corpus digest `a829fd28e437ce2a`.

That is 200,468 lines of C, 6.5 MiB, in 3,637 files, and it is the denominator for every time and every size below. A compile time with no size next to it cannot be read.

| compiler | version | role |
|---|---|---|
| `gcc-16` | gcc-16 (Ubuntu 16-20260315-1ubuntu1~24~ppa1) 16.0.1 20260315 (experimental) [trunk r16-8100-g3aca3bae8ee] | reference |
| `rucc` | rucc 0.18.11 | under test |

## Did it meet the targets

| target | wanted | got | met |
|---|---|---|---|
| `correctness` | 0 failures | 230 failures | no |
| `code-quality:rucc` | within 10 percent | 24 percent more | no |
| `compile-throughput:rucc` | no worse | 38 percent less | yes |
| `size-model:rucc` | no worse | level | yes |

- `correctness`: every case prints the answer the generator computed, on every compiler, at every level.
- `code-quality:rucc`: the code rucc produces at -O2 is within ten percent of what gcc-16 produces at -O2, counted over the whole corpus by byte.
- `compile-throughput:rucc`: rucc compiles the corpus at least as fast as gcc-16 does, which is the corpus proxy for the throughput target in spec 00.
- `size-model:rucc`: the code rucc produces at -Os is no larger than the code it produces at -O2, and where gcc-16 found something to trade away rucc found something too, since -Os is a different cost function and not a cheaper -O2.

## What happened

| compiler | ran | passed | wrong answer | wrongly rejected | not built yet | wrongly accepted | crashed | skipped |
|---|---|---|---|---|---|---|---|---|
| `gcc-16` | 16988 | 16873 | 0 | 115 | 0 | 0 | 0 | 600 |
| `rucc` | 16988 | 16873 | 0 | 115 | 0 | 0 | 0 | 600 |

## What went wrong

### `asm-goto.branch.c17.33690266`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-O0`.

Expected:

```
the program compiles
```

Got:

```
case.c: In function ‘step’:
case.c:10:5: error: ‘asm’ undeclared (first use in this function)
   10 |     asm goto("testl $1, %0\n\tjnz %l[odd]" : : "r"(x) : "cc" : odd);
      |     ^~~
case.c:10:5: note: each undeclared identifier is reported only once for each function it appears in
case.c:10:8: error: expected ‘;’ before ‘goto’
   10 |     asm goto("testl $1, %0\n\tjnz %l[odd]" : : "r"(x) : "cc" : odd);
      |        ^~~~~
      |        ;
```

The program is `programs/correctness/asm-goto/asm-goto.branch.c17.33690266.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.branch.c17.33690266`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-O1`.

Expected:

```
the program compiles
```

Got:

```
case.c: In function ‘step’:
case.c:10:5: error: ‘asm’ undeclared (first use in this function)
   10 |     asm goto("testl $1, %0\n\tjnz %l[odd]" : : "r"(x) : "cc" : odd);
      |     ^~~
case.c:10:5: note: each undeclared identifier is reported only once for each function it appears in
case.c:10:8: error: expected ‘;’ before ‘goto’
   10 |     asm goto("testl $1, %0\n\tjnz %l[odd]" : : "r"(x) : "cc" : odd);
      |        ^~~~~
      |        ;
```

The program is `programs/correctness/asm-goto/asm-goto.branch.c17.33690266.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.branch.c17.33690266`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-O2`.

Expected:

```
the program compiles
```

Got:

```
case.c: In function ‘step’:
case.c:10:5: error: ‘asm’ undeclared (first use in this function)
   10 |     asm goto("testl $1, %0\n\tjnz %l[odd]" : : "r"(x) : "cc" : odd);
      |     ^~~
case.c:10:5: note: each undeclared identifier is reported only once for each function it appears in
case.c:10:8: error: expected ‘;’ before ‘goto’
   10 |     asm goto("testl $1, %0\n\tjnz %l[odd]" : : "r"(x) : "cc" : odd);
      |        ^~~~~
      |        ;
```

The program is `programs/correctness/asm-goto/asm-goto.branch.c17.33690266.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.branch.c17.33690266`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-O3`.

Expected:

```
the program compiles
```

Got:

```
case.c: In function ‘step’:
case.c:10:5: error: ‘asm’ undeclared (first use in this function)
   10 |     asm goto("testl $1, %0\n\tjnz %l[odd]" : : "r"(x) : "cc" : odd);
      |     ^~~
case.c:10:5: note: each undeclared identifier is reported only once for each function it appears in
case.c:10:8: error: expected ‘;’ before ‘goto’
   10 |     asm goto("testl $1, %0\n\tjnz %l[odd]" : : "r"(x) : "cc" : odd);
      |        ^~~~~
      |        ;
```

The program is `programs/correctness/asm-goto/asm-goto.branch.c17.33690266.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.branch.c17.33690266`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-Os`.

Expected:

```
the program compiles
```

Got:

```
case.c: In function ‘step’:
case.c:10:5: error: ‘asm’ undeclared (first use in this function)
   10 |     asm goto("testl $1, %0\n\tjnz %l[odd]" : : "r"(x) : "cc" : odd);
      |     ^~~
case.c:10:5: note: each undeclared identifier is reported only once for each function it appears in
case.c:10:8: error: expected ‘;’ before ‘goto’
   10 |     asm goto("testl $1, %0\n\tjnz %l[odd]" : : "r"(x) : "cc" : odd);
      |        ^~~~~
      |        ;
```

The program is `programs/correctness/asm-goto/asm-goto.branch.c17.33690266.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.branch.c17.33690266`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-O0`.

Expected:

```
the program compiles
```

Got:

```
case.c:10:8: error: expected `;`, found `goto` [E0400]
```

The program is `programs/correctness/asm-goto/asm-goto.branch.c17.33690266.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.branch.c17.33690266`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-O1`.

Expected:

```
the program compiles
```

Got:

```
case.c:10:8: error: expected `;`, found `goto` [E0400]
```

The program is `programs/correctness/asm-goto/asm-goto.branch.c17.33690266.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.branch.c17.33690266`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-O2`.

Expected:

```
the program compiles
```

Got:

```
case.c:10:8: error: expected `;`, found `goto` [E0400]
```

The program is `programs/correctness/asm-goto/asm-goto.branch.c17.33690266.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.branch.c17.33690266`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-O3`.

Expected:

```
the program compiles
```

Got:

```
case.c:10:8: error: expected `;`, found `goto` [E0400]
```

The program is `programs/correctness/asm-goto/asm-goto.branch.c17.33690266.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.branch.c17.33690266`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-Os`.

Expected:

```
the program compiles
```

Got:

```
case.c:10:8: error: expected `;`, found `goto` [E0400]
```

The program is `programs/correctness/asm-goto/asm-goto.branch.c17.33690266.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.in-loop.c17.340e5676`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-O0`.

Expected:

```
the program compiles
```

Got:

```
case.c: In function ‘bits’:
case.c:12:9: error: ‘asm’ undeclared (first use in this function)
   12 |         asm goto("btl %1, %0\n\tjc %l[set]" : : "r"(x), "r"(k) : "cc" : set);
      |         ^~~
case.c:12:9: note: each undeclared identifier is reported only once for each function it appears in
case.c:12:12: error: expected ‘;’ before ‘goto’
   12 |         asm goto("btl %1, %0\n\tjc %l[set]" : : "r"(x), "r"(k) : "cc" : set);
      |            ^~~~~
      |            ;
```

The program is `programs/correctness/asm-goto/asm-goto.in-loop.c17.340e5676.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.in-loop.c17.340e5676`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-O1`.

Expected:

```
the program compiles
```

Got:

```
case.c: In function ‘bits’:
case.c:12:9: error: ‘asm’ undeclared (first use in this function)
   12 |         asm goto("btl %1, %0\n\tjc %l[set]" : : "r"(x), "r"(k) : "cc" : set);
      |         ^~~
case.c:12:9: note: each undeclared identifier is reported only once for each function it appears in
case.c:12:12: error: expected ‘;’ before ‘goto’
   12 |         asm goto("btl %1, %0\n\tjc %l[set]" : : "r"(x), "r"(k) : "cc" : set);
      |            ^~~~~
      |            ;
```

The program is `programs/correctness/asm-goto/asm-goto.in-loop.c17.340e5676.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.in-loop.c17.340e5676`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-O2`.

Expected:

```
the program compiles
```

Got:

```
case.c: In function ‘bits’:
case.c:12:9: error: ‘asm’ undeclared (first use in this function)
   12 |         asm goto("btl %1, %0\n\tjc %l[set]" : : "r"(x), "r"(k) : "cc" : set);
      |         ^~~
case.c:12:9: note: each undeclared identifier is reported only once for each function it appears in
case.c:12:12: error: expected ‘;’ before ‘goto’
   12 |         asm goto("btl %1, %0\n\tjc %l[set]" : : "r"(x), "r"(k) : "cc" : set);
      |            ^~~~~
      |            ;
```

The program is `programs/correctness/asm-goto/asm-goto.in-loop.c17.340e5676.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.in-loop.c17.340e5676`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-O3`.

Expected:

```
the program compiles
```

Got:

```
case.c: In function ‘bits’:
case.c:12:9: error: ‘asm’ undeclared (first use in this function)
   12 |         asm goto("btl %1, %0\n\tjc %l[set]" : : "r"(x), "r"(k) : "cc" : set);
      |         ^~~
case.c:12:9: note: each undeclared identifier is reported only once for each function it appears in
case.c:12:12: error: expected ‘;’ before ‘goto’
   12 |         asm goto("btl %1, %0\n\tjc %l[set]" : : "r"(x), "r"(k) : "cc" : set);
      |            ^~~~~
      |            ;
```

The program is `programs/correctness/asm-goto/asm-goto.in-loop.c17.340e5676.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.in-loop.c17.340e5676`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-Os`.

Expected:

```
the program compiles
```

Got:

```
case.c: In function ‘bits’:
case.c:12:9: error: ‘asm’ undeclared (first use in this function)
   12 |         asm goto("btl %1, %0\n\tjc %l[set]" : : "r"(x), "r"(k) : "cc" : set);
      |         ^~~
case.c:12:9: note: each undeclared identifier is reported only once for each function it appears in
case.c:12:12: error: expected ‘;’ before ‘goto’
   12 |         asm goto("btl %1, %0\n\tjc %l[set]" : : "r"(x), "r"(k) : "cc" : set);
      |            ^~~~~
      |            ;
```

The program is `programs/correctness/asm-goto/asm-goto.in-loop.c17.340e5676.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.in-loop.c17.340e5676`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-O0`.

Expected:

```
the program compiles
```

Got:

```
case.c:12:12: error: expected `;`, found `goto` [E0400]
```

The program is `programs/correctness/asm-goto/asm-goto.in-loop.c17.340e5676.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.in-loop.c17.340e5676`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-O1`.

Expected:

```
the program compiles
```

Got:

```
case.c:12:12: error: expected `;`, found `goto` [E0400]
```

The program is `programs/correctness/asm-goto/asm-goto.in-loop.c17.340e5676.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.in-loop.c17.340e5676`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-O2`.

Expected:

```
the program compiles
```

Got:

```
case.c:12:12: error: expected `;`, found `goto` [E0400]
```

The program is `programs/correctness/asm-goto/asm-goto.in-loop.c17.340e5676.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.in-loop.c17.340e5676`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-O3`.

Expected:

```
the program compiles
```

Got:

```
case.c:12:12: error: expected `;`, found `goto` [E0400]
```

The program is `programs/correctness/asm-goto/asm-goto.in-loop.c17.340e5676.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.in-loop.c17.340e5676`

rucc would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-Os`.

Expected:

```
the program compiles
```

Got:

```
case.c:12:12: error: expected `;`, found `goto` [E0400]
```

The program is `programs/correctness/asm-goto/asm-goto.in-loop.c17.340e5676.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.output.c17.2a15ca0e`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-O0`.

Expected:

```
the program compiles
```

Got:

```
case.c: In function ‘step’:
case.c:11:5: error: ‘asm’ undeclared (first use in this function)
   11 |     asm goto("movl %1, %0\n\tandl $3, %0\n\tjz %l[zero]"
      |     ^~~
case.c:11:5: note: each undeclared identifier is reported only once for each function it appears in
case.c:11:8: error: expected ‘;’ before ‘goto’
   11 |     asm goto("movl %1, %0\n\tandl $3, %0\n\tjz %l[zero]"
      |        ^~~~~
      |        ;
```

The program is `programs/correctness/asm-goto/asm-goto.output.c17.2a15ca0e.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.output.c17.2a15ca0e`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-O1`.

Expected:

```
the program compiles
```

Got:

```
case.c: In function ‘step’:
case.c:11:5: error: ‘asm’ undeclared (first use in this function)
   11 |     asm goto("movl %1, %0\n\tandl $3, %0\n\tjz %l[zero]"
      |     ^~~
case.c:11:5: note: each undeclared identifier is reported only once for each function it appears in
case.c:11:8: error: expected ‘;’ before ‘goto’
   11 |     asm goto("movl %1, %0\n\tandl $3, %0\n\tjz %l[zero]"
      |        ^~~~~
      |        ;
```

The program is `programs/correctness/asm-goto/asm-goto.output.c17.2a15ca0e.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.output.c17.2a15ca0e`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-O2`.

Expected:

```
the program compiles
```

Got:

```
case.c: In function ‘step’:
case.c:11:5: error: ‘asm’ undeclared (first use in this function)
   11 |     asm goto("movl %1, %0\n\tandl $3, %0\n\tjz %l[zero]"
      |     ^~~
case.c:11:5: note: each undeclared identifier is reported only once for each function it appears in
case.c:11:8: error: expected ‘;’ before ‘goto’
   11 |     asm goto("movl %1, %0\n\tandl $3, %0\n\tjz %l[zero]"
      |        ^~~~~
      |        ;
```

The program is `programs/correctness/asm-goto/asm-goto.output.c17.2a15ca0e.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.output.c17.2a15ca0e`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-O3`.

Expected:

```
the program compiles
```

Got:

```
case.c: In function ‘step’:
case.c:11:5: error: ‘asm’ undeclared (first use in this function)
   11 |     asm goto("movl %1, %0\n\tandl $3, %0\n\tjz %l[zero]"
      |     ^~~
case.c:11:5: note: each undeclared identifier is reported only once for each function it appears in
case.c:11:8: error: expected ‘;’ before ‘goto’
   11 |     asm goto("movl %1, %0\n\tandl $3, %0\n\tjz %l[zero]"
      |        ^~~~~
      |        ;
```

The program is `programs/correctness/asm-goto/asm-goto.output.c17.2a15ca0e.c` and the working directory of the failing build was kept under the run directory.

### `asm-goto.output.c17.2a15ca0e`

gcc-16 would not compile a valid program about asm goto as static keys and user copies use it, with and without outputs. This is a case about asm goto as static keys and user copies use it, with and without outputs, built at `-Os`.

Expected:

```
the program compiles
```

Got:

```
case.c: In function ‘step’:
case.c:11:5: error: ‘asm’ undeclared (first use in this function)
   11 |     asm goto("movl %1, %0\n\tandl $3, %0\n\tjz %l[zero]"
      |     ^~~
case.c:11:5: note: each undeclared identifier is reported only once for each function it appears in
case.c:11:8: error: expected ‘;’ before ‘goto’
   11 |     asm goto("movl %1, %0\n\tandl $3, %0\n\tjz %l[zero]"
      |        ^~~~~
      |        ;
```

The program is `programs/correctness/asm-goto/asm-goto.output.c17.2a15ca0e.c` and the working directory of the failing build was kept under the run directory.

And 205 more, which are all in `findings.sarif` and in `report.json`.

## How big the code is

Code size is the size of the executable sections, not the size of the file, so the runtime and the symbol table do not get counted as somebody's optimizer. Everything below is at `-O2` against `gcc-16` at `-O2`. The per facet figures are medians over the cases in that facet, because a facet holds programs of very different sizes and one tiny program should not set the facet's number. The headline for each compiler is the corpus total instead, every byte counted once, because that is the number that does not move when somebody adds a facet.

The run time column is the same comparison over the fastest of the repetitions of each program, and it says `inside the noise` where the difference between the two compilers is smaller than the difference this machine produced running one of them several times. Size is exact and time is not, so on most of these facets the size column is the honest signal and the time column is there to be checked rather than quoted. Every repetition is in `runs.jsonl` under `execute.samples`, so any number here can be traced back to what was measured.

The instructions column is how many instructions this compiler's programs ran across the facet, less how many the reference's ran. A count rather than a clock, so it comes back the same to within one part in a hundred thousand on a machine where the wall clock moves by a factor of twelve. It is a difference rather than a ratio on purpose. A program in this corpus retires about a hundred and forty thousand instructions and about a hundred and eight thousand of those are the process starting up before `main`, which is the same code on both sides, so a ratio has all of that in its denominator and comes out at one however the compilers differed. A difference has none of it, because a constant on both sides subtracts away exactly. The ratio is still in `report.json` as `instruction_ratio` for anyone who wants to check that. The column says `not measured` where the machine would not count, which is any machine without `perf` and any machine that will not hand a counter to an unprivileged process.

### `rucc`

Over the whole corpus, `rucc` produces 24 percent more. The middle facet is 4 percent less, over 96 facets. The two differ when the larger facets are the ones going badly, and the total is the one to believe.

Furthest behind:

| facet | phase | cases | code size | instructions | run time | compile time |
|---|---|---|---|---|---|---|
| `interpreter-dispatch` | backend | 36 | 203 percent more | not measured | inside the noise | 46 percent less |
| `simd-lfind` | correctness | 30 | 98 percent more | not measured | inside the noise | 46 percent less |
| `bit-builtins` | backend | 22 | 87 percent more | not measured | inside the noise | 40 percent less |
| `overflow-builtins` | correctness | 144 | 57 percent more | not measured | inside the noise | 47 percent less |
| `long-double` | backend | 10 | 49 percent more | not measured | level | 33 percent less |
| `scheduling` | backend | 20 | 43 percent more | not measured | inside the noise | 34 percent less |
| `target-attribute` | correctness | 52 | 14 percent more | not measured | level | 46 percent less |
| `tail-call` | interprocedural | 36 | 10 percent more | not measured | inside the noise | 41 percent less |

Furthest ahead:

| facet | phase | cases | code size | instructions | run time | compile time |
|---|---|---|---|---|---|---|
| `short-circuit` | local | 24 | 37 percent less | not measured | inside the noise | 44 percent less |
| `narrow-shift` | backend | 8 | 36 percent less | not measured | inside the noise | 57 percent less |
| `conditional-store` | local | 20 | 34 percent less | not measured | inside the noise | 46 percent less |
| `stack-slots` | backend | 6 | 33 percent less | not measured | inside the noise | 43 percent less |
| `prune` | global | 15 | 32 percent less | not measured | inside the noise | 43 percent less |
| `value-replacement` | global | 13 | 30 percent less | not measured | inside the noise | 47 percent less |
| `loop-idiom` | loops | 78 | 27 percent less | not measured | level | 38 percent less |
| `value-settled` | local | 13 | 26 percent less | not measured | level | 43 percent less |

## What `-Os` does

Every other number in this report is one compiler against another at the same level. These are one compiler against itself at two, because `-Os` is a different cost function rather than a cheaper `-O2`, so it picks different rewrites, and the question worth asking is whether it picks any. Each figure is that compiler's own code size at `-Os` over its own code size at `-O2`, as a median over the cases that built at both.

| compiler | cases compared | code size at `-Os` | came out the same size |
|---|---|---|---|
| `gcc-16` | 3373 | 4 percent less | 1154 |
| `rucc` | 3373 | level | 1029 |

The row for `gcc-16` is the control. It is a compiler with a size cost model that works, so it says what this measurement looks like when the flag is doing something.

### `rucc` at `-Os`

Where `-Os` saves the most:

| facet | cases | code size at `-Os` |
|---|---|---|
| `scheduling` | 20 | 21 percent less |
| `simd-lfind` | 30 | 19 percent less |
| `compare-fold` | 118 | 10 percent less |
| `store-fold-constant` | 118 | 10 percent less |
| `load-fold` | 40 | 10 percent less |
| `target-attribute` | 52 | 9 percent less |

Where it saves the least, which is where it is spending size and getting nothing for it when the number is above level:

| facet | cases | code size at `-Os` |
|---|---|---|
| `sigsetjmp` | 112 | 6 percent more |
| `function-purity` | 28 | 7 percent more |
| `stack-slots` | 6 | 11 percent more |
| `induction-variable` | 42 | 12 percent more |
| `loop-restructure` | 48 | 13 percent more |
| `loop-idiom` | 78 | 14 percent more |

## By phase of the plan

The phases are the ones in the M4 plan, so this table is the one to read when deciding what to implement next.

| phase | facets | cases | lines | `rucc` passed | `rucc` code size |
|---|---|---|---|---|---|
| floor | 6 | 118 | 3,458 | 558 of 558 | 4 percent less |
| local | 11 | 411 | 20,527 | 2055 of 2055 | 5 percent less |
| global | 12 | 242 | 5,794 | 1210 of 1210 | 5 percent less |
| loops | 12 | 700 | 14,151 | 3500 of 3500 | 6 percent less |
| interprocedural | 14 | 351 | 10,173 in 419 files | 1755 of 1755 | 3 percent less |
| backend | 27 | 1008 | 57,956 | 5040 of 5040 | 4 percent less |
| correctness | 20 | 694 | 88,409 in 739 files | 2755 of 2870 | 1 percent more |

## What gcc-16 said about these programs

Asked with `-fopt-info`, gcc-16 reports what it optimized and what it wanted to optimize and could not. None of it is a pass or fail signal, and a miss is not a bug in anybody. It is a mature compiler's opinion, per facet, about what these programs allow, which is the best available answer to the question of what is worth implementing next.

| facet | phase | it optimized | it says it missed |
|---|---|---|---|
| `crc32c` | correctness | 7840 | 61488 |
| `sigsetjmp` | correctness | 1128 | 17244 |
| `overflow-builtins` | correctness | 0 | 14400 |
| `target-attribute` | correctness | 2505 | 7220 |
| `interpreter-dispatch` | backend | 306 | 7834 |
| `frame-size` | backend | 2311 | 2503 |
| `bundle` | correctness | 396 | 3735 |
| `atomics` | correctness | 0 | 4026 |
| `bit-builtins` | backend | 0 | 3352 |
| `simd-lfind` | correctness | 1290 | 1386 |
| `tail-dispatch` | interprocedural | 0 | 1792 |
| `simplify` | local | 161 | 1135 |
| `inline` | interprocedural | 900 | 372 |
| `store-fold-constant` | backend | 444 | 820 |
| `compare-fold` | backend | 472 | 584 |
| `division` | backend | 203 | 802 |
| `lkmm` | correctness | 132 | 792 |
| `float-conversion` | backend | 0 | 850 |
| `tail-call` | interprocedural | 300 | 480 |
| `call-motion` | interprocedural | 10 | 720 |

## What each switch became

Every case with a `switch` in it is compiled once more with `-S` to see what its switches were lowered as. rucc says so under `-fopt-info`, and gcc-16's assembly is read for a jump table, a bit test or a lookup table of answers, with compares meaning none of the three. A case agrees when both compilers used the same of those. The two do not have to inline the same functions, so a disagreement is a lead to read the assembly for and not a verdict.

| compiler | level | cases | same | different |
|---|---|---|---|---|
| rucc | O0 | 122 | 77 | 45 |
| rucc | O1 | 120 | 72 | 48 |
| rucc | O2 | 120 | 72 | 48 |
| rucc | O3 | 120 | 72 | 48 |
| rucc | Os | 120 | 76 | 44 |

### rucc at O0 against gcc-16

| rucc | gcc-16 | cases |
|---|---|---|
| table | compares | 26 |
| compares | table | 19 |

| case | facet | rucc | gcc-16 |
|---|---|---|---|
| `interpreter-dispatch.12.table.direct.8.c17.fe4a8b3d` | `interpreter-dispatch` | compares | table |
| `interpreter-dispatch.12.table.none.8.c17.fd7b1f54` | `interpreter-dispatch` | compares | table |
| `interpreter-dispatch.12.table.pointer.8.c17.085ef334` | `interpreter-dispatch` | compares | table |
| `interpreter-dispatch.12.threaded.direct.8.c17.ea92116c` | `interpreter-dispatch` | compares | table |
| `interpreter-dispatch.12.threaded.none.8.c17.0a37857e` | `interpreter-dispatch` | compares | table |
| `interpreter-dispatch.12.threaded.pointer.8.c17.31d76aaa` | `interpreter-dispatch` | compares | table |
| `interpreter-dispatch.24.table.direct.8.c17.d33ddfed` | `interpreter-dispatch` | compares | table |
| `interpreter-dispatch.24.table.none.8.c17.dd956486` | `interpreter-dispatch` | compares | table |
| `interpreter-dispatch.24.table.pointer.8.c17.a376142a` | `interpreter-dispatch` | compares | table |
| `interpreter-dispatch.24.threaded.direct.8.c17.288d7cd7` | `interpreter-dispatch` | compares | table |
| `interpreter-dispatch.24.threaded.none.8.c17.1ad7034a` | `interpreter-dispatch` | compares | table |
| `interpreter-dispatch.24.threaded.pointer.8.c17.5cdc26d1` | `interpreter-dispatch` | compares | table |
| `interpreter-dispatch.4.table.direct.8.c17.ad381896` | `interpreter-dispatch` | compares | table |
| `interpreter-dispatch.4.table.none.8.c17.66690ed6` | `interpreter-dispatch` | compares | table |
| `interpreter-dispatch.4.table.pointer.8.c17.f16c570b` | `interpreter-dispatch` | compares | table |
| `interpreter-dispatch.4.threaded.direct.8.c17.81bbc59c` | `interpreter-dispatch` | compares | table |
| `interpreter-dispatch.4.threaded.none.8.c17.47a678c0` | `interpreter-dispatch` | compares | table |
| `interpreter-dispatch.4.threaded.pointer.8.c17.77dc9ec0` | `interpreter-dispatch` | compares | table |
| `objtool-shapes.jump-table.c17.9da06433` | `objtool-shapes` | compares | table |
| `switch-dispatch.affine.in-order.c17.3de0f02c` | `switch-dispatch` | table | compares |
| `switch-dispatch.affine.unpredictable.c17.1095fab2` | `switch-dispatch` | table | compares |
| `switch-dispatch.below-zero.unpredictable.c17.0348e7c3` | `switch-dispatch` | table | compares |
| `switch-dispatch.holes.unpredictable.c17.343584c6` | `switch-dispatch` | table | compares |
| `switch-dispatch.into-letters.unpredictable.c17.6275bbd4` | `switch-dispatch` | table | compares |
| `switch-dispatch.masked.unpredictable.c17.fcb9805f` | `switch-dispatch` | table | compares |

And 20 more, all in `reports/report.json`.

### rucc at O1 against gcc-16

| rucc | gcc-16 | cases |
|---|---|---|
| compares | bit-test, table | 18 |
| table | bit-test, table | 18 |
| compares | table | 12 |

| case | facet | rucc | gcc-16 |
|---|---|---|---|
| `interpreter-dispatch.12.table.direct.32.c17.49df9385` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.table.direct.8.c17.fe4a8b3d` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.12.table.none.32.c17.d9ec709c` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.table.none.8.c17.fd7b1f54` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.12.table.pointer.32.c17.090126a6` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.table.pointer.8.c17.085ef334` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.12.threaded.direct.32.c17.960c9160` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.threaded.direct.8.c17.ea92116c` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.12.threaded.none.32.c17.eec600eb` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.threaded.none.8.c17.0a37857e` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.12.threaded.pointer.32.c17.e1cf7a44` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.threaded.pointer.8.c17.31d76aaa` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.24.table.direct.32.c17.dc0e6741` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.table.direct.8.c17.d33ddfed` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.24.table.none.32.c17.2e0c71b0` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.table.none.8.c17.dd956486` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.24.table.pointer.32.c17.11a0293b` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.table.pointer.8.c17.a376142a` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.24.threaded.direct.32.c17.80c136bc` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.threaded.direct.8.c17.288d7cd7` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.24.threaded.none.32.c17.bb796889` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.threaded.none.8.c17.1ad7034a` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.24.threaded.pointer.32.c17.2aaaaaae` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.threaded.pointer.8.c17.5cdc26d1` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.4.table.direct.32.c17.8f2d2ccd` | `interpreter-dispatch` | table | bit-test, table |

And 23 more, all in `reports/report.json`.

### rucc at O2 against gcc-16

| rucc | gcc-16 | cases |
|---|---|---|
| compares | bit-test, table | 18 |
| table | bit-test, table | 18 |
| compares | table | 9 |
| lookup | compares | 3 |

| case | facet | rucc | gcc-16 |
|---|---|---|---|
| `interpreter-dispatch.12.table.direct.32.c17.49df9385` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.table.direct.8.c17.fe4a8b3d` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.12.table.none.32.c17.d9ec709c` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.table.none.8.c17.fd7b1f54` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.12.table.pointer.32.c17.090126a6` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.table.pointer.8.c17.085ef334` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.12.threaded.direct.32.c17.960c9160` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.threaded.direct.8.c17.ea92116c` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.12.threaded.none.32.c17.eec600eb` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.threaded.none.8.c17.0a37857e` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.12.threaded.pointer.32.c17.e1cf7a44` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.threaded.pointer.8.c17.31d76aaa` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.24.table.direct.32.c17.dc0e6741` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.table.direct.8.c17.d33ddfed` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.24.table.none.32.c17.2e0c71b0` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.table.none.8.c17.dd956486` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.24.table.pointer.32.c17.11a0293b` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.table.pointer.8.c17.a376142a` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.24.threaded.direct.32.c17.80c136bc` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.threaded.direct.8.c17.288d7cd7` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.24.threaded.none.32.c17.bb796889` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.threaded.none.8.c17.1ad7034a` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.24.threaded.pointer.32.c17.2aaaaaae` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.threaded.pointer.8.c17.5cdc26d1` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.4.table.direct.32.c17.8f2d2ccd` | `interpreter-dispatch` | table | bit-test, table |

And 23 more, all in `reports/report.json`.

### rucc at O3 against gcc-16

| rucc | gcc-16 | cases |
|---|---|---|
| compares | bit-test, table | 18 |
| table | bit-test, table | 18 |
| compares | table | 9 |
| lookup | compares | 3 |

| case | facet | rucc | gcc-16 |
|---|---|---|---|
| `interpreter-dispatch.12.table.direct.32.c17.49df9385` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.table.direct.8.c17.fe4a8b3d` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.12.table.none.32.c17.d9ec709c` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.table.none.8.c17.fd7b1f54` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.12.table.pointer.32.c17.090126a6` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.table.pointer.8.c17.085ef334` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.12.threaded.direct.32.c17.960c9160` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.threaded.direct.8.c17.ea92116c` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.12.threaded.none.32.c17.eec600eb` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.threaded.none.8.c17.0a37857e` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.12.threaded.pointer.32.c17.e1cf7a44` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.threaded.pointer.8.c17.31d76aaa` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.24.table.direct.32.c17.dc0e6741` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.table.direct.8.c17.d33ddfed` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.24.table.none.32.c17.2e0c71b0` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.table.none.8.c17.dd956486` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.24.table.pointer.32.c17.11a0293b` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.table.pointer.8.c17.a376142a` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.24.threaded.direct.32.c17.80c136bc` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.threaded.direct.8.c17.288d7cd7` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.24.threaded.none.32.c17.bb796889` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.threaded.none.8.c17.1ad7034a` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.24.threaded.pointer.32.c17.2aaaaaae` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.threaded.pointer.8.c17.5cdc26d1` | `interpreter-dispatch` | compares | bit-test, table |
| `interpreter-dispatch.4.table.direct.32.c17.8f2d2ccd` | `interpreter-dispatch` | table | bit-test, table |

And 23 more, all in `reports/report.json`.

### rucc at Os against gcc-16

| rucc | gcc-16 | cases |
|---|---|---|
| table | bit-test, table | 36 |
| compares | table | 4 |
| lookup | compares | 3 |
| table | compares | 1 |

| case | facet | rucc | gcc-16 |
|---|---|---|---|
| `interpreter-dispatch.12.table.direct.32.c17.49df9385` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.table.direct.8.c17.fe4a8b3d` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.table.none.32.c17.d9ec709c` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.table.none.8.c17.fd7b1f54` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.table.pointer.32.c17.090126a6` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.table.pointer.8.c17.085ef334` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.threaded.direct.32.c17.960c9160` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.threaded.direct.8.c17.ea92116c` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.threaded.none.32.c17.eec600eb` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.threaded.none.8.c17.0a37857e` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.threaded.pointer.32.c17.e1cf7a44` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.12.threaded.pointer.8.c17.31d76aaa` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.table.direct.32.c17.dc0e6741` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.table.direct.8.c17.d33ddfed` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.table.none.32.c17.2e0c71b0` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.table.none.8.c17.dd956486` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.table.pointer.32.c17.11a0293b` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.table.pointer.8.c17.a376142a` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.threaded.direct.32.c17.80c136bc` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.threaded.direct.8.c17.288d7cd7` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.threaded.none.32.c17.bb796889` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.threaded.none.8.c17.1ad7034a` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.threaded.pointer.32.c17.2aaaaaae` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.24.threaded.pointer.8.c17.5cdc26d1` | `interpreter-dispatch` | table | bit-test, table |
| `interpreter-dispatch.4.table.direct.32.c17.8f2d2ccd` | `interpreter-dispatch` | table | bit-test, table |

And 19 more, all in `reports/report.json`.

## Running this yourself

```sh
cargo run --release -p rucc-corpus -- run \
    --toolchain gcc-16 \
    --toolchain rucc \
    --reference gcc-16
```

The corpus is generated from the crates in this repository, so it is a function of the source and nothing else. Any run of the same commit produces the same 3524 programs with the same digest, and a report that disagrees with this one is a report about a different commit.
