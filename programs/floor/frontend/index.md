# frontend

language shape rather than optimization, including C23. Part of the floor phase of the M4 plan.

30 programs. Each row gives the axis point the program was generated for and the output it must produce. A compiler that prints anything else has a bug, whatever optimization level it was asked for.

| program | axes | dialect | must print |
|---|---|---|---|
| [`frontend.anonymous-members.c17.bd368ac8`](frontend.anonymous-members.c17.bd368ac8.c) | dialect-feature=anonymous-members | c17 | `1 12` |
| [`frontend.assign-to-const.c17.7feb04d5`](frontend.assign-to-const.c17.7feb04d5.c) | rejected=assign-to-const | c17 | nothing, it must be rejected with a diagnostic mentioning `read-only` |
| [`frontend.attributes.c23.e40fc972`](frontend.attributes.c23.e40fc972.c) | dialect-feature=attributes | c23 | `42 12` |
| [`frontend.auto.c23.83fbc2f6`](frontend.auto.c23.83fbc2f6.c) | dialect-feature=auto | c23 | `42` |
| [`frontend.binary-literals.c23.622ebb8b`](frontend.binary-literals.c23.622ebb8b.c) | dialect-feature=binary-literals | c23 | `42 6 16` |
| [`frontend.bit-fields.c17.171cd761`](frontend.bit-fields.c17.171cd761.c) | dialect-feature=bit-fields | c17 | `7 31 -8` |
| [`frontend.bit-precise.c23.6f93abb7`](frontend.bit-precise.c23.6f93abb7.c) | dialect-feature=bit-precise | c23 | `2000 300 2047` |
| [`frontend.bool-keyword.c23.b849aaef`](frontend.bool-keyword.c23.b849aaef.c) | dialect-feature=bool-keyword | c23 | `1 0 1 1` |
| [`frontend.break-outside-loop.c17.7e6ed84b`](frontend.break-outside-loop.c17.7e6ed84b.c) | rejected=break-outside-loop | c17 | nothing, it must be rejected with a diagnostic mentioning `break` |
| [`frontend.comma-and-conditional.c17.46b2e329`](frontend.comma-and-conditional.c17.46b2e329.c) | dialect-feature=comma-and-conditional | c17 | `5 4 1` |
| [`frontend.compound-literal.c17.93fca652`](frontend.compound-literal.c17.93fca652.c) | dialect-feature=compound-literal | c17 | `42 15` |
| [`frontend.constexpr.c23.563790df`](frontend.constexpr.c23.563790df.c) | dialect-feature=constexpr | c23 | `42 41` |
| [`frontend.designated-initializers.c17.8f65e174`](frontend.designated-initializers.c17.8f65e174.c) | dialect-feature=designated-initializers | c17 | `1 0 3 ...` and 2 more lines |
| [`frontend.digit-separators.c23.5bcc3026`](frontend.digit-separators.c23.5bcc3026.c) | dialect-feature=digit-separators | c23 | `1000 65535 170` |
| [`frontend.duplicate-case.c17.6c9f8c7a`](frontend.duplicate-case.c17.6c9f8c7a.c) | rejected=duplicate-case | c17 | nothing, it must be rejected with a diagnostic mentioning `duplicate case` |
| [`frontend.empty-initializer.c23.6d3cbe37`](frontend.empty-initializer.c23.6d3cbe37.c) | dialect-feature=empty-initializer | c23 | `0 0 0` |
| [`frontend.enum-fixed-underlying.c23.038ce1b6`](frontend.enum-fixed-underlying.c23.038ce1b6.c) | dialect-feature=enum-fixed-underlying | c23 | `1 200` |
| [`frontend.enum-values.c17.db23b9b8`](frontend.enum-values.c17.db23b9b8.c) | dialect-feature=enum-values | c17 | `0 10 11 -1` |
| [`frontend.failed-static-assert.c17.047a0cb3`](frontend.failed-static-assert.c17.047a0cb3.c) | rejected=failed-static-assert | c17 | nothing, it must be rejected with a diagnostic mentioning `static assert` |
| [`frontend.flexible-array.c17.a1f96800`](frontend.flexible-array.c17.a1f96800.c) | dialect-feature=flexible-array | c17 | `3 6` |
| [`frontend.generic-selection.c17.f14ed32a`](frontend.generic-selection.c17.f14ed32a.c) | dialect-feature=generic-selection | c17 | `1 2 3 0` |
| [`frontend.incompatible-return.c17.31403006`](frontend.incompatible-return.c17.31403006.c) | rejected=incompatible-return | c17 | nothing, it must be rejected with a diagnostic mentioning `incompatible` |
| [`frontend.nested-declarators.c17.d8097819`](frontend.nested-declarators.c17.d8097819.c) | dialect-feature=nested-declarators | c17 | `5 42` |
| [`frontend.nullptr.c23.0f11bf60`](frontend.nullptr.c23.0f11bf60.c) | dialect-feature=nullptr | c23 | `1 1 12` |
| [`frontend.redefinition.c17.9fa50e5b`](frontend.redefinition.c17.9fa50e5b.c) | rejected=redefinition | c17 | nothing, it must be rejected with a diagnostic mentioning `redefinition` |
| [`frontend.static-assert-keyword.c23.82e69fba`](frontend.static-assert-keyword.c23.82e69fba.c) | dialect-feature=static-assert-keyword | c23 | `10` |
| [`frontend.static-assert.c17.f958de34`](frontend.static-assert.c17.f958de34.c) | dialect-feature=static-assert | c17 | `1000` |
| [`frontend.too-few-arguments.c17.4a83774a`](frontend.too-few-arguments.c17.4a83774a.c) | rejected=too-few-arguments | c17 | nothing, it must be rejected with a diagnostic mentioning `too few arguments` |
| [`frontend.typeof.c23.abfcf86d`](frontend.typeof.c23.abfcf86d.c) | dialect-feature=typeof | c23 | `42` |
| [`frontend.undeclared-identifier.c17.df912590`](frontend.undeclared-identifier.c17.df912590.c) | rejected=undeclared-identifier | c17 | nothing, it must be rejected with a diagnostic mentioning `undeclared` |

