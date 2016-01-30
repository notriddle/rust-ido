A pseudo-monadic immediate bind
===============================

[![Build Status](https://travis-ci.org/notriddle/rust-ido.svg)](https://travis-ci.org/notriddle/rust-ido)

It can't support every kind of monad, in particular it can't be used for
anything that will execute the binding asynchronously or more than once.
It is enough to support error-handling, logging, transactions, and similar, but
not stuff like iteration or promises. Use [mdo] for those.

However, unlike mdo, it does not use any closures in its implementation, so it
has a much less constraining lifetime signature and can be broken out of using
`return` or `break`. In other words, it's a usable alternative to try-catch in
imperative code. It just expands into boilerplate-heavy matching.

