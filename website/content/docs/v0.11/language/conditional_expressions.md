+++
title = "Conditional Expressions"
slug = "conditional_expressions"
weight = 10
+++

# Conditional Expressions

## if

`if` expressions come in two flavours; single-line:

````koto
x = 99
if x % 2 == 0 then print 'even' else print 'odd'
# -> odd
````

{% example_playground_link() %}
play.clear_output()

x = 99
if x % 2 == 0 then print 'even' else print 'odd'
# -> odd

{% end %}
...and multi-line:

````koto
x = 24
if x < 0
  print 'negative'
else if x > 24
  print 'no way!'
else 
  print 'ok'
# -> ok
````

{% example_playground_link() %}
play.clear_output()

x = 24
if x < 0
  print 'negative'
else if x > 24
  print 'no way!'
else 
  print 'ok'
# -> ok

{% end %}
The result of an `if` expression is the final expression in the branch that gets
executed.

````koto
x = if 1 + 1 == 2 then 3 else -1
x, x
# -> (3, 3)

foo = if x > 0
  y = x * 10
  y + 3
else 
  y = x * 100
  y * y

foo, foo 
# -> (33, 33)
````

{% example_playground_link() %}
play.clear_output()

x = if 1 + 1 == 2 then 3 else -1
print x, x
# -> (3, 3)

foo = if x > 0
  y = x * 10
  y + 3
else 
  y = x * 100
  y * y

print foo, foo 
# -> (33, 33)

{% end %}
## switch

`switch` expressions can be used as more minimal alternative to `if`/`else if`/`else` 
cascades.

````koto
fib = |n|
  switch
    n <= 0 then 0
    n == 1 then 1
    else (fib n - 1) + (fib n - 2)

fib 7
# -> 13
````

{% example_playground_link() %}
play.clear_output()

fib = |n|
  switch
    n <= 0 then 0
    n == 1 then 1
    else (fib n - 1) + (fib n - 2)

print fib 7
# -> 13

{% end %}
## match

`match` expressions can be used to match a value against a series of patterns, 
with the matched pattern causing a specific branch of code to be executed.

Patterns can be literals or value bindings, 
and can include an `if` condition to further filter matches.

````koto
match 40 + 2
  0 then 'zero'
  1 then 'one'
  x if x < 10 then 'less than 10: $x'
  x if x < 50 then 'less than 50: $x'
  x then 'other: $x'
# -> less than 50: 42
````

{% example_playground_link() %}
play.clear_output()

print match 40 + 2
  0 then 'zero'
  1 then 'one'
  x if x < 10 then 'less than 10: $x'
  x if x < 50 then 'less than 50: $x'
  x then 'other: $x'
# -> less than 50: 42

{% end %}
The `_` wildcard match can be used to match against any value 
(when the matched value itself can be ignored), 
and `else` can be used for fallback branches.

````koto
fizz_buzz = |n|
  match n % 3, n % 5
    0, 0 then "Fizz Buzz"
    0, _ then "Fizz"
    _, 0 then "Buzz"
    else n

(10, 11, 12, 13, 14, 15)
  .each |n| fizz_buzz n
  .to_tuple()
# -> ('Buzz', 11, 'Fizz', 13, 14, 'Fizz Buzz')
````

{% example_playground_link() %}
play.clear_output()

fizz_buzz = |n|
  match n % 3, n % 5
    0, 0 then "Fizz Buzz"
    0, _ then "Fizz"
    _, 0 then "Buzz"
    else n

print (10, 11, 12, 13, 14, 15)
  .each |n| fizz_buzz n
  .to_tuple()
# -> ('Buzz', 11, 'Fizz', 13, 14, 'Fizz Buzz')

{% end %}
List and Tuple entries can be matched against, with `...` available for capturing the 
rest of the list.

````koto
match ['a', 'b', 'c'].extend [1, 2, 3]
  [1, ...] then "Starts with '1'"
  [..., 'y', last] then "Ends with 'y' followed by '$last'"
  ['a', x, others...] then
    "Starts with 'a', followed by '$x', then ${others.size()} others"
  unmatched then "other: $unmatched"
# -> Starts with 'a', followed by 'b', then 4 others
````

{% example_playground_link() %}
play.clear_output()

print match ['a', 'b', 'c'].extend [1, 2, 3]
  [1, ...] then "Starts with '1'"
  [..., 'y', last] then "Ends with 'y' followed by '$last'"
  ['a', x, others...] then
    "Starts with 'a', followed by '$x', then ${others.size()} others"
  unmatched then "other: $unmatched"
# -> Starts with 'a', followed by 'b', then 4 others

{% end %}
