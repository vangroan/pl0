# PL/0

![test & lint workflow](https://github.com/vangroan/pl0/actions/workflows/check.yaml/badge.svg)

Implementation of the PL/0 programming language compiler.

# Notes

- Procedures do not have call arguments, and there are no functions.
  Arguments are passed to calls via global variables.
- According to the grammar a *block* only contains one statement.
  It can't have none, or more than one. The `begin..end` sections are statements.
- A program is ended with by a period `.` and not necessarily `end.`.
  The `end` is part of the `begin..end` statement.
- Semicolons don't end statements, but act as a separator between statements
  in a `begin..end` statement. A final semicolon isn't allowed before `end`.

# Licence

This is free and unencumbered software released into the public domain.
